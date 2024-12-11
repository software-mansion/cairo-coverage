use crate::run::RunArgs;
use clap::{Parser, Subcommand};

pub mod run;

#[derive(Parser, Debug)]
#[command(version, args_conflicts_with_subcommands = true)]
pub struct CairoCoverageArgs {
    /// Arguments for the `run` subcommand so user can use
    /// `cairo-coverage` without specifying the subcommand.
    #[clap(flatten)]
    pub run_args: RunArgs,
    /// Subcommand and its arguments.
    #[command(subcommand)]
    pub command: Option<Command>,
}

/// Subcommand and its arguments.
#[derive(Subcommand, Debug)]
pub enum Command {
    /// Run `cairo-coverage` tool.
    Run(RunArgs),
}
