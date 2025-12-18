//! Interactive debugger for RISC-V programs

use riscv32i_sim::Cpu;

pub struct Debugger {
    cpu: Cpu,
    breakpoints: Vec<u32>,
}

impl Debugger {
    pub fn new(cpu: Cpu) -> Self {
        Self {
            cpu,
            breakpoints: Vec::new(),
        }
    }

    pub fn add_breakpoint(&mut self, address: u32) {
        self.breakpoints.push(address);
    }

    pub fn step(&mut self) {
        self.cpu.clock();
    }

    pub fn run_until_breakpoint(&mut self) -> u32 {
        loop {
            let pc = self.cpu.control.get_pc();
            if self.breakpoints.contains(&pc) {
                return pc;
            }
            self.cpu.clock();
        }
    }

    pub fn get_cpu(&self) -> &Cpu {
        &self.cpu
    }

    pub fn get_cpu_mut(&mut self) -> &mut Cpu {
        &mut self.cpu
    }
}
