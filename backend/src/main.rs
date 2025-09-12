use std::env;
use std::fs;

mod ast;
mod interpreter;
mod lexer;
mod parser;

use finnlang::run_finn_code;

fn main() {
    // Get filename from args, or use fallback
    let filename = env::args()
        .nth(1)
        .unwrap_or_else(|| "example.finn".to_string());

    // Read code from file
    let source = fs::read_to_string(&filename).expect("Failed to read the .finn source file");

    // Run the interpreter and print the result
    let result = run_finn_code(&source);
    match result {
        Ok(output) => println!("{}", output),
        Err(error) => eprintln!("{}", error),
    }
}
