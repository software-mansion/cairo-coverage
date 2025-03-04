pub mod args;
mod build;
mod coverage;
mod hashmap_utils;
mod loading;
mod output;

use crate::args::RunOptions;
use crate::build::coverage_input;
use crate::build::filter::ignore_matcher;
use crate::build::filter::statement_category_filter;
use crate::hashmap_utils::merge::merge;
use crate::loading::execution_data;
use crate::output::lcov;
use anyhow::{Context, Result};
use camino::Utf8PathBuf;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

/// Run the core logic of `cairo-coverage` with the provided trace files, project path and [`RunOptions`].
/// This function generates a coverage report in the LCOV format.
/// # Errors
/// Fails if it can't produce the coverage report with the error message explaining the reason.
#[expect(clippy::needless_pass_by_value)] // In case if we ever needed to take ownership of the arguments.
pub fn run(
    trace_files: Vec<Utf8PathBuf>,
    project_path: Utf8PathBuf,
    RunOptions {
        include,
        no_truncation,
    }: RunOptions,
) -> Result<String> {
    let ignore_matcher = ignore_matcher::build(&project_path)?;

    let mut project_coverage = execution_data::load(&trace_files)?
        .into_par_iter()
        .map(|execution_data| {
            let filter = statement_category_filter::build(
                &project_path,
                &include,
                &ignore_matcher,
                &execution_data.enriched_program,
            );

            coverage_input::build(execution_data, &filter)
        })
        .map(coverage::project::create)
        .collect::<Vec<_>>()
        .into_iter()
        // Versioned programs and contract classes can represent the same piece of code,
        // so we merge the file coverage after processing them to avoid duplicate entries.
        .reduce(merge)
        .context("at least one trace file must be provided")?;

    if !no_truncation {
        coverage::project::truncate_to_one(&mut project_coverage);
    }

    Ok(lcov::fmt_string(&project_coverage))
}
