use crate::models::PortInfo;

/// Display ports in an organized table format
pub fn display_table(ports: &[PortInfo], long: bool) {
    use comfy_table::{Row, Table, TableComponent, presets};

    let mut table: Table = Table::new();
    table.load_preset(presets::NOTHING);
    table.set_style(TableComponent::HeaderLines, '─');

    let mut header = vec![
        Cell::new("CMD").set_alignment(CellAlignment::Left),
        Cell::new("PID").set_alignment(CellAlignment::Right),
    let header: Row = build_header(long);
    table.set_header(header);

    for port in ports {
        table.add_row(port.to_row(long));
    }

    println!("\n{table}\n");
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
