use crate::types::*;

/// RISC-V Control Unit - Instruction decoder
/// Decodes 32-bit RISC-V instructions and generates control signals
pub struct ControlUnit {
    current_instruction: Instruction,
    control_signals: ControlSignals,
    program_counter: Addr,
}

impl ControlUnit {
    pub fn new() -> Self {
        Self {
            current_instruction: Instruction::new(0),
            control_signals: ControlSignals::new(),
            program_counter: 0,
        }
    }

    /// Decode RISC-V instruction - combinational logic
    fn decode(&mut self) {
        let inst = &self.current_instruction;
        let opcode = inst.opcode();
        let funct3 = inst.funct3();
        let funct7 = inst.funct7();
        
        let mut signals = ControlSignals::new();

        match opcode {
            // LUI - Load Upper Immediate
            0b0110111 => {
                signals.alu_op = AluOp::PassB;
                signals.alu_src = true;
                signals.reg_write = true;
            }
            
            // AUIPC - Add Upper Immediate to PC
            0b0010111 => {
                signals.alu_op = AluOp::Add;
                signals.alu_src = true;
                signals.reg_write = true;
            }
            
            // JAL - Jump and Link
            0b1101111 => {
                signals.alu_op = AluOp::Add;
                signals.jump = true;
                signals.reg_write = true;
            }
            
            // JALR - Jump and Link Register
            0b1100111 => {
                signals.alu_op = AluOp::Add;
                signals.alu_src = true;
                signals.jump = true;
                signals.reg_write = true;
            }
            
            // Branch instructions
            0b1100011 => {
                signals.alu_op = AluOp::Sub;  // For comparison
                signals.branch = true;
            }
            
            // Load instructions
            0b0000011 => {
                signals.alu_op = AluOp::Add;
                signals.alu_src = true;
                signals.mem_read = true;
                signals.mem_to_reg = true;
                signals.reg_write = true;
            }
            
            // Store instructions
            0b0100011 => {
                signals.alu_op = AluOp::Add;
                signals.alu_src = true;
                signals.mem_write = true;
            }
            
            // I-type ALU operations
            0b0010011 => {
                signals.alu_src = true;
                signals.reg_write = true;
                
                signals.alu_op = match funct3 {
                    0b000 => AluOp::Add,   // ADDI
                    0b010 => AluOp::Slt,   // SLTI
                    0b011 => AluOp::Sltu,  // SLTIU
                    0b100 => AluOp::Xor,   // XORI
                    0b110 => AluOp::Or,    // ORI
                    0b111 => AluOp::And,   // ANDI
                    0b001 => AluOp::Sll,   // SLLI
                    0b101 => {
                        // SRLI or SRAI based on funct7
                        if funct7 & 0x20 != 0 {
                            AluOp::Sra
                        } else {
                            AluOp::Srl
                        }
                    }
                    _ => AluOp::Add,
                };
            }
            
            // R-type ALU operations
            0b0110011 => {
                signals.reg_write = true;
                
                signals.alu_op = match (funct3, funct7) {
                    (0b000, 0b0000000) => AluOp::Add,   // ADD
                    (0b000, 0b0100000) => AluOp::Sub,   // SUB
                    (0b001, _) => AluOp::Sll,           // SLL
                    (0b010, _) => AluOp::Slt,           // SLT
                    (0b011, _) => AluOp::Sltu,          // SLTU
                    (0b100, _) => AluOp::Xor,           // XOR
                    (0b101, 0b0000000) => AluOp::Srl,   // SRL
                    (0b101, 0b0100000) => AluOp::Sra,   // SRA
                    (0b110, _) => AluOp::Or,            // OR
                    (0b111, _) => AluOp::And,           // AND
                    _ => AluOp::Add,
                };
            }
            
            // SYSTEM (ECALL, EBREAK)
            0b1110011 => {
                // For now, treat as NOP
                signals.alu_op = AluOp::PassA;
            }
            
            _ => {
                // Unknown instruction - NOP
                signals.alu_op = AluOp::PassA;
            }
        }

        self.control_signals = signals;
    }

    /// Clock edge - fetch and decode
    /// RISC-V rule: PC must be 4-byte aligned
    pub fn clock(&mut self, instruction: Instruction) {
        self.current_instruction = instruction;
        self.decode();
    }

    /// Update PC based on control flow
    /// RISC-V rule: PC increments by 4 (instruction width)
    pub fn update_pc(&mut self, branch_taken: bool, jump_target: Addr) {
        if self.control_signals.jump || (self.control_signals.branch && branch_taken) {
            // Ensure alignment to 4-byte boundary
            self.program_counter = jump_target & !0x3;
        } else {
            // RISC-V: All instructions are 4 bytes
            self.program_counter = self.program_counter.wrapping_add(4);
        }
    }

    pub fn get_control_signals(&self) -> ControlSignals {
        self.control_signals
    }

    pub fn get_pc(&self) -> Addr {
        self.program_counter
    }

    pub fn set_pc(&mut self, pc: Addr) {
        // Enforce 4-byte alignment
        self.program_counter = pc & !0x3;
    }

    pub fn reset(&mut self) {
        self.program_counter = 0;
        self.control_signals = ControlSignals::new();
    }
}
