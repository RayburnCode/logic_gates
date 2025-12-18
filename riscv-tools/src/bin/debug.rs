//! riscv-debug: Interactive debugger

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about = "Interactive RISC-V debugger", long_about = None)]
struct Args {
    /// Program to debug
    file: String,
}

fn main() {
    let _args = Args::parse();

    println!("RISC-V Interactive Debugger");
    println!("Stub - to be implemented");
    println!("\nPlanned commands:");
    println!("  step (s)      - Execute one instruction");
    println!("  continue (c)  - Run until breakpoint");
    println!("  break (b)     - Set breakpoint");
    println!("  registers (r) - Show register state");
    println!("  memory (m)    - Show memory");
    println!("  quit (q)      - Exit debugger");
}
