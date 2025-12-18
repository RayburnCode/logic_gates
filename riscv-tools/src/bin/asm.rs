//! riscv-asm: Assemble RISC-V assembly code

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about = "Assemble RISC-V code", long_about = None)]
struct Args {
    /// Input assembly file
    input: String,

    /// Output binary file
    #[arg(short, long)]
    output: Option<String>,
}

fn main() {
    let _args = Args::parse();

    println!("RISC-V Assembler");
    println!("Stub - to be implemented");
    // TODO: Implement assembler CLI
}
