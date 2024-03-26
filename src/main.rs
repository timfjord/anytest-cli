use clap::Parser;
use cli::{format_command, Args};
use std::error::Error;
use std::io::{self, Write};

mod cli;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let context = args.build_context()?;
    let scope = args.scope(&context);
    let mut command = any_test::build_command(scope, context)?;

    if args.is_dry_run() {
        println!("{}", format_command(&command));
    } else {
        let output = command.output()?;

        io::stdout().write_all(&output.stdout)?;
        io::stderr().write_all(&output.stderr)?;
    }

    Ok(())
}
