use regex::{Error, Regex};

pub trait NamedPattern {
    fn name(&self) -> Option<String>;

    fn pattern(&self) -> Result<Regex, Error>;

    fn find(&self, haystack: &str) -> Option<(String, Option<String>)> {
        match self.pattern() {
            Ok(pattern) => pattern.captures(haystack).map(|c| {
                (
                    c.get(1).map(|m| m.as_str().to_string()).unwrap_or_default(),
                    self.name(),
                )
            }),
            Err(error) => {
                log::warn!("{}", error);
                None
            }
        }
    }
}

impl NamedPattern for String {
    fn name(&self) -> Option<String> {
        None
    }

    fn pattern(&self) -> Result<Regex, Error> {
        Regex::new(self)
    }
}
