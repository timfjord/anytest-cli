use crate::registry::{EnvHashMap, TestFramework, TestFrameworkMeta};

use super::Rust;

#[derive(TestFrameworkMeta)]
pub struct Cargotest {
    language: Rust,
    pattern: String,
    program: String,
    env: EnvHashMap,
}

impl Default for Cargotest {
    fn default() -> Self {
        Self {
            language: Rust::default(),
            pattern: r".rs$".into(),
            program: "cargo test".into(),
            env: Default::default(),
        }
    }
}

impl TestFramework for Cargotest {}
