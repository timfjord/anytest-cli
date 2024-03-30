use crate::{Context, Scope};
use regex::{Error, Regex};
use std::collections::HashMap;

mod rust;

type ArgsVec = Vec<String>;
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

    fn default_program(&self) -> &str;

    fn args(&self) -> &ArgsVec;

    fn env(&self) -> &EnvHashMap;
}

pub trait TestFramework: TestFrameworkMeta {
    fn is_suitable_for(&self, context: &Context) -> bool {
        match self.pattern() {
            Ok(pattern) => pattern.is_match(context.path().to_str().unwrap_or_default()),
            Err(error) => {
                log::warn!("{}", error);
                false
            }
        }
    }

    fn program(&self) -> &str {
        self.default_program()
    }

    fn suite_position_args(&self, _context: &Context) -> ArgsVec {
        vec![]
    }

    fn file_position_args(&self, context: &Context) -> ArgsVec {
        vec![context.path_str().into()]
    }

    fn line_position_args(&self, context: &Context) -> ArgsVec {
        let path_with_line = format!("{}:{}", context.path_str(), context.line().unwrap_or(1));

        vec![path_with_line]
    }

    fn position_args(&self, scope: &Scope, context: &Context) -> ArgsVec {
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

        registry.add(Box::new(rust::Cargotest::default()));

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
