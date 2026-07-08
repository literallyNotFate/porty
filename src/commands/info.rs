use crate::{
    cli::info::InfoArgs,
    models::{HostType, PortInfo, PortState},
    services::{parser, scanner},
};
use colored::Colorize;
use std::collections::BTreeMap;
use sysinfo::{Pid, ProcessesToUpdate, System};

/// `porty info` command handler
pub fn run(args: InfoArgs) -> anyhow::Result<()> {
    let raw: String = scanner::run_lsof()?;
    let ports = parser::parse(&raw);

    let filtered: Vec<&PortInfo> = if let Ok(num) = args.target.parse::<u32>() {
        let by_port: Vec<&PortInfo> = ports.iter().filter(|p| p.port as u32 == num).collect();
        if !by_port.is_empty() {
            by_port
        } else {
            ports.iter().filter(|p| p.pid == num).collect()
        }
    } else {
        let target_lower: String = args.target.to_lowercase();
        ports
            .iter()
            .filter(|p| p.cmd.to_lowercase().contains(&target_lower))
            .collect()
    };

    if filtered.is_empty() {
        println!(
            "\n{} `{}`\n",
            "No active ports or processes found matching",
            args.target.yellow().bold()
        );
        return Ok(());
    }

    let mut grouped_by_pid: BTreeMap<u32, Vec<&PortInfo>> = BTreeMap::new();
    for port in filtered {
        grouped_by_pid.entry(port.pid).or_default().push(port);
    }

    let mut sys = System::new();
    sys.refresh_processes(ProcessesToUpdate::All, true);

    for (pid, ports_list) in grouped_by_pid {
        render_grouped_info(pid, &ports_list, &sys);
    }

    Ok(())
}

/// Helper function to render one process info card (with all its ports)
fn render_grouped_info(pid: u32, ports: &[&PortInfo], sys: &System) {
    let cmd_name: &str = ports.first().map(|p| p.cmd.as_str()).unwrap_or("Unknown");

    println!(
        "\n{}  Inspecting PID: {} ({})",
        "󰛨".magenta().bold(),
        pid.to_string().yellow().bold(),
        cmd_name.white().bold()
    );
    println!("{}", "───────────────────────────────────────".dimmed());

    println!(" {}", "󰆍  PROCESS INFO".blue().bold());
    println!(" ├─ Command:    {}", cmd_name.white().bold());
    println!(" ├─ PID:        {}", pid.to_string().magenta());
    if let Some(first_port) = ports.first() {
        println!(" ├─ User:       {}", first_port.user.green());
    }

    if let Some(process) = sys.process(Pid::from(pid as usize)) {
        let parent_info = if let Some(ppid) = process.parent() {
            let p_name = sys
                .process(ppid)
                .map(|p| p.name().to_string_lossy().into_owned())
                .unwrap_or_else(|| "Unknown".to_string());
            format!("{} (PPID: {})", p_name, ppid)
        } else {
            "None".to_string()
        };

        let exe_str: String = process
            .exe()
            .map(|p| p.to_string_lossy().into_owned())
            .unwrap_or_else(|| "Unknown".to_string());

        let cwd_str: String = process
            .cwd()
            .map(|p| p.to_string_lossy().into_owned())
            .unwrap_or_else(|| "Unknown".to_string());

        println!(" ├─ Executable: {}", exe_str.dimmed());
        println!(" ├─ Work Dir:   {}", cwd_str.dimmed());
        println!(" └─ Parent:     {}", parent_info.white());

        println!("\n {}", "󰓅  PERFORMANCE".blue().bold());
        let memory_mb: f64 = process.memory() as f64 / 1024.0 / 1024.0;
        println!(" ├─ Memory:     {:.1} MB", memory_mb);
        println!(" └─ CPU Load:   {:.1}%", process.cpu_usage());
    } else {
        println!(
            " └─ OS Details: {}",
            "Process metrics unavailable (terminated or permission denied)".dimmed()
        );
    }

    println!("\n {}", "󰓅  NETWORK INFO (BOUND PORTS)".blue().bold());

    for (i, port) in ports.iter().enumerate() {
        let is_last: bool = i == ports.len() - 1;
        let prefix: &str = if is_last { " └─" } else { " ├─" };

        let state_str = match &port.state {
            Some(PortState::Listen) => "LISTEN".green().bold(),
            Some(PortState::Established) => "ESTABLISHED".cyan(),
            Some(PortState::Other(s)) => s.normal(),
            None => "-".dimmed(),
        };

        let host_type_str = match port.host_type() {
            HostType::Localhost => format!("{} (localhost)", port.host).dimmed(),
            HostType::Any => format!("{} (any interface)", port.host).dimmed(),
            HostType::External => port.host.white(),
        };

        println!(
            "{} Port {}: {} | {} | {} [{}]",
            prefix,
            port.port.to_string().yellow().bold(),
            format!("{:?}", port.protocol).green(),
            state_str,
            host_type_str,
            port.fd.cyan()
        );
    }

    println!(
        "{}",
        "─────────────────────────────────────────────────────────────".dimmed()
    );
    println!(
        "{}  Hint: To terminate this process, run: {}",
        "󰁨".yellow(),
        format!("kill -9 {}", pid).white().bold()
    );
    println!();
}
