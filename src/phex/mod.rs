mod phex_lexer;
mod phex_parser;

use phex_lexer::PhexLexer;
use phex_parser::PhexParser;

use regex::Regex;
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

        let targets = vec!["aho", "kaho", "paho", "ao", "kow", "luw", "amwu", "qauwuij"];
        for word in targets {
            let mut result = String::from(word);
            for i in parser.get_expressions() {
                result = i.run(&result);
            }
            println!("{word} -> {result}");
        }
    }
}

#[derive(Debug, Clone)]
pub struct Phex {
    left: String,
    right: String,
    case: String,
}
impl Phex {
    fn new(left: String, right: String, case: String) -> Phex {
        Phex { left, right, case }
    }

    fn run(&self, target: &str) -> String {
        let pattern = self.case.replace("_", &self.left);
        let subtitute = making_replace_pattern(&self.case).replace("_", &{
            let mut return_value = String::new();
            if self.right == String::from("∅") {
                return_value = "".to_string();
            } else {
                return_value = self.right.clone();
            }
            return_value
        });

        Regex::new(&pattern)
            .unwrap()
            .replace_all(target, &subtitute)
            .into_owned()
    }
}

fn making_replace_pattern(target: &str) -> String {
    let mut index = 0;
    let mut replace_pattern = Regex::new(r"(\^|\$)")
        .unwrap()
        .replace(target, "")
        .into_owned();

    while let Some(_) = replace_pattern.find("(") {
        index += 1;

        replace_pattern = Regex::new(r"\([^()]*\)")
            .unwrap()
            .replace(&replace_pattern, "@")
            .into_owned()
            .replace("@", &index_to_found_group(index as usize));
    }
    replace_pattern
}

fn index_to_found_group(num: usize) -> String {
    String::from("${") + &num.to_string() + "}"
}
