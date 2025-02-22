use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Arguments {
    #[clap(subcommand)]
    pub subcommand: RootSubcommand,
}

#[derive(Subcommand, Debug)]
pub enum RootSubcommand {
    Evaluate(Evaluate),
    From(FromString),
}

#[derive(Parser, Debug)]
pub struct Evaluate {}

#[derive(Parser, Debug)]
pub struct FromString {
    #[clap(short, long)]
    pub string: String,
}
