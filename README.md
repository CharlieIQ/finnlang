# FinnLang 🐕
*Yes, I named this after my dog*<br>

FinnLang is a simple, statically typed scripting language designed for learning interpreters and compiler basics. It supports basic variable declarations, expressions, control flow, and printing.

## How to use 🐶
1. Write your finnlang code in a `.finn` file, for example:

```finnlang
let x = 0;
while (x < 5) {
    woof(x);
    x = x + 1;
}

woof("Hello world");
```

Save this as `file_name.finn` in the `./backend` folder.

2. Build and run the interpreter using Cargo:
```bash
cargo run -- file_name.finn
```
Replace `file_name.finn` with the path to your own `.finn` file if needed. (Or you can use your own file path)

3. Output will be printed to the terminal as your program executes.

## Language Features 🐾
### Types
- int — 64-bit signed integers
- bool — Boolean values: true or false
- string — Double-quoted strings "hello"
- double — Floating point numbers

### Expressions
Supports:
- Arithmetic: + for numbers and string concatenation
- Boolean logic: && (and), || (or), ! (not)
- Comparison: ==, !=, < (less than)

```finnlang
let a = 5 + 3;
let b = "Hello, " + "world!";
let c = (a == 8);
```

### Printing
Use `woof` to output values:

```finnlang
woof("Hello, world!");
let a = "HI";
woof(a);
```

### Control Flow: While Loops
Repeat code while a condition is true:

```finnlang
let x = 0;
while (x < 5) {
    woof(x);
    x = x + 1;
}
```

### Assignment
Assign new values to existing variables:
```finnlang
let count = 10;
woof(count);
count = count + 1;
woof(count);
```

### Variable Declaration
Declare variables with `let`. (Type annotation is optional)

```finnlang
let x = 10;
let name: string = "Charlie";
let name2 = "Charlie";
let flag: bool = true;
```

## Features I want to add 🦴
- A web compiler playground (that's why this is in a ./backend folder)
- Functions
- Lambda functions
- Classes/OOP support (Maybe)