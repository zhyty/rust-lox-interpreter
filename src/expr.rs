use crate::scanner;

pub trait Visitor {
    type Result;

    fn visit_expr(&mut self, expr: &Expr) -> Self::Result;
}

pub enum Expr<'a> {
    Binary {
        left: Box<Expr<'a>>,
        operator: scanner::AnnotatedToken<'a>,
        right: Box<Expr<'a>>,
    },
    Grouping(Box<Expr<'a>>),
    LiteralNumber(f64),
    LiteralString(&'a str),
    Unary {
        operator: scanner::AnnotatedToken<'a>,
        right: Box<Expr<'a>>,
    },
}
