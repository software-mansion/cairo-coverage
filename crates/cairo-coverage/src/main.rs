mod cli;
mod inner;

use anyhow::{Context, Result};
use clap::Parser;
use cli::Cli;
use std::fs::File;
use std::io::Write;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let output_path = cli.output_path.unwrap_or_else(|| {
        let output_file_name = cli.trace_file.file_stem().unwrap(); // Safe to unwrap since we checked that the file exists
        format!("./{output_file_name}.lcov").into()
    });

    File::create(&output_path)
        .context(format!(
            "Failed to create output file at path: {output_path}"
        ))?
        .write_all(b"")
        .context("Failed to write to output file")?;

    Ok(())
}
