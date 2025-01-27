use crate::build::coverage_input::CoverageInput;
use crate::build::statement_information::LineRange;
use crate::coverage::file::FileCoverage;
use crate::coverage::function::FunctionCoverage;
use cairo_annotations::annotations::coverage::SourceFileFullPath;
use std::collections::HashMap;

/// Mapping of [`SourceFileFullPath`] to [`FileCoverage`].
/// This is used to represent the coverage of a single project.
pub type ProjectCoverage = HashMap<SourceFileFullPath, FileCoverage>;

/// Creates a [`ProjectCoverage`] from the given [`CoverageInput`].
pub fn create(coverage_input: CoverageInput) -> ProjectCoverage {
    coverage_input.statement_information_map.into_iter().fold(
        ProjectCoverage::new(),
        |mut acc, (id, origin)| {
            let function_details = acc
                .entry(origin.source_file_full_path)
                .or_default()
                .entry(origin.function_name)
                .or_default();

            let executed_statement_count = coverage_input
                .executed_statement_count
                .get(&id)
                .copied()
                .unwrap_or_default();

            register_line_execution(
                function_details,
                &origin.line_range,
                executed_statement_count,
            );

            acc
        },
    )
}

/// Registers the execution of sierra statements in the given [`FunctionCoverage`].
fn register_line_execution(
    function_coverage: &mut FunctionCoverage,
    line_range: &LineRange,
    executed_statement_count: usize,
) {
    for line in line_range {
        *function_coverage.entry(line).or_default() += executed_statement_count;
    }
}
