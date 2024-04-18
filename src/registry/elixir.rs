use crate::language::Language;

// Test frameworks
pub use exunit::ExUnit;
pub use espec::ESpec;

mod exunit;
mod espec;

#[derive(Language, Default)]
struct Elixir {}
