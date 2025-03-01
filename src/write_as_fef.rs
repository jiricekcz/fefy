use std::io::Write;

use anyhow::{Context, Ok, Result};
use fef::v0::{
    config::DEFAULT_CONFIG,
    metadata::{MetadataRecord, NameMetadataRecordObj, VariableNameMetadataRecordObj},
    write::write_metadata_vec_expression_tree_as_single_formula,
};

use crate::parser::ParsedToken;

pub(crate) fn write_tokens_as_fef_to_stream(
    tokens: &mut impl Iterator<Item = Result<ParsedToken>>,
    stream: &mut impl Write,
    name: Option<String>,
) -> Result<()> {
    let mut variable_names: Vec<String> = Vec::new();
    let expr_tree = crate::into_expr_tree::into_expr_tree(tokens, &mut variable_names, false)?;

    let name_metadata_record =
        name.map(|name| MetadataRecord::Name(NameMetadataRecordObj::new(name)));

    let variable_names_metadata_records: Vec<MetadataRecord> = variable_names
        .into_iter()
        .enumerate()
        .map(|(i, name)| {
            MetadataRecord::VariableName(VariableNameMetadataRecordObj::new(name, i.into()))
        })
        .collect();

    let metadata_records: Vec<_> = name_metadata_record
        .into_iter()
        .chain(variable_names_metadata_records.into_iter())
        .collect();

    write_metadata_vec_expression_tree_as_single_formula(
        stream,
        &expr_tree,
        &DEFAULT_CONFIG,
        &metadata_records,
    )
    .context("FEF Write Error")?;
    Ok(())
}
