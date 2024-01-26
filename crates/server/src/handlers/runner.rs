use axum::{http::StatusCode, Json};
use cairo_lang_compiler::{compile_cairo_project_at_path, CompilerConfig};
use cairo_lang_sierra_ap_change::calc_ap_changes;
use cairo_lang_sierra_gas::gas_info::GasInfo;
use cairo_lang_sierra_to_casm::metadata::calc_metadata;
use cairo_lang_sierra_to_casm::metadata::Metadata;
use cairo_lang_sierra_to_casm::metadata::MetadataComputationConfig;
use cairo_lang_sierra_to_casm::metadata::MetadataError;
use cairo_vm::vm::errors::vm_errors::VirtualMachineError;
use rand::distributions::{Distribution, Uniform};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;

const CAIRO_LANG_COMPILER_VERSION: &'static str = "2.5.0";

fn write_to_temp_file(content: &str) -> PathBuf {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let mut rng = rand::thread_rng();
    let alphabet = Uniform::from('a'..'z');
    let file_name: String = std::iter::repeat_with(|| alphabet.sample(&mut rng))
        .take(30)
        .collect();
    let file_path = current_dir.join(format!("{}.cairo", file_name));
    fs::write(&file_path, content).expect("Failed to write to file");
    file_path
}

#[derive(Serialize, Deserialize)]
pub struct RunnerPayload {
    cairo_program_code: String,
}

#[derive(Serialize, Deserialize)]
pub struct RunnerResult {
    sierra_program_code: String,
    casm_program_code: String,
    cairo_lang_compiler_version: String,
}

pub async fn runner_handler(
    Json(payload): Json<RunnerPayload>,
) -> Result<Json<RunnerResult>, StatusCode> {
    let file_path = write_to_temp_file(&payload.cairo_program_code);

    let compiler_config = CompilerConfig {
        replace_ids: true,
        ..CompilerConfig::default()
    };
    let sierra_program = match compile_cairo_project_at_path(&file_path, compiler_config) {
        Ok(program) => program,
        Err(err) => {
            // Delete the temporary file if an error occurs
            fs::remove_file(&file_path).expect("Failed to delete temporary file");
            println!(
                "Failed to compile cairo program; error: {}",
                err.to_string()
            );
            return Err(StatusCode::EXPECTATION_FAILED);
        }
    };

    let metadata_config = Some(Default::default());

    let gas_usage_check = metadata_config.is_some();
    let metadata = match create_metadata(&sierra_program, metadata_config) {
        Ok(metadata) => metadata,
        Err(_) => {
            fs::remove_file(&file_path).expect("Failed to delete temporary file");
            println!("Failed to compute metadata");
            return Err(StatusCode::EXPECTATION_FAILED);
        }
    };

    let casm_program = match cairo_lang_sierra_to_casm::compiler::compile(
        &sierra_program,
        &metadata,
        gas_usage_check,
    ) {
        Ok(casm_program) => casm_program,
        Err(_) => {
            fs::remove_file(&file_path).expect("Failed to delete temporary file");
            println!("Failed to compile sierra program");
            return Err(StatusCode::EXPECTATION_FAILED);
        }
    };

    // Delete the temporary file
    fs::remove_file(&file_path).expect("Failed to delete temporary file");

    Ok(Json(RunnerResult {
        sierra_program_code: format!("{sierra_program}"),
        casm_program_code: format!("{casm_program}"),
        cairo_lang_compiler_version: CAIRO_LANG_COMPILER_VERSION.to_string(),
    }))
}

/// Creates the metadata required for a Sierra program lowering to casm.
fn create_metadata(
    sierra_program: &cairo_lang_sierra::program::Program,
    metadata_config: Option<MetadataComputationConfig>,
) -> Result<Metadata, VirtualMachineError> {
    if let Some(metadata_config) = metadata_config {
        calc_metadata(sierra_program, metadata_config).map_err(|err| match err {
            MetadataError::ApChangeError(_) => VirtualMachineError::Unexpected,
            MetadataError::CostError(_) => VirtualMachineError::Unexpected,
        })
    } else {
        Ok(Metadata {
            ap_change_info: calc_ap_changes(sierra_program, |_, _| 0)
                .map_err(|_| VirtualMachineError::Unexpected)?,
            gas_info: GasInfo {
                variable_values: Default::default(),
                function_costs: Default::default(),
            },
        })
    }
}
