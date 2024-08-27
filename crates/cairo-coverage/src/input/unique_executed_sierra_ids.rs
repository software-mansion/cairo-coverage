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
    pub fn new(casm: &CairoProgram, call_trace: &CallTrace) -> Result<Self> {
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

        Ok(Self(
            vm_trace
                .iter()
                .map(|step| step.pc)
                .filter(|pc| pc >= &real_minimal_pc)
                .map(|pc| {
                    let real_pc_code_offset = pc - real_minimal_pc;
                    casm.debug_info
                        .sierra_statement_info
                        .partition_point(|debug_info| {
                            debug_info.start_offset <= real_pc_code_offset
                        })
                        - 1
                })
                .map(StatementIdx)
                .counts(),
        ))
    }
}
