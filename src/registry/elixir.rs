use crate::language::Language;

// Test frameworks
pub use espec::ESpec;
pub use exunit::ExUnit;

mod espec;
mod exunit;

#[derive(Language, Default)]
struct Elixir {}
