use crate::{pratt::E, tokenizer::OP};

fn resolve(expression: E) -> i32 {
    match expression {
        E::LITERAL(v) => v as i32,
        E::UNARY(OP::PLUS, v) | E::PAREN(v) => resolve(*v),
        E::UNARY(OP::MINUS, v) => -resolve(*v),
        E::BINARY(v1, OP::MINUS, v2) => resolve(*v1) - resolve(*v2),
        E::BINARY(v1, OP::PLUS, v2) => resolve(*v1) + resolve(*v2),
        E::BINARY(v1, OP::MULT, v2) => resolve(*v1) * resolve(*v2),
        E::BINARY(v1, OP::DIV, v2) => resolve(*v1) / resolve(*v2),
        E::BINARY(v1, OP::POW, v2) => resolve(*v1).pow(resolve(*v2) as u32) as i32,
        E::IF(cond, then, elze) => {
            let cond = resolve(*cond);
            if cond == 1 {
                resolve(*then)
            } else {
                resolve(*elze)
            }
        }
        _ => todo!("Not implemented"),
    }
}

#[cfg(test)]
mod tests {
    use crate::{parser::Parser, pratt::expression, tokenizer::Tokenizer};

    use super::*;

    fn test(input: &str, result: i32) {
        let binding = String::from(input);
        let mut tokenizer = Tokenizer::new(&binding);
        let tokens = tokenizer.run();

        // LR(1) parser
        //let mut parser = Parser::new(tokens);
        //let ast = parser.run();
        //let resolved = resolve(ast.to_owned());
        //assert_eq!(result, resolved, "LR(1)");


        // Pratt parser
        let ast = expression(&mut tokens.iter().peekable(), 0);
        let resolved = resolve(ast.to_owned());
        assert_eq!(result, resolved, "Pratt");
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

    #[test]
    fn true_if() {
        test("if 1 | 1 else 2", 1);
    }

    #[test]
    fn false_if() {
        test("if 0 | 1 else 2", 2);
    }

    #[test]
    fn addition_if() {
        test("1 + if 0 | 1 else 2", 3);
    }

    #[test]
    fn addition_right_if() {
        test("if 1 | 1 else 2 + 1", 2);
    }
}
