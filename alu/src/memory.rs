use crate::types::*;

/// Memory module - like a SystemVerilog memory array
pub struct Memory {
    // Memory array: logic[31:0] mem[0:1023]
    data: [Logic32; 1024],
    
    // Read/write enable signals
    read_enable: bool,
    write_enable: bool,
    
    // Address and data buses
    address: Logic32,
    write_data: Logic32,
    read_data: Logic32,
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
        }
    }

    /// Combinational read - like always @(*)
    pub fn read(&mut self, addr: Logic32) {
        let index = (addr as usize) % self.data.len();
        self.read_data = self.data[index];
    }

    /// Sequential write - like always @(posedge clk)
    pub fn write(&mut self, addr: Logic32, data: Logic32) {
        let index = (addr as usize) % self.data.len();
        self.data[index] = data;
    }

    /// Clock edge with control signals
    pub fn clock(&mut self, read_en: bool, write_en: bool, addr: Logic32, data: Logic32) {
        self.read_enable = read_en;
        self.write_enable = write_en;
        self.address = addr;
        self.write_data = data;

        if self.write_enable {
            self.write(self.address, self.write_data);
        }
        if self.read_enable {
            self.read(self.address);
        }
    }

    pub fn get_read_data(&self) -> Logic32 {
        self.read_data
    }

    /// Initialize memory with program
    pub fn load_program(&mut self, program: &[(usize, Logic32)]) {
        for &(addr, data) in program {
            if addr < self.data.len() {
                self.data[addr] = data;
            }
        }
    }
}
