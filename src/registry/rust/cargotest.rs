use crate::registry::{EnvHashMap, Language, TestFramework, TestFrameworkMeta};
use regex::{Error, Regex};

use super::Rust;

// derive[TestFrameworkMeta]
pub struct CargoTest {
    // language,
    language: Rust,
    // name
    name: String,
    // pattern
    pattern: String,
    // env
    env: EnvHashMap,
}

impl TestFrameworkMeta for CargoTest {
    fn language(&self) -> Box<&dyn Language> {
        Box::new(&self.language)
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn pattern(&self) -> Result<Regex, Error> {
        Regex::new(&self.pattern)
    }

    fn env(&self) -> &EnvHashMap {
        &self.env
    }
}

impl TestFramework for CargoTest {
    fn executable(&self) -> String {
        "cargo test".into()
    }
}

impl Default for CargoTest {
    fn default() -> Self {
        Self {
            language: Rust::default(),
            name: "cargotest".into(),
            pattern: r".rs$".into(),
            env: Default::default(),
        }
    }
}
