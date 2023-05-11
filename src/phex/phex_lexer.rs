use std::str::Chars;

const EOF_CHAR: char = '\0';

#[derive(Debug, Clone)]
pub struct PhexLexer<'a> {
    code: Chars<'a>,
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

    pub fn get_token(&self) -> Vec<Token> {
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
                let mut comment = String::new();
                loop {
                    if self.first() != c && self.first() != '\0' {
                        println!("-{}-", self.first());
                        comment.push_str(&self.next_char().unwrap().to_string());
                    } else {
                        self.next_char();
                        break;
                    }
                }
                Token::Comment(comment)
            }
            c if is_phoneme_or_keyword(c) => {
                let mut phoneme_or_keyword = String::from(c);
                loop {
                    if is_phoneme_or_keyword(self.first()) {
                        phoneme_or_keyword.push_str(&self.next_char().unwrap().to_string());
                    } else {
                        break;
                    }
                }
                Token::PhonemeOrKeyword(phoneme_or_keyword)
            }
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
            c if c.is_uppercase() => Token::Group(c.to_string()),
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
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    PhonemeOrKeyword(String),
    Group(String),
    Comment(String),
    Operator(String), //  → # _ + - /
    Null,             // ∅ or *
    LParam,           // (
    RParam,           // )
    LBracket,         // {
    RBracket,         // }
    LBrace,           // [
    RBrace,           // ]
    WhiteSpace,       //
    NewLine,          //
    SOF,              // Start of file
    EOF,              // End of file
    Unknown(String),
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
