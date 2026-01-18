use std::path::{Path, PathBuf};

use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

/// A single line with syntax highlighting information.
#[derive(Debug, Clone)]
pub struct HighlightedLine {
    pub segments: Vec<(Style, String)>,
}

/// File preview content.
#[derive(Debug)]
pub enum PreviewContent {
    /// File is loading.
    Loading,
    /// Plain text content (no highlighting).
    Text(Vec<String>),
    /// Syntax highlighted content.
    Highlighted(Vec<HighlightedLine>),
    /// Binary file (not displayable).
    Binary,
    /// Error loading file.
    Error(String),
    /// File is too large to preview.
    TooLarge,
    /// Directory (show summary instead).
    Directory,
}

/// Manages file preview state.
pub struct Preview {
    /// Current file path being previewed.
    pub path: Option<PathBuf>,
    /// Preview content.
    pub content: PreviewContent,
    /// Scroll offset.
    pub scroll_offset: usize,
    /// Syntax highlighting resources.
    syntax_set: SyntaxSet,
    theme_set: ThemeSet,
}

impl Preview {
    /// Maximum file size to preview (1 MB).
    const MAX_FILE_SIZE: u64 = 1024 * 1024;

    /// Create a new preview manager.
    pub fn new() -> Self {
        Self {
            path: None,
            content: PreviewContent::Loading,
            scroll_offset: 0,
            syntax_set: SyntaxSet::load_defaults_newlines(),
            theme_set: ThemeSet::load_defaults(),
        }
    }

    /// Load a file for preview.
    pub fn load(&mut self, path: &Path) -> color_eyre::Result<()> {
        self.path = Some(path.to_path_buf());
        self.scroll_offset = 0;

        // Check if it's a directory
        if path.is_dir() {
            self.content = PreviewContent::Directory;
            return Ok(());
        }

        // Check file size
        let metadata = std::fs::metadata(path)?;
        if metadata.len() > Self::MAX_FILE_SIZE {
            self.content = PreviewContent::TooLarge;
            return Ok(());
        }

        // Try to read as text
        match std::fs::read_to_string(path) {
            Ok(text) => {
                // Try syntax highlighting
                let syntax = self
                    .syntax_set
                    .find_syntax_for_file(path)
                    .ok()
                    .flatten()
                    .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text());

                let theme = &self.theme_set.themes["base16-ocean.dark"];
                let mut highlighter = HighlightLines::new(syntax, theme);

                let lines: Vec<HighlightedLine> = LinesWithEndings::from(&text)
                    .map(|line| {
                        let ranges = highlighter
                            .highlight_line(line, &self.syntax_set)
                            .unwrap_or_default();
                        HighlightedLine {
                            segments: ranges
                                .into_iter()
                                .map(|(style, text)| (style, text.to_string()))
                                .collect(),
                        }
                    })
                    .collect();

                self.content = PreviewContent::Highlighted(lines);
            }
            Err(_) => {
                // Likely binary file
                self.content = PreviewContent::Binary;
            }
        }

        Ok(())
    }

    /// Clear the preview.
    pub fn clear(&mut self) {
        self.path = None;
        self.content = PreviewContent::Loading;
        self.scroll_offset = 0;
    }

    /// Scroll down.
    pub fn scroll_down(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_add(1);
    }

    /// Scroll up.
    pub fn scroll_up(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_sub(1);
    }

    /// Get the number of lines in the preview.
    pub fn line_count(&self) -> usize {
        match &self.content {
            PreviewContent::Highlighted(lines) => lines.len(),
            PreviewContent::Text(lines) => lines.len(),
            _ => 0,
        }
    }
}

impl Default for Preview {
    fn default() -> Self {
        Self::new()
    }
}
