use super::JavaScript;
use crate::{
    test_framework::{TestFramework, TestFrameworkMeta},
    utils::{self, concat},
};
use smart_default::SmartDefault;

#[derive(TestFrameworkMeta, SmartDefault)]
pub struct Jest {
    language: JavaScript,
    #[default = r"(__tests__[/\\].*|(spec|test))\.(js|jsx|coffee|ts|tsx)$"]
    pattern: String,
    executable: Vec<&'static str>,
    #[default(_code = r#"vec!["--runTestsByPath"]"#)]
    args: Vec<&'static str>,
    #[default = r#"^\s*(?:it|test)\s*[\( ]\s*["'\`](.*?)["'\`]"#]
    test_pattern: String,
    #[default = r#"^\s*(?:describe|suite|context)\s*[( ]\s*["'\`](.*?)["'\`]"#]
    namespace_pattern: String,
}

// TODO: after adding configuration handle the case when the executable contains `yarn`
// see https://github.com/timfjord/AnyTest/blob/main/plugin/test_frameworks/javascript/jest.py#L34-L40
impl TestFramework for Jest {
    fn build_executable(&self, context: &crate::Context) -> crate::ArgsList {
        if let Some(bin) = context.find_file("node_modules/.bin/jest") {
            vec![bin.rel_str().to_string()]
        } else {
            vec!["jest".to_string()]
        }
    }

    fn build_file_position_args(
        &self,
        context: &crate::Context,
    ) -> Result<crate::ArgsList, Box<dyn std::error::Error>> {
        Ok(vec![utils::EOO.into(), context.rel_str().to_string()])
    }

    fn build_line_position_args(
        &self,
        context: &crate::Context,
    ) -> Result<crate::ArgsList, Box<dyn std::error::Error>> {
        let args = self.build_file_position_args(context)?;
        let nearest = self.find_nearest(context)?;
        let name = format!(
            "{}{}{}",
            if !nearest.namespaces().is_empty() {
                "^"
            } else {
                ""
            },
            [nearest.namespaces(), nearest.tests()].concat().join(" "),
            if !nearest.tests().is_empty() { "$" } else { "" }
        );

        if name.is_empty() {
            Ok(args)
        } else {
            Ok(concat(["-t", &name], args))
        }
    }
}
