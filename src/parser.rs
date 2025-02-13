use core::{fmt, panic};
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

#[derive(Clone, PartialEq, Eq)]
pub enum Expression {
    BINARY(Box<Expression>, Op, Box<Expression>),
    UNARY(Op, Box<Expression>),
    PAREN(Box<Expression>),
    LITERAL(u32),
    NIL,
}

impl fmt::Debug for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::BINARY(e1, op, e2) => {
                let op = match op {
                    Op::PLUS => "+",
                    Op::MINUS => "-",
                    Op::MULT => "*",
                    Op::POW => "^",
                    Op::DIV => "/",
                };
                write!(f, "({} {:?} {:?})", op, e1, e2)
            }
            Expression::LITERAL(v) => {
                write!(f, "{}", v)
            }
            _ => write!(f, ""),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Op {
    PLUS,
    MINUS,
    MULT,
    DIV,
    POW,
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
pub struct Parser<'a> {
    parse_stack: Vec<Type>,
    tokens: Peekable<Iter<'a, Token>>,
}

// TODO complete me
// BNF
// VALUE ::= LITERAL
// OP_PREC_1 = PLUS | MINUS
// OP_PREC_2 = MULT | DIV
// EXPRESSION ::= EXPRESSION POW EXPRESSION
//                | EXPRESSION OP_PREC_2 EXPRESSION
//                | EXPRESSION OP_PREC_1 EXPRESSION
//                | LPAREN EXPRESSION RPAREN
//                | VALUE
impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Parser<'a> {
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
                    // cheat and insert literal as expression immediatly
                    Token::LITERAL(v) => self
                        .parse_stack
                        .push(Type::EXPRESSION(Expression::LITERAL(*v))),
                    Token::PLUS => self.parse_stack.push(Type::OP(Op::PLUS)),
                    Token::MULT => self.parse_stack.push(Type::OP(Op::MULT)),
                    Token::LPAREN => self.parse_stack.push(Type::LPAREN),
                    Token::RPAREN => self.parse_stack.push(Type::RPAREN),
                    Token::POW => self.parse_stack.push(Type::OP(Op::POW)),
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

    fn check_precedence(&mut self) -> bool {
        if self.parse_stack.len() < 2 {
            return false;
        }

        let peek = self.tokens.peek();
        let peek_precedence = match peek {
            Some(Token::PLUS) | Some(Token::MINUS) => Some(1),
            Some(Token::MULT) => Some(2),
            Some(Token::POW) => Some(3),
            _ => None,
        };

        let prev = self.parse_stack.get(self.parse_stack.len() - 2);

        let prev_precedence = match prev {
            Some(Type::OP(Op::PLUS)) | Some(Type::OP(Op::MINUS)) => Some(1),
            Some(Type::OP(Op::MULT)) => Some(2),
            Some(Type::OP(Op::POW)) => Some(3),
            _ => None,
        };

        match (prev_precedence, peek_precedence) {
            (None, _) | (_, None) => false,
            (Some(prev), Some(peek)) => peek > prev,
        }
    }

    fn reduce(&mut self) -> bool {
        // TODO maybe check the last operator on the top level and compare precedence with the next
        // reduce now if the incoming is lower, otherwise
        // skip and shift
        if self.check_precedence() {
            return false;
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

    pub fn run(&mut self) -> &Expression {
        let mut go = true;
        while go {
            let shifted = self.shift();
            let reduced = self.reduce();
            let mut reduced_current = reduced;
            while reduced_current {
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

    #[test]
    fn plus_then_mult() {
        let tokens = vec![
            Token::LITERAL(10),
            Token::PLUS,
            Token::LITERAL(15),
            Token::MULT,
            Token::LITERAL(10),
        ];
        let result = Expression::BINARY(
            Box::new(Expression::LITERAL(10)),
            Op::PLUS,
            Box::new(Expression::BINARY(
                Box::new(Expression::LITERAL(15)),
                Op::MULT,
                Box::new(Expression::LITERAL(10)),
            )),
        );
        test(tokens, result);
    }

    #[test]
    fn plus_then_mult_then_mult() {
        let tokens = vec![
            Token::LITERAL(10),
            Token::PLUS,
            Token::LITERAL(10),
            Token::MULT,
            Token::LITERAL(10),
            Token::MULT,
            Token::LITERAL(10),
        ];
        let result = Expression::BINARY(
            Box::new(Expression::LITERAL(10)),
            Op::PLUS,
            Box::new(Expression::BINARY(
                Box::new(Expression::BINARY(
                    Box::new(Expression::LITERAL(10)),
                    Op::MULT,
                    Box::new(Expression::LITERAL(10)),
                )),
                Op::MULT,
                Box::new(Expression::LITERAL(10)),
            )),
        );
        test(tokens, result);
    }

    #[test]
    fn plus_then_mult_then_plus() {
        let tokens = vec![
            Token::LITERAL(10),
            Token::PLUS,
            Token::LITERAL(10),
            Token::MULT,
            Token::LITERAL(10),
            Token::PLUS,
            Token::LITERAL(10),
        ];
        let result = Expression::BINARY(
            Box::new(Expression::BINARY(
                Box::new(Expression::LITERAL(10)),
                Op::PLUS,
                Box::new(Expression::BINARY(
                    Box::new(Expression::LITERAL(10)),
                    Op::MULT,
                    Box::new(Expression::LITERAL(10)),
                )),
            )),
            Op::PLUS,
            Box::new(Expression::LITERAL(10)),
        );
        test(tokens, result);
    }

    #[test]
    fn mult_then_plus_then_mult() {
        let tokens = vec![
            Token::LITERAL(10),
            Token::MULT,
            Token::LITERAL(10),
            Token::PLUS,
            Token::LITERAL(10),
            Token::MULT,
            Token::LITERAL(10),
        ];

        let lhs = Box::new(Expression::BINARY(
            Box::new(Expression::LITERAL(10)),
            Op::MULT,
            Box::new(Expression::LITERAL(10)),
        ));
        let rhs = lhs.clone();

        let result = Expression::BINARY(lhs, Op::PLUS, rhs);
        test(tokens, result);
    }

    #[test]
    fn plus_mult_pow_plus() {
        let tokens = vec![
            Token::LITERAL(10),
            Token::PLUS,
            Token::LITERAL(10),
            Token::MULT,
            Token::LITERAL(10),
            Token::POW,
            Token::LITERAL(10),
            Token::PLUS,
            Token::LITERAL(99),
        ];

        let pow = Box::new(Expression::BINARY(
            Box::new(Expression::LITERAL(10)),
            Op::POW,
            Box::new(Expression::LITERAL(10)),
        ));

        let mult = Box::new(Expression::BINARY(
            Box::new(Expression::LITERAL(10)),
            Op::MULT,
            pow,
        ));

        let first_plus = Box::new(Expression::BINARY(
            Box::new(Expression::LITERAL(10)),
            Op::PLUS,
            mult,
        ));

        let last_plus = Expression::BINARY(first_plus, Op::PLUS, Box::new(Expression::LITERAL(99)));

        test(tokens, last_plus);
    }
}
