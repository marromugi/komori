mod action;
mod app;
mod directory;
mod event;
mod preview;
mod tui;
mod ui;

use std::env;
use std::path::PathBuf;
use std::time::Duration;

use color_eyre::Result;

use app::App;
use event::{Event, EventHandler};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize error handling
    color_eyre::install()?;

    // Get starting directory from args or use current directory
    let start_dir = env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

    // Initialize terminal
    let mut terminal = tui::init()?;

    // Create app and event handler
    let mut app = App::new(&start_dir)?;
    let mut events = EventHandler::new(Duration::from_millis(250));

    // Main loop
    while !app.should_quit {
        // Render
        terminal.draw(|frame| ui::render(&app, frame))?;

        // Handle events
        if let Some(event) = events.next().await {
            match event {
                Event::Key(key) => {
                    let action = app.handle_key(key);
                    app.execute(action)?;
                }
                Event::Resize(_, _) => {
                    // Terminal will redraw on next iteration
                }
                Event::Tick => {
                    // Could be used for animations or periodic updates
                }
            }
        }
    }

    // Restore terminal
    tui::restore()?;

    Ok(())
}
