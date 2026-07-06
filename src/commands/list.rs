use crate::services::{parser, scanner};

/// `porty list` command handler
pub fn run() -> anyhow::Result<()> {
    let raw = scanner::run_lsof()?;
    let ports = parser::parse(&raw);

    println!("{:?}", ports);
    Ok(())
}
