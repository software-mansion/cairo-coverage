mod cli;

use crate::cli::DEFAULT_OUTPUT_NAME;
use anyhow::{Context, Result};
use clap::Parser;
use cli::Cli;
use std::fs::File;
use std::io::Write;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let output_path = cli
        .output_path
        .unwrap_or_else(|| format!("./{DEFAULT_OUTPUT_NAME}.lcov").into());

    File::create(&output_path)
        .context(format!(
            "Failed to create output file at path: {output_path}"
        ))?
        .write_all(b"")
        .context("Failed to write to output file")?;

    Ok(())
}
