mod tests;
use crate::token::Token;
use std::fmt;

pub trait Node {
    fn literal_value(&self) -> String;
}

#[derive(Debug)]
pub enum Statement {
    Let(Token, Expression, Expression),
    Return(Token, Expression),
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Statement::Let(token, identifier, value) => write!(
                f,
                "{} {} = {};",
                token.token_value(),
                identifier.to_string(),
                value.to_string()
            ),
            Statement::Return(token, expression) => {
                write!(f, "{} {};", token.token_value(), expression.to_string())
            }
        }
    }
}

impl Node for Statement {
    fn literal_value(&self) -> String {
        match self {
            Statement::Let(token, _, _) => token.token_value(),
            Statement::Return(token, _) => token.token_value(),
        }
    }
}

#[derive(Debug)]
pub enum Expression {
    Identifier(String),
    r#String(String),
    Unit,
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::Identifier(string) => write!(f, "{}", string),
            Expression::String(string) => write!(f, "{}", string),
            Expression::Unit => write!(f, "()"),
        }
    }
}

impl Node for Expression {
    fn literal_value(&self) -> String {
        match self {
            Expression::Identifier(identifier) => identifier.to_string(),
            Expression::String(string) => string.to_string(),
            Expression::Unit => "".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn new() -> Self {
        Program { statements: vec![] }
    }
}

impl Node for Program {
    fn literal_value(&self) -> String {
        if !self.statements.is_empty() {
            return self.statements[0].literal_value();
        }
        Token::EOF.token_value()
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out = String::new();
        for statement in &self.statements {
            out.push_str(&statement.to_string());
            //statement.to_string()
        }
        write!(f, "{}", out)
    }
}
