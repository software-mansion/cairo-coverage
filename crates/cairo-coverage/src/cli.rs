use anyhow::{ensure, Result};
use camino::Utf8PathBuf;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
pub struct Cli {
    /// Path to the .json file with trace data.
    #[arg(value_parser = parse_trace_file)]
    pub trace_file: Utf8PathBuf,

    /// Path to the output file. [default: `<TRACE_NAME>.lcov`]
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
