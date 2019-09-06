mod ast;
mod lexer;
mod parser;
mod repl;
mod token;

use repl::Repl;

fn main() {
    println!("Hello, world!");
    println!("Monkey REPL Shell");
    Repl::start();
}
