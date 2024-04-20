use crate::{
    context::Nearest, language::Language, named_pattern::NamedPattern, ArgsList, Context, Scope,
};
use regex::Regex;
use std::error::Error;

pub trait TestFrameworkMeta {
    fn language(&self) -> &dyn Language;

    fn language_name(&self) -> &str {
        self.language().name()
    }

    fn name(&self) -> &str;

    fn pattern(&self) -> Result<Regex, regex::Error>;

    fn default_executable(&self) -> Option<ArgsList>;

    fn args(&self) -> ArgsList;

    fn test_pattern(&self) -> &str;

    fn namespace_pattern(&self) -> &str;
}

pub trait TestFramework: TestFrameworkMeta {
    fn test_patterns(&self) -> Vec<NamedPattern> {
        vec![self.test_pattern().into()]
    }

    fn namespace_patterns(&self) -> Vec<NamedPattern> {
        let namespace_pattern = self.namespace_pattern();

        if namespace_pattern.is_empty() {
            vec![]
        } else {
            vec![namespace_pattern.into()]
        }
    }

    fn is_suitable_for(&self, context: &Context) -> bool {
        match self.pattern() {
            Ok(pattern) => pattern.is_match(context.path().to_str().unwrap_or_default()),
            Err(error) => {
                log::warn!("{}", error);
                false
            }
        }
    }

    fn build_executable(&self, _context: &Context) -> ArgsList {
        vec![]
    }

    fn executable(&self, context: &Context) -> ArgsList {
        self.default_executable()
            .unwrap_or(self.build_executable(context))
    }

    fn build_suite_position_args(&self, _context: &Context) -> Result<ArgsList, Box<dyn Error>> {
        Ok(vec![])
    }

    fn build_file_position_args(&self, context: &Context) -> Result<ArgsList, Box<dyn Error>> {
        Ok(vec![context.rel_str().into()])
    }

    fn build_line_position_args(&self, context: &Context) -> Result<ArgsList, Box<dyn Error>> {
        Ok(vec![context.rel_full()])
    }

    fn position_args(&self, context: &Context) -> Result<ArgsList, Box<dyn Error>> {
        match context.scope() {
            Scope::Suite => self.build_suite_position_args(context),
            Scope::File => self.build_file_position_args(context),
            Scope::Line => self.build_line_position_args(context),
        }
    }

    fn find_nearest(&self, context: &Context) -> Result<Nearest, Box<dyn Error>> {
        if let Some(line) = context.line_nr() {
            context.find_nearest(&self.test_patterns(), &self.namespace_patterns(), line..=1)
        } else {
            Err("Line number is required".into())
        }
    }
}
