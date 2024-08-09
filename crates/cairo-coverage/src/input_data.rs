use crate::data_loader::deserialize;
use crate::data_loader::types::StatementMap;
use anyhow::{Context, Result};
use cairo_lang_sierra::program::{Program, VersionedProgram};
use camino::Utf8PathBuf;
use trace_data::CallTrace;

#[allow(dead_code)] // Temporary
struct InputData {
    call_trace: CallTrace,
    program: Program,
    statement_map: StatementMap,
}

impl InputData {
    #[allow(dead_code)] // Temporary
    pub fn new(
        call_trace_path: &Utf8PathBuf,
        versioned_program_path: &Utf8PathBuf,
    ) -> Result<Self> {
        let VersionedProgram::V1 { program, .. } = deserialize::from_file(versioned_program_path)?;
        let annotations = &program
            .debug_info
            .context(format!("Debug info not found in: {versioned_program_path}"))?
            .annotations;

        Ok(Self {
            call_trace: deserialize::from_file(call_trace_path)?,
            program: program.program,
            statement_map: annotations.try_into()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_loader::types::{Position, Range};
    use cairo_lang_sierra::program::StatementIdx;

    const TRACE: &str = "tests/data/config/trace.json";
    const VERSIONED_PROGRAM: &str = "tests/data/config/versioned_program.json";

    #[test]
    fn versioned_program() {
        let config = InputData::new(&TRACE.into(), &VERSIONED_PROGRAM.into()).unwrap();
        assert_eq!(config.statement_map.len(), 142);

        let (file_location, range) = &config.statement_map[&StatementIdx(1)].code_locations[0];
        assert!(file_location.ends_with("test_call.cairo"));

        let statement_details = &config.statement_map[&StatementIdx(1)];
        assert_eq!(
            statement_details.function_names,
            &["tests::test_call::my_test"]
        );
        assert_eq!(
            range,
            &Range {
                start: Position { line: 5 },
                end: Position { line: 5 },
            }
        );
    }
}
