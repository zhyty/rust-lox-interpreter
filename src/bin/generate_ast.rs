use clap::Parser;
use std::path::Path;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(about, long_about = None)]
/// Helper binary to generate AST file.
struct Args {
    output_dir: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    define_ast(
        &args.output_dir,
        "generated_expr",
        &vec![
            "Binary   : Expr left, Token operator, Expr right",
            "Grouping : Expr expression",
            "Literal  : Object value",
            "Unary    : Token operator, Expr right",
        ],
    )
}

fn define_ast(output_dir: &Path, basename: &str, types: &Vec<&str>) -> anyhow::Result<()> {
    let path = output_dir.join(format!("{}.rs", basename));
    Ok(())
}
