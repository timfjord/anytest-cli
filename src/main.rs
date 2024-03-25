use clap::Parser;
use cli::{format_command, Args};
use std::error::Error;

mod cli;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let context = args.build_context()?;
    let scope = args.scope(&context);
    let mut command = any_test::build_command(scope, context)?;

    if args.is_dry_run() {
        println!("{}", format_command(&command));
    } else {
        command.spawn()?;
    }

    Ok(())
}
