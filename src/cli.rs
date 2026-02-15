use clap::Parser;
use std::path::PathBuf;

/// devfetch: A discovery engine for developer tools and project ecosystems
#[derive(Parser, Debug)]
#[command(
    name = "devfetch",
    version,
    about = "Discover developer tools and project ecosystems",
    long_about = "devfetch dynamically discovers developer tools, runtimes, compilers, and SDKs \
                  on your system without hardcoded lists. It scans your PATH for global tools \
                  and analyzes project directories for ecosystem-specific information."
)]
pub struct Cli {
    /// Target directory to inspect (default: current directory)
    #[arg(value_name = "PATH")]
    pub path: Option<PathBuf>,

    /// Show only global tools (ignore project context)
    #[arg(long, conflicts_with = "local")]
    pub global: bool,

    /// Show only project-specific information (ignore global tools)
    #[arg(long, conflicts_with = "global")]
    pub local: bool,

    /// Output in JSON format
    #[arg(long)]
    pub json: bool,

    /// Verbose output for debugging
    #[arg(short, long)]
    pub verbose: bool,

    /// Disable colored output (useful for piping)
    #[arg(long = "no-color")]
    pub no_color: bool,
}

impl Cli {
    pub fn should_scan_global(&self) -> bool {
        !self.local
    }

    pub fn should_scan_local(&self) -> bool {
        !self.global
    }

    pub fn target_path(&self) -> PathBuf {
        self.path
            .clone()
            .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")))
    }
}

