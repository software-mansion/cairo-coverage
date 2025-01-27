use cairo_coverage_core::args::RunOptions;
use cairo_coverage_test_utils::read_files_from_dir;
use camino::Utf8PathBuf;
use criterion::Criterion;
use std::time::Duration;

pub mod starknet_staking;

/// Wrapper around `read_files_from_dir` to make it easier to use in benchmarks.
fn trace_files_for_benches(dir_name: &str) -> Vec<Utf8PathBuf> {
    read_files_from_dir(format!("benches/project-traces/{dir_name}/trace"))
}

/// Create [`RunOptions`] with set `project_path` and empty `include`.
/// If we leave the `project_path` as `None`, the `scarb_metadata` will fail.
fn run_options(dir_name: &str) -> RunOptions {
    let project_path = Utf8PathBuf::from(format!("benches/project-traces/{dir_name}"))
        .canonicalize_utf8()
        .unwrap()
        .into();
    RunOptions {
        include: Vec::default(),
        project_path,
    }
}

/// Config of [`Criterion`] that should be used for all benchmarks.
/// We lower the sample sizes so benchmarks finish faster and increase
/// the measurement time to get more accurate results and don't get warnings.
fn config() -> Criterion {
    Criterion::default()
        .sample_size(10)
        .measurement_time(Duration::from_secs(200))
}
