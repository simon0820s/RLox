use crate::constants::{Literal, TokenType, keywords};
use crate::error::RLoxError;
use crate::token::Token;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }
    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while self.is_at_end() == false {
            self.start = self.current;
            self.scan_token();
        }
        self.add_token(TokenType::Eof, None);
        self.tokens.clone()
    }

    fn scan_token(&mut self) {
        let c: char = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen, None),
            ')' => self.add_token(TokenType::RightParen, None),
            '{' => self.add_token(TokenType::LeftBrace, None),
            '}' => self.add_token(TokenType::RightBrace, None),
            ',' => self.add_token(TokenType::Comma, None),
            '.' => self.add_token(TokenType::Dot, None),
            '-' => self.add_token(TokenType::Minus, None),
            '+' => self.add_token(TokenType::Plus, None),
            ';' => self.add_token(TokenType::Semicolon, None),
            '*' => self.add_token(TokenType::Star, None),
            '!' => {
                let token_type = if self.match_char('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(token_type, None);
            }
            '=' => {
                let token_type = if self.match_char('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(token_type, None);
            }
            '<' => {
                let token_type = if self.match_char('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(token_type, None);
            }
            '>' => {
                let token_type = if self.match_char('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(token_type, None);
            }
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, None);
                }
            }
            '"' => self.string(),
            ' ' => {}
            '\r' => {}
            '\t' => {}
            '\n' => self.line += 1,
            _ => {
                if c.is_digit(10) {
                    self.number();
                } else if self.is_alpha(c) {
                    self.identifier();
                } else {
                    RLoxError::new(self.line, "", &format!("Unexpected character: {}", c));
                }
            }
        }
    }
    fn advance(&mut self) -> char {
        let c: char = self.source[self.current..].chars().next().unwrap();

        self.current += c.len_utf8();

        c
    }

    fn add_token(&mut self, type_: TokenType, literal: Option<Literal>) {
        let text: String = self.source[self.start..self.current].to_string();
        self.tokens
            .push(Token::new(type_, text, literal, self.line));
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len() - 1
    }
    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source[self.current..].chars().next().unwrap() != expected {
            return false;
        }
        self.current += expected.len_utf8();
        true
    }
    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source[self.current..].chars().next().unwrap()
    }
    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source[self.current + 1..].chars().next().unwrap()
    }
    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            RLoxError::new(self.line, "", "Unterminated string.");
            return;
        }
        self.advance();
        let value: String = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token(TokenType::String, Some(Literal::String(value)));
    }
    fn number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }
        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();
            while self.peek().is_digit(10) {
                self.advance();
            }
        }
        let value: f64 = self.source[self.start..self.current].parse().unwrap();
        self.add_token(TokenType::Number, Some(Literal::Number(value)));
    }
    fn identifier(&mut self) {
        while self.is_alphanumeric(self.peek()) {
            self.advance();
        }
        let text = self.source[self.start..self.current].to_string();
        let token_type = keywords().get(&text.as_str()).cloned().unwrap_or(TokenType::Identifier);
        self.add_token(token_type, None);
    }
    fn is_alpha(&self, c: char) -> bool {
        c.is_alphabetic() || c == '_'
    }
    fn is_alphanumeric(&self, c: char) -> bool {
        self.is_alpha(c) || c.is_digit(10)
    }
}
