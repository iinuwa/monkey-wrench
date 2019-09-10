use crate::token::Token;

pub trait Node {
    fn literal_value(&self) -> String;
}

pub trait Statement: Node {
    fn statement(&self);
}

pub trait Expression: Node {
    fn expression(&self);
}

pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Node for Program {
    fn literal_value(&self) -> String {
        if !self.statements.is_empty() {
            return self.statements[0].literal_value();
        }
        Token::EOF.token_value()
    }
}

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

pub struct Identifier {
    token: Token,
}

impl Node for Identifier {
    fn literal_value(&self) -> String {
        self.token.token_value()
    }
}
impl Expression for Identifier {
    fn expression(&self) {}
}
