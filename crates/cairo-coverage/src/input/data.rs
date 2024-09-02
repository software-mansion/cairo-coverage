use crate::data_loader::LoadedData;
use crate::input::test_function_filter::TestFunctionFilter;
use crate::input::{create_sierra_to_cairo_map, SierraToCairoMap, UniqueExecutedSierraIds};
use anyhow::{Context, Result};
use cairo_lang_sierra::program::Program;
use cairo_lang_sierra_to_casm::compiler::{CairoProgram, SierraToCasmConfig};
use cairo_lang_sierra_to_casm::metadata::{calc_metadata, MetadataComputationConfig};

const SNFORGE_TEST_EXECUTABLE: &str = "snforge_internal_test_executable";

pub struct InputData {
    pub unique_executed_sierra_ids: UniqueExecutedSierraIds,
    pub sierra_to_cairo_map: SierraToCairoMap,
}

impl InputData {
    pub fn new(
        LoadedData {
            program,
            debug_info,
            call_traces,
        }: &LoadedData,
        include_test_functions: bool,
    ) -> Result<Self> {
        let test_function_filter = TestFunctionFilter::new(
            debug_info
                .executables
                .get(SNFORGE_TEST_EXECUTABLE)
                .unwrap_or(&Vec::new()),
            include_test_functions,
        );

        let sierra_to_cairo_map = create_sierra_to_cairo_map(debug_info, &test_function_filter)?;
        let casm = compile_to_casm(program)?;
        let unique_executed_sierra_ids = call_traces
            .iter()
            .map(|call_trace| UniqueExecutedSierraIds::new(&casm, call_trace, &sierra_to_cairo_map))
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .reduce(|mut acc, unique_executed_sierra_ids| {
                acc.extend(unique_executed_sierra_ids.clone().into_iter());
                acc
            })
            .context("Failed to create unique executed sierra ids")?;

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
