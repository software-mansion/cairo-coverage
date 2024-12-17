use crate::args::clean::CleanArgs;
use anyhow::{Context, Result};
use std::fs;
use walkdir::WalkDir;

/// Run the `cairo-coverage clean` command with [`CleanArgs`].
/// This command deletes all files with the name specified by `files_to_delete` in the `root_dir` and all its subdirectories.
pub fn run(
    CleanArgs {
        root_dir,
        files_to_delete,
    }: CleanArgs,
) -> Result<()> {
    let target_file_name = files_to_delete
        .file_name()
        .context("Failed to obtain the file name from `files_to_delete`.")?;

    WalkDir::new(root_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .try_for_each(|entry| -> Result<()> {
            let path = entry.path();

            if let Some(file_name) = path.file_name() {
                if file_name == target_file_name {
                    println!("Deleting file: {}", path.display());
                    fs::remove_file(path)
                        .with_context(|| format!("Failed to delete file: {}", path.display()))?;
                }
            }

            Ok(())
        })?;

    println!("Cleanup complete.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::super::clean;
    use super::CleanArgs;
    use assert_fs::fixture::PathChild;
    use assert_fs::TempDir;
    use cairo_coverage_test_utils::{CreateFile, Utf8PathBufConversion};

    #[test]
    fn test_clean_removes_target_files() {
        let temp_dir = TempDir::new().unwrap();

        let sub_dir = temp_dir.child("sub_dir");
        let sub_sub_dir = sub_dir.child("nested_dir");

        let file_to_delete_1 = temp_dir.create_file("coverage.lcov");
        let file_to_delete_2 = sub_dir.create_file("coverage.lcov");
        let file_to_delete_3 = sub_sub_dir.create_file("coverage.lcov");
        let non_target_file = temp_dir.create_file("keep_this.txt");

        let clean_args = CleanArgs {
            root_dir: temp_dir.to_utf8_path_buf(),
            files_to_delete: "coverage.lcov".into(),
        };

        clean::run(clean_args).unwrap();

        assert!(!file_to_delete_1.exists());
        assert!(!file_to_delete_2.exists());
        assert!(!file_to_delete_3.exists());
        assert!(non_target_file.exists());
    }

    #[test]
    fn test_clean_does_nothing_for_nonexistent_files() {
        let temp_dir = TempDir::new().unwrap();

        let sub_dir = temp_dir.child("sub_dir");
        let file_1 = temp_dir.create_file("some_file.txt");
        let file_2 = sub_dir.create_file("another_file.rs");

        let clean_args = CleanArgs {
            root_dir: temp_dir.to_utf8_path_buf(),
            files_to_delete: "nonexistent_file.txt".into(),
        };

        clean::run(clean_args).unwrap();

        assert!(file_1.exists());
        assert!(file_2.exists());
    }

    #[test]
    fn test_clean_handles_empty_directory() {
        let temp_dir = TempDir::new().unwrap();

        let clean_args = CleanArgs {
            root_dir: temp_dir.to_utf8_path_buf(),
            files_to_delete: "coverage.lcov".into(),
        };

        clean::run(clean_args).unwrap();

        // No assertion needed—just ensuring no panics or errors occur
    }
}
