use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fs;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetupCL {
    phonemes: HashMap<String, Vec<String>>,
    syllable_struct: String,
    word_length: u32,
}

impl SetupCL {
    pub fn from_json(file: &str) -> SetupCL {
        let content = fs::read_to_string(file).expect("Failed to read the json file");
        serde_json::from_str::<SetupCL>(&content).expect("Error on parsing the json file")
    }

    pub fn break_syllable_struct(&self) -> Vec<GroupOption> {
        let re_group = Regex::new(r"\(?(\w)\)?").unwrap();
        let mut ret: Vec<GroupOption> = Vec::new();

        for cap in re_group.captures_iter(&self.syllable_struct) {
            let group_name = cap.get(1).unwrap().as_str().to_string();
            if cap.get(0) != cap.get(1) {
                ret.push(GroupOption::Opt(group_name));
            } else {
                ret.push(GroupOption::Obl(group_name));
            }
        }
        ret
    }

    pub fn get_phonemes(&self) -> HashMap<String, Vec<String>> {
        self.phonemes.clone()
    }
    pub fn get_syllable_struct(&self) -> String {
        self.syllable_struct.clone()
    }
    pub fn get_word_length(&self) -> u32 {
        self.word_length
    }
}

pub enum GroupOption {
    // Optional
    Opt(String),
    // Obligatory
    Obl(String),
}
