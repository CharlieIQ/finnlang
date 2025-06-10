# FinnLang 🐕
*Yes, I named this after my dog*<br>

FinnLang is a simple, scripting language designed for learning interpreters and compiler basics. It supports basic variable declarations, expressions, control flow, and woofing.

## Running the web sandbox locally 🐶
>I made a web sandbox where you can play with FinnLang.<br>
Here are the installation steps, enjoy!

*Note: Make sure you have the latest versions of **Cargo**, and **npm***

1. Clone the Repository
```bash
git clone https://github.com/yourusername/finnlang.git
cd finnlang
```
1. Start the Backend Server
Navigate to the backend directory and run the Rust server:
```bash
cd backend
cargo run --bin server
```
This'll start the FinnLang backend on localhost

1. Set Up and Run the Frontend
In a new terminal window, go to the frontend sandbox directory:

```bash
cd ../finnland-sandbox
```
Install the dependencies:

```bash
npm install
```
Then, start the development server:
```bash
npm run dev
```
This should open the web sandbox in your browser at http://localhost:5173 (or whatever Vite uses by default).

## How to use in a CLI 🐶
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
### If, Elif, Else Statements
Use `if`, `elif`, and `else` to declate conditionals. `elif` and `else` are entirely optional.

```finnlang
if (x < 0){
    woof("x is less than 0");
}elif (x == 5){
    woof("x is equal to 5");
}else{
    woof("x is something else");
}
```

### While Loops
Repeat code while a condition is true:

```finnlang
let x = 0;
while (x < 5) {
    woof(x);
    x = x + 1;
}
```

### Arrays
You can declare arrays of data
```finnlang
let nums = [0, 1, 2, 3, 4, 5];
let names = ["Alice", "Bob", "Charlie"];
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
- Functions
- Lambda functions
- Classes/OOP support (Maybe)