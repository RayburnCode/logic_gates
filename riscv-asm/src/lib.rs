//! RISC-V Assembler
//! 
//! Converts RISC-V assembly language to machine code.
//! 
//! # Example
//! ```rust,ignore
//! use riscv_asm::Assembler;
//! 
//! let asm = Assembler::new();
//! let program = asm.assemble("
//!     addi x1, x0, 42
//!     addi x2, x0, 8
//!     add  x3, x1, x2
//! ").unwrap();
//! ```

use std::collections::HashMap;
use riscv32i_sim::{Word, InstructionEncoder};

pub mod parser;
pub mod encoder;
pub mod labels;

pub use parser::parse_assembly;
pub use encoder::encode_instruction;

#[derive(Debug, thiserror::Error)]
pub enum AsmError {
    #[error("Parse error: {0}")]
    ParseError(String),
    
    #[error("Unknown instruction: {0}")]
    UnknownInstruction(String),
    
    #[error("Invalid register: {0}")]
    InvalidRegister(String),
    
    #[error("Undefined label: {0}")]
    UndefinedLabel(String),
    
    #[error("Invalid immediate value: {0}")]
    InvalidImmediate(String),
}

pub type Result<T> = std::result::Result<T, AsmError>;

/// RISC-V Assembler
pub struct Assembler {
    labels: HashMap<String, u32>,
}

impl Assembler {
    pub fn new() -> Self {
        Self {
            labels: HashMap::new(),
        }
    }

    /// Assemble RISC-V assembly code into machine code
    pub fn assemble(&mut self, source: &str) -> Result<Vec<(u32, Word)>> {
        // First pass: collect labels
        self.collect_labels(source)?;
        
        // Second pass: generate machine code
        self.generate_code(source)
    }

    fn collect_labels(&mut self, source: &str) -> Result<()> {
        let mut address = 0u32;
        
        for line in source.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            if line.ends_with(':') {
                let label = line.trim_end_matches(':');
                self.labels.insert(label.to_string(), address);
            } else {
                address += 4;
            }
        }
        
        Ok(())
    }

    fn generate_code(&self, source: &str) -> Result<Vec<(u32, Word)>> {
        let mut program = Vec::new();
        let mut address = 0u32;
        
        for line in source.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') || line.ends_with(':') {
                continue;
            }
            
            let instruction = encode_instruction(line, address, &self.labels)?;
            program.push((address, instruction));
            address += 4;
        }
        
        Ok(program)
    }
}

impl Default for Assembler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_assembly() {
        let mut asm = Assembler::new();
        let result = asm.assemble("addi x1, x0, 42");
        assert!(result.is_ok());
    }
}
