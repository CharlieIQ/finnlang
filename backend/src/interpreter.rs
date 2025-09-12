#![allow(dead_code)]
// Import the standard HashMap type for tracking variable bindings
use std::collections::HashMap;

// Import AST node definitions for expressions and statements
use crate::ast::{Expr, Stmt, Type};

// Define a function definition structure
#[derive(Debug, Clone)]
pub struct FunctionDef {
    pub params: Vec<(String, Type)>,
    pub return_type: Option<Type>,
    pub body: Vec<Stmt>,
}

// Define the possible runtime values that the interpreter can handle
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Bool(bool),
    Str(String),
    Double(f64),
    Array(Vec<Value>),
}

// Define a return control flow exception
#[derive(Debug, Clone)]
pub enum ControlFlow {
    None,
    Return(Option<Value>),
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
            Value::Array(arr) => {
                let elements: Vec<String> = arr.iter().map(|v| v.to_string()).collect();
                write!(f, "[{}]", elements.join(", "))
            }
        }
    }
}

// Define the interpreter struct, which holds the environment for variable storage
pub struct Interpreter {
    // Environment mapping variable names to their current values
    env: HashMap<String, Value>,
    // Function definitions mapping function names to their definitions
    functions: HashMap<String, FunctionDef>,
    // Output buffer for collecting all output
    output_buffer: String,
}

impl Interpreter {
    // Create a new interpreter with an empty environment
    pub fn new() -> Self {
        Interpreter {
            env: HashMap::new(),
            functions: HashMap::new(),
            output_buffer: String::new(),
        }
    }

    // Execute a program (a vector of statements) in order
    pub fn run(&mut self, program: Vec<Stmt>) -> String {
        let mut output = String::new();
        // Execute each statement in sequence
        for stmt in program {
            match self.execute_with_control(stmt) {
                (Some(result), ControlFlow::None) => {
                    output.push_str(&result);
                    output.push('\n');
                }
                (Some(result), ControlFlow::Return(_)) => {
                    output.push_str(&result);
                    output.push('\n');
                    break;
                }
                (None, ControlFlow::Return(_)) => {
                    break;
                }
                _ => {}
            }
            // Also include any output collected during expression evaluation (like function calls)
            if !self.output_buffer.is_empty() {
                output.push_str(&self.output_buffer);
                self.output_buffer.clear();
            }
        }
        output.trim_end().to_string() // remove trailing newline
    }

    // Execute a single statement, returning output and control flow
    fn execute_with_control(&mut self, stmt: Stmt) -> (Option<String>, ControlFlow) {
        match stmt {
            Stmt::Let(_var_type_opt, name, expr) => {
                let value = self.eval(expr);
                self.env.insert(name, value);
                (None, ControlFlow::None)
            }

            Stmt::Print(expr) => {
                let value = self.eval(expr);
                (Some(value.to_string()), ControlFlow::None)
            }

            Stmt::While(cond, body) => {
                let mut output = String::new();
                loop {
                    if let Value::Bool(true) = self.eval(cond.clone()) {
                        for stmt in &body {
                            let (out, control) = self.execute_with_control(stmt.clone());
                            if let Some(out) = out {
                                output.push_str(&out);
                                output.push('\n');
                            }
                            if let ControlFlow::Return(val) = control {
                                return (
                                    if output.is_empty() {
                                        None
                                    } else {
                                        Some(output)
                                    },
                                    ControlFlow::Return(val),
                                );
                            }
                        }
                    } else {
                        break;
                    }
                }
                if output.is_empty() {
                    (None, ControlFlow::None)
                } else {
                    (Some(output), ControlFlow::None)
                }
            }

            Stmt::If(cond, if_block, elifs, else_block) => {
                let mut output = String::new();
                if let Value::Bool(true) = self.eval(cond) {
                    for stmt in if_block {
                        let (out, control) = self.execute_with_control(stmt);
                        if let Some(out) = out {
                            output.push_str(&out);
                            output.push('\n');
                        }
                        if let ControlFlow::Return(val) = control {
                            return (
                                if output.is_empty() {
                                    None
                                } else {
                                    Some(output)
                                },
                                ControlFlow::Return(val),
                            );
                        }
                    }
                    return (
                        if output.is_empty() {
                            None
                        } else {
                            Some(output)
                        },
                        ControlFlow::None,
                    );
                } else {
                    // Check elif branches
                    for (elif_cond, elif_block) in elifs {
                        if let Value::Bool(true) = self.eval(elif_cond) {
                            for stmt in elif_block {
                                let (out, control) = self.execute_with_control(stmt);
                                if let Some(out) = out {
                                    output.push_str(&out);
                                    output.push('\n');
                                }
                                if let ControlFlow::Return(val) = control {
                                    return (
                                        if output.is_empty() {
                                            None
                                        } else {
                                            Some(output)
                                        },
                                        ControlFlow::Return(val),
                                    );
                                }
                            }
                            return (
                                if output.is_empty() {
                                    None
                                } else {
                                    Some(output)
                                },
                                ControlFlow::None,
                            );
                        }
                    }
                    if let Some(else_block) = else_block {
                        for stmt in else_block {
                            let (out, control) = self.execute_with_control(stmt);
                            if let Some(out) = out {
                                output.push_str(&out);
                                output.push('\n');
                            }
                            if let ControlFlow::Return(val) = control {
                                return (
                                    if output.is_empty() {
                                        None
                                    } else {
                                        Some(output)
                                    },
                                    ControlFlow::Return(val),
                                );
                            }
                        }
                    }
                    (
                        if output.is_empty() {
                            None
                        } else {
                            Some(output)
                        },
                        ControlFlow::None,
                    )
                }
            }

            Stmt::Assign(name, expr) => {
                let value = self.eval(expr);
                if self.env.contains_key(&name) {
                    self.env.insert(name, value);
                } else {
                    panic!("Cannot assign to undeclared variable: {}", name);
                }
                (None, ControlFlow::None)
            }

            Stmt::For(init, condition, update, body) => {
                let mut output = String::new();
                // Execute init statement if present
                if let Some(init_stmt) = init {
                    self.execute_with_control(*init_stmt);
                }
                // Execute loop
                loop {
                    // Check condition (default to true if not present)
                    let should_continue = if let Some(cond) = &condition {
                        matches!(self.eval(cond.clone()), Value::Bool(true))
                    } else {
                        true
                    };
                    if !should_continue {
                        break;
                    }
                    // Execute body
                    for stmt in &body {
                        let (out, control) = self.execute_with_control(stmt.clone());
                        if let Some(out) = out {
                            output.push_str(&out);
                            output.push('\n');
                        }
                        if let ControlFlow::Return(val) = control {
                            return (
                                if output.is_empty() {
                                    None
                                } else {
                                    Some(output)
                                },
                                ControlFlow::Return(val),
                            );
                        }
                    }
                    // Execute update statement if present
                    if let Some(update_stmt) = &update {
                        self.execute_with_control(*update_stmt.clone());
                    }
                }
                if output.is_empty() {
                    (None, ControlFlow::None)
                } else {
                    (Some(output), ControlFlow::None)
                }
            }

            Stmt::FunctionDef(name, params, return_type, body) => {
                let func_def = FunctionDef {
                    params,
                    return_type,
                    body,
                };
                self.functions.insert(name, func_def);
                (None, ControlFlow::None)
            }

            Stmt::Return(expr_opt) => {
                let value = expr_opt.map(|expr| self.eval(expr));
                (None, ControlFlow::Return(value))
            }

            Stmt::ExprStmt(expr) => {
                // Execute expression for side effects (like function calls)
                self.eval(expr);
                // Check if any output was collected during expression evaluation
                if !self.output_buffer.is_empty() {
                    let output = self.output_buffer.clone();
                    self.output_buffer.clear();
                    (Some(output.trim_end().to_string()), ControlFlow::None)
                } else {
                    (None, ControlFlow::None)
                }
            }
        }
    }

    // Evaluate an expression and return its runtime value and any output from side effects
    fn eval(&mut self, expr: Expr) -> Value {
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

            Expr::ArrayLiteral(elements) => {
                let mut result = Vec::new();
                for e in elements {
                    result.push(self.eval(e));
                }
                Value::Array(result)
            }

            Expr::Index(array_expr, index_expr) => {
                let array = self.eval(*array_expr);
                let index = self.eval(*index_expr);
                match (array, index) {
                    (Value::Array(arr), Value::Int(i)) => arr
                        .get(i as usize)
                        .cloned()
                        .unwrap_or(Value::Str("Index out of bounds".into())),
                    _ => panic!("Invalid indexing operation"),
                }
            }

            Expr::AssignIndex(array_expr, index_expr, value_expr) => {
                let array_val = self.eval(*array_expr);
                let index = self.eval(*index_expr);
                let new_val = self.eval(*value_expr);

                if let (Value::Array(mut arr), Value::Int(i)) = (array_val.clone(), index) {
                    if (i as usize) < arr.len() {
                        arr[i as usize] = new_val.clone();
                        // update environment if needed
                        // ...
                        Value::Array(arr)
                    } else {
                        panic!("Index out of bounds");
                    }
                } else {
                    panic!("Invalid array assignment");
                }
            }

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

            Expr::FunctionCall(name, args) => {
                if let Some(func_def) = self.functions.get(&name).cloned() {
                    let mut func_interpreter = Interpreter::new();
                    func_interpreter.functions = self.functions.clone();
                    if args.len() != func_def.params.len() {
                        panic!(
                            "Function {} expects {} arguments, got {}",
                            name,
                            func_def.params.len(),
                            args.len()
                        );
                    }
                    for (i, (param_name, _param_type)) in func_def.params.iter().enumerate() {
                        let arg_value = self.eval(args[i].clone());
                        func_interpreter.env.insert(param_name.clone(), arg_value);
                    }
                    let mut return_value: Option<Value> = None;
                    for stmt in func_def.body {
                        let (out, control) = func_interpreter.execute_with_control(stmt);
                        if let Some(output) = out {
                            self.output_buffer.push_str(&output);
                            self.output_buffer.push('\n');
                        }
                        if let ControlFlow::Return(val) = control {
                            return_value = val;
                            break;
                        }
                    }
                    // If function has a return type, return the value, else return Int(0) by default
                    return_value.unwrap_or(Value::Int(0))
                } else {
                    panic!("Undefined function: {}", name);
                }
            }
        }
    }
}
