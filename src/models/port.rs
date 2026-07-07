use comfy_table::Row;
use std::{convert::Infallible, str::FromStr};

/// Used transport protocols
#[derive(Debug, Clone, PartialEq, Eq, clap::ValueEnum)]
pub enum Protocol {
    #[value(name = "tcp")]
    Tcp,
    #[value(name = "udp")]
    Udp,
    #[value(skip)]
    Unknown(String),
}

/// IP version
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum IPVersion {
    #[value(name = "ipv4")]
    Ipv4,
    #[value(name = "ipv6")]
    Ipv6,
}

/// Process owner
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum ProcessOwner {
    #[value(name = "root")]
    Root,
    #[value(name = "current")]
    Current,
}

/// State of the port
#[derive(Debug, Clone, PartialEq, Eq, clap::ValueEnum)]
pub enum PortState {
    #[value(name = "listen")]
    Listen,
    #[value(name = "established")]
    Established,
    #[value(skip)]
    Other(String),
}

/// Port host type
#[derive(Debug, PartialEq)]
pub enum HostType {
    Localhost,
    Any,
    External,
}

/// Main port info structure
#[derive(Debug, Clone)]
pub struct PortInfo {
    pub cmd: String,
    pub pid: u32,
    pub protocol: Protocol,
    pub user: String,
    pub ip_version: IPVersion,
    pub fd: String,
    pub port: u16,
    pub host: String,
    pub state: Option<PortState>,
}

impl FromStr for Protocol {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "UDP" => Ok(Self::Udp),
            "TCP" => Ok(Self::Tcp),
            _ => Ok(Self::Unknown(s.into())),
        }
    }
}

impl FromStr for PortState {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "LISTEN" => Ok(Self::Listen),
            "ESTABLISHED" => Ok(Self::Established),
            other => Ok(Self::Other(other.into())),
        }
    }
}

impl PortInfo {
    /// Specifies the host type based on markers
    pub fn host_type(&self) -> HostType {
        match self.host.as_str() {
            "127.0.0.1" | "localhost" | "::1" | "[::1]" => HostType::Localhost,
            "*" | "0.0.0.0" | "::" | "[::]" => HostType::Any,
            _ => HostType::External,
        }
    }

    /// Helper method to build row for comfy_table
    pub fn to_row(&self) -> Row {
        use comfy_table::{Attribute, Cell, CellAlignment, Color, Row};

        let mut row: Row = Row::new();

        row.add_cell(Cell::new(&self.cmd).set_alignment(CellAlignment::Left));
        row.add_cell(
            Cell::new(self.pid)
                .set_alignment(CellAlignment::Right)
                .fg(Color::Magenta),
        );

        let proto_cell: Cell = match &self.protocol {
            Protocol::Tcp => Cell::new("TCP").fg(Color::Blue),
            Protocol::Udp => Cell::new("UDP").fg(Color::DarkMagenta),
            Protocol::Unknown(s) => Cell::new(s).fg(Color::DarkGrey),
        };
        row.add_cell(proto_cell.set_alignment(CellAlignment::Center));

        row.add_cell(
            Cell::new(self.port)
                .set_alignment(CellAlignment::Right)
                .fg(Color::Yellow),
        );

        row.add_cell(
            Cell::new(&self.host)
                .set_alignment(CellAlignment::Left)
                .fg(Color::DarkGrey),
        );

        let state_cell: Cell = match &self.state {
            Some(PortState::Listen) => Cell::new("LISTEN")
                .fg(Color::Green)
                .add_attribute(Attribute::Bold),
            Some(PortState::Established) => Cell::new("ESTABLISHED").fg(Color::Cyan),
            Some(PortState::Other(s)) => Cell::new(s),
            None => Cell::new("-").fg(Color::DarkGrey),
        };
        row.add_cell(state_cell.set_alignment(CellAlignment::Left));

        row
    }
}
