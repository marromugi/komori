use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::app::App;
use crate::preview::PreviewContent;

/// Render the file preview view.
pub fn render_preview_view(app: &App, frame: &mut Frame, area: Rect) {
    // Split into header (path) and content
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)])
        .split(area);

    let header_area = chunks[0];
    let content_area = chunks[1];

    // Render path header
    let path_display = app
        .preview
        .path
        .as_ref()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| "No file".to_string());

    let header = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray))
        .title(path_display)
        .title_style(Style::default().fg(Color::Green).bold());

    frame.render_widget(header, header_area);

    // Render content
    let inner_area = Rect {
        x: content_area.x + 1,
        y: content_area.y + 1,
        width: content_area.width.saturating_sub(2),
        height: content_area.height.saturating_sub(2),
    };

    let content_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));

    frame.render_widget(content_block, content_area);

    match &app.preview.content {
        PreviewContent::Loading => {
            let text = Paragraph::new("Loading...")
                .style(Style::default().fg(Color::DarkGray));
            frame.render_widget(text, inner_area);
        }
        PreviewContent::Highlighted(lines) => {
            render_highlighted_content(lines, app.preview.scroll_offset, frame, inner_area);
        }
        PreviewContent::Text(lines) => {
            let visible_lines: Vec<Line> = lines
                .iter()
                .skip(app.preview.scroll_offset)
                .take(inner_area.height as usize)
                .enumerate()
                .map(|(i, line)| {
                    let line_num = app.preview.scroll_offset + i + 1;
                    Line::from(vec![
                        Span::styled(
                            format!("{:4} ", line_num),
                            Style::default().fg(Color::DarkGray),
                        ),
                        Span::raw(line),
                    ])
                })
                .collect();

            let paragraph = Paragraph::new(visible_lines);
            frame.render_widget(paragraph, inner_area);
        }
        PreviewContent::Binary => {
            let text = Paragraph::new("Binary file (cannot display)")
                .style(Style::default().fg(Color::Yellow));
            frame.render_widget(text, inner_area);
        }
        PreviewContent::TooLarge => {
            let text = Paragraph::new("File too large to preview (> 1MB)")
                .style(Style::default().fg(Color::Yellow));
            frame.render_widget(text, inner_area);
        }
        PreviewContent::Directory => {
            let text = Paragraph::new("Directory")
                .style(Style::default().fg(Color::Blue));
            frame.render_widget(text, inner_area);
        }
        PreviewContent::Error(msg) => {
            let text = Paragraph::new(format!("Error: {}", msg))
                .style(Style::default().fg(Color::Red));
            frame.render_widget(text, inner_area);
        }
    }
}

/// Render syntax-highlighted content.
fn render_highlighted_content(
    lines: &[crate::preview::HighlightedLine],
    scroll_offset: usize,
    frame: &mut Frame,
    area: Rect,
) {
    let visible_lines: Vec<Line> = lines
        .iter()
        .skip(scroll_offset)
        .take(area.height as usize)
        .enumerate()
        .map(|(i, line)| {
            let line_num = scroll_offset + i + 1;
            let mut spans = vec![Span::styled(
                format!("{:4} ", line_num),
                Style::default().fg(Color::DarkGray),
            )];

            for (style, text) in &line.segments {
                let fg = Color::Rgb(style.foreground.r, style.foreground.g, style.foreground.b);
                spans.push(Span::styled(text.clone(), Style::default().fg(fg)));
            }

            Line::from(spans)
        })
        .collect();

    let paragraph = Paragraph::new(visible_lines);
    frame.render_widget(paragraph, area);
}
