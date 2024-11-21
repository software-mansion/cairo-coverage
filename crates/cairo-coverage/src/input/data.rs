use crate::data_loader::LoadedData;
use crate::input::statement_category_filter::StatementCategoryFilter;
use crate::input::{create_sierra_to_cairo_map, SierraToCairoMap, UniqueExecutedSierraIds};
use crate::merge::MergeOwned;
use anyhow::{Context, Result};

pub struct InputData {
    pub unique_executed_sierra_ids: UniqueExecutedSierraIds,
    pub sierra_to_cairo_map: SierraToCairoMap,
}

impl InputData {
    pub fn new(
        LoadedData {
            debug_info,
            casm_level_infos,
            casm_debug_info,
            program,
        }: &LoadedData,
        filter: &StatementCategoryFilter,
    ) -> Result<Self> {
        let sierra_to_cairo_map = create_sierra_to_cairo_map(debug_info, filter, program)?;
        let unique_executed_sierra_ids = casm_level_infos
            .iter()
            .map(|casm_level_info| {
                UniqueExecutedSierraIds::new(casm_debug_info, casm_level_info, &sierra_to_cairo_map)
            })
            .reduce(MergeOwned::merge_owned)
            .context("Failed to create unique executed sierra ids")?;

        Ok(Self {
            unique_executed_sierra_ids,
            sierra_to_cairo_map,
        })
    }
}
