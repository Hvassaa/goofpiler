use core::panic;
use std::{iter::Peekable, slice::Iter};

use crate::tokenizer::Token;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Type {
    EXPRESSION(Expression),
    LITERAL(u32),
    OP(Op),
    LPAREN,
    RPAREN,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Expression {
    BINARY(Box<Expression>, Op, Box<Expression>),
    UNARY(Op, Box<Expression>),
    PAREN(Box<Expression>),
    LITERAL(u32),
    NIL,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Op {
    PLUS,
    MINUS,
    MULT,
    DIV,
}

impl Op {
    fn from(token: &Token) -> Option<Op> {
        match token {
            Token::PLUS => Some(Op::PLUS),
            Token::MINUS => Some(Op::MINUS),
            Token::MULT => Some(Op::MULT),
            Token::DIV => Some(Op::DIV),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Parser<'a> {
    parse_stack: Vec<Type>,
    tokens: Peekable<Iter<'a, Token>>,
}

// TODO complete me
// BNF
// VALUE ::= LITERAL
// OP ::= PLUS | MINUS | DIV | MULT
// EXPRESSION ::= EXPRESSION OP EXPRESSION
//                | LPAREN EXPRESSION RPAREN
//                | VALUE
impl<'a> Parser<'a> {
    fn new(tokens: &'a Vec<Token>) -> Parser<'a> {
        let a = tokens.iter();
        Parser {
            parse_stack: vec![],
            tokens: a.peekable(),
        }
    }

    fn shift(&mut self) -> bool {
        match self.tokens.next() {
            Some(t) => {
                match t {
                    Token::LITERAL(v) => self.parse_stack.push(Type::LITERAL(*v)),
                    Token::PLUS => self.parse_stack.push(Type::OP(Op::PLUS)),
                    Token::LPAREN => self.parse_stack.push(Type::LPAREN),
                    Token::RPAREN => self.parse_stack.push(Type::RPAREN),
                    _ => todo!(),
                }
                true
            }
            None => false,
        }
    }

    fn peek_three(&self) -> Option<(&Type, &Type, &Type)> {
        if self.parse_stack.len() < 3 {
            None
        } else {
            let last_idx = self.parse_stack.len() - 1;
            let last = self.parse_stack.get(last_idx).unwrap();
            let second_last = self.parse_stack.get(last_idx - 1).unwrap();
            let third_last = self.parse_stack.get(last_idx - 2).unwrap();
            Some((third_last, second_last, last))
        }
    }

    fn pop(&mut self, n: u32) {
        (0..n).for_each(|_| {
            self.parse_stack.pop();
        });
    }

    fn reduce(&mut self) -> bool {
        // TODO just use last but cannot borrow as mutable already borrowed as imutable bla bla
        let pop = self.parse_stack.pop();
        if match pop {
            Some(t) => match t {
                // EXPRESSION ::= VALUE
                Type::LITERAL(v) => {
                    self.parse_stack
                        .push(Type::EXPRESSION(Expression::LITERAL(v.clone())));
                    true
                }
                // TODO remove when using last (reinserts the popped val)
                v => {
                    self.parse_stack.push(v);
                    false
                }
            },
            None => false,
        } {
            return true;
        }

        if match self.peek_three() {
            Some(tokens) => match tokens {
                // EXPRESSION ::= EXPRESSION OP EXPRESSION
                (Type::EXPRESSION(v1), Type::OP(op), Type::EXPRESSION(v2)) => {
                    let e =
                        Expression::BINARY(Box::new(v1.clone()), op.clone(), Box::new(v2.clone()));
                    self.pop(3);
                    self.parse_stack.push(Type::EXPRESSION(e));
                    true
                }
                (Type::LPAREN, Type::EXPRESSION(e), Type::RPAREN) => {
                    let e = Expression::PAREN(Box::new(e.clone()));
                    self.pop(3);
                    self.parse_stack.push(Type::EXPRESSION(e));
                    true
                }
                _ => false,
            },
            _ => false,
        } {
            return true;
        }

        false
    }

    fn run(&mut self) -> &Expression {
        let mut go = true;
        while go {
            let shifted = self.shift();
            let reduced = self.reduce();
            let mut reduced_current = reduced;
            dbg!("1");
            while reduced_current {
                dbg!("2");
                reduced_current = self.reduce();
            }
            go = shifted || reduced;
        }

        if self.parse_stack.len() == 1 {
            match self.parse_stack.first().unwrap() {
                Type::EXPRESSION(a) => a,
                a => {
                    dbg!(a);
                    panic!("Malformed program")
                }
            }
        } else {
            dbg!(&self.parse_stack);
            panic!("Malformed program");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(tokens: Vec<Token>, result: Expression) {
        let mut parser = Parser::new(&tokens);
        let ast = parser.run();
        assert_eq!(result, *ast)
    }

    #[test]
    fn single_literal() {
        let tokens = vec![Token::LITERAL(10)];
        let result = Expression::LITERAL(10);
        test(tokens, result);
    }

    #[test]
    fn addition() {
        let tokens = vec![Token::LITERAL(10), Token::PLUS, Token::LITERAL(15)];
        let result = Expression::BINARY(
            Box::new(Expression::LITERAL(10)),
            Op::PLUS,
            Box::new(Expression::LITERAL(15)),
        );
        test(tokens, result);
    }

    #[test]
    fn sum() {
        let tokens = vec![
            Token::LITERAL(10),
            Token::PLUS,
            Token::LITERAL(15),
            Token::PLUS,
            Token::LITERAL(10),
        ];
        let result = Expression::BINARY(
            Box::new(Expression::BINARY(
                Box::new(Expression::LITERAL(10)),
                Op::PLUS,
                Box::new(Expression::LITERAL(15)),
            )),
            Op::PLUS,
            Box::new(Expression::LITERAL(10)),
        );
        test(tokens, result);
    }

    #[test]
    fn paren() {
        let tokens = vec![Token::LPAREN, Token::LITERAL(15), Token::RPAREN];
        let result = Expression::PAREN(Box::new(Expression::LITERAL(15)));
        test(tokens, result);
    }
}
