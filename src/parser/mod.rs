mod tests;

use crate::ast::{Program, Statement};
use crate::lexer::Lexer;
use crate::token::Token;
use std::error::Error;
use std::fmt;
#[allow(dead_code)]
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
    peek_token: Token,
}

impl<'a> Parser<'a> {
    #[allow(dead_code)]
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

    #[allow(dead_code)]
    pub fn parse_program(&mut self) -> Result<Program, ParserError> {
        let mut program = Program::new();
        while {
            if let Token::EOF = self.current_token {
                true
            } else {
                false
            }
        } {
            if let Ok(statement) = self.parse_statement() {
                program.statements.push(statement);
                self.next_token();
            }
            else {
                return Err(ParserError);
            }
        }
        Ok(program)
    }

    pub fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        unimplemented!();
    }
}

#[derive(Debug)]
pub struct ParserError;

impl Error for ParserError {}

impl fmt::Display for ParserError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Error parsing source")
    }
}
