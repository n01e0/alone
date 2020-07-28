use codespan::*;
use super::ast;

enum TokeniseState {
    Start,
    Lparen,
    Rparen,
    Number,
    Symbol,
    Whitespace,
    Comment,
}

fn tokenise(source: &str) -> Vec<ast::Token> {
    use TokeniseState::*;

    let mut ret = Vec::new();
    let mut start = 0;

    loop {
        let mut state = Start;
        let mut end = start;

        for c in source[start..].chars() {
            let next = match state {
                Start => match c {
                    '(' => Some(Lparen),
                    ')' => Some(Rparen),
                    '0'..='9' => Some(Number),
                    ';' => Some(Comment),
                    'a'..='z'|'A'..='Z'
                    |'!'|'%'|'&'|'*'|'+'|'-'|'.'|'/'|':'|'<'|'>'|'='|'?'|'@'|'$'|'^'
                        => Some(Symbol),
                    c if c.is_whitespace() => Some(Whitespace),
                    _ => None,
                }
                Lparen | Rparen => None,
                Number => match c {
                    '0'..='9' => Some(Number),
                    _ => None
                },
                Symbol => {
                    match c {
                        'a'..='z'|'A'..='Z'
                        |'!'|'%'|'&'|'*'|'+'|'-'|'.'|'/'|':'|'<'|'>'|'='|'?'|'@'|'$'|'^'
                        |'0'..='9' => Some(Symbol),
                        _ => None,
                    }
                },
                Whitespace => {
                    if c.is_whitespace() {
                        Some(Whitespace)
                    } else {
                        None
                    }
                },
                Comment => {
                    if c == '\r' || c == '\n' {
                        None
                    } else {
                        Some(Comment)
                    }
                }
            };

            if let Some(next_state) = next {
                state = next_state;
                end += c.len_utf8();
            } else {
                break;
            }
        }

        let token_str = &source[start..end];
        let span = Span::new(ByteIndex::from(start as u32), ByteIndex::from(end as u32));

        start = end;

        let kind = match state {
            Start => break,
            Lparen => ast::TokenKind::LeftBracket,
            Rparen => ast::TokenKind::RightBracket,
            Number => ast::TokenKind::Number(token_str.parse().unwrap()),
            Symbol => ast::TokenKind::Symbol(token_str.to_string()),
            Whitespace | Comment => continue,
        };

        let span = Span::new(ByteIndex::from(span.start().to_usize() as u32 + 1), ByteIndex::from(span.end().to_usize() as u32 + 1));
        ret.push(ast::Token::with_span(kind, span));
    }

    ret
}

struct ParseState<I: Iterator<Item = ast::Token>>(std::iter::Peekable<I>);
use ast::TokenKind::*;

impl<I> ParseState<I> 
where I: Iterator<Item = ast::Token>
{
    fn parse_expr(&mut self) -> ast::Expr {
        if let Some(token) = self.0.next() {
            match token.kind {
                LeftBracket => self.parse_form(token),
                RightBracket => panic!("unexpected token!"),
                Number(n) => ast::Expr::Number(token, n),
                Symbol(ref s) => {
                    let sym = s.clone();
                    ast::Expr::Symbol(token, sym)
                }
            }
        } else {
            panic!("invalid expression")
        }
    }

    fn parse_form(&mut self, open: ast::Token) -> ast::Expr {
        match self.0.peek() {
            Some(&ast::Token {
                kind: Symbol(ref sym),
                ..
            }) => match &sym[..] {
                "if" => {
                    let if_tok = self.0.next().unwrap();
                    let cond = self.parse_expr();
                    let true_then = self.parse_expr();
                    let false_then = self.parse_expr();
                    let close = self.0.next().unwrap();
                    ast::Expr::If(
                        open,
                        if_tok,
                        Box::new(cond),
                        Box::new(true_then),
                        Box::new(false_then),
                        close,
                    )
                },
                "define" => {
                    let define_tok = self.0.next().unwrap();
                    let sym_tok = self.0.next().unwrap();
                    let value = self.parse_expr();
                    let close = self.0.next().unwrap();
                    ast::Expr::Define(
                        open,
                        define_tok,
                        sym_tok,
                        Box::new(value),
                        close
                    )
                },
                _ => {
                    let sym_tok = self.0.next().unwrap();
                    let mut args = Vec::new();
                    while let Some(token) = self.0.peek() {
                        if token.kind == RightBracket {
                            break;
                        }
                        args.push(self.parse_expr());
                    }
                    let close = self.0.next().unwrap();
                    ast::Expr::Call(
                        open,
                        sym_tok,
                        args,
                        close
                    )
                }
            },
            _ => panic!("invalid expression"),
        }
    }
}

pub fn parse(source: &str) -> ast::Expr {
    let tokens = tokenise(source);
    ParseState(tokens.into_iter().peekable()).parse_expr()
}

