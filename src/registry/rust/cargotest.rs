use crate::{
    test_framework::{TestFramework, TestFrameworkMeta},
    ArgsList, EnvVars,
};
use regex::Regex;
use smart_default::SmartDefault;

use super::Rust;

#[derive(TestFrameworkMeta, SmartDefault)]
pub struct Cargotest {
    language: Rust,
    #[default = r".rs$"]
    pattern: String,
    #[default = "cargo test"]
    program: String,
    args: ArgsList,
    env: EnvVars,
    #[default = r"(#\[(?:\w+::|rs)?test)"]
    test_pattern: String,
    #[default = r"mod (tests?)"]
    namespace_pattern: String,

    #[default = r"\s*(?:async )?fn\s+(\w+)"]
    forward_test_pattern: String,
}

impl Cargotest {
    const NEAREST_SEPARATOR: &'static str = "::";
}

impl TestFramework for Cargotest {
    fn build_file_position_args(
        &self,
        context: &crate::Context,
    ) -> Result<ArgsList, Box<dyn std::error::Error>> {
        let mut args = vec![];

        let mut modules = context
            .rel()
            .with_extension("")
            .components()
            .map(|c| c.as_os_str().to_str().unwrap_or_default().to_string())
            .collect::<Vec<String>>();

        if Regex::new(r"^(main|lib|mod)$")?
            .is_match(modules.last().ok_or("Relative path is invalid")?)
        {
            modules.pop();
        }

        for i in (1..modules.len()).rev() {
            let parts = modules
                .clone()
                .into_iter()
                .take(i)
                .chain(std::iter::once(super::MANIFEST_FILE.into()))
                .collect::<Vec<String>>()
                .join(std::path::MAIN_SEPARATOR.to_string().as_str());
            if context.root().join(parts).exists() {
                args.push("--package".to_string());
                args.push(modules[i - 1].to_string());
                break;
            }
        }

        if modules[0] == "tests" && modules.len() == 2 || modules.len() <= 1 {
            return Ok(args);
        }

        let namespace = modules
            .into_iter()
            .skip(1)
            .chain(std::iter::once("".into()))
            .collect::<Vec<String>>()
            .join(Self::NEAREST_SEPARATOR);
        Ok(args
            .iter()
            .chain(std::iter::once(&namespace))
            .map(|s| s.to_string())
            .collect())
    }
}
