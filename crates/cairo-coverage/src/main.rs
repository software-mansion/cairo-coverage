use anyhow::Result;
use cairo_coverage_args::CairoCoverageArgs;
use clap::Parser;

fn main() -> Result<()> {
    let cairo_coverage_args = CairoCoverageArgs::parse();

    cairo_coverage_core::run(cairo_coverage_args)
}
