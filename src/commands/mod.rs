mod info;
mod list;

use crate::cli::Commands;

/// Main entry point for command execution.
/// Routes CLI commands to their respective logic modules
pub fn run(command: Commands) -> anyhow::Result<()> {
    match command {
        Commands::List(args) => list::run(args)?,
        Commands::Info(args) => info::run(args)?,
    }

    Ok(())
}
