//! Output formatting utilities

use prettytable::{Table, Row, Cell, format};
use colored::Colorize;

pub fn format_register_table(regs: &[(String, u32)]) -> Table {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_BOX_CHARS);
    
    table.add_row(Row::new(vec![
        Cell::new("Register").style_spec("Fb"),
        Cell::new("Value (Hex)").style_spec("Fb"),
        Cell::new("Value (Dec)").style_spec("Fb"),
    ]));

    for (name, value) in regs {
        table.add_row(Row::new(vec![
            Cell::new(name),
            Cell::new(&format!("0x{:08X}", value)),
            Cell::new(&format!("{}", *value as i32)),
        ]));
    }

    table
}

pub fn colorize_instruction(asm: &str) -> String {
    // Simple colorization
    let parts: Vec<&str> = asm.split_whitespace().collect();
    if parts.is_empty() {
        return asm.to_string();
    }

    let mnemonic = parts[0].blue().bold();
    let rest = parts[1..].join(" ");
    
    format!("{} {}", mnemonic, rest)
}
