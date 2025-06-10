#![allow(dead_code)]
// Represents the basic data types supported by the language
#[derive(Clone, Debug)]
pub enum Type {
    // Integer 
    Int,    
    // Boolean 
    Bool,   
    // String 
    String,
    // Floating-point
    Double, 
}

// Represents different kinds of statements in the language
#[derive(Debug, Clone)]
pub enum Stmt {
    // Variable declaration with a type, variable name, and initial expression
    Let(Type, String, Expr),

    // Assignment of a new value to an existing variable
    Assign(String, Expr),

    // Print statement to output the value of an expression
    Print(Expr),

    // While loop with a condition expression and a block of statements to execute repeatedly
    While(Expr, Vec<Stmt>),

    // If/Elif/Else statement
    If(
        Expr,
        Vec<Stmt>, 
        Vec<(Expr, Vec<Stmt>)>, 
        Option<Vec<Stmt>>,      
    ),
}

// Represents expressions that can be evaluated to produce values
#[derive(Debug, Clone)]
pub enum Expr {
    // Literal integer number
    Number(i64),

    // Literal boolean value
    Bool(bool),

    // Literal string value
    StrLiteral(String),

    // Literal floating point number
    Double(f64),

    // Variable reference by name
    Var(String),

    // Arithmetic binary operations (addition, subtraction, multiplication, division, modulo)
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Mod(Box<Expr>, Box<Expr>),
    // Arrays
    ArrayLiteral(Vec<Expr>),
    Index(Box<Expr>, Box<Expr>),
    AssignIndex(Box<Expr>, Box<Expr>, Box<Expr>), // arr[index] = value
    // Comparison operations (equality, inequality, less than, greater than, etc.)
    Eq(Box<Expr>, Box<Expr>),
    LessThan(Box<Expr>, Box<Expr>),
    GreaterThan(Box<Expr>, Box<Expr>),
    LessEqual(Box<Expr>, Box<Expr>),
    GreaterEqual(Box<Expr>, Box<Expr>),
    Neq(Box<Expr>, Box<Expr>),

    // Unary negation (arithmetic negative)
    Neg(Box<Expr>),

    // Logical binary operations (and, or)
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),

    // Logical negation (not)
    Not(Box<Expr>),
}
