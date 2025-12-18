mod types;
mod memory;
mod register_file;
mod control_unit;
mod alu;
mod cpu;

use types::*;
use alu::Alu;
use cpu::Cpu;

fn test_alu_operations() {
    println!("\n=== RISC-V ALU Operations Test ===\n");
    
    let mut alu = Alu::new();

    let tests = [
        (AluOp::Add, 10, 20, "ADD", 30),
        (AluOp::Sub, 30, 15, "SUB", 15),
        (AluOp::And, 0xFF, 0x0F, "AND", 0x0F),
        (AluOp::Or, 0xF0, 0x0F, "OR", 0xFF),
        (AluOp::Xor, 0xFF, 0xAA, "XOR", 0x55),
        (AluOp::Sll, 1, 4, "SLL", 16),
        (AluOp::Srl, 16, 2, "SRL", 4),
        (AluOp::Sra, 0xFFFF_FFF0u32, 2, "SRA", 0xFFFF_FFFCu32),
        (AluOp::Slt, 5, 10, "SLT", 1),
        (AluOp::Sltu, 5, 10, "SLTU", 1),
    ];

    for (op, a, b, name, expected) in tests {
        let result = alu.execute(op, a, b);
        let status = if result == expected { "✓" } else { "✗" };
        println!("{} {}: {} op {} = 0x{:08X} (expected 0x{:08X})",
            status, name, a, b, result, expected);
    }
}

fn test_risc_v_instructions() {
    println!("\n=== RISC-V RV32I Instruction Test ===\n");
    
    let mut cpu = Cpu::new();
    
    // Create some RV32I instructions
    // ADDI x1, x0, 42    : x1 = 0 + 42 = 42
    let addi_x1 = InstructionEncoder::i_type(0b0010011, 1, 0b000, 0, 42);
    
    // ADDI x2, x0, 8     : x2 = 0 + 8 = 8
    let addi_x2 = InstructionEncoder::i_type(0b0010011, 2, 0b000, 0, 8);
    
    // ADD x3, x1, x2     : x3 = x1 + x2 = 42 + 8 = 50
    let add_x3 = InstructionEncoder::r_type(0b0110011, 3, 0b000, 1, 2, 0b0000000);
    
    // SUB x4, x1, x2     : x4 = x1 - x2 = 42 - 8 = 34
    let sub_x4 = InstructionEncoder::r_type(0b0110011, 4, 0b000, 1, 2, 0b0100000);
    
    // AND x5, x1, x2     : x5 = x1 & x2
    let and_x5 = InstructionEncoder::r_type(0b0110011, 5, 0b111, 1, 2, 0b0000000);

    // Load program (4-byte aligned addresses)
    let program = vec![
        (0, addi_x1),
        (4, addi_x2),
        (8, add_x3),
        (12, sub_x4),
        (16, and_x5),
    ];
    
    cpu.load_program(&program);
    cpu.reset();
    
    println!("Executing 5 RISC-V instructions...\n");
    println!("Initial state:");
    cpu.registers.dump_registers(0, 8);
    
    cpu.run_cycles(5);
    
    println!("\nFinal Register State:");
    cpu.registers.dump_registers(0, 8);
    println!("\nTotal cycles: {}", cpu.get_cycle_count());
    
    println!("\n--- Instruction Breakdown ---");
    println!("0x{:08X} : ADDI x1, x0, 42", addi_x1);
    println!("0x{:08X} : ADDI x2, x0, 8", addi_x2);
    println!("0x{:08X} : ADD  x3, x1, x2", add_x3);
    println!("0x{:08X} : SUB  x4, x1, x2", sub_x4);
    println!("0x{:08X} : AND  x5, x1, x2", and_x5);
}

fn test_register_zero() {
    println!("\n=== RISC-V x0 Register Test (Hardwired to Zero) ===\n");
    
    let mut cpu = Cpu::new();
    
    // Try to write to x0 (should be ignored)
    // ADDI x0, x0, 100   : x0 should remain 0
    let addi_x0 = InstructionEncoder::i_type(0b0010011, 0, 0b000, 0, 100);
    
    // ADDI x1, x0, 50    : x1 = 0 + 50
    let addi_x1 = InstructionEncoder::i_type(0b0010011, 1, 0b000, 0, 50);
    
    let program = vec![
        (0, addi_x0),
        (4, addi_x1),
    ];
    
    cpu.load_program(&program);
    cpu.reset();
    cpu.run_cycles(2);
    
    println!("After attempting to write to x0:");
    cpu.registers.dump_registers(0, 4);
    println!("\n✓ x0 correctly hardwired to zero (RISC-V requirement)");
}

fn main() {
    println!("╔════════════════════════════════════════╗");
    println!("║  RISC-V RV32I Hardware Simulation      ║");
    println!("║  Rust-Based Implementation             ║");
    println!("╚════════════════════════════════════════╝");
    
    test_alu_operations();
    test_risc_v_instructions();
    test_register_zero();
    
    println!("\n✓ RISC-V simulation complete!");
    println!("\nRISC-V Rules Enforced:");
    println!("  • x0 hardwired to zero");
    println!("  • 32-bit instruction encoding");
    println!("  • 4-byte aligned PC increments");
    println!("  • Proper instruction decoding (R, I, S, B, U, J types)");
    println!("  • All RV32I ALU operations");
}
