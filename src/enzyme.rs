pub enum RestrictionEnzymeType {
    AatII,
    AgeI,
    ApaI,
    AvaI,
    BamHI,
    BglII,
    BsaI,
    BsmBI,
    BsrFI,
    BsrGI,
    BssHII,
    BstEII,
    BstZ17I,
    CspI,
    ClaI,
    DpnI,
    DraI,
    EagI,
    EcoNI,
    EcoR1,
    EcoRII,
    FokI,
    HaeII,
    HaeIII,
    HhaI,
    HincII,
    HindIII,
    HpaI,
    HpaII,
    Kpn2I,
    KpnI,
    MboI,
    MfeI,
    MluI,
    MluNI,
    NcoI,
    NdeI,
    NdeII,
    NheI,
    NlaIII,
    NlaIV,
    NotI,
    PmeI,
    PstI,
    PsiI,
    PvuI,
    PvuII,
    RsaI,
    SacI,
    SacII,
    SalI,
    ScaI,
    SfiI,
    SgrAI,
    SmaI,
    SnaBI,
    SnaI,
    SphI,
    SspI,
    SpeI,
    StuI,
    TaqI,
    TliI,
    Tth111I,
    Uba1314I,
    XbaI,
    XhoI,
    XmaCI,
    XmaI,
}

fn get_recognition_sequence(enzyme: RestrictionEnzymeType) -> &'static str {
    match enzyme {
        RestrictionEnzymeType::AatII => "GACGTC",
        RestrictionEnzymeType::AgeI => "ACCGGT",

        _ => panic!("Invalid enzyme type provided"),
    }
}

pub struct RestrictionEnzyme {
    name: String,
    enzyme_type: RestrictionEnzymeType,
    recognition_sequence: String,
    cut_offset: usize,
}

impl RestrictionEnzyme {
    pub fn new(name: &str, enzyme_type: RestrictionEnzymeType, recognition_sequence: &str, cut_offset: usize) -> Self {
        Self {
            name: name.to_string(),
            enzyme_type,
            recognition_sequence: recognition_sequence.to_string(),
            cut_offset,
        }
    }

    
}