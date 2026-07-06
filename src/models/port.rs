use comfy_table::Row;
use std::{convert::Infallible, str::FromStr};

/// Used trasport protocols
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Protocol {
    Tcp,
    Udp,
    Unknown(String),
}

/// State of the port
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PortState {
    Listen,
    Established,
    Other(String),
}

/// Main port info structure
#[derive(Debug, Clone)]
pub struct PortInfo {
    pub cmd: String,
    pub pid: u32,
    pub protocol: Protocol,
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
