use any_test::{Context, Line, Scope};
use clap::Parser;
use regex::Regex;
use std::error::Error;
use std::process::Command;

const PATH_REGEX: &str = r"^(.*?)(?::(\d+))?$";

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
        let re = Regex::new(PATH_REGEX)?;
        let caps = re.captures(&self.path).ok_or("Invalid path")?;
        let path = caps.get(1).ok_or("Invalid path")?.as_str();
        let line = caps
            .get(2)
            .map(|m| m.as_str().parse::<Line>())
            .transpose()?;

        Context::new(self.root.as_deref(), path, line)
    }

    pub fn scope(&self, context: &Context) -> Scope {
        if let Some(scope) = &self.scope {
            scope.clone()
        } else if let Some(_) = context.line() {
            Scope::Line
        } else {
            Scope::File
        }
    }

    pub fn is_dry_run(&self) -> bool {
        self.dry_run
    }
}

pub fn format_command(command: &Command) -> String {
    format!(
        "{} {}",
        command.get_program().to_str().unwrap_or_default(),
        command
            .get_args()
            .map(|a| a.to_str().unwrap_or_default())
            .collect::<Vec<&str>>()
            .join(" ")
    )
}
