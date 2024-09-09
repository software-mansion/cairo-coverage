use anyhow::{Context, Result};
use assert_fs::fixture::PathCopy;
use assert_fs::TempDir;
use camino::Utf8PathBuf;
use snapbox::cmd::{cargo_bin, Command as SnapboxCommand};
use std::fs;
use std::path::PathBuf;

pub struct TestProject {
    dir: TempDir,
}

impl TestProject {
    pub fn new(test_project_name: &str) -> Result<Self> {
        let dir = TempDir::new().context("Failed to create a temporary directory")?;
        dir.copy_from(
            format!("tests/data/{test_project_name}/"),
            &["*.toml", "*.cairo"],
        )
        .context("Failed to copy project files to the temporary directory")?;
        Ok(Self { dir })
    }

    fn generate_trace_files(self) -> Self {
        SnapboxCommand::new("snforge")
            .arg("test")
            .arg("--save-trace-data")
            .current_dir(&self.dir)
            .assert()
            .success();
        self
    }

    fn find_trace_files(&self) -> Result<Vec<String>> {
        let trace_path = self.dir.path().join("snfoundry_trace");
        fs::read_dir(&trace_path)
            .context("Failed to read the directory for trace files")?
            .map(|entry| {
                entry
                    .context("Failed to read a directory entry")
                    .map(|e| e.path().display().to_string())
            })
            .collect()
    }

    fn output_lcov_path(&self) -> PathBuf {
        self.dir.path().join("coverage.lcov")
    }

    fn run_coverage(self, args: &[&str]) -> Result<Self> {
        let trace_files = self.find_trace_files()?;
        SnapboxCommand::new(cargo_bin!("cairo-coverage"))
            .args(&trace_files)
            .args(args)
            .current_dir(&self.dir)
            .assert()
            .success();
        Ok(self)
    }

    fn run_genhtml(self) -> Self {
        SnapboxCommand::new("genhtml")
            .arg(self.output_lcov_path())
            .arg("--output-directory")
            .arg(self.dir.path())
            .assert()
            .success();
        self
    }

    fn output(self) -> Result<CoverageTestOutput> {
        let content = fs::read_to_string(self.output_lcov_path())
            .context("Failed to read the generated `lcov` file")?;
        let dir = self.dir.path().canonicalize()?.try_into()?;
        Ok(CoverageTestOutput { dir, content })
    }
    pub fn run(test_project_name: &str) -> CoverageTestOutput {
        Self::run_with_args(test_project_name, &[])
    }

    pub fn run_with_args(test_project_name: &str, args: &[&str]) -> CoverageTestOutput {
        Self::new(test_project_name)
            .unwrap()
            .generate_trace_files()
            .run_coverage(args)
            .unwrap()
            .run_genhtml()
            .output()
            .unwrap()
    }
}

pub struct CoverageTestOutput {
    pub dir: Utf8PathBuf,
    pub content: String,
}

impl CoverageTestOutput {
    pub fn output_same_as_in_file(&self, expected_file: &str) {
        let expected = fs::read_to_string(format!("tests/expected_output/{expected_file}"))
            .unwrap()
            .replace("{dir}", self.dir.as_ref());
        assert_eq!(self.content, expected);
    }
}
