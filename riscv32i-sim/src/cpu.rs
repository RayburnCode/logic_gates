use crate::types::*;
use crate::memory::Memory;
use crate::register_file::RegisterFile;
use crate::control_unit::ControlUnit;
use crate::alu::Alu;

/// RISC-V CPU - integrates all submodules
/// Implements RV32I base integer instruction set
pub struct Cpu {
    // Submodules
    pub memory: Memory,
    pub registers: RegisterFile,
    pub control: ControlUnit,
    pub alu: Alu,
    
    // Pipeline state
    cycle_count: u64,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            memory: Memory::new(),
            registers: RegisterFile::new(),
            control: ControlUnit::new(),
            alu: Alu::new(),
            cycle_count: 0,
        }
    }

    /// Single clock cycle - RISC-V fetch-decode-execute
    pub fn clock(&mut self) {
        self.cycle_count += 1;

        // FETCH: Get instruction at PC
        // RISC-V: PC is byte-addressed, instructions are 4-byte aligned
        let pc = self.control.get_pc();
        let instruction_word = self.memory.fetch(pc);
        let inst = Instruction::new(instruction_word);

        // DECODE: Generate control signals
        self.control.clock(inst);
        let ctrl = self.control.get_control_signals();

        // READ REGISTERS: Read rs1 and rs2
        let rs1 = inst.rs1();
        let rs2 = inst.rs2();
        let rd = inst.rd();
        
        self.registers.clock(rs1, 0, false, rs2);
        let rs1_data = self.registers.get_read_data_a();
        let rs2_data = self.registers.get_read_data_b();

        // EXECUTE: ALU operation
        let alu_operand_b = if ctrl.alu_src {
            // Use immediate value
            inst.imm_i() as Word
        } else {
            // Use rs2
            rs2_data
        };

        // Special handling for AUIPC (add upper immediate to PC)
        let alu_operand_a = if inst.opcode() == 0b0010111 {
            pc  // AUIPC uses PC as operand A
        } else if inst.opcode() == 0b0110111 {
            0   // LUI uses 0 as operand A
        } else {
            rs1_data
        };

        let alu_result = self.alu.execute(ctrl.alu_op, alu_operand_a, alu_operand_b);

        // MEMORY: Load/Store operations
        let mut mem_data = 0;
        if ctrl.mem_read || ctrl.mem_write {
            self.memory.clock(ctrl.mem_read, ctrl.mem_write, alu_result, rs2_data);
            mem_data = self.memory.get_read_data();
        }

        // WRITE BACK: Write result to register
        if ctrl.reg_write {
            let write_data = if ctrl.mem_to_reg {
                mem_data
            } else if ctrl.jump {
                // JAL/JALR: Save return address (PC + 4)
                pc.wrapping_add(4)
            } else {
                alu_result
            };
            
            self.registers.clock(rd, write_data, true, 0);
        }

        // UPDATE PC
        let branch_taken = self.should_branch(&inst, rs1_data, rs2_data);
        let jump_target = self.calculate_jump_target(&inst, pc, rs1_data);
        self.control.update_pc(branch_taken, jump_target);
    }

    /// Determine if branch should be taken (RISC-V branch conditions)
    fn should_branch(&self, inst: &Instruction, rs1_data: Word, rs2_data: Word) -> bool {
        if inst.opcode() != 0b1100011 {
            return false;  // Not a branch instruction
        }

        match inst.funct3() {
            0b000 => rs1_data == rs2_data,                  // BEQ
            0b001 => rs1_data != rs2_data,                  // BNE
            0b100 => (rs1_data as i32) < (rs2_data as i32), // BLT
            0b101 => (rs1_data as i32) >= (rs2_data as i32),// BGE
            0b110 => rs1_data < rs2_data,                   // BLTU
            0b111 => rs1_data >= rs2_data,                  // BGEU
            _ => false,
        }
    }

    /// Calculate jump/branch target address
    fn calculate_jump_target(&self, inst: &Instruction, pc: Addr, rs1_data: Word) -> Addr {
        match inst.opcode() {
            0b1101111 => pc.wrapping_add(inst.imm_j() as u32),        // JAL
            0b1100111 => rs1_data.wrapping_add(inst.imm_i() as u32) & !1, // JALR (bit 0 = 0)
            0b1100011 => pc.wrapping_add(inst.imm_b() as u32),        // Branch
            _ => pc.wrapping_add(4),
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
        self.control.reset();
        self.registers.reset();
        self.memory.reset();
        self.cycle_count = 0;
    }

    /// Load RISC-V program into memory
    pub fn load_program(&mut self, program: &[(Addr, Word)]) {
        self.memory.load_program(program);
    }
}
