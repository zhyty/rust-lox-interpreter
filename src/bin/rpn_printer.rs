use expr::Expr::*;
use first_interpreter::expr;
use first_interpreter::expr::Visitor;
use first_interpreter::rpn_print::RPNPrinter;
use first_interpreter::scanner;
use scanner::AnnotatedToken;
use scanner::Token::*;

fn main() -> anyhow::Result<()> {
    // (-123) * (45.67)
    let test = Binary {
        left: Box::new(Unary {
            operator: AnnotatedToken {
                token: Minus,
                line_number: 1,
            },
            right: Box::new(LiteralNumber(123.0)),
        }),
        operator: AnnotatedToken {
            token: Star,
            line_number: 1,
        },
        right: Box::new(Grouping(Box::new(LiteralNumber(45.67)))),
    };
    // (1 + 2) * (4 - 3)
    let test2 = Binary {
        left: Box::new(Binary {
            left: Box::new(LiteralNumber(1.0)),
            operator: AnnotatedToken {
                token: Plus,
                line_number: 1,
            },
            right: Box::new(LiteralNumber(2.0)),
        }),
        operator: AnnotatedToken {
            token: Star,
            line_number: 1,
        },
        right: Box::new(Binary {
            left: Box::new(LiteralNumber(4.0)),
            operator: AnnotatedToken {
                token: Minus,
                line_number: 1,
            },
            right: Box::new(LiteralNumber(3.0)),
        }),
    };
    println!("{}", RPNPrinter.visit_expr(&test));
    println!("{}", RPNPrinter.visit_expr(&test2));
    Ok(())
}
