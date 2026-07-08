#[derive(clap::Args)]
pub struct InfoArgs {
    /// The port number or Process ID (PID) to inspect
    #[arg(value_name = "TARGET")]
    pub target: String,
}
