use std::collections::HashMap;
use std::sync::OnceLock;

pub fn keywords() -> &'static HashMap<&'static str, TokenType> {
    static KEYWORDS: OnceLock<HashMap<&'static str, TokenType>> = OnceLock::new();

    KEYWORDS.get_or_init(|| {
        let mut m: HashMap<&'static str, TokenType> = HashMap::new();

        m.insert("and", TokenType::And);
        m.insert("class", TokenType::Class);
        m.insert("else", TokenType::Else);
        m.insert("false", TokenType::False);
        m.insert("for", TokenType::For);
        m.insert("fun", TokenType::Fun);
        m.insert("if", TokenType::If);
        m.insert("nil", TokenType::Nil);
        m.insert("or", TokenType::Or);
        m.insert("print", TokenType::Print);
        m.insert("return", TokenType::Return);
        m.insert("super", TokenType::Super);
        m.insert("this", TokenType::This);
        m.insert("true", TokenType::True);
        m.insert("var", TokenType::Var);
        m.insert("while", TokenType::While);

        m
    })
}

#[derive(Clone, Debug)]
pub enum Literal {
    String(String),
    Number(f64),
    Boolean(bool),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
  // Single-character tokens.
  LeftParen, RightParen, LeftBrace, RightBrace,
  Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

  // One or two character tokens.
  Bang, BangEqual,
  Equal, EqualEqual,
  Greater, GreaterEqual,
  Less, LessEqual,

  // Literals.
  Identifier, String, Number,

  // Keywords.
  And, Class, Else, False, Fun, For, If, Nil, Or,
  Print, Return, Super, This, True, Var, While,

  Eof
}
