#[cfg(test)]
use crate::ast::{Expression, LetStatement, Node, Program, Statement};
#[cfg(test)]
use crate::lexer::Lexer;
#[cfg(test)]
use crate::parser::{Parser, ParserError};

#[test]
fn test_let_statements() {
    let input = "
    let x = 5;
    let y = 10;
    let foobar = 838383;
    ";

    let lexer = Lexer::new(input.as_bytes());
    let parser = Parser::new(lexer);
    let parse_result: Result<Program<'_, LetStatement<dyn Expression>>, ParserError> =
        parser.parse_program();
    assert!(parse_result.is_ok());
    if let Ok(program) = parse_result {
        assert!(
            program.statements.len() == 3,
            "Expected 3 statements, got {}",
            program.statements.len()
        );

        for (i, identifier_expected) in ["x", "y", "foobar"].iter().enumerate() {
            test_let_statement(program.statements[i], identifier_expected);
        }
    }
}

#[cfg(test)]
fn test_let_statement(statement: dyn Statement, name: String) {
    assert_eq!("let".to_string(), statement.literal_value());

    let let_statement: LetStatement<dyn Expression> = statement;
    assert_eq!(name, let_statement.name.literal_value());
    assert_eq!(name, let_statement.literal_value());
}
