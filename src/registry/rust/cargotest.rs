use crate::registry::{EnvHashMap, TestFramework, TestFrameworkMeta};

use super::Rust;

#[derive(TestFrameworkMeta)]
pub struct Cargotest {
    language: Rust,
    pattern: String,
    env: EnvHashMap,
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

impl TestFramework for Cargotest {
    fn executable(&self) -> String {
        "cargo test".into()
    }
}
