use crate::input::SierraToCairoMap;
use crate::merge::MergeOwned;
use cairo_annotations::map_pcs_to_sierra_statement_ids;
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
        squash_idx_pointing_to_same_statement(
            map_pcs_to_sierra_statement_ids(casm_debug_info, casm_level_info)
                .into_iter()
                .filter_map(Option::from),
            sierra_to_cairo_map,
        )
    }
}

fn squash_idx_pointing_to_same_statement(
    iter: impl Iterator<Item = StatementIdx>,
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
