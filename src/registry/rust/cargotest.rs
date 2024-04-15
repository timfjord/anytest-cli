use crate::{
    test_framework::{TestFramework, TestFrameworkMeta},
    ArgsVec, EnvHashMap,
};
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
    #[default = r"(#\[(?:\w+::|rs)?test)"]
    test_pattern: String,
    #[default = r"mod (tests?)"]
    namespace_pattern: String,
}

impl TestFramework for Cargotest {}
