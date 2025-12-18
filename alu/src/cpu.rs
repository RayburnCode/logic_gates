use crate::types::*;
use crate::memory::Memory;
use crate::register_file::RegisterFile;
use crate::control_unit::ControlUnit;

/// Top-level CPU module - integrates all submodules
/// Like: module cpu(...); in SystemVerilog
pub struct Cpu {
    // Submodules
    pub memory: Memory,
    pub registers: RegisterFile,
    pub control: ControlUnit,
    
    // ALU result and flags
    alu_result: Logic32,
    alu_flags: Flags,
    
    // Pipeline state
    cycle_count: u64,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            memory: Memory::new(),
            registers: RegisterFile::new(),
            control: ControlUnit::new(),
            alu_result: 0,
            alu_flags: Flags::new(),
            cycle_count: 0,
        }
    }

    /// ALU operation - combinational
    fn execute_alu(&mut self, op: AluOp, a: Logic32, b: Logic32) {
        let (result, carry, overflow) = match op {
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

        self.alu_result = result;
        self.alu_flags.zero = result == 0;
        self.alu_flags.carry = carry;
        self.alu_flags.negative = (result & 0x8000_0000) != 0;
        self.alu_flags.overflow = overflow;
    }

    /// Single clock cycle - like always @(posedge clk)
    pub fn clock(&mut self) {
        self.cycle_count += 1;

        // Fetch instruction from memory
        let pc = self.control.get_pc();
        self.memory.clock(true, false, pc, 0);
        let instruction_word = self.memory.get_read_data();
        
        // Convert to instruction format
        let instruction = Instruction {
            opcode: (instruction_word & 0xFF) as Logic8,
            address: ((instruction_word >> 8) & 0xFFFF) as Logic16,
            flags: ((instruction_word >> 24) & 0x0F) as Bit4,
        };

        // Decode and generate control signals
        self.control.clock(instruction, self.alu_flags);
        let ctrl = self.control.get_control_signals();

        // Read registers
        let rs1 = ((instruction.flags >> 0) & 0x0F) as u8;
        let rs2 = ((instruction.flags >> 4) & 0x0F) as u8;
        self.registers.clock(rs1, 0, false, rs2);
        
        let operand_a = self.registers.get_read_data_a();
        let operand_b = self.registers.get_read_data_b();

        // Execute ALU operation
        self.execute_alu(ctrl.alu_op, operand_a, operand_b);

        // Memory access
        if ctrl.mem_read || ctrl.mem_write {
            self.memory.clock(ctrl.mem_read, ctrl.mem_write, 
                            self.alu_result, operand_b);
        }

        // Write back to register
        if ctrl.reg_write {
            let write_data = if ctrl.mem_read {
                self.memory.get_read_data()
            } else {
                self.alu_result
            };
            self.registers.clock(rs1, write_data, true, 0);
        }
    }

    pub fn run_cycles(&mut self, count: usize) {
        for _ in 0..count {
            self.clock();
        }
    }

    pub fn get_cycle_count(&self) -> u64 {
        self.cycle_count
    }

    pub fn reset(&mut self) {
        self.control.set_pc(0);
        self.cycle_count = 0;
        self.alu_result = 0;
        self.alu_flags = Flags::new();
    }

    /// Load program into instruction memory
    pub fn load_program(&mut self, program: &[(usize, Logic32)]) {
        self.memory.load_program(program);
    }
}
