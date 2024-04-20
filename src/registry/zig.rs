use crate::language::Language;

// Test frameworks
pub use zigtest::Zigtest;

mod zigtest;

#[derive(Language, Default)]
struct Zig {}
