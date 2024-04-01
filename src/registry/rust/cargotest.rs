use crate::registry::{ArgsVec, EnvHashMap, TestFramework, TestFrameworkMeta};
use smart_default::SmartDefault;

use super::Rust;

#[derive(TestFrameworkMeta, SmartDefault)]
pub struct Cargotest {
    language: Rust,
    #[default = r".rs$"]
    pattern: String,
    #[default = "cargo test"]
    program: String,
    args: ArgsVec,
    env: EnvHashMap,
}

impl TestFramework for Cargotest {}
