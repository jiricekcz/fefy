use std::io::Write;

use anyhow::{Context, Ok, Result};
use fef::v0::{
    config::DEFAULT_CONFIG,
    metadata::{MetadataRecord, VariableNameMetadataRecordObj},
    write::{
        write_metadata_from_vec, write_metadata_vec_expression_tree_as_single_formula,
        write_single_formula,
    },
};

use crate::parser::ParsedToken;

pub(crate) fn write_tokens_as_fef_to_stream(
    tokens: &mut impl Iterator<Item = Result<ParsedToken>>,
    stream: &mut impl Write,
) -> Result<()> {
    let mut variable_names: Vec<String> = Vec::new();
    let expr_tree = crate::into_expr_tree::into_expr_tree(tokens, &mut variable_names, false)?;

    let variable_names_metadata_records: Vec<MetadataRecord> = variable_names
        .into_iter()
        .enumerate()
        .map(|(i, name)| {
            MetadataRecord::VariableName(VariableNameMetadataRecordObj::new(name, i.into()))
        })
        .collect();

    write_metadata_vec_expression_tree_as_single_formula(
        stream,
        &expr_tree,
        &DEFAULT_CONFIG,
        &variable_names_metadata_records,
    )
    .context("FEF Write Error")?;
    Ok(())
}
