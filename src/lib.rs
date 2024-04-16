use std::collections::HashMap;
use std::error::Error;
use std::process::Command;

pub use context::Context;
pub use context::Scope;
pub use rel_path::RelPath;

#[macro_use]
extern crate anytest_derive;

mod context;
mod language;
mod named_pattern;
mod registry;
mod rel_path;
mod test_framework;

pub type LineNr = usize;
pub type ArgsList = Vec<String>;
pub type EnvVars = HashMap<String, String>;

pub fn build_command(context: &Context) -> Result<Command, Box<dyn Error>> {
    let registry = registry::Registry::new();
    let test_framework = registry.find(context)?;
    let (program, program_args) = test_framework.program()?;
    let mut command = Command::new(program);

    command.current_dir(context.root());
    command.args(program_args);
    command.args(test_framework.args());
    command.args(test_framework.position_args(context)?);
    command.envs(test_framework.env());

    Ok(command)
}
