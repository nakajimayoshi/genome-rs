#[cfg(test)]
mod tests {
    use crate::dna::DNA;
    use crate::rna::RNA;
    use crate::shape::Shape;
    use crate::traits::NucleicAcid;


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
       assert!(DNA::new("err4", "AA TT", Shape::LINEAR).is_none());
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
       assert!(RNA::new("err4", "AA UU", Shape::LINEAR).is_none());
    }
    // Rest of the tests
}