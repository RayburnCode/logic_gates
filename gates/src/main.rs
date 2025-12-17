mod gate;
mod truth_table;
use gate::{LogicGate, GateCollection};
use truth_table::TruthTable;

fn main() {
    // Create gate collection
    let gates = GateCollection::new();
    
    // Get specific gates
    let and_gate = gates.get_gate("AND").unwrap();
    let or_gate = gates.get_gate("OR").unwrap();
    let xor_gate = gates.get_gate("XOR").unwrap();
    let nand_gate = gates.get_gate("NAND").unwrap();
    
    // Create truth table for 2-input gates
    let selected_gates = vec![and_gate, or_gate, xor_gate, nand_gate];
    let truth_table = TruthTable::new(&selected_gates, 2);
    
    println!("Truth Table for 2-input gates:");
    println!("{}", truth_table);
    println!();
    
    // Get matrix representation
    let (matrix, column_names) = truth_table.to_matrix();
    
    println!("Matrix representation:");
    print!("  ");
    for name in &column_names {
        print!(" {:^5}", name);
    }
    println!();
    
    for (i, row) in matrix.iter().enumerate() {
        print!("{}: ", i);
        for &value in row {
            print!(" {:^5}", if value { "1" } else { "0" });
        }
        println!();
    }
    
    // Demonstrate gate evaluation
    println!("\nDirect gate evaluation:");
    println!("AND(true, false) = {}", and_gate.evaluate(true, false));
    println!("OR(true, false) = {}", or_gate.evaluate(true, false));
    println!("XOR(true, true) = {}", xor_gate.evaluate(true, true));
    
    // Create a circuit combining multiple gates
    println!("\nCircuit example: (A AND B) OR (C AND D)");
    let a = true;
    let b = false;
    let c = true;
    let d = true;
    
    let ab_and = and_gate.evaluate(a, b);
    let cd_and = and_gate.evaluate(c, d);
    let result = or_gate.evaluate(ab_and, cd_and);
    
    println!("A={}, B={}, C={}, D={}", 
             a as u8, b as u8, c as u8, d as u8);
    println!("(A AND B) = {}", ab_and as u8);
    println!("(C AND D) = {}", cd_and as u8);
    println!("Final result = {}", result as u8);
}

// Additional utility: Generate truth table for custom function
fn generate_custom_truth_table<F>(func: F, num_inputs: usize, name: &str) -> TruthTable
where
    F: Fn(&[bool]) -> bool + 'static,
{
    let total_combinations = 1 << num_inputs;
    let mut inputs = Vec::new();
    let mut outputs = Vec::new();
    
    // Create a temporary gate struct
    let custom_gate = LogicGate {
        name: name.to_string(),
        function: |a, b| {
            // This is a simplified version - for multi-input you'd need different handling
            a && b // Placeholder
        },
        symbol: 'ƒ',
    };
    
    for i in 0..total_combinations {
        let mut input_row = Vec::new();
        for bit in 0..num_inputs {
            input_row.push((i >> (num_inputs - 1 - bit)) & 1 == 1);
        }
        
        let output = func(&input_row);
        
        inputs.push(input_row);
        outputs.push(vec![output]);
    }
    
    TruthTable {
        inputs,
        outputs,
        gate_names: vec![name.to_string()],
    }
}