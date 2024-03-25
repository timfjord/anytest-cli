use clap::ValueEnum;
use std::error::Error;
use std::path::PathBuf;
use std::process::Command;

pub type Line = usize;

#[derive(ValueEnum, Clone, Debug)]
pub enum Scope {
    Suite,
    File,
    Line,
}

#[derive(Debug)]
pub struct Context {
    root: PathBuf,
    path: PathBuf,
    line: Option<Line>,
}

impl Context {
    pub fn new(root: PathBuf, path: PathBuf, line: Option<Line>) -> Context {
        Self { root, path, line }
    }

    pub fn root(&self) -> &PathBuf {
        &self.root
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn line(&self) -> Option<&Line> {
        self.line.as_ref()
    }
}

pub fn build_command(scope: Scope, context: Context) -> Result<Command, Box<dyn Error>> {
    let mut command = Command::new("echo");
    command.current_dir(&context.root);

    match scope {
        Scope::Suite => command.args(["suite"]),
        Scope::File => command.args(["file", context.path.to_str().unwrap()]),
        Scope::Line => command.args([
            "line",
            context.path.to_str().unwrap(),
            context.line.unwrap().to_string().as_str(),
        ]),
    };

    Ok(command)
}
