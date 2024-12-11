use anyhow::{Context, Result};
use cairo_coverage_args::clean::CleanArgs;
use std::fs;
use walkdir::WalkDir;

/// Run the `cairo-coverage clean` command with [`CleanArgs`].
/// This command deletes all files with the name specified in `files_to_delete` in the `root_dir` recursively.
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
