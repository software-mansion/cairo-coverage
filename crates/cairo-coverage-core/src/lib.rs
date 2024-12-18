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
use anyhow::{bail, ensure, Context, Result};
use camino::Utf8PathBuf;
use indoc::indoc;
use merge::MergeOwned;

const SNFORGE_SIERRA_DIR: &str = ".snfoundry_versioned_programs";

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
    let coverage_data = LoadedDataMap::load(&trace_files)?
        .iter()
        .map(|(source_sierra_path, loaded_data)| {
            let project_path = &get_project_path(source_sierra_path, project_path.as_ref())?;
            let filter = StatementCategoryFilter::new(project_path, &include, loaded_data);
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

fn get_project_path(
    source_sierra_path: &Utf8PathBuf,
    project_path: Option<&Utf8PathBuf>,
) -> Result<Utf8PathBuf> {
    if let Some(project_path) = project_path {
        Ok(project_path.clone())
    } else {
        find_user_project_path(source_sierra_path).context(indoc! {
            r"Inference of project path failed.
            Please provide the project path explicitly using the --project-path flag."
        })
    }
}

fn find_user_project_path(source_sierra_path: &Utf8PathBuf) -> Result<Utf8PathBuf> {
    ensure!(
        source_sierra_path.extension() == Some("json"),
        "Source sierra path should have a .json extension, got: {source_sierra_path}"
    );

    match source_sierra_path.with_extension("").extension() {
        Some("sierra") => {
            navigate_and_check(source_sierra_path, &["target", "dev"])
                .or_else(|| navigate_and_check(source_sierra_path, &[SNFORGE_SIERRA_DIR]))
                .context(format!(
                    "Source sierra path should be in one of the formats: \
                    <project_root>/{SNFORGE_SIERRA_DIR}/<file>.sierra.json \
                    or <project_root>/target/dev/<file>.sierra.json, got: {source_sierra_path}"
                ))
        }
        Some("contract_class") => {
            navigate_and_check(source_sierra_path, &["target", "dev"])
                .context(format!(
                    "Source sierra path should be in the format: \
                    <project_root>/target/dev/<file>.contract_class.json, got: {source_sierra_path}"
                ))
        }
        _ => bail!(
            "Source sierra path should have a .sierra or .contract_class extension, got: {source_sierra_path}"
        ),
    }
}

fn navigate_and_check(path: &Utf8PathBuf, folders: &[&str]) -> Option<Utf8PathBuf> {
    folders
        .iter()
        .rev()
        .try_fold(path.parent()?, |current, &folder| {
            current
                .file_name()
                .filter(|name| *name == folder)
                .map(|_| current.parent())?
        })
        .map(Utf8PathBuf::from)
}
