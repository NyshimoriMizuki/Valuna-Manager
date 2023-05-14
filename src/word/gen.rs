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

    pub fn gen_words_to(
        &mut self,
        words: &Vec<String>,
        num_of_options: u32,
    ) -> Vec<(Vec<String>, String)> {
        let mut new_words: Vec<(Vec<String>, String)> = Vec::new();
        for word in words {
            let words_options = self.gen_words(num_of_options.try_into().unwrap());
            new_words.push((words_options, word.to_string()));
        }
        new_words
    }

    pub fn gen_words(&mut self, num: usize) -> Vec<String> {
        let mut new_words: Vec<String> = Vec::new();
        for _ in 0..num {
            let word = self.gen_word();
            new_words.push(word);
        }
        new_words
    }

    pub fn gen_word(&mut self) -> String {
        let mut word = String::new();
        for _ in 0..choice_word_len(self.setup.get_word_length()) {
            word.push_str(&self.gen_syllable());
        }
        self.words_list.push(word.clone());
        word
    }

    pub fn get_words(&self) -> Vec<String> {
        self.words_list.clone()
    }

    fn gen_syllable(&self) -> String {
        let mut new_syllable = String::new();

        for group in self.setup.break_syllable_struct() {
            match group {
                GroupOption::Obl(g) => new_syllable.push_str(&self.get_a_phoneme(&g)),
                GroupOption::Opt(g) => {
                    if isnt_to_jump() {
                        new_syllable.push_str(&self.get_a_phoneme(&g));
                    } else {
                        continue;
                    }
                }
            }
        }
        new_syllable
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
