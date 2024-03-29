use crate::registry::{EnvHashMap, Language, TestFramework, TestFrameworkMeta};
use regex::{Error, Regex};

use super::Rust;

#[derive(TestFrameworkMeta)]
pub struct Cargotest {
    language: Rust,
    pattern: String,
    env: EnvHashMap,
}

impl TestFramework for Cargotest {
    fn executable(&self) -> String {
        "cargo test".into()
    }
}

impl Default for Cargotest {
    fn default() -> Self {
        Self {
            language: Rust::default(),
            pattern: r".rs$".into(),
            env: Default::default(),
        }
    }
}
