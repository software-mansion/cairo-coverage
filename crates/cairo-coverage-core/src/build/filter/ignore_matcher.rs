use anyhow::{Error, Result};
use camino::{Utf8Path, Utf8PathBuf};
use ignore::Match;
use ignore::gitignore::Gitignore;

const CAIRO_COVERAGE_IGNORE: &str = ".cairo-coverage-ignore";

/// Create a new instance of the [`CairoCoverageIgnoreMatcher`] that will be based on the [`CAIRO_COVERAGE_IGNORE`] file.
pub fn build(path: &Utf8Path) -> Result<CairoCoverageIgnoreMatcher> {
    let ignore_matcher = find_ignore_file(path)
        .map(Gitignore::new)
        .map(|(ignore, error)| {
            if let Some(error) = error {
                Err(Error::from(error))
            } else {
                Ok(ignore)
            }
        })
        .transpose()?
        .unwrap_or_else(Gitignore::empty);

    Ok(CairoCoverageIgnoreMatcher(ignore_matcher))
}

/// Search for a [`CAIRO_COVERAGE_IGNORE`] file in the given directory.
fn find_ignore_file(dir: &Utf8Path) -> Option<Utf8PathBuf> {
    let candidate = dir.join(CAIRO_COVERAGE_IGNORE);
    candidate.is_file().then_some(candidate)
}

pub struct CairoCoverageIgnoreMatcher(Gitignore);

impl CairoCoverageIgnoreMatcher {
    /// Check if the given path is ignored by the [`CAIRO_COVERAGE_IGNORE`] file.
    pub fn is_ignored(&self, path: &str) -> bool {
        let path: Utf8PathBuf = path.to_string().into();
        let result = self.0.matched(&path, path.is_dir());
        matches!(result, Match::Ignore(_))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_fs::TempDir;
    use cairo_coverage_test_utils::{CreateFile, Utf8PathBufConversion};

    #[test]
    fn test_finds_ignore_file_in_same_directory() {
        let temp_dir = TempDir::new().unwrap();
        let ignore_file = temp_dir
            .create_file(CAIRO_COVERAGE_IGNORE)
            .to_utf8_path_buf();

        let result = find_ignore_file(&temp_dir.to_utf8_path_buf());

        assert_eq!(result, Some(ignore_file));
    }

    #[test]
    fn test_no_ignore_file_found() {
        let temp_dir = TempDir::new().unwrap();

        let result = find_ignore_file(&temp_dir.to_utf8_path_buf());

        assert_eq!(result, None);
    }
}
