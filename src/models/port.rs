use comfy_table::{Attribute, Cell, CellAlignment, Color, Row};

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
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum HostType {
    #[value(name = "localhost")]
    Localhost,
    #[value(name = "any")]
    Any,
    #[value(name = "external")]
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

impl PortInfo {
    /// Specifies the host type based on markers
    pub fn host_type(&self) -> HostType {
        match self.host.as_str() {
            "127.0.0.1" | "localhost" | "::1" | "[::1]" => HostType::Localhost,
            "*" | "0.0.0.0" | "::" | "[::]" => HostType::Any,
            _ => HostType::External,
        }
    }

    /// Helper method to build row for comfy_table based on view mode
    pub fn to_row(&self, long: bool) -> Row {
        let mut row: Row = Row::new();

        row.add_cell(Cell::new(&self.cmd).set_alignment(CellAlignment::Left));
        row.add_cell(
            Cell::new(self.pid)
                .set_alignment(CellAlignment::Right)
                .fg(Color::Magenta),
        );

        if long {
            row.add_cell(
                Cell::new(&self.user)
                    .set_alignment(CellAlignment::Left)
                    .fg(Color::Green),
            );
            row.add_cell(
                Cell::new(&self.fd)
                    .set_alignment(CellAlignment::Center)
                    .fg(Color::Cyan),
            );
            row.add_cell(
                Cell::new(self.ip_version.as_str())
                    .set_alignment(CellAlignment::Center)
                    .fg(Color::DarkYellow),
            );
        }

        row.add_cell(self.protocol.to_cell());
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
        row.add_cell(PortState::to_cell(&self.state));

        row
    }
}

impl Protocol {
    /// Generates stylized cell for Protocol
    pub fn to_cell(&self) -> Cell {
        match self {
            Self::Tcp => Cell::new("TCP").fg(Color::Blue),
            Self::Udp => Cell::new("UDP").fg(Color::DarkMagenta),
            Self::Unknown(s) => Cell::new(s).fg(Color::DarkGrey),
        }
        .set_alignment(CellAlignment::Center)
    }
}

impl PortState {
    /// Generates stylized cell for PortState
    pub fn to_cell(state: &Option<Self>) -> Cell {
        match state {
            Some(Self::Listen) => Cell::new("LISTEN")
                .fg(Color::Green)
                .add_attribute(Attribute::Bold),
            Some(Self::Established) => Cell::new("ESTABLISHED").fg(Color::Cyan),
            Some(Self::Other(s)) => Cell::new(s),
            None => Cell::new("-").fg(Color::DarkGrey),
        }
        .set_alignment(CellAlignment::Left)
    }
}

impl IPVersion {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Ipv4 => "IPv4",
            Self::Ipv6 => "IPv6",
        }
    }
}
