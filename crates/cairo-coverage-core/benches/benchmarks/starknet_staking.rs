use crate::benchmarks::{config, trace_files_for_benches};
use cairo_coverage_core::args::RunOptions;
use criterion::{criterion_group, Criterion};
use std::hint::black_box;

/// Benchmark of [`starknet_staking`](https://github.com/starkware-libs/starknet-staking) with default options.
/// The trace files should be generated using `download_bench_project.sh` script.
fn starknet_staking_benchmark(c: &mut Criterion) {
    let trace_files = trace_files_for_benches("starknet-staking");
    let run_options = RunOptions::default();
    c.bench_function("starknet-staking default options", |b| {
        b.iter(|| {
            cairo_coverage_core::run(
                black_box(trace_files.clone()),
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
