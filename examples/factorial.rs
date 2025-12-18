//! Factorial calculator in RISC-V

use riscv32i_sim::{Cpu, InstructionEncoder};

fn main() {
    println!("=== Factorial Calculator Demo ===\n");

    let mut cpu = Cpu::new();

    // Calculate 5! = 120
    let program = vec![
        // x1 = result (starts at 1)
        // x2 = n (starts at 5)
        (0,  InstructionEncoder::i_type(0b0010011, 1, 0b000, 0, 1)),   // x1 = 1
        (4,  InstructionEncoder::i_type(0b0010011, 2, 0b000, 0, 5)),   // x2 = 5
        
        // Loop: multiply result by n, decrement n
        // TODO: Need M extension for MUL instruction
        // For now, this is a stub showing the structure
    ];

    cpu.load_program(&program);
    cpu.reset();

    println!("Calculating 5! ...\n");
    println!("Note: Requires M extension (multiplication) - stub only\n");

    cpu.run_cycles(10);

    println!("Register state:");
    cpu.registers.dump_registers(0, 3);
}
