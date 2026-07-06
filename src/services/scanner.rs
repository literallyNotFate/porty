use anyhow::{Context, Result};
use std::process::{Command, Output};

/// Run `lsof` command
pub fn run_lsof() -> Result<String> {
    let args: Vec<&str> = vec!["-i", "-P", "-n"];
    let output: Output = Command::new("lsof")
        .args(&args)
        .output()
        .context("Failed to run `lsof`. Is it installed?")?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
