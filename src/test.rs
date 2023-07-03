#[cfg(test)]
mod nucleotide_tests {
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
          DNA::new("test4", "N", Shape::LINEAR).unwrap(),
          DNA::new("test5", "a", Shape::LINEAR).unwrap(),
          DNA::new("test6", "aattccgg", Shape::LINEAR).unwrap(),
          DNA::new("test7", "nNatcgatt", Shape::LINEAR).unwrap(),
       ];
 
       assert_eq!(strands[0].complement_raw_sequence(), "TGAC");
       assert_eq!(strands[1].complement_raw_sequence(), "AATT");
       assert_eq!(strands[2].complement_raw_sequence(), "CGGCAATGCCGGTT");
       assert_eq!(strands[3].complement_raw_sequence(), "T");
       assert_eq!(strands[4].complement_raw_sequence(), "N");
       assert_eq!(strands[5].complement_raw_sequence(), "T");
       assert_eq!(strands[6].complement_raw_sequence(), "TTAAGGCC");
       assert_eq!(strands[7].complement_raw_sequence(), "NNTAGCTAA");
 
   
    }
 
    #[test]
    fn dna_throws_error() {
       assert!(DNA::new("err1", "", Shape::LINEAR).is_none());
       assert!(DNA::new("err2", "FFF", Shape::LINEAR).is_none());
       assert!(DNA::new("err3", "123", Shape::LINEAR).is_none());
       assert!(DNA::new("err4", "--++", Shape::LINEAR).is_none());
       assert!(DNA::new("err4", "AA TT", Shape::LINEAR).is_none());
       assert!(DNA::new("err4", "AA NTT", Shape::LINEAR).is_none());
    }
 
    #[test]
    fn rna_complement_sequence() {
       let strands = [
          RNA::new("test0", "ACUG", Shape::LINEAR).unwrap(),
          RNA::new("test1", "UUAA", Shape::LINEAR).unwrap(),
          RNA::new("test2", "GCCGUUACGGCCAA", Shape::LINEAR).unwrap(),
          RNA::new("test3", "A", Shape::LINEAR).unwrap(),
          RNA::new("test4", "N", Shape::LINEAR).unwrap(),
          RNA::new("test5", "a", Shape::LINEAR).unwrap(),
          RNA::new("test6", "aauuucccg", Shape::LINEAR).unwrap(),
          RNA::new("test7", "aunNnnn", Shape::LINEAR).unwrap(),
       ];
 
       assert_eq!(strands[0].complement_raw_sequence(), "UGAC");
       assert_eq!(strands[1].complement_raw_sequence(), "AAUU");
       assert_eq!(strands[2].complement_raw_sequence(), "CGGCAAUGCCGGUU");
       assert_eq!(strands[3].complement_raw_sequence(), "U");
       assert_eq!(strands[4].complement_raw_sequence(), "N");
       assert_eq!(strands[5].complement_raw_sequence(), "U");
       assert_eq!(strands[6].complement_raw_sequence(), "UUAAAGGGC");
       assert_eq!(strands[7].complement_raw_sequence(), "UANNNNN");
 
    }
 
    #[test]
    fn rna_throws_error() {
       assert!(RNA::new("err1", "", Shape::LINEAR).is_none());
       assert!(RNA::new("err2", "FFF", Shape::LINEAR).is_none());
       assert!(RNA::new("err3", "123", Shape::LINEAR).is_none());
       assert!(RNA::new("err4", "--++", Shape::LINEAR).is_none());
       assert!(RNA::new("err4", "AA UU", Shape::LINEAR).is_none());
       assert!(RNA::new("err4", "AA NUU", Shape::LINEAR).is_none());
    }
    // Rest of the tests
}


#[cfg(test)]
mod enzyme_tests {
   use crate::enzyme;

   #[test]
   fn test_get_sequence() {
      let eco_r1 = enzyme::RestrictionEnzyme::new("EcoRI").unwrap();
      assert_eq!(eco_r1.recognition_sequence(), "GAATTC");

      let eco_rv = enzyme::RestrictionEnzyme::new("EcoRV").unwrap();
      assert_eq!(eco_rv.recognition_sequence(), "GATATC");

      let bsa_ai = enzyme::RestrictionEnzyme::new("BsaAI").unwrap();
      assert_eq!(bsa_ai.recognition_sequence(), "YACGTR");

      let bsa_bi = enzyme::RestrictionEnzyme::new("BsaBI").unwrap();
      assert_eq!(bsa_bi.recognition_sequence(), "GATNNNNATC");

      let bsa_hi = enzyme::RestrictionEnzyme::new("BsaHI").unwrap();
      assert_eq!(bsa_hi.recognition_sequence(), "GRCGYC");

      let bsi_ei = enzyme::RestrictionEnzyme::new("BsiEI").unwrap();
      assert_eq!(bsi_ei.recognition_sequence(), "CGRYCG");

      let bsr_bi = enzyme::RestrictionEnzyme::new("BsrBI").unwrap();
      assert_eq!(bsr_bi.recognition_sequence(), "CCGCTC");

      let bst_bi = enzyme::RestrictionEnzyme::new("BstBI").unwrap();
      assert_eq!(bst_bi.recognition_sequence(), "TTCGAA");

      let bst_ui = enzyme::RestrictionEnzyme::new("BstUI").unwrap();
      assert_eq!(bst_ui.recognition_sequence(), "CGCG");

      let bsa_wi = enzyme::RestrictionEnzyme::new("BsaWI").unwrap();
      assert_eq!(bsa_wi.recognition_sequence(), "WCCGGW");

   }

}