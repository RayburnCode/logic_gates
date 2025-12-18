//! Label resolution

use std::collections::HashMap;

pub struct LabelTable {
    labels: HashMap<String, u32>,
}

impl LabelTable {
    pub fn new() -> Self {
        Self {
            labels: HashMap::new(),
        }
    }

    pub fn insert(&mut self, label: String, address: u32) {
        self.labels.insert(label, address);
    }

    pub fn get(&self, label: &str) -> Option<u32> {
        self.labels.get(label).copied()
    }
}

impl Default for LabelTable {
    fn default() -> Self {
        Self::new()
    }
}
