pub mod list;

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "porty")]
#[command(about = "Lightweight port managing CLI application for macOS", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Managing color output
    #[arg(long, value_enum, default_value_t = ColorMode::Auto)]
    pub color: ColorMode,
}

#[derive(Subcommand)]
pub enum Commands {
    /// List all available ports
    List(list::ListArgs),
}

/// Color output modes
#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorMode {
    Always,
    Never,
    Auto,
}
