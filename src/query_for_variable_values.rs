use std::io::Read;

use anyhow::Result;
use fef::v0::{config::DEFAULT_CONFIG, metadata, read::read_metadata_as_vec};

pub(crate) fn query_for_variable_values(fef_stream: &mut impl Read) -> Result<Vec<f64>> {
    let metadata_vec = read_metadata_as_vec(fef_stream, &DEFAULT_CONFIG)?;
    let mut variable_values = Vec::new();

    for metadata in metadata_vec {
        if let fef::v0::metadata::MetadataRecord::VariableName(variable_name) = metadata {
            println!("Enter value for variable '{}':", variable_name.name());
            let value = crate::cl_tools::read_line().parse::<f64>()?;
            variable_values.push(value);
        }
    }

    Ok(variable_values)
}
