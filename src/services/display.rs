use crate::models::{PortInfo, PortState};

/// Display ports in an organized table format
pub fn display_table(ports: &[PortInfo], long: bool) {
    use comfy_table::{ContentArrangement, Row, Table, TableComponent, presets};

    let mut table: Table = Table::new();
    table.load_preset(presets::NOTHING);
    table.set_style(TableComponent::HeaderLines, '─');
    table.set_content_arrangement(ContentArrangement::Dynamic);

    if !colored::control::SHOULD_COLORIZE.should_colorize() {
        table.force_no_tty();
    }

    let header: Row = build_header(long);
    table.set_header(header);

    for port in ports {
        table.add_row(port.to_row(long));
    }

    println!("\n{table}\n");
    render_summary(ports);
}

/// Build main table header
fn build_header(long: bool) -> comfy_table::Row {
    use comfy_table::{Cell, CellAlignment::*};
    let mut header: Vec<Cell> = vec![
        Cell::new("CMD").set_alignment(Left),
        Cell::new("PID").set_alignment(Right),
    ];

    if long {
        header.extend(vec![
            Cell::new("USER").set_alignment(Left),
            Cell::new("FD").set_alignment(Center),
            Cell::new("TYPE").set_alignment(Center),
        ]);
    }

    header.extend(vec![
        Cell::new("PROTO").set_alignment(Center),
        Cell::new("PORT").set_alignment(Right),
        Cell::new("HOST").set_alignment(Left),
        Cell::new("STATE").set_alignment(Left),
    ]);

    comfy_table::Row::from(header)
}

/// Render summary
fn render_summary(ports: &[PortInfo]) {
    use colored::Colorize;
    let listen_count: usize = ports
        .iter()
        .filter(|p| matches!(p.state, Some(PortState::Listen)))
        .count();
    let estab_count: usize = ports
        .iter()
        .filter(|p| matches!(p.state, Some(PortState::Established)))
        .count();

    println!(
        " {}  Total: {} active ports ({} listening, {} established)\n",
        "󱌢".magenta().bold(),
        ports.len().to_string().yellow().bold(),
        listen_count.to_string().green().bold(),
        estab_count.to_string().cyan().bold()
    );
}
