use crate::{Context, Scope};
use regex::{Error, Regex};
use std::collections::HashMap;

mod rust;

type EnvHashMap = HashMap<String, String>;

pub struct LanguageData {
    name: String,
    env: EnvHashMap,
}

impl LanguageData {
    fn name(&self) -> &str {
        &self.name
    }

    fn env(&self) -> &EnvHashMap {
        &self.env
    }
}

pub trait Language {
    fn data(&self) -> &LanguageData;

    fn name(&self) -> &str {
        self.data().name()
    }

    fn env(&self) -> &EnvHashMap {
        self.data().env()
    }
}

pub struct FrameworkData {
    name: String,
    env: EnvHashMap,
    pattern: String,
}

impl FrameworkData {
    fn name(&self) -> &str {
        &self.name
    }

    fn env(&self) -> &EnvHashMap {
        &self.env
    }

    fn pattern(&self) -> Result<Regex, Error> {
        Regex::new(&self.pattern)
    }
}

pub trait Framework {
    fn language_data(&self) -> Box<&dyn Language>;

    fn framework_data(&self) -> &FrameworkData;

    fn language(&self) -> &str {
        self.language_data().name()
    }

    fn framework(&self) -> &str {
        self.framework_data().name()
    }

    fn env(&self) -> EnvHashMap {
        let mut env = self.language_data().env().clone();
        env.extend(self.framework_data().env().clone());
        env
    }

    fn is_suitable_for(&self, context: Context) -> bool {
        if let Ok(pattern) = self.framework_data().pattern() {
            pattern.is_match(context.path().to_str().unwrap_or_default())
        } else {
            false
        }
    }

    fn executable(&self) -> String;

    fn suite_position_args(&self) -> Vec<String>;

    fn file_position_args(&self) -> Vec<String>;

    fn line_position_args(&self) -> Vec<String>;

    fn position_args(&self, scope: Scope) -> Vec<String> {
        match scope {
            Scope::Suite => self.suite_position_args(),
            Scope::File => self.file_position_args(),
            Scope::Line => self.line_position_args(),
        }
    }
}

pub struct Registry {
    frameworks: Vec<Box<dyn Framework>>,
}

impl Registry {
    pub fn new() -> Self {
        let mut registry = Self { frameworks: vec![] };

        registry.add(Box::new(rust::CargoTest::default()));

        registry
    }

    fn add(&mut self, framework: Box<dyn Framework>) {
        self.frameworks.push(framework);
    }
}

impl IntoIterator for Registry {
    type Item = Box<dyn Framework>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.frameworks.into_iter()
    }
}
