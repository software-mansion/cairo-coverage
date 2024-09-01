use crate::input::data_loader::types::{
    CodeLocation, CoverageAnnotations, LineRange, ProfilerAnnotations,
};
use crate::types::{FileLocation, FunctionName};
use anyhow::{Context, Result};
use cairo_lang_sierra::debug_info::Annotations;
use cairo_lang_sierra::program::StatementIdx;
use derived_deref::Deref;
use regex::Regex;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::sync::LazyLock;

static VIRTUAL_FILE_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\[.*?]").unwrap());

#[derive(Deref)]
pub struct SierraToCairoMap(HashMap<StatementIdx, StatementOrigin>);

#[derive(Clone, Eq, PartialEq)]
pub struct StatementOrigin {
    pub function_name: FunctionName,
    pub file_location: FileLocation,
    pub line_range: LineRange,
}

pub fn create_sierra_to_cairo_map(annotations: &Annotations) -> Result<SierraToCairoMap> {
    let CoverageAnnotations {
        statements_code_locations,
    } = CoverageAnnotations::get_namespace(annotations)?;

    let ProfilerAnnotations {
        statements_functions,
    } = ProfilerAnnotations::get_namespace(annotations)?;

    Ok(SierraToCairoMap(
        statements_code_locations
            .into_iter()
            .filter_map(|(key, code_locations)| {
                statements_functions
                    .get(&key)
                    .and_then(|function_names| {
                        find_statement_origin(&code_locations, function_names)
                    })
                    .map(|statement_origin| (key, statement_origin))
            })
            .collect(),
    ))
}

fn find_statement_origin(
    code_locations: &[CodeLocation],
    function_names: &[FunctionName],
) -> Option<StatementOrigin> {
    code_locations
        .iter()
        .zip(function_names)
        // TODO: We should probably filter by path to user project not by path to cache
        // TODO: Can get this from source_sierra_path in call trace
        .find(|((file_location, _), _)| !file_location.contains("com.swmansion.scarb"))
        .map(
            |((file_location, line_range), function_name)| StatementOrigin {
                function_name: function_name.to_owned(),
                file_location: remove_virtual_files(file_location),
                line_range: line_range.move_by_1(),
            },
        )
}

fn remove_virtual_files(file_location: &str) -> FileLocation {
    VIRTUAL_FILE_REGEX
        .replace_all(file_location, "")
        .to_string()
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
