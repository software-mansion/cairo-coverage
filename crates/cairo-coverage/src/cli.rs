use anyhow::{ensure, Result};
use camino::Utf8PathBuf;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
pub struct Cli {
    /// Paths to the .json files with trace data.
    #[arg(value_parser = parse_trace_file, num_args = 1.., required = true)]
    pub trace_files: Vec<Utf8PathBuf>,

    /// Path to the output file.
    #[arg(short, long, default_value = "coverage.lcov")]
    pub output_path: Utf8PathBuf,

    /// Run coverage on functions marked with `#[test]` attribute.
    ///
    /// [default: false]
    ///
    /// Note: We currently recommend setting this to false as there
    /// might be issues with mappings for `#[test]` attribute.
    #[arg(long, default_value_t = false)]
    pub include_test_functions: bool,
}

fn parse_trace_file(path: &str) -> Result<Utf8PathBuf> {
    let trace_file = Utf8PathBuf::from(path);

    ensure!(trace_file.exists(), "Trace file does not exist");
    ensure!(trace_file.is_file(), "Trace file is not a file");
    ensure!(
        matches!(trace_file.extension(), Some("json")),
        "Trace file must have a JSON extension"
    );

    Ok(trace_file)
}
