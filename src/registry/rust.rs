use super::{EnvHashMap, Language, LanguageMeta};

// Test frameworks
pub use cargotest::CargoTest;

mod cargotest;

// derive[LanguageMeta]
struct Rust {
    // name
    name: String,
    // env
    env: EnvHashMap,
}

impl LanguageMeta for Rust {
    fn name(&self) -> &str {
        &self.name
    }

    fn env(&self) -> &EnvHashMap {
        &self.env
    }
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
