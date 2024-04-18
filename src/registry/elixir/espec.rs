use super::Elixir;
use crate::test_framework::{TestFramework, TestFrameworkMeta};
use smart_default::SmartDefault;

#[derive(TestFrameworkMeta, SmartDefault)]
pub struct ESpec {
    language: Elixir,
    #[default = r"_spec\.exs$"]
    pattern: String,
    #[default(_code = r#"vec!["mix", "espec"]"#)]
    executable: Vec<&'static str>,
    test_pattern: String,
    namespace_pattern: String,
}

impl TestFramework for ESpec {}
