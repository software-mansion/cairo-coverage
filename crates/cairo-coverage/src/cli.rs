use anyhow::{ensure, Result};
use camino::Utf8PathBuf;
use clap::Parser;

pub const DEFAULT_OUTPUT_NAME: &str = "coverage";

#[derive(Parser, Debug)]
#[command(version)]
pub struct Cli {
    /// Paths to the .json files with trace data.
    #[arg(value_parser = parse_trace_file, num_args = 1.., required = true)]
    pub trace_files: Vec<Utf8PathBuf>,

    /// Path to the output file. [default: `coverage.lcov`]
    #[arg(short, long)]
    pub output_path: Option<Utf8PathBuf>,
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
