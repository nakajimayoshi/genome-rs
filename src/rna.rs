
use crate::base;
use crate::shape;
use crate::traits;

pub struct RNA {
    name: String,
    pub raw_sequence: String,
    sequence: Vec<base::RNABase>,
    shape: shape::Shape,
}

impl RNA {
    pub fn new(name: &str, sequence: &str, shape: shape::Shape) -> Option<Self> {
        let is_invalid_sequence = sequence.is_empty() || sequence.chars().any(|c| match c {
            'A' | 'U' | 'C' | 'G' => false,
            _ => true,
        }); 
        
        if is_invalid_sequence {
            println!("Invalid sequence provided");
            return None;
        }
        
        Some (
            Self {
            name: name.to_string(),
            raw_sequence: sequence.to_string(),
            sequence: <RNA as traits::NucleicAcid>::string_to_sequence(sequence),
            shape: shape            
            }
        )
    }
    }
