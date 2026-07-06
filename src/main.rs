use clap::Parser;
use colored::{Colorize, control};
use porty::{
    cli::{Cli, ColorMode},
    commands,
};

/// Main entry point
fn main() {
    let cli: Cli = Cli::parse();
    match cli.color {
        ColorMode::Always => control::set_override(true),
        ColorMode::Never => control::set_override(false),
        ColorMode::Auto => {
            control::unset_override();
        }
    }

    if let Err(err) = run(cli) {
        eprintln!(
            "\n{}  {} {}\n",
            "󰜺".red().bold(),
            "Error:".red().bold(),
            format!("{:#}", err).white()
        );

        std::process::exit(1);
    }
}

/// Run application
fn run(cli: Cli) -> anyhow::Result<()> {
    commands::run(cli.command)?;
    Ok(())
}
