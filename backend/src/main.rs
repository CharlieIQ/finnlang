// Import the modules defining the Abstract Syntax Tree, interpreter, lexer, and parser
mod ast;
mod interpreter;
mod lexer;
mod parser;

// Bring the Interpreter, Lexer, and Parser types into scope for easier access
use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;

fn main() {
    // Define a simple source code string with variable declarations and print statements
    let source = r#"
let count = 10;
print(count);
count = count + 1;
print(count);
"#;

    // Initialize a lexer with the source code to tokenize the input
    let lexer = Lexer::new(source);

    // Create a parser that consumes tokens from the lexer to produce an AST
    let mut parser = Parser::new(lexer);

    // Parse the source code into an Abstract Syntax Tree (AST) representing the program
    let program = parser.parse();

    // Create a new interpreter instance for executing the parsed program
    let mut interpreter = Interpreter::new();

    // Run the interpreter on the AST to execute the source code
    interpreter.run(program);
}
