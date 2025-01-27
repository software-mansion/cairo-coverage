pub mod args;
mod build;
mod coverage;
mod loading;
mod merge;
mod output;
mod types;

use crate::args::RunOptions;
use crate::build::coverage_input;
use crate::build::filter::{ignore_matcher, statement_category_filter};
use crate::loading::execution_data;
use crate::output::lcov::LcovFormat;
use anyhow::{Context, Result};
use camino::Utf8PathBuf;
use merge::MergeOwned;
use scarb_metadata::{Metadata, MetadataCommand};

/// Run the core logic of `cairo-coverage` with the provided trace files and [`RunOptions`].
/// This function generates a coverage report in the LCOV format.
/// # Errors
/// Fails if it can't produce the coverage report with the error message explaining the reason.
#[allow(clippy::needless_pass_by_value)] // In case if we ever needed to take ownership of the arguments.
pub fn run(
    trace_files: Vec<Utf8PathBuf>,
    RunOptions {
        include,
        project_path,
    }: RunOptions,
) -> Result<LcovFormat> {
    let project_path = if let Some(project_path) = project_path {
        project_path
    } else {
        scarb_metadata()?.workspace.root
    };

    let ignore_matcher = ignore_matcher::build(&project_path)?;

    let coverage_data = execution_data::load(&trace_files)?
        .into_iter()
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
        // Versioned programs and contract classes can represent the same piece of code,
        // so we merge the file coverage after processing them to avoid duplicate entries.
        .reduce(MergeOwned::merge_owned)
        .context("No elements to merge")?;

    Ok(LcovFormat::from(coverage_data))
}

/// Run `scarb metadata` command and return the metadata.
fn scarb_metadata() -> Result<Metadata> {
    MetadataCommand::new()
        .inherit_stderr()
        .inherit_stdout()
        .exec()
        .context("error: could not gather project metadata from Scarb due to previous error")
}
