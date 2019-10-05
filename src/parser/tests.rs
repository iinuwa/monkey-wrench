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
    //println!("{:?}", parse_result);
    assert!(parse_result.is_ok());
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

#[test]
fn test_let_errors() {
    let input = "
    let x  5;
    let = 10
    let foobar  838383;
    ";

    let lexer = Lexer::new(input.as_bytes());
    let mut parser = Parser::new(lexer);
    let parse_result = parser.parse_program();
    assert!(parse_result.is_err());
    //println!("{:?}", parse_result);
    check_parser_errors(parser);
}

#[cfg(test)]
fn test_let_statement(statement: &Statement, name: String) {
    assert_eq!("let".to_string(), statement.literal_value());
    match statement {
        Statement::Let(_, identifier, _) => assert_eq!(name, identifier.literal_value()),
        _ => unreachable!(),
    };
}

#[cfg(test)]
fn check_parser_errors(parser: Parser) {
    let errors = parser.get_errors();
    if errors.is_empty() {
        return;
    } else {
        for error in errors {
            println!("{}", error);
        }
    }
}

#[test]
fn test_return_statements() {
    let input = "
    return 5;
    return 10;
    return 838383;
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

        for statement in program.statements {
            assert_eq!("return".to_string(), statement.literal_value());
            match statement {
                Statement::Return(_, _) => assert!(true),
                _ => unreachable!(),
            };
        }
    }
}

#[test]
fn test_identifier_statements() {
    let input = "foobar;";
    let lexer = Lexer::new(input.as_bytes());
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program().unwrap();
    //println!("{}", program);
    check_parser_errors(parser);
    assert_eq!(
        1,
        program.statements.len(),
        "Expected one statement, got {}",
        program.statements.len()
    );
    if let Statement::Expression(token, expression) = &program.statements[0] {
        assert_eq!("foobar", token.token_value());
        match expression {
            Expression::Identifier(string) => assert_eq!("foobar", string),
            _ => panic!("Expected Identifier expression, received {:?}", expression),
        }
    } else {
        panic!("Unexpected statement type. Expected expression statement.");
    }
}

#[test]
fn test_integer_literal_expression() {
    let input = "5;";
    let lexer = Lexer::new(input.as_bytes());
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program().unwrap();
    check_parser_errors(parser);
    assert_eq!(1, program.statements.len());
    match program.statements.first() {
        Some(Statement::Expression(_, e)) => {
            assert_eq!("5".to_owned(), e.literal_value());
            match e {
                Expression::Integer(integer) => {
                    assert_eq!(&5, integer);
                }
                x => panic!("Expected integer expression, received {:?}", x),
            }
        }
        x => panic!("Expected integer expression, received {:?}", x),
    }
}

#[test]
fn test_parse_prefix_operators() {
    struct PrefixTests {
        input: String,
        operator: String,
        int_value: usize,
    }
    let mut prefix_tests = Vec::new();
    prefix_tests.push(PrefixTests {
        input: "!5;".to_owned(),
        operator: "!".to_owned(),
        int_value: 5,
    });
    prefix_tests.push(PrefixTests {
        input: "-15;".to_owned(),
        operator: "-".to_owned(),
        int_value: 15,
    });

    for test in prefix_tests {
        let lexer = Lexer::new(test.input.as_bytes());
        let mut parser = Parser::new(lexer);
        let program_result = parser.parse_program();
        check_parser_errors(parser);
        let program = program_result.unwrap();
        assert_eq!(
            1,
            program.statements.len(),
            "Expected 1 statement in program, received {}",
            program.statements.len()
        );
        match program.statements.first().unwrap() {
            Statement::Expression(_, e) => {
                println!("{}", e);
                match e {
                    Expression::Prefix(operator, right) => {
                        assert_eq!(&test.operator, operator);
                        match &**right {
                            Expression::Integer(integer) => assert_eq!(test.int_value, *integer),
                            x => panic!("Expected integer expression, received {:?}", x),
                        }
                    }
                    x => panic!("Expected prefix expression, received {:?}", x),
                };
            }
            x => panic!("Expected expression statement, received {:?}", x),
        }
    }
}
