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

}

impl NucleicAcid for dna::DNA {
    type BasePair = base::DNABase;
    fn string_to_sequence(sequence: &str) -> Vec<Self::BasePair> {
        let mut result = vec![];
    
        for base in sequence.chars() {
            match base.to_ascii_uppercase() {
                'A' => result.push(base::DNABase::ADENINE),
                'C' => result.push(base::DNABase::CYTOSINE),
                'T' => result.push(base::DNABase::THYMINE),
                'G' => result.push(base::DNABase::GUANINE),
                'N' => result.push(base::DNABase::ANY),
                _ => println!("Empty space found in sequence {}", base)
            }
        }
        return result;
    }
    
    fn complement_raw_sequence(&self) -> String {
        let mut result = vec![];
        
        for base in self.raw_sequence().chars() {
            match base.to_ascii_uppercase() {
                'A' => result.push('T'),
                'C' => result.push('G'),
                'T' => result.push('A'),
                'G' => result.push('C'),
                'N' => result.push('N'),
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
            'N' => 'N',
            _ => panic!("Invalid base"),
        }
    }
    
}

impl NucleicAcid for rna::RNA {
    type BasePair = base::RNABase;
    
    fn string_to_sequence(sequence: &str) -> Vec<Self::BasePair> {
        let mut result = vec![];
        let mut index = 0;
    
        for base in sequence.chars() {
            index += 1;
            match base.to_ascii_uppercase() {
                'A' => result.push(base::RNABase::ADENINE),
                'C' => result.push(base::RNABase::CYTOSINE),
                'U' => result.push(base::RNABase::URACIL),
                'G' => result.push(base::RNABase::GUANINE),
                'N' => result.push(base::RNABase::ANY),
                _ => println!("Invalid char found in sequence {} at index: {}", sequence, index)
            }
        }
        return result;
    }
    
    fn complement_raw_sequence(&self) -> String {
        let mut result = vec![];
        
        for base in self.raw_sequence().chars() {
            match base.to_ascii_uppercase() {
                'A' => result.push('U'),
                'C' => result.push('G'),
                'U' => result.push('A'),
                'G' => result.push('C'),
                'N' => result.push('N'),
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
            'N' => 'N',
            _ => panic!("Invalid base"),
        }
    }    
}