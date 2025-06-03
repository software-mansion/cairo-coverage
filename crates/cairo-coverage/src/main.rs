use crate::args::CairoCoverageArgs;
use anyhow::Result;

use clap::Parser;
use std::process::ExitCode;

mod args;
mod commands;
mod ui;

fn main() -> ExitCode {
    if let Err(error) = main_inner() {
        ui::error(error);
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

fn main_inner() -> Result<()> {
    let CairoCoverageArgs { command } = CairoCoverageArgs::parse();

    commands::run(command)
}
