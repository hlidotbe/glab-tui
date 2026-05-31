use crossterm::event::{self, Event as CrosstermEvent, KeyEvent, MouseEvent};
use std::time::{Duration, Instant};
use tokio::sync::mpsc;

#[derive(Clone, Debug)]
pub enum Event {
    Tick,
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
}

#[derive(Debug)]
pub struct EventHandler {
    receiver: mpsc::UnboundedReceiver<Event>,
}

impl EventHandler {
    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (sender, receiver) = mpsc::unbounded_channel();
        let _sender = sender.clone();
        
        tokio::spawn(async move {
            let mut last_tick = Instant::now();
            loop {
                let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or_else(|| Duration::from_secs(0));

                if event::poll(timeout).expect("failed to poll new events") {
                    match event::read().expect("failed to read event") {
                        CrosstermEvent::Key(e) => sender.send(Event::Key(e)),
                        CrosstermEvent::Mouse(e) => sender.send(Event::Mouse(e)),
                        CrosstermEvent::Resize(w, h) => sender.send(Event::Resize(w, h)),
                        CrosstermEvent::FocusGained => Ok(()),
                        CrosstermEvent::FocusLost => Ok(()),
                        CrosstermEvent::Paste(_) => Ok(()),
                    }
                    .expect("failed to send event");
                }

                if last_tick.elapsed() >= tick_rate {
                    sender.send(Event::Tick).expect("failed to send tick event");
                    last_tick = Instant::now();
                }
            }
        });

        Self { receiver }
    }

    pub async fn next(&mut self) -> Option<Event> {
        self.receiver.recv().await
    }
}
