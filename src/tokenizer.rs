use core::panic;
use std::{iter::Peekable, str::Chars};

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    LITERAL(u32),
    MINUS,
    PLUS,
    MULT,
    DIV,
    EQUALITY,
    GREATER,
    LESS,
    GEQ,
    LEQ,
    LPAREN,
    RPAREN,
    POW,
}

pub struct Tokenizer<'a> {
    source: Peekable<Chars<'a>>,
    tokens: Vec<Token>,
}

impl<'a> Tokenizer<'a> {
    fn new(source: &'a String) -> Tokenizer<'a> {
        let source = source.chars().peekable();
        Tokenizer {
            source,
            tokens: vec![],
        }
    }

    fn next(&mut self) -> Option<char> {
        self.source.next()
    }

    fn peek(&mut self) -> Option<&char> {
        self.source.peek()
    }

    fn run(&mut self) -> &Vec<Token> {
        match self.next() {
            Some(c) => {
                match c {
                    '+' => self.tokens.push(Token::PLUS),
                    '-' => self.tokens.push(Token::MINUS),
                    '*' => self.tokens.push(Token::MULT),
                    '/' => self.tokens.push(Token::DIV),
                    '(' => self.tokens.push(Token::LPAREN),
                    ')' => self.tokens.push(Token::RPAREN),
                    '^' => self.tokens.push(Token::POW),
                    '>' => {
                        if self.peek().is_some() && *self.peek().unwrap() == '=' {
                            self.tokens.push(Token::GEQ);
                            self.next();
                        } else {
                            self.tokens.push(Token::GREATER);
                        }
                    }
                    '<' => {
                        if self.peek().is_some() && *self.peek().unwrap() == '=' {
                            self.tokens.push(Token::LEQ);
                            self.next();
                        } else {
                            self.tokens.push(Token::LESS);
                        }
                    }
                    '=' => {
                        if self.next().unwrap() == '=' {
                            self.tokens.push(Token::EQUALITY);
                        } else {
                            panic!("MALFORMED");
                        }
                    }
                    _ if c.is_numeric() => {
                        let mut digits = vec![c.to_digit(10).unwrap()];
                        while self.peek().is_some() && self.peek().unwrap().is_numeric() {
                            digits.push(self.next().unwrap().to_digit(10).unwrap());
                        }
                        let q = digits.iter().rev().enumerate().fold(0, |acc, (i, j)| {
                            let base: i32 = 10;
                            let power = base.pow(i as u32) as u32;
                            acc + (power * (*j))
                        });
                        self.tokens.push(Token::LITERAL(q));
                    }
                    _ if c.is_whitespace() => (),
                    _ => todo!(),
                }

                self.run()
            }
            None => &self.tokens,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(input: &str, expected: Vec<Token>) {
        let test = String::from(input);
        let mut tokenizer = Tokenizer::new(&test);
        let res = tokenizer.run();

        assert_eq!(expected, *res);
    }

    #[test]
    fn single_literal() {
        test("1", vec![Token::LITERAL(1)]);
    }

    #[test]
    fn single_big_literal() {
        test("123", vec![Token::LITERAL(123)]);
    }

    #[test]
    fn unary_minus_number() {
        let expected = vec![Token::MINUS, Token::LITERAL(123)];
        test("-123", expected);
    }

    #[test]
    fn addition() {
        let expected = vec![Token::LITERAL(3), Token::PLUS, Token::LITERAL(5)];
        test("3+5", expected);
    }

    #[test]
    fn mult() {
        let expected = vec![Token::LITERAL(5), Token::MULT, Token::LITERAL(123)];
        test("5*123", expected);
    }

    #[test]
    fn div() {
        let expected = vec![Token::LITERAL(10), Token::DIV, Token::LITERAL(2)];
        test("10/2", expected);
    }

    #[test]
    fn whitespace() {
        let expected = vec![Token::LITERAL(5)];
        test(" 5", expected);
    }

    #[test]
    fn whitespace_and_addition() {
        let expected = vec![Token::LITERAL(5), Token::PLUS, Token::LITERAL(10)];
        test(" 5 +     10", expected);
    }

    #[test]
    fn equality() {
        let expected = vec![Token::LITERAL(10), Token::EQUALITY, Token::LITERAL(10)];
        test("10==10", expected);
    }

    #[test]
    fn less() {
        let expected = vec![Token::LITERAL(10), Token::LESS, Token::LITERAL(9)];
        test("10 < 9", expected);
    }

    #[test]
    fn greater() {
        let expected = vec![Token::LITERAL(10), Token::GREATER, Token::LITERAL(11)];
        test("10 > 11", expected);
    }

    #[test]
    fn less_equal() {
        let expected = vec![Token::LITERAL(10), Token::LEQ, Token::LITERAL(10)];
        test("10 <= 10", expected);
    }

    #[test]
    fn greater_equal() {
        let expected = vec![Token::LITERAL(10), Token::GEQ, Token::LITERAL(11)];
        test("10 >= 11", expected);
    }

    #[test]
    fn parens() {
        let expected = vec![
            Token::LPAREN,
            Token::LITERAL(2),
            Token::PLUS,
            Token::LITERAL(5),
            Token::RPAREN,
        ];
        test("(2 + 5)", expected);
    }
}
