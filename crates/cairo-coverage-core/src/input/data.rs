use crate::input::filter::statement_category_filter::StatementCategoryFilter;
use crate::input::{create_sierra_to_cairo_map, SierraToCairoMap, UniqueExecutedSierraIds};
use crate::loading::enriched_program::EnrichedProgram;
use crate::loading::execution_data::ExecutionData;
use crate::merge::MergeOwned;
use anyhow::{Context, Result};
use cairo_lang_sierra::program::Program;
use cairo_lang_sierra_to_casm::compiler::{CairoProgramDebugInfo, SierraToCasmConfig};
use cairo_lang_sierra_to_casm::metadata::{calc_metadata, MetadataComputationConfig};

pub struct InputData {
    pub unique_executed_sierra_ids: UniqueExecutedSierraIds,
    pub sierra_to_cairo_map: SierraToCairoMap,
}

impl InputData {
    pub fn new(
        ExecutionData {
            casm_level_infos,
            enriched_program:
                EnrichedProgram {
                    program,
                    coverage_annotations,
                    profiler_annotations,
                    ..
                },
        }: ExecutionData,
        filter: &StatementCategoryFilter,
    ) -> Result<Self> {
        let casm_debug_info = compile(&program).expect("Failed to compile program to casm");
        let sierra_to_cairo_map = create_sierra_to_cairo_map(
            coverage_annotations,
            profiler_annotations,
            filter,
            &program,
        );
        let unique_executed_sierra_ids = casm_level_infos
            .iter()
            .map(|casm_level_info| {
                UniqueExecutedSierraIds::new(
                    &casm_debug_info,
                    casm_level_info,
                    &sierra_to_cairo_map,
                )
            })
            .reduce(MergeOwned::merge_owned)
            .context("Failed to create unique executed sierra ids")?;

        Ok(Self {
            unique_executed_sierra_ids,
            sierra_to_cairo_map,
        })
    }
}

fn compile(program: &Program) -> Result<CairoProgramDebugInfo> {
    let casm = cairo_lang_sierra_to_casm::compiler::compile(
        program,
        &calc_metadata(program, MetadataComputationConfig::default())?,
        SierraToCasmConfig {
            gas_usage_check: false,
            max_bytecode_size: usize::MAX,
        },
    )?;
    Ok(casm.debug_info)
}
