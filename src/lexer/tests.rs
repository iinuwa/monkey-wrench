use crate::lexer::Lexer;
use crate::token::Token;
#[test]
fn lexer_creates_tokens() {
    let source = String::from("let x = 5 + 5 ;");
    let tokens = Lexer::tokenize(source);
    let token_list = vec![
        Token::Let,
        Token::Identifier("x".to_string()),
        Token::EqualSign,
        Token::Integer(5),
        Token::PlusSign,
        Token::Integer(5),
        Token::Semicolon,
    ];

    assert_eq!(token_list, tokens);
}
