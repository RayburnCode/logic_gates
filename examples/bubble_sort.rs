//! Bubble sort implementation in RISC-V

use riscv32i_sim::{Cpu, InstructionEncoder};

fn main() {
    println!("=== Bubble Sort Demo ===\n");

    let mut cpu = Cpu::new();

    // Stub: Will implement when load/store are fully working
    let program = vec![
        (0, InstructionEncoder::i_type(0b0010011, 1, 0b000, 0, 0)), // NOP placeholder
    ];

    cpu.load_program(&program);
    cpu.reset();

    println!("Bubble sort requires:");
    println!("  - Load/Store instructions (LW, SW)");
    println!("  - Array in memory");
    println!("  - Nested loops with branches");
    println!("\nStub - to be implemented\n");
}
