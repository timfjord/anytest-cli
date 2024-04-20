use super::Zig;
use crate::{
    test_framework::{TestFramework, TestFrameworkMeta},
    utils, ArgsList, Context,
};
use smart_default::SmartDefault;

#[derive(TestFrameworkMeta, SmartDefault)]
pub struct Zigtest {
    language: Zig,
    #[default = r".zig$"]
    pattern: String,
    #[default(_code = r#"vec!["zig"]"#)]
    executable: Vec<&'static str>,
    args: Vec<&'static str>,
    #[default = r#"^\s*test\s+"(.+)""#]
    test_pattern: String,
    namespace_pattern: String,
}

impl TestFramework for Zigtest {
    fn build_suite_position_args(
        &self,
        _context: &Context,
    ) -> Result<ArgsList, Box<dyn std::error::Error>> {
        Ok(vec!["build".into(), "test".into()])
    }

    fn build_file_position_args(
        &self,
        context: &Context,
    ) -> Result<ArgsList, Box<dyn std::error::Error>> {
        Ok(vec!["test".into(), context.rel_str().into()])
    }

    fn build_line_position_args(
        &self,
        context: &Context,
    ) -> Result<ArgsList, Box<dyn std::error::Error>> {
        let args = self.build_file_position_args(context)?;
        let nearest = self.find_nearest(context)?;

        if !nearest.has_tests() {
            return Ok(args);
        };

        let filter = nearest.tests().join("");
        let line_args = utils::concat(["--test-filter"], [utils::quote(&filter)]);

        Ok(utils::concat(args, line_args))
    }
}
