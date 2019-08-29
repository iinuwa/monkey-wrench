#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal,
    EOF,

    // Identifiers + literals
    Identifier(String),
    Integer(usize),
    // Operators
    PlusSign,
    EqualSign,

    // Delimiters
    Semicolon,
    //
    // Keywords
    Let,
}
