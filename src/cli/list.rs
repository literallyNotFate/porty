use crate::models::{PortState, Protocol};
use clap::Args;

#[derive(Args)]
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
    #[arg(long, value_parser = ["localhost", "any", "external"])]
    pub host: Option<String>,

    /// Exclude a specific port from results
    #[arg(long)]
    pub exclude: Option<u16>,
}
