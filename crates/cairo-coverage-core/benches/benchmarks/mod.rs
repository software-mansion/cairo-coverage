use cairo_coverage_test_utils::read_files_from_dir;
use camino::Utf8PathBuf;
use criterion::Criterion;
use std::time::Duration;

pub mod starknet_staking;

/// Wrapper around `read_files_from_dir` to make it easier to use in benchmarks.
fn trace_files_for_benches(dir_name: &str) -> Vec<Utf8PathBuf> {
    read_files_from_dir(format!("benches/project-traces/{dir_name}/trace"))
}

/// Return  `project_path` and for the benchmark project.
fn project_path(dir_name: &str) -> Utf8PathBuf {
    Utf8PathBuf::from(format!("benches/project-traces/{dir_name}"))
        .canonicalize_utf8()
        .unwrap()
}

/// Config of [`Criterion`] that should be used for all benchmarks.
/// We lower the sample sizes so benchmarks finish faster and increase
/// the measurement time to get more accurate results and don't get warnings.
fn config() -> Criterion {
    Criterion::default()
        .sample_size(10)
        .measurement_time(Duration::from_secs(200))
}
