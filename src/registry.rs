use crate::{test_framework::TestFramework, Context};

mod elixir;
mod javascript;
mod python;
mod ruby;
mod rust;
mod zig;

pub struct Registry {
    frameworks: Vec<Box<dyn TestFramework>>,
}

impl Registry {
    pub fn new() -> Self {
        let mut registry = Self { frameworks: vec![] };

        registry.add(Box::<elixir::ESpec>::default());
        registry.add(Box::<elixir::ExUnit>::default());
        registry.add(Box::<javascript::Jest>::default());
        registry.add(Box::<python::Pytest>::default());
        registry.add(Box::<ruby::RSpec>::default());
        registry.add(Box::<rust::Cargotest>::default());
        registry.add(Box::<zig::Zigtest>::default());

        registry
    }

    fn add(&mut self, framework: Box<dyn TestFramework>) {
        self.frameworks.push(framework);
    }

    pub fn find(
        &self,
        context: &Context,
    ) -> Result<&dyn TestFramework, Box<dyn std::error::Error>> {
        for framework in &self.frameworks {
            if framework.is_suitable_for(context) {
                return Ok(framework.as_ref());
            }
        }

        Err("No suitable test framework found".into())
    }
}
