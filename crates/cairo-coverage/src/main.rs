use crate::args::{CairoCoverageArgs, Command};
use anyhow::Result;
use clap::Parser;

mod args;
mod commands;

fn main() -> Result<()> {
    let cairo_coverage_args = CairoCoverageArgs::parse();

    let command = match cairo_coverage_args.command {
        Some(command) => command,
        // TODO:
        // * In 0.5.0 add deprecation warning
        // * In 0.6.0 remove the default command
        None => Command::Run(cairo_coverage_args.run_args.unwrap_or_else(|| {
            unreachable!("`run_args` should be set when no subcommand is provided")
        })),
    };

    commands::run(command)
}
