use crate::{language::Language, EnvVars};

// Test frameworks
pub use cargotest::Cargotest;

mod cargotest;

const MANIFEST_FILE: &str = "Cargo.toml";

#[derive(Language, Default)]
struct Rust {
    env: EnvVars,
}
