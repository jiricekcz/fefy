use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Arguments {
    #[clap(subcommand)]
    pub subcommand: RootSubcommand,
}

#[derive(Subcommand, Debug)]
pub enum RootSubcommand {
    /// Evaluates a fef Single Formula file using a f64 interpreter
    Evaluate(Evaluate),

    /// Creates a new fef Single Formula file from user input
    Create(Create),
}

#[derive(Parser, Debug)]
pub struct Evaluate {
    /// The path to the fef file to evaluate
    #[clap(short, long)]
    pub input: PathBuf,
}

#[derive(Parser, Debug)]
pub struct Create {
    /// The path to and the name of the created fef file
    #[clap(short, long)]
    pub output: PathBuf,
}
