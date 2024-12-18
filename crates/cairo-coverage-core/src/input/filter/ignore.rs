use crate::input::filter::statement_category_filter::VIRTUAL_FILE_REGEX;
use camino::{Utf8Path, Utf8PathBuf};
use ignore::gitignore::Gitignore;
use ignore::Match;

const CAIRO_COVERAGE_IGNORE: &str = ".cairo-coverage-ignore";

pub struct CairoCoverageIgnoreMatcher(Gitignore);

impl CairoCoverageIgnoreMatcher {
    /// Create a new instance of the [`CairoCoverageIgnoreMatcher`] that will be based on the [`CAIRO_COVERAGE_IGNORE`] file.
    pub fn new(path: &Utf8Path) -> anyhow::Result<Self> {
        let ignore_matcher = find_ignore_file(path)
            .map(Gitignore::new)
            .map(|(ignore, error)| {
                if let Some(error) = error {
                    Err(anyhow::Error::from(error))
                } else {
                    Ok(ignore)
                }
            })
            .transpose()?
            .unwrap_or_else(Gitignore::empty);

        Ok(Self(ignore_matcher))
    }

    /// Check if the given path is ignored by the [`CAIRO_COVERAGE_IGNORE`] file.
    pub fn is_ignored(&self, path: &str) -> bool {
        let path: Utf8PathBuf = VIRTUAL_FILE_REGEX.replace_all(path, "").to_string().into();
        let result = self.0.matched(&path, path.is_dir());
        matches!(result, Match::Ignore(_))
    }
}

/// Search for a [`CAIRO_COVERAGE_IGNORE`] file from the given directory and its parents until the root.
fn find_ignore_file(start_dir: &Utf8Path) -> Option<Utf8PathBuf> {
    let mut current_dir = Some(start_dir);

    while let Some(dir) = current_dir {
        let candidate = dir.join(CAIRO_COVERAGE_IGNORE);
        if candidate.is_file() {
            return Some(candidate);
        }
        current_dir = dir.parent();
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_fs::fixture::PathChild;
    use assert_fs::TempDir;
    use cairo_coverage_test_utils::{CreateFile, Utf8PathBufConversion};

    #[test]
    fn test_finds_ignore_file_in_same_directory() {
        let temp_dir = TempDir::new().unwrap();
        let ignore_file = temp_dir
            .create_file(CAIRO_COVERAGE_IGNORE)
            .to_utf8_path_buf();

        let result = find_ignore_file(&ignore_file);

        assert_eq!(result, Some(ignore_file));
    }

    #[test]
    fn test_finds_ignore_file_in_parent_directory() {
        let temp_dir = TempDir::new().unwrap();
        let parent_dir = temp_dir.child("parent");
        let child_dir = parent_dir.child("child");

        let ignore_file = parent_dir
            .create_file(CAIRO_COVERAGE_IGNORE)
            .to_utf8_path_buf();

        let result = find_ignore_file(&child_dir.to_utf8_path_buf());

        assert_eq!(result, Some(ignore_file));
    }

    #[test]
    fn test_finds_ignore_file_multiple_levels_up() {
        let temp_dir = TempDir::new().unwrap();
        let root_dir = temp_dir.child("root");
        let middle_dir = root_dir.child("middle");
        let child_dir = middle_dir.child("child");

        let ignore_file = root_dir
            .create_file(CAIRO_COVERAGE_IGNORE)
            .to_utf8_path_buf();

        let result = find_ignore_file(&child_dir.to_utf8_path_buf());

        assert_eq!(result, Some(ignore_file));
    }

    #[test]
    fn test_no_ignore_file_found() {
        let temp_dir = TempDir::new().unwrap();

        let result = find_ignore_file(&temp_dir.to_utf8_path_buf());

        assert_eq!(result, None);
    }

    #[test]
    fn test_stops_at_first_ignore_file() {
        let temp_dir = TempDir::new().unwrap();
        let root_dir = temp_dir.child("root");
        let middle_dir = root_dir.child("middle");
        let child_dir = middle_dir.child("child");

        root_dir
            .create_file(CAIRO_COVERAGE_IGNORE)
            .to_utf8_path_buf();
        let middle_ignore_file = middle_dir
            .create_file(CAIRO_COVERAGE_IGNORE)
            .to_utf8_path_buf();

        let result = find_ignore_file(&child_dir.to_utf8_path_buf());

        assert_eq!(result, Some(middle_ignore_file));
    }
}
