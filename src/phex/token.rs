#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Token {
    Phoneme(String),
    Identifier(String),
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
