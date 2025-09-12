#![allow(dead_code)]

use crate::ast::{Expr, Stmt, Type};
use crate::lexer::{Lexer, Token};

pub struct Parser {
    lexer: Lexer,
    current: Token,
}

// This is the FinnLang parser
impl Parser {
    // Create a new parser instance from a lexer
    pub fn new(mut lexer: Lexer) -> Self {
        let current = lexer.next_token();
        Parser { lexer, current }
    }

    // Advance to the next token
    fn advance(&mut self) {
        self.current = self.lexer.next_token();
    }

    // Parse the entire input and return a vector of statements
    pub fn parse(&mut self) -> Vec<Stmt> {
        // Parse until EOF
        let mut stmts = Vec::new();
        while self.current != Token::EOF {
            // If we can parse a statement, add it to the list
            if let Some(stmt) = self.parse_stmt() {
                stmts.push(stmt);
            } else {
                // Error or unexpected token, skip it
                self.advance();
            }
        }
        stmts
    }

    // Parse a single statement based on the current token
    fn parse_stmt(&mut self) -> Option<Stmt> {
        match &self.current {
            Token::Let => self.parse_let_stmt(),
            Token::Print => self.parse_print_stmt(),
            Token::While => self.parse_while_stmt(),
            Token::For => self.parse_for_stmt(),
            Token::If => self.parse_if_stmt(),
            Token::Funct => self.parse_function_def(),
            Token::Return => self.parse_return_stmt(),
            Token::Ident(_) => {
                // Look ahead to see if there's an assignment operator
                if self.is_assignment() {
                    self.parse_assign_stmt()
                } else {
                    self.parse_expr_stmt()
                }
            }
            _ => None,
        }
    }

    // Helper method to check if current identifier is part of an assignment
    fn is_assignment(&self) -> bool {
        // Look at the current identifier and peek ahead
        if let Token::Ident(_) = &self.current {
            // Create a temporary lexer to peek ahead
            let mut temp_lexer = self.lexer.clone();
            temp_lexer.next_token(); // skip the identifier
            let next_token = temp_lexer.next_token();
            matches!(next_token, Token::Assign)
        } else {
            false
        }
    }

    // Parse expression statement: expr;
    fn parse_expr_stmt(&mut self) -> Option<Stmt> {
        let expr = self.parse_expr()?;

        // Expect semicolon
        if self.current != Token::Semicolon {
            return None;
        }
        self.advance();

        Some(Stmt::ExprStmt(expr))
    }
    /**
     * Parse the let variable keyword
     * e.g. let x = 5;
     */
    fn parse_let_stmt(&mut self) -> Option<Stmt> {
        // consume 'let'
        self.advance();

        // Expect identifier
        let var_name = if let Token::Ident(name) = &self.current {
            name.clone()
        } else {
            return None;
        };
        // consume identifier
        self.advance();

        // This is for the optional colon + type syntax
        let var_type = if self.current == Token::Colon {
            // consume ':'
            self.advance();
            self.parse_type()?
        } else {
            // Default type, e.g. Int or a placeholder
            Type::Int
        };

        // Expect '='
        if self.current != Token::Assign {
            return None;
        }
        // consume '='
        self.advance();

        // Parse expression
        let expr = self.parse_expr()?;

        // Expect semicolon
        if self.current != Token::Semicolon {
            return None;
        }
        // consume ';'
        self.advance();

        Some(Stmt::Let(var_type, var_name, expr))
    }

    /**
     * This will parse a data type
     * e.g. Int, Bool, String, Double
     */
    fn parse_type(&mut self) -> Option<Type> {
        match &self.current {
            Token::Int => {
                self.advance();
                Some(Type::Int)
            }
            Token::Bool => {
                self.advance();
                Some(Type::Bool)
            }
            Token::StringType => {
                self.advance();
                Some(Type::String)
            }
            Token::DoubleType => {
                self.advance();
                Some(Type::Double)
            }
            _ => None,
        }
    }

    /**
     * This is for parsing the woof() statement
     * e.g. woof(x);
     */
    fn parse_print_stmt(&mut self) -> Option<Stmt> {
        // consume 'print'
        self.advance();

        if self.current != Token::LParen {
            return None;
        }
        // consume '('
        self.advance();

        let expr = self.parse_expr()?;

        if self.current != Token::RParen {
            return None;
        }
        // consume ')'
        self.advance();

        if self.current != Token::Semicolon {
            return None;
        }
        // consume ';'
        self.advance();

        Some(Stmt::Print(expr))
    }

    /**
     * This is for parsing the while() loop statement
     */
    fn parse_while_stmt(&mut self) -> Option<Stmt> {
        // consume 'while'
        self.advance();

        if self.current != Token::LParen {
            return None;
        }
        // consume '('
        self.advance();

        let condition = self.parse_expr()?;

        if self.current != Token::RParen {
            return None;
        }
        // consume ')'
        self.advance();

        if self.current != Token::LBrace {
            return None;
        }
        // consume '{'
        self.advance();

        let mut body = Vec::new();
        while self.current != Token::RBrace && self.current != Token::EOF {
            if let Some(stmt) = self.parse_stmt() {
                body.push(stmt);
            } else {
                // skip unexpected tokens inside loop
                self.advance();
            }
        }

        if self.current != Token::RBrace {
            return None;
        }
        // consume '}'
        self.advance();

        Some(Stmt::While(condition, body))
    }

    /**
     * This is for parsing the if() statement
     * e.g. if (x < 5) { ... } elif (x == 5) { ... } else { ... }
     */
    fn parse_if_stmt(&mut self) -> Option<Stmt> {
        // consume 'if'
        self.advance();

        if self.current != Token::LParen {
            return None;
        }
        self.advance();

        let condition = self.parse_expr()?;

        if self.current != Token::RParen {
            return None;
        }

        self.advance();

        if self.current != Token::LBrace {
            return None;
        }
        self.advance();
        // Parse the if block
        let mut if_block = Vec::new();
        while self.current != Token::RBrace && self.current != Token::EOF {
            if let Some(stmt) = self.parse_stmt() {
                if_block.push(stmt);
            } else {
                self.advance();
            }
        }
        if self.current != Token::RBrace {
            return None;
        }
        self.advance();
        // Parse zero or more elif branches
        let mut elif_branches = Vec::new();
        while self.current == Token::Elif {
            self.advance();
            if self.current != Token::LParen {
                return None;
            }
            self.advance();
            let elif_cond = self.parse_expr()?;
            if self.current != Token::RParen {
                return None;
            }
            self.advance();
            if self.current != Token::LBrace {
                return None;
            }
            self.advance();
            let mut elif_block = Vec::new();
            while self.current != Token::RBrace && self.current != Token::EOF {
                if let Some(stmt) = self.parse_stmt() {
                    elif_block.push(stmt);
                } else {
                    self.advance();
                }
            }
            if self.current != Token::RBrace {
                return None;
            }
            self.advance();
            elif_branches.push((elif_cond, elif_block));
        }

        // Optional else
        let else_block = if self.current == Token::Else {
            self.advance();
            if self.current != Token::LBrace {
                return None;
            }
            self.advance();
            let mut block = Vec::new();
            while self.current != Token::RBrace && self.current != Token::EOF {
                if let Some(stmt) = self.parse_stmt() {
                    block.push(stmt);
                } else {
                    self.advance();
                }
            }
            if self.current != Token::RBrace {
                return None;
            }
            self.advance();
            Some(block)
        } else {
            None
        };

        Some(Stmt::If(condition, if_block, elif_branches, else_block))
    }

    // Parse an expression
    fn parse_expr(&mut self) -> Option<Expr> {
        self.parse_or_expr()
    }

    // Parse the OR expression
    fn parse_or_expr(&mut self) -> Option<Expr> {
        let mut left = self.parse_and_expr()?;
        while self.current == Token::Or {
            self.advance();
            let right = self.parse_and_expr()?;
            left = Expr::Or(Box::new(left), Box::new(right));
        }
        Some(left)
    }

    // Parse the AND expression
    fn parse_and_expr(&mut self) -> Option<Expr> {
        let mut left = self.parse_equality_expr()?;
        while self.current == Token::And {
            self.advance();
            let right = self.parse_equality_expr()?;
            left = Expr::And(Box::new(left), Box::new(right));
        }
        Some(left)
    }
    // Parse the equality expression
    fn parse_equality_expr(&mut self) -> Option<Expr> {
        let mut left = self.parse_rel_expr()?;
        while self.current == Token::Eq || self.current == Token::Neq {
            let op = self.current.clone();
            self.advance();
            let right = self.parse_rel_expr()?;
            left = match op {
                Token::Eq => Expr::Eq(Box::new(left), Box::new(right)),
                Token::Neq => Expr::Neq(Box::new(left), Box::new(right)),
                _ => unreachable!(),
            };
        }
        Some(left)
    }
    // This is for parsing different comparison operators
    fn parse_rel_expr(&mut self) -> Option<Expr> {
        let mut left = self.parse_add_expr()?;

        while self.current == Token::LessThan
            || self.current == Token::GreaterThan
            || self.current == Token::LessEqual
            || self.current == Token::GreaterEqual
        {
            let op = self.current.clone();
            self.advance();

            let right = self.parse_add_expr()?;

            left = match op {
                Token::LessThan => Expr::LessThan(Box::new(left), Box::new(right)),
                Token::GreaterThan => Expr::GreaterThan(Box::new(left), Box::new(right)),
                Token::LessEqual => Expr::LessEqual(Box::new(left), Box::new(right)),
                Token::GreaterEqual => Expr::GreaterEqual(Box::new(left), Box::new(right)),
                _ => unreachable!(),
            };
        }

        Some(left)
    }
    // This is for parsing the addition sign
    fn parse_add_expr(&mut self) -> Option<Expr> {
        let mut left = self.parse_mul_expr()?;
        while self.current == Token::Plus || self.current == Token::Minus {
            let op = self.current.clone();
            self.advance();
            let right = self.parse_mul_expr()?;
            left = match op {
                Token::Plus => Expr::Add(Box::new(left), Box::new(right)),
                Token::Minus => Expr::Sub(Box::new(left), Box::new(right)),
                _ => unreachable!(),
            };
        }
        Some(left)
    }
    // This is for parsing the multiplication sign, and handles division and modulo
    fn parse_mul_expr(&mut self) -> Option<Expr> {
        let mut left = self.parse_unary_expr()?;
        while self.current == Token::Star
            || self.current == Token::Slash
            || self.current == Token::Percent
        {
            let op = self.current.clone();
            self.advance();
            let right = self.parse_unary_expr()?;
            left = match op {
                Token::Star => Expr::Mul(Box::new(left), Box::new(right)),
                Token::Slash => Expr::Div(Box::new(left), Box::new(right)),
                Token::Percent => Expr::Mod(Box::new(left), Box::new(right)),
                _ => unreachable!(),
            };
        }
        Some(left)
    }
    // This is for parsing the unary expressions
    fn parse_unary_expr(&mut self) -> Option<Expr> {
        if self.current == Token::Not {
            self.advance();
            let expr = self.parse_unary_expr()?;
            Some(Expr::Not(Box::new(expr)))
        } else if self.current == Token::Minus {
            self.advance();
            let expr = self.parse_unary_expr()?;
            Some(Expr::Neg(Box::new(expr)))
        } else {
            self.parse_term()
        }
    }
    // this is for parsing different terms
    fn parse_term(&mut self) -> Option<Expr> {
        let term = match &self.current {
            Token::Number(n) => {
                let expr = Expr::Number(*n);
                self.advance();
                Some(expr)
            }
            Token::Double(f) => {
                let expr = Expr::Double(*f);
                self.advance();
                Some(expr)
            }
            Token::BoolLiteral(b) => {
                let expr = Expr::Bool(*b);
                self.advance();
                Some(expr)
            }
            Token::StrLiteral(s) => {
                let expr = Expr::StrLiteral(s.clone());
                self.advance();
                Some(expr)
            }
            Token::Ident(name) => {
                let name = name.clone();
                self.advance();

                // Check if it's a function call (followed by '(')
                if self.current == Token::LParen {
                    self.advance(); // consume '('

                    let mut args = Vec::new();

                    // Parse arguments
                    if self.current != Token::RParen {
                        loop {
                            let arg = self.parse_expr()?;
                            args.push(arg);

                            if self.current == Token::Comma {
                                self.advance();
                            } else {
                                break;
                            }
                        }
                    }

                    // Expect ')'
                    if self.current != Token::RParen {
                        return None;
                    }
                    self.advance();

                    Some(Expr::FunctionCall(name, args))
                } else {
                    // Just a variable reference
                    Some(Expr::Var(name))
                }
            }
            Token::LBracket => {
                self.advance(); // consume '['
                let mut elements = Vec::new();
                if self.current != Token::RBracket {
                    loop {
                        let expr = self.parse_expr()?;
                        elements.push(expr);
                        if self.current == Token::Comma {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                }
                if self.current != Token::RBracket {
                    panic!("Expected closing bracket for array literal");
                }
                self.advance(); // consume ']'
                Some(Expr::ArrayLiteral(elements))
            }
            Token::LParen => {
                self.advance();
                let expr = self.parse_expr()?;
                if self.current != Token::RParen {
                    return None;
                }
                self.advance();
                Some(expr)
            }
            _ => None,
        };
        let expr = term?;
        self.parse_postfix(expr)
    }

    /**
     * This is for parsing postfix expressions
     * e.g. array indexing arr[0]
     */
    fn parse_postfix(&mut self, mut expr: Expr) -> Option<Expr> {
        loop {
            match &self.current {
                Token::LBracket => {
                    self.advance(); // consume '['
                    let index = self.parse_expr()?;
                    if self.current != Token::RBracket {
                        panic!("Expected closing bracket for index");
                    }
                    self.advance(); // consume ']'
                    expr = Expr::Index(Box::new(expr), Box::new(index));
                }
                _ => break,
            }
        }
        Some(expr)
    }

    // This is for parsing assignment operators
    fn parse_assign_stmt(&mut self) -> Option<Stmt> {
        // Assume current is Token::Ident
        let name = if let Token::Ident(n) = &self.current {
            n.clone()
        } else {
            return None;
        };
        // consume identifier
        self.advance();

        if self.current != Token::Assign {
            return None;
        }
        // consume '='
        self.advance();

        let expr = self.parse_expr()?;

        if self.current != Token::Semicolon {
            return None;
        }
        // consume ';'
        self.advance();

        Some(Stmt::Assign(name, expr))
    }

    // Parse for loop: for (init; condition; update) { body }
    fn parse_for_stmt(&mut self) -> Option<Stmt> {
        // consume 'for'
        self.advance();

        // Expect '('
        if self.current != Token::LParen {
            return None;
        }
        self.advance();

        // Parse init statement (optional)
        let init = if self.current == Token::Semicolon {
            None
        } else {
            // Parse init statement (typically let or assignment) - handle semicolon manually
            let init_stmt = match &self.current {
                Token::Let => self.parse_let_stmt_no_semicolon(),
                Token::Ident(_) => self.parse_assign_stmt_no_semicolon(),
                _ => return None,
            };
            init_stmt.map(Box::new)
        };

        // Expect semicolon after init
        if self.current != Token::Semicolon {
            return None;
        }
        self.advance();

        // Parse condition (optional)
        let condition = if self.current == Token::Semicolon {
            None
        } else {
            self.parse_expr()
        };

        // Expect semicolon after condition
        if self.current != Token::Semicolon {
            return None;
        }
        self.advance();

        // Parse update statement (optional)
        let update = if self.current == Token::RParen {
            None
        } else {
            // Parse update (typically assignment) - no semicolon in for loop
            if let Token::Ident(_) = &self.current {
                self.parse_assign_stmt_no_semicolon().map(Box::new)
            } else {
                return None;
            }
        };

        // Expect ')'
        if self.current != Token::RParen {
            return None;
        }
        self.advance();

        // Expect '{'
        if self.current != Token::LBrace {
            return None;
        }
        self.advance();

        // Parse body statements
        let mut body = Vec::new();
        while self.current != Token::RBrace && self.current != Token::EOF {
            if let Some(stmt) = self.parse_stmt() {
                body.push(stmt);
            } else {
                self.advance(); // skip unknown tokens
            }
        }

        // Expect '}'
        if self.current != Token::RBrace {
            return None;
        }
        self.advance();

        Some(Stmt::For(init, condition, update, body))
    }

    /**
     * This is for parsing function definitions
     * e.g. funct add(a: Int, b: Int): Int { ... }
     */
    fn parse_function_def(&mut self) -> Option<Stmt> {
        // consume 'funct'
        self.advance();

        // Expect function name
        let name = if let Token::Ident(n) = &self.current {
            n.clone()
        } else {
            return None;
        };
        self.advance();

        // Expect '('
        if self.current != Token::LParen {
            return None;
        }
        self.advance();

        // Parse parameters
        let mut params = Vec::new();
        while self.current != Token::RParen {
            // Parse parameter name
            let param_name = if let Token::Ident(n) = &self.current {
                n.clone()
            } else {
                return None;
            };
            self.advance();

            // Expect ':'
            if self.current != Token::Colon {
                return None;
            }
            self.advance();

            // Parse parameter type
            let param_type = self.parse_type()?;
            params.push((param_name, param_type));

            // Check for comma or end
            if self.current == Token::Comma {
                self.advance();
            } else if self.current != Token::RParen {
                return None;
            }
        }

        // consume ')'
        self.advance();

        // Parse optional return type
        let return_type = if self.current == Token::Colon {
            self.advance();
            Some(self.parse_type()?)
        } else {
            None
        };

        // Expect '{'
        if self.current != Token::LBrace {
            return None;
        }
        self.advance();

        // Parse body statements
        let mut body = Vec::new();
        while self.current != Token::RBrace && self.current != Token::EOF {
            if let Some(stmt) = self.parse_stmt() {
                body.push(stmt);
            } else {
                self.advance(); // skip unknown tokens
            }
        }

        // Expect '}'
        if self.current != Token::RBrace {
            return None;
        }
        self.advance();

        Some(Stmt::FunctionDef(name, params, return_type, body))
    }

    // Parse return statement: return expr;
    fn parse_return_stmt(&mut self) -> Option<Stmt> {
        // consume 'return'
        self.advance();

        // Parse optional expression
        let expr = if self.current == Token::Semicolon {
            None
        } else {
            Some(self.parse_expr()?)
        };

        // Expect semicolon
        if self.current != Token::Semicolon {
            return None;
        }
        self.advance();

        Some(Stmt::Return(expr))
    }

    // Helper methods for for-loop parsing that don't consume semicolons
    fn parse_let_stmt_no_semicolon(&mut self) -> Option<Stmt> {
        // consume 'let'
        self.advance();

        // Expect identifier
        let var_name = if let Token::Ident(name) = &self.current {
            name.clone()
        } else {
            return None;
        };
        // consume identifier
        self.advance();

        // This is for the optional colon + type syntax
        let var_type = if self.current == Token::Colon {
            // consume ':'
            self.advance();
            self.parse_type()?
        } else {
            // Default type, e.g. Int or a placeholder
            Type::Int
        };

        // Expect '='
        if self.current != Token::Assign {
            return None;
        }
        // consume '='
        self.advance();

        // Parse expression
        let expr = self.parse_expr()?;

        // Don't consume semicolon here
        Some(Stmt::Let(var_type, var_name, expr))
    }

    /**
     * This is for parsing assignment statements without consuming the semicolon
     * e.g. x = 10
     */
    fn parse_assign_stmt_no_semicolon(&mut self) -> Option<Stmt> {
        // Assume current is Token::Ident
        let name = if let Token::Ident(n) = &self.current {
            n.clone()
        } else {
            return None;
        };
        // consume identifier
        self.advance();

        if self.current != Token::Assign {
            return None;
        }
        // consume '='
        self.advance();

        let expr = self.parse_expr()?;

        // Don't consume semicolon here
        Some(Stmt::Assign(name, expr))
    }
}
