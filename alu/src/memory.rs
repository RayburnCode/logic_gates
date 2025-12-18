use crate::types::*;

/// RISC-V Memory module
/// - Little-endian byte ordering
/// - 4-byte aligned word access (RISC-V requirement)
/// - 1024 words (4096 bytes) of addressable memory
pub struct Memory {
    // Memory array: 1024 words of 32-bit data
    data: [Word; 1024],
    
    // Control signals
    read_enable: bool,
    write_enable: bool,
    
    // Address and data buses
    address: Addr,
    write_data: Word,
    read_data: Word,
    
    // Byte enable for partial word writes (RISC-V SB, SH)
    write_mask: u8,  // 0b1111 for word, 0b0011 for halfword, 0b0001 for byte
}

impl Memory {
    pub fn new() -> Self {
        Self {
            data: [0; 1024],
            read_enable: false,
            write_enable: false,
            address: 0,
            write_data: 0,
            read_data: 0,
            write_mask: 0b1111,
        }
    }

    /// Combinational read - like always @(*)
    /// RISC-V: Word addresses must be 4-byte aligned
    fn combinational_read(&mut self, addr: Addr) {
        // Convert byte address to word address
        let word_addr = (addr >> 2) as usize % self.data.len();
        self.read_data = self.data[word_addr];
    }

    /// Sequential write - like always @(posedge clk)
    /// RISC-V: Supports byte, halfword, and word writes
    fn sequential_write(&mut self, addr: Addr, data: Word, mask: u8) {
        let word_addr = (addr >> 2) as usize % self.data.len();
        let byte_offset = (addr & 0x3) as usize;
        
        let mut current = self.data[word_addr];
        
        // Apply write mask for partial word writes
        match mask {
            0b0001 => {
                // Byte write (SB)
                let shift = byte_offset * 8;
                let byte_mask = 0xFF << shift;
                current = (current & !byte_mask) | ((data & 0xFF) << shift);
            }
            0b0011 => {
                // Halfword write (SH) - must be 2-byte aligned
                if addr & 0x1 == 0 {
                    let shift = byte_offset * 8;
                    let half_mask = 0xFFFF << shift;
                    current = (current & !half_mask) | ((data & 0xFFFF) << shift);
                }
            }
            0b1111 => {
                // Word write (SW) - must be 4-byte aligned
                if addr & 0x3 == 0 {
                    current = data;
                }
            }
            _ => {} // Invalid mask
        }
        
        self.data[word_addr] = current;
    }

    /// Clock edge with control signals
    pub fn clock(&mut self, read_en: bool, write_en: bool, addr: Addr, data: Word) {
        self.read_enable = read_en;
        self.write_enable = write_en;
        self.address = addr;
        self.write_data = data;

        // Write on clock edge
        if self.write_enable {
            self.sequential_write(self.address, self.write_data, self.write_mask);
        }
        
        // Read (combinational, but latched for simplicity)
        if self.read_enable {
            self.combinational_read(self.address);
        }
    }

    pub fn get_read_data(&self) -> Word {
        self.read_data
    }

    /// Set write mask for partial word writes
    pub fn set_write_mask(&mut self, mask: u8) {
        self.write_mask = mask;
    }

    /// Initialize memory with program (4-byte aligned addresses)
    /// RISC-V rule: Instructions must be 4-byte aligned
    pub fn load_program(&mut self, program: &[(Addr, Word)]) {
        for &(addr, data) in program {
            // Ensure 4-byte alignment
            if addr & 0x3 == 0 {
                let word_addr = (addr >> 2) as usize;
                if word_addr < self.data.len() {
                    self.data[word_addr] = data;
                }
            }
        }
    }

    /// Direct read for fetch (always word-aligned)
    pub fn fetch(&self, addr: Addr) -> Word {
        let word_addr = (addr >> 2) as usize % self.data.len();
        self.data[word_addr]
    }

    pub fn reset(&mut self) {
        self.data = [0; 1024];
        self.read_data = 0;
    }
}
