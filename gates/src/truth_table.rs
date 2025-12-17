use std::fmt;
use crate::gate::LogicGate;

#[derive(Debug, Clone)]
pub struct TruthTable {
    pub inputs: Vec<Vec<bool>>,
    pub outputs: Vec<Vec<bool>>,
    pub gate_names: Vec<String>,
}

impl TruthTable {
    pub fn new(gates: &[&LogicGate], num_inputs: usize) -> Self {
        // Generate all possible input combinations
        let total_combinations = 1 << num_inputs; // 2^n combinations
        let mut inputs = Vec::new();
        
    
        for i in 0..total_combinations {
            let mut row = Vec::new();
            for bit in 0..num_inputs {
                row.push((i >> (num_inputs - 1 - bit)) & 1 == 1);
            }
            inputs.push(row);
        }
        
        // Calculate outputs for each gate
        let mut outputs = Vec::new();
        let gate_names: Vec<String> = gates.iter().map(|g| g.name.clone()).collect();
        
        for input_row in &inputs {
            let mut output_row = Vec::new();
            for gate in gates {
                if num_inputs == 1 {
                    // For single input, use first input only (though most gates need 2 inputs)
                    output_row.push(gate.evaluate(input_row[0], false));
                } else if num_inputs == 2 {
                    output_row.push(gate.evaluate(input_row[0], input_row[1]));
                } else {
                    // For more inputs, chain operations
                    let mut result = input_row[0];
                    for i in 1..num_inputs {
                        result = gate.evaluate(result, input_row[i]);
                    }
                    output_row.push(result);
                }
            }
            outputs.push(output_row);
        }
        
        TruthTable {
            inputs,
            outputs,
            gate_names,
        }
    }
    
   pub fn to_matrix(&self) -> (Vec<Vec<bool>>, Vec<String>) {
        // Combine inputs and outputs into a single matrix
        let mut matrix = Vec::new();
        
        for i in 0..self.inputs.len() {
            let mut row = self.inputs[i].clone();
            row.extend(self.outputs[i].clone());
            matrix.push(row);
        }
        
        (matrix, self.gate_names.clone())
    }
}

impl fmt::Display for TruthTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Create header
        let input_labels: Vec<String> = (0..self.inputs[0].len())
            .map(|i| format!("I{}", i))
            .collect();
        
        let header_parts = input_labels
            .iter()
            .chain(self.gate_names.iter())
            .map(|s| format!("{:^6}", s))
            .collect::<Vec<String>>()
            .join(" | ");
        
        writeln!(f, "{}", header_parts)?;
        writeln!(f, "{}", "-".repeat(header_parts.len()))?;
        
        // Create rows
        for i in 0..self.inputs.len() {
            let input_values: Vec<String> = self.inputs[i]
                .iter()
                .map(|&b| if b { "1" } else { "0" }.to_string())
                .collect();
            
            let output_values: Vec<String> = self.outputs[i]
                .iter()
                .map(|&b| if b { "1" } else { "0" }.to_string())
                .collect();
            
            let row_parts = input_values
                .iter()
                .chain(output_values.iter())
                .map(|s| format!("{:^6}", s))
                .collect::<Vec<String>>()
                .join(" | ");
            
            writeln!(f, "{}", row_parts)?;
        }
        
        Ok(())
    }
}