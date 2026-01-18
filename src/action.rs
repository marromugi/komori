/// Actions that can be performed in the application.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    /// Copy file from pasted path to current directory.
    CopyFile(String),
    /// Move selection down.
    MoveDown,
    /// Move selection up.
    MoveUp,
    /// Go to first item.
    GoToTop,
    /// Go to last item.
    GoToBottom,
    /// Enter directory or open file preview.
    Enter,
    /// Go to parent directory.
    Back,
    /// Toggle between directory view and preview view.
    TogglePreview,
    /// Enter search mode.
    EnterSearch,
    /// Exit current mode (search, etc.).
    ExitMode,
    /// Add character to search query.
    SearchInput(char),
    /// Delete last character from search query.
    SearchBackspace,
    /// Quit the application.
    Quit,
    /// No action.
    None,
}
