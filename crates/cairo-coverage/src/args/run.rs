use anyhow::{Result, ensure};
use camino::Utf8PathBuf;
use clap::{Parser, ValueEnum};

/// Arguments accepted by the `run` subcommand.
#[derive(Parser, Debug)]
pub struct RunArgs {
    /// Paths to the .json files with trace data.
    #[arg(value_parser = parse_trace_file, num_args = 1.., required = true)]
    pub trace_files: Vec<Utf8PathBuf>,

    /// Path to the output file.
    #[arg(short, long, default_value = "coverage.lcov")]
    pub output_path: Utf8PathBuf,

    /// Include additional components in the coverage report.
    #[arg(long, short, num_args = 0.., default_value = "macros")]
    pub include: Vec<IncludedComponent>,

    /// Path to the project directory. If not provided, the project directory is inferred using `scarb metadata`.
    #[arg(value_parser = parse_project_path, long)]
    pub project_path: Option<Utf8PathBuf>,
}

/// Additional components that can be included in the coverage report.
#[derive(ValueEnum, Debug, Clone, Eq, PartialEq)]
pub enum IncludedComponent {
    /// Run coverage on functions marked with `#[test]` attribute
    TestFunctions,
    /// Run coverage on macros and generated code by them. This includes inline macros, attribute macros, and derive macros.
    Macros,
}

fn parse_trace_file(path: &str) -> Result<Utf8PathBuf> {
    let trace_file = Utf8PathBuf::from(path);

    ensure!(trace_file.exists(), "trace file does not exist");
    ensure!(trace_file.is_file(), "trace file is not a file");
    ensure!(
        matches!(trace_file.extension(), Some("json")),
        "trace file must have a JSON extension"
    );

    Ok(trace_file)
}

fn parse_project_path(path: &str) -> Result<Utf8PathBuf> {
    let project_path = Utf8PathBuf::from(path);

    ensure!(project_path.exists(), "project path does not exist");
    ensure!(project_path.is_dir(), "project path is not a directory");

    Ok(project_path)
}
