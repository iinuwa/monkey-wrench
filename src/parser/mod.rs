mod tests;

use crate::ast::{Program, Statement};
use crate::lexer::Lexer;
use crate::token::Token;
use std::error::Error;
use std::fmt;
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
    peek_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut parser = Parser {
            lexer: lexer,
            current_token: Token::EOF,
            peek_token: Token::EOF,
        };
        parser.next_token();
        parser.next_token();
        parser
    }

    pub fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_program<T>(&mut self) -> Result<Program<'a, T>, ParserError>
    where
        T: Statement + Sized,
    {
        unimplemented!()
    }
}

#[derive(Debug)]
struct ParserError;

impl Error for ParserError {}

impl fmt::Display for ParserError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Error parsing source")
    }
}
