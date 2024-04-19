use clap::Parser;
use cli::Args;
use std::error::Error;

mod cli;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let context = args.to_context()?;
    let mut command = anytest::build_command(&context)?;

    if args.is_dry_run() {
        println!("{}", anytest::format_command(&command));
    } else {
        let output = command.spawn()?.wait_with_output()?;

        if !output.status.success() {
            // TODO: handle this differently
            std::process::exit(output.status.code().unwrap_or(1));
        }
    }

    Ok(())
}
