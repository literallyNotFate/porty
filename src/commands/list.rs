use crate::services::{display, parser, scanner};

/// `porty list` command handler
pub fn run() -> anyhow::Result<()> {
    let raw: String = scanner::run_lsof()?;
    let ports = parser::parse(&raw);

    display::display_table(&ports);
    Ok(())
}
