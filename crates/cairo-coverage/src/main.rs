mod cli;
mod coverage_data;
mod input;
mod output;
mod types;

use crate::cli::DEFAULT_OUTPUT_NAME;
use crate::coverage_data::create_files_coverage_data_with_hits;
use crate::input::InputData;
use crate::output::lcov::LcovFormat;
use anyhow::{Context, Result};
use clap::Parser;
use cli::Cli;
use std::fs::OpenOptions;
use std::io::Write;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let output_path = cli
        .output_path
        .unwrap_or_else(|| format!("./{DEFAULT_OUTPUT_NAME}.lcov").into());

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&output_path)
        .context(format!("Failed to open output file at path: {output_path}"))?;

    for trace_file in &cli.trace_files {
        let input_data = InputData::try_from(trace_file)?;
        let coverage_data = create_files_coverage_data_with_hits(&input_data);
        let output_data = LcovFormat::from(coverage_data);

        file.write_all(output_data.to_string().as_bytes())
            .context("Failed to write to output file")?;
    }

    Ok(())
}
