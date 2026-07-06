use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "porty")]
#[command(about = "Lightweight port managing CLI application for macOS", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// List all available ports
    List,
}
