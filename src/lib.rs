use clap::Parser;
use cli::{Args, Scope};
use std::error::Error;
use std::path::PathBuf;
use std::process::Command;

mod cli;

type Line = usize;

#[derive(Debug)]
struct Context {
    root: PathBuf,
    path: PathBuf,
    line: Option<Line>,
}

impl Context {
    fn new(root: PathBuf, path: PathBuf, line: Option<Line>) -> Context {
        Self { root, path, line }
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let context = args.build_context()?;
    let scope = args.scope(&context);

    let mut command = Command::new("echo");
    match scope {
        Scope::Suite => command.args(["suite"]),
        Scope::File => command.args(["file", context.path.to_str().unwrap()]),
        Scope::Line => command.args([
            "line",
            context.path.to_str().unwrap(),
            context.line.unwrap().to_string().as_str(),
        ]),
    };

    command.current_dir(&context.root).spawn()?;

    Ok(())
}
