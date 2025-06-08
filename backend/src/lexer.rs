// Add tokens for types and keywords
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Let,
    Int,
    Bool,
    StringType,
    DoubleType, // used for type annotations
    Double(f64),
    While,
    Print,
    Ident(String),
    Number(i64),
    BoolLiteral(bool),
    StrLiteral(String),
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Or,
    And,
    Eq,
    LessThan,
    GreaterThan,
    LessEqual,
    GreaterEqual,
    Neq,
    Not,
    Assign,
    Colon,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    EOF,
    Unknown(char),
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
        }
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.peek();
        self.position += 1;
        ch
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let ch = self.advance();

        match ch {
            Some('=') => {
                if self.peek() == Some('=') {
                    self.advance();
                    Token::Eq
                } else {
                    Token::Assign
                }
            }
            Some('+') => Token::Plus,
            Some('-') => Token::Minus,
            Some('*') => Token::Star,
            Some('/') => Token::Slash,
            Some('%') => Token::Percent,
            Some(';') => Token::Semicolon,
            Some('(') => Token::LParen,
            Some(')') => Token::RParen,
            Some('{') => Token::LBrace,
            Some('}') => Token::RBrace,
            Some(':') => Token::Colon,
            Some('"') => {
                let mut s = String::new();
                while let Some(next) = self.peek() {
                    if next == '"' {
                        self.advance(); // consume closing quote
                        break;
                    } else {
                        s.push(self.advance().unwrap());
                    }
                }
                Token::StrLiteral(s)
            }
            Some(c) if c.is_ascii_digit() => {
                let mut num = c.to_string();
                let mut is_float = false;

                while let Some(next) = self.peek() {
                    if next.is_ascii_digit() {
                        num.push(self.advance().unwrap());
                    } else if next == '.' && !is_float {
                        is_float = true;
                        num.push(self.advance().unwrap());
                    } else {
                        break;
                    }
                }

                if is_float {
                    Token::Double(num.parse::<f64>().unwrap())
                } else {
                    Token::Number(num.parse::<i64>().unwrap())
                }
            }

            Some(c) if c.is_ascii_alphabetic() => {
                let mut ident = c.to_string();
                while let Some(next) = self.peek() {
                    if next.is_ascii_alphanumeric() {
                        ident.push(self.advance().unwrap());
                    } else {
                        break;
                    }
                }
                match ident.as_str() {
                    "let" => Token::Let,
                    "print" => Token::Print,
                    "int" => Token::Int,
                    "bool" => Token::Bool,
                    "string" => Token::StringType,
                    "double" => Token::DoubleType,
                    "while" => Token::While,
                    "true" => Token::BoolLiteral(true),
                    "false" => Token::BoolLiteral(false),
                    _ => Token::Ident(ident),
                }
            }
            Some('<') => {
                if self.peek() == Some('=') {
                    self.advance();
                    Token::LessEqual
                } else {
                    Token::LessThan
                }
            }
            Some('>') => {
                if self.peek() == Some('=') {
                    self.advance();
                    Token::GreaterEqual
                } else {
                    Token::GreaterThan
                }
            }
            None => Token::EOF,
            Some(other) => Token::Unknown(other),
        }
    }
}
