#[cfg(test)]
use crate::ast::{Expression, Program, Statement};
#[cfg(test)]
use crate::token::Token;

#[test]
fn test_program_as_string() {
    let mut program = Program::new();
    program.statements = vec![Statement::Let(
        Token::Let,
        Expression::Identifier("my_var".to_string()),
        Expression::String("another_var".to_string()),
    )];
    println!("{}", program.to_string());
    assert_eq!("let my_var = another_var;", program.to_string());
}
