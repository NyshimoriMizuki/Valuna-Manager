pub mod phex_lexer;
mod phex_parser;
mod token;

use phex_lexer::PhexLexer;
use phex_parser::PhexParser;

use regex::Regex;

pub struct PhexReader<'a> {
    lexer: PhexLexer<'a>,
    parser: PhexParser,
    expressions: Vec<Phex>,
}

impl<'a> PhexReader<'a> {
    pub fn new(phex_text: &'a str) -> PhexReader<'a> {
        PhexReader {
            lexer: PhexLexer::new(phex_text),
            parser: PhexParser::new(),
            expressions: Vec::new(),
        }
    }

    pub fn read(&mut self) {
        self.lexer.tokenize();
        self.parser.parse(&self.lexer.get_tokens());

        self.expressions = self.parser.get_and_clear_expressions();
    }

    pub fn run_all(&self, base_words: &Vec<String>) -> Vec<(String, String)> {
        let mut new_words: Vec<(String, String)> = Vec::new();

        for word in base_words {
            let mut current_word = word.to_string();
            for phex in &self.expressions {
                current_word = phex.run(&current_word);
            }
            new_words.push((word.to_string(), current_word));
        }
        new_words
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
