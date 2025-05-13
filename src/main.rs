mod cli;
mod io_utils;
mod path_utils;

use std::io::{Read, Write};

use clap::Parser;
use cli::Cli;
use io_utils::{Reader, Writer, get_reader, get_writer};

pub type DynError = Box<dyn std::error::Error>;
pub type DynResult<T> = Result<T, DynError>;

fn main() -> DynResult<()> {
    let cli = Cli::parse();

    let source = cli.target.as_deref();
    let mut reader = get_reader(source)?;
    let mut writer = get_writer(source)?;

    match cli.command {
        cli::Command::Format { no_check } => format(&mut reader, &mut writer, &cli, no_check)?,
        cli::Command::Check => check(&mut reader, &cli)?,
    }

    Ok(())
}

fn format(reader: &mut Reader, writer: &mut Writer, cli: &Cli, no_check: bool) -> DynResult<()> {
    let mut input = String::new();
    reader.read_to_string(&mut input)?;

    let tokens = shared::lexer::tokenize(&input)?;
    let old_cst = shared::cst::create_cst(tokens, cli.verbose)?;

    if !no_check {
        // Check if the old_cst and new_cst are the same but ignoring
    }

    let output = input; // temporary placeholder for the output

    writer.write_all(output.as_bytes())?;
    Ok(())
}

fn check(reader: &mut Reader, cli: &Cli) -> DynResult<()> {
    let mut input = String::new();
    reader.read_to_string(&mut input)?;

    let tokens = shared::lexer::tokenize(&input)?;
    let cst = shared::cst::create_cst(tokens, cli.verbose)?;

    let output = &input; // temporary placeholder for the output

    if output != &input {
        std::process::exit(1);
    }

    Ok(())
}
