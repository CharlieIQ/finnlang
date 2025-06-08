# FinnLang — Minimal Typed Scripting Language
** I got bored and made this **
FinnLang is a simple, statically typed scripting language designed for learning interpreters and compiler basics. It supports variable declarations, expressions, control flow, and printing.

## Language Features
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
Use print to output values:

```finnlang
print("Hello, world!");
let a = "HI";
print(a);
```

### Control Flow: While Loops
Repeat code while a condition is true:

```finnlang
let x = 0;
while (x < 5) {
    print(x);
    x = x + 1;
}
```

### Assignment
Assign new values to existing variables:
```finnlang
let count = 10;
print(count);
count = count + 1;
print(count);
```

### Variable Declaration
Declare variables with `let`. (Type annotation is optional)

```finnlang
let x = 10;
let name: string = "Charlie";
let name2 = "Charlie";
let flag: bool = true;
```

