use crate::{Context, Line};
use clap::{Parser, ValueEnum};
use regex::Regex;
use std::env;
use std::error::Error;
use std::path::PathBuf;

#[derive(ValueEnum, Clone, Debug)]
pub enum Scope {
    Suite,
    File,
    Line,
}

/// Run any test from CLI
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg()]
    /// Path to the test file
    path: String,

    /// Specify what tests to run
    #[arg(short, long)]
    scope: Option<Scope>,

    /// Path to the root directory, if not passed, the current directory is used
    #[arg(short, long)]
    root: Option<String>,

    /// Path to the config file
    #[arg(short, long)]
    config: Option<String>,

    /// Whether to run in dry-run mode
    #[arg(long, default_value_t = false)]
    dry_run: bool,
}

impl Args {
    pub fn build_context(&self) -> Result<Context, Box<dyn Error>> {
        let root = if let Some(root) = &self.root {
            PathBuf::from(root)
        } else {
            env::current_dir()?
        };
        let re = Regex::new(r"^(.*?)(?::(\d+))?$")?;
        let caps = re.captures(&self.path).ok_or("Invalid path")?;
        let path = caps.get(1).ok_or("Invalid path")?.as_str();
        let line = caps
            .get(2)
            .map(|m| m.as_str().parse::<Line>())
            .transpose()?;

        Ok(Context::new(root, PathBuf::from(path), line))
    }

    pub fn scope(&self, context: &Context) -> Scope {
        if let Some(scope) = &self.scope {
            scope.clone()
        } else if let Some(_) = context.line {
            Scope::Line
        } else {
            Scope::File
        }
    }
}
