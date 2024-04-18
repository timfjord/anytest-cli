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
mod utils;

pub type LineNr = usize;
pub type ArgsList = Vec<String>;

pub fn build_command(context: &Context) -> Result<Command, Box<dyn Error>> {
    let registry = registry::Registry::new();
    let test_framework = registry.find(context)?;
    let mut program_args = test_framework.executable(context).into_iter();
    let program = program_args.next().ok_or("Program must be present")?;
    let mut command = Command::new(program);

    command.current_dir(context.root());
    command.args(program_args);
    command.args(test_framework.position_args(context)?);

    Ok(command)
}

pub fn format_command(command: &Command) -> String {
    format!(
        "{} {}",
        command.get_program().to_str().unwrap_or_default(),
        command
            .get_args()
            .map(|a| a.to_str().unwrap_or_default())
            .collect::<Vec<&str>>()
            .join(" ")
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_command() {
        let mut command = Command::new("echo");
        command.arg("Hello,").arg("World!");

        assert_eq!(format_command(&command), "echo Hello, World!");
    }
}
