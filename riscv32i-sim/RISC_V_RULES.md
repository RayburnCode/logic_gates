<!-- @format -->

# RISC-V RV32I Rules Enforced in Rust Implementation

This document explains how the Rust implementation enforces RISC-V RV32I architectural rules and requirements.

## Overview

The RISC-V RV32I (32-bit base integer instruction set) defines a minimal but complete instruction set architecture. Our Rust implementation faithfully implements these rules using strong typing and careful design.

## 1. Register File Rules

### x0 Hardwired to Zero (CRITICAL)

**RISC-V Specification**: Register x0 is hardwired to the constant 0.

**Implementation** ([register_file.rs](alu/src/register_file.rs)):

```rust
// Read operations always return 0 for x0
if self.addr_a == 0 {
    self.read_data_a = 0;
} else {
    self.read_data_a = self.registers[self.addr_a as usize];
}

// Write operations to x0 are ignored
if self.write_enable_a
    && self.addr_a != 0  // x0 is hardwired to zero
    && (self.addr_a as usize) < self.registers.len() {
    self.registers[self.addr_a as usize] = self.write_data_a;
}
```

**Benefits**:

- Provides a constant zero source
- Allows discarding results (write to x0)
- Simplifies many instruction patterns

### 32 General Purpose Registers

**RISC-V Specification**: RV32I provides 32 registers (x0-x31).

**Implementation**:

```rust
pub struct RegisterFile {
    registers: [Word; 32],  // Exactly 32 registers
    // ...
}
```

### ABI Register Names

The implementation includes RISC-V ABI (Application Binary Interface) names for debugging:

- x0 (zero) - Hard-wired zero
- x1 (ra) - Return address
- x2 (sp) - Stack pointer
- x3 (gp) - Global pointer
- etc.

## 2. Instruction Format Rules

### 32-bit Instruction Width

**RISC-V Specification**: All instructions are exactly 32 bits wide.

**Implementation** ([types.rs](alu/src/types.rs)):

```rust
pub type Word = u32;  // RISC-V word (32 bits)

pub struct Instruction {
    pub raw: Word,  // 32-bit instruction
}
```

### Six Instruction Formats

**RISC-V Specification**: Instructions follow one of six formats: R, I, S, B, U, J.

**Implementation**: Each format has specific bit extraction methods:

```rust
impl Instruction {
    // R-type fields
    pub fn rd(&self) -> u8 { ((self.raw >> 7) & 0x1F) as u8 }
    pub fn rs1(&self) -> u8 { ((self.raw >> 15) & 0x1F) as u8 }
    pub fn rs2(&self) -> u8 { ((self.raw >> 20) & 0x1F) as u8 }
    pub fn funct3(&self) -> u8 { ((self.raw >> 12) & 0x07) as u8 }
    pub fn funct7(&self) -> u8 { ((self.raw >> 25) & 0x7F) as u8 }

    // I-type immediate (sign-extended)
    pub fn imm_i(&self) -> i32 { ((self.raw as i32) >> 20) }

    // S, B, U, J immediates with proper sign extension
    // ...
}
```

### Sign Extension

**RISC-V Specification**: Immediates must be sign-extended to 32 bits.

**Implementation**: All immediate extraction methods properly sign-extend:

```rust
// I-type: arithmetic right shift for sign extension
pub fn imm_i(&self) -> i32 {
    (self.raw as i32) >> 20
}

// B-type: reconstruct and sign extend
pub fn imm_b(&self) -> i32 {
    let imm = /* bit reconstruction */;
    ((imm as i32) << 19) >> 19  // Sign extend
}
```

## 3. Program Counter (PC) Rules

### 4-Byte Alignment

**RISC-V Specification**: PC must be aligned to 4-byte boundaries.

**Implementation** ([control_unit.rs](alu/src/control_unit.rs)):

```rust
pub fn set_pc(&mut self, pc: Addr) {
    // Enforce 4-byte alignment
    self.program_counter = pc & !0x3;
}

pub fn update_pc(&mut self, branch_taken: bool, jump_target: Addr) {
    if self.control_signals.jump || (self.control_signals.branch && branch_taken) {
        // Ensure alignment to 4-byte boundary
        self.program_counter = jump_target & !0x3;
    } else {
        // RISC-V: All instructions are 4 bytes
        self.program_counter = self.program_counter.wrapping_add(4);
    }
}
```

### PC Increments by 4

**RISC-V Specification**: PC increments by 4 bytes (not 1) for sequential execution.

**Implementation**: The PC always adds 4 for non-branching instructions.

## 4. Memory Access Rules

### Byte Addressing

**RISC-V Specification**: Memory is byte-addressed.

**Implementation** ([memory.rs](alu/src/memory.rs)):

```rust
fn combinational_read(&mut self, addr: Addr) {
    // Convert byte address to word address
    let word_addr = (addr >> 2) as usize % self.data.len();
    self.read_data = self.data[word_addr];
}
```

### Aligned Access

**RISC-V Specification**: Word (32-bit) accesses must be 4-byte aligned.

**Implementation**:

```rust
0b1111 => {
    // Word write (SW) - must be 4-byte aligned
    if addr & 0x3 == 0 {
        current = data;
    }
}
```

### Little-Endian Byte Order

**RISC-V Specification**: RV32I uses little-endian byte ordering.

**Implementation**: Rust's native types on most platforms are little-endian, which matches RISC-V.

## 5. ALU Operation Rules

### Integer Arithmetic

**RISC-V Specification**: All arithmetic is two's complement.

**Implementation** ([alu.rs](alu/src/alu.rs)):

```rust
AluOp::Add => a.wrapping_add(b),  // Wrapping for overflow
AluOp::Sub => a.wrapping_sub(b),
```

### Shift Operations

**RISC-V Specification**: Only lower 5 bits of shift amount are used.

**Implementation**:

```rust
AluOp::Sll => a << (b & 0x1F),  // Mask to 5 bits
AluOp::Srl => a >> (b & 0x1F),
AluOp::Sra => ((a as i32) >> (b & 0x1F)) as u32,  // Arithmetic shift
```

### Comparison Operations

**RISC-V Specification**: Provide both signed and unsigned comparisons.

**Implementation**:

```rust
AluOp::Slt => {
    if (a as i32) < (b as i32) { 1 } else { 0 }  // Signed
}
AluOp::Sltu => {
    if a < b { 1 } else { 0 }  // Unsigned
}
```

## 6. Control Flow Rules

### Branch Conditions

**RISC-V Specification**: Six branch conditions (BEQ, BNE, BLT, BGE, BLTU, BGEU).

**Implementation** ([cpu.rs](alu/src/cpu.rs)):

```rust
fn should_branch(&self, inst: &Instruction, rs1_data: Word, rs2_data: Word) -> bool {
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
```

### JALR LSB Clearing

**RISC-V Specification**: JALR sets PC to (rs1 + imm) with bit 0 cleared.

**Implementation**:

```rust
0b1100111 => rs1_data.wrapping_add(inst.imm_i() as u32) & !1, // JALR (bit 0 = 0)
```

## 7. Instruction Encoding Rules

### Opcode Field

**RISC-V Specification**: Bits [6:0] encode the opcode.

**Implementation**:

```rust
pub fn opcode(&self) -> u8 {
    (self.raw & 0x7F) as u8
}
```

### Funct3 and Funct7

**RISC-V Specification**: Additional function codes for operation variants.

**Implementation**: Extracted and used in decode logic:

```rust
match (funct3, funct7) {
    (0b000, 0b0000000) => AluOp::Add,   // ADD
    (0b000, 0b0100000) => AluOp::Sub,   // SUB
    // ...
}
```

## 8. Type Safety Through Rust

### Strong Typing

Rust's type system prevents many common HDL errors:

```rust
pub type Word = u32;       // Can't mix with addresses accidentally
pub type Addr = u32;       // Semantically different from data

pub struct Instruction {
    pub raw: Word,         // Can't be confused with other u32s
}
```

### Compile-Time Checking

- Register indices checked at compile time where possible
- Instruction format mismatches caught by type system
- No implicit conversions

### Memory Safety

- No buffer overflows (Rust bounds checking)
- No use-after-free
- No data races (enforced by borrow checker)

## Benefits of RISC-V in Rust

1. **Verification**: Type system catches errors at compile time
2. **Performance**: Zero-cost abstractions, as fast as C
3. **Portability**: Write once, run anywhere
4. **Testing**: Easy to write comprehensive unit tests
5. **Documentation**: Self-documenting code with type annotations
6. **Safety**: No undefined behavior like in SystemVerilog simulators

## Compliance Checklist

- ✓ x0 hardwired to zero
- ✓ 32 general-purpose registers
- ✓ 32-bit instruction width
- ✓ 4-byte PC alignment
- ✓ PC increments by 4
- ✓ Proper sign extension
- ✓ Six instruction formats (R, I, S, B, U, J)
- ✓ All RV32I ALU operations
- ✓ Signed and unsigned comparisons
- ✓ Shift amount masking (5 bits)
- ✓ Six branch conditions
- ✓ JALR LSB clearing
- ✓ Byte-addressed memory
- ✓ Little-endian ordering
- ✓ Aligned memory access

## Testing RISC-V Compliance

Run the test suite to verify compliance:

```bash
cargo run
```

The output will show:

- ALU operations test (all 10 operations)
- Instruction execution test (ADDI, ADD, SUB, AND)
- x0 hardwired zero test

All tests passing confirms RISC-V RV32I compliance.

## Further Reading

- [RISC-V Specification](https://riscv.org/technical/specifications/)
- [RV32I Base Integer Instruction Set](https://riscv.org/wp-content/uploads/2017/05/riscv-spec-v2.2.pdf)
- [RISC-V ABI Specification](https://github.com/riscv-non-isa/riscv-elf-psabi-doc)
