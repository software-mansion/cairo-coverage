use anyhow::{Context, Result};
use cairo_lang_sierra::debug_info::DebugInfo;
use cairo_lang_sierra::program::{Program, ProgramArtifact, VersionedProgram};
use cairo_lang_sierra_to_casm::compiler::{CairoProgramDebugInfo, SierraToCasmConfig};
use cairo_lang_sierra_to_casm::metadata::{calc_metadata, MetadataComputationConfig};
use cairo_lang_starknet_classes::casm_contract_class::CasmContractClass;
use cairo_lang_starknet_classes::contract_class::ContractClass;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(untagged)]
pub enum SierraProgram {
    VersionedProgram(VersionedProgram),
    ContractClass(ContractClass),
}

pub trait GetDebugInfos {
    fn compile_and_get_debug_infos(self) -> Result<(DebugInfo, CairoProgramDebugInfo)>;
}

impl GetDebugInfos for VersionedProgram {
    fn compile_and_get_debug_infos(self) -> Result<(DebugInfo, CairoProgramDebugInfo)> {
        let VersionedProgram::V1 {
            program:
                ProgramArtifact {
                    program,
                    debug_info,
                },
            ..
        } = self;

        let debug_info = debug_info.context("Debug info not found in program")?;
        let casm_debug_info = compile_program_to_casm_debug_info(&program)?;
        Ok((debug_info, casm_debug_info))
    }
}

impl GetDebugInfos for ContractClass {
    fn compile_and_get_debug_infos(self) -> Result<(DebugInfo, CairoProgramDebugInfo)> {
        let debug_info = self
            .sierra_program_debug_info
            .context("Debug info not found in contract")?;

        // OPTIMIZATION:
        // Debug info is unused in the compilation. This saves us a costly clone.
        let casm_debug_info = compile_contract_class_to_casm_debug_info(ContractClass {
            sierra_program_debug_info: None,
            ..self
        })?;

        Ok((debug_info, casm_debug_info))
    }
}
impl GetDebugInfos for SierraProgram {
    fn compile_and_get_debug_infos(self) -> Result<(DebugInfo, CairoProgramDebugInfo)> {
        match self {
            SierraProgram::VersionedProgram(program) => program.compile_and_get_debug_infos(),
            SierraProgram::ContractClass(contract_class) => {
                contract_class.compile_and_get_debug_infos()
            }
        }
    }
}

fn compile_program_to_casm_debug_info(program: &Program) -> Result<CairoProgramDebugInfo> {
    cairo_lang_sierra_to_casm::compiler::compile(
        program,
        &calc_metadata(program, MetadataComputationConfig::default())
            .context("Failed calculating Sierra variables")?,
        SierraToCasmConfig {
            gas_usage_check: false,
            max_bytecode_size: usize::MAX,
        },
    )
    .map(|casm| casm.debug_info)
    .context("Failed to compile program to casm")
}

fn compile_contract_class_to_casm_debug_info(
    contract_class: ContractClass,
) -> Result<CairoProgramDebugInfo> {
    CasmContractClass::from_contract_class_with_debug_info(contract_class, false, usize::MAX)
        .map(|(_, casm_debug_info)| casm_debug_info)
        .context("Failed to compile contract class to casm")
}
