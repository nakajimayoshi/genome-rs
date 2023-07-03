use crate::base;
use crate::dna;
use crate::rna;

pub trait NucleicAcid {

    type BasePair;

    fn string_to_sequence(sequence: &str) -> Vec<Self::BasePair>;

    fn complement_base(&self, base: char) -> char;

    fn complement_raw_sequence(&self) -> String {
        let mut result = vec![];
    
        for base in self.raw_sequence().chars() {
            result.push(self.complement_base(base));
        }
    
        return result.into_iter().collect();
    }

    fn raw_sequence(&self) -> String;

    fn check_sequence(sequence: &str) -> Option<&str>;

}

impl NucleicAcid for dna::DNA {
    type BasePair = base::DNABase;
    fn string_to_sequence(sequence: &str) -> Vec<Self::BasePair> {
        let mut result = vec![];
    
        for base in sequence.chars() {
            match base.to_ascii_uppercase() {
                'A' => result.push(base::DNABase::ADENINE('A')),
                'C' => result.push(base::DNABase::CYTOSINE('C')),
                'T' => result.push(base::DNABase::THYMINE('T')),
                'G' => result.push(base::DNABase::GUANINE('G')),
                _ => println!("Empty space found in sequence {}", base)
            }
        }
        return result;
    }
    
    fn complement_raw_sequence(&self) -> String {
        let mut result = vec![];
        
        for base in self.raw_sequence().chars() {
            match base {
                'A' => result.push('T'),
                'C' => result.push('G'),
                'T' => result.push('A'),
                'G' => result.push('C'),
                _ => println!("No base found")
            }
        }
    
        return result.into_iter().collect();
    }
    
    fn raw_sequence(&self) -> String {
        return self.raw_sequence.clone();
    }
    
    fn complement_base(&self, base: char) -> char {
        match base {
            'A' => 'T',
            'C' => 'G',
            'T' => 'A',
            'G' => 'C',
            _ => panic!("Invalid base"),
        }
    }
    
    fn check_sequence(sequence: &str) -> Option<&str> {
        for c in sequence.chars() {
            match c {
                'A' | 'T' | 'C' | 'G' => {},
                _ => return None,
            }
        }
        Some(sequence)
    }
    
    
}

impl NucleicAcid for rna::RNA {
    type BasePair = base::RNABase;
    
    fn string_to_sequence(sequence: &str) -> Vec<Self::BasePair> {
        let mut result = vec![];
    
        for base in sequence.chars() {
            match base.to_ascii_uppercase() {
                'A' => result.push(base::RNABase::ADENINE('A')),
                'C' => result.push(base::RNABase::CYTOSINE('C')),
                'T' => result.push(base::RNABase::URACIL('U')),
                'G' => result.push(base::RNABase::GUANINE('G')),
                _ => println!("Empty space found in sequence {}", base)
            }
        }
        return result;
    }
    
    fn complement_raw_sequence(&self) -> String {
        let mut result = vec![];
        
        for base in self.raw_sequence().chars() {
            match base {
                'A' => result.push('U'),
                'C' => result.push('G'),
                'U' => result.push('A'),
                'G' => result.push('C'),
                _ => println!("No base found")
            }
        }
    
        return result.into_iter().collect();
    }
    
    fn raw_sequence(&self) -> String {
        return self.raw_sequence.clone();
    }
    
    fn complement_base(&self, base: char) -> char {
        match base {
            'A' => 'U',
            'C' => 'G',
            'U' => 'A',
            'G' => 'C',
            _ => panic!("Invalid base"),
        }
    }
    
    fn check_sequence(sequence: &str) -> Option<&str> {
        for c in sequence.chars() {
            match c {
                'A' | 'U' | 'C' | 'G' => {},
                _ => return None,
            }
        }
        Some(sequence)
    }
    
    
}