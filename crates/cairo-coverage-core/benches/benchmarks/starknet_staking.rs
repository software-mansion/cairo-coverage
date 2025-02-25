use crate::benchmarks::{config, project_path, trace_files_for_benches};
use cairo_coverage_core::args::RunOptions;
use criterion::{Criterion, criterion_group};
use std::hint::black_box;

const PROJECT_NAME: &str = "starknet-staking";

/// Benchmark of [`starknet_staking`](https://github.com/starkware-libs/starknet-staking) with default options.
/// The trace files should be generated using `download_bench_project.sh` script.
fn starknet_staking_benchmark(c: &mut Criterion) {
    let trace_files = trace_files_for_benches(PROJECT_NAME);
    let project_path = project_path(PROJECT_NAME);
    let run_options = RunOptions::default();
    c.bench_function("starknet-staking", |b| {
        b.iter(|| {
            cairo_coverage_core::run(
                black_box(trace_files.clone()),
                black_box(project_path.clone()),
                black_box(run_options.clone()),
            )
        });
    });
}

criterion_group! {
    name = benches;
    config = config();
    targets = starknet_staking_benchmark
}
