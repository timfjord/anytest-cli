use anytest::{self, Context, LineNr, Scope};
use std::{env, path::PathBuf};

pub struct Project {
    root: PathBuf,
}

impl Project {
    pub fn new(name: &str) -> Self {
        let root = env::current_dir()
            .unwrap()
            .join("tests/fixtures")
            .join(name);

        Self { root }
    }

    fn test(&self, file: &str, line: Option<LineNr>, scope: Scope) -> String {
        let context =
            Context::new(Some(self.root.to_str().unwrap()), file, line, Some(scope)).unwrap();
        let command = anytest::build_command(&context).unwrap();

        anytest::format_command(&command)
    }

    pub fn test_line(&self, file: &str, line: anytest::LineNr) -> String {
        self.test(file, Some(line), Scope::Line)
    }

    pub fn test_file(&self, file: &str) -> String {
        self.test(file, None, Scope::File)
    }

    pub fn test_suite(&self, file: &str) -> String {
        self.test(file, None, Scope::Suite)
    }
}
