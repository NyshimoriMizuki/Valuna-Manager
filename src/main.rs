mod phex;
mod setupcl;
mod word;

mod repl;

use setupcl::SetupCL;
use word::WordGenerator;

use std::fs;

const VERSION: &str = "0.0.1";

fn main() {
    repl::Repl::new(VERSION).run();
}

#[allow(dead_code)]
fn using() {
    let setup = SetupCL::from_json("using/keidran-sucl.json");
    let mut generator = WordGenerator::new(&setup);

    let _list_of_meanings = fs::read_to_string("using/base_words.txt")
        .expect("Couldn't read the file")
        .lines()
        .map(String::from)
        .collect::<Vec<String>>();

    println!("{:?}", generator.gen_words(50));
    // for (words, meaning) in generator.gen_words_to(&_list_of_meanings, 3) {
    //     println!("{:?}\t\t{:?}", words, meaning);
    // }
}
