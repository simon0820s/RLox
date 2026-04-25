use crate::token_type::TokenType;

#[derive(Clone)]
pub struct Token {
    pub type_: TokenType,
    pub lexeme: String,
    pub literal: Option<String>,
    pub line: i32,
}

impl Token {
    pub fn new(type_: TokenType, lexeme: String, literal: Option<String>, line: i32) -> Token {
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
