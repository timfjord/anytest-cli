use anytest::{Context, Line, Scope};
use clap::Parser;
use regex::Regex;
use std::error::Error;
use std::process::Command;

const PATH_REGEX: &str = r"^(.*?)(?::(\d*))?$";

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
    pub fn to_context(&self) -> Result<Context, Box<dyn Error>> {
        let re = Regex::new(PATH_REGEX)?;
        let caps = re.captures(&self.path).ok_or("Invalid path")?;
        let path = caps.get(1).ok_or("Invalid path")?.as_str();
        let line = caps
            .get(2)
            .map(|m| m.as_str().parse::<Line>())
            .transpose()
            .unwrap_or(None);

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

#[cfg(test)]
mod tests {
    use std::{
        env,
        fs::{self, File},
        path::PathBuf,
    };

    use super::*;

    fn init(folder_name: &str, file_name: &str) -> (PathBuf, PathBuf) {
        let dir = env::temp_dir();
        let folder = dir.join(folder_name);

        if let Err(error) = fs::create_dir(&folder) {
            if error.kind() != std::io::ErrorKind::AlreadyExists {
                panic!("Cannot create folder");
            }
        };

        let file = folder.join(file_name);

        File::create(&file).unwrap();

        (folder, file)
    }

    fn build_args(root: &PathBuf, path: &str) -> Args {
        Args {
            path: path.to_string(),
            scope: None,
            root: Some(root.to_str().unwrap().to_string()),
            config: None,
            dry_run: false,
        }
    }

    #[test]
    fn test_args_to_context() {
        let (folder, file) = init("folder", "test.rs");

        let args = build_args(&folder, "test.rs:123");
        let context = args.to_context().unwrap();

        assert_eq!(context.root(), &folder);
        assert_eq!(context.path(), &file);
        assert_eq!(context.rel_path(), &PathBuf::from("test.rs"));
        assert_eq!(context.line(), Some(123));

        let args = build_args(&folder, "test.rs:");
        let context = args.to_context().unwrap();

        assert_eq!(context.root(), &folder);
        assert_eq!(context.path(), &file);
        assert_eq!(context.rel_path(), &PathBuf::from("test.rs"));
        assert_eq!(context.line(), None);

        let args = build_args(&folder, "test.rs");
        let context = args.to_context().unwrap();

        assert_eq!(context.root(), &folder);
        assert_eq!(context.path(), &file);
        assert_eq!(context.rel_path(), &PathBuf::from("test.rs"));
        assert_eq!(context.line(), None);
    }

    #[test]
    fn test_args_scope() {
        let (folder, _) = init("folder", "test.rs");

        let mut args = build_args(&folder, "test.rs:123");
        let context = args.to_context().unwrap();

        assert!(matches!(args.scope(&context), Scope::Line));

        args.scope = Some(Scope::Suite);
        assert!(matches!(args.scope(&context), Scope::Suite));

        let args = build_args(&folder, "test.rs");
        let context = args.to_context().unwrap();

        assert!(matches!(args.scope(&context), Scope::File));
    }

    #[test]
    fn test_format_command() {
        let mut command = Command::new("echo");
        command.arg("Hello,").arg("World!");

        assert_eq!(format_command(&command), "echo Hello, World!");
    }
}
