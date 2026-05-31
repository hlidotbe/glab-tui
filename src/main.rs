mod app;
mod event;
mod ui;
mod gitlab;
pub mod utils;

use anyhow::Result;
use app::App;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use event::{Event, EventHandler};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

#[tokio::main]
async fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and event handler
    let mut app = App::new();
    let mut events = EventHandler::new(250);

    // Initialize gitlab context
    if let Ok(context) = gitlab::client::get_project_context().await {
        app.project_context = context;
    }

    if let Ok(client) = gitlab::client::GitlabClient::new().await {
        if let Ok(issues) = gitlab::issues::list_issues(&client, &app.project_context).await {
            app.issues.items = issues;
        } else {
            app.error_message = Some("Failed to fetch issues".to_string());
        }
        if let Ok(mrs) = gitlab::mr::list_mrs(&client, &app.project_context).await {
            app.mrs.items = mrs;
        }
        if let Ok(pipelines) = gitlab::pipelines::list_pipelines(&client, &app.project_context).await {
            app.pipelines.items = pipelines;
        }
        app.gitlab_client = Some(client);
    } else {
        app.error_message = Some("Failed to initialize GitLab client".to_string());
    }

    // Run app
    while app.running {
        terminal.draw(|f| ui::render(f, &mut app))?;

        if let Some(event) = events.next().await {
            match event {
                Event::Tick => app.tick(),
                Event::Key(key_event) => {
                    if app.error_message.is_some() {
                        if key_event.code == KeyCode::Enter || key_event.code == KeyCode::Esc {
                            app.error_message = None;
                        }
                        continue;
                    }

                    if app.is_typing_search {
                        match key_event.code {
                            KeyCode::Enter | KeyCode::Esc => app.is_typing_search = false,
                            KeyCode::Backspace => {
                                app.search_query.pop();
                            }
                            KeyCode::Char(c) => {
                                app.search_query.push(c);
                            }
                            _ => {}
                        }
                        continue;
                    }

                    match key_event.code {
                        KeyCode::Char('q') | KeyCode::Esc => app.quit(),
                        KeyCode::Char('/') => {
                            app.is_typing_search = true;
                        }
                        KeyCode::Tab | KeyCode::Right | KeyCode::Char('l') => app.next_tab(),
                        KeyCode::BackTab | KeyCode::Left | KeyCode::Char('h') => app.previous_tab(),
                        KeyCode::Down | KeyCode::Char('j') => {
                            match app.active_tab {
                                app::Tab::Issues => app.issues.next(app.issues.items.len()),
                                app::Tab::MergeRequests => app.mrs.next(app.mrs.items.len()),
                                app::Tab::Pipelines => app.pipelines.next(app.pipelines.items.len()),
                                _ => {}
                            }
                        }
                        KeyCode::Up | KeyCode::Char('k') => {
                            match app.active_tab {
                                app::Tab::Issues => app.issues.previous(app.issues.items.len()),
                                app::Tab::MergeRequests => app.mrs.previous(app.mrs.items.len()),
                                app::Tab::Pipelines => app.pipelines.previous(app.pipelines.items.len()),
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
