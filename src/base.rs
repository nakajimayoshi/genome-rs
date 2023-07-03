#[derive(Debug, Clone)]
pub enum DNABase {
    ADENINE(char),
    CYTOSINE(char),
    THYMINE(char),
    GUANINE(char),
}

pub enum RNABase {
    ADENINE(char),
    CYTOSINE(char),
    GUANINE(char),
    URACIL(char),
}
