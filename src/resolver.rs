use crate::parser::{Expression, Op};

fn resolve(expression: Expression) -> i32 {
    match expression {
        Expression::LITERAL(v) => v as i32,
        Expression::UNARY(Op::PLUS, v) | Expression::PAREN(v) => resolve(*v),
        Expression::UNARY(Op::MINUS, v) => -resolve(*v),
        Expression::BINARY(v1, Op::MINUS, v2) => resolve(*v1) - resolve(*v2),
        Expression::BINARY(v1, Op::PLUS, v2) => resolve(*v1) + resolve(*v2),
        Expression::BINARY(v1, Op::MULT, v2) => resolve(*v1) * resolve(*v2),
        Expression::BINARY(v1, Op::DIV, v2) => resolve(*v1) / resolve(*v2),
        Expression::BINARY(v1, Op::POW, v2) => resolve(*v1).pow(resolve(*v2) as u32) as i32,
        _ => todo!("Not implemented"),
    }
}

#[cfg(test)]
mod tests {
    use crate::{parser::Parser, tokenizer::Tokenizer};

    use super::*;

    fn test(input: &str, result: i32) {
        let binding = String::from(input);
        let mut tokenizer = Tokenizer::new(&binding);
        let mut parser = Parser::new(tokenizer.run());
        let ast = parser.run();
        let resolved = resolve(ast.to_owned());
        assert_eq!(result, resolved)
    }

    #[test]
    fn single_literal() {
        test("1", 1);
    }

    #[test]
    fn addition() {
        test("1 + 3", 4);
    }

    #[test]
    fn plus_mult() {
        test("1 + 3 * 2", 7);
    }

    #[test]
    fn plus_mult_with_whitespace() {
        test("1 +\n 3 * 2", 7);
    }

    #[test]
    fn pow() {
        test("1 + 2 ^ 3 + 2 + 2 * 3", 17);
    }

    #[test]
    fn pow_paren() {
        test("(1 + 2) ^ 3 + 2 + 2 * 3", 35);
    }
}
