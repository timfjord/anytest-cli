use crate::registry::{ArgsVec, EnvHashMap, TestFramework, TestFrameworkMeta};

use super::Rust;

#[derive(TestFrameworkMeta)]
pub struct Cargotest {
    language: Rust,
    pattern: String,
    program: String,
    args: ArgsVec,
    env: EnvHashMap,
}

impl Default for Cargotest {
    fn default() -> Self {
        Self {
            language: Rust::default(),
            pattern: r".rs$".into(),
            program: "cargo test".into(),
            args: ArgsVec::default(),
            env: Default::default(),
        }
    }
}

impl TestFramework for Cargotest {}
