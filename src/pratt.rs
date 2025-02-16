use core::fmt;
use std::{iter::Peekable, slice::Iter};

use crate::tokenizer::{Token, OP};

#[derive(Clone, PartialEq, Eq)]
pub enum E {
    LITERAL(u32),
    UNARY(OP, Box<E>),
    BINARY(Box<E>, OP, Box<E>),
    PAREN(Box<E>),
}

impl fmt::Debug for E {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LITERAL(v) => write!(f, "{}", v),
            Self::UNARY(op, v) => write!(f, "({:?} {:?})", op, v),
            Self::BINARY(v1, op, v2) => write!(f, "({:?} {:?} {:?})", op, v1, v2),
            Self::PAREN(v) => write!(f, "({:?})", v),
        }
    }
}

pub fn expression(tokens: &mut Peekable<Iter<Token>>, prev_bp: u8) -> E {
    let mut lhs = nud(tokens);
    dbg!(prev_bp);

    loop {
        let next = tokens.peek();

        let eof = next.is_none();
        if eof {
            break;
        }

        let curr_bp = bp(next.unwrap());
        if curr_bp < prev_bp {
            break;
        }

        lhs = led(lhs, tokens);
    }

    lhs
}

fn nud(tokens: &mut Peekable<Iter<Token>>) -> E {
    match tokens.next().unwrap() {
        Token::LITERAL(v) => E::LITERAL(*v),
        Token::OPERATOR(op) => {
            let bp = 80; // TODO maybe make unary precedence more explicit?
            E::UNARY(op.clone(), Box::new(expression(tokens, bp)))
        }
        Token::LPAREN => {
            let e = E::PAREN(Box::new(expression(tokens, 1)));
            tokens.next();
            e
        }
        _ => panic!(),
    }
}

fn led(left: E, tokens: &mut Peekable<Iter<Token>>) -> E {
    let token = tokens.next().unwrap();
    let bp = bp(token);
    match token {
        Token::OPERATOR(operator) => E::BINARY(
            Box::new(left),
            operator.clone(),
            Box::new(expression(tokens, bp)),
        ),
        _ => todo!(),
    }
}

pub fn bp(token: &Token) -> u8 {
    match token {
        Token::OPERATOR(op) => match op {
            OP::PLUS | OP::MINUS => 10,
            OP::MULT => 20,
            OP::POW => 30,
            _ => todo!(),
        },
        Token::RPAREN => 0, // TODO should this be "lower" than initial?
        _ => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn literal(i: u32) -> Box<E> {
        Box::new(E::LITERAL(i))
    }

    fn boxx(e: E) -> Box<E> {
        Box::new(e)
    }

    #[test]
    fn single_literal() {
        let tokens = vec![Token::LITERAL(10)];
        let mut iter = tokens.iter().peekable();
        let result = expression(&mut iter, 0);
        assert_eq!(E::LITERAL(10), result);
    }

    #[test]
    fn single_literal_parens() {
        let tokens = vec![Token::LPAREN, Token::LITERAL(10), Token::RPAREN];
        let mut iter = tokens.iter().peekable();
        let result = expression(&mut iter, 0);
        assert_eq!(E::PAREN(literal(10)), result);
    }

    #[test]
    fn single_literal_parens_x2() {
        let tokens = vec![
            Token::LPAREN,
            Token::LPAREN,
            Token::LITERAL(10),
            Token::RPAREN,
            Token::RPAREN,
        ];
        let mut iter = tokens.iter().peekable();
        let result = expression(&mut iter, 0);
        assert_eq!(E::PAREN(Box::new(E::PAREN(literal(10)))), result);
    }

    #[test]
    fn unary_minus() {
        let tokens = vec![Token::OPERATOR(OP::MINUS), Token::LITERAL(10)];
        let mut iter = tokens.iter().peekable();
        let result = expression(&mut iter, 0);
        let asd = E::UNARY(OP::MINUS, Box::new(E::LITERAL(10)));
        assert_eq!(asd, result);
    }

    #[test]
    fn unary_minus_plus() {
        let tokens = vec![
            Token::OPERATOR(OP::MINUS),
            Token::LITERAL(1),
            Token::OPERATOR(OP::PLUS),
            Token::LITERAL(2),
            Token::OPERATOR(OP::MULT),
            Token::LITERAL(3),
        ];
        let mut iter = tokens.iter().peekable();
        let result = expression(&mut iter, 0);
        let unary = E::UNARY(OP::MINUS, literal(1));
        let mult = E::BINARY(literal(2), OP::MULT, literal(3));
        let asd = E::BINARY(boxx(unary), OP::PLUS, boxx(mult));
        assert_eq!(asd, result);
    }

    #[test]
    fn addition() {
        let tokens = vec![
            Token::LITERAL(1),
            Token::OPERATOR(OP::PLUS),
            Token::LITERAL(2),
        ];
        let mut iter = tokens.iter().peekable();
        let result = expression(&mut iter, 0);
        let lit1 = Box::new(E::LITERAL(1));
        let lit2 = Box::new(E::LITERAL(2));
        let bin = E::BINARY(lit1, OP::PLUS, lit2);
        assert_eq!(bin, result);
    }

    #[test]
    fn plus_mult() {
        let tok_lit = Token::LITERAL(10);
        let tokens = vec![
            tok_lit.clone(),
            Token::OPERATOR(OP::PLUS),
            tok_lit.clone(),
            Token::OPERATOR(OP::MULT),
            tok_lit.clone(),
            Token::OPERATOR(OP::PLUS),
            tok_lit,
        ];
        let mut iter = tokens.iter().peekable();
        let result = expression(&mut iter, 0);
        let lit = Box::new(E::LITERAL(10));
        let mult = Box::new(E::BINARY(lit.clone(), OP::MULT, lit.clone()));
        let bin = Box::new(E::BINARY(mult, OP::PLUS, lit.clone()));
        let bin = E::BINARY(lit, OP::PLUS, bin);
        assert_eq!(bin, result);
    }
}
