use std::env;
use std::fs;

mod ast;
mod interpreter;
mod lexer;
mod parser;

use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;

fn main() {
    // Get the first CLI argument as the filename, or fallback to a default
    let filename = env::args().nth(1).unwrap_or_else(|| "example.finn".to_string());

    // Read the source code from the file
    let source = fs::read_to_string(&filename)
        .expect("Failed to read the .finn source file");

    // Tokenize the source
    let lexer = Lexer::new(&source);

    // Parse tokens into AST
    let mut parser = Parser::new(lexer);
    let program = parser.parse();

    // Interpret the AST
    let mut interpreter = Interpreter::new();
    interpreter.run(program);
}
