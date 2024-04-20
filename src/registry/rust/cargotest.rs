use super::Rust;
use crate::{
    test_framework::{TestFramework, TestFrameworkMeta},
    utils, ArgsList,
};
use regex::Regex;
use smart_default::SmartDefault;
use std::iter;
use std::path;

const SEPARATOR: &str = "::";

#[derive(TestFrameworkMeta, SmartDefault)]
pub struct Cargotest {
    language: Rust,
    #[default = r".rs$"]
    pattern: String,
    #[default(_code = r#"vec!["cargo", "test"]"#)]
    executable: Vec<&'static str>,
    args: Vec<&'static str>,
    #[default = r"(#\[(?:\w+::|rs)?test)"]
    test_pattern: String,
    #[default = r"mod (tests?)"]
    namespace_pattern: String,

    #[default = r"\s*(?:async )?fn\s+(\w+)"]
    forward_test_pattern: String,
}

impl TestFramework for Cargotest {
    // TODO: Refactor this method, since it is almost the exact adaptation of the Python implementation
    //       (which is the adaptation of the Vimscript implementation).
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
                .join(path::MAIN_SEPARATOR.to_string().as_str());
            if context.root().join(parts).exists() {
                args.push("--package".to_string());
                args.push(modules[i - 1].to_string());
                modules.drain(0..i);
                break;
            }
        }

        if modules.len() <= 1 {
            return Ok(args);
        }

        if modules.len() == 2 && modules[0] == "tests" {
            return Ok(utils::concat(args, ["--test", &modules[1]]));
        }

        let namespace = [&modules[1..], &["".into()]].concat().join(SEPARATOR);
        Ok(utils::concat(args, [namespace]))
    }

    fn build_line_position_args(
        &self,
        context: &crate::Context,
    ) -> Result<ArgsList, Box<dyn std::error::Error>> {
        let mut args = self.build_file_position_args(context)?;
        let nearest = self.find_nearest(context)?;

        if !nearest.has_tests() || !Regex::new(r"#\[.*")?.is_match(&nearest.tests()[0]) {
            return Ok(args);
        }

        let forward_nearest = context.find_nearest(
            &[self.forward_test_pattern.as_str().into()],
            Default::default(),
            nearest.line_nr().unwrap()..=context.line_nr().unwrap(),
        )?;
        let test_name = if nearest.namespaces().is_empty() {
            forward_nearest.tests().first().unwrap().to_string()
        } else if forward_nearest.tests().is_empty() {
            nearest.tests().first().unwrap().to_string()
        } else {
            [&nearest.namespaces()[0..1], &forward_nearest.tests()[0..1]]
                .concat()
                .join(SEPARATOR)
        };
        let file_namespace = if args.len() > 0 && args[0] != "--test" {
            args.pop().unwrap_or_default()
        } else {
            String::new()
        };

        Ok(args
            .into_iter()
            .chain(iter::once(format!("{}{}", file_namespace, test_name)))
            .chain(iter::once("--".into()))
            .chain(iter::once("--exact".into()))
            .collect())
    }
}
