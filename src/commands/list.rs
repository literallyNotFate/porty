use crate::{
    cli::list::ListArgs,
    models::HostType,
    services::{display, parser, scanner},
};

/// `porty list` command handler
pub fn run(args: ListArgs) -> anyhow::Result<()> {
    let raw: String = scanner::run_lsof()?;
    let mut ports = parser::parse(&raw);

    if let Some(ref q) = args.query {
        if let Ok(num) = q.parse::<u32>() {
            ports.retain(|p| p.port == num as u16 || p.pid == num);
        } else {
            let search_lower: String = q.to_lowercase();
            ports.retain(|p| p.cmd.to_lowercase().contains(&search_lower));
        }
    }

    if let Some(ref target_proto) = args.proto {
        ports.retain(|p| p.protocol == *target_proto);
    }

    if let Some(ref target_state) = args.state {
        ports.retain(|p| p.state.as_ref() == Some(target_state));
    }

    if let Some(ref host_scope) = args.host {
        ports.retain(|p| match (p.host_type(), host_scope.as_str()) {
            (HostType::Localhost, "localhost") => true,
            (HostType::Any, "any") => true,
            (HostType::External, "external") => true,
            _ => false,
        });
    }

    if let Some(exclude_port) = args.exclude {
        ports.retain(|p| p.port != exclude_port);
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
