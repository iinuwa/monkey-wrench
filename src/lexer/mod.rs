mod tests;

use crate::token::Token;

#[derive(Debug)]
pub struct Lexer;

impl Lexer {
    fn tokenize(source: String) -> Vec<Token> {
        let word_list: Vec<&str> = source.split(' ').collect();
        let mut token_list = vec![];
        for word in word_list {
            println!("{}", word);
            match word {
                "let" => token_list.push(Token::Let),
                "x" => token_list.push(Token::Identifier("x".to_string())),
                "=" => token_list.push(Token::EqualSign),
                "5" => token_list.push(Token::Integer(5)),
                "+" => token_list.push(Token::PlusSign),
                ";" => token_list.push(Token::Semicolon),
                _ => token_list.push(Token::Illegal),
            }
        }
        token_list
    }
}
