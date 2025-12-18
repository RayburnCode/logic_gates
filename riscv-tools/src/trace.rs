//! Execution tracing

use riscv32i_sim::{Word, Addr};

#[derive(Debug, Clone)]
pub struct TraceEntry {
    pub cycle: u64,
    pub pc: Addr,
    pub instruction: Word,
    pub disassembly: String,
}

pub struct ExecutionTrace {
    entries: Vec<TraceEntry>,
    max_entries: usize,
}

impl ExecutionTrace {
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: Vec::new(),
            max_entries,
        }
    }

    pub fn record(&mut self, entry: TraceEntry) {
        self.entries.push(entry);
        if self.entries.len() > self.max_entries {
            self.entries.remove(0);
        }
    }

    pub fn get_entries(&self) -> &[TraceEntry] {
        &self.entries
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }
}
