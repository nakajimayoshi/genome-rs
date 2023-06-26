#[cfg(test)]
mod tests {
    use crate::{DNA, RNA, Shape, NucleicAcid};

   #[test]
   fn dna_complement_sequence() {
      let strands = [
         DNA::new("test0", "ACTG", Shape::LINEAR).unwrap(),
         DNA::new("test1", "TTAA", Shape::LINEAR).unwrap(),
         DNA::new("test2", "GCCGTTACGGCCAA", Shape::LINEAR).unwrap(),
         DNA::new("test3", "A", Shape::LINEAR).unwrap(),
      ];

      assert_eq!(strands[0].complement_raw_sequence(), "TGAC");
      assert_eq!(strands[1].complement_raw_sequence(), "AATT");
      assert_eq!(strands[2].complement_raw_sequence(), "CGGCAATGCCGGTT");
      assert_eq!(strands[3].complement_raw_sequence(), "T");

  
   }

   #[test]
   fn dna_throws_error() {
      assert!(DNA::new("err1", "", Shape::LINEAR).is_none());
      assert!(DNA::new("err2", "FFF", Shape::LINEAR).is_none());
      assert!(DNA::new("err3", "123", Shape::LINEAR).is_none());
      assert!(DNA::new("err4", "--++", Shape::LINEAR).is_none());
   }

   #[test]
   fn rna_complement_sequence() {
      let strands = [
         RNA::new("test0", "ACUG", Shape::LINEAR).unwrap(),
         RNA::new("test1", "UUAA", Shape::LINEAR).unwrap(),
         RNA::new("test2", "GCCGUUACGGCCAA", Shape::LINEAR).unwrap(),
         RNA::new("test3", "A", Shape::LINEAR).unwrap(),
      ];

      assert_eq!(strands[0].complement_raw_sequence(), "UGAC");
      assert_eq!(strands[1].complement_raw_sequence(), "AAUU");
      assert_eq!(strands[2].complement_raw_sequence(), "CGGCAAUGCCGGUU");
      assert_eq!(strands[3].complement_raw_sequence(), "U");

   }

   #[test]
   fn rna_throws_error() {
      assert!(RNA::new("err1", "", Shape::LINEAR).is_none());
      assert!(RNA::new("err2", "FFF", Shape::LINEAR).is_none());
      assert!(RNA::new("err3", "123", Shape::LINEAR).is_none());
      assert!(RNA::new("err4", "--++", Shape::LINEAR).is_none());
   }
}


trait Plasmid {
    fn debug_print() {
       println!("This is a plasmid")
    }
 }
 
 trait Genome {
    fn debug_print() {
       println!("This is a Genome")
    }
 }
 
 #[derive(Debug, Clone)]
 enum DNABase {
    ADENINE(char),
    CYTOSINE(char),
    THYMINE(char),
    GUANINE(char),
 }
 
 enum RNABase {
    ADENINE(char),
    CYTOSINE(char),
    GUANINE(char),
    URACIL(char)
 }
 
 
 
 #[derive(Debug, Clone)]
 enum Shape {
    CIRCULAR,
    LINEAR
 }
 
 enum Enzyme {
    Tth111I,
    EcoR1,
 }
 
 trait NucleicAcid {
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
 
 impl NucleicAcid for DNA {
    type BasePair = DNABase;
    fn string_to_sequence(sequence: &str) -> Vec<Self::BasePair> {
       let mut result = vec![];
    
       for base in sequence.chars() {
          match base.to_ascii_uppercase() {
             'A' => result.push(DNABase::ADENINE('A')),
             'C' => result.push(DNABase::CYTOSINE('C')),
             'T' => result.push(DNABase::THYMINE('T')),
             'G' => result.push(DNABase::GUANINE('G')),
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
 
 impl NucleicAcid for RNA {
    type BasePair = RNABase;
 
    fn string_to_sequence(sequence: &str) -> Vec<Self::BasePair> {
       let mut result = vec![];
    
       for base in sequence.chars() {
          match base.to_ascii_uppercase() {
             'A' => result.push(RNABase::ADENINE('A')),
             'C' => result.push(RNABase::CYTOSINE('C')),
             'T' => result.push(RNABase::URACIL('U')),
             'G' => result.push(RNABase::GUANINE('G')),
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
 
 
 struct DNA {
    name: String,
    raw_sequence: String,
    sequence: Vec<DNABase>,
    shape: Shape,
 }
 
 impl DNA {
    pub fn new(name: &str, sequence: &str, shape: Shape) -> Option<Self> {
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
          sequence: Self::string_to_sequence(sequence),
          shape: shape            
          }
       )
    }
 }
 
 
 struct RNA {
    name: String,
    raw_sequence: String,
    sequence: Vec<RNABase>,
    shape: Shape,
 }
 
 impl RNA {
    pub fn new(name: &str, sequence: &str, shape: Shape) -> Option<Self> {
       if sequence.is_empty() || sequence.chars().any(|c| match c {
          'A' | 'U' | 'C' | 'G' => false,
          _ => true,
      }) {
          println!("Invalid sequence provided");
          return None;
      }
       Some (
          Self {
          name: name.to_string(),
          raw_sequence: sequence.to_string(),
          sequence: Self::string_to_sequence(sequence),
          shape: shape            
          }
       )
    }
 }
 
 
 
 