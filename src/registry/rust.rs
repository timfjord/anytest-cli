use crate::{language::Language, EnvHashMap};

// Test frameworks
pub use cargotest::Cargotest;

mod cargotest;

#[derive(Language, Default)]
struct Rust {
    env: EnvHashMap,
}
