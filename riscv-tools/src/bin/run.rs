//! riscv-run: Execute RISC-V programs

use clap::Parser;
use colored::Colorize;
use riscv32i_sim::Cpu;

#[derive(Parser)]
#[command(author, version, about = "Execute RISC-V programs", long_about = None)]
struct Args {
    /// Assembly file to run
    #[arg(short, long)]
    file: Option<String>,

    /// Show register state after execution
    #[arg(short, long)]
    registers: bool,

    /// Show execution trace
    #[arg(short, long)]
    trace: bool,

    /// Maximum cycles to execute
    #[arg(short = 'c', long, default_value = "1000")]
    max_cycles: usize,
}

fn main() {
    let args = Args::parse();

    println!("{}", "RISC-V Simulator".green().bold());
    println!("{}", "=".repeat(50));

    let mut cpu = Cpu::new();
    cpu.reset();

    if let Some(_file) = args.file {
        println!("{}", "Loading program...".yellow());
        // TODO: Load from file
        println!("{}", "File loading not yet implemented".red());
        return;
    }

    println!("{}", "Running demo program...".cyan());
    println!("{}", "=".repeat(50));

    cpu.run_cycles(args.max_cycles);

    println!("\n{}", format!("Executed {} cycles", cpu.get_cycle_count()).green());

    if args.registers {
        println!("\n{}", "Register State:".blue().bold());
        cpu.registers.dump_registers(0, 32);
    }
}
