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
                // println!("parsed a statement successfully!");
                // println!("{:?}", statement);
                program.statements.push(statement);
                self.next_token();
            } else {
                return Err(ParserError(String::from("Error parsing statement.")));
            }
        }
        Ok(program)
    }

    fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        // println!("trying to parse a statement.");
        // println!("{:?}", self.current_token);
        match self.current_token {
            Token::Let => self.parse_let_statement(),
            _ => Err(ParserError(format!(
                "Unknown token encountered: {:?}",
                self.current_token
            ))),
        }
    }

    fn parse_let_statement(&mut self) -> Result<Statement, ParserError> {
        self.next_token();
        let identifier_expression: Expression;
        if let Token::Identifier(identifier) = &self.current_token {
            identifier_expression =
                Expression::Identifier(Token::Identifier(identifier.to_string()));
            self.next_token();
            if let Token::Assign = self.current_token {
                self.next_token();
                if let Ok(value) = self.parse_expression() {
                    self.next_token();

                    // println!("Is this a semicolon? {:?}", self.current_token);
                    if let Token::Semicolon = self.current_token {
                        Ok(Statement::Let(Token::Let, identifier_expression, value))
                    } else {
                        Err(ParserError(format!(
                            "Expected \"{}\" here.",
                            Token::Semicolon.token_value()
                        )))
                    }
                } else {
                    Err(ParserError(String::from(
                        "Could not parse let statement value.",
                    )))
                }
            } else {
                Err(ParserError(format!(
                    "Expected \"{}\" here.",
                    Token::Assign.token_value()
                )))
            }
        } else {
            Err(ParserError(String::from("Invalid identifier specified.")))
        }
    }

    fn parse_expression(&mut self) -> Result<Expression, ParserError> {
        Ok(Expression::Unit)
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
