// Define a boolean function type for flexibility
type BoolFunc = fn(bool, bool) -> bool;

// Logic gate functions
fn and_gate(a: bool, b: bool) -> bool {
    a && b
}

fn or_gate(a: bool, b: bool) -> bool {
    a || b
}

fn xor_gate(a: bool, b: bool) -> bool {
    a ^ b
}

fn nand_gate(a: bool, b: bool) -> bool {
    !(a && b)
}

fn nor_gate(a: bool, b: bool) -> bool {
    !(a || b)
}

// Gate struct representing a logic gate
#[derive(Debug, Clone)]
pub struct LogicGate {
   pub name: String,
   pub function: BoolFunc,
   pub symbol: char,
}

impl LogicGate {
    pub fn new(name: &str, function: BoolFunc, symbol: char) -> Self {
        LogicGate {
            name: name.to_string(),
            function,
            symbol,
        }
    }
    
   pub fn evaluate(&self, a: bool, b: bool) -> bool {
        (self.function)(a, b)
    }
}

// Collection of standard gates
pub struct GateCollection {
    gates: Vec<LogicGate>,
}

impl GateCollection {
    pub fn new() -> Self {
        let mut gates = Vec::new();
        
        gates.push(LogicGate::new("AND", and_gate, '∧'));
        gates.push(LogicGate::new("OR", or_gate, '∨'));
        gates.push(LogicGate::new("XOR", xor_gate, '⊕'));
        gates.push(LogicGate::new("NAND", nand_gate, '⊼'));
        gates.push(LogicGate::new("NOR", nor_gate, '⊽'));
        
        GateCollection { gates }
    }
    
    pub fn get_gate(&self, name: &str) -> Option<&LogicGate> {
        self.gates.iter().find(|g| g.name == name)
    }
}