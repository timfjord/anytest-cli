use anytest::{Context, LineNr, Scope};
use clap::Parser;
use regex::Regex;
use std::error::Error;

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
        let line_nr = caps
            .get(2)
            .map(|m| m.as_str().parse::<LineNr>())
            .transpose()
            .unwrap_or(None);

        Context::new(self.root.as_deref(), path, line_nr, self.scope.clone())
    }

    pub fn is_dry_run(&self) -> bool {
        self.dry_run
    }
}

#[cfg(test)]
mod tests {
    use std::{
        env,
        fs::{self, File},
        path::PathBuf
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
        assert_eq!(context.rel(), &PathBuf::from("test.rs"));
        assert_eq!(context.line_nr(), Some(123));
        assert!(matches!(context.scope(), &Scope::Line));

        let args = build_args(&folder, "test.rs:");
        let context = args.to_context().unwrap();

        assert_eq!(context.root(), &folder);
        assert_eq!(context.path(), &file);
        assert_eq!(context.rel(), &PathBuf::from("test.rs"));
        assert_eq!(context.line_nr(), None);
        assert!(matches!(context.scope(), &Scope::File));

        let args = build_args(&folder, "test.rs");
        let context = args.to_context().unwrap();

        assert_eq!(context.root(), &folder);
        assert_eq!(context.path(), &file);
        assert_eq!(context.rel(), &PathBuf::from("test.rs"));
        assert_eq!(context.line_nr(), None);
        assert!(matches!(context.scope(), &Scope::File));
    }
}
