mod cli;
mod coverage_data;
mod data_loader;
mod input;
mod output;
mod types;

use crate::coverage_data::create_files_coverage_data_with_hits;
use crate::data_loader::LoadedDataMap;
use crate::input::InputData;
use crate::output::lcov::LcovFormat;
use anyhow::{Context, Result};
use clap::Parser;
use cli::Cli;
use std::fs::OpenOptions;
use std::io::Write;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let output_path = &cli.output_path;

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(output_path)
        .context(format!("Failed to open output file at path: {output_path}"))?;

    let loaded_data = LoadedDataMap::load(&cli.trace_files)?;
    for (_, loaded_data) in loaded_data.iter() {
        let input_data = InputData::new(loaded_data, cli.include_test_functions)?;
        let coverage_data = create_files_coverage_data_with_hits(&input_data);
        let output_data = LcovFormat::from(coverage_data);

        file.write_all(output_data.to_string().as_bytes())
            .context("Failed to write to output file")?;
    }

    Ok(())
}
