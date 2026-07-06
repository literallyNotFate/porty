use crate::models::{PortInfo, PortState, Protocol};

/// Parse raw `lsof` output
pub fn parse(raw: &str) -> Vec<PortInfo> {
    raw.lines().skip(1).filter_map(parse_line).collect()
}

/// Parse single output row
fn parse_line(line: &str) -> Option<PortInfo> {
    let fields: Vec<&str> = line.split_whitespace().collect();
    if fields.len() < 9 {
        return None;
    }

    let cmd: String = fields[0].to_string();
    let pid: u32 = fields[1].parse::<u32>().ok()?;
    let protocol: Protocol = fields[7].parse::<Protocol>().ok()?;

    let raw_addr: String = fields[8..].join(" ");
    let (host, port, state) = parse_addr(&raw_addr)?;

    Some(PortInfo {
        cmd,
        pid,
        protocol,
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
            let state: Option<PortState> = state_str.parse::<PortState>().ok();
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
