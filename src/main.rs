use clap::Parser;
use cli::Args;
use std::{error::Error, process::ExitCode};

mod cli;

fn main() -> Result<ExitCode, Box<dyn Error>> {
    let args = Args::parse();
    let context = args.to_context()?;
    let mut command = anytest::build_command(&context)?;

    if args.is_dry_run() {
        println!("{}", anytest::format_command(&command));
    } else {
        let output = command.spawn()?.wait_with_output()?;

        if !output.status.success() {
            return Ok(output
                .status
                .code()
                .unwrap_or(1)
                .try_into()
                .unwrap_or(1)
                .into());
        }
    }

    Ok(ExitCode::SUCCESS)
}
