use clap::Parser;
use porty::{cli::Cli, commands};

/// Main entry point
fn main() {
    let cli: Cli = Cli::parse();

    if let Err(err) = run(cli) {
        eprintln!("\n{} {}\n", "Error:", format!("{:#}", err));
        std::process::exit(1);
    }
}

/// Run application
fn run(cli: Cli) -> anyhow::Result<()> {
    commands::run(cli.command)?;
    Ok(())
}
