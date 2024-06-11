// CFG procedures
// Expr -> <RightShift> | <LeftShift> | <Plus> | <Minus> | <Read> | <Print>
// Loop -> <LeftBracket> Expr <RightBracket>

use crate::lexer::Token;
use crate::lexer::TokenType;

#[derive(Debug)]
pub enum Expr {
    Expr(Option<Token>, Option<Box<Expr>>),
    Loop(Box<Expr>, Option<Box<Expr>>),
}

impl Expr {
    pub fn push(&mut self, v: Token) {
        match self {
            Expr::Expr(_, b) => {
                if b.is_some() {
                    b.as_mut().unwrap().push(v);
                } else {
                    *b = Some(Box::new(Expr::Expr(Some(v), None)));
                }
            },

            Expr::Loop(_, b) => {
                if b.is_some() {
                    b.as_mut().unwrap().push(v);
                } else {
                    *b = Some(Box::new(Expr::Expr(Some(v), None)));
                }
            }
        }

    }

    pub fn extend(&mut self, e: Expr) {
        match self {
            Expr::Expr(_, b) => {
                if b.is_some() {
                    b.as_mut().unwrap().extend(e);
                } else {
                    *b = Some(Box::new(e));
                }
            },

            Expr::Loop(_, b) => {
                if b.is_some() {
                    b.as_mut().unwrap().extend(e);
                } else {
                    *b = Some(Box::new(e));
                }
            }
        }

    }

    pub fn next(&self) -> Option<&Box<Expr>> {
        match self {
            Expr::Expr(_, Some(b)) => {
                return Some(b);
            },

            Expr::Loop(_, Some(b)) => {
                return Some(b);
            },

            _ => ()
        }

        None
    }
}

pub fn parse(tokens: Vec<Token>) -> (Expr, usize) {
    let mut rs = Expr::Expr(None, None);
    let mut i = 0;

    while i < tokens.len() {
        match &tokens[i].type_ {
            TokenType::LeftBracket => {
                rs.extend({
                    let ne = parse(tokens[i+1..].to_vec());
                    i += ne.1 + 1;

                    ne.0
                });
            },

            TokenType::RightBracket => {
                return (Expr::Loop(Box::new(rs), None), i);
            },

            _ => {
                rs.push(tokens[i].clone());
            }
        }

        i += 1;
    }

    (rs, i)
}
