use crate::input::filter::statement_category_filter::{
    StatementCategoryFilter, VIRTUAL_FILE_REGEX,
};
use cairo_annotations::annotations::coverage::{
    CodeLocation, CoverageAnnotationsV1, LineNumber, SourceCodeSpan, SourceFileFullPath,
    VersionedCoverageAnnotations,
};
use cairo_annotations::annotations::profiler::{
    FunctionName, ProfilerAnnotationsV1, VersionedProfilerAnnotations,
};
use cairo_lang_sierra::program::{Program, Statement, StatementIdx};
use derived_deref::Deref;
use serde::Deserialize;
use std::collections::HashMap;
use std::iter;
use std::ops::RangeInclusive;

#[derive(Deref)]
pub struct SierraToCairoMap(HashMap<StatementIdx, StatementOrigin>);

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct SimpleLibfuncName(pub String);

#[derive(Clone, Eq, PartialEq)]
pub struct StatementOrigin {
    pub function_name: FunctionName,
    pub source_file_full_path: SourceFileFullPath,
    pub line_range: LineRange,
    pub simple_libfunc_name: SimpleLibfuncName,
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
    VersionedCoverageAnnotations::V1(CoverageAnnotationsV1 {
        statements_code_locations,
    }): VersionedCoverageAnnotations,
    VersionedProfilerAnnotations::V1(ProfilerAnnotationsV1 {
        statements_functions,
    }): VersionedProfilerAnnotations,
    filter: &StatementCategoryFilter,
    program: &Program,
) -> SierraToCairoMap {
    let libfuncs_long_ids_by_ids: HashMap<_, _> = program
        .libfunc_declarations
        .iter()
        .map(|libfunc_declaration| (&libfunc_declaration.id, &libfunc_declaration.long_id))
        .collect();

    let libfunc_names_by_idx: HashMap<_, _> = program
        .statements
        .iter()
        .enumerate()
        .filter_map(|(idx, statement)| {
            let simple_libfunc_name = match statement {
                Statement::Invocation(invocation_statement) => libfuncs_long_ids_by_ids
                    .get(&invocation_statement.libfunc_id)?
                    .to_string(),
                Statement::Return(_) => "return".to_string(),
            };
            Some((StatementIdx(idx), SimpleLibfuncName(simple_libfunc_name)))
        })
        .collect();

    SierraToCairoMap(
        statements_code_locations
            .into_iter()
            .filter_map(|(key, code_locations)| {
                let function_names = statements_functions.get(&key)?;
                let simple_libfunc_name = libfunc_names_by_idx.get(&key)?;

                let statement_origin = find_statement_origin(
                    &code_locations,
                    function_names,
                    filter,
                    simple_libfunc_name,
                )?;
                Some((key, statement_origin))
            })
            .collect(),
    )
}

fn find_statement_origin(
    code_locations: &[CodeLocation],
    function_names: &[FunctionName],
    filter: &StatementCategoryFilter,
    libfunc_name: &SimpleLibfuncName,
) -> Option<StatementOrigin> {
    code_locations
        .iter()
        .zip(function_names)
        .map(
            |(CodeLocation(source_file_full_path, line_range), function_name)| StatementOrigin {
                function_name: function_name.clone(),
                source_file_full_path: source_file_full_path.clone(),
                line_range: line_range.into(),
                simple_libfunc_name: libfunc_name.clone(),
            },
        )
        .find(|statement_origin| filter.should_include(statement_origin))
        .map(|mut statement_origin| {
            statement_origin.remove_virtual_file_prefix();
            statement_origin
        })
}
