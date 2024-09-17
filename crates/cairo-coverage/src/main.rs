mod cli;
mod coverage_data;
mod data_loader;
mod input;
mod merge;
mod output;
mod types;

use crate::coverage_data::create_files_coverage_data_with_hits;
use crate::data_loader::LoadedDataMap;
use crate::input::{InputData, StatementCategoryFilter};
use crate::output::lcov::LcovFormat;
use anyhow::{bail, ensure, Context, Result};
use camino::Utf8PathBuf;
use clap::Parser;
use cli::Cli;
use indoc::indoc;
use merge::MergeOwned;
use std::fs::OpenOptions;
use std::io::Write;

const SNFORGE_SIERRA_DIR: &str = ".snfoundry_versioned_programs";

fn main() -> Result<()> {
    let Cli {
        trace_files,
        include,
        output_path,
        project_path,
    } = &Cli::parse();

    let coverage_data = LoadedDataMap::load(trace_files)?
        .iter()
        .map(|(source_sierra_path, loaded_data)| {
            let project_path = &get_project_path(source_sierra_path, project_path)?;
            let filter = StatementCategoryFilter::new(project_path, include, loaded_data);
            let input_data = InputData::new(loaded_data, &filter)?;
            Ok(create_files_coverage_data_with_hits(&input_data))
        })
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        // Versioned programs and contract classes can represent the same piece of code,
        // so we merge the file coverage after processing them to avoid duplicate entries.
        .reduce(MergeOwned::merge_owned)
        .context("No elements to merge")?;

    OpenOptions::new()
        .append(true)
        .create(true)
        .open(output_path)
        .context(format!("Failed to open output file at path: {output_path}"))?
        .write_all(LcovFormat::from(coverage_data).to_string().as_bytes())
        .context("Failed to write to output file")?;

    Ok(())
}

fn get_project_path(
    source_sierra_path: &Utf8PathBuf,
    project_path: &Option<Utf8PathBuf>,
) -> Result<Utf8PathBuf> {
    if let Some(project_path) = project_path {
        Ok(project_path.clone())
    } else {
        find_user_project_path(source_sierra_path).context(indoc! {
            r"Inference of project path failed.
            Please provide the project path explicitly using the --project-path flag.
            If you are using snforge, it is not possible to use cairo-coverage flags.
            You need to run cairo-coverage directly."
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
            source_sierra_path
                .parent()
                .filter(|parent| parent.file_name() == Some(SNFORGE_SIERRA_DIR))
                .and_then(|parent| parent.parent())
                .map(Utf8PathBuf::from)
                .context(format!(
                    "Source sierra path should be in the format: <project_root>/{SNFORGE_SIERRA_DIR}/<file>.sierra.json, got: {source_sierra_path}"
                ))
        }
        Some("contract_class") => {
            source_sierra_path
                .parent()
                .filter(|parent| parent.file_name() == Some("dev"))
                .and_then(|parent| parent.parent())
                .filter(|parent| parent.file_name() == Some("target"))
                .and_then(|parent| parent.parent())
                .map(Utf8PathBuf::from)
                .context(format!(
                    "Source sierra path should be in the format: <project_root>/target/dev/<file>.contract_class.json, got: {source_sierra_path}"
                ))
        }
        _ => bail!(
            "Source sierra path should have a .sierra or .contract_class extension, got: {source_sierra_path}"
        ),
    }
}
