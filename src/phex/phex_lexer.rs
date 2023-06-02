use super::token::Token;
use std::str::Chars;

const EOF_CHAR: char = '\0';

#[derive(Debug, Clone)]
pub struct PhexLexer<'a> {
    pub(super) code: Chars<'a>,
    tokens: Vec<Token>,
    last_token: Token,
}

impl<'a> PhexLexer<'a> {
    pub fn new(sourcecode: &'a str) -> PhexLexer<'a> {
        PhexLexer {
            code: sourcecode.chars(),
            tokens: Vec::new(),
            last_token: Token::SOF,
        }
    }

    pub fn get_tokens(&self) -> Vec<Token> {
        self.tokens.clone()
    }

    pub fn tokenize(&mut self) {
        loop {
            let new_token = self.next_token();
            self.tokens.push(new_token.clone());

            if new_token == Token::EOF {
                break;
            }
        }
    }

    fn next_token(&mut self) -> Token {
        let current_char = match self.next_char() {
            Some(c) => c,
            None => return Token::EOF,
        };

        let new_token = match current_char {
            c if c == '\'' || c == '"' => {
                Token::Comment(self.build_large_value(c, &|x| x != c && x != '\0') + &c.to_string())
            }
            c if c.is_uppercase() && self.first().is_alphabetic() => {
                Token::Identifier(self.build_large_value(c, &|x| is_phoneme_or_keyword(x)))
            }
            c if is_phoneme_or_keyword(c) => {
                // comment only to break a line
                Token::Phoneme(self.build_large_value(c, &|x| is_phoneme_or_keyword(x)))
            }
            c if c.is_uppercase() => Token::Group(c.to_string()),
            '-' => {
                if self.first() == '>' {
                    self.next_char();
                    Token::Operator('→'.to_string())
                } else {
                    Token::Operator('-'.to_string())
                }
            }
            c if c == '\r' && self.first() == '\n' => {
                self.next_char();
                Token::NewLine
            }
            c if is_operator(c) => Token::Operator(c.to_string()),
            '→' | '>' => Token::Operator('→'.to_string()),
            '∅' | '*' => Token::Null,
            ' ' | '\t' => Token::WhiteSpace,
            '\n' => Token::NewLine,
            '(' => Token::LParam,
            ')' => Token::RParam,
            '{' => Token::LBracket,
            '}' => Token::RBracket,
            '[' => Token::LBrace,
            ']' => Token::RBrace,
            c => Token::Unknown(c.to_string()),
        };

        self.last_token = new_token.clone();
        new_token
    }

    fn first(&self) -> char {
        self.code.clone().next().unwrap_or(EOF_CHAR)
    }
    fn next_char(&mut self) -> Option<char> {
        let c = self.code.next()?;
        Some(c)
    }

    fn build_large_value(&mut self, start_char: char, f: &dyn Fn(char) -> bool) -> String {
        let mut building = String::from(start_char);
        loop {
            if f(self.first()) {
                building.push_str(&self.next_char().unwrap().to_string());
            } else {
                break;
            }
        }
        building
    }
}

fn is_operator(char_: char) -> bool {
    match char_ {
        '→' | '#' | '_' | '+' | '-' | '/' => true,
        _ => false,
    }
}

fn is_phoneme_or_keyword(char_: char) -> bool {
    match char_ {
        c if c.is_whitespace() || c.is_uppercase() => false,
        '-' | '>' | '→' | ',' | '/' | '∅' | '*' | '+' | '{' | '}' | '[' | ']' | '<' | '&' | '@'
        | '%' | '#' | '!' | '|' | '$' | '_' | '\0' => false,
        _ => true,
    }
}
