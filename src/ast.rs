use codespan::*;

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    span: Span,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    LeftBracket,
    RightBracket,
    Number(i64),
    Symbol(String),
    Str(String),
}

impl Token {
    pub fn with_span(kind: TokenKind, span: Span) -> Self {
        Token { kind, span }
    }
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Symbol(Token, String),
    Str(String),
    Number(Token, i64),
    If(Token, Token, Box<Expr>, Box<Expr>, Box<Expr>, Token),
    Define(Token, Token, Token, Box<Expr>, Token),
    Call(Token, Token, Vec<Expr>, Token),
}
