use crate::registry::{Framework, FrameworkData, Language};

use super::Rust;

pub struct CargoTest {
    language_data: Rust,
    framework_data: FrameworkData,
}

impl Default for CargoTest {
    fn default() -> Self {
        Self {
            language_data: Rust::default(),
            framework_data: FrameworkData {
                name: "cargotest".into(),
                env: Default::default(),
                pattern: r".rs$".into(),
            },
        }
    }
}

impl Framework for CargoTest {
    fn language_data(&self) -> Box<&dyn Language> {
        Box::new(&self.language_data)
    }

    fn framework_data(&self) -> &FrameworkData {
        &self.framework_data
    }

    fn executable(&self) -> String {
        "cargo test".into()
    }

    fn suite_position_args(&self) -> Vec<String> {
        vec!["suite".into()]
    }

    fn file_position_args(&self) -> Vec<String> {
        vec!["suite".into()]
    }

    fn line_position_args(&self) -> Vec<String> {
        vec!["line".into()]
    }
}
