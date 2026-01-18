# komori

A lightweight file explorer TUI.

## Features

- **Vim-style navigation** - Navigate with `hjkl` keys
- **Syntax highlighting** - Preview files with syntax highlighting
- **Fast** - Built with Rust for performance
- **Minimal** - Simple, focused interface

## Installation

```bash
cargo install --path .
```

## Usage

```bash
# Open current directory
komori

# Open specific directory
komori /path/to/directory
```

## Keybindings

| Key | Action |
|-----|--------|
| `j` / `Down` | Move down |
| `k` / `Up` | Move up |
| `l` / `Enter` | Enter directory / Preview file |
| `h` / `-` | Go to parent directory |
| `gg` | Go to top |
| `G` | Go to bottom |
| `Tab` / `p` | Toggle preview |
| `/` | Search (filter by filename) |
| `Esc` | Exit search mode |
| `q` | Quit |

## License

MIT
