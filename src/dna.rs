use crate::base;
use crate::shape;
use crate::traits;

pub struct DNA {
    name: String,
    /// We want to keep the raw sequence around for debugging purposes, as well as interop between lower and upper case
    pub raw_sequence: String,
    /// Sequence is the parsed version of the raw sequence
    sequence: Vec<base::DNABase>,
    /// Refers to the shape of the DNA molecule
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

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_shape(&self) -> &shape::Shape {
        &self.shape
    }

    pub fn get_sequence(&self) -> &Vec<base::DNABase> {
        &self.sequence
    }
}

