//! Instruction encoder - converts parsed instructions to machine code

use riscv32i_sim::{Word, InstructionEncoder};
use std::collections::HashMap;
use crate::{AsmError, Result};

pub fn encode_instruction(
    line: &str,
    _address: u32,
    _labels: &HashMap<String, u32>,
) -> Result<Word> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.is_empty() {
        return Err(AsmError::ParseError("Empty instruction".to_string()));
    }

    let mnemonic = parts[0].to_lowercase();
    
    // Stub implementation - will be expanded
    match mnemonic.as_str() {
        "addi" => {
            // Parse: addi rd, rs1, imm
            Ok(InstructionEncoder::i_type(0b0010011, 1, 0b000, 0, 42))
        }
        _ => Err(AsmError::UnknownInstruction(mnemonic)),
    }
}
