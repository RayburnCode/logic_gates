<!-- @format -->

# SystemVerilog to Rust: Hardware Modeling Guide

## Overview

This document explains how to model hardware designs in Rust using SystemVerilog-inspired patterns. While Rust is a software language, its strong type system, ownership model, and zero-cost abstractions make it excellent for hardware simulation and verification.

## Why Model Hardware in Rust?

### Advantages

1. **Type Safety**: Rust's type system catches errors at compile time that would be runtime errors in traditional HDL simulators
2. **Performance**: Near-C performance without garbage collection overhead
3. **Modern Tooling**: Excellent IDE support, package management (Cargo), and testing frameworks
4. **Software Integration**: Seamless integration with software tools and verification frameworks
5. **Portability**: Write once, run on any platform
6. **Memory Safety**: Ownership model prevents many classes of bugs
7. **Concurrency**: Built-in support for parallel simulation

### Use Cases

- **Functional Verification**: High-level testbenches and verification
- **Performance Modeling**: Cycle-accurate simulations
- **Algorithm Prototyping**: Test algorithms before RTL implementation
- **Co-simulation**: Interface with SystemVerilog via DPI or other methods
- **Education**: Learn hardware concepts with modern tooling

## Core Mapping: SystemVerilog → Rust

### Data Types

| SystemVerilog | Rust Equivalent    | Notes                  |
| ------------- | ------------------ | ---------------------- |
| `logic[7:0]`  | `u8` or `Logic8`   | Unsigned 8-bit         |
| `logic[15:0]` | `u16` or `Logic16` | Unsigned 16-bit        |
| `logic[31:0]` | `u32` or `Logic32` | Unsigned 32-bit        |
| `logic[63:0]` | `u64` or `Logic64` | Unsigned 64-bit        |
| `bit[N:0]`    | `uN` types         | Same as logic in Rust  |
| `byte`        | `i8`               | Signed 8-bit           |
| `shortint`    | `i16`              | Signed 16-bit          |
| `int`         | `i32`              | Signed 32-bit          |
| `longint`     | `i64`              | Signed 64-bit          |
| `real`        | `f64`              | Double precision float |
| `string`      | `String`           | Heap-allocated string  |

### Type Aliases for Clarity

```rust
// Define hardware-inspired type aliases
pub type Logic8 = u8;
pub type Logic16 = u16;
pub type Logic32 = u32;
pub type Bit4 = u8;  // Use with mask: value & 0x0F
```

### Arrays and Memories

| SystemVerilog             | Rust             |
| ------------------------- | ---------------- |
| `logic[7:0] mem[0:1023]`  | `[u8; 1024]`     |
| `bit[31:0] regs[16]`      | `[u32; 16]`      |
| `logic data[4][4]`        | `[[bool; 4]; 4]` |
| `logic[7:0] matrix[4][4]` | `[[u8; 4]; 4]`   |

## Structural Mapping

### Module Declaration

**SystemVerilog:**

```systemverilog
module alu (
    input logic clk,
    input logic rst,
    input logic[31:0] a,
    input logic[31:0] b,
    output logic[31:0] result
);
    // Module body
endmodule
```

**Rust:**

```rust
pub struct Alu {
    // Internal state (registers)
    result: u32,
    // Other state...
}

impl Alu {
    pub fn new() -> Self {
        Self {
            result: 0,
        }
    }

    pub fn clock(&mut self, a: u32, b: u32) -> u32 {
        // Combinational and sequential logic
        self.result = a + b;
        self.result
    }
}
```

### Always Blocks

#### Combinational Logic: `always @(*)`

**SystemVerilog:**

```systemverilog
always @(*) begin
    result = a + b;
end
```

**Rust:**

```rust
fn combinational_logic(&self, a: u32, b: u32) -> u32 {
    a + b
}
```

#### Sequential Logic: `always @(posedge clk)`

**SystemVerilog:**

```systemverilog
always @(posedge clk) begin
    if (rst)
        counter <= 0;
    else
        counter <= counter + 1;
end
```

**Rust:**

```rust
pub fn clock(&mut self, rst: bool) {
    if rst {
        self.counter = 0;
    } else {
        self.counter += 1;
    }
}
```

### Packed and Unpacked Structs

**SystemVerilog:**

```systemverilog
typedef struct packed {
    logic[7:0] opcode;
    logic[15:0] address;
} instruction_t;
```

**Rust:**

```rust
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct Instruction {
    pub opcode: u8,
    pub address: u16,
}
```

### Enumerations

**SystemVerilog:**

```systemverilog
typedef enum logic[1:0] {
    IDLE = 2'b00,
    READ = 2'b01,
    WRITE = 2'b10
} state_t;
```

**Rust:**

```rust
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Idle = 0x00,
    Read = 0x01,
    Write = 0x02,
}
```

## Design Patterns

### Pattern 1: State Machine

**SystemVerilog:**

```systemverilog
always @(posedge clk) begin
    case(state)
        IDLE: state <= READ;
        READ: state <= WRITE;
        WRITE: state <= IDLE;
    endcase
end
```

**Rust:**

```rust
pub fn clock(&mut self) {
    self.state = match self.state {
        State::Idle => State::Read,
        State::Read => State::Write,
        State::Write => State::Idle,
    };
}
```

### Pattern 2: Register File (Dual-Port)

**SystemVerilog:**

```systemverilog
logic[31:0] registers[32];

always @(posedge clk) begin
    if (write_enable)
        registers[write_addr] <= write_data;
end

assign read_data_a = registers[read_addr_a];
assign read_data_b = registers[read_addr_b];
```

**Rust:**

```rust
pub struct RegisterFile {
    registers: [u32; 32],
}

impl RegisterFile {
    pub fn clock(&mut self, write_enable: bool, write_addr: u8, write_data: u32) {
        if write_enable && (write_addr as usize) < 32 {
            self.registers[write_addr as usize] = write_data;
        }
    }

    pub fn read(&self, addr: u8) -> u32 {
        self.registers[addr as usize % 32]
    }
}
```

### Pattern 3: Memory Module

**SystemVerilog:**

```systemverilog
logic[31:0] mem[1024];

always @(posedge clk) begin
    if (write_enable)
        mem[address] <= write_data;
    if (read_enable)
        read_data <= mem[address];
end
```

**Rust:**

```rust
pub struct Memory {
    data: [u32; 1024],
}

impl Memory {
    pub fn clock(&mut self, addr: u32, write_data: u32, write_enable: bool) -> u32 {
        let index = (addr as usize) % self.data.len();

        if write_enable {
            self.data[index] = write_data;
        }

        self.data[index]
    }
}
```

### Pattern 4: Hierarchical Modules

**SystemVerilog:**

```systemverilog
module cpu;
    alu alu_inst(...);
    memory mem_inst(...);
    control ctrl_inst(...);
endmodule
```

**Rust:**

```rust
pub struct Cpu {
    pub alu: Alu,
    pub memory: Memory,
    pub control: ControlUnit,
}

impl Cpu {
    pub fn clock(&mut self) {
        // Coordinate submodules
        let ctrl = self.control.get_signals();
        let result = self.alu.execute(ctrl.alu_op, ...);
        self.memory.clock(...);
    }
}
```

## Testing and Verification

### Testbench Pattern

**SystemVerilog:**

```systemverilog
initial begin
    reset = 1;
    #10 reset = 0;

    a = 10;
    b = 20;
    #10;

    $display("Result: %d", result);
end
```

**Rust:**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alu_add() {
        let mut alu = Alu::new();
        alu.reset();

        let result = alu.execute(AluOp::Add, 10, 20);
        assert_eq!(result, 30);
    }
}
```

### Advanced Testing with Property-Based Testing

```rust
#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_alu_commutative(a in 0u32..1000, b in 0u32..1000) {
            let mut alu = Alu::new();
            let r1 = alu.execute(AluOp::Add, a, b);
            let r2 = alu.execute(AluOp::Add, b, a);
            prop_assert_eq!(r1, r2);
        }
    }
}
```

## Flag Handling

### Packed Flags

**SystemVerilog:**

```systemverilog
typedef struct packed {
    logic zero;
    logic carry;
    logic negative;
    logic overflow;
} flags_t;
```

**Rust:**

```rust
#[derive(Debug, Clone, Copy)]
pub struct Flags {
    pub zero: bool,
    pub carry: bool,
    pub negative: bool,
    pub overflow: bool,
}

impl Flags {
    pub fn to_bits(&self) -> u8 {
        (self.overflow as u8) << 3
            | (self.negative as u8) << 2
            | (self.carry as u8) << 1
            | (self.zero as u8)
    }

    pub fn from_bits(bits: u8) -> Self {
        Self {
            zero: bits & 0x01 != 0,
            carry: bits & 0x02 != 0,
            negative: bits & 0x04 != 0,
            overflow: bits & 0x08 != 0,
        }
    }
}
```

## Best Practices

### 1. Separation of Concerns

- **Combinational logic**: Pure functions
- **Sequential logic**: `clock()` methods
- **Module interface**: Public methods

### 2. Type Safety

```rust
// Use newtypes for different signal types
#[derive(Debug, Clone, Copy)]
pub struct Address(pub u32);

#[derive(Debug, Clone, Copy)]
pub struct Data(pub u32);

// Prevents accidentally mixing address and data
```

### 3. Explicit State

```rust
// Keep all state in the struct
pub struct Module {
    // Registers (state)
    register_a: u32,
    register_b: u32,

    // Wires (intermediate values)
    wire_x: u32,
    wire_y: u32,
}
```

### 4. Documentation

```rust
/// Memory module with single-cycle read/write
///
/// # Timing
/// - Write: Data written on clock edge
/// - Read: Combinational (same cycle)
pub struct Memory {
    // ...
}
```

## Project Structure

```
alu/
├── Cargo.toml
├── src/
│   ├── main.rs           # Top-level testbench
│   ├── types.rs          # Common types and aliases
│   ├── memory.rs         # Memory module
│   ├── register_file.rs  # Register file module
│   ├── control_unit.rs   # Control unit
│   └── cpu.rs           # Top-level integration
└── tests/
    └── integration_tests.rs
```

## Limitations and Workarounds

### 1. No Built-in X/Z States

**Workaround**: Use `Option<T>` for values that might be unknown

```rust
pub struct Signal(Option<u32>);
```

### 2. No Automatic Sensitivity Lists

**Workaround**: Explicitly call combinational functions

```rust
// Call after any input change
self.update_combinational_logic();
```

### 3. No Delay Modeling

**Workaround**: Use explicit cycle counts or event queues

```rust
pub struct DelayedSignal<T> {
    value: T,
    delay_cycles: usize,
    queue: VecDeque<T>,
}
```

### 4. No Generate Blocks

**Workaround**: Use Rust macros or loops

```rust
macro_rules! replicate_module {
    ($count:expr) => {
        [(); $count].map(|_| Module::new())
    };
}
```

## Conclusion

Rust provides a powerful, modern alternative to traditional HDL simulation. While it lacks some HDL-specific features, its strengths in type safety, performance, and tooling make it an excellent choice for:

- High-level verification
- Performance modeling
- Algorithm development
- Education and prototyping

The patterns shown in this guide enable you to leverage Rust's ecosystem while maintaining hardware design thinking.

## Further Reading

- [Rust Book](https://doc.rust-lang.org/book/)
- [Embedded Rust Book](https://rust-embedded.github.io/book/)
- Hardware design patterns in Rust
- Formal verification with Rust
