use crate::scanner;

pub trait Visitor {
    type Result;

    fn visit_expr(&mut self, expr: &Expr) -> Self::Result;
}

pub trait Acceptor {
    fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Result;
}

pub enum Expr<'a> {
    Binary {
        left: Box<Expr<'a>>,
        operator: scanner::Token<'a>,
        right: Box<Expr<'a>>,
    },
    Grouping(Box<Expr<'a>>),
    LiteralNumber(f64),
    LiteralString(&'a str),
    Unary {
        operator: scanner::Token<'a>,
        right: Box<Expr<'a>>,
    },
}

impl<'a> Acceptor for Expr<'a> {
    fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Result {
        visitor.visit_expr(self)
    }
}
