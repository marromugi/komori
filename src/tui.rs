use std::io::{self, stdout, Stdout};

use crossterm::{
    event::{DisableBracketedPaste, EnableBracketedPaste},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;

pub type Tui = Terminal<CrosstermBackend<Stdout>>;

/// Initialize the terminal for TUI rendering.
pub fn init() -> io::Result<Tui> {
    execute!(stdout(), EnterAlternateScreen, EnableBracketedPaste)?;
    enable_raw_mode()?;
    Terminal::new(CrosstermBackend::new(stdout()))
}

/// Restore the terminal to its original state.
pub fn restore() -> io::Result<()> {
    execute!(stdout(), LeaveAlternateScreen, DisableBracketedPaste)?;
    disable_raw_mode()?;
    Ok(())
}
