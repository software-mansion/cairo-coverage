use crate::loading::enriched_program::EnrichedProgram;
use crate::loading::{enriched_program, execution_infos};
use anyhow::Result;
use cairo_annotations::trace_data::CasmLevelInfo;
use camino::Utf8PathBuf;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::collections::HashMap;

/// Struct with all the necessary data loaded from the traces.
pub struct ExecutionData {
    pub casm_level_infos: Vec<CasmLevelInfo>,
    pub enriched_program: EnrichedProgram,
}

/// Load the [`ExecutionData`] from the given call trace path.
/// We do that in bulk for deserialization and execution optimizations
pub fn load(call_trace_paths: &[Utf8PathBuf]) -> Result<Vec<ExecutionData>> {
    let grouped_execution_infos = execution_infos::load_grouped(call_trace_paths)?;
    create_from_execution_infos(grouped_execution_infos)
}

/// Create the [`ExecutionData`] from the grouped execution infos.
fn create_from_execution_infos(
    grouped_execution_infos: HashMap<Utf8PathBuf, Vec<CasmLevelInfo>>,
) -> Result<Vec<ExecutionData>> {
    grouped_execution_infos
        .into_par_iter()
        .map(|(source_sierra_path, casm_level_infos)| {
            let loaded_program = enriched_program::load(&source_sierra_path)?;
            let execution_data = ExecutionData {
                casm_level_infos,
                enriched_program: loaded_program,
            };
            Ok(execution_data)
        })
        .collect()
}
