use std::fmt;
use std::num::NonZeroUsize;

#[derive(Debug, Clone)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Identifier,
    String_,
    Number,
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Eof,
}

#[derive(Clone, Debug)]
pub struct Token {
    type_: TokenType,
    lexeme: String,
    literal: Option<Literal>,
    line: NonZeroUsize,
}

impl Token {
    pub fn new(type_: TokenType, lexeme: &str, literal: Option<Literal>, line: NonZeroUsize) -> Self {
        Self {
           type_,
           lexeme: lexeme.to_owned(),
           literal,
           line
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {} {:?}", self.type_, self.lexeme, self.literal)
    }
}

#[derive(Debug, Clone)]
pub enum Literal {
    String_(String),
    Number(f64),
    Bool(bool),
    Nil
}
