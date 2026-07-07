use crate::models::{IPVersion, PortInfo, PortState, Protocol};
use std::{convert::Infallible, str::FromStr};

/// Parse raw `lsof` output into a list of active ports
pub fn parse(raw: &str) -> Vec<PortInfo> {
    raw.lines()
        .skip(1)
        .filter_map(|line| parse_line(line))
        .collect()
}

/// Parse single output row
fn parse_line(line: &str) -> Option<PortInfo> {
    let fields: Vec<&str> = line.split_whitespace().collect();
    if fields.len() < 9 {
        return None;
    }

    let protocol: Protocol = parse_from_str::<Protocol>(fields[7])?;
    let ip_version: IPVersion = parse_from_str::<IPVersion>(fields[4]).unwrap_or(IPVersion::Ipv4);

    let raw_addr: String = fields[8..].join(" ");
    let (host, port, state) = parse_addr(&raw_addr)?;

    Some(PortInfo {
        cmd: fields[0].to_string(),
        pid: fields[1].parse().ok()?,
        user: fields[2].to_string(),
        fd: fields[3].to_string(),
        protocol,
        ip_version,
        port,
        host,
        state,
    })
}

/// Parse row address
fn parse_addr(raw: &str) -> Option<(String, u16, Option<PortState>)> {
    let (name, state) = match raw.find(" (") {
        Some(idx) => {
            let state_str: &str = raw[idx + 2..].trim_end_matches(')');
            let state: Option<PortState> = parse_from_str::<PortState>(state_str);
            (&raw[..idx], state)
        }
        None => (raw, None),
    };

    let local: &str = match name.find("->") {
        Some(idx) => &name[..idx],
        None => name,
    };

    let idx: usize = local.rfind(':')?;
    let host: String = local[..idx].to_string();
    let port: u16 = local[idx + 1..].parse::<u16>().ok()?;

    let state: Option<PortState> = if name.contains("->") {
        Some(PortState::Established)
    } else {
        state
    };

    Some((host, port, state))
}

/// Helper string parser (instead of FromStr in models)
fn parse_from_str<T: FromStr>(s: &str) -> Option<T> {
    s.parse().ok()
}

/// Models parsing

impl FromStr for Protocol {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "UDP" => Ok(Self::Udp),
            "TCP" => Ok(Self::Tcp),
            _ => Ok(Self::Unknown(s.into())),
        }
    }
}

impl FromStr for PortState {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "LISTEN" => Ok(Self::Listen),
            "ESTABLISHED" => Ok(Self::Established),
            other => Ok(Self::Other(other.into())),
        }
    }
}

impl FromStr for IPVersion {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "IPV4" | "4" => Ok(Self::Ipv4),
            "IPV6" | "6" => Ok(Self::Ipv6),
            _ => Err("Unknown IP version"),
        }
    }
}
