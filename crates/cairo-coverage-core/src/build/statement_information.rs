use crate::build::filter::statement_category_filter::StatementCategoryFilter;
use cairo_annotations::annotations::coverage::{
    CodeLocation, CoverageAnnotationsV1, LineNumber, SourceCodeSpan, SourceFileFullPath,
    VersionedCoverageAnnotations,
};
use cairo_annotations::annotations::profiler::{
    FunctionName, ProfilerAnnotationsV1, VersionedProfilerAnnotations,
};
use cairo_lang_sierra::program::StatementIdx;
use serde::Deserialize;
use std::collections::HashMap;
use std::iter;
use std::ops::RangeInclusive;

/// Mapping from Sierra statement IDs to additional information about the statement.
pub type StatementInformationMap = HashMap<StatementIdx, StatementInformation>;

/// Additional information about a statement that is needed for coverage analysis.
#[derive(Clone, Eq, PartialEq)]
pub struct StatementInformation {
    pub idx: StatementIdx,
    pub function_name: FunctionName,
    pub source_file_full_path: SourceFileFullPath,
    pub line_range: LineRange,
}

#[derive(Deserialize, Clone, Eq, PartialEq)]
pub struct LineRange {
    /// Line number is 1-based
    pub start: LineNumber,
    /// Line number is 1-based
    pub end: LineNumber,
}

impl From<SourceCodeSpan> for LineRange {
    fn from(span: SourceCodeSpan) -> Self {
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
        // FIXME: Use the whole range instead of the start once we drop support for scarb 2.8.*
        (self.start.0..=self.start.0).map(LineNumber)
    }
}

/// Builds [`StatementInformationMap`].
pub fn build_map(
    VersionedCoverageAnnotations::V1(CoverageAnnotationsV1 {
        statements_code_locations,
    }): VersionedCoverageAnnotations,
    VersionedProfilerAnnotations::V1(ProfilerAnnotationsV1 {
        mut statements_functions,
    }): VersionedProfilerAnnotations,
    filter: &StatementCategoryFilter,
) -> StatementInformationMap {
    statements_code_locations
        .into_iter()
        .filter_map(|(key, code_locations)| {
            let function_names = statements_functions.remove(&key)?;
            let statement_origin =
                get_statement_information(key, code_locations, function_names, filter)?;
            Some((key, statement_origin))
        })
        .collect()
}

/// Get the statement information for a given statement ID.
/// We take the first code location that matches the filter as they are sorted by priority.
fn get_statement_information(
    idx: StatementIdx,
    code_locations: Vec<CodeLocation>,
    function_names: Vec<FunctionName>,
    filter: &StatementCategoryFilter,
) -> Option<StatementInformation> {
    code_locations.into_iter().zip(function_names).find_map(
        |(CodeLocation(source_file_full_path, line_range, is_macro), function_name)| {
            let is_macro = is_macro.unwrap_or_default();
            filter
                .should_include(idx, &function_name, &source_file_full_path, is_macro)
                .then(|| StatementInformation {
                    function_name,
                    source_file_full_path,
                    line_range: line_range.into(),
                    idx,
                })
        },
    )
}
