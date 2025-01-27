use crate::args::run::{IncludedComponent, RunArgs};
use anyhow::{Context, Result};
use cairo_coverage_core::args::{IncludedComponent as CoreIncludedComponent, RunOptions};
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
    let options = RunOptions {
        include: include.into_iter().map(Into::into).collect(),
        project_path,
    };

    let lcov = cairo_coverage_core::run(trace_files, options)?;

    OpenOptions::new()
        .append(true)
        .create(true)
        .open(&output_path)
        .context(format!("Failed to open output file at path: {output_path}"))?
        .write_all(lcov.as_bytes())
        .context("Failed to write to output file")?;

    Ok(())
}

impl From<IncludedComponent> for CoreIncludedComponent {
    fn from(component: IncludedComponent) -> Self {
        match component {
            IncludedComponent::TestFunctions => CoreIncludedComponent::TestFunctions,
            IncludedComponent::Macros => CoreIncludedComponent::Macros,
        }
    }
}
