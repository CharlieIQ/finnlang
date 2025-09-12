mod ast;
mod interpreter;
mod lexer;
mod parser;

use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;
use std::panic;

#[derive(Debug)]
pub enum FinnLangError {
    ParseError(String),
    RuntimeError(String),
}

impl std::fmt::Display for FinnLangError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FinnLangError::ParseError(msg) => write!(f, "Parse Error: {}", msg),
            FinnLangError::RuntimeError(msg) => write!(f, "Runtime Error: {}", msg),
        }
    }
}

impl std::error::Error for FinnLangError {}

pub fn run_finn_code(source: &str) -> Result<String, FinnLangError> {
    // Catch panics and convert them to errors
    let result = panic::catch_unwind(|| {
        let lexer = Lexer::new(source);
        let mut parser = Parser::new(lexer);
        let program = parser.parse();

        let mut interpreter = Interpreter::new();
        interpreter.run(program)
    });

    match result {
        Ok(output) => Ok(output),
        Err(panic_info) => {
            let error_msg = if let Some(s) = panic_info.downcast_ref::<&str>() {
                s.to_string()
            } else if let Some(s) = panic_info.downcast_ref::<String>() {
                s.clone()
            } else {
                "Unknown compilation/runtime error occurred".to_string()
            };

            // Determine if it's a parse error or runtime error based on content
            if error_msg.contains("Expected") || error_msg.contains("Unexpected") {
                Err(FinnLangError::ParseError(error_msg))
            } else {
                Err(FinnLangError::RuntimeError(error_msg))
            }
        }
    }
}
