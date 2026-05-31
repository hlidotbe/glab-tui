use ratatui::{
    layout::{Constraint, Direction, Layout, Rect, Alignment},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph, Clear, Table, Row, Cell, BorderType},
    Frame,
};

use crate::app::{App, Tab};
use crate::utils::format::{truncate, time_ago};

pub fn render(f: &mut Frame, app: &mut App) {
    let size = f.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(size);

    // Header styling
    let header_bg = Color::Rgb(30, 30, 46);
    let title_style = Style::default().fg(Color::Rgb(137, 180, 250)).bg(header_bg).add_modifier(Modifier::BOLD);

    // Top: Title & Context
    let title_text = if app.is_typing_search {
        format!(" 🦊 GitLab TUI | {} | Search: {}_ ", app.project_context, app.search_query)
    } else if !app.search_query.is_empty() {
        format!(" 🦊 GitLab TUI | {} | Search: {} ", app.project_context, app.search_query)
    } else {
        format!(" 🦊 GitLab TUI | {} ", app.project_context)
    };

    let title = Paragraph::new(title_text)
        .style(title_style)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Rgb(137, 180, 250)))
        );
    f.render_widget(title, chunks[0]);

    // Middle: Sidebar | Main Area | Preview Area
    let middle_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(15),
            Constraint::Percentage(55),
            Constraint::Percentage(30),
        ])
        .split(chunks[1]);

    // Sidebar: Tabs
    let sidebar_items: Vec<ListItem> = Tab::ALL
        .iter()
        .map(|t| {
            if *t == app.active_tab {
                ListItem::new(format!(" ▶ {} ", t.title()))
                    .style(Style::default().fg(Color::Rgb(249, 226, 175)).add_modifier(Modifier::BOLD))
            } else {
                ListItem::new(format!("   {} ", t.title()))
                    .style(Style::default().fg(Color::DarkGray))
            }
        })
        .collect();
    
    let sidebar = List::new(sidebar_items)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Rgb(148, 226, 213)))
            .title(" Navigation ")
        );
    f.render_widget(sidebar, middle_chunks[0]);

    // Main Area: Tables
    let main_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Rgb(137, 180, 250)))
        .title(format!(" {} ", app.active_tab.title()));
    
    let sq = app.search_query.to_lowercase();
    let highlight_style = Style::default().bg(Color::Rgb(69, 71, 90)).add_modifier(Modifier::BOLD);
    let header_style = Style::default().fg(Color::Rgb(180, 190, 254)).add_modifier(Modifier::BOLD);

    match app.active_tab {
        Tab::Issues => {
            let filtered_issues: Vec<_> = app.issues.items.iter()
                .filter(|i| sq.is_empty() || i.title.to_lowercase().contains(&sq))
                .collect();
                
            let rows = filtered_issues.iter().map(|i| {
                let state_color = if i.state == "opened" { Color::Green } else { Color::Red };
                Row::new(vec![
                    Cell::from(format!("#{}", i.iid)),
                    Cell::from(i.state.clone()).style(Style::default().fg(state_color)),
                    Cell::from(truncate(&i.title, 50)),
                    Cell::from(truncate(&i.author.username, 15)),
                    Cell::from(time_ago(&i.updated_at)),
                ]).height(1)
            });

            let widths = [
                Constraint::Length(6),
                Constraint::Length(8),
                Constraint::Percentage(50),
                Constraint::Length(15),
                Constraint::Length(15),
            ];

            let table = Table::new(rows, widths)
                .header(Row::new(vec!["ID", "State", "Title", "Author", "Updated"]).style(header_style).height(1))
                .block(main_block)
                .row_highlight_style(highlight_style)
                .highlight_symbol(" 🚀 ");
            
            f.render_stateful_widget(table, middle_chunks[1], &mut app.issues.state);

            // Preview pane for Issues
            let preview_block = Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).title(" Details ");
            if let Some(selected) = app.issues.state.selected() {
                if let Some(issue) = filtered_issues.get(selected) {
                    let labels = if issue.labels.is_empty() { "None".to_string() } else { issue.labels.join(", ") };
                    let details = format!(
                        "Title: {}\n\nAuthor: @{}\nState: {}\nUpdated: {}\n\nLabels: {}",
                        issue.title, issue.author.username, issue.state, time_ago(&issue.updated_at), labels
                    );
                    f.render_widget(Paragraph::new(details).block(preview_block).wrap(ratatui::widgets::Wrap { trim: true }), middle_chunks[2]);
                } else {
                    f.render_widget(Paragraph::new("").block(preview_block), middle_chunks[2]);
                }
            } else {
                f.render_widget(Paragraph::new("Select an item to view details...").block(preview_block).style(Style::default().fg(Color::DarkGray)), middle_chunks[2]);
            }
        },
        Tab::MergeRequests => {
            let filtered_mrs: Vec<_> = app.mrs.items.iter()
                .filter(|m| sq.is_empty() || m.title.to_lowercase().contains(&sq))
                .collect();
                
            let rows = filtered_mrs.iter().map(|m| {
                let state_color = if m.state == "opened" { Color::Green } else { Color::Red };
                Row::new(vec![
                    Cell::from(format!("!{}", m.iid)),
                    Cell::from(m.state.clone()).style(Style::default().fg(state_color)),
                    Cell::from(truncate(&m.title, 50)),
                    Cell::from(truncate(&m.author.username, 15)),
                    Cell::from(time_ago(&m.updated_at)),
                ]).height(1)
            });

            let widths = [
                Constraint::Length(6),
                Constraint::Length(8),
                Constraint::Percentage(50),
                Constraint::Length(15),
                Constraint::Length(15),
            ];

            let table = Table::new(rows, widths)
                .header(Row::new(vec!["ID", "State", "Title", "Author", "Updated"]).style(header_style).height(1))
                .block(main_block)
                .row_highlight_style(highlight_style)
                .highlight_symbol(" 🚀 ");
            
            f.render_stateful_widget(table, middle_chunks[1], &mut app.mrs.state);

            // Preview pane
            let preview_block = Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).title(" Details ");
            if let Some(selected) = app.mrs.state.selected() {
                if let Some(mr) = filtered_mrs.get(selected) {
                    let labels = if mr.labels.is_empty() { "None".to_string() } else { mr.labels.join(", ") };
                    let details = format!(
                        "Title: {}\n\nAuthor: @{}\nState: {}\nUpdated: {}\n\nLabels: {}",
                        mr.title, mr.author.username, mr.state, time_ago(&mr.updated_at), labels
                    );
                    f.render_widget(Paragraph::new(details).block(preview_block).wrap(ratatui::widgets::Wrap { trim: true }), middle_chunks[2]);
                } else {
                    f.render_widget(Paragraph::new("").block(preview_block), middle_chunks[2]);
                }
            } else {
                f.render_widget(Paragraph::new("Select an item to view details...").block(preview_block).style(Style::default().fg(Color::DarkGray)), middle_chunks[2]);
            }
        },
        Tab::Pipelines => {
            let filtered_pipelines: Vec<_> = app.pipelines.items.iter()
                .filter(|p| sq.is_empty() || p.r#ref.to_lowercase().contains(&sq))
                .collect();
                
            let rows = filtered_pipelines.iter().map(|p| {
                let status_color = match p.status.as_str() {
                    "success" => Color::Green,
                    "failed" => Color::Red,
                    "running" => Color::Cyan,
                    _ => Color::DarkGray,
                };
                Row::new(vec![
                    Cell::from(format!("#{}", p.id)),
                    Cell::from(p.status.clone()).style(Style::default().fg(status_color)),
                    Cell::from(truncate(&p.r#ref, 50)),
                    Cell::from(time_ago(&p.updated_at)),
                ]).height(1)
            });

            let widths = [
                Constraint::Length(10),
                Constraint::Length(10),
                Constraint::Percentage(60),
                Constraint::Length(15),
            ];

            let table = Table::new(rows, widths)
                .header(Row::new(vec!["ID", "Status", "Ref", "Updated"]).style(header_style).height(1))
                .block(main_block)
                .row_highlight_style(highlight_style)
                .highlight_symbol(" 🚀 ");
            
            f.render_stateful_widget(table, middle_chunks[1], &mut app.pipelines.state);

            let preview_block = Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).title(" Details ");
            if let Some(selected) = app.pipelines.state.selected() {
                if let Some(p) = filtered_pipelines.get(selected) {
                    let details = format!(
                        "Pipeline ID: {}\n\nRef: {}\nStatus: {}\nUpdated: {}",
                        p.id, p.r#ref, p.status, time_ago(&p.updated_at)
                    );
                    f.render_widget(Paragraph::new(details).block(preview_block).wrap(ratatui::widgets::Wrap { trim: true }), middle_chunks[2]);
                } else {
                    f.render_widget(Paragraph::new("").block(preview_block), middle_chunks[2]);
                }
            } else {
                f.render_widget(Paragraph::new("Select an item to view details...").block(preview_block).style(Style::default().fg(Color::DarkGray)), middle_chunks[2]);
            }
        },
        _ => {
            let paragraph = Paragraph::new("\n\n  Feature pending...").block(main_block).style(Style::default().fg(Color::DarkGray));
            f.render_widget(paragraph, middle_chunks[1]);
            
            let preview_block = Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).title(" Details ");
            f.render_widget(Paragraph::new("").block(preview_block), middle_chunks[2]);
        }
    }

    // Bottom: Help Bar
    let help_text = match app.active_tab {
        Tab::Issues => "  h/l: Tabs • j/k: Navigate • /: Search • Enter: View • q: Quit  ",
        Tab::MergeRequests => "  h/l: Tabs • j/k: Navigate • /: Search • a: Approve • m: Merge • q: Quit  ",
        _ => "  h/l: Tabs • j/k: Navigate • /: Search • q: Quit  ",
    };
    let help = Paragraph::new(help_text)
        .style(Style::default().fg(Color::Rgb(205, 214, 244)).bg(Color::Rgb(49, 50, 68)).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::NONE));
    f.render_widget(help, chunks[2]);

    // Error Popup overlay
    if let Some(err) = &app.error_message {
        let block = Block::default().title(" Error ").borders(Borders::ALL).border_type(BorderType::Thick).style(Style::default().fg(Color::Red).bg(Color::Rgb(30, 30, 46)));
        let paragraph = Paragraph::new(err.clone())
            .block(block)
            .alignment(Alignment::Center);
        
        let area = centered_rect(60, 20, size);
        f.render_widget(Clear, area); //this clears out the background
        f.render_widget(paragraph, area);
    }
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
