use crate::models::{HostType, IPVersion, PortState, ProcessOwner, Protocol};

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
