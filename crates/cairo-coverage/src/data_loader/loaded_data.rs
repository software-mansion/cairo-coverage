use anyhow::Context;
use anyhow::Result;
use cairo_lang_sierra::debug_info::DebugInfo;
use cairo_lang_sierra::program::{Program, ProgramArtifact, VersionedProgram};
use camino::Utf8PathBuf;
use derived_deref::Deref;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::fs;
use trace_data::CallTrace;

type SourceSierraPath = String;

#[derive(Deref)]
pub struct LoadedDataMap(HashMap<SourceSierraPath, LoadedData>);

pub struct LoadedData {
    pub program: Program,
    pub debug_info: DebugInfo,
    pub call_traces: Vec<CallTrace>,
}

impl LoadedDataMap {
    pub fn load(call_trace_paths: &Vec<Utf8PathBuf>) -> Result<Self> {
        let mut map: HashMap<SourceSierraPath, LoadedData> = HashMap::new();
        for call_trace_path in call_trace_paths {
            let call_trace: CallTrace = read_and_deserialize(call_trace_path)?;

            let source_sierra_path = &call_trace
                .cairo_execution_info
                .as_ref()
                .context("Missing key 'cairo_execution_info' in call trace")?
                .source_sierra_path;

            if let Some(loaded_data) = map.get_mut(&source_sierra_path.to_string()) {
                loaded_data.call_traces.push(call_trace);
            } else {
                let VersionedProgram::V1 {
                    program:
                        ProgramArtifact {
                            program,
                            debug_info,
                        },
                    ..
                } = read_and_deserialize(source_sierra_path)?;

                map.insert(
                    source_sierra_path.to_string(),
                    LoadedData {
                        program,
                        debug_info: debug_info
                            .context(format!("Debug info not found in: {source_sierra_path}"))?,
                        call_traces: vec![call_trace],
                    },
                );
            }
        }
        Ok(Self(map))
    }
}

fn read_and_deserialize<T: DeserializeOwned>(file_path: &Utf8PathBuf) -> anyhow::Result<T> {
    fs::read_to_string(file_path)
        .context(format!("Failed to read file at path: {file_path}"))
        .and_then(|content| {
            serde_json::from_str(&content).context(format!(
                "Failed to deserialize JSON content from file at path: {file_path}"
            ))
        })
}
