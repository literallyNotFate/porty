use clap::Args;

#[derive(Args)]
pub struct ListArgs {
    /// Show only listening ports
    #[arg(long)]
    pub listen_only: bool,

    /// Show only TCP ports
    #[arg(long)]
    pub tcp: bool,

    /// Show only UDP ports
    #[arg(long)]
    pub udp: bool,
}
