use chrono::{DateTime, Utc};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

pub fn truncate(s: &str, max_chars: usize) -> String {
    match s.char_indices().nth(max_chars) {
        None => String::from(s),
        Some((idx, _)) => {
            let mut truncated = String::from(&s[..idx]);
            truncated.push_str("...");
            truncated
        }
    }
}

pub fn time_ago(date_str: &str) -> String {
    if let Ok(parsed_time) = date_str.parse::<DateTime<Utc>>() {
        let now = Utc::now();
        let duration = now.signed_duration_since(parsed_time);

        let days = duration.num_days();
        if days > 0 {
            if days == 1 {
                return "1 day ago".to_string();
            }
            return format!("{} days ago", days);
        }

        let hours = duration.num_hours();
        if hours > 0 {
            if hours == 1 {
                return "1 hr ago".to_string();
            }
            return format!("{} hrs ago", hours);
        }

        let minutes = duration.num_minutes();
        if minutes > 0 {
            if minutes == 1 {
                return "1 min ago".to_string();
            }
            return format!("{} mins ago", minutes);
        }

        "just now".to_string()
    } else {
        date_str.to_string()
    }
}

pub fn format_ref(r#ref: &str) -> String {
    if let Some(mr_id) = r#ref.strip_prefix("refs/merge-requests/").and_then(|s| s.strip_suffix("/merge")) {
        format!("MR !{}", mr_id)
    } else if let Some(mr_id) = r#ref.strip_prefix("refs/merge-requests/").and_then(|s| s.split('/').next()) {
        format!("MR !{}", mr_id)
    } else if let Some(branch) = r#ref.strip_prefix("refs/heads/") {
        branch.to_string()
    } else if let Some(tag) = r#ref.strip_prefix("refs/tags/") {
        tag.to_string()
    } else {
        r#ref.to_string()
    }
}


pub fn render_markdown(markdown: &str) -> Vec<Line<'static>> {
    let mut lines = Vec::new();
    for line in markdown.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("# ") {
            let content = trimmed.strip_prefix("# ").unwrap_or(trimmed);
            lines.push(Line::from(vec![
                Span::styled(format!("# {}", content), Style::default().fg(Color::Rgb(187, 153, 238)).add_modifier(Modifier::BOLD | Modifier::UNDERLINED)),
            ]));
        } else if trimmed.starts_with("## ") {
            let content = trimmed.strip_prefix("## ").unwrap_or(trimmed);
            lines.push(Line::from(vec![
                Span::styled(format!("## {}", content), Style::default().fg(Color::Rgb(97, 175, 239)).add_modifier(Modifier::BOLD)),
            ]));
        } else if trimmed.starts_with("### ") {
            let content = trimmed.strip_prefix("### ").unwrap_or(trimmed);
            lines.push(Line::from(vec![
                Span::styled(format!("### {}", content), Style::default().fg(Color::Rgb(152, 195, 121)).add_modifier(Modifier::BOLD)),
            ]));
        } else if trimmed.starts_with("- ") || trimmed.starts_with("* ") {
            let content = if trimmed.starts_with("- ") {
                trimmed.strip_prefix("- ").unwrap()
            } else {
                trimmed.strip_prefix("* ").unwrap()
            };
            let mut spans = vec![
                Span::styled("  • ", Style::default().fg(Color::Rgb(187, 153, 238)).add_modifier(Modifier::BOLD)),
            ];
            spans.extend(parse_inline_styles(content));
            lines.push(Line::from(spans));
        } else if trimmed.starts_with("> ") {
            let content = trimmed.strip_prefix("> ").unwrap_or(trimmed);
            let mut spans = vec![
                Span::styled("  ▌ ", Style::default().fg(Color::Rgb(127, 132, 142))),
            ];
            spans.extend(parse_inline_styles(content));
            lines.push(Line::from(spans));
        } else {
            lines.push(Line::from(parse_inline_styles(line)));
        }
    }
    lines
}

fn parse_inline_styles(text: &str) -> Vec<Span<'static>> {
    let mut spans = Vec::new();
    let chars = text.chars().collect::<Vec<char>>();
    let mut i = 0;
    let mut current_segment = String::new();

    while i < chars.len() {
        if chars[i] == '`' {
            if !current_segment.is_empty() {
                spans.push(Span::styled(current_segment.clone(), Style::default().fg(Color::Rgb(171, 178, 191))));
                current_segment.clear();
            }
            i += 1;
            let mut code = String::new();
            while i < chars.len() && chars[i] != '`' {
                code.push(chars[i]);
                i += 1;
            }
            spans.push(Span::styled(
                code,
                Style::default().fg(Color::Rgb(224, 108, 117)).bg(Color::Rgb(40, 44, 52)),
            ));
            if i < chars.len() {
                i += 1;
            }
        } else if i + 1 < chars.len() && chars[i] == '*' && chars[i+1] == '*' {
            if !current_segment.is_empty() {
                spans.push(Span::styled(current_segment.clone(), Style::default().fg(Color::Rgb(171, 178, 191))));
                current_segment.clear();
            }
            i += 2;
            let mut bold_text = String::new();
            while i + 1 < chars.len() && !(chars[i] == '*' && chars[i+1] == '*') {
                bold_text.push(chars[i]);
                i += 1;
            }
            if i < chars.len() && (i + 1 >= chars.len() || !(chars[i] == '*' && chars[i+1] == '*')) {
                bold_text.push(chars[i]);
                i += 1;
            }
            spans.push(Span::styled(
                bold_text,
                Style::default().fg(Color::Rgb(220, 223, 228)).add_modifier(Modifier::BOLD),
            ));
            if i + 1 < chars.len() && chars[i] == '*' && chars[i+1] == '*' {
                i += 2;
            }
        } else {
            current_segment.push(chars[i]);
            i += 1;
        }
    }

    if !current_segment.is_empty() {
        spans.push(Span::styled(current_segment, Style::default().fg(Color::Rgb(171, 178, 191))));
    }

    spans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_ref() {
        assert_eq!(format_ref("refs/merge-requests/123/merge"), "MR !123");
        assert_eq!(format_ref("refs/merge-requests/456/head"), "MR !456");
        assert_eq!(format_ref("refs/heads/feature/login"), "feature/login");
        assert_eq!(format_ref("refs/tags/v1.2.3"), "v1.2.3");
        assert_eq!(format_ref("main"), "main");
    }

    #[test]
    fn test_render_markdown() {
        let md = "# Header1\n## Header2\n- Bullet `code` item\nNormal line with **bold** text";
        let lines = render_markdown(md);
        assert_eq!(lines.len(), 4);
    }
}
