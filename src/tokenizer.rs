use core::panic;
use std::{iter::Peekable, str::Chars};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum OP {
    PLUS,
    MINUS,
    MULT,
    POW,
    DIV,
    EQUALITY,
    GREATER,
    LESS,
    GEQ,
    LEQ,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    LITERAL(u32),
    BOOL(bool),
    OPERATOR(OP),
    LPAREN,
    RPAREN,
    IF,
    THEN,
    ELSE
}

pub struct Tokenizer<'a> {
    source: Peekable<Chars<'a>>,
    tokens: Vec<Token>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(source: &'a String) -> Tokenizer<'a> {
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

    pub fn run(&mut self) -> &Vec<Token> {
        match self.next() {
            Some(c) => {
                match c {
                    '+' => self.tokens.push(Token::OPERATOR(OP::PLUS)),
                    '-' => self.tokens.push(Token::OPERATOR(OP::MINUS)),
                    '*' => self.tokens.push(Token::OPERATOR(OP::MULT)),
                    '/' => self.tokens.push(Token::OPERATOR(OP::DIV)),
                    '^' => self.tokens.push(Token::OPERATOR(OP::POW)),
                    '|' => self.tokens.push(Token::THEN),
                    't' => {
                        let next = vec!['r', 'u', 'e'];
                        if next.iter().all(|c| *c == self.next().unwrap()) {
                            self.tokens.push(Token::BOOL(true));
                        } else {
                            panic!("Malformed true");
                        }
                    }
                    'f' => {
                        let next = vec!['a', 'l', 's', 'e'];
                        if next.iter().all(|c| *c == self.next().unwrap()) {
                            self.tokens.push(Token::BOOL(false));
                        } else {
                            panic!("Malformed false");
                        }
                    }
                    'i' => {
                        if self.next().unwrap() == 'f' {
                            self.tokens.push(Token::IF);
                        } else {
                            panic!("Malformed if");
                        }
                    }
                    'e' => {
                        let next = vec!['l', 's', 'e'];
                        if next.iter().all(|c| *c == self.next().unwrap()) {
                            self.tokens.push(Token::ELSE);
                        } else {
                            panic!("Malformed else");
                        }
                    }
                    '>' => {
                        if self.peek().is_some() && *self.peek().unwrap() == '=' {
                            self.tokens.push(Token::OPERATOR(OP::GEQ));
                            self.next();
                        } else {
                            self.tokens.push(Token::OPERATOR(OP::GREATER));
                        }
                    }
                    '<' => {
                        if self.peek().is_some() && *self.peek().unwrap() == '=' {
                            self.tokens.push(Token::OPERATOR(OP::LEQ));
                            self.next();
                        } else {
                            self.tokens.push(Token::OPERATOR(OP::LESS));
                        }
                    }
                    '=' => {
                        if self.next().unwrap() == '=' {
                            self.tokens.push(Token::OPERATOR(OP::EQUALITY));
                        } else {
                            panic!("MALFORMED");
                        }
                    }
                    '(' => self.tokens.push(Token::LPAREN),
                    ')' => self.tokens.push(Token::RPAREN),
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
        let expected = vec![Token::OPERATOR(OP::MINUS), Token::LITERAL(123)];
        test("-123", expected);
    }

    #[test]
    fn addition() {
        let expected = vec![Token::LITERAL(3), Token::OPERATOR(OP::PLUS), Token::LITERAL(5)];
        test("3+5", expected);
    }

    #[test]
    fn mult() {
        let expected = vec![Token::LITERAL(5), Token::OPERATOR(OP::MULT), Token::LITERAL(123)];
        test("5*123", expected);
    }

    #[test]
    fn div() {
        let expected = vec![Token::LITERAL(10), Token::OPERATOR(OP::DIV), Token::LITERAL(2)];
        test("10/2", expected);
    }

    #[test]
    fn whitespace() {
        let expected = vec![Token::LITERAL(5)];
        test(" 5", expected);
    }

    #[test]
    fn whitespace_and_addition() {
        let expected = vec![Token::LITERAL(5), Token::OPERATOR(OP::PLUS), Token::LITERAL(10)];
        test(" 5 +     10", expected);
    }

    #[test]
    fn equality() {
        let expected = vec![Token::LITERAL(10), Token::OPERATOR(OP::EQUALITY), Token::LITERAL(10)];
        test("10==10", expected);
    }

    #[test]
    fn less() {
        let expected = vec![Token::LITERAL(10), Token::OPERATOR(OP::LESS), Token::LITERAL(9)];
        test("10 < 9", expected);
    }

    #[test]
    fn greater() {
        let expected = vec![Token::LITERAL(10), Token::OPERATOR(OP::GREATER), Token::LITERAL(11)];
        test("10 > 11", expected);
    }

    #[test]
    fn less_equal() {
        let expected = vec![Token::LITERAL(10), Token::OPERATOR(OP::LEQ), Token::LITERAL(10)];
        test("10 <= 10", expected);
    }

    #[test]
    fn greater_equal() {
        let expected = vec![Token::LITERAL(10), Token::OPERATOR(OP::GEQ), Token::LITERAL(11)];
        test("10 >= 11", expected);
    }

    #[test]
    fn parens() {
        let expected = vec![
            Token::LPAREN,
            Token::LITERAL(2),
            Token::OPERATOR(OP::PLUS),
            Token::LITERAL(5),
            Token::RPAREN,
        ];
        test("(2 + 5)", expected);
    }

    #[test]
    fn parse_true() {
        let expected = vec![
            Token::BOOL(true)
        ];
        test("true", expected);
    }
}
