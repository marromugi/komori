use ratatui::prelude::*;

use crate::app::{App, Mode, View};

/// Render the status bar.
pub fn render_status_bar(app: &App, frame: &mut Frame, area: Rect) {
    let mode_str = match app.mode {
        Mode::Normal => match app.view {
            View::Directory => "NORMAL",
            View::Preview => "PREVIEW",
        },
        Mode::Search => "SEARCH",
    };

    let mode_style = match app.mode {
        Mode::Normal => match app.view {
            View::Directory => Style::default().fg(Color::Black).bg(Color::Blue),
            View::Preview => Style::default().fg(Color::Black).bg(Color::Green),
        },
        Mode::Search => Style::default().fg(Color::Black).bg(Color::Yellow),
    };

    // Get current file/item info
    let (item_name, item_count) = match app.view {
        View::Directory => {
            let name = app
                .directory
                .selected_entry()
                .map(|e| e.name.clone())
                .unwrap_or_default();
            let count = app.directory.entries.len();
            (name, format!("{} items", count))
        }
        View::Preview => {
            let name = app
                .preview
                .path
                .as_ref()
                .and_then(|p| p.file_name())
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default();
            let count = app.preview.line_count();
            (name, format!("{} lines", count))
        }
    };

    // Sandbox indicator
    let sandbox_indicator = if app.directory.sandbox_enabled {
        Span::styled(" [SANDBOX] ", Style::default().fg(Color::Black).bg(Color::Magenta))
    } else {
        Span::raw("")
    };

    let status_line = Line::from(vec![
        Span::styled(format!(" {} ", mode_str), mode_style),
        sandbox_indicator,
        Span::styled(" ", Style::default()),
        Span::styled(item_name, Style::default().fg(Color::White)),
        Span::styled(" | ", Style::default().fg(Color::DarkGray)),
        Span::styled(item_count, Style::default().fg(Color::DarkGray)),
    ]);

    frame.render_widget(status_line, area);
}
