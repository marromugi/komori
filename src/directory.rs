use std::path::{Path, PathBuf};

/// Represents a single entry in a directory listing.
#[derive(Debug, Clone)]
pub struct DirEntry {
    /// The file/directory name.
    pub name: String,
    /// The full path.
    pub path: PathBuf,
    /// Whether this is a directory.
    pub is_dir: bool,
}

impl DirEntry {
    /// Create a parent directory entry ("../").
    pub fn parent() -> Self {
        Self {
            name: "../".to_string(),
            path: PathBuf::from(".."),
            is_dir: true,
        }
    }
}

/// Manages directory listing state.
#[derive(Debug)]
pub struct Directory {
    /// Current directory path.
    pub current_dir: PathBuf,
    /// The directory where navigation is sandboxed - cannot go above this.
    pub sandbox_root: PathBuf,
    /// Whether sandbox mode is enabled.
    pub sandbox_enabled: bool,
    /// Entries in the current directory.
    pub entries: Vec<DirEntry>,
    /// Currently selected index.
    pub selected: usize,
    /// Scroll offset for viewport.
    pub scroll_offset: usize,
}

impl Directory {
    /// Create a new directory listing for the given path.
    pub fn new(path: &Path, sandbox_enabled: bool) -> color_eyre::Result<Self> {
        let current_dir = path.canonicalize()?;
        let mut dir = Self {
            current_dir: current_dir.clone(),
            sandbox_root: current_dir,
            sandbox_enabled,
            entries: Vec::new(),
            selected: 0,
            scroll_offset: 0,
        };
        dir.reload()?;
        Ok(dir)
    }

    /// Returns true if at sandbox root and sandbox is enabled.
    pub fn is_at_sandbox_root(&self) -> bool {
        self.sandbox_enabled && self.current_dir == self.sandbox_root
    }

    /// Reload the directory contents.
    pub fn reload(&mut self) -> color_eyre::Result<()> {
        self.entries.clear();

        // Add parent directory entry if not at sandbox root
        if self.current_dir.parent().is_some() && !self.is_at_sandbox_root() {
            self.entries.push(DirEntry::parent());
        }

        // Read directory entries
        let mut entries: Vec<DirEntry> = std::fs::read_dir(&self.current_dir)?
            .filter_map(|entry| entry.ok())
            .map(|entry| {
                let path = entry.path();
                let is_dir = path.is_dir();
                let mut name = entry.file_name().to_string_lossy().to_string();
                if is_dir {
                    name.push('/');
                }
                DirEntry { name, path, is_dir }
            })
            .collect();

        // Sort: directories first, then files, both alphabetically
        entries.sort_by(|a, b| match (a.is_dir, b.is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        });

        self.entries.extend(entries);

        // Reset selection if out of bounds
        if self.selected >= self.entries.len() {
            self.selected = self.entries.len().saturating_sub(1);
        }

        Ok(())
    }

    /// Move selection down.
    pub fn move_down(&mut self) {
        if self.selected < self.entries.len().saturating_sub(1) {
            self.selected += 1;
        }
    }

    /// Move selection up.
    pub fn move_up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    /// Go to first entry.
    pub fn go_to_top(&mut self) {
        self.selected = 0;
        self.scroll_offset = 0;
    }

    /// Go to last entry.
    pub fn go_to_bottom(&mut self) {
        self.selected = self.entries.len().saturating_sub(1);
    }

    /// Get the currently selected entry.
    pub fn selected_entry(&self) -> Option<&DirEntry> {
        self.entries.get(self.selected)
    }

    /// Enter the selected directory or return the file path.
    pub fn enter(&mut self) -> color_eyre::Result<Option<PathBuf>> {
        let Some(entry) = self.selected_entry() else {
            return Ok(None);
        };

        if entry.is_dir {
            // Navigate into directory
            if entry.name == "../" {
                // Go to parent (respects sandbox)
                if !self.is_at_sandbox_root() {
                    if let Some(parent) = self.current_dir.parent() {
                        self.current_dir = parent.to_path_buf();
                        self.selected = 0;
                        self.scroll_offset = 0;
                        self.reload()?;
                    }
                }
            } else {
                // Go into subdirectory
                self.current_dir = entry.path.clone();
                self.selected = 0;
                self.scroll_offset = 0;
                self.reload()?;
            }
            Ok(None)
        } else {
            // Return file path for preview
            Ok(Some(entry.path.clone()))
        }
    }

    /// Go to parent directory.
    pub fn go_back(&mut self) -> color_eyre::Result<()> {
        if self.is_at_sandbox_root() {
            return Ok(());
        }
        if let Some(parent) = self.current_dir.parent() {
            self.current_dir = parent.to_path_buf();
            self.selected = 0;
            self.scroll_offset = 0;
            self.reload()?;
        }
        Ok(())
    }

    /// Filter entries by search query, returning matching indices.
    pub fn filter(&self, query: &str) -> Vec<usize> {
        if query.is_empty() {
            return (0..self.entries.len()).collect();
        }

        let query_lower = query.to_lowercase();
        self.entries
            .iter()
            .enumerate()
            .filter(|(_, entry)| entry.name.to_lowercase().contains(&query_lower))
            .map(|(i, _)| i)
            .collect()
    }
}
