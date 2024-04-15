use crate::test_framework::TestFramework;

mod rust;

pub struct Registry {
    frameworks: Vec<Box<dyn TestFramework>>,
}

impl Registry {
    pub fn new() -> Self {
        let mut registry = Self { frameworks: vec![] };

        registry.add(Box::new(rust::Cargotest::default()));

        registry
    }

    fn add(&mut self, framework: Box<dyn TestFramework>) {
        self.frameworks.push(framework);
    }
}

impl IntoIterator for Registry {
    type Item = Box<dyn TestFramework>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.frameworks.into_iter()
    }
}
