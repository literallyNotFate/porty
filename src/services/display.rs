use crate::models::PortInfo;

/// Display ports in an organized table format
pub fn display_table(ports: &[PortInfo], long: bool) {
    use comfy_table::{Cell, CellAlignment, Table, TableComponent, presets};

    let mut table: Table = Table::new();
    table.load_preset(presets::NOTHING);
    table.set_style(TableComponent::HeaderLines, '─');

    let mut header = vec![
        Cell::new("CMD").set_alignment(CellAlignment::Left),
        Cell::new("PID").set_alignment(CellAlignment::Right),
    ];

    if long {
        header.extend(vec![
            Cell::new("USER").set_alignment(CellAlignment::Left),
            Cell::new("FD").set_alignment(CellAlignment::Center),
            Cell::new("TYPE").set_alignment(CellAlignment::Center),
        ]);
    }

    header.extend(vec![
        Cell::new("PROTO").set_alignment(CellAlignment::Center),
        Cell::new("PORT").set_alignment(CellAlignment::Right),
        Cell::new("HOST").set_alignment(CellAlignment::Left),
        Cell::new("STATE").set_alignment(CellAlignment::Left),
    ]);

    table.set_header(header);
    for port in ports {
        table.add_row(port.to_row(long));
    }

    println!("\n{table}\n");
}
