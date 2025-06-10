#![allow(dead_code)]

use crate::ast::{Expr, Stmt, Type};
use crate::lexer::{Lexer, Token};

pub struct Parser {
    lexer: Lexer,
    current: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current = lexer.next_token();
        Parser { lexer, current }
    }

    fn advance(&mut self) {
        self.current = self.lexer.next_token();
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();
        while self.current != Token::EOF {
            if let Some(stmt) = self.parse_stmt() {
                stmts.push(stmt);
            } else {
                // Error or unexpected token, skip it
                self.advance();
            }
        }
        stmts
    }

    fn parse_stmt(&mut self) -> Option<Stmt> {
        match &self.current {
            Token::Let => self.parse_let_stmt(),
            Token::Print => self.parse_print_stmt(),
            Token::While => self.parse_while_stmt(),
            Token::If => self.parse_if_stmt(),
            Token::Ident(_) => self.parse_assign_stmt(),
            _ => None,
        }
    }
    /**
     * Parse the let variable keyword
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
     * This is for parsing the print() statement
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

    fn parse_if_stmt(&mut self) -> Option<Stmt> {
        println!("Trying to parse an if statement...");
        // consume 'if'
        self.advance();

        if self.current != Token::LParen {
            println!("Expected '(', found {:?}", self.current);
            return None;
        }
        self.advance();

        let condition = self.parse_expr()?;
        println!("Parsed condition: {:?}", condition);

        if self.current != Token::RParen {
            println!("Expected ')', found {:?}", self.current);
            return None;
        }

        self.advance();

        if self.current != Token::LBrace {
            println!("Expected '{{', found {:?}", self.current);
            return None;
        }
        self.advance();

        let mut if_block = Vec::new();
        while self.current != Token::RBrace && self.current != Token::EOF {
            if let Some(stmt) = self.parse_stmt() {
                if_block.push(stmt);
            } else {
                println!("Skipping invalid stmt in if block");
                self.advance();
            }
        }
        if self.current != Token::RBrace {
            return None;
        }
        self.advance();
        println!("Parsed if block with {} statements", if_block.len());
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
        match &self.current {
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
                let expr = Expr::Var(name.clone());
                self.advance();
                Some(expr)
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
        }
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
}
