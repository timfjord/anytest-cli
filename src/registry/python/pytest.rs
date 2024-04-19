use std::error::Error;

use super::Python;
use crate::{
    test_framework::{TestFramework, TestFrameworkMeta},
    utils, ArgsList, Context,
};
use smart_default::SmartDefault;

const SEPARATOR: &str = "::";

#[derive(TestFrameworkMeta, SmartDefault)]
pub struct Pytest {
    language: Python,
    #[default = r"(test_[^/\\]+|[^/\\]+_test)\.py$"]
    pattern: String,
    executable: Vec<&'static str>,
    #[default = r"\s*(?:async )?def (test_\w+)"]
    test_pattern: String,
    #[default = r"\s*class (\w+)"]
    namespace_pattern: String,
}

impl TestFramework for Pytest {
    fn build_executable(&self, context: &Context) -> ArgsList {
        let executable: ArgsList = if utils::is_executable("pytest") {
            vec!["pytest".into()]
        } else if utils::is_executable("py.test") {
            vec!["py.test".into()]
        } else {
            vec!["python".into(), "-m".into(), "pytest".into()]
        };

        if context.find_file("Pipfile").is_some() {
            utils::concat(["pipenv", "run"], executable)
        } else if context.find_file("poetry.lock").is_some() {
            utils::concat(["poetry", "run"], executable)
        } else if context.find_file("pdm.lock").is_some() {
            utils::concat(["pdm", "run"], executable)
        } else {
            executable
        }
    }

    fn build_line_position_args(&self, context: &Context) -> Result<ArgsList, Box<dyn Error>> {
        let file_args = self.build_file_position_args(context)?;
        let nearest = self.find_nearest(context)?;

        let arg = [&file_args, nearest.namespaces(), nearest.tests()]
            .concat()
            .join(SEPARATOR);
        Ok(vec![arg])
    }
}
