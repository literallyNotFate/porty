use crate::models::{HostType, IPVersion, PortInfo, PortState, Protocol};

#[derive(clap::Args)]
pub struct ListArgs {
    /// Smart search: matches port number, PID, or process name (e.g., "3000", "node")
    pub query: Option<String>,

    /// Filter by network protocol
    #[arg(long, value_enum)]
    pub proto: Option<Protocol>,

    /// Filter by connection state
    #[arg(long, value_enum)]
    pub state: Option<PortState>,

    /// Filter by host visibility
    #[arg(long, value_enum)]
    pub host: Option<HostType>,

    /// Filter by IP version
    #[arg(long = "ip", value_enum)]
    pub ip_version: Option<IPVersion>,

    /// Filter by process owner (root or your current system user)
    #[arg(short, long, value_enum)]
    pub user: Option<ProcessOwner>,

    /// Exclude a specific port from results
    #[arg(long)]
    pub exclude: Option<u16>,

    /// Display extended information (includes USER, FD, TYPE columns)
    #[arg(short, long)]
    pub long: bool,
}

/// Process owner
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum ProcessOwner {
    #[value(name = "root")]
    Root,
    #[value(name = "current")]
    Current,
}

impl ListArgs {
    /// Filter ports
    pub fn filter(&self, ports: &mut Vec<PortInfo>) {
        let current_user: String = std::env::var("USER").unwrap_or_default();
        let query_low: Option<String> = self.query.as_ref().map(|q| q.to_lowercase());
        let query_num: Option<u32> = self.query.as_ref().and_then(|q| q.parse::<u32>().ok());

        ports.retain(|p| {
            if let Some(ref q_low) = query_low {
                let num_match = query_num.map_or(false, |num| p.port == num as u16 || p.pid == num);
                let str_match: bool = p.cmd.to_lowercase().contains(q_low);
                if !num_match && !str_match {
                    return false;
                }
            }
            if let Some(ref target_proto) = self.proto {
                if p.protocol != *target_proto {
                    return false;
                }
            }
            if let Some(ref target_state) = self.state {
                if p.state.as_ref() != Some(target_state) {
                    return false;
                }
            }
            if let Some(ref target_ip_ver) = self.ip_version {
                if p.ip_version != *target_ip_ver {
                    return false;
                }
            }
            if let Some(ref user_filter) = self.user {
                let matches_user: bool = match user_filter {
                    ProcessOwner::Root => p.user == "root",
                    ProcessOwner::Current => p.user == current_user,
                };
                if !matches_user {
                    return false;
                }
            }
            if let Some(ref host_scope) = self.host {
                if p.host_type() != *host_scope {
                    return false;
                }
            }
            if let Some(exclude_port) = self.exclude {
                if p.port == exclude_port {
                    return false;
                }
            }

            true
        });
    }
}
