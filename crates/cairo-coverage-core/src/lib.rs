pub mod args;
mod coverage_data;
mod data_loader;
mod input;
mod merge;
mod output;
mod types;

use crate::args::RunOptions;
use crate::coverage_data::create_files_coverage_data_with_hits;
use crate::data_loader::LoadedDataMap;
use crate::input::{InputData, StatementCategoryFilter};
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

    let coverage_data = LoadedDataMap::load(&trace_files)?
        .iter()
        .map(|(_, loaded_data)| {
            let filter = StatementCategoryFilter::new(&project_path, &include, loaded_data);
            let input_data = InputData::new(loaded_data, &filter)?;
            Ok(create_files_coverage_data_with_hits(&input_data))
        })
        .collect::<Result<Vec<_>>>()?
        .into_iter()
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
