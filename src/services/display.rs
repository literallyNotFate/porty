use crate::models::PortInfo;

/// Display ports in a organized table format
pub fn display_table(ports: &[PortInfo]) {
    use comfy_table::{Cell, CellAlignment, Table, TableComponent, presets};

    let mut table: Table = Table::new();
    table.load_preset(presets::NOTHING);
    table.set_style(TableComponent::HeaderLines, '─');

    table.set_header(vec![
        Cell::new("CMD").set_alignment(CellAlignment::Left),
        Cell::new("PID").set_alignment(CellAlignment::Right),
        Cell::new("PROTO").set_alignment(CellAlignment::Center),
        Cell::new("PORT").set_alignment(CellAlignment::Right),
        Cell::new("HOST").set_alignment(CellAlignment::Left),
        Cell::new("STATE").set_alignment(CellAlignment::Left),
    ]);

    for port in ports {
        table.add_row(port.to_row());
    }

    println!("\n{table}\n");
}
