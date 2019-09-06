#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Illegal(String),
    EOF,

    // Identifiers + literals
    Identifier(String),
    Integer(usize),

    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    Less,
    Greater,
    Equal,
    NotEqual,

    // Delimiters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Semicolon,

    // Keywords
    Let,
    Function,
    True,
    False,
    If,
    Else,
    Return,
}

impl Token {
    pub fn lookup_identifier(identifier: String) -> Token {
        match identifier.as_str() {
            "let" => Token::Let,
            "fn" => Token::Function,
            "true" => Token::True,
            "false" => Token::False,
            "if" => Token::If,
            "else" => Token::Else,
            "return" => Token::Return,
            _ => Token::Identifier(identifier),
        }
    }

    pub fn is_valid_identifier_char(identifier_char: char) -> bool {
        identifier_char.is_alphabetic() || identifier_char == '_'
    }

    pub fn token_value(&self) -> String {
        match self {
            Token::Illegal(value) => value.to_string(),
            Token::EOF => "".to_string(),

            // Identifiers + literals
            Token::Identifier(value) => value.to_string(),
            Token::Integer(value) => value.to_string(),

            // Operators
            Token::Assign => "=".to_string(),
            Token::Plus => "+".to_string(),
            Token::Minus => "-".to_string(),
            Token::Bang => "!".to_string(),
            Token::Asterisk => "*".to_string(),
            Token::Slash => "/".to_string(),

            Token::Less => "<".to_string(),
            Token::Greater => ">".to_string(),
            Token::Equal => "==".to_string(),
            Token::NotEqual => "!=".to_string(),

            // Delimiters
            Token::LeftParen => "(".to_string(),
            Token::RightParen => ")".to_string(),
            Token::LeftBrace => "{".to_string(),
            Token::RightBrace => "}".to_string(),
            Token::Comma => ",".to_string(),
            Token::Semicolon => ";".to_string(),

            // Keywords
            Token::Let => "let".to_string(),
            Token::Function => "fn".to_string(),
            Token::True => "true".to_string(),
            Token::False => "false".to_string(),
            Token::If => "if".to_string(),
            Token::Else => "else".to_string(),
            Token::Return => "return".to_string(),
        }
    }
}
