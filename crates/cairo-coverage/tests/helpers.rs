use anyhow::{Context, Result};
use assert_fs::fixture::PathCopy;
use assert_fs::TempDir;
use camino::Utf8PathBuf;
use snapbox::cmd::{cargo_bin, Command as SnapboxCommand, Command};
use std::{env, fs};

#[allow(clippy::missing_errors_doc)]
pub fn run_test_project(test_project_name: &str) -> Result<(Utf8PathBuf, String)> {
    run_test_project_with_args(test_project_name, &[])
}

#[allow(clippy::missing_errors_doc)]
pub fn run_test_project_with_args(
    test_project_name: &str,
    args: &[&str],
) -> Result<(Utf8PathBuf, String)> {
    let temp_dir = TempDir::new().context("Failed to create a temporary directory")?;
    temp_dir
        .copy_from(
            format!("tests/data/{test_project_name}/"),
            &["*.toml", "*.cairo"],
        )
        .context("Failed to copy project files to the temporary directory")?;

    SnapboxCommand::new("snforge")
        .arg("test")
        .arg("--save-trace-data")
        .current_dir(&temp_dir)
        .assert()
        .success();

    let trace_path = temp_dir.path().join("snfoundry_trace");
    fs::read_dir(&trace_path)
        .context("Failed to read the directory for trace files")?
        .map(|entry| {
            entry
                .context("Failed to read a directory entry")
                .map(|e| e.path())
        })
        .collect::<Result<Vec<_>>>()?
        .iter()
        .map(|path| path.display().to_string())
        .fold(
            SnapboxCommand::new(cargo_bin!("cairo-coverage")),
            Command::arg,
        )
        .args(args)
        .current_dir(&temp_dir)
        .assert()
        .success();

    let output_path = temp_dir.path().join("coverage.lcov");

    SnapboxCommand::new("genhtml")
        .arg(&output_path)
        .arg("--output-directory")
        .arg(temp_dir.path())
        .assert()
        .success();

    let temp_dir_path: Utf8PathBuf = temp_dir.path().canonicalize()?.try_into()?;

    fs::read_to_string(&output_path)
        .context("Failed to read the generated `lcov` file")
        .map(|content| (temp_dir_path, content))
}
