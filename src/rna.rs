
use crate::base;
use crate::shape;
use crate::traits;

pub struct RNA {
    name: String,
    pub raw_sequence: String,
    sequence: Vec<base::RNABase>,
    shape: shape::Shape,
}

fn is_valid_rna_sequence(sequence: &str) -> bool {

    if sequence.is_empty() {
        return false;
    }

    let formatted_sequence = sequence.to_ascii_uppercase();
 
    formatted_sequence.chars().all(|c| match c {
        'A' | 'U' | 'C' | 'G' | 'N' => true,
        _ => false,
    })
}

impl RNA {
    pub fn new(name: &str, sequence: &str, shape: shape::Shape) -> Option<Self> {

        
        if !is_valid_rna_sequence(sequence) {
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

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_shape(&self) -> &shape::Shape {
        &self.shape
    }

    pub fn get_sequence(&self) -> &Vec<base::RNABase> {
        &self.sequence
    }

    fn morph(&mut self, shape: shape::Shape) {
        self.shape = shape;
    }
}
