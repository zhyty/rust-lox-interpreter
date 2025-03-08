use crate::ast_print::token_to_string;
use crate::expr;

/// Prints in reverse-polish notation. 
pub struct RPNPrinter;

impl expr::Visitor for RPNPrinter {
    type Result = String;

    fn visit_expr(&mut self, expr: &expr::Expr) -> Self::Result {
        use expr::Expr::*;
        match expr {
            Binary {
                left,
                operator,
                right,
            } => {
                format!(
                    "{} {} {}",
                    self.visit_expr(left),
                    self.visit_expr(right),
                    token_to_string(operator),
                )
            }
            Grouping(expr) => {
                format!("({})", self.visit_expr(expr))
            }
            LiteralNumber(number) => {
                format!("{}", number)
            }
            LiteralString(ss) => {
                format!("{}", ss)
            }
            LiteralBool(bb) => {
                format!("{}", bb)
            }
            Nil => "nil".to_owned(),
            // Note: this is tricky... we'd have to represent the unary
            // differently to distinguish it from a binary operation. We just
            // use the debug representation.
            Unary { operator, right } => {
                format!("{} {:#?}", self.visit_expr(right), operator.token)
            }
        }
    }
}
