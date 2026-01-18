use std::path::Path;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::action::Action;
use crate::directory::Directory;
use crate::preview::Preview;

/// Application mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// Normal navigation mode.
    Normal,
    /// Search mode (filtering by filename).
    Search,
}

/// Current view.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum View {
    /// Directory listing view.
    Directory,
    /// File preview view.
    Preview,
}

/// Main application state.
pub struct App {
    /// Whether the app should quit.
    pub should_quit: bool,
    /// Current mode.
    pub mode: Mode,
    /// Current view.
    pub view: View,
    /// Directory listing state.
    pub directory: Directory,
    /// File preview state.
    pub preview: Preview,
    /// Search query.
    pub search_query: String,
    /// Filtered indices (when searching).
    pub filtered_indices: Vec<usize>,
    /// Pending 'g' key for gg command.
    pub pending_g: bool,
}

impl App {
    /// Create a new application.
    pub fn new(path: &Path, sandbox_enabled: bool) -> color_eyre::Result<Self> {
        Ok(Self {
            should_quit: false,
            mode: Mode::Normal,
            view: View::Directory,
            directory: Directory::new(path, sandbox_enabled)?,
            preview: Preview::new(),
            search_query: String::new(),
            filtered_indices: Vec::new(),
            pending_g: false,
        })
    }

    /// Map a key event to an action.
    pub fn handle_key(&mut self, key: KeyEvent) -> Action {
        match self.mode {
            Mode::Normal => self.handle_normal_key(key),
            Mode::Search => self.handle_search_key(key),
        }
    }

    fn handle_normal_key(&mut self, key: KeyEvent) -> Action {
        // Handle pending 'g' for gg command
        if self.pending_g {
            self.pending_g = false;
            if key.code == KeyCode::Char('g') {
                return Action::GoToTop;
            }
            // Any other key cancels the pending g
        }

        match key.code {
            // Navigation
            KeyCode::Char('j') | KeyCode::Down => Action::MoveDown,
            KeyCode::Char('k') | KeyCode::Up => Action::MoveUp,
            KeyCode::Char('g') => {
                self.pending_g = true;
                Action::None
            }
            KeyCode::Char('G') => Action::GoToBottom,

            // Directory navigation
            KeyCode::Char('h') | KeyCode::Char('-') | KeyCode::Left => Action::Back,
            KeyCode::Char('l') | KeyCode::Enter | KeyCode::Right => Action::Enter,

            // View toggle
            KeyCode::Tab | KeyCode::Char('p') => Action::TogglePreview,

            // Search
            KeyCode::Char('/') => Action::EnterSearch,

            // Quit
            KeyCode::Char('q') => Action::Quit,
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => Action::Quit,

            _ => Action::None,
        }
    }

    fn handle_search_key(&mut self, key: KeyEvent) -> Action {
        match key.code {
            KeyCode::Esc => Action::ExitMode,
            KeyCode::Enter => Action::ExitMode,
            KeyCode::Backspace => Action::SearchBackspace,
            KeyCode::Char(c) => Action::SearchInput(c),
            _ => Action::None,
        }
    }

    /// Execute an action.
    pub fn execute(&mut self, action: Action) -> color_eyre::Result<()> {
        match action {
            Action::Quit => {
                self.should_quit = true;
            }
            Action::MoveDown => {
                if self.view == View::Preview {
                    self.preview.scroll_down();
                } else {
                    self.directory.move_down();
                }
            }
            Action::MoveUp => {
                if self.view == View::Preview {
                    self.preview.scroll_up();
                } else {
                    self.directory.move_up();
                }
            }
            Action::GoToTop => {
                if self.view == View::Directory {
                    self.directory.go_to_top();
                } else {
                    self.preview.scroll_offset = 0;
                }
            }
            Action::GoToBottom => {
                if self.view == View::Directory {
                    self.directory.go_to_bottom();
                }
            }
            Action::Enter => {
                if self.view == View::Directory {
                    if let Some(path) = self.directory.enter()? {
                        // File selected, show preview
                        self.preview.load(&path)?;
                        self.view = View::Preview;
                    }
                }
            }
            Action::Back => {
                if self.view == View::Preview {
                    self.view = View::Directory;
                } else {
                    self.directory.go_back()?;
                }
            }
            Action::TogglePreview => {
                match self.view {
                    View::Directory => {
                        // Load preview for selected file if it's a file
                        if let Some(entry) = self.directory.selected_entry() {
                            if !entry.is_dir {
                                self.preview.load(&entry.path)?;
                                self.view = View::Preview;
                            }
                        }
                    }
                    View::Preview => {
                        self.view = View::Directory;
                    }
                }
            }
            Action::EnterSearch => {
                self.mode = Mode::Search;
                self.search_query.clear();
                self.filtered_indices = self.directory.filter("");
            }
            Action::ExitMode => {
                self.mode = Mode::Normal;
                self.search_query.clear();
                self.filtered_indices.clear();
            }
            Action::SearchInput(c) => {
                self.search_query.push(c);
                self.filtered_indices = self.directory.filter(&self.search_query);
                // Move selection to first match if any
                if let Some(&first) = self.filtered_indices.first() {
                    self.directory.selected = first;
                }
            }
            Action::SearchBackspace => {
                self.search_query.pop();
                self.filtered_indices = self.directory.filter(&self.search_query);
            }
            Action::None => {}
        }
        Ok(())
    }
}
