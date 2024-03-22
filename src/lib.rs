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

fn format_command(command: &Command) -> String {
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

    if args.is_dry_run() {
        println!("{}", format_command(&command));
    } else {
        command.current_dir(&context.root).spawn()?;
    }

    Ok(())
}
