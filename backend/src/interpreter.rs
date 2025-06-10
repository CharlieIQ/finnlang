#![allow(dead_code)]
// Import the standard HashMap type for tracking variable bindings
use std::collections::HashMap;

// Import AST node definitions for expressions and statements
use crate::ast::{Expr, Stmt};

// Define the possible runtime values that the interpreter can handle
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Bool(bool),
    Str(String),
    Double(f64),
}

// Implement how each value variant should be displayed as a string
use std::fmt;

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(i) => write!(f, "{}", i),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Str(s) => write!(f, "{}", s),
            Value::Double(d) => write!(f, "{}", d),
        }
    }
}

// Define the interpreter struct, which holds the environment for variable storage
pub struct Interpreter {
    // Environment mapping variable names to their current values
    env: HashMap<String, Value>,
}

impl Interpreter {
    // Create a new interpreter with an empty environment
    pub fn new() -> Self {
        Interpreter {
            env: HashMap::new(),
        }
    }

    // Execute a program (a vector of statements) in order
    pub fn run(&mut self, program: Vec<Stmt>) -> String {
        let mut output = String::new();

        for stmt in program {
            if let Some(result) = self.execute(stmt) {
                output.push_str(&result);
                output.push('\n');
            }
        }

        output.trim_end().to_string() // remove trailing newline
    }

    // Execute a single statement
    fn execute(&mut self, stmt: Stmt) -> Option<String> {
        match stmt {
            Stmt::Let(_var_type_opt, name, expr) => {
                let value = self.eval(expr);
                self.env.insert(name, value);
                None
            }

            Stmt::Print(expr) => {
                let value = self.eval(expr);
                Some(value.to_string()) // Return instead of printing
            }

            Stmt::While(cond, body) => {
                let mut output = String::new();
                while let Value::Bool(true) = self.eval(cond.clone()) {
                    for stmt in &body {
                        if let Some(out) = self.execute(stmt.clone()) {
                            output.push_str(&out);
                            output.push('\n');
                        }
                    }
                }
                if output.is_empty() {
                    None
                } else {
                    Some(output)
                }
            }

            Stmt::If(cond, if_block, elifs, else_block) => {
                if let Value::Bool(true) = self.eval(cond) {
                    let mut output = String::new();
                    for stmt in if_block {
                        if let Some(out) = self.execute(stmt) {
                            output.push_str(&out);
                            output.push('\n');
                        }
                    }
                    if output.is_empty() {
                        None
                    } else {
                        Some(output)
                    }
                } else {
                    // Check elif branches
                    let mut matched = false;
                    let mut output = String::new();
                    for (elif_cond, elif_block) in elifs {
                        if let Value::Bool(true) = self.eval(elif_cond) {
                            for stmt in elif_block {
                                if let Some(out) = self.execute(stmt) {
                                    output.push_str(&out);
                                    output.push('\n');
                                }
                            }
                            matched = true;
                            break;
                        }
                    }
                    if !matched {
                        if let Some(else_block) = else_block {
                            for stmt in else_block {
                                if let Some(out) = self.execute(stmt) {
                                    output.push_str(&out);
                                    output.push('\n');
                                }
                            }
                        }
                    }
                    if output.is_empty() {
                        None
                    } else {
                        Some(output)
                    }
                }
            }

            Stmt::Assign(name, expr) => {
                let value = self.eval(expr);
                if self.env.contains_key(&name) {
                    self.env.insert(name, value);
                } else {
                    panic!("Cannot assign to undeclared variable: {}", name);
                }
                None
            }
        }
    }

    // Evaluate an expression and return its runtime value
    fn eval(&self, expr: Expr) -> Value {
        match expr {
            // Literal values
            Expr::Number(n) => Value::Int(n),
            Expr::Bool(b) => Value::Bool(b),
            Expr::StrLiteral(s) => Value::Str(s),
            Expr::Double(f) => Value::Double(f),

            // Lookup a variable’s value in the environment
            Expr::Var(name) => self
                .env
                .get(&name)
                .cloned()
                .unwrap_or_else(|| panic!("Undefined variable: {}", name)),

            // Arithmetic and string addition
            Expr::Add(left, right) => {
                let left_val = self.eval(*left);
                let right_val = self.eval(*right);
                match (left_val, right_val) {
                    (Value::Int(l), Value::Int(r)) => Value::Int(l + r),
                    (Value::Double(l), Value::Double(r)) => Value::Double(l + r),
                    (Value::Str(l), Value::Str(r)) => Value::Str(l + &r),
                    (Value::Str(l), v) => Value::Str(l + &v.to_string()),
                    (v, Value::Str(r)) => Value::Str(v.to_string() + &r),
                    _ => panic!("Unsupported addition types"),
                }
            }

            // Unary negation
            Expr::Neg(expr) => {
                let val = self.eval(*expr);
                match val {
                    Value::Int(i) => Value::Int(-i),
                    Value::Double(f) => Value::Double(-f),
                    _ => panic!("Unsupported negation type"),
                }
            }

            // Equality check
            Expr::Eq(left, right) => {
                let l = self.eval(*left);
                let r = self.eval(*right);
                Value::Bool(l == r)
            }

            // Inequality check
            Expr::Neq(left, right) => {
                let l = self.eval(*left);
                let r = self.eval(*right);
                Value::Bool(l != r)
            }

            // Logical AND (short-circuiting)
            Expr::And(left, right) => {
                if let Value::Bool(l) = self.eval(*left) {
                    if !l {
                        return Value::Bool(false);
                    }
                } else {
                    panic!("Expected boolean in And");
                }
                if let Value::Bool(r) = self.eval(*right) {
                    Value::Bool(r)
                } else {
                    panic!("Expected boolean in And");
                }
            }

            // Logical OR (short-circuiting)
            Expr::Or(left, right) => {
                if let Value::Bool(l) = self.eval(*left) {
                    if l {
                        return Value::Bool(true);
                    }
                } else {
                    panic!("Expected boolean in Or");
                }
                if let Value::Bool(r) = self.eval(*right) {
                    Value::Bool(r)
                } else {
                    panic!("Expected boolean in Or");
                }
            }

            // Logical NOT
            Expr::Not(expr) => {
                if let Value::Bool(b) = self.eval(*expr) {
                    Value::Bool(!b)
                } else {
                    panic!("Expected boolean in Not");
                }
            }

            // Subtraction
            Expr::Sub(left, right) => {
                let left_val = self.eval(*left);
                let right_val = self.eval(*right);
                match (left_val, right_val) {
                    (Value::Int(l), Value::Int(r)) => Value::Int(l - r),
                    (Value::Double(l), Value::Double(r)) => Value::Double(l - r),
                    _ => panic!("Unsupported subtraction types"),
                }
            }

            // Multiplication
            Expr::Mul(left, right) => {
                let left_val = self.eval(*left);
                let right_val = self.eval(*right);
                match (left_val, right_val) {
                    (Value::Int(l), Value::Int(r)) => Value::Int(l * r),
                    (Value::Double(l), Value::Double(r)) => Value::Double(l * r),
                    _ => panic!("Unsupported multiplication types"),
                }
            }

            // Division with divide-by-zero checks
            Expr::Div(left, right) => {
                let left_val = self.eval(*left);
                let right_val = self.eval(*right);
                match (left_val, right_val) {
                    (Value::Int(l), Value::Int(r)) => {
                        if r == 0 {
                            panic!("Division by zero");
                        }
                        Value::Int(l / r)
                    }
                    (Value::Double(l), Value::Double(r)) => {
                        if r == 0.0 {
                            panic!("Division by zero");
                        }
                        Value::Double(l / r)
                    }
                    _ => panic!("Unsupported division types"),
                }
            }

            // Modulo operator with zero-check
            Expr::Mod(left, right) => {
                let left_val = self.eval(*left);
                let right_val = self.eval(*right);
                match (left_val, right_val) {
                    (Value::Int(l), Value::Int(r)) => {
                        if r == 0 {
                            panic!("Modulo by zero");
                        }
                        Value::Int(l % r)
                    }
                    _ => panic!("Unsupported modulo types"),
                }
            }

            // Comparison: less than
            Expr::LessThan(left, right) => {
                let l = self.eval(*left);
                let r = self.eval(*right);
                match (l, r) {
                    (Value::Int(li), Value::Int(ri)) => Value::Bool(li < ri),
                    (Value::Double(ld), Value::Double(rd)) => Value::Bool(ld < rd),
                    _ => panic!("Unsupported types for LessThan comparison"),
                }
            }

            // Comparison: greater than
            Expr::GreaterThan(left, right) => {
                let l = self.eval(*left);
                let r = self.eval(*right);
                match (l, r) {
                    (Value::Int(li), Value::Int(ri)) => Value::Bool(li > ri),
                    (Value::Double(ld), Value::Double(rd)) => Value::Bool(ld > rd),
                    _ => panic!("Unsupported types for GreaterThan comparison"),
                }
            }

            // Comparison: less than or equal to
            Expr::LessEqual(left, right) => {
                let l = self.eval(*left);
                let r = self.eval(*right);
                match (l, r) {
                    (Value::Int(li), Value::Int(ri)) => Value::Bool(li <= ri),
                    (Value::Double(ld), Value::Double(rd)) => Value::Bool(ld <= rd),
                    _ => panic!("Unsupported types for LessEqual comparison"),
                }
            }

            // Comparison: greater than or equal to
            Expr::GreaterEqual(left, right) => {
                let l = self.eval(*left);
                let r = self.eval(*right);
                match (l, r) {
                    (Value::Int(li), Value::Int(ri)) => Value::Bool(li >= ri),
                    (Value::Double(ld), Value::Double(rd)) => Value::Bool(ld >= rd),
                    _ => panic!("Unsupported types for GreaterEqual comparison"),
                }
            }
        }
    }
}
