use assert_fs::fixture::{ChildPath, FileTouch, PathChild};
use camino::Utf8PathBuf;
use std::fs;
use std::path::Path;

/// Function to read all files from a directory and return their paths as [`Utf8PathBuf`].
/// # Panics
/// - if [`fs::read_dir`] panics for the `dir_name` provided
/// - if conversion from [`PathBuf`] to [`Utf8PathBuf`] fails
pub fn read_files_from_dir(dir_name: impl AsRef<Path>) -> Vec<Utf8PathBuf> {
    fs::read_dir(dir_name)
        .unwrap()
        .flatten()
        .map(|entry| entry.path().to_utf8_path_buf())
        .filter(|path| path.is_file())
        .collect()
}

/// Utility trait for creating files using [`assert_fs`].
pub trait CreateFile {
    fn create_file(&self, file_name: &str) -> ChildPath;
}

impl<T: PathChild> CreateFile for T {
    fn create_file(&self, file_name: &str) -> ChildPath {
        let file = self.child(file_name);
        file.touch().unwrap();
        file
    }
}

/// Utility trait for converting [`Path`] to [`Utf8PathBuf`].
/// It should only be used in tests as it panics on invalid UTF-8.
pub trait Utf8PathBufConversion {
    fn to_utf8_path_buf(&self) -> Utf8PathBuf;
}

impl<T: AsRef<Path>> Utf8PathBufConversion for T {
    fn to_utf8_path_buf(&self) -> Utf8PathBuf {
        Utf8PathBuf::from_path_buf(self.as_ref().to_path_buf()).unwrap()
    }
}
