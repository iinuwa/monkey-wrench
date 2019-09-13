use crate::lexer::Lexer;
use std::io::{self, Write};

pub struct Repl;

impl Repl {
    pub fn start() {
        loop {
            print!(">> ");
            io::stdout().flush().unwrap();
            let mut command = String::new();
            io::stdin()
                .read_line(&mut command)
                .expect("failed to read line.");
            if command == "exit()\n" {
                break;
            }
            let mut lexer = Lexer::new(command.as_bytes());
            for token in lexer.tokenize() {
                println!("{:?}", token);
            }
        }
    }
}
