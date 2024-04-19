use crate::language::Language;

// Test frameworks
pub use pytest::Pytest;

mod pytest;

#[derive(Language, Default)]
struct Python {}
