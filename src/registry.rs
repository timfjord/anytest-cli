use crate::{Context, Scope};
use regex::{Error, Regex};
use std::collections::HashMap;

mod rust;

type EnvHashMap = HashMap<String, String>;

pub trait LanguageMeta {
    fn name(&self) -> &str;

    fn env(&self) -> &EnvHashMap;
}

pub trait Language: LanguageMeta {}

pub trait TestFrameworkMeta {
    fn language(&self) -> Box<&dyn Language>;

    fn language_name(&self) -> &str {
        self.language().name()
    }

    fn name(&self) -> &str;

    fn pattern(&self) -> Result<Regex, Error>;

    fn env(&self) -> &EnvHashMap;
}

pub trait TestFramework: TestFrameworkMeta {
    fn is_suitable_for(&self, context: Context) -> bool {
        if let Ok(pattern) = self.pattern() {
            pattern.is_match(context.path().to_str().unwrap_or_default())
        } else {
            false
        }
    }

    fn executable(&self) -> String;

    fn suite_position_args(&self, _context: &Context) -> Vec<String> {
        vec![]
    }

    fn file_position_args(&self, context: &Context) -> Vec<String> {
        vec![context.path_str().into()]
    }

    fn line_position_args(&self, context: &Context) -> Vec<String> {
        let path_with_line = format!("{}:{}", context.path_str(), context.line().unwrap_or(1));

        vec![path_with_line]
    }

    fn position_args(&self, scope: &Scope, context: &Context) -> Vec<String> {
        match scope {
            Scope::Suite => self.suite_position_args(context),
            Scope::File => self.file_position_args(context),
            Scope::Line => self.line_position_args(context),
        }
    }
}

pub struct Registry {
    frameworks: Vec<Box<dyn TestFramework>>,
}

impl Registry {
    pub fn new() -> Self {
        let mut registry = Self { frameworks: vec![] };

        registry.add(Box::new(rust::CargoTest::default()));

        registry
    }

    fn add(&mut self, framework: Box<dyn TestFramework>) {
        self.frameworks.push(framework);
    }
}

impl IntoIterator for Registry {
    type Item = Box<dyn TestFramework>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.frameworks.into_iter()
    }
}
