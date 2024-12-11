use anyhow::Result;
use cairo_coverage_args::run::RunArgs;

/// Run the `cairo-coverage run` command with [`RunArgs`].
/// This is done by calling the [`cairo_coverage_core`] crate.
pub fn run(args: RunArgs) -> Result<()> {
    cairo_coverage_core::run(args)
}
