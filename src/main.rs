use clap::Parser;
use cli::Args;
use std::error::Error;
use std::io::{self, Write};

mod cli;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let context = args.to_context()?;
    let mut command = anytest::build_command(&context)?;

    if args.is_dry_run() {
        println!("{}", anytest::format_command(&command));
    } else {
        let output = command.output()?;

        io::stderr().write_all(&output.stderr)?;
        io::stdout().write_all(&output.stdout)?;
    }

    Ok(())
}
