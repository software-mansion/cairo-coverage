use crate::args::{CairoCoverageArgs, Command};
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
    let cairo_coverage_args = CairoCoverageArgs::parse();

    // TODO:
    // * In 0.6.0 remove the default command
    let command = if let Some(command) = cairo_coverage_args.command {
        command
    } else {
        ui::warning("running `cairo-coverage` without a subcommand is deprecated");
        ui::help("consider using `cairo-coverage run`");

        Command::Run(cairo_coverage_args.run_args.unwrap_or_else(|| {
            unreachable!("`run_args` should be set when no subcommand is provided")
        }))
    };

    commands::run(command)
}
