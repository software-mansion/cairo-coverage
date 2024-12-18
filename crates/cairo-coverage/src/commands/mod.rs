mod run;

use anyhow::Result;
use cairo_coverage_args::Command;

/// Run chosen [`Command`].
pub fn run(command: Command) -> Result<()> {
    match command {
        Command::Run(args) => run::run(args),
    }
}
