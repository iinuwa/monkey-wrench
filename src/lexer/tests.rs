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
        Token::Let("let".to_string()),
        Token::Identifier("five".to_string()),
        Token::Assign("=".to_string()),
        Token::Integer(5.to_string()),
        Token::Semicolon(";".to_string()),
        Token::Let("let".to_string()),
        Token::Identifier("ten".to_string()),
        Token::Assign("=".to_string()),
        Token::Integer(10.to_string()),
        Token::Semicolon(";".to_string()),
        Token::Let("let".to_string()),
        Token::Identifier("add".to_string()),
        Token::Assign("=".to_string()),
        Token::Function("fn".to_string()),
        Token::LeftParen("(".to_string()),
        Token::Identifier("x".to_string()),
        Token::Comma(",".to_string()),
        Token::Identifier("y".to_string()),
        Token::RightParen(")".to_string()),
        Token::LeftBrace("{".to_string()),
        Token::Identifier("x".to_string()),
        Token::Plus("+".to_string()),
        Token::Identifier("y".to_string()),
        Token::Semicolon(";".to_string()),
        Token::RightBrace("}".to_string()),
        Token::Semicolon(";".to_string()),
        Token::Let("let".to_string()),
        Token::Identifier("result".to_string()),
        Token::Assign("=".to_string()),
        Token::Identifier("add".to_string()),
        Token::LeftParen("(".to_string()),
        Token::Identifier("five".to_string()),
        Token::Comma(",".to_string()),
        Token::Identifier("ten".to_string()),
        Token::RightParen(")".to_string()),
        Token::Semicolon(";".to_string()),
        Token::Bang("!".to_string()),
        Token::Minus("-".to_string()),
        Token::Slash("/".to_string()),
        Token::Asterisk("*".to_string()),
        Token::Integer(5.to_string()),
        Token::Semicolon(";".to_string()),
        Token::Integer(5.to_string()),
        Token::Less("<".to_string()),
        Token::Integer(10.to_string()),
        Token::Greater(">".to_string()),
        Token::Integer(5.to_string()),
        Token::Semicolon(";".to_string()),
        Token::If("if".to_string()),
        Token::LeftParen("(".to_string()),
        Token::Integer(5.to_string()),
        Token::Less("<".to_string()),
        Token::Integer(10.to_string()),
        Token::RightParen(")".to_string()),
        Token::LeftBrace("{".to_string()),
        Token::Return("return".to_string()),
        Token::True("true".to_string()),
        Token::Semicolon(";".to_string()),
        Token::RightBrace("}".to_string()),
        Token::Else("else".to_string()),
        Token::LeftBrace("{".to_string()),
        Token::Return("return".to_string()),
        Token::False("false".to_string()),
        Token::Semicolon(";".to_string()),
        Token::RightBrace("}".to_string()),
        Token::Integer(10.to_string()),
        Token::Equal("==".to_string()),
        Token::Integer(10.to_string()),
        Token::Semicolon(";".to_string()),
        Token::Integer(10.to_string()),
        Token::NotEqual("!=".to_string()),
        Token::Integer(9.to_string()),
        Token::Semicolon(";".to_string()),
        Token::EOF("".to_string()),
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
