use camino::Utf8PathBuf;
use clap::Parser;

/// Arguments accepted by the `clean` subcommand.
#[derive(Parser, Debug)]
pub struct CleanArgs {
    /// Root directory to search for files to clean.
    /// From this directory, all subdirectories are searched recursively.
    #[arg(short, long, default_value = ".")]
    pub root_dir: Utf8PathBuf,

    /// File name of a file to clean. It should also include the extension.
    #[arg(short, long, default_value = "coverage.lcov")]
    pub files_to_delete: Utf8PathBuf,
}
