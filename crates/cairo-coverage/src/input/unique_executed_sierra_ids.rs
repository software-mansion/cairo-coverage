use crate::input::SierraToCairoMap;
use crate::merge::MergeOwned;
use cairo_annotations::trace_data::CasmLevelInfo;
use cairo_lang_sierra::program::StatementIdx;
use cairo_lang_sierra_to_casm::compiler::CairoProgramDebugInfo;
use derived_deref::Deref;
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Deref)]
pub struct UniqueExecutedSierraIds(HashMap<StatementIdx, usize>);

impl MergeOwned for UniqueExecutedSierraIds {
    fn merge_owned(self, other: Self) -> Self {
        Self(self.0.merge_owned(other.0))
    }
}

impl UniqueExecutedSierraIds {
    pub fn new(
        casm_debug_info: &CairoProgramDebugInfo,
        casm_level_info: &CasmLevelInfo,
        sierra_to_cairo_map: &SierraToCairoMap,
    ) -> Self {
        let CasmLevelInfo {
            run_with_call_header,
            vm_trace,
        } = &casm_level_info;

        let real_minimal_pc = run_with_call_header
            .then(|| vm_trace.last().map_or(1, |trace| trace.pc + 1))
            .unwrap_or(1);

        let iter = vm_trace
            .iter()
            .map(|step| step.pc)
            .filter(|pc| pc >= &real_minimal_pc)
            .map(|pc| {
                let real_pc_code_offset = pc - real_minimal_pc;
                casm_debug_info
                    .sierra_statement_info
                    .partition_point(|debug_info| debug_info.start_offset <= real_pc_code_offset)
                    - 1
            })
            .map(StatementIdx);

        squash_idx_pointing_to_same_statement(iter, sierra_to_cairo_map)
    }
}

fn squash_idx_pointing_to_same_statement<I: Iterator<Item = StatementIdx>>(
    iter: I,
    sierra_to_cairo_map: &SierraToCairoMap,
) -> UniqueExecutedSierraIds {
    UniqueExecutedSierraIds(
        iter.fold(Vec::new(), |mut acc, statement_idx| {
            if points_to_different_statement(sierra_to_cairo_map, acc.last(), statement_idx) {
                acc.push(statement_idx);
            }
            acc
        })
        .into_iter()
        .counts(),
    )
}

fn points_to_different_statement(
    sierra_to_cairo_map: &SierraToCairoMap,
    last_idx: Option<&StatementIdx>,
    current_idx: StatementIdx,
) -> bool {
    last_idx.map_or(true, |last_idx| {
        sierra_to_cairo_map.get(last_idx) != sierra_to_cairo_map.get(&current_idx)
    })
}
