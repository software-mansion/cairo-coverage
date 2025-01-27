use crate::loading::read_and_deserialize;
use anyhow::Result;
use cairo_annotations::trace_data::{
    CairoExecutionInfo, CallTraceNode, CallTraceV1, CasmLevelInfo, VersionedCallTrace,
};
use camino::Utf8PathBuf;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use std::collections::HashMap;

/// Load the grouped [`CairoExecutionInfo`].
/// # Optimization
/// - We group them by `source_sierra_path` so that the same Sierra program does not need to be deserialized multiple times.
/// - We grouped them in hashmap of `source_sierra_path` to `Vec<CasmLevelInfo>` not `Vec<CairoExecutionInfo>` to avoid cloning the `source_sierra_path` multiple times.
pub fn load_grouped(
    call_trace_paths: &[Utf8PathBuf],
) -> Result<HashMap<Utf8PathBuf, Vec<CasmLevelInfo>>> {
    let call_traces = call_trace_paths
        .par_iter()
        .map(read_and_deserialize)
        .collect::<Result<Vec<_>>>()?;

    let execution_infos = call_traces
        .into_par_iter()
        .flat_map(load_cairo_execution_infos)
        .collect();

    Ok(group_by_sierra_path(execution_infos))
}

/// Group the [`CairoExecutionInfo`] by `source_sierra_path`.
fn group_by_sierra_path(
    execution_infos: Vec<CairoExecutionInfo>,
) -> HashMap<Utf8PathBuf, Vec<CasmLevelInfo>> {
    execution_infos.into_iter().fold(
        HashMap::new(),
        |mut acc: HashMap<_, Vec<_>>, execution_info| {
            acc.entry(execution_info.source_sierra_path)
                .or_default()
                .push(execution_info.casm_level_info);
            acc
        },
    )
}

/// Load the [`CairoExecutionInfo`] from the given call trace.
/// As the [`CallTraceV1`] is nested, we need to load recursively.
fn load_cairo_execution_infos(
    VersionedCallTrace::V1(call_trace): VersionedCallTrace,
) -> Vec<CairoExecutionInfo> {
    fn load_recursively(call_trace: CallTraceV1, acc: &mut Vec<CairoExecutionInfo>) {
        if let Some(execution_info) = call_trace.cairo_execution_info {
            acc.push(execution_info);
        }

        for call_trace_node in call_trace.nested_calls {
            if let CallTraceNode::EntryPointCall(nested_call_trace) = call_trace_node {
                load_recursively(*nested_call_trace, acc);
            }
        }
    }

    let mut execution_infos = Vec::new();
    load_recursively(call_trace, &mut execution_infos);
    execution_infos
}
