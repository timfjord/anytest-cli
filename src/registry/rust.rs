use super::{EnvHashMap, Language};

// Test frameworks
pub use cargotest::Cargotest;

mod cargotest;

#[derive(Language, Default)]
struct Rust {
    env: EnvHashMap,
}
