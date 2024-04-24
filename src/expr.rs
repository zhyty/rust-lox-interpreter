trait ExprVisitor {
    fn visitBinary(binary: Binary);
    fn visitGrouping(grouping: Grouping);
    fn visitLiteral(literal: Literal);
    fn visitUnary(unary: Unary);
}

trait Expr {
    fn accept(visitor: impl ExprVisitor);
}

struct Binary<'a> {
    left: Box<dyn Expr>,
    operator: AnnotatedToken<'a>,
    right: Box<dyn Expr>,
}

struct Grouping(Box<dyn Expr>);

// TODO: numbers for now, but should at least include strings later.
struct Literal(f64);

struct Unary<'a> {
    operator: AnnotatedToken<'a>,
    right: Box<dyn Expr>,
}

impl<'a> Expr for Binary<'a> {
    fn accept(visitor: impl ExprVisitor) {
        visitor.visitBinary(self);
    }
}

impl Expr for Grouping {
    fn accept(visitor: impl ExprVisitor) {
        visitor.visitGrouping(self);
    }
}

impl Expr for Literal {
    fn accept(visitor: impl ExprVisitor) {
        visitor.visitLiteral(self);
    }
}

impl<'a> Expr for Unary<'a> {
    fn accept(visitor: impl ExprVisitor) {
        visitor.visitUnary(self);
    }
}