use crate::input::statement_category_filter::{StatementCategoryFilter, VIRTUAL_FILE_REGEX};
use anyhow::{Context, Result};
use cairo_annotations::annotations::coverage::{
    CodeLocation, CoverageAnnotationsV1, LineNumber, SourceCodeSpan, SourceFileFullPath,
    VersionedCoverageAnnotations,
};
use cairo_annotations::annotations::profiler::{
    FunctionName, ProfilerAnnotationsV1, VersionedProfilerAnnotations,
};
use cairo_annotations::annotations::TryFromDebugInfo;
use cairo_lang_sierra::debug_info::DebugInfo;
use cairo_lang_sierra::program::StatementIdx;
use derived_deref::Deref;
use indoc::indoc;
use serde::Deserialize;
use std::collections::HashMap;
use std::iter;
use std::ops::RangeInclusive;

const RECOMMENDED_CAIRO_PROFILE_TOML: &str = indoc! {
    r#"
    Perhaps you are missing the following entries in Scarb.toml:

    [profile.dev.cairo]
    unstable-add-statements-functions-debug-info = true
    unstable-add-statements-code-locations-debug-info = true
    inlining-strategy = "avoid"
    "#
};

#[derive(Deref)]
pub struct SierraToCairoMap(HashMap<StatementIdx, StatementOrigin>);

#[derive(Clone, Eq, PartialEq)]
pub struct StatementOrigin {
    pub function_name: FunctionName,
    pub source_file_full_path: SourceFileFullPath,
    pub line_range: LineRange,
}

impl StatementOrigin {
    pub fn remove_virtual_file_prefix(&mut self) {
        self.source_file_full_path = SourceFileFullPath(
            VIRTUAL_FILE_REGEX
                .replace_all(&self.source_file_full_path.0, "")
                .to_string(),
        );
    }
}

#[derive(Deserialize, Clone, Eq, PartialEq)]
pub struct LineRange {
    /// Line number is 1-based
    pub start: LineNumber,
    /// Line number is 1-based
    pub end: LineNumber,
}

impl From<&SourceCodeSpan> for LineRange {
    fn from(span: &SourceCodeSpan) -> Self {
        // `SourceCodeSpan` is 0-based, so we need to add 1 to the line numbers
        Self {
            start: span.start.line + LineNumber(1),
            end: span.end.line + LineNumber(1),
        }
    }
}

impl IntoIterator for &LineRange {
    type Item = LineNumber;
    type IntoIter = iter::Map<RangeInclusive<usize>, fn(usize) -> LineNumber>;

    fn into_iter(self) -> Self::IntoIter {
        (self.start.0..=self.end.0).map(LineNumber)
    }
}

pub fn create_sierra_to_cairo_map(
    debug_info: &DebugInfo,
    filter: &StatementCategoryFilter,
) -> Result<SierraToCairoMap> {
    let VersionedCoverageAnnotations::V1(CoverageAnnotationsV1 {
        statements_code_locations,
    }) = VersionedCoverageAnnotations::try_from_debug_info(debug_info)
        .context(RECOMMENDED_CAIRO_PROFILE_TOML)?;

    let VersionedProfilerAnnotations::V1(ProfilerAnnotationsV1 {
        statements_functions,
    }) = VersionedProfilerAnnotations::try_from_debug_info(debug_info)
        .context(RECOMMENDED_CAIRO_PROFILE_TOML)?;

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
            |(CodeLocation(source_file_full_path, line_range), function_name)| StatementOrigin {
                function_name: function_name.clone(),
                source_file_full_path: source_file_full_path.clone(),
                line_range: line_range.into(),
            },
        )
        .find(|statement_origin| filter.should_include(statement_origin))
        .map(|mut statement_origin| {
            statement_origin.remove_virtual_file_prefix();
            statement_origin
        })
}
