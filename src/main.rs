use clap::Parser;
use std::fs::File;
use std::io::BufRead;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
/// Lox tree-walk interpreter
struct Args {
    script: Option<PathBuf>,
}

#[derive(Debug)]
struct Lox {
    // TODO: some state
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

        Ok(())
    }

    pub fn run_repl() -> anyhow::Result<()> {
        let mut lox = Lox::new();
        let stdin = std::io::stdin();
        for line in stdin.lock().lines() {
            let line = line?;
            lox.run(&line);
        }

        Ok(())
    }

    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&mut self, code: &str) {
        unimplemented!()
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
    }

    return Ok(());
}
