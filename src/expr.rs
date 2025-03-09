use crate::scanner;

pub trait Visitor {
    type Result;

    fn visit_expr(&mut self, expr: &Expr) -> Self::Result;
}

// TODO: maybe expr should take tokens as reference? It would avoid cloning
pub enum Expr<'a> {
    Binary {
        left: Box<Expr<'a>>,
        operator: scanner::AnnotatedToken<'a>,
        right: Box<Expr<'a>>,
    },
    Grouping(Box<Expr<'a>>),
    Nil,
    LiteralNumber(f64),
    LiteralString(&'a str),
    LiteralBool(bool),
    Unary {
        operator: scanner::AnnotatedToken<'a>,
        right: Box<Expr<'a>>,
    },
}
