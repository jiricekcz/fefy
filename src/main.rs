use std::{path::PathBuf, str::FromStr};

use anyhow::Result;
use tasks::{evaluate_from_file, write_to_file_from_stdin};

mod arguments;
mod cl_tools;
mod evaluate_fef_stream;
mod into_expr_tree;
mod parser;
mod tasks;
mod write_as_fef;

fn main() -> Result<()> {
    let file_name = PathBuf::from_str("formula.fef")?;
    write_to_file_from_stdin(&file_name)?;

    evaluate_from_file(&file_name)?;

    Ok(())
}
