# FinnLang Test Suite 

The wonderful thing about testing a programming language is that I can just write code in the language and simply run it. I mean I could probably just find a Rust test suite but I'm lazy and don't want to. Plus, I can learn my own language.

## How to Run Tests

To run any test file, navigate to the `backend` directory and use:

```bash
cargo run -- test_files/[subfolder]/[filename].finn
```

For example:
```bash
cargo run -- test_files/basic/variables.finn
cargo run -- test_files/complex/calculator.finn
```

## Test Structure

### 📁 basic/
Tests fundamental language features and syntax:

- **variables.finn** - Variable declarations with type annotations
- **arithmetic.finn** - Basic arithmetic operations (+, -, *, /, %)
- **assignment.finn** - Variable assignment and reassignment
- **woof.finn** - Print (woof) functionality testing

### 📁 control_flow/
Tests conditional statements and loops:

- **if_simple.finn** - Basic if statements and boolean conditions
- **if_elif_else.finn** - Complex if/elif/else chains and nested conditions
- **while_simple.finn** - While loops with various conditions
- **for_loops.finn** - For loops including nested loops

### 📁 functions/
Tests function declarations and calls:

- **simple.finn** - Functions without parameters
- **parameters.finn** - Functions with various parameter types
- **nested.finn** - Nested function calls and complex function interactions

### 📁 arrays/
Tests array functionality:

- **basic.finn** - Array declarations with different data types
- **loops.finn** - Processing arrays with loops (simulated since indexing may not be implemented)

### 📁 expressions/
Tests complex expressions and operations:

- **boolean.finn** - Boolean logic (&&, ||, !) and comparisons
- **strings.finn** - String concatenation and operations
- **arithmetic.finn** - Complex arithmetic expressions and operator precedence

### 📁 complex/
Integration tests combining multiple features:

- **calculator.finn** - Full calculator program with functions and error handling
- **guessing_game.finn** - Number guessing game simulation
- **prime_numbers.finn** - Prime number checker and generator

### 📁 edge_cases/
Tests boundary conditions and potential error scenarios:

- **numbers.finn** - Large numbers, zero operations, negative numbers
- **strings.finn** - Empty strings, long strings, special characters
- **control_flow.finn** - Edge cases in loops and conditional statements

## Test Categories by Difficulty

### 🟢 Beginner Tests
Start with these if you're new to FinnLang:
- `basic/variables.finn`
- `basic/woof.finn`
- `basic/arithmetic.finn`
- `control_flow/if_simple.finn`

### 🟡 Intermediate Tests
Once comfortable with basics:
- `functions/simple.finn`
- `control_flow/while_simple.finn`
- `expressions/boolean.finn`
- `arrays/basic.finn`

### 🔴 Advanced Tests
For testing complex scenarios:
- `complex/calculator.finn`
- `complex/prime_numbers.finn`
- `edge_cases/control_flow.finn`

## Expected Outputs

Most test files will produce console output showing the results of various operations. Here are some examples:

### variables.finn output:
```
Integer x: 42
Negative number: -15
Name: Charlie
Greeting: Hello, world!
Flag: true
Finished: false
Price: 19.99
Temperature: -5.5
```

### calculator.finn output:
```
=== FinnLang Calculator ===
1. Addition
2. Subtraction
3. Multiplication
4. Division
5. Factorial
========================
Testing Addition:
15 + 25 = 40
100 + 50 = 150
...
```

## Running All Tests

To run multiple tests quickly, you can use a script or run them individually:

```bash
# Run all basic tests
cargo run -- test_files/basic/variables.finn
cargo run -- test_files/basic/arithmetic.finn
cargo run -- test_files/basic/assignment.finn
cargo run -- test_files/basic/woof.finn

# Run all control flow tests
cargo run -- test_files/control_flow/if_simple.finn
cargo run -- test_files/control_flow/if_elif_else.finn
cargo run -- test_files/control_flow/while_simple.finn
cargo run -- test_files/control_flow/for_loops.finn
```
