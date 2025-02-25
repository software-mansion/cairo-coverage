use crate::loading::read_and_deserialize;
use anyhow::{Context, Result};
use cairo_annotations::annotations::coverage::VersionedCoverageAnnotations;
use cairo_annotations::annotations::profiler::VersionedProfilerAnnotations;
use cairo_annotations::annotations::{AnnotationsError, TryFromDebugInfo};
use cairo_lang_sierra::debug_info::DebugInfo;
use cairo_lang_sierra::ids::FunctionId;
use cairo_lang_sierra::program::{Program, ProgramArtifact, VersionedProgram};
use cairo_lang_starknet_classes::contract_class::ContractClass;
use camino::Utf8PathBuf;
use indoc::indoc;
use serde::Deserialize;

/// Sierra [`Program`] with:
/// - `test_executables` to know which functions are test functions
/// - `coverage_annotations` for line mappings
/// - `profiler_annotations` for function mappings
/// - `program` itself for the future transformations to casm debug info
#[derive(Clone)]
pub struct EnrichedProgram {
    pub test_executables: Vec<FunctionId>,
    pub coverage_annotations: VersionedCoverageAnnotations,
    pub profiler_annotations: VersionedProfilerAnnotations,
    pub program: Program,
}
/// As sierra program can be in the form of raw sierra (normal cairo program) and contract class (starknet contracts).
/// We need to be able to deserialize both. That's why we create [`SierraProgram`] enum.
#[derive(Deserialize)]
#[serde(untagged)]
enum SierraProgram {
    VersionedProgram(VersionedProgram),
    ContractClass(ContractClass),
}

/// Load [`EnrichedProgram`] from a given path.
pub fn load(source_sierra_path: &Utf8PathBuf) -> Result<EnrichedProgram> {
    let sierra_program = read_and_deserialize(source_sierra_path)?;
    let (program, debug_info) = extract(sierra_program)?;
    let coverage_annotations = deserialize_annotations(&debug_info)?;
    let profiler_annotations = deserialize_annotations(&debug_info)?;
    let test_executables = extract_test_executables(debug_info);
    Ok(EnrichedProgram {
        test_executables,
        coverage_annotations,
        profiler_annotations,
        program,
    })
}

/// Extract [`Program`] and [`DebugInfo`] from [`SierraProgram`].
fn extract(sierra_program: SierraProgram) -> Result<(Program, DebugInfo)> {
    match sierra_program {
        SierraProgram::VersionedProgram(program) => extract_versioned_program(program),
        SierraProgram::ContractClass(contract_class) => extract_contract_class(contract_class),
    }
}

/// Extract [`Program`] and [`DebugInfo`] from [`VersionedProgram`].
fn extract_versioned_program(
    VersionedProgram::V1 {
        program: ProgramArtifact {
            program,
            debug_info,
        },
        ..
    }: VersionedProgram,
) -> Result<(Program, DebugInfo)> {
    Ok((
        program,
        debug_info.context("debug info not found in program")?,
    ))
}

/// Extract [`Program`] and [`DebugInfo`] from [`ContractClass`].
fn extract_contract_class(contract_class: ContractClass) -> Result<(Program, DebugInfo)> {
    let program = contract_class.extract_sierra_program()?;
    let debug_info = contract_class
        .sierra_program_debug_info
        .context("debug info not found in contract")?;
    Ok((program, debug_info))
}

/// Extract test executables from [`DebugInfo`].
fn extract_test_executables(mut debug_info: DebugInfo) -> Vec<FunctionId> {
    const SNFORGE_TEST_EXECUTABLE: &str = "snforge_internal_test_executable";

    debug_info
        .executables
        .remove(SNFORGE_TEST_EXECUTABLE)
        .unwrap_or_default()
}

/// Deserialize annotations from [`DebugInfo`] and provide a helpful error message.
fn deserialize_annotations<T: TryFromDebugInfo<Error = AnnotationsError>>(
    debug_info: &DebugInfo,
) -> Result<T> {
    const RECOMMENDED_CAIRO_PROFILE_TOML: &str = indoc! {
        r#"
        perhaps you are missing the following entries in Scarb.toml:

        [profile.dev.cairo]
        unstable-add-statements-functions-debug-info = true
        unstable-add-statements-code-locations-debug-info = true
        inlining-strategy = "avoid"
        "#
    };

    T::try_from_debug_info(debug_info).context(RECOMMENDED_CAIRO_PROFILE_TOML)
}
