use first_interpreter::expr;
use first_interpreter::expr::Visitor;
use first_interpreter::scanner;

struct AstPrinter;

impl expr::Visitor for AstPrinter {
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
                    "({} {} {})",
                    token_to_string(operator),
                    self.visit_expr(left),
                    self.visit_expr(right)
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
            Unary { operator, right } => {
                format!("({} {})", token_to_string(operator), self.visit_expr(right))
            }
        }
    }
}

fn token_to_string(token: &scanner::Token) -> String {
    use scanner::Token::*;
    match token {
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
    let test = Binary {
        left: Box::new(Unary {
            operator: Minus,
            right: Box::new(LiteralNumber(123.0)),
        }),
        operator: Star,
        right: Box::new(Grouping(Box::new(LiteralNumber(45.67)))),
    };
    println!("{}", AstPrinter.visit_expr(&test));
    Ok(())
}
