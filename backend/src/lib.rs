mod ast;
mod interpreter;
mod lexer;
mod parser;

use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;

pub fn run_finn_code(source: &str) -> String {
    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer);
    let program = parser.parse();

    let mut interpreter = Interpreter::new();
    interpreter.run(program)
}
