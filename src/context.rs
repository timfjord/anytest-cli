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
        } else if line_nr.is_some() {
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
        self.rel_path.root()
    }

    pub fn path(&self) -> &PathBuf {
        self.rel_path.path()
    }

    pub fn rel(&self) -> &PathBuf {
        self.rel_path.rel()
    }

    pub fn rel_str(&self) -> &str {
        self.rel_path.rel_str()
    }

    pub fn line_nr(&self) -> Option<LineNr> {
        self.line_nr
    }

    pub fn line_nr_or_default(&self) -> LineNr {
        self.line_nr.unwrap_or(1)
    }

    pub fn rel_full(&self) -> String {
        format!("{}:{}", self.rel_str(), self.line_nr_or_default())
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

        namespaces.reverse();
        Ok(Nearest {
            tests,
            namespaces,
            line_nr: test_line_nr,
        })
    }

    pub fn find_file(&self, rel_path: &str) -> Option<RelPath> {
        if let Ok(rel_path) = self.rel_path.file(rel_path) {
            Some(rel_path)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_scope(line: Option<LineNr>, scope: Option<Scope>) -> Scope {
        let context = Context::new(Some("tests/fixtures/folder"), "file.txt", line, scope).unwrap();
        context.scope().clone()
    }

    #[test]
    fn test_context_new() {
        assert!(matches!(get_scope(Some(123), None), Scope::Line));
        assert!(matches!(
            get_scope(Some(123), Some(Scope::Suite)),
            Scope::Suite
        ));
        assert!(matches!(get_scope(None, None), Scope::File));
    }

    fn find_nearest(
        test_patterns: &[NamedPattern],
        namespace_patters: &[NamedPattern],
        range: impl ops::RangeBounds<LineNr>,
    ) -> Nearest {
        Context::new(Some("tests/fixtures/folder"), "file.rb", None, None)
            .unwrap()
            .find_nearest(test_patterns, namespace_patters, range)
            .unwrap()
    }

    #[test]
    fn test_content_find_nearest() {
        let nearest = find_nearest(
            &[r"^\s*def\s+(test_\w+)".into()],
            &[r"^\s*(?:class|module)\s+(\S+)".into()],
            2..=1,
        );

        assert_eq!(nearest.tests(), vec!["test_method".to_string()]);
        assert_eq!(nearest.namespaces(), vec!["TestClass".to_string()]);
        assert_eq!(nearest.line_nr(), Some(2));
    }
}
