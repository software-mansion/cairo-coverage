use crate::data_loader::{CodeLocation, CoverageAnnotations, LineRange, ProfilerAnnotations};
use crate::input::statement_category_filter::{StatementCategoryFilter, VIRTUAL_FILE_REGEX};
use crate::types::{FileLocation, FunctionName};
use anyhow::{Context, Result};
use cairo_lang_sierra::debug_info::{Annotations, DebugInfo};
use cairo_lang_sierra::program::StatementIdx;
use derived_deref::Deref;
use serde::de::DeserializeOwned;
use std::collections::HashMap;

#[derive(Deref)]
pub struct SierraToCairoMap(HashMap<StatementIdx, StatementOrigin>);

#[derive(Clone, Eq, PartialEq)]
pub struct StatementOrigin {
    pub function_name: FunctionName,
    pub file_location: FileLocation,
    pub line_range: LineRange,
}

impl StatementOrigin {
    pub fn remove_virtual_file_prefix(&mut self) {
        self.file_location = VIRTUAL_FILE_REGEX
            .replace_all(&self.file_location, "")
            .to_string();
    }
}

pub fn create_sierra_to_cairo_map(
    debug_info: &DebugInfo,
    filter: &StatementCategoryFilter,
) -> Result<SierraToCairoMap> {
    let CoverageAnnotations {
        statements_code_locations,
    } = CoverageAnnotations::get_namespace(&debug_info.annotations)?;

    let ProfilerAnnotations {
        statements_functions,
    } = ProfilerAnnotations::get_namespace(&debug_info.annotations)?;

    Ok(SierraToCairoMap(
        statements_code_locations
            .into_iter()
            .filter_map(|(key, code_locations)| {
                statements_functions
                    .get(&key)
                    .and_then(|function_names| {
                        find_statement_origin(&code_locations, function_names, filter)
                    })
                    .map(|statement_origin| (key, statement_origin))
            })
            .collect(),
    ))
}

fn find_statement_origin(
    code_locations: &[CodeLocation],
    function_names: &[FunctionName],
    filter: &StatementCategoryFilter,
) -> Option<StatementOrigin> {
    code_locations
        .iter()
        .zip(function_names)
        .map(
            |((file_location, line_range), function_name)| StatementOrigin {
                function_name: function_name.clone(),
                file_location: file_location.clone(),
                line_range: line_range.move_by_1(),
            },
        )
        .find(|statement_origin| filter.should_include(statement_origin))
        .map(|mut statement_origin| {
            statement_origin.remove_virtual_file_prefix();
            statement_origin
        })
}

trait Namespace {
    const NAMESPACE: &'static str;

    fn get_namespace<T: DeserializeOwned>(annotations: &Annotations) -> Result<T> {
        annotations
            .get(Self::NAMESPACE)
            .cloned()
            .context(format!("Expected key: {} but was missing", Self::NAMESPACE))
            .and_then(|value| {
                serde_json::from_value(value)
                    .context(format!("Failed to deserialize at key: {}", Self::NAMESPACE))
            })
    }
}

impl Namespace for CoverageAnnotations {
    const NAMESPACE: &'static str = "github.com/software-mansion/cairo-coverage";
}

impl Namespace for ProfilerAnnotations {
    const NAMESPACE: &'static str = "github.com/software-mansion/cairo-profiler";
}
