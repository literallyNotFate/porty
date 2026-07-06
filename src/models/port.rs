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
    pub process: String,
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
