use crate::input::test_function_filter::TestFunctionFilter;
use crate::input::{create_sierra_to_cairo_map, SierraToCairoMap, UniqueExecutedSierraIds};
use anyhow::{Context, Result};
use cairo_lang_sierra::program::{Program, ProgramArtifact, VersionedProgram};
use cairo_lang_sierra_to_casm::compiler::{CairoProgram, SierraToCasmConfig};
use cairo_lang_sierra_to_casm::metadata::{calc_metadata, MetadataComputationConfig};
use camino::Utf8PathBuf;
use serde::de::DeserializeOwned;
use std::fs;
use trace_data::CallTrace;

const SNFORGE_TEST_EXECUTABLE: &str = "snforge_internal_test_executable";

pub struct InputData {
    pub unique_executed_sierra_ids: UniqueExecutedSierraIds,
    pub sierra_to_cairo_map: SierraToCairoMap,
}

impl InputData {
    pub fn new(call_trace_path: &Utf8PathBuf, include_test_functions: bool) -> Result<Self> {
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

        let debug_info =
            debug_info.context(format!("Debug info not found in: {source_sierra_path}"))?;

        let test_function_filter = TestFunctionFilter::new(
            debug_info
                .executables
                .get(SNFORGE_TEST_EXECUTABLE)
                .unwrap_or(&Vec::new()),
            include_test_functions,
        );

        let sierra_to_cairo_map = create_sierra_to_cairo_map(&debug_info, &test_function_filter)?;
        let casm = compile_to_casm(&program)?;
        let unique_executed_sierra_ids =
            UniqueExecutedSierraIds::new(&casm, &call_trace, &sierra_to_cairo_map)?;

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
