//! riscv-disasm: Disassemble RISC-V machine code

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about = "Disassemble RISC-V code", long_about = None)]
struct Args {
    /// Input binary file
    input: String,

    /// Show addresses
    #[arg(short, long)]
    addresses: bool,
}

fn main() {
    let _args = Args::parse();

    println!("RISC-V Disassembler");
    println!("Stub - to be implemented");
    // TODO: Implement disassembler CLI
}
