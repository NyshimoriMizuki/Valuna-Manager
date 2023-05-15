mod phex_lexer;
mod phex_parser;

use phex_lexer::PhexLexer;
use phex_parser::PhexParser;

use std::fs;

pub struct PhexReader<'a> {
    raw_file: String,
    lexer: PhexLexer<'a>,
    parser: PhexParser,
    expressions: Vec<Phex>,
}

impl<'a> PhexReader<'a> {
    pub fn new(&self, phex_file: &'a String) -> PhexReader<'a> {
        PhexReader {
            raw_file: phex_file.to_string(),
            lexer: PhexLexer::new(phex_file),
            parser: PhexParser::new(),
            expressions: Vec::new(),
        }
    }
    pub fn teste() {
        let source = fs::read_to_string("./samples/teste.phex").expect("Error on reading");

        let mut lexer = PhexLexer::new(&source);
        lexer.tokenize();

        let mut parser = PhexParser::new();
        parser.load_tokens(&lexer.get_tokens());
        parser.parse();

        println!("{:?}", parser.get_expressions());
    }
}

#[derive(Debug, Clone)]
pub struct Phex {
    left: String,
    right: String,
    case: String,
}
