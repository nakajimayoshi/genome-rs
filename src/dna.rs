use crate::base;
use crate::shape;
use crate::traits;

pub struct DNA {
    name: String,
    pub raw_sequence: String,
    sequence: Vec<base::DNABase>,
    shape: shape::Shape,
}

impl DNA {
    pub fn new(name: &str, sequence: &str, shape: shape::Shape) -> Option<Self> {
        if sequence.is_empty() || sequence.chars().any(|c| match c {
            'A' | 'T' | 'C' | 'G' => false,
            _ => true,
        }) {
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

