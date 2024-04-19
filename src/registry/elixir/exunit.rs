use std::error::Error;

use super::Elixir;
use crate::{
    test_framework::{TestFramework, TestFrameworkMeta},
    ArgsList, Context,
};
use smart_default::SmartDefault;

#[derive(TestFrameworkMeta, SmartDefault)]
pub struct ExUnit {
    language: Elixir,
    #[default = r"_test\.exs$"]
    pattern: String,
    executable: Vec<&'static str>,
    test_pattern: String,
    namespace_pattern: String,
}

// TODO: cache this method
fn is_mix(context: &Context) -> bool {
    context.find_file("mix.exs").is_some()
}

impl TestFramework for ExUnit {
    fn build_executable(&self, context: &Context) -> ArgsList {
        if is_mix(context) {
            vec!["mix".into(), "test".into()]
        } else {
            vec!["elixir".into()]
        }
    }

    fn build_suite_position_args(&self, context: &Context) -> Result<ArgsList, Box<dyn Error>> {
        if is_mix(context) {
            Ok(vec![])
        } else {
            Ok(vec!["*.exs".into()])
        }
    }

    fn build_line_position_args(&self, context: &Context) -> Result<ArgsList, Box<dyn Error>> {
        let file_args = self.build_file_position_args(context)?;

        if !is_mix(context) || context.line_nr_or_default() < 2 {
            return Ok(file_args);
        }

        Ok(vec![context.rel_full()])
    }
}
