mod clean;
mod run;

use crate::args::Command;
use anyhow::Result;

/// Run chosen [`Command`].
pub fn run(command: Command) -> Result<()> {
    match command {
        Command::Clean(args) => clean::run(args),
        Command::Run(args) => run::run(args),
    }
}
