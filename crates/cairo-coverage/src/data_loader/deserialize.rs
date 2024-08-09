use anyhow::{Context, Result};
use cairo_lang_sierra::debug_info::Annotations;
use camino::Utf8PathBuf;
use serde::de::DeserializeOwned;
use std::fs;

pub fn from_file<T: DeserializeOwned>(file_path: &Utf8PathBuf) -> Result<T> {
    fs::read_to_string(file_path)
        .context(format!("Failed to read file at path: {file_path}"))
        .and_then(|content| {
            serde_json::from_str(&content).context(format!(
                "Failed to deserialize JSON content from file at path: {file_path}"
            ))
        })
}

pub fn by_namespace<T: DeserializeOwned>(annotations: &Annotations, namespace: &str) -> Result<T> {
    annotations
        .get(namespace)
        .cloned()
        .context(format!("Expected key: {namespace} but was missing"))
        .and_then(|value| {
            serde_json::from_value(value)
                .context(format!("Failed to deserialize at key: {namespace}"))
        })
}
