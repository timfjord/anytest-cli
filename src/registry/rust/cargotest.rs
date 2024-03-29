use crate::registry::{EnvHashMap, Language, TestFramework, TestFrameworkMeta};
use regex::{Error, Regex};

use super::Rust;

#[derive(TestFrameworkMeta)]
pub struct CargoTest {
    language: Rust,
    name: String,
    pattern: String,
    env: EnvHashMap,
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
