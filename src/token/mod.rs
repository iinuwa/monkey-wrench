#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Illegal(String),
    EOF,

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

    pub fn token_value(&self) -> String {
        match self {
            Token::Illegal(value) => value.to_string(),
            Token::EOF => "".to_string(),

            // Identifiers + literals
            Token::Identifier(value) => value.to_string(),
            Token::Integer(value) => value.to_string(),

            // Operators
            Token::Assign(String) => "=".to_string(),
            Token::Plus(String) => "+".to_string(),
            Token::Minus(String) => "-".to_string(),
            Token::Bang(String) => "!".to_string(),
            Token::Asterisk(String) => "*".to_string(),
            Token::Slash(String) => "/".to_string(),

            Token::Less(String) => "<".to_string(),
            Token::Greater(String) => ">".to_string(),
            Token::Equal(String) => "==".to_string(),
            Token::NotEqual(String) => "!=".to_string(),

            // Delimiters
            Token::LeftParen(String) => "(".to_string(),
            Token::RightParen(String) => ")".to_string(),
            Token::LeftBrace(String) => "{".to_string(),
            Token::RightBrace(String) => "}".to_string(),
            Token::Comma(String) => ",".to_string(),
            Token::Semicolon(String) => ";".to_string(),

            // Keywords
            //Token::Let(String) => "let".to_string(),
            Token::Function(String) => "fn".to_string(),
            Token::True(String) => "true".to_string(),
            Token::False(String) => "false".to_string(),
            Token::If(String) => "if".to_string(),
            Token::Else(String) => "else".to_string(),
            Token::Return(String) => "return".to_string(),
        }
    }
}
