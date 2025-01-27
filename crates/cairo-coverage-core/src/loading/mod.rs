//! This module handles **reading**, **deserializing**, and **extracting fields** from files into
//! structured data. It focuses on basic data loading tasks, leaving advanced operations to the
//! [`build`](crate::build) module.
//!
//! ## Main Entry Point
//! [`execution_data::load`] is the primary function, coordinating the loading of traces and programs
//! through various helper functions.
//!
//! ## Naming Conventions
//! - Functions start with the `load` prefix.
//! - If transformations or grouping are involved, append a descriptor (e.g., `load_grouped`).
//! - Prefer standalone functions over struct-based factory methods to
//!   avoid wrapping or redefining external types unnecessarily.

use anyhow::{Context, Result};
use camino::Utf8PathBuf;
use serde::de::DeserializeOwned;
use std::fs;

pub mod enriched_program;
pub mod execution_data;
mod execution_infos;

/// Utility function to read and deserialize a JSON file.
fn read_and_deserialize<T: DeserializeOwned>(file_path: &Utf8PathBuf) -> Result<T> {
    let content = fs::read_to_string(file_path)
        .context(format!("Failed to read file at path: {file_path}"))?;

    serde_json::from_str(&content).context(format!(
        "Failed to deserialize JSON content from file at path: {file_path}"
    ))
}
