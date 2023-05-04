mod phex_lexer;
mod phex_parser;

use phex_lexer::PhexLexer;
use phex_parser::PhexParser;

use std::fs;

pub struct Phex<'a> {
    raw_file: String,
    lexer: PhexLexer<'a>,
    parser: PhexParser,
    expressions: Vec<PhexExpression>,
}

impl Phex<'_> {
    pub fn new(phex_file: &String) -> Phex<'_> {
        Phex {
            raw_file: phex_file.to_string(),
            lexer: PhexLexer::new(phex_file),
            parser: PhexParser,
            expressions: Vec::new(),
        }
    }
    pub fn teste() {
        let source = fs::read_to_string("./samples/teste.phex").expect("Error on reading");

        let mut lexer = PhexLexer::new(&source);
        lexer.tokenize();

        println!("{:?}", lexer);
    }
}
struct PhexExpression {
    left: &'static str,
    right: &'static str,
    case: &'static str,
}
