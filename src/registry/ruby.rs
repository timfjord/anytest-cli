use crate::language::Language;

// Test frameworks
pub use rspec::RSpec;

mod rspec;

#[derive(Language, Default)]
struct Ruby {}
