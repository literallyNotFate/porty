use crate::{
    cli::list::ListArgs,
    services::{display, parser, scanner},
};

/// `porty list` command handler
pub fn run(args: ListArgs) -> anyhow::Result<()> {
    let raw: String = scanner::run_lsof()?;
    let mut ports = parser::parse(&raw);

    args.filter(&mut ports);

    if ports.is_empty() {
        use colored::Colorize;
        println!(
            "\n{}\n",
            "No active ports found matching your criteria.".dimmed()
        );
        return Ok(());
    }

    ports.sort();
    display::display_table(&ports, args.long);
    Ok(())
}
