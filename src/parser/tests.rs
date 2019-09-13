#[cfg(test)]
use crate::ast::{Expression, Node, Statement};
#[cfg(test)]
use crate::lexer::Lexer;
#[cfg(test)]
use crate::parser::Parser;

#[test]
fn test_let_statements() {
    let input = "
    let x = 5;
    let y = 10;
    let foobar = 838383;
    ";

    let lexer = Lexer::new(input.as_bytes());
    let mut parser = Parser::new(lexer);
    let parse_result = parser.parse_program();
    assert!(parse_result.is_ok());
    //println!("{:?}", parse_result);
    if let Ok(program) = parse_result {
        assert!(
            program.statements.len() == 3,
            "Expected 3 statements, got {}",
            program.statements.len()
        );

        for (i, identifier_expected) in ["x", "y", "foobar"].iter().enumerate() {
            test_let_statement(&program.statements[i], identifier_expected.to_string());
        }
    }
}

#[cfg(test)]
fn test_let_statement(statement: &Statement, name: String) {
    assert_eq!("let".to_string(), statement.literal_value());
    match statement {
        Statement::Let(_, identifier, _) => match identifier {
            Expression::Identifier(token) => assert_eq!(name, token.token_value()),
            _ => panic!("Invalid token"),
        },
    };
    //assert_eq!(name, statement.literal_value());
}
