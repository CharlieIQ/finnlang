#[derive(Debug, Clone, PartialEq)]
// Define the full set of token types that the language recognizes
pub enum Token {
    // Keywords and types
    Let,
    Int,
    Bool,
    StringType,
    DoubleType,
    While,
    Print,

    // Literals
    Number(i64),
    Double(f64),
    BoolLiteral(bool),
    StrLiteral(String),
    Ident(String),

    // Operators
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

    // Punctuation
    Colon,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,

    // Special tokens
    EOF,
    Unknown(char),
}

// The lexer takes source code and turns it into a stream of tokens
pub struct Lexer {
    // All characters of the input source
    input: Vec<char>,
    // Current position in the input
    position: usize,
}

impl Lexer {
    // Create a new lexer instance from a source string
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
        }
    }

    // Peek at the current character without consuming it
    fn peek(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }

    // Advance the position and return the current character
    fn advance(&mut self) -> Option<char> {
        let ch = self.peek();
        self.position += 1;
        ch
    }

    // Skip any whitespace characters (space, tab, newline)
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    // Get the next token from the source input
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let ch = self.advance();

        match ch {
            // Handle '=' or '=='
            Some('=') => {
                if self.peek() == Some('=') {
                    self.advance();
                    Token::Eq
                } else {
                    Token::Assign
                }
            }

            // Basic operators and symbols
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

            // Handle string literals
            Some('"') => {
                let mut s = String::new();
                while let Some(next) = self.peek() {
                    if next == '"' {
                        self.advance(); // Consume closing quote
                        break;
                    } else {
                        s.push(self.advance().unwrap());
                    }
                }
                Token::StrLiteral(s)
            }

            // Handle number and floating point literals
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

            // Handle identifiers and keywords
            Some(c) if c.is_ascii_alphabetic() => {
                let mut ident = c.to_string();
                while let Some(next) = self.peek() {
                    if next.is_ascii_alphanumeric() {
                        ident.push(self.advance().unwrap());
                    } else {
                        break;
                    }
                }

                // Match keywords and return appropriate token
                match ident.as_str() {
                    "let" => Token::Let,
                    "woof" => Token::Print,
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

            // Handle relational operators
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

            // End of input
            None => Token::EOF,

            // Unrecognized character
            Some(other) => Token::Unknown(other),
        }
    }
}
