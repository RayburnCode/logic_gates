use crate::types::*;

/// RISC-V ALU - Arithmetic Logic Unit
/// Implements all RV32I ALU operations
pub struct Alu {
    result: Word,
    zero: bool,
}

impl Alu {
    pub fn new() -> Self {
        Self {
            result: 0,
            zero: false,
        }
    }

    /// Execute ALU operation - combinational logic
    pub fn execute(&mut self, op: AluOp, a: Word, b: Word) -> Word {
        let result = match op {
            AluOp::Add => a.wrapping_add(b),
            AluOp::Sub => a.wrapping_sub(b),
            AluOp::And => a & b,
            AluOp::Or => a | b,
            AluOp::Xor => a ^ b,
            AluOp::Sll => a << (b & 0x1F),  // Only use lower 5 bits
            AluOp::Srl => a >> (b & 0x1F),
            AluOp::Sra => ((a as i32) >> (b & 0x1F)) as u32,
            AluOp::Slt => {
                if (a as i32) < (b as i32) { 1 } else { 0 }
            }
            AluOp::Sltu => {
                if a < b { 1 } else { 0 }
            }
            AluOp::PassA => a,
            AluOp::PassB => b,
        };

        self.result = result;
        self.zero = result == 0;
        result
    }

    /// Check if result is zero (for branches)
    pub fn is_zero(&self) -> bool {
        self.zero
    }

    pub fn get_result(&self) -> Word {
        self.result
    }
}
