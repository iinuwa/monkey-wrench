#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal(String),
    EOF(String),

    // Identifiers + literals
    Identifier(String),
    Integer(String),

    // Operators
    Assign(String),
    Plus(String),
    Minus(String),
    Bang(String),
    Asterisk(String),
    Slash(String),

    Less(String),
    Greater(String),
    Equal(String),
    NotEqual(String),

    // Delimiters
    LeftParen(String),
    RightParen(String),
    LeftBrace(String),
    RightBrace(String),
    Comma(String),
    Semicolon(String),

    // Keywords
    Let(String),
    Function(String),
    True(String),
    False(String),
    If(String),
    Else(String),
    Return(String),
}

impl Token {
    pub fn lookup_identifier(identifier: String) -> Token {
        match identifier.as_str() {
            "let" => Token::Let(identifier),
            "fn" => Token::Function(identifier),
            "true" => Token::True(identifier),
            "false" => Token::False(identifier),
            "if" => Token::If(identifier),
            "else" => Token::Else(identifier),
            "return" => Token::Return(identifier),
            _ => Token::Identifier(identifier),
        }
    }

    pub fn is_valid_identifier_char(identifier_char: char) -> bool {
        identifier_char.is_alphabetic() || identifier_char == '_'
    }
}
