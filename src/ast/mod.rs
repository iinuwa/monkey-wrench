use crate::token::Token;

pub trait Node {
    fn literal_value(&self) -> String;
}

pub trait Statement: Node {
    fn statement(&self);
}

pub trait Expression {
    fn expression(&self);
}

pub struct Program<'a, T>
where
    T: Statement + Node,
{
    pub statements: Vec<&'a T>,
}

impl<T> Node for Program<'_, T>
where
    T: Statement + Node,
{
    fn literal_value(&self) -> String {
        if !self.statements.is_empty() {
            return self.statements[0].literal_value();
        }
        Token::EOF.token_value()
    }
}

pub struct LetStatement<'a, T>
where
    T: Expression + Sized,
{
    token: Token,
    name: &'a Identifier,
    value: T,
}

impl<'a, T: Expression> Statement for LetStatement<'a, T> {
    fn statement(&self) {}
}
impl<'a, T: Expression> Node for LetStatement<'a, T> {
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
