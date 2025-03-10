use crate::expr;
use crate::scanner::AnnotatedToken;
use crate::scanner::Token;
use expr::Expr;

pub struct Parser<'a> {
    // TODO: actually this would probably be some iterator of tokens.
    tokens: &'a Vec<AnnotatedToken<'a>>,
    current_index: usize,
}

impl<'a> Parser<'a> {
    // TODO: should we require that tokens ends with EOF?
    // TODO: this should be a stream, I guess
    pub fn new(tokens: &'a Vec<AnnotatedToken>) -> Self {
        Parser {
            tokens,
            current_index: 0,
        }
    }

    pub fn parse(&mut self) -> anyhow::Result<Box<expr::Expr<'a>>> {
        self.expression()
    }

    // Recursive descent parsing
    fn expression(&mut self) -> anyhow::Result<Box<expr::Expr<'a>>> {
        self.equality()
    }

    // comparison ( ( "!=" | "==" ) comparison )*
    fn equality(&mut self) -> anyhow::Result<Box<expr::Expr<'a>>> {
        self.subrule_operator_subrule_left_associative_helper(Self::comparison, |token| {
            matches!(token, Token::BangEqual | Token::EqualEqual)
        })
    }

    // term ( ( ">" | ">=" | "<" | "<=" ) term )*
    // NOTE: left-associative
    fn comparison(&mut self) -> anyhow::Result<Box<expr::Expr<'a>>> {
        self.subrule_operator_subrule_left_associative_helper(Self::term, |token| {
            matches!(
                token,
                Token::Greater | Token::GreaterEqual | Token::Less | Token::LessEqual
            )
        })
    }

    // factor ( ( "-" | "+" ) factor )*
    // NOTE: left-associative
    fn term(&mut self) -> anyhow::Result<Box<expr::Expr<'a>>> {
        self.subrule_operator_subrule_left_associative_helper(Self::factor, |token| {
            matches!(token, Token::Minus | Token::Plus)
        })
    }

    // unary ( ( "/" | "*" ) unary )*
    // LEFT-associative
    fn factor(&mut self) -> anyhow::Result<Box<expr::Expr<'a>>> {
        self.subrule_operator_subrule_left_associative_helper(Self::unary, |token| {
            matches!(token, Token::Slash | Token::Star)
        })
    }

    // NOTE: RIGHT associative
    fn unary(&mut self) -> anyhow::Result<Box<expr::Expr<'a>>> {
        let annotated_token = self.peek().clone();
        match annotated_token.token {
            Token::Plus | Token::Minus => {
                self.advance();
                return Ok(Box::new(Expr::Unary {
                    operator: annotated_token,
                    right: self.unary()?,
                }));
            }
            _ => return self.primary(),
        }
    }

    fn primary(&mut self) -> anyhow::Result<Box<expr::Expr<'a>>> {
        let expr = match &self.peek().token {
            Token::False => Box::new(expr::Expr::LiteralBool(false)),
            Token::True => Box::new(expr::Expr::LiteralBool(true)),
            Token::Nil => Box::new(expr::Expr::Nil),
            Token::Number { number } => Box::new(expr::Expr::LiteralNumber(number.clone())),
            Token::String { quoted_str } => Box::new(expr::Expr::LiteralString(quoted_str)),
            Token::LeftParen => {
                self.advance();
                let parens_expr = self.expression()?;
                if !self.matches(|token| matches!(token, Token::RightParen)) {
                    anyhow::bail!("Couldn't find closing parens.");
                }
                Box::new(expr::Expr::Grouping(parens_expr))
            }
            _ => {
                anyhow::bail!("Unexpected token");
            }
        };

        self.advance();
        Ok(expr)
    }

    // Helpers
    fn advance(&mut self) {
        self.current_index += 1;
    }

    fn matches(&self, pred: fn(&Token) -> bool) -> bool {
        pred(&self.peek().token)
    }

    fn peek(&self) -> &AnnotatedToken<'a> {
        &self.tokens[self.current_index]
    }

    fn subrule_operator_subrule_left_associative_helper(
        &mut self,
        parse_subrule_fn: fn(&mut Self) -> anyhow::Result<Box<expr::Expr<'a>>>,
        operator_pred: fn(&Token) -> bool,
    ) -> anyhow::Result<Box<expr::Expr<'a>>> {
        let mut left_expr = parse_subrule_fn(self)?;
        loop {
            let possible_operator = self.peek().clone();
            if operator_pred(&possible_operator.token) {
                self.advance();
                let right_expr = parse_subrule_fn(self)?;
                left_expr = Box::new(Expr::Binary {
                    left: left_expr,
                    operator: possible_operator,
                    right: right_expr,
                });
            } else {
                return Ok(left_expr);
            }
        }
    }

    fn at_end(&self) -> bool {
        self.peek().token == Token::EOF
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast_print;
    use crate::expr::Visitor;
    use crate::scanner;

    fn test_with_ast(source: &'static str, expected_ast: &'static str) {
        let mut scanner = scanner::Scanner::new(source);
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();
        assert_eq!(ast_print::AstPrinter.visit_expr(&expr), expected_ast);
    }

    #[test]
    fn basic_equality() {
        test_with_ast("1 == 2;", "(== 1 2)");
    }

    #[test]
    fn multiple_equality() {
        test_with_ast("1 == 2 != 3;", "(!= (== 1 2) 3)");
    }

    #[test]
    fn unary_equality() {
        test_with_ast("+1 == -1;", "(== (+ 1) (- 1))");
    }

    #[test]
    fn unary_factor_equality() {
        test_with_ast("+1 * -2 == -1 / 4;", "(== (* (+ 1) (- 2)) (/ (- 1) 4))");
    }

    #[test]
    fn nested_unary() {
        test_with_ast("-+-+1", "(- (+ (- (+ 1))))");
    }

    #[test]
    fn bedmas_priority_left_associativity() {
        test_with_ast("+1 * -2 + +3 - 4;", "(- (+ (* (+ 1) (- 2)) (+ 3)) 4)");
    }

    #[test]
    fn bracket_primary() {
        test_with_ast("(12 + 23) * (3 - 4)", "(* ((+ 12 23)) ((- 3 4)))")
    }

    #[test]
    fn strings() {
        test_with_ast("\"str\" + \"otherstr\"", "(+ \"str\" \"otherstr\")")
    }
}
