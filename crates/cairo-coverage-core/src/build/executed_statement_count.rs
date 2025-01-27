use crate::build::statement_information::StatementInformationMap;
use cairo_annotations::map_pcs_to_sierra_statement_ids;
use cairo_annotations::trace_data::CasmLevelInfo;
use cairo_lang_sierra::program::StatementIdx;
use cairo_lang_sierra_to_casm::compiler::CairoProgramDebugInfo;
use itertools::Itertools;
use std::collections::HashMap;

/// Sierra statement IDs that were executed and their count.
pub type ExecutedStatementCount = HashMap<StatementIdx, usize>;

/// Build the executed statement count.
/// This involves:
/// - mapping casm level info to sierra statement IDs
/// - deduplicating the IDs
pub fn build(
    casm_level_infos: &[CasmLevelInfo],
    casm_debug_info: &CairoProgramDebugInfo,
    statement_information_map: &StatementInformationMap,
) -> ExecutedStatementCount {
    let executed_statement_ids = casm_level_infos
        .iter()
        .flat_map(|casm_level_info| {
            map_pcs_to_sierra_statement_ids(casm_debug_info, casm_level_info)
        })
        .filter_map(Option::from)
        .collect();

    deduplicate(executed_statement_ids, statement_information_map)
}

/// Deduplicate the executed statement IDs and that are neighboring.
/// As from one line, many statements can be generated. We want to count the line only once.
fn deduplicate(
    iter: Vec<StatementIdx>,
    statement_information_map: &StatementInformationMap,
) -> ExecutedStatementCount {
    iter.into_iter()
        .fold(Vec::new(), |mut acc, statement_idx| {
            if points_to_different_statement(statement_information_map, acc.last(), statement_idx) {
                acc.push(statement_idx);
            }
            acc
        })
        .into_iter()
        .counts()
}

fn points_to_different_statement(
    statement_information_map: &StatementInformationMap,
    last_idx: Option<&StatementIdx>,
    current_idx: StatementIdx,
) -> bool {
    last_idx.map_or(true, |last_idx| {
        statement_information_map.get(last_idx) != statement_information_map.get(&current_idx)
    })
}
