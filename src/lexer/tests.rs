#[cfg(test)]
use crate::lexer::Lexer;
#[cfg(test)]
use crate::token::Token;

#[test]
fn lexer_creates_tokens() {
    let source = String::from(
        "let five = 5;
        let ten = 10;
        let add = fn(x, y) {x + y;};

        let result = add(five, ten);
        !-/*5;
        5 < 10 > 5;

        if (5 < 10) {
            return true;
        } else {
            return false;
        }

        10 == 10;
        10 != 9;",
    );
    let mut lexer = Lexer::new(source.as_bytes());
    let tokens_expected = vec![
        Token::Let,
        Token::Identifier("five".to_string()),
        Token::Assign,
        Token::Integer(5),
        Token::Semicolon,
        Token::Let,
        Token::Identifier("ten".to_string()),
        Token::Assign,
        Token::Integer(10),
        Token::Semicolon,
        Token::Let,
        Token::Identifier("add".to_string()),
        Token::Assign,
        Token::Function,
        Token::LeftParen,
        Token::Identifier("x".to_string()),
        Token::Comma,
        Token::Identifier("y".to_string()),
        Token::RightParen,
        Token::LeftBrace,
        Token::Identifier("x".to_string()),
        Token::Plus,
        Token::Identifier("y".to_string()),
        Token::Semicolon,
        Token::RightBrace,
        Token::Semicolon,
        Token::Let,
        Token::Identifier("result".to_string()),
        Token::Assign,
        Token::Identifier("add".to_string()),
        Token::LeftParen,
        Token::Identifier("five".to_string()),
        Token::Comma,
        Token::Identifier("ten".to_string()),
        Token::RightParen,
        Token::Semicolon,
        Token::Bang,
        Token::Minus,
        Token::Slash,
        Token::Asterisk,
        Token::Integer(5),
        Token::Semicolon,
        Token::Integer(5),
        Token::Less,
        Token::Integer(10),
        Token::Greater,
        Token::Integer(5),
        Token::Semicolon,
        Token::If,
        Token::LeftParen,
        Token::Integer(5),
        Token::Less,
        Token::Integer(10),
        Token::RightParen,
        Token::LeftBrace,
        Token::Return,
        Token::True,
        Token::Semicolon,
        Token::RightBrace,
        Token::Else,
        Token::LeftBrace,
        Token::Return,
        Token::False,
        Token::Semicolon,
        Token::RightBrace,
        Token::Integer(10),
        Token::Equal,
        Token::Integer(10),
        Token::Semicolon,
        Token::Integer(10),
        Token::NotEqual,
        Token::Integer(9),
        Token::Semicolon,
        Token::EOF,
    ];

    for (i, token_expected) in tokens_expected.iter().enumerate() {
        let token = lexer.next_token();
        assert_eq!(
            token_expected,
            &token,
            "Token {}: Expected token {:?}, got {:?}",
            i + 1,
            token_expected,
            token
        );
    }
}
