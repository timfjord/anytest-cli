use regex::{Error, Regex};

pub struct NamedPattern {
    pattern: Result<Regex, Error>,
    name: Option<String>,
}

impl NamedPattern {
    pub fn new(pattern: &str, name: Option<&str>) -> Self {
        Self {
            pattern: Regex::new(pattern),
            name: name.map(|s| s.to_string()),
        }
    }

    pub fn pattern(&self) -> Result<&Regex, &Error> {
        self.pattern.as_ref()
    }

    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    pub fn find(&self, haystack: &str) -> Option<(String, Option<&String>)> {
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

impl From<&str> for NamedPattern {
    fn from(s: &str) -> Self {
        Self::new(s, None)
    }
}

impl From<String> for NamedPattern {
    fn from(s: String) -> Self {
        s.as_str().into()
    }
}
