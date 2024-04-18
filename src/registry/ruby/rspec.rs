use super::Ruby;
use crate::{
    test_framework::{TestFramework, TestFrameworkMeta},
    utils, ArgsList, Context,
};
use smart_default::SmartDefault;

#[derive(TestFrameworkMeta, SmartDefault)]
pub struct RSpec {
    language: Ruby,
    #[default = r"(_spec\.rb|spec[/\\].*\.feature)$"]
    pattern: String,
    executable: Vec<&'static str>,
    test_pattern: String,
    namespace_pattern: String,
}

impl TestFramework for RSpec {
    fn build_executable(&self, context: &Context) -> ArgsList {
        let executable: ArgsList = vec!["rspec".into()];

        if context.find_file(".zeus.sock").is_some() {
            utils::concat(["zeus"], executable)
        } else if let Some(spring_bin) = context.find_file("bin/spring") {
            utils::concat([spring_bin.rel_str()], executable)
        } else if let Some(bin) = context.find_file("bin/rspec") {
            utils::concat([bin.rel_str()], executable)
        } else if context.find_file("Gemfile").is_some() {
            utils::concat(["bundle", "exec"], executable)
        } else {
            executable
        }
    }
}
