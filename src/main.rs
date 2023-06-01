mod phex;
mod setupcl;
mod word;

use setupcl::SetupCL;
use word::WordGenerator;

use std::fs;

fn main() {
    let file_content = fs::read_to_string("samples/teste.phex").expect("Error on reading");

    let mut lexer = phex::phex_lexer::PhexLexer::new(&file_content);
    lexer.tokenize();
    println!("{:?}", lexer.get_tokens());
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
