use crate::setupcl::{GroupOption, SetupCL};

use rand::{self, seq::SliceRandom, Rng};
use std::collections::HashMap;

pub struct WordGenerator {
    phonemes: HashMap<String, Vec<String>>,
    setup: SetupCL,
    words_list: Vec<String>,
}
impl WordGenerator {
    pub fn new(setup: &SetupCL) -> WordGenerator {
        WordGenerator {
            phonemes: setup.get_phonemes(),
            setup: setup.clone(),
            words_list: Vec::new(),
        }
    }

    pub fn gen_words(&mut self, num: usize) -> Vec<String> {
        let mut new_words: Vec<String> = Vec::new();
        for _ in 0..num {
            let word = self.gen_word();
            new_words.push(word);
        }
        new_words
    }

    pub fn gen_word(&self) -> String {
        let mut word = String::new();
        for _ in 0..choice_word_len(self.setup.get_word_length()) {
            word.push_str(&self.gen_syllable());
        }

        word
    }

    pub fn get_words(&self) -> Vec<String> {
        self.words_list.clone()
    }

    fn gen_syllable(&self) -> String {
        let mut building_syllable = String::new();

        for group in self.setup.break_syllable_struct() {
            match group {
                GroupOption::Obl(g) => building_syllable.push_str(&self.get_a_phoneme(&g)),
                GroupOption::Opt(g) => {
                    if isnt_to_jump() {
                        building_syllable.push_str(&self.get_a_phoneme(&g));
                    } else {
                        continue;
                    }
                }
            }
        }
        building_syllable
    }

    fn get_a_phoneme(&self, group: &str) -> String {
        match self.phonemes.get(group) {
            Some(c) => c
                .to_vec()
                .choose(&mut rand::thread_rng())
                .unwrap()
                .to_owned(),
            None => panic!("Non existent group"),
        }
    }
}

fn isnt_to_jump() -> bool {
    let ret = [true, false].choose(&mut rand::thread_rng()).unwrap();
    *ret
}
fn choice_word_len(max: u32) -> u32 {
    rand::thread_rng().gen_range(1..max)
}
