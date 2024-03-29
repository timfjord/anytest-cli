use super::{EnvHashMap, Language, LanguageMeta};

// Test frameworks
pub use cargotest::Cargotest;

mod cargotest;

#[derive(LanguageMeta)]
struct Rust {
    env: EnvHashMap,
}

impl Language for Rust {}

impl Default for Rust {
    fn default() -> Self {
        Self {
            env: Default::default(),
        }
    }
}
