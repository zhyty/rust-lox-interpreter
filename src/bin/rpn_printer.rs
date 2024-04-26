use first_interpreter::expr;
use first_interpreter::expr::Visitor;
use first_interpreter::scanner;

/// Prints in reverse-polish notation.
struct RPNPrinter;

impl expr::Visitor for RPNPrinter {
    type Result = String;

    // TODO: non-debug string representation of operator token?
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
            // Note: this is tricky... we'd have to represent the unary
            // differently to distinguish it from a binary operation. We just
            // use the debug representation.
            Unary { operator, right } => {
                format!("{} {:#?}", self.visit_expr(right), operator.token)
            }
        }
    }
}

// TODO: AST printer should probably expose this

fn token_to_string(annotated: &scanner::AnnotatedToken) -> String {
    use scanner::Token::*;
    match annotated.token {
        LeftParen => "(".to_owned(),
        RightParen => ")".to_owned(),
        LeftBrace => "{".to_owned(),
        RightBrace => "}".to_owned(),
        Comma => ",".to_owned(),
        Dot => ".".to_owned(),
        Minus => "-".to_owned(),
        Plus => "+".to_owned(),
        Semicolon => ";".to_owned(),
        Slash => "/".to_owned(),
        Star => "*".to_owned(),

        Bang => "!".to_owned(),
        BangEqual => "!=".to_owned(),
        Equal => "=".to_owned(),
        EqualEqual => "==".to_owned(),
        Greater => ">".to_owned(),
        GreaterEqual => ">=".to_owned(),
        Less => "<".to_owned(),
        LessEqual => "<=".to_owned(),

        And => "and".to_owned(),
        Class => "class".to_owned(),
        Else => "else".to_owned(),
        False => "false".to_owned(),
        Fun => "fun".to_owned(),
        For => "for".to_owned(),
        If => "if".to_owned(),
        Nil => "nil".to_owned(),
        Or => "or".to_owned(),
        Print => "print".to_owned(),
        Return => "return".to_owned(),
        Super => "super".to_owned(),
        This => "this".to_owned(),
        True => "true".to_owned(),
        Var => "var".to_owned(),
        While => "while".to_owned(),

        // Literals.
        Identifier { identifier } => identifier.to_string(),
        String { quoted_str } => quoted_str.to_string(),
        Number { number } => format!("{}", number),

        EOF => "EOF".to_owned(),
    }
}

fn main() -> anyhow::Result<()> {
    use expr::Expr::*;
    use scanner::Token::*;
    use scanner::AnnotatedToken;
    // (-123) * (45.67)
    let test = Binary {
        left: Box::new(Unary {
            operator: AnnotatedToken { token: Minus, line_number: 1 },
            right: Box::new(LiteralNumber(123.0)),
        }),
        operator: AnnotatedToken { token: Star, line_number: 1 },
        right: Box::new(Grouping(Box::new(LiteralNumber(45.67)))),
    };
    // (1 + 2) * (4 - 3)
    let test2 = Binary {
        left: Box::new(Binary {
            left: Box::new(LiteralNumber(1.0)),
            operator: AnnotatedToken { token: Plus, line_number: 1 },
            right: Box::new(LiteralNumber(2.0)),
        }),
        operator: AnnotatedToken { token: Star, line_number: 1 },
        right: Box::new(Binary {
            left: Box::new(LiteralNumber(4.0)),
            operator: AnnotatedToken { token: Minus, line_number: 1 },
            right: Box::new(LiteralNumber(3.0)),
        }),
    };
    println!("{}", RPNPrinter.visit_expr(&test));
    println!("{}", RPNPrinter.visit_expr(&test2));
    Ok(())
}
