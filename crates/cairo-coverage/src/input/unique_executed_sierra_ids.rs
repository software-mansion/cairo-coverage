use crate::input::SierraToCairoMap;
use anyhow::{Context, Result};
use cairo_lang_sierra::program::StatementIdx;
use cairo_lang_sierra_to_casm::compiler::CairoProgram;
use derived_deref::Deref;
use itertools::Itertools;
use std::collections::HashMap;
use trace_data::{CallTrace, CasmLevelInfo};

#[derive(Deref)]
pub struct UniqueExecutedSierraIds(HashMap<StatementIdx, usize>);

impl UniqueExecutedSierraIds {
    pub fn new(
        casm: &CairoProgram,
        call_trace: &CallTrace,
        sierra_to_cairo_map: &SierraToCairoMap,
    ) -> Result<Self> {
        let CasmLevelInfo {
            run_with_call_header,
            vm_trace,
        } = &call_trace
            .cairo_execution_info
            .as_ref()
            .context("Missing key 'cairo_execution_info' in call trace")?
            .casm_level_info;

        let real_minimal_pc = run_with_call_header
            .then(|| vm_trace.last().map_or(1, |trace| trace.pc + 1))
            .unwrap_or(1);

        let iter = vm_trace
            .iter()
            .map(|step| step.pc)
            .filter(|pc| pc >= &real_minimal_pc)
            .map(|pc| {
                let real_pc_code_offset = pc - real_minimal_pc;
                casm.debug_info
                    .sierra_statement_info
                    .partition_point(|debug_info| debug_info.start_offset <= real_pc_code_offset)
                    - 1
            })
            .map(StatementIdx);

        Ok(squash_idx_pointing_to_same_statement(
            iter,
            sierra_to_cairo_map,
        ))
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
