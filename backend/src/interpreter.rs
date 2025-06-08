use std::collections::HashMap;

use crate::ast::{Expr, Stmt};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Bool(bool),
    Str(String),
    Double(f64),
}

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

pub struct Interpreter {
    env: HashMap<String, Value>, // for now, only store int variables
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            env: HashMap::new(),
        }
    }

    pub fn run(&mut self, program: Vec<Stmt>) {
        for stmt in program {
            self.execute(stmt);
        }
    }

    fn execute(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::Let(_var_type_opt, name, expr) => {
                let value = self.eval(expr);
                self.env.insert(name, value);
            }

            Stmt::Print(expr) => {
                let value = self.eval(expr);
                println!("{}", value);
            }
            Stmt::While(cond, body) => {
                while let Value::Bool(true) = self.eval(cond.clone()) {
                    for stmt in &body {
                        self.execute(stmt.clone());
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
            }
        }
    }

    fn eval(&self, expr: Expr) -> Value {
        match expr {
            Expr::Number(n) => Value::Int(n),
            Expr::Bool(b) => Value::Bool(b),
            Expr::StrLiteral(s) => Value::Str(s),
            Expr::Double(f) => Value::Double(f),
            Expr::Var(name) => self
                .env
                .get(&name)
                .cloned()
                .unwrap_or_else(|| panic!("Undefined variable: {}", name)),
            Expr::Add(left, right) => {
                let left_val = self.eval(*left);
                let right_val = self.eval(*right);
                match (left_val, right_val) {
                    (Value::Int(l), Value::Int(r)) => Value::Int(l + r),
                    (Value::Double(l), Value::Double(r)) => Value::Double(l + r),
                    (Value::Str(l), Value::Str(r)) => Value::Str(l + &r),
                    (Value::Str(l), v) => Value::Str(l + &v.to_string()),
                    (v, Value::Str(r)) => Value::Str(v.to_string() + &r),
                    // optionally add int + double conversions here
                    _ => panic!("Unsupported addition types"),
                }
            }
            Expr::Neg(expr) => {
                let val = self.eval(*expr);
                match val {
                    Value::Int(i) => Value::Int(-i),
                    Value::Double(f) => Value::Double(-f),
                    _ => panic!("Unsupported negation type"),
                }
            }
            Expr::Eq(left, right) => {
                let l = self.eval(*left);
                let r = self.eval(*right);
                Value::Bool(l == r)
            }
            Expr::Neq(left, right) => {
                let l = self.eval(*left);
                let r = self.eval(*right);
                Value::Bool(l != r)
            }
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
            Expr::Not(expr) => {
                if let Value::Bool(b) = self.eval(*expr) {
                    Value::Bool(!b)
                } else {
                    panic!("Expected boolean in Not");
                }
            }
            Expr::Sub(left, right) => {
                let left_val = self.eval(*left);
                let right_val = self.eval(*right);
                match (left_val, right_val) {
                    (Value::Int(l), Value::Int(r)) => Value::Int(l - r),
                    (Value::Double(l), Value::Double(r)) => Value::Double(l - r),
                    _ => panic!("Unsupported subtraction types"),
                }
            }

            Expr::Mul(left, right) => {
                let left_val = self.eval(*left);
                let right_val = self.eval(*right);
                match (left_val, right_val) {
                    (Value::Int(l), Value::Int(r)) => Value::Int(l * r),
                    (Value::Double(l), Value::Double(r)) => Value::Double(l * r),
                    _ => panic!("Unsupported multiplication types"),
                }
            }

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

            Expr::LessThan(left, right) => {
                let l = self.eval(*left);
                let r = self.eval(*right);
                match (l, r) {
                    (Value::Int(li), Value::Int(ri)) => Value::Bool(li < ri),
                    (Value::Double(ld), Value::Double(rd)) => Value::Bool(ld < rd),
                    _ => panic!("Unsupported types for LessThan comparison"),
                }
            }
            Expr::GreaterThan(left, right) => {
                let l = self.eval(*left);
                let r = self.eval(*right);
                match (l, r) {
                    (Value::Int(li), Value::Int(ri)) => Value::Bool(li > ri),
                    (Value::Double(ld), Value::Double(rd)) => Value::Bool(ld > rd),
                    _ => panic!("Unsupported types for GreaterThan comparison"),
                }
            }
            Expr::LessEqual(left, right) => {
                let l = self.eval(*left);
                let r = self.eval(*right);
                match (l, r) {
                    (Value::Int(li), Value::Int(ri)) => Value::Bool(li <= ri),
                    (Value::Double(ld), Value::Double(rd)) => Value::Bool(ld <= rd),
                    _ => panic!("Unsupported types for LessEqual comparison"),
                }
            }
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
