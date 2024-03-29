use super::{Language, LanguageData};

// Test frameworks
pub use cargotest::CargoTest;

mod cargotest;

struct Rust {
    data: LanguageData,
}

impl Default for Rust {
    fn default() -> Self {
        Self {
            data: LanguageData {
                name: "rust".into(),
                env: Default::default(),
            },
        }
    }
}

impl Language for Rust {
    fn data(&self) -> &LanguageData {
        &self.data
    }
}
