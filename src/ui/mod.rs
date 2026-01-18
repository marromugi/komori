mod dir_view;
mod preview_view;
mod status_bar;

use ratatui::prelude::*;

use crate::app::{App, View};

use dir_view::render_dir_view;
use preview_view::render_preview_view;
use status_bar::render_status_bar;

/// Render the entire UI.
pub fn render(app: &App, frame: &mut Frame) {
    let area = frame.area();

    // Split into main area and status bar
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3), Constraint::Length(1)])
        .split(area);

    let main_area = chunks[0];
    let status_area = chunks[1];

    // Render main view
    match app.view {
        View::Directory => render_dir_view(app, frame, main_area),
        View::Preview => render_preview_view(app, frame, main_area),
    }

    // Render status bar
    render_status_bar(app, frame, status_area);
}
