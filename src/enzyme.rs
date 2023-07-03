use std::collections::HashMap;
use std::error::Error;
use regex::Regex;

fn read_enzymes(file_path: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(file_path)?;
    let mut enzymes = HashMap::new();

    for result in rdr.records() {
        let record = result?;
        let sequence = record[0].to_string();
        let names = record[1].split_whitespace();

        for name in names {
            let clean_name = clean_name(name);
            enzymes.insert(clean_name, sequence.clone());
        }
    }

    Ok(enzymes)
}

fn clean_name(name: &str) -> String {
    let mut cleaned = name.to_string();
    cleaned.retain(|c| c.is_alphanumeric());
    return cleaned;
}

fn get_enzyme_sequence(enzyme_name: &str) -> Result<String, Box<dyn Error>> {
    let enzymes = read_enzymes("enzymes.csv")?;
    let re = Regex::new(r"[^a-zA-Z ]").unwrap(); // Matches anything that's not a letter or a space

    for (name, seq) in enzymes.iter() {
        if name.contains(enzyme_name) {
            let cleaned_seq = re.replace_all(seq, "");
            return Ok(cleaned_seq.to_string());
        }
    }

    Err(format!("No sequence found for enzyme {}", enzyme_name).into())
}


pub struct RestrictionEnzyme<'a> {
    name: &'a str,
    recognition_sequence: String,
    cut_offset: usize,
}

impl<'a> RestrictionEnzyme<'a> {
    pub fn new(name: &'a str) -> Result<Self, Box<dyn Error>> {
        let recognition_sequence = get_enzyme_sequence(name)?;
        let cut_offset = recognition_sequence.len() / 2;
        Ok(Self {
            name,
            recognition_sequence: recognition_sequence.to_string(),
            cut_offset,
        })
    }

    pub fn name(&self) -> &'a str {
        self.name
    }

    pub fn recognition_sequence(&self) -> &str {
        &self.recognition_sequence
    }
}
