use crate::{
    cli::list::ListArgs,
    models::{PortState, Protocol},
    services::{display, parser, scanner},
};

/// `porty list` command handler
pub fn run(args: ListArgs) -> anyhow::Result<()> {
    let raw: String = scanner::run_lsof()?;
    let mut ports = parser::parse(&raw);

    if args.listen_only {
        ports.retain(|p| matches!(p.state, Some(PortState::Listen)));
    }

    if args.tcp || args.udp {
        ports.retain(|p| match p.protocol {
            Protocol::Tcp => args.tcp,
            Protocol::Udp => args.udp,
            Protocol::Unknown(_) => false,
        });
    }

    if ports.is_empty() {
        use colored::Colorize;
        println!(
            "\n{}\n",
            "No active ports found matching your criteria.".dimmed()
        );
        return Ok(());
    }

    display::display_table(&ports);
    Ok(())
}
