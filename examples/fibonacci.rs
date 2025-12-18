//! Fibonacci sequence calculator in RISC-V
//! 
//! Calculates the first N Fibonacci numbers

use riscv32i_sim::{Cpu, InstructionEncoder};

fn main() {
    println!("=== Fibonacci Sequence Demo ===\n");

    let mut cpu = Cpu::new();

    // RISC-V program to calculate Fibonacci
    let program = vec![
        // Initialize: fib(0)=1, fib(1)=1, counter=10
        (0,  InstructionEncoder::i_type(0b0010011, 1, 0b000, 0, 1)),      // x1 = 1 (fib n-2)
        (4,  InstructionEncoder::i_type(0b0010011, 2, 0b000, 0, 1)),      // x2 = 1 (fib n-1)
        (8,  InstructionEncoder::i_type(0b0010011, 3, 0b000, 0, 10)),     // x3 = 10 (counter)
        
        // Loop: calculate next fibonacci
        (12, InstructionEncoder::r_type(0b0110011, 4, 0b000, 1, 2, 0b0000000)), // x4 = x1 + x2
        (16, InstructionEncoder::r_type(0b0110011, 1, 0b000, 2, 0, 0b0000000)), // x1 = x2
        (20, InstructionEncoder::r_type(0b0110011, 2, 0b000, 4, 0, 0b0000000)), // x2 = x4
        (24, InstructionEncoder::i_type(0b0010011, 3, 0b000, 3, -1)),           // x3 = x3 - 1
        
        // Branch if not zero
        (28, InstructionEncoder::r_type(0b1100011, 0b001, 0, 3, 0, 0) | ((-16i32 as u32 & 0x1000) << 19) | ((-16i32 as u32 & 0x7E0) << 20) | ((-16i32 as u32 & 0x1E) << 7) | ((-16i32 as u32 & 0x800) >> 4)),
    ];

    cpu.load_program(&program);
    cpu.reset();

    println!("Calculating 10 Fibonacci numbers...\n");
    
    // Run for enough cycles to complete
    cpu.run_cycles(100);

    println!("Results:");
    println!("  Cycles executed: {}", cpu.get_cycle_count());
    println!("  Final result in x2: {}\n", cpu.registers.get_read_data_a());

    println!("Register state:");
    cpu.registers.dump_registers(0, 5);

    println!("\n✓ Fibonacci calculation complete!");
}
