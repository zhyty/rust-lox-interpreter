use crate::expr;
use crate::scanner;
use crate::scanner::AnnotatedToken;
use crate::scanner::Token;
use std::iter::Peekable;

pub struct Parser<'a> {
    tokens: &'a Vec<scanner::AnnotatedToken<'a>>,
    current_index: usize,
}

impl<'a> Parser<'a> {
    // TODO: should we require that tokens ends with EOF?
    pub fn new(tokens: &'a Vec<scanner::AnnotatedToken>) -> Self {
        Parser {
            tokens,
            current_index: 0,
        }
    }

    // Recursive descent parsing
    fn expression(&mut self) -> anyhow::Result<Box<expr::Expr<'a>>> {
        self.equality()
    }

    fn equality(&mut self) -> anyhow::Result<Box<expr::Expr<'a>>> {
        let mut expr = self.comparison()?;
        loop {
            let possible_op = self.peek();
            let annotated_token = if let Some(annotated_token) = possible_op {
                annotated_token.clone()
            } else { 
                break;
            };
            match annotated_token.token {
                Token::BangEqual | Token::EqualEqual => {
                    self.advance();
                    expr = Box::new(expr::Expr::Binary {
                        left: expr,
                        operator: annotated_token,
                        right: self.comparison()?,
                    });
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> anyhow::Result<Box<expr::Expr<'a>>> {
        // TODO
        self.term()
    }

    fn term(&mut self) -> anyhow::Result<Box<expr::Expr<'a>>> {
        // TODO
        self.factor()
    }

    fn factor(&mut self) -> anyhow::Result<Box<expr::Expr<'a>>> {
        // TODO
        self.unary()
    }

    fn unary(&mut self) -> anyhow::Result<Box<expr::Expr<'a>>> {
        // TODO
        self.primary()
    }

    fn primary(&mut self) -> anyhow::Result<Box<expr::Expr<'a>>> {
        let token = if let Some(annotated_token) = self.peek() {
            annotated_token.token.clone()
        } else {
            anyhow::bail!("Expect expression");
        };
        
        let expr = match token {
            Token::False => Box::new(expr::Expr::LiteralBool(false)),
            Token::True => Box::new(expr::Expr::LiteralBool(true)),
            Token::Nil => Box::new(expr::Expr::Nil),
            Token::Number { number } => {
                Box::new(expr::Expr::LiteralNumber(number.clone()))
            }
            Token::String { quoted_str } => {
                Box::new(expr::Expr::LiteralString(quoted_str))
            }
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
    
    fn matches(&self, pred: fn(&scanner::Token) -> bool) -> bool {
        if let Some(annotated_token) = self.peek() {
            let matches = pred(&annotated_token.token);
            matches
        } else {
            false
        }
    }

    fn advance_if_matches(&mut self, pred: fn(&scanner::Token) -> bool) -> bool {
        let matches = self.matches(pred);
        if matches {
            self.advance();
        }
        matches
    }

    fn peek(&self) -> Option<&scanner::AnnotatedToken<'a>> {
        if self.at_end() {
            return None;
        }
        Some(&self.tokens[self.current_index])
    }

    fn at_end(&self) -> bool {
        // TODO: check for EOF token?
        self.current_index >= self.tokens.len()
    }
}
