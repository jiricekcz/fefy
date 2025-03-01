use anyhow::Result;

use crate::{
    arguments::{Arguments, Create, Evaluate, RootSubcommand},
    evaluate_from_file, write_to_file_from_stdin,
};

pub(crate) fn evaluate(arguments: Arguments) -> Result<()> {
    match arguments.subcommand {
        RootSubcommand::Create(Create { output }) => write_to_file_from_stdin(&output),
        RootSubcommand::Evaluate(Evaluate { input }) => evaluate_from_file(&input),
    }
}
