use anyhow::Result;
use arguments::Arguments;
use clap::Parser;
use tasks::{evaluate_from_file, write_to_file_from_stdin};

mod arguments;
mod cl_tools;
mod cli;
mod evaluate_fef_stream;
mod into_expr_tree;
mod parser;
mod tasks;
mod write_as_fef;

fn main() -> Result<()> {
    let arguments = Arguments::parse();

    cli::evaluate(arguments)?;
    Ok(())
}
