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
