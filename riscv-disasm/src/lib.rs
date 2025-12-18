//! RISC-V Disassembler
//! 
//! Converts RISC-V machine code back to human-readable assembly.
//! 
//! # Example
//! ```rust,ignore
//! use riscv_disasm::disassemble;
//! 
//! let asm = disassemble(0x02A00093);  // "addi x1, x0, 42"
//! println!("{}", asm);
//! ```

use riscv32i_sim::{Word, Instruction};

#[derive(Debug, thiserror::Error)]
pub enum DisasmError {
    #[error("Unknown opcode: 0x{0:02X}")]
    UnknownOpcode(u8),
    
    #[error("Invalid instruction: 0x{0:08X}")]
    InvalidInstruction(Word),
}

pub type Result<T> = std::result::Result<T, DisasmError>;

/// Disassemble a single instruction
pub fn disassemble(word: Word) -> Result<String> {
    let inst = Instruction::new(word);
    let opcode = inst.opcode();
    
    match opcode {
        0b0110111 => disasm_lui(&inst),
        0b0010111 => disasm_auipc(&inst),
        0b1101111 => disasm_jal(&inst),
        0b1100111 => disasm_jalr(&inst),
        0b1100011 => disasm_branch(&inst),
        0b0000011 => disasm_load(&inst),
        0b0100011 => disasm_store(&inst),
        0b0010011 => disasm_op_imm(&inst),
        0b0110011 => disasm_op(&inst),
        0b1110011 => disasm_system(&inst),
        _ => Err(DisasmError::UnknownOpcode(opcode)),
    }
}

fn reg_name(reg: u8) -> String {
    let abi_names = [
        "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2",
        "s0", "s1", "a0", "a1", "a2", "a3", "a4", "a5",
        "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7",
        "s8", "s9", "s10", "s11", "t3", "t4", "t5", "t6",
    ];
    
    if (reg as usize) < abi_names.len() {
        format!("x{}({})", reg, abi_names[reg as usize])
    } else {
        format!("x{}", reg)
    }
}

fn disasm_lui(inst: &Instruction) -> Result<String> {
    let rd = inst.rd();
    let imm = inst.imm_u();
    Ok(format!("lui {}, 0x{:X}", reg_name(rd), (imm as u32) >> 12))
}

fn disasm_auipc(inst: &Instruction) -> Result<String> {
    let rd = inst.rd();
    let imm = inst.imm_u();
    Ok(format!("auipc {}, 0x{:X}", reg_name(rd), (imm as u32) >> 12))
}

fn disasm_jal(inst: &Instruction) -> Result<String> {
    let rd = inst.rd();
    let imm = inst.imm_j();
    Ok(format!("jal {}, {}", reg_name(rd), imm))
}

fn disasm_jalr(inst: &Instruction) -> Result<String> {
    let rd = inst.rd();
    let rs1 = inst.rs1();
    let imm = inst.imm_i();
    Ok(format!("jalr {}, {}({})", reg_name(rd), imm, reg_name(rs1)))
}

fn disasm_branch(inst: &Instruction) -> Result<String> {
    let rs1 = inst.rs1();
    let rs2 = inst.rs2();
    let imm = inst.imm_b();
    let funct3 = inst.funct3();
    
    let mnemonic = match funct3 {
        0b000 => "beq",
        0b001 => "bne",
        0b100 => "blt",
        0b101 => "bge",
        0b110 => "bltu",
        0b111 => "bgeu",
        _ => return Err(DisasmError::InvalidInstruction(inst.raw)),
    };
    
    Ok(format!("{} {}, {}, {}", mnemonic, reg_name(rs1), reg_name(rs2), imm))
}

fn disasm_load(inst: &Instruction) -> Result<String> {
    let rd = inst.rd();
    let rs1 = inst.rs1();
    let imm = inst.imm_i();
    let funct3 = inst.funct3();
    
    let mnemonic = match funct3 {
        0b000 => "lb",
        0b001 => "lh",
        0b010 => "lw",
        0b100 => "lbu",
        0b101 => "lhu",
        _ => return Err(DisasmError::InvalidInstruction(inst.raw)),
    };
    
    Ok(format!("{} {}, {}({})", mnemonic, reg_name(rd), imm, reg_name(rs1)))
}

fn disasm_store(inst: &Instruction) -> Result<String> {
    let rs1 = inst.rs1();
    let rs2 = inst.rs2();
    let imm = inst.imm_s();
    let funct3 = inst.funct3();
    
    let mnemonic = match funct3 {
        0b000 => "sb",
        0b001 => "sh",
        0b010 => "sw",
        _ => return Err(DisasmError::InvalidInstruction(inst.raw)),
    };
    
    Ok(format!("{} {}, {}({})", mnemonic, reg_name(rs2), imm, reg_name(rs1)))
}

fn disasm_op_imm(inst: &Instruction) -> Result<String> {
    let rd = inst.rd();
    let rs1 = inst.rs1();
    let imm = inst.imm_i();
    let funct3 = inst.funct3();
    
    let mnemonic = match funct3 {
        0b000 => "addi",
        0b010 => "slti",
        0b011 => "sltiu",
        0b100 => "xori",
        0b110 => "ori",
        0b111 => "andi",
        0b001 => "slli",
        0b101 => {
            if inst.funct7() & 0x20 != 0 {
                "srai"
            } else {
                "srli"
            }
        }
        _ => return Err(DisasmError::InvalidInstruction(inst.raw)),
    };
    
    Ok(format!("{} {}, {}, {}", mnemonic, reg_name(rd), reg_name(rs1), imm))
}

fn disasm_op(inst: &Instruction) -> Result<String> {
    let rd = inst.rd();
    let rs1 = inst.rs1();
    let rs2 = inst.rs2();
    let funct3 = inst.funct3();
    let funct7 = inst.funct7();
    
    let mnemonic = match (funct3, funct7) {
        (0b000, 0b0000000) => "add",
        (0b000, 0b0100000) => "sub",
        (0b001, _) => "sll",
        (0b010, _) => "slt",
        (0b011, _) => "sltu",
        (0b100, _) => "xor",
        (0b101, 0b0000000) => "srl",
        (0b101, 0b0100000) => "sra",
        (0b110, _) => "or",
        (0b111, _) => "and",
        _ => return Err(DisasmError::InvalidInstruction(inst.raw)),
    };
    
    Ok(format!("{} {}, {}, {}", mnemonic, reg_name(rd), reg_name(rs1), reg_name(rs2)))
}

fn disasm_system(inst: &Instruction) -> Result<String> {
    match inst.raw {
        0x00000073 => Ok("ecall".to_string()),
        0x00100073 => Ok("ebreak".to_string()),
        _ => Err(DisasmError::InvalidInstruction(inst.raw)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use riscv32i_sim::InstructionEncoder;

    #[test]
    fn test_disasm_addi() {
        let inst = InstructionEncoder::i_type(0b0010011, 1, 0b000, 0, 42);
        let asm = disassemble(inst).unwrap();
        assert!(asm.contains("addi"));
        assert!(asm.contains("42"));
    }

    #[test]
    fn test_disasm_add() {
        let inst = InstructionEncoder::r_type(0b0110011, 3, 0b000, 1, 2, 0b0000000);
        let asm = disassemble(inst).unwrap();
        assert!(asm.contains("add"));
    }
}
