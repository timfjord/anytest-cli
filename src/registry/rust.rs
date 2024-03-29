use super::{EnvHashMap, Language, LanguageMeta};

// Test frameworks
pub use cargotest::CargoTest;

mod cargotest;

#[derive(LanguageMeta)]
struct Rust {
    name: String,
    env: EnvHashMap,
}

impl Language for Rust {}

impl Default for Rust {
    fn default() -> Self {
        Self {
            name: "rust".into(),
            env: Default::default(),
        }
    }
}
