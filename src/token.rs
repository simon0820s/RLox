use crate::constants::{Literal, TokenType};

#[derive(Debug, Clone)]
pub struct Token {
    pub type_: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: usize,
}

impl Token {
    pub fn new(type_: TokenType, lexeme: String, literal: Option<Literal>, line: usize) -> Token {
        Token {
            type_,
            lexeme,
            literal,
            line,
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {} {:?}", self.type_, self.lexeme, self.literal)
    }
}
