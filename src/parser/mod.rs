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
    errors: Vec<String>,
}

impl<'a> Parser<'a> {
    #[allow(dead_code)]
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut parser = Parser {
            lexer,
            current_token: Token::EOF,
            peek_token: Token::EOF,
            errors: Vec::new(),
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
    pub fn parse_program(&mut self) -> ParseResult<Program> {
        let mut program = Program::new();
        while {
            if let Token::EOF = self.current_token {
                false
            } else {
                true
            }
        } {
            match self.parse_statement() {
                Ok(statement) => program.statements.push(statement),
                Err(err) => self.errors.push(err.0.to_string()),
            }
            self.next_token();
        }
        if self.errors.is_empty() {
            Ok(program)
        } else {
            Err(ParserError("Error(s) parsing program.".to_string()))
        }
    }

    fn parse_statement(&mut self) -> ParseResult<Statement> {
        match self.current_token {
            Token::Let => self.parse_let_statement(),
            Token::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Result<Statement, ParserError> {
        let identifier_expression: Expression;
        if let Token::Identifier(identifier) = &self.peek_token {
            identifier_expression = Expression::Identifier(identifier.to_string());
        } else {
            return Err(ParserError(String::from("Expected identifier after `let`")));
        }

        self.next_token();

        if self.check_next_token(Token::Assign).is_ok() {
            self.next_token();
        }

        if let Ok(value) = self.parse_expression(Precedence::Lowest) {
            Ok(Statement::Let(Token::Let, identifier_expression, value))
        } else {
            Err(ParserError(String::from(
                "Could not parse let statement value.",
            )))
        }
    }

    fn parse_return_statement(&mut self) -> ParseResult<Statement> {
        self.next_token();
        if let Ok(expression) = self.parse_expression(Precedence::Lowest) {
            Ok(Statement::Return(Token::Return, expression))
        } else {
            Err(ParserError(String::from(
                "Could not parse return statement value.",
            )))
        }
    }

    fn parse_expression_statement(&mut self) -> ParseResult<Statement> {
        let token = self.current_token.clone();
        if let Ok(expression) = self.parse_expression(Precedence::Lowest) {
            if self.peek_token == Token::Semicolon {
                self.next_token();
            }

            return Ok(Statement::Expression(token, expression));
        }

        Err(ParserError("Could not parse expression".to_string()))
    }

    fn parse_expression(&mut self, _precedence: Precedence) -> ParseResult<Expression> {
        println!("{}", self.current_token.token_value());
        if let Some(parse_fn) = Self::get_prefix_parse_functions(&self.current_token) {
            return Ok(parse_fn(&self));
        }

        Err(ParserError("Could not parse prefix expression".to_string()))
    }

    fn check_current_token(&self, token: Token) -> bool {
        self.current_token == token
    }

    fn check_next_token(&mut self, token: Token) -> ParseResult<()> {
        if self.peek_token == token {
            return Ok(());
        } else {
            let error_string = format!(
                "Expected \"{}\" here, got \"{}\" instead.",
                Token::Assign.token_value(),
                self.peek_token.token_value()
            );
            // I don't like this side effect...
            self.errors.push(error_string.to_string());
            Err(ParserError(error_string))
        }
    }

    pub fn get_errors(&self) -> &Vec<String> {
        &self.errors
    }

    fn parse_identifier(parser: &Parser) -> Expression {
        Expression::Identifier(parser.current_token.token_value())
    }

    fn unit(_parser: &Parser) -> Expression {
        Expression::Unit
    }

    fn parse_integer(parser: &Parser) -> Expression {
        if let Token::Integer(value) = parser.current_token {
            Expression::Integer(value)
        } else {
            unimplemented!();
        }
    }

    fn get_prefix_parse_functions(token: &Token) -> Option<&'a PrefixParseFn> {
        match token {
            Token::Identifier(_) => Some(&Parser::parse_identifier),
            Token::Integer(_) => Some(&Parser::parse_integer),
            _ => Some(&Parser::unit),
        }
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

enum Precedence {
    Lowest = 0,
    Equals = 10,
    LessGreater = 20,
    Sum = 30,
    Product = 40,
    Prefix = 50,
    Call = 60,
}

type PrefixParseFn = dyn Fn(&Parser) -> Expression;
type InfixParseFn = dyn Fn(Parser, Expression) -> Expression;

type ParseResult<T> = Result<T, ParserError>;
