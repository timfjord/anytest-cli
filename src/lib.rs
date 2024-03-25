use clap::ValueEnum;
use std::error::Error;
use std::path::PathBuf;
use std::process::Command;

pub type Line = usize;

#[derive(Debug)]
pub struct RelPath {
    root: PathBuf,
    path: PathBuf,
}

impl RelPath {
    pub fn new(root: Option<&str>, path: &str) -> Result<Self, Box<dyn Error>> {
        let root = if let Some(root) = root {
            PathBuf::from(root)
        } else {
            std::env::current_dir()?
        };
        let path = PathBuf::from(path);

        Ok(Self { root, path })
    }

    pub fn root(&self) -> &PathBuf {
        &self.root
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}

#[derive(ValueEnum, Clone, Debug)]
pub enum Scope {
    Suite,
    File,
    Line,
}

#[derive(Debug)]
pub struct Context {
    rel_path: RelPath,
    line: Option<Line>,
}

impl Context {
    pub fn new(root: Option<&str>, path: &str, line: Option<Line>) -> Result<Self, Box<dyn Error>> {
        let rel_path = RelPath::new(root, path)?;

        Ok(Self { rel_path, line })
    }

    pub fn root(&self) -> &PathBuf {
        &self.rel_path.root()
    }

    pub fn path(&self) -> &PathBuf {
        &self.rel_path.path()
    }

    pub fn line(&self) -> Option<Line> {
        self.line
    }
}

pub fn build_command(scope: Scope, context: Context) -> Result<Command, Box<dyn Error>> {
    let mut command = Command::new("echo");
    command.current_dir(&context.root());

    match scope {
        Scope::Suite => command.args(["suite"]),
        Scope::File => command.args(["file", context.path().to_str().unwrap()]),
        Scope::Line => command.args([
            "line",
            context.path().to_str().unwrap(),
            context.line().unwrap().to_string().as_str(),
        ]),
    };

    Ok(command)
}
