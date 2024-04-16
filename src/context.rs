use crate::{named_pattern::NamedPattern, LineNr, RelPath};
use clap::ValueEnum;
use std::{error::Error, ops, path::PathBuf};

#[derive(ValueEnum, Clone, Debug)]
pub enum Scope {
    Suite,
    File,
    Line,
}

pub struct Nearest {
    tests: Vec<String>,
    namespaces: Vec<String>,
    line_nr: Option<LineNr>,
    // names: Vec<String>,
}

impl Nearest {
    pub fn tests(&self) -> &[String] {
        &self.tests
    }

    pub fn namespaces(&self) -> &[String] {
        &self.namespaces
    }

    pub fn line_nr(&self) -> Option<LineNr> {
        self.line_nr
    }

    pub fn has_tests(&self) -> bool {
        !self.tests.is_empty()
    }
}

#[derive(Debug)]
pub struct Context {
    rel_path: RelPath,
    line_nr: Option<LineNr>,
    scope: Scope,
}

impl Context {
    pub fn new(
        root: Option<&str>,
        path: &str,
        line_nr: Option<LineNr>,
        scope: Option<Scope>,
    ) -> Result<Self, Box<dyn Error>> {
        let rel_path = RelPath::new(root, path)?;
        let scope = if let Some(scope) = scope {
            scope
        } else if let Some(_) = line_nr {
            Scope::Line
        } else {
            Scope::File
        };

        Ok(Self {
            rel_path,
            line_nr,
            scope,
        })
    }

    pub fn root(&self) -> &PathBuf {
        &self.rel_path.root()
    }

    pub fn path(&self) -> &PathBuf {
        &self.rel_path.path()
    }

    pub fn rel(&self) -> &PathBuf {
        &self.rel_path.rel()
    }

    pub fn rel_str(&self) -> &str {
        &self.rel().to_str().unwrap_or_default()
    }

    pub fn line_nr(&self) -> Option<LineNr> {
        self.line_nr
    }

    pub fn scope(&self) -> &Scope {
        &self.scope
    }

    pub fn find_nearest(
        &self,
        test_patterns: &[NamedPattern],
        namespace_patters: &[NamedPattern],
        range: impl ops::RangeBounds<LineNr>,
    ) -> Result<Nearest, Box<dyn Error>> {
        if test_patterns.is_empty() {
            return Err("Test patterns are empty".into());
        }

        let mut tests: Vec<String> = Vec::new();
        let mut namespaces: Vec<String> = Vec::new();
        // let names: Vec<String> = Vec::new();
        let mut test_line_nr: Option<LineNr> = None;
        let mut last_namespace_line_nr: Option<LineNr> = None;
        let mut last_indent: Option<LineNr> = None;

        for (line, number) in self.rel_path.lines(range)? {
            let test_match = test_patterns.iter().find_map(|pattern| pattern.find(&line));
            let namespace_match = namespace_patters
                .iter()
                .find_map(|pattern| pattern.find(&line));
            let indent = line.chars().take_while(|c| c.is_whitespace()).count();

            if let Some((test_match, _)) = test_match {
                if last_indent.is_none()
                    || (test_line_nr.is_none()
                        && last_indent.unwrap() > indent
                        && last_namespace_line_nr.is_some()
                        && last_namespace_line_nr.unwrap() > number)
                {
                    if let Some(namespace_line_nr) = last_namespace_line_nr {
                        if namespace_line_nr > number {
                            namespaces.clear();
                            last_namespace_line_nr = None;
                        }
                    }
                    tests.push(test_match);
                    // if let Some(test_name) = test_name {
                    //     names.push(test_name);
                    // }
                    last_indent = Some(indent);
                    test_line_nr = Some(number);
                }
            } else if let Some((namespace_match, _)) = namespace_match {
                if last_indent.is_none() || indent < last_indent.unwrap() {
                    namespaces.push(namespace_match);
                    last_indent = Some(indent);
                    last_namespace_line_nr = Some(number);
                }
            }
        }

        Ok(Nearest {
            tests,
            namespaces,
            line_nr: test_line_nr,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        env,
        fs::{self, File},
        io::Write,
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

    fn get_scope(
        root: &PathBuf,
        path: &PathBuf,
        line: Option<LineNr>,
        scope: Option<Scope>,
    ) -> Scope {
        let context = Context::new(
            Some(root.to_str().unwrap()),
            path.to_str().unwrap(),
            line,
            scope,
        )
        .unwrap();
        context.scope().clone()
    }

    #[test]
    fn test_context_new() {
        let dir = env::temp_dir();
        let folder = create_folder(&dir, "folder").unwrap();
        let file = folder.join("file.rb");

        File::create(&file).unwrap();

        assert!(matches!(
            get_scope(&folder, &file, Some(123), None),
            Scope::Line
        ));
        assert!(matches!(
            get_scope(&folder, &file, Some(123), Some(Scope::Suite)),
            Scope::Suite
        ));
        assert!(matches!(get_scope(&folder, &file, None, None), Scope::File));
    }

    fn find_nearest(
        root: &str,
        path: &str,
        test_patterns: &[NamedPattern],
        namespace_patters: &[NamedPattern],
        range: impl ops::RangeBounds<LineNr>,
    ) -> Nearest {
        Context::new(Some(root), path, None, None)
            .unwrap()
            .find_nearest(test_patterns, namespace_patters, range)
            .unwrap()
    }

    #[test]
    fn test_content_find_nearest() {
        let dir = env::temp_dir();
        let folder = create_folder(&dir, "folder").unwrap();
        let file = folder.join("file.rb");

        let mut f = File::create(&file).unwrap();
        f.write_all("class TestClass\n  def test_method do\n  end\nend\n".as_bytes())
            .unwrap();

        let nearest = find_nearest(
            dir.to_str().unwrap(),
            "folder/file.rb",
            &[r"^\s*def\s+(test_\w+)".into()],
            &[r"^\s*(?:class|module)\s+(\S+)".into()],
            2..=1,
        );

        assert_eq!(nearest.tests(), vec!["test_method".to_string()]);
        assert_eq!(nearest.namespaces(), vec!["TestClass".to_string()]);
        assert_eq!(nearest.line_nr(), Some(2));
    }
}
