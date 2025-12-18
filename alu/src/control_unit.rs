use crate::types::*;

/// Control signals - like a SystemVerilog packed struct of control bits
#[derive(Debug, Clone, Copy)]
pub struct ControlSignals {
    pub alu_op: AluOp,
    pub reg_write: bool,
    pub mem_read: bool,
    pub mem_write: bool,
    pub branch: bool,
    pub jump: bool,
}

/// Instruction decoder and control unit
pub struct ControlUnit {
    current_instruction: Instruction,
    control_signals: ControlSignals,
    program_counter: Logic32,
}

impl ControlUnit {
    pub fn new() -> Self {
        Self {
            current_instruction: Instruction {
                opcode: 0,
                address: 0,
                flags: 0,
            },
            control_signals: ControlSignals {
                alu_op: AluOp::Nop,
                reg_write: false,
                mem_read: false,
                mem_write: false,
                branch: false,
                jump: false,
            },
            program_counter: 0,
        }
    }

    /// Decode instruction - combinational logic
    fn decode(&mut self) {
        // Decode opcode to control signals
        let opcode = self.current_instruction.opcode;
        
        // Default control signals
        let mut signals = ControlSignals {
            alu_op: AluOp::Nop,
            reg_write: false,
            mem_read: false,
            mem_write: false,
            branch: false,
            jump: false,
        };

        // Instruction format decode
        match opcode {
            0x00 => { // NOP
                signals.alu_op = AluOp::Nop;
            }
            0x01 => { // ADD
                signals.alu_op = AluOp::Add;
                signals.reg_write = true;
            }
            0x02 => { // SUB
                signals.alu_op = AluOp::Sub;
                signals.reg_write = true;
            }
            0x03 => { // AND
                signals.alu_op = AluOp::And;
                signals.reg_write = true;
            }
            0x04 => { // OR
                signals.alu_op = AluOp::Or;
                signals.reg_write = true;
            }
            0x05 => { // XOR
                signals.alu_op = AluOp::Xor;
                signals.reg_write = true;
            }
            0x10 => { // LOAD from memory
                signals.alu_op = AluOp::Add;
                signals.mem_read = true;
                signals.reg_write = true;
            }
            0x11 => { // STORE to memory
                signals.alu_op = AluOp::Add;
                signals.mem_write = true;
            }
            0x20 => { // BRANCH if zero
                signals.alu_op = AluOp::Sub;
                signals.branch = true;
            }
            0x21 => { // JUMP
                signals.jump = true;
            }
            _ => {
                // Unknown instruction - default to NOP
            }
        }

        self.control_signals = signals;
    }

    /// Clock edge - fetch and decode
    pub fn clock(&mut self, instruction: Instruction, flags: Flags) {
        self.current_instruction = instruction;
        self.decode();

        // Update program counter
        if self.control_signals.jump {
            self.program_counter = self.current_instruction.address as Logic32;
        } else if self.control_signals.branch && flags.zero {
            self.program_counter = self.current_instruction.address as Logic32;
        } else {
            self.program_counter += 1;
        }
    }

    pub fn get_control_signals(&self) -> ControlSignals {
        self.control_signals
    }

    pub fn get_pc(&self) -> Logic32 {
        self.program_counter
    }

    pub fn set_pc(&mut self, pc: Logic32) {
        self.program_counter = pc;
    }
}
