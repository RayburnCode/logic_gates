mod types;
mod memory;
mod register_file;
mod control_unit;
mod cpu;

use types::*;
use cpu::Cpu;

/// ALU module - like a SystemVerilog module
pub struct Alu {
    // Registers (like reg in SystemVerilog)
    accumulator: Logic32,
    result: Logic32,
    flags: Flags,
    state: AluState,
    
    // Internal memory (like logic[7:0] mem[0:255])
    registers: [Logic32; 16],
}

impl Alu {
    /// Constructor - like module instantiation
    pub fn new() -> Self {
        Self {
            accumulator: 0,
            result: 0,
            flags: Flags::new(),
            state: AluState::Idle,
            registers: [0; 16],
        }
    }

    /// Reset - like reset logic in always block
    pub fn reset(&mut self) {
        self.accumulator = 0;
        self.result = 0;
        self.flags = Flags::new();
        self.state = AluState::Idle;
        self.registers = [0; 16];
    }

    /// Clock edge - like always @(posedge clk)
    pub fn clock(&mut self, instruction: Instruction) {
        match self.state {
            AluState::Idle => {
                self.state = AluState::Fetch;
            }
            AluState::Fetch => {
                self.state = AluState::Execute;
            }
            AluState::Execute => {
                self.execute(instruction);
                self.state = AluState::WriteBack;
            }
            AluState::WriteBack => {
                self.accumulator = self.result;
                self.state = AluState::Idle;
            }
        }
    }

    /// Combinational logic - like always @(*)
    fn execute(&mut self, instruction: Instruction) {
        let opcode = unsafe { std::mem::transmute::<Logic8, AluOp>(instruction.opcode) };
        let a = self.accumulator;
        let b = instruction.address as Logic32;

        // Combinational logic for ALU operations
        let (result, carry, overflow) = match opcode {
            AluOp::Nop => (a, false, false),
            AluOp::Add => {
                let (res, c) = a.overflowing_add(b);
                let v = ((a ^ res) & (b ^ res)) & 0x8000_0000 != 0;
                (res, c, v)
            }
            AluOp::Sub => {
                let (res, c) = a.overflowing_sub(b);
                let v = ((a ^ b) & (a ^ res)) & 0x8000_0000 != 0;
                (res, c, v)
            }
            AluOp::And => (a & b, false, false),
            AluOp::Or => (a | b, false, false),
            AluOp::Xor => (a ^ b, false, false),
            AluOp::Not => (!a, false, false),
            AluOp::Shl => (a << (b & 0x1F), false, false),
            AluOp::Shr => (a >> (b & 0x1F), false, false),
        };

        self.result = result;
        
        // Flag updates - like flag register logic
        self.flags.zero = result == 0;
        self.flags.carry = carry;
        self.flags.negative = (result & 0x8000_0000) != 0;
        self.flags.overflow = overflow;
    }

    /// Read operation - like continuous assignment
    pub fn get_result(&self) -> Logic32 {
        self.result
    }

    pub fn get_flags(&self) -> Flags {
        self.flags
    }

    pub fn get_state(&self) -> AluState {
        self.state
    }

    /// Register file access - like memory operations
    pub fn read_register(&self, addr: u8) -> Logic32 {
        if (addr as usize) < self.registers.len() {
            self.registers[addr as usize]
        } else {
            0
        }
    }

    pub fn write_register(&mut self, addr: u8, value: Logic32) {
        if (addr as usize) < self.registers.len() {
            self.registers[addr as usize] = value;
        }
    }
}

fn test_standalone_alu() {
    println!("\n=== Standalone ALU Test ===\n");
    
    let mut alu = Alu::new();
    alu.reset();

    let tests = [
        (AluOp::Add, 10, 20, "ADD"),
        (AluOp::Sub, 30, 15, "SUB"),
        (AluOp::And, 0xFF, 0x0F, "AND"),
        (AluOp::Or, 0xF0, 0x0F, "OR"),
        (AluOp::Xor, 0xFF, 0xAA, "XOR"),
        (AluOp::Not, 0xFF, 0, "NOT"),
        (AluOp::Shl, 1, 4, "SHL"),
        (AluOp::Shr, 16, 2, "SHR"),
    ];

    for (op, a, b, name) in tests {
        alu.accumulator = a;
        
        let instruction = Instruction {
            opcode: op as Logic8,
            address: b as Logic16,
            flags: 0,
        };

        for _ in 0..4 {
            alu.clock(instruction);
        }

        let result = alu.get_result();
        let flags = alu.get_flags();

        println!("{}: {} and {} = {} [Z={} C={} N={} V={}]",
            name, a, b, result,
            flags.zero as u8,
            flags.carry as u8,
            flags.negative as u8,
            flags.overflow as u8
        );
    }
}

fn test_integrated_cpu() {
    println!("\n=== Integrated CPU Test ===\n");
    
    let mut cpu = Cpu::new();
    
    // Simple program: ADD instructions
    // Format: [opcode (8) | address (16) | flags (8)]
    let program = vec![
        (0, 0x01_0014_00), // ADD R0, R0, #20
        (1, 0x01_001E_00), // ADD R0, R0, #30
        (2, 0x02_000A_00), // SUB R0, R0, #10
        (3, 0x00_0000_00), // NOP
    ];
    
    cpu.load_program(&program);
    cpu.reset();
    
    println!("Running program for 4 cycles...");
    cpu.run_cycles(4);
    
    println!("\nFinal Register State:");
    cpu.registers.dump_registers(0, 8);
    println!("\nTotal cycles: {}", cpu.get_cycle_count());
}

fn main() {
    println!("╔════════════════════════════════════════╗");
    println!("║  Rust-Based Hardware Simulation        ║");
    println!("║  SystemVerilog-Inspired Design         ║");
    println!("╚════════════════════════════════════════╝");
    
    test_standalone_alu();
    test_integrated_cpu();
    
    println!("\n✓ Simulation complete!");
}
