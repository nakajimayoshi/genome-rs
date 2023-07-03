#[derive(Debug, Clone)]
pub enum DNABase {
    ADENINE,
    CYTOSINE,
    THYMINE,
    GUANINE,
    ANY,
}

pub enum RNABase {
    ADENINE,
    CYTOSINE,
    GUANINE,
    URACIL,
    ANY,
}
