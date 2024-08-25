use crate::input::{create_sierra_to_cairo_map, SierraToCairoMap, UniqueExecutedSierraIds};
use anyhow::{Context, Result};
use cairo_lang_sierra::program::{Program, ProgramArtifact, VersionedProgram};
use cairo_lang_sierra_to_casm::compiler::{CairoProgram, SierraToCasmConfig};
use cairo_lang_sierra_to_casm::metadata::{calc_metadata, MetadataComputationConfig};
use camino::Utf8PathBuf;
use serde::de::DeserializeOwned;
use std::fs;
use trace_data::CallTrace;

#[allow(dead_code)] // Temporary
pub struct InputData {
    pub unique_executed_sierra_ids: UniqueExecutedSierraIds,
    pub sierra_to_cairo_map: SierraToCairoMap,
}

impl TryFrom<&Utf8PathBuf> for InputData {
    type Error = anyhow::Error;

    fn try_from(call_trace_path: &Utf8PathBuf) -> Result<Self, Self::Error> {
        let call_trace: CallTrace = read_and_deserialize(call_trace_path)?;

        let source_sierra_path = &call_trace
            .cairo_execution_info
            .as_ref()
            .context("Missing key 'cairo_execution_info' in call trace")?
            .source_sierra_path;

        let VersionedProgram::V1 {
            program:
                ProgramArtifact {
                    program,
                    debug_info,
                },
            ..
        } = read_and_deserialize(source_sierra_path)?;

        let annotations = debug_info
            .context(format!("Debug info not found in: {source_sierra_path}"))?
            .annotations;

        let sierra_to_cairo_map = create_sierra_to_cairo_map(&annotations)?;
        let casm = compile_to_casm(&program)?;
        let unique_executed_sierra_ids = UniqueExecutedSierraIds::new(&casm, &call_trace)?;

        Ok(Self {
            unique_executed_sierra_ids,
            sierra_to_cairo_map,
        })
    }
}

fn compile_to_casm(program: &Program) -> Result<CairoProgram> {
    cairo_lang_sierra_to_casm::compiler::compile(
        program,
        &calc_metadata(program, MetadataComputationConfig::default())
            .context("Failed calculating Sierra variables")?,
        SierraToCasmConfig {
            gas_usage_check: false,
            max_bytecode_size: usize::MAX,
        },
    )
    .context("Failed to compile sierra to casm")
}

fn read_and_deserialize<T: DeserializeOwned>(file_path: &Utf8PathBuf) -> Result<T> {
    fs::read_to_string(file_path)
        .context(format!("Failed to read file at path: {file_path}"))
        .and_then(|content| {
            serde_json::from_str(&content).context(format!(
                "Failed to deserialize JSON content from file at path: {file_path}"
            ))
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::data_loader::types::{LineRange, Position};
    use cairo_lang_sierra::program::StatementIdx;

    const TRACE: &str = "tests/data/test_project/snfoundry_trace/tests::test_call::my_test.json";

    #[test]
    fn test_all() {
        let input_data = InputData::try_from(&TRACE.into()).unwrap();

        assert_eq!(input_data.sierra_to_cairo_map.len(), 142);

        let origin = &input_data.sierra_to_cairo_map[&StatementIdx(1)];
        assert!(origin.file_location.ends_with("test_call.cairo"));

        let statement_details = &input_data.sierra_to_cairo_map[&StatementIdx(1)];
        assert_eq!(statement_details.function_name, "tests::test_call::my_test");
        assert!(
            origin.line_range
                == LineRange {
                    start: Position { line: 5 },
                    end: Position { line: 5 },
                }
        );
    }
}
