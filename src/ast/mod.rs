use crate::token::Token;

pub trait Node {
    fn literal_value(&self) -> String;
}

#[derive(Debug)]
pub enum Statement {
    Let(Token, Expression, Box<dyn Expression>),
}

impl Node for Statement {
    fn literal_value(&self) -> String {
        match self {
            Statement::Let(token, _, _) => token.token_value(),
        }

    }
}

pub enum Expression {
    Identifier(Token),
}

impl Node for Expression {
    fn literal_value(&self) -> String {
        match self {
            Expression::Identifier(token) => token.token_value(),
        }
    }
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn new() -> Self {
        Program {
            statements: vec![],
        }
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

/*
pub struct LetStatement {
    token: Token,
    name: Identifier,
    value: Box<dyn Expression>,
}

impl Statement for LetStatement {
    fn statement(&self) {}
}

impl Node for LetStatement {
    fn literal_value(&self) -> String {
        self.token.token_value()
    }
}

#[derive(Debug)]
pub struct Identifier {
    pub token: Token,
}

impl Node for Identifier {
    fn literal_value(&self) -> String {
        self.token.token_value()
    }
}
impl Expression for Identifier {
    fn expression(&self) {}
}
*/
