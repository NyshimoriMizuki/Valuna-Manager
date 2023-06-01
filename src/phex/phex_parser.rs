use regex::Regex;
use std::collections::HashMap;

use super::token::Token;

#[derive(Debug)]
pub struct PhexParser {
    phoneme_groups: HashMap<String, Vec<String>>,
    expressions: Vec<super::Phex>,
}

impl PhexParser {
    pub fn new() -> PhexParser {
        PhexParser {
            phoneme_groups: HashMap::new(),
            expressions: Vec::new(),
        }
    }

    pub fn parse(&mut self, tokens: &Vec<Token>) {
        for tokens in self.split_in_lines(tokens) {
            let mut tokens_to_node: Vec<PhexNode> = Vec::new();
            let mut id = 0;

            loop {
                let t = match tokens.get(id) {
                    Some(t) => t.clone(),
                    None => break,
                };

                match t {
                    Token::Phoneme(c) => {
                        tokens_to_node.push(PhexNode::PU(PhoneUnit::Single(c.to_string())))
                    }
                    Token::Operator(c) => tokens_to_node.push(match &c[..] {
                        "/" => PhexNode::OperSlash,
                        "_" => PhexNode::OperPlaceholder,
                        "#" => PhexNode::OperBoundery,
                        "→" => PhexNode::OperTo,
                        c => PhexNode::Unknown(c.to_string()),
                    }),
                    Token::LBracket => {
                        let mut new_set: Vec<String> = Vec::new();
                        loop {
                            let s = match tokens.get(id) {
                                Some(t) => t,
                                None => break,
                            };
                            if *s == Token::RBracket {
                                tokens_to_node.push(PhexNode::PU(PhoneUnit::Set(new_set)));
                                break;
                            } else if let Token::Phoneme(p) = s {
                                new_set.push(p.to_string());
                            }
                            id += 1;
                        }
                    }
                    Token::WhiteSpace => tokens_to_node.push(PhexNode::Space),
                    Token::NewLine => tokens_to_node.push(PhexNode::NewLine),
                    Token::EOF => tokens_to_node.push(PhexNode::EOF),
                    Token::Null => tokens_to_node.push(PhexNode::Phoneme("∅".to_string())),
                    _ => continue,
                }

                id += 1;
            }

            let final_nodes: Vec<PhexNode> = {
                let mut building_tokens_vector: Vec<PhexNode> = Vec::new();
                for token in tokens_to_node {
                    if let PhexNode::PU(c) = token {
                        building_tokens_vector.push(match c {
                            PhoneUnit::Single(s) => PhexNode::Phoneme(make_optionals(&s)),
                            PhoneUnit::Set(s) => PhexNode::Set(make_set(&s)),
                        });
                    } else {
                        building_tokens_vector.push(token);
                    }
                }
                building_tokens_vector
            };

            let mut splited: Vec<Vec<PhexNode>> = Vec::new();
            let mut part: Vec<PhexNode> = Vec::new();
            for i in final_nodes.into_iter() {
                match i {
                    PhexNode::OperTo | PhexNode::OperSlash => {
                        splited.push(part);
                        part = Vec::new();
                    }
                    PhexNode::Space => continue,
                    c => part.push(c),
                }
            }
            splited.push(part);

            match splited.get(2) {
                Some(_) => (),
                None => splited.push(vec![PhexNode::OperPlaceholder]),
            }
            for (l, r) in splited
                .get(0)
                .unwrap()
                .iter()
                .zip(splited.get(1).unwrap().iter())
            {
                let (left, right) = match (l, r) {
                    (PhexNode::Set(ss), PhexNode::Phoneme(sp))
                    | (PhexNode::Phoneme(ss), PhexNode::Phoneme(sp)) => {
                        (ss.to_string(), sp.to_string())
                    }
                    (_, _) => ("∅".to_string(), "∅".to_string()),
                };
                let case = build_case(splited.get(2).unwrap());

                let expression = super::Phex::new(left, right, case);
                self.expressions.push(expression);
            }
        }
    }

    pub fn get_and_clear_expressions(&mut self) -> Vec<super::Phex> {
        let exprs = self.expressions.clone();
        self.expressions = Vec::new();

        exprs
    }

    fn split_in_lines(&self, tokens: &Vec<Token>) -> Vec<Vec<Token>> {
        let mut splitted: Vec<Vec<Token>> = Vec::new();
        let mut building_line: Vec<Token> = Vec::new();

        for t in tokens.iter() {
            if *t == Token::NewLine || *t == Token::EOF {
                if building_line.len() == 0 {
                    continue;
                }
                splitted.push(building_line.clone());
                building_line = Vec::new();
            } else {
                building_line.push(t.clone());
            }
        }
        splitted
    }
}

fn make_optionals(target: &str) -> String {
    Regex::new(r"\((.+)\)")
        .unwrap()
        .replace(target, "[${1}]?")
        .into_owned()
}

fn make_set(set: &Vec<String>) -> String {
    let set_with_optionals = {
        let mut phonemes: Vec<String> = Vec::new();
        for i in set {
            phonemes.push(make_optionals(i));
        }
        phonemes
    };
    String::from("(") + &set_with_optionals.join("|") + ")"
}

fn build_case(tokens: &Vec<PhexNode>) -> String {
    let mut building: String = String::new();
    let mut already_gone_the_placeholder = false;

    for i in tokens.iter() {
        match i {
            PhexNode::Set(c) | PhexNode::Phoneme(c) => building.push_str(c),
            PhexNode::OperPlaceholder => {
                already_gone_the_placeholder = true;
                building.push_str("_")
            }
            PhexNode::OperBoundery => {
                if already_gone_the_placeholder {
                    building.push_str("$")
                } else {
                    building.push_str("^")
                }
            }
            _ => continue,
        }
    }
    building
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum PhexNode {
    PU(PhoneUnit),
    Set(String),
    Phoneme(String),
    OperTo,
    OperSlash,
    OperPlaceholder,
    OperBoundery,
    NewLine,
    Space,
    EOF,
    Unknown(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum PhoneUnit {
    Single(String),
    Set(Vec<String>),
}
