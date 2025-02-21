use crate::build::executed_statement_count::ExecutedStatementCount;
use crate::build::filter::statement_category_filter::StatementCategoryFilter;
use crate::build::statement_information::StatementInformationMap;
use crate::build::{executed_statement_count, statement_information};
use crate::loading::enriched_program::EnrichedProgram;
use crate::loading::execution_data::ExecutionData;
use anyhow::Result;
use cairo_lang_sierra::program::Program;
use cairo_lang_sierra_to_casm::compiler::{CairoProgramDebugInfo, SierraToCasmConfig};
use cairo_lang_sierra_to_casm::metadata::{MetadataComputationConfig, calc_metadata};

/// All necessary data for the coverage analysis.
#[derive(Clone)]
pub struct CoverageInput {
    pub executed_statement_count: ExecutedStatementCount,
    pub statement_information_map: StatementInformationMap,
}

/// Build the [`CoverageInput`].
/// This involves compiling the program to `casm`, which is a costly operation.
/// # Panics
/// - panics if the program cannot be compiled to `casm`.
pub fn build(
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
) -> CoverageInput {
    let casm_debug_info = compile(&program).expect("Failed to compile program to casm");

    let statement_information_map =
        statement_information::build_map(coverage_annotations, profiler_annotations, filter);

    let executed_statement_count = executed_statement_count::build(
        &casm_level_infos,
        &casm_debug_info,
        &statement_information_map,
    );

    CoverageInput {
        executed_statement_count,
        statement_information_map,
    }
}

/// Compile the given [`Program`] to `casm` and return the [`CairoProgramDebugInfo`].
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
