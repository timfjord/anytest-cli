use crate::EnvVars;

pub trait Language {
    fn name(&self) -> &str;

    fn env(&self) -> &EnvVars;
}
