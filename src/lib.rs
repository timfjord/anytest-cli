use clap::ValueEnum;
use std::error::Error;
use std::path::PathBuf;
use std::process::Command;

pub type Line = usize;

#[derive(Debug)]
pub struct RelPath {
    root: PathBuf,
    path: PathBuf,
    rel_path: PathBuf,
}

impl RelPath {
    pub fn new(root: Option<&str>, path: &str) -> Result<Self, Box<dyn Error>> {
        let root = if let Some(root) = root {
            PathBuf::from(root)
        } else {
            std::env::current_dir()?
        };

        if !root.is_absolute() {
            return Err("Root path must be absolute".into());
        }

        if !root.is_dir() {
            return Err("Root path must be an existing directory".into());
        }

        let mut path = PathBuf::from(path);

        if !path.is_absolute() {
            path = root.join(path);
        }

        if !path.exists() {
            return Err("Path does not exist".into());
        }

        let rel_path = path
            .clone()
            .strip_prefix(&root)
            .map_err(|_| "Path must be a subpath of the root path")?
            .to_path_buf();

        Ok(Self {
            root,
            path,
            rel_path,
        })
    }

    pub fn root(&self) -> &PathBuf {
        &self.root
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn rel_path(&self) -> &PathBuf {
        &self.rel_path
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

    pub fn rel_path(&self) -> &PathBuf {
        &self.rel_path.rel_path()
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
        Scope::File => command.args(["file", context.rel_path().to_str().unwrap()]),
        Scope::Line => command.args([
            "line",
            context.rel_path().to_str().unwrap(),
            context.line().unwrap_or(1).to_string().as_str(),
        ]),
    };

    Ok(command)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        env,
        fs::{self, File},
    };

    fn create_folder(base: &PathBuf, path: &str) -> Result<PathBuf, Box<dyn Error>> {
        let folder = base.join(path);

        match fs::create_dir(&folder) {
            Ok(_) => Ok(folder),
            Err(error) => match error.kind() {
                std::io::ErrorKind::AlreadyExists => Ok(folder),
                _ => Err(error.into()),
            },
        }
    }

    fn rel_path_error(root: &str, path: &str) -> String {
        RelPath::new(Some(root), path).unwrap_err().to_string()
    }

    #[test]
    fn test_rel_path_new() {
        let dir = env::temp_dir();
        let folder = create_folder(&dir, "folder").unwrap();
        let file = folder.join("file.rs");
        let other_file = dir.join("other_file.rs");

        File::create(&file).unwrap();
        File::create(&other_file).unwrap();

        assert_eq!(
            rel_path_error("some_folder", ""),
            "Root path must be absolute"
        );

        assert_eq!(
            rel_path_error(file.to_str().unwrap(), ""),
            "Root path must be an existing directory"
        );

        assert_eq!(
            rel_path_error("/tmp/some_folder", ""),
            "Root path must be an existing directory"
        );

        assert_eq!(
            rel_path_error(folder.to_str().unwrap(), "non_existent.rs"),
            "Path does not exist"
        );

        assert_eq!(
            rel_path_error(folder.to_str().unwrap(), other_file.to_str().unwrap()),
            "Path must be a subpath of the root path"
        );

        let rel_path =
            RelPath::new(Some(folder.to_str().unwrap()), file.to_str().unwrap()).unwrap();

        assert_eq!(*rel_path.root(), folder);
        assert_eq!(*rel_path.path(), file);
        assert_eq!(*rel_path.rel_path(), PathBuf::from("file.rs"));
    }
}
