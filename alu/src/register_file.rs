use crate::types::*;

/// RISC-V Register File (32 registers, x0 hardwired to zero)
/// RV32I specifies 32 general-purpose registers x0-x31
/// x0 (zero) always reads 0 and writes are ignored
pub struct RegisterFile {
    registers: [Word; 32],
    
    // Port A (read/write)
    addr_a: u8,
    write_data_a: Word,
    read_data_a: Word,
    write_enable_a: bool,
    
    // Port B (read only)
    addr_b: u8,
    read_data_b: Word,
}

impl RegisterFile {
    pub fn new() -> Self {
        Self {
            registers: [0; 32],
            addr_a: 0,
            write_data_a: 0,
            read_data_a: 0,
            write_enable_a: false,
            addr_b: 0,
            read_data_b: 0,
        }
    }

    /// Combinational read - always @(*)
    /// x0 always reads as 0
    fn combinational_read(&mut self) {
        // Port A read - enforce x0 = 0
        if self.addr_a == 0 {
            self.read_data_a = 0;
        } else if (self.addr_a as usize) < self.registers.len() {
            self.read_data_a = self.registers[self.addr_a as usize];
        }
        
        // Port B read - enforce x0 = 0
        if self.addr_b == 0 {
            self.read_data_b = 0;
        } else if (self.addr_b as usize) < self.registers.len() {
            self.read_data_b = self.registers[self.addr_b as usize];
        }
    }

    /// Sequential write - always @(posedge clk)
    /// RISC-V rule: x0 is read-only (writes are ignored)
    pub fn clock(&mut self, 
                 addr_a: u8, 
                 write_data_a: Word, 
                 write_enable_a: bool,
                 addr_b: u8) {
        // Update inputs
        self.addr_a = addr_a;
        self.write_data_a = write_data_a;
        self.write_enable_a = write_enable_a;
        self.addr_b = addr_b;

        // Write on clock edge
        // RISC-V RULE: Ignore writes to x0
        if self.write_enable_a 
            && self.addr_a != 0  // x0 is hardwired to zero
            && (self.addr_a as usize) < self.registers.len() {
            self.registers[self.addr_a as usize] = self.write_data_a;
        }

        // Update read outputs
        self.combinational_read();
    }

    pub fn get_read_data_a(&self) -> Word {
        self.read_data_a
    }

    pub fn get_read_data_b(&self) -> Word {
        self.read_data_b
    }

    /// Debug access - display RISC-V ABI register names
    pub fn dump_registers(&self, start: usize, count: usize) {
        let abi_names = [
            "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2",
            "s0", "s1", "a0", "a1", "a2", "a3", "a4", "a5",
            "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7",
            "s8", "s9", "s10", "s11", "t3", "t4", "t5", "t6",
        ];
        
        for i in start..(start + count).min(self.registers.len()) {
            let value = if i == 0 { 0 } else { self.registers[i] };
            println!("x{:<2} ({:<4}): 0x{:08X} ({})", 
                i, abi_names[i], value, value as i32);
        }
    }

    /// Reset all registers (except x0 which is always 0)
    pub fn reset(&mut self) {
        self.registers = [0; 32];
    }
}
