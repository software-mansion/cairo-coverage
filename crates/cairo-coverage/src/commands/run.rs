use crate::args::run::{IncludedComponent, RunArgs};
use anyhow::{Context, Result};
use cairo_coverage_core::args::{IncludedComponent as CoreIncludedComponent, RunOptions};
use scarb_metadata::{Metadata, MetadataCommand};
use std::fs::OpenOptions;
use std::io::Write;

/// Run the `cairo-coverage run` command with [`RunArgs`].
/// This is done by calling the [`cairo_coverage_core`] crate and writing the output to the `output_path`.
pub fn run(
    RunArgs {
        include,
        project_path,
        output_path,
        trace_files,
    }: RunArgs,
) -> Result<()> {
    let project_path = if let Some(project_path) = project_path {
        project_path
    } else {
        scarb_metadata()?.workspace.root
    };

    let options = RunOptions {
        include: include.into_iter().map(Into::into).collect(),
    };

    let lcov = cairo_coverage_core::run(trace_files, project_path, options)?;

    OpenOptions::new()
        .append(true)
        .create(true)
        .open(&output_path)
        .context(format!("failed to open output file at path: {output_path}"))?
        .write_all(lcov.as_bytes())
        .context("failed to write to output file")?;

    Ok(())
}

/// Run `scarb metadata` command and return the metadata.
fn scarb_metadata() -> Result<Metadata> {
    MetadataCommand::new()
        .inherit_stderr()
        .exec()
        .context("could not gather project metadata from Scarb due to previous error")
}

impl From<IncludedComponent> for CoreIncludedComponent {
    fn from(component: IncludedComponent) -> Self {
        match component {
            IncludedComponent::TestFunctions => CoreIncludedComponent::TestFunctions,
            IncludedComponent::Macros => CoreIncludedComponent::Macros,
        }
    }
}
