use crate::base;
use crate::dna;
use crate::rna;

/// `NucleicAcid` trait represents the common behavior of nucleic acids.
///
/// # Examples
///
/// Implementations for DNA and RNA are provided.
pub trait NucleicAcid {
    /// The base pair type.
    type BasePair;

    /// Converts a string to a sequence of base pairs.
    ///
    /// # Parameters
    ///
    /// * `sequence`: A string representation of the nucleic acid sequence.
    ///
    /// # Returns
    ///
    /// A `Vec` containing the base pairs in the sequence.
    fn string_to_sequence(sequence: &str) -> Vec<Self::BasePair>;

    /// Returns the complement of a base.
    ///
    /// # Parameters
    ///
    /// * `base`: A `char` representation of the base.
    ///
    /// # Returns
    ///
    /// The `char` representation of the complement base.
    fn complement_base(&self, base: char) -> char;

    /// Returns the complement of a sequence.
    ///
    /// # Returns
    ///
    /// The `String` representation of the complement sequence.
    fn complement_raw_sequence(&self) -> String {
        let mut result = vec![];
    
        for base in self.raw_sequence().chars() {
            result.push(self.complement_base(base));
        }
    
        return result.into_iter().collect();
    }

    /// Returns the raw sequence as a String.
    ///
    /// # Returns
    ///
    /// The `String` representation of the raw sequence.
    fn raw_sequence(&self) -> String;

}

impl NucleicAcid for dna::DNA {
    type BasePair = base::DNABase;

    /// Converts a string to a sequence of DNA specific base pairs
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

    /// Converts a string to a sequence of RNA specific base pairs
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