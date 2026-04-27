use crate::enums::{Literal, TokenType};
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
        loop {
            self.start = self.current;
            if self.is_at_end() {
                break;
            }
            self.scan_token();
        }
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
            _ => {
                RLoxError::new(self.line, "", &format!("Unexpected character: {}", c));
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
}
