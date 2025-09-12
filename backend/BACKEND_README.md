# FinnLang Backend Implementation Guide 🐕⚙️

This readme explains the backend code for Finnlang. Mostly to help reinforce how this was made and some of the thoughts behind everything. Shoutout CSCI2100 for coming in clutch!

## Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [Project Structure](#project-structure)
3. [Language Pipeline](#language-pipeline)
4. [Module Deep Dive](#module-deep-dive)
5. [Building and Running](#building-and-running)
6. [Language Features](#language-features)
7. [Adding New Features](#adding-new-features)

## Architecture Overview

FinnLang is implemented as a **tree-walking interpreter** written in Rust. The interpreter follows a standard compiler/interpreter pipeline:

```
Source Code → Lexer → Tokens → Parser → AST → Interpreter → Output
```

### Key Design Decisions

- **Tree-walking interpreter**: The AST is directly executed rather than compiled to bytecode
- **Rust implementation**: I wanted to learn Rust
- **Dynamic typing**: Easier to code and more flexible for a small language
- **Immediate evaluation**: Expressions are evaluated as soon as they're encountered

## Project Structure

```
backend/
├── src/
│   ├── main.rs           # CLI entry point
│   ├── lib.rs            # Library interface for web integration
│   ├── lexer.rs          # Tokenizes source code
│   ├── ast.rs            # Abstract Syntax Tree definitions
│   ├── parser.rs         # Converts tokens to AST
│   ├── interpreter.rs    # Executes the AST
│   └── bin/
│       └── server.rs     # Web server for sandbox integration
├── Cargo.toml            # Rust project configuration
├── test_files/           # Contains test files for language features
```

## Language Pipeline
Enjoy the complicated yap that is Theoretical Computer Science!

### 1. Lexical Analysis (Lexer)

The lexer (`lexer.rs`) converts raw source code into a stream of tokens.

**Input**: `let x = 5 + 3;`
**Output**: `[Let, Ident("x"), Assign, Number(5), Plus, Number(3), Semicolon]`

#### Key Features:
- **Comments**: Supports `//` single-line and `/* */` multi-line comments (including nested)
- **Keywords**: `let`, `woof`, `if`, `while`, `for`, `funct`, `return`, etc.
- **Operators**: Arithmetic (`+`, `-`, `*`, `/`), comparison (`==`, `<`, `>`), logical (`&&`, `||`)
- **Literals**: Numbers (int/float), strings, booleans, arrays
- **Symbols**: Parentheses, braces, brackets, semicolons

### 2. Syntax Analysis (Parser)

The parser (`parser.rs`) uses recursive descent parsing to build an Abstract Syntax Tree (AST).

**Input**: Token stream
**Output**: AST representing the program structure

#### Parsing Strategy:
- **Statements**: Variable declarations, assignments, control flow, function definitions
- **Expressions**: Arithmetic, function calls, variable references, literals
- **Precedence**: Handles operator precedence correctly (e.g., `*` before `+`)
- **Error handling**: Graceful recovery from syntax errors

### 3. Abstract Syntax Tree (AST)

The AST (`ast.rs`) defines the structure of FinnLang programs using Rust enums.

#### Statement Types:
```rust
pub enum Stmt {
    Let(Type, String, Expr),        // Variable declaration
    Assign(String, Expr),           // Assignment
    Print(Expr),                    // woof statement
    While(Expr, Vec<Stmt>),         // While loop
    For(Option<Box<Stmt>>, Option<Expr>, Option<Box<Stmt>>, Vec<Stmt>), // For loop
    If(Expr, Vec<Stmt>, Vec<(Expr, Vec<Stmt>)>, Option<Vec<Stmt>>),     // If/elif/else
    FunctionDef(String, Vec<(String, Type)>, Option<Type>, Vec<Stmt>),  // Function definition
    Return(Option<Expr>),           // Return statement
    ExprStmt(Expr),                 // Expression statement (e.g., function calls)
}
```

#### Expression Types:
```rust
pub enum Expr {
    Number(i64),                    // Integer literals
    Double(f64),                    // Float literals
    Bool(bool),                     // Boolean literals
    StrLiteral(String),             // String literals
    Var(String),                    // Variable references
    FunctionCall(String, Vec<Expr>), // Function calls
    ArrayLiteral(Vec<Expr>),        // Array literals
    // Arithmetic operations
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    // ... other operators
}
```

### 4. Interpretation (Interpreter)

The interpreter (`interpreter.rs`) walks the AST and executes the program.

#### Key Components:
- **Environment**: HashMap storing variable bindings
- **Function Storage**: HashMap storing function definitions
- **Value System**: Runtime representation of FinnLang values
- **Output Collection**: Manages program output for display

Living proof that hashmaps can be used for everything. (Maybe I should add a standard library hashmap implementation)
## Module Deep Dive

### Lexer (`lexer.rs`)

The lexer is a character by character scanner that builds tokens:

```rust
pub struct Lexer {
    input: Vec<char>,    // Source code as characters
    position: usize,     // Current position in input
}

impl Lexer {
    pub fn next_token(&mut self) -> Token {
        // Skip whitespace, handle comments, tokenize
    }
}
```

**Comment Handling**:
- `//` comments: Skip until newline
- `/* */` comments: Skip until `*/`, supports nesting

**Number Parsing**: Distinguishes between integers and floats based on decimal point presence.

### Parser (`parser.rs`)

Recursive descent parser (Again shoutout CSCI2100) with separate methods for different language constructs:

```rust
impl Parser {
    fn parse_stmt(&mut self) -> Option<Stmt>      // Parse statements
    fn parse_expr(&mut self) -> Option<Expr>      // Parse expressions
    fn parse_for_stmt(&mut self) -> Option<Stmt>  // Parse for loops
    fn parse_function_def(&mut self) -> Option<Stmt> // Parse functions
    // ... specialized parsers for each construct
}
```

**Key Parsing Challenges**:
- **For loops**: Special handling for semicolon placement in `for (init; condition; update)`
- **Function calls vs variables**: Lookahead to distinguish `func()` from `var`
- **Expression statements**: Supporting standalone function calls like `myFunc();`

### Interpreter (`interpreter.rs`)

The heart of the execution engine:

```rust
pub struct Interpreter {
    env: HashMap<String, Value>,           // Variable storage
    functions: HashMap<String, FunctionDef>, // Function storage
    output_buffer: String,                 // Collects output from expressions
}

pub enum Value {
    Int(i64),
    Bool(bool),
    Str(String),
    Double(f64),
    Array(Vec<Value>),
}
```

**Execution Flow**:
1. `run()`: Executes a program (vector of statements)
2. `execute()`: Executes individual statements
3. `eval()`: Evaluates expressions to values

**Function Execution**:
- Creates new interpreter instance for function scope
- Binds parameters to arguments
- Executes function body
- Collects output and handles return values

## Building and Running

### Prerequisites
- Rust (latest stable version)
- Cargo (comes with Rust)

### Build Commands
```bash
# Build the project
cargo build

# Run the CLI interpreter
cargo run --bin finnlang -- your_file.finn

# Run the web server (for sandbox integration)
cargo run --bin server

# Check for compilation errors
cargo check
```

### Example Usage
```bash
# Create a FinnLang file
echo 'woof("Hello, FinnLang!");' > hello.finn

# Run it
cargo run --bin finnlang -- hello.finn
```

## Very Cool Very Epic Language Features

### Variable System
- **Declaration**: `let x = 5;` or `let x: int = 5;`
- **Assignment**: `x = 10;`
- **Types**: int, double, bool, string, arrays

### Control Flow
- **If statements**: `if (condition) { ... } elif (condition) { ... } else { ... }`
- **While loops**: `while (condition) { ... }`
- **For loops**: `for (let i = 0; i < n; i = i + 1) { ... }`

### Functions
- **Definition**: `funct name(param: type) { ... }`
- **Calls**: `name(arguments)`
- **Parameters**: Type-annotated parameters
- **Return types**: Optional return type annotations

### Built-in Features
- **Output**: `woof(expression)` - woofs to stdout
- **Arrays**: `[1, 2, 3]` with indexing `arr[0]`
- **Comments**: Single-line `//` and multi-line `/* */`

## Adding New Features

To add a new language feature, follow these four steps:

### 1. Add Tokens (if needed)
```rust
// In lexer.rs, add to Token enum
pub enum Token {
    // ... existing tokens
    NewKeyword,  // Your new token
}

// Add keyword recognition in next_token()
"newkeyword" => Token::NewKeyword,
```

Make sure it doesn't conflict with existing keywords, and at least is creative.

### 2. Extend AST
```rust
// In ast.rs, add to appropriate enum
pub enum Stmt {
    // ... existing statements
    NewStatement(/* parameters */),
}
```

### 3. Update Parser
```rust
// In parser.rs, add parsing method
fn parse_new_statement(&mut self) -> Option<Stmt> {
    // Parsing logic
}

// Add to parse_stmt() match
Token::NewKeyword => self.parse_new_statement(),
```

### 4. Implement Execution
```rust
// In interpreter.rs, add to execute() match
Stmt::NewStatement(params) => {
    // Execution logic
}
```

### Example: Adding a `repeat` Statement

1. **Lexer**: Add `Repeat` token
2. **AST**: Add `Repeat(Expr, Vec<Stmt>)` to Stmt enum
3. **Parser**: Parse `repeat (n) { ... }` syntax
4. **Interpreter**: Execute body n times

## Error Handling

The interpreter uses Rust's `Option` and `Result` types for error handling:

- **Parse errors**: Return `None` from parsing methods
- **Runtime errors**: Use `panic!` for critical errors
- **Type errors**: Runtime checks with descriptive messages

## Testing
In lieu of a traditional testing framework, Finnlang tests are literally tests of the language itself. Test files demonstrate language features:

- `test_comments.finn`: Comment parsing
- `test_for.finn`: For loop functionality  
- `test_functions.finn`: Function definition and calls
- `test_all_features.finn`: Comprehensive feature test

---

TLDR: This backend powers both the CLI interpreter and the web sandbox, providing the foundation for the FinnLang programming language. It's very cool, and I wish my dog could understand what I made.

Yes, this is a Rust wrapper 💀