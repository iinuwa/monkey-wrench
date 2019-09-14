mod tests;

use crate::ast::{Expression, Program, Statement};
use crate::lexer::Lexer;
use crate::token::Token;
use std::error;
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
            lexer,
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
                false
            } else {
                true
            }
        } {
            if let Ok(statement) = self.parse_statement() {
                program.statements.push(statement);
                self.next_token();
            } else {
                return Err(ParserError(String::from("Error parsing statement.")));
            }
        }
        Ok(program)
    }

    fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        match self.current_token {
            Token::Let => self.parse_let_statement(),
            _ => Err(ParserError(format!(
                "Unknown token encountered: {:?}",
                self.current_token
            ))),
        }
    }

    fn parse_let_statement(&mut self) -> Result<Statement, ParserError> {
        let identifier_expression: Expression;
        if let Token::Identifier(identifier) = &self.peek_token {
            identifier_expression =
                Expression::Identifier(Token::Identifier(identifier.to_string()));
        } else {
            return Err(ParserError(String::from("Expected identifier after `let`")));
        }

        self.next_token();

        if !self.check_next_token(Token::Assign) {
            return Err(ParserError(format!(
                "Expected \"{}\" here.",
                Token::Assign.token_value()
            )));
        }

        self.next_token();

        if let Ok(value) = self.parse_expression() {
            Ok(Statement::Let(Token::Let, identifier_expression, value))
        } else {
            Err(ParserError(String::from(
                "Could not parse let statement value.",
            )))
        }
    }

    fn parse_expression(&mut self) -> Result<Expression, ParserError> {
        while !self.check_current_token(Token::Semicolon) {
            self.next_token();

            if let Token::EOF = self.current_token {
                return Err(ParserError(format!(
                    "Expected \"{}\" here.",
                    Token::Semicolon.token_value()
                )));
            }
        }
        Ok(Expression::Unit)
    }

    fn check_current_token(&self, token: Token) -> bool {
        self.current_token == token
    }

    fn check_next_token(&self, token: Token) -> bool {
        token == self.peek_token
    }
}

#[derive(Debug)]
pub struct ParserError(String);

impl error::Error for ParserError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Error parsing source")
    }
}
