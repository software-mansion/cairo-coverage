use assert_fs::fixture::{ChildPath, FileTouch, PathChild};
use camino::Utf8PathBuf;
use std::path::Path;

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
