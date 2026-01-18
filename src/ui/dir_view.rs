use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, List, ListItem, ListState};

use crate::app::{App, Mode};

/// Render the directory listing view.
pub fn render_dir_view(app: &App, frame: &mut Frame, area: Rect) {
    // Split into header (path) and list
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)])
        .split(area);

    let header_area = chunks[0];
    let list_area = chunks[1];

    // Render path header
    let path_display = app
        .directory
        .current_dir
        .to_string_lossy()
        .to_string();

    let header = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray))
        .title(path_display)
        .title_style(Style::default().fg(Color::Cyan).bold());

    frame.render_widget(header, header_area);

    // Build list items
    let items: Vec<ListItem> = app
        .directory
        .entries
        .iter()
        .enumerate()
        .map(|(i, entry)| {
            let style = if entry.is_dir {
                Style::default().fg(Color::Blue).bold()
            } else {
                Style::default().fg(Color::White)
            };

            // Check if this item matches the search filter
            let is_filtered_out = app.mode == Mode::Search
                && !app.filtered_indices.is_empty()
                && !app.filtered_indices.contains(&i);

            let style = if is_filtered_out {
                style.fg(Color::DarkGray)
            } else {
                style
            };

            ListItem::new(format!("  {}", entry.name)).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray)),
        )
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");

    let mut state = ListState::default();
    state.select(Some(app.directory.selected));

    frame.render_stateful_widget(list, list_area, &mut state);

    // Render search input if in search mode
    if app.mode == Mode::Search {
        render_search_overlay(app, frame, area);
    }
}

/// Render the search input overlay.
fn render_search_overlay(app: &App, frame: &mut Frame, area: Rect) {
    let search_area = Rect {
        x: area.x,
        y: area.y + area.height.saturating_sub(3),
        width: area.width,
        height: 1,
    };

    let search_line = Line::from(vec![
        Span::styled("/", Style::default().fg(Color::Yellow)),
        Span::styled(&app.search_query, Style::default().fg(Color::White)),
        Span::styled("_", Style::default().fg(Color::White).add_modifier(Modifier::SLOW_BLINK)),
    ]);

    frame.render_widget(search_line, search_area);
}
