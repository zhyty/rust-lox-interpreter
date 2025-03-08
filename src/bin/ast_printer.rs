use first_interpreter::ast_print;
use first_interpreter::expr::Visitor;

fn main() -> anyhow::Result<()> {
    use first_interpreter::expr::Expr::*;
    use first_interpreter::scanner::Token::*;
    use first_interpreter::scanner::AnnotatedToken;
    let test = Binary {
        left: Box::new(Unary {
            operator: AnnotatedToken { token: Minus, line_number: 1 },
            right: Box::new(LiteralNumber(123.0)),
        }),
        operator: AnnotatedToken { token: Star, line_number: 1 },
        right: Box::new(Grouping(Box::new(LiteralNumber(45.67)))),
    };
    println!("{}", ast_print::AstPrinter.visit_expr(&test));
    Ok(())
}
