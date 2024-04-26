use anyhow::bail;
use clap::Parser;
use std::fs::File;
use std::io::BufRead;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;

use first_interpreter::scanner::Scanner;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
/// Lox tree-walk interpreter
struct Args {
    script: Option<PathBuf>,
}

#[derive(Debug)]
struct Lox {
    // NOTE: might be better if Lox initialized with some strategy for error
    // handling.
    has_error: bool,
}

impl Lox {
    pub fn run_file(script: &Path) -> anyhow::Result<()> {
        // Note: we don't need a bufreader for now since we're just reading all
        // at once.
        let mut f = File::open(script)?;
        let mut contents = String::new();
        f.read_to_string(&mut contents)?;
        let mut lox = Lox::new();
        lox.run(&contents);

        if lox.has_error() {
            bail!("TODO: some error msg");
        }

        Ok(())
    }

    pub fn run_repl() -> anyhow::Result<()> {
        let mut lox = Lox::new();
        let stdin = std::io::stdin();
        for line in stdin.lock().lines() {
            let line = line?;
            lox.run(&line);
        }

        if lox.has_error() {
            bail!("TODO: some error msg");
        }

        Ok(())
    }

    pub fn new() -> Self {
        Self { has_error: false }
    }

    pub fn run(&mut self, code: &str) {
        let mut scanner = Scanner::new(code);
        let tokens = scanner.scan_tokens();
        println!("Tokens:");
        println!("{:#?}", tokens);
        // TODO:
    }

    pub fn has_error(&self) -> bool {
        self.has_error
    }

    fn report_error(&mut self, line: u64, message: &str) {
        eprintln!("[line {}] Error (TODO where): {}", line, message);
        self.has_error = true;
    }
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    match args.script {
        Some(script) => {
            Lox::run_file(&script)?;
        }
        None => {
            Lox::run_repl()?;
        }
    };

    Ok(())
}
