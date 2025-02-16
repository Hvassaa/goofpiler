use core::panic;
use std::{iter::Peekable, slice::Iter};

use crate::{pratt::E, tokenizer::{Token, OP}};

#[derive(Clone, Debug, PartialEq, Eq)]
enum Type {
    EXPRESSION(E),
    LITERAL(u32),
    OP(OP),
    LPAREN,
    RPAREN,
}

impl OP {
    fn from(token: &Token) -> Option<OP> {
        match token {
            Token::OPERATOR(OP::PLUS) => Some(OP::PLUS),
            Token::OPERATOR(OP::MINUS) => Some(OP::MINUS),
            Token::OPERATOR(OP::MULT) => Some(OP::MULT),
            Token::OPERATOR(OP::DIV) => Some(OP::DIV),
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
                        .push(Type::EXPRESSION(E::LITERAL(*v))),
                    Token::OPERATOR(OP::PLUS) => self.parse_stack.push(Type::OP(OP::PLUS)),
                    Token::OPERATOR(OP::MULT) => self.parse_stack.push(Type::OP(OP::MULT)),
                    Token::LPAREN => self.parse_stack.push(Type::LPAREN),
                    Token::RPAREN => self.parse_stack.push(Type::RPAREN),
                    Token::OPERATOR(OP::POW) => self.parse_stack.push(Type::OP(OP::POW)),
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
            Some(Token::OPERATOR(OP::PLUS)) | Some(Token::OPERATOR(OP::MINUS)) => Some(1),
            Some(Token::OPERATOR(OP::MULT)) => Some(2),
            Some(Token::OPERATOR(OP::POW)) => Some(3),
            _ => None,
        };

        let prev = self.parse_stack.get(self.parse_stack.len() - 2);

        let prev_precedence = match prev {
            Some(Type::OP(OP::PLUS)) | Some(Type::OP(OP::MINUS)) => Some(1),
            Some(Type::OP(OP::MULT)) => Some(2),
            Some(Type::OP(OP::POW)) => Some(3),
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
                        E::BINARY(Box::new(v1.clone()), op.clone(), Box::new(v2.clone()));
                    self.pop(3);
                    self.parse_stack.push(Type::EXPRESSION(e));
                    true
                }
                (Type::LPAREN, Type::EXPRESSION(e), Type::RPAREN) => {
                    let e = E::PAREN(Box::new(e.clone()));
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

    pub fn run(&mut self) -> &E {
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

    fn test(tokens: Vec<Token>, result: E) {
        let mut parser = Parser::new(&tokens);
        let ast = parser.run();
        assert_eq!(result, *ast)
    }

    #[test]
    fn single_literal() {
        let tokens = vec![Token::LITERAL(10)];
        let result = E::LITERAL(10);
        test(tokens, result);
    }

    #[test]
    fn addition() {
        let tokens = vec![Token::LITERAL(10), Token::OPERATOR(OP::PLUS), Token::LITERAL(15)];
        let result = E::BINARY(
            Box::new(E::LITERAL(10)),
            OP::PLUS,
            Box::new(E::LITERAL(15)),
        );
        test(tokens, result);
    }

    #[test]
    fn sum() {
        let tokens = vec![
            Token::LITERAL(10),
            Token::OPERATOR(OP::PLUS),
            Token::LITERAL(15),
            Token::OPERATOR(OP::PLUS),
            Token::LITERAL(10),
        ];
        let result = E::BINARY(
            Box::new(E::BINARY(
                Box::new(E::LITERAL(10)),
                OP::PLUS,
                Box::new(E::LITERAL(15)),
            )),
            OP::PLUS,
            Box::new(E::LITERAL(10)),
        );
        test(tokens, result);
    }

    #[test]
    fn paren() {
        let tokens = vec![Token::LPAREN, Token::LITERAL(15), Token::RPAREN];
        let result = E::PAREN(Box::new(E::LITERAL(15)));
        test(tokens, result);
    }

    #[test]
    fn plus_then_mult() {
        let tokens = vec![
            Token::LITERAL(10),
            Token::OPERATOR(OP::PLUS),
            Token::LITERAL(15),
            Token::OPERATOR(OP::MULT),
            Token::LITERAL(10),
        ];
        let result = E::BINARY(
            Box::new(E::LITERAL(10)),
            OP::PLUS,
            Box::new(E::BINARY(
                Box::new(E::LITERAL(15)),
                OP::MULT,
                Box::new(E::LITERAL(10)),
            )),
        );
        test(tokens, result);
    }

    #[test]
    fn plus_then_mult_then_mult() {
        let tokens = vec![
            Token::LITERAL(10),
            Token::OPERATOR(OP::PLUS),
            Token::LITERAL(10),
            Token::OPERATOR(OP::MULT),
            Token::LITERAL(10),
            Token::OPERATOR(OP::MULT),
            Token::LITERAL(10),
        ];
        let result = E::BINARY(
            Box::new(E::LITERAL(10)),
            OP::PLUS,
            Box::new(E::BINARY(
                Box::new(E::BINARY(
                    Box::new(E::LITERAL(10)),
                    OP::MULT,
                    Box::new(E::LITERAL(10)),
                )),
                OP::MULT,
                Box::new(E::LITERAL(10)),
            )),
        );
        test(tokens, result);
    }

    #[test]
    fn plus_then_mult_then_plus() {
        let tokens = vec![
            Token::LITERAL(10),
            Token::OPERATOR(OP::PLUS),
            Token::LITERAL(10),
            Token::OPERATOR(OP::MULT),
            Token::LITERAL(10),
            Token::OPERATOR(OP::PLUS),
            Token::LITERAL(10),
        ];
        let result = E::BINARY(
            Box::new(E::BINARY(
                Box::new(E::LITERAL(10)),
                OP::PLUS,
                Box::new(E::BINARY(
                    Box::new(E::LITERAL(10)),
                    OP::MULT,
                    Box::new(E::LITERAL(10)),
                )),
            )),
            OP::PLUS,
            Box::new(E::LITERAL(10)),
        );
        test(tokens, result);
    }

    #[test]
    fn mult_then_plus_then_mult() {
        let tokens = vec![
            Token::LITERAL(10),
            Token::OPERATOR(OP::MULT),
            Token::LITERAL(10),
            Token::OPERATOR(OP::PLUS),
            Token::LITERAL(10),
            Token::OPERATOR(OP::MULT),
            Token::LITERAL(10),
        ];

        let lhs = Box::new(E::BINARY(
            Box::new(E::LITERAL(10)),
            OP::MULT,
            Box::new(E::LITERAL(10)),
        ));
        let rhs = lhs.clone();

        let result = E::BINARY(lhs, OP::PLUS, rhs);
        test(tokens, result);
    }

    #[test]
    fn plus_mult_pow_plus() {
        let tokens = vec![
            Token::LITERAL(10),
            Token::OPERATOR(OP::PLUS),
            Token::LITERAL(10),
            Token::OPERATOR(OP::MULT),
            Token::LITERAL(10),
            Token::OPERATOR(OP::POW),
            Token::LITERAL(10),
            Token::OPERATOR(OP::PLUS),
            Token::LITERAL(99),
        ];

        let pow = Box::new(E::BINARY(
            Box::new(E::LITERAL(10)),
            OP::POW,
            Box::new(E::LITERAL(10)),
        ));

        let mult = Box::new(E::BINARY(
            Box::new(E::LITERAL(10)),
            OP::MULT,
            pow,
        ));

        let first_plus = Box::new(E::BINARY(
            Box::new(E::LITERAL(10)),
            OP::PLUS,
            mult,
        ));

        let last_plus = E::BINARY(first_plus, OP::PLUS, Box::new(E::LITERAL(99)));

        test(tokens, last_plus);
    }
}
