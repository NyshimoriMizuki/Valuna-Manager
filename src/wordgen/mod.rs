use crate::setupcl::{GroupOption, SetupCL};

use rand::{self, seq::SliceRandom};
use std::collections::HashMap;

pub struct WordGenerator {
    phonemes: HashMap<String, Vec<String>>,
    setup: SetupCL,
}
impl WordGenerator {
    pub fn new(setup: &SetupCL) -> WordGenerator {
        WordGenerator {
            phonemes: setup.get_phonemes(),
            setup: setup.clone(),
        }
    }

    fn gen_words(&self, num: usize) {}

    pub fn gen_syllable(&self) -> String {
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
