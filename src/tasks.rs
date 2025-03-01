use std::{collections::BTreeMap, io::Write, path::Path};

use anyhow::{bail, Context, Result};
use fef::v0::{
    config::{OverridableConfig, DEFAULT_CONFIG},
    metadata::MetadataRecord,
    raw::VariableLengthEnum,
    read::{read_configuration_with_default_configuration, read_metadata_as_vec},
    tokens::FileContentTypeToken,
    traits::ReadFrom,
};

pub(crate) fn write_to_file_from_stdin(file: &Path) -> Result<()> {
    let mut write_stream = std::fs::File::create(file)?;

    let name = {
        print!("Enter name for formula:");

        crate::cl_tools::read_line()
    }
    .trim()
    .to_string();

    let name = if name.is_empty() { None } else { Some(name) };

    let formula = {
        print!("Enter formula:");
        crate::cl_tools::read_line()
    };

    let input_chars = formula.chars();
    let mut tokens = crate::parser::Tokens::new(input_chars);

    crate::write_as_fef::write_tokens_as_fef_to_stream(&mut tokens, &mut write_stream, name)?;

    write_stream.flush()?;

    Ok(())
}

pub(crate) fn evaluate_from_file(file: &Path) -> Result<()> {
    let mut read_stream = std::fs::File::open(file)?;

    let version: usize = VariableLengthEnum::read_from(&mut read_stream, &DEFAULT_CONFIG)
        .context("Reading version from file.")?
        .try_into()
        .context("Version parse")?;

    if version != 0 {
        bail!("Unsupported version: {}", version);
    }

    let file_content_type = FileContentTypeToken::read_from(&mut read_stream, &DEFAULT_CONFIG)
        .context("Reading file content type from file.")?;

    let configuration = match file_content_type {
        FileContentTypeToken::SingleFormula => {
            read_configuration_with_default_configuration(&mut read_stream)
                .context("Reading configuration from file.")?
        }
        FileContentTypeToken::RawFormula => OverridableConfig::default(),
        _ => bail!("Unsupported file content type: {:?}", file_content_type),
    };

    let metadata_vec = match file_content_type {
        FileContentTypeToken::SingleFormula => {
            read_metadata_as_vec(&mut read_stream, &configuration)
                .context("Reading metadata from file.")?
        }
        FileContentTypeToken::RawFormula => Vec::new(),
        _ => bail!("Unsupported file content type: {:?}", file_content_type),
    };

    let names = metadata_vec
        .iter()
        .filter_map(|record| match record {
            MetadataRecord::Name(name_record) => Some(name_record.name()),
            _ => None,
        })
        .collect::<Vec<_>>();

    let variable_names: Vec<(&VariableLengthEnum, &str)> = metadata_vec
        .iter()
        .filter_map(|record| match record {
            MetadataRecord::VariableName(variable_name_record) => Some((
                variable_name_record.variable_identifier(),
                variable_name_record.name(),
            )),
            _ => None,
        })
        .collect();

    if names.len() > 1 {
        bail!("Malformed FEF file: more than one name record.");
    }

    if names.len() == 1 {
        println!("============================================================");
        println!("Evaluating {}", names[0]);
    }

    if variable_names.len() > 0 {
        println!("============================================================");
        println!("Enter values for variables:");
        println!("------------------------------------------------------------");
    }

    let mut variable_values: BTreeMap<VariableLengthEnum, f64> = BTreeMap::new();

    for (variable_identifier, variable_name) in variable_names.iter() {
        print!("Enter value for variable '{}':", variable_name);
        let value = crate::cl_tools::read_line().trim().parse::<f64>()?;
        variable_values.insert(variable_identifier.to_owned().clone(), value);
    }

    if variable_names.len() > 0 {
        println!("============================================================");
    }

    let result =
        crate::evaluate_fef_stream::evaluate_stream_as_fef_expr(&mut read_stream, variable_values);

    match result {
        Ok(result) => {
            println!("Result: {}", result);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

    Ok(())
}
