use crate::base;
use crate::shape;
use crate::traits;

pub struct DNA {
    name: String,
    pub raw_sequence: String,
    sequence: Vec<base::DNABase>,
    shape: shape::Shape,
}

fn is_valid_dna_sequence(sequence: &str) -> bool {
    
    if sequence.is_empty() {
        return false;
    }

    let formatted_sequence = sequence.to_ascii_uppercase();
    
    formatted_sequence.chars().all(|c| match c {
        'A' | 'T' | 'C' | 'G' | 'N' => true,
        _ => false,
    })
}


impl DNA {
    pub fn new(name: &str, sequence: &str, shape: shape::Shape) -> Option<Self> {
        
        if !is_valid_dna_sequence(sequence) {
            println!("Invalid sequence provided");
            return None;
        }
        
        Some (
            Self {
            name: name.to_string(),
            raw_sequence: sequence.to_string(),
            sequence: <DNA as traits::NucleicAcid>::string_to_sequence(sequence),
            shape: shape            
            }
        )
    }
}

