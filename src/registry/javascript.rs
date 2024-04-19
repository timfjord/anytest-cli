use crate::language::Language;

// Test frameworks
pub use jest::Jest;

mod jest;

#[derive(Language, Default)]
struct JavaScript {}
