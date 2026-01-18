use std::path::PathBuf;

use serde::Deserialize;

/// Configuration file structure loaded from ~/.config/komori/config.toml
#[derive(Debug, Default, Deserialize)]
pub struct ConfigFile {
    #[serde(default)]
    pub sandbox: SandboxConfig,
}

/// Sandbox configuration section
#[derive(Debug, Deserialize)]
pub struct SandboxConfig {
    /// Whether sandbox mode is enabled (default: true)
    #[serde(default = "default_true")]
    pub enabled: bool,
}

fn default_true() -> bool {
    true
}

impl Default for SandboxConfig {
    fn default() -> Self {
        Self { enabled: true }
    }
}

/// CLI arguments parsed from command line
#[derive(Debug, Default)]
pub struct CliArgs {
    /// Starting directory
    pub directory: Option<PathBuf>,
    /// Explicit sandbox override from CLI (None = not specified)
    pub sandbox: Option<bool>,
    /// Show help
    pub help: bool,
}

/// Final resolved configuration
#[derive(Debug)]
pub struct Config {
    /// Starting directory
    pub start_dir: PathBuf,
    /// Whether sandbox mode is enabled
    pub sandbox_enabled: bool,
}

impl Config {
    /// Load configuration from CLI args and config file.
    /// Priority: CLI > Config File > Defaults
    pub fn load() -> color_eyre::Result<Self> {
        let cli = parse_cli();

        if cli.help {
            print_help();
            std::process::exit(0);
        }

        let file_config = load_config_file();

        // Resolve start directory
        let start_dir = cli
            .directory
            .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

        // Resolve sandbox setting: CLI > Config > Default (true)
        let sandbox_enabled = cli.sandbox.unwrap_or(file_config.sandbox.enabled);

        Ok(Self {
            start_dir,
            sandbox_enabled,
        })
    }
}

/// Parse CLI arguments manually
fn parse_cli() -> CliArgs {
    let mut args = CliArgs::default();

    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "--sandbox" | "-s" => args.sandbox = Some(true),
            "--no-sandbox" | "-S" => args.sandbox = Some(false),
            "--help" | "-h" => args.help = true,
            s if !s.starts_with('-') => args.directory = Some(PathBuf::from(s)),
            _ => {
                eprintln!("Unknown argument: {}", arg);
                std::process::exit(1);
            }
        }
    }

    args
}

/// Load config file from ~/.config/komori/config.toml
fn load_config_file() -> ConfigFile {
    dirs::config_dir()
        .map(|p| p.join("komori/config.toml"))
        .filter(|p| p.exists())
        .and_then(|p| std::fs::read_to_string(&p).ok())
        .and_then(|s| toml::from_str(&s).ok())
        .unwrap_or_default()
}

fn print_help() {
    println!(
        r#"komori - TUI file explorer

USAGE: komori [OPTIONS] [DIRECTORY]

ARGS:
    [DIRECTORY]    Starting directory (default: current directory)

OPTIONS:
    -s, --sandbox      Enable sandbox mode (cannot navigate above start dir) [default]
    -S, --no-sandbox   Disable sandbox mode
    -h, --help         Show help

CONFIG: ~/.config/komori/config.toml

    [sandbox]
    enabled = true
"#
    );
}
