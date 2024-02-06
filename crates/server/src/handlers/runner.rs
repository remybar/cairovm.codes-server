use axum::{http::StatusCode, Json};
use cairo1_run::{run_program_at_path, RunResult, CAIRO_LANG_COMPILER_VERSION};
use rand::distributions::{Distribution, Uniform};
use serde::{Deserialize, Serialize};
use std::{env, fs, path::PathBuf};
use tracer::{make_tracer_data, TracerData};

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

#[derive(Serialize)]
pub struct RunnerResult {
    sierra_program_code: String,
    casm_program_code: String,
    cairo_lang_compiler_version: String,
    serialized_output: Option<String>,
    tracer_data: TracerData,
    casm_formatted_instructions: Vec<String>,
}

pub async fn runner_handler(
    Json(payload): Json<RunnerPayload>,
) -> Result<Json<RunnerResult>, StatusCode> {
    let file_path = write_to_temp_file(&payload.cairo_program_code);

    let RunResult {
        sierra_program,
        casm_program,
        serialized_output,
        trace,
        memory,
        instructions,
    } = match run_program_at_path(&file_path) {
        Ok(result) => result,
        Err(error) => {
            dbg!(error);
            fs::remove_file(&file_path).expect("Failed to delete temporary file");
            return Err(StatusCode::EXPECTATION_FAILED);
        }
    };

    // Delete the temporary file
    fs::remove_file(&file_path).expect("Failed to delete temporary file");

    let tracer_data = make_tracer_data(trace, memory);

    let casm_program_code = instructions
        .iter()
        .map(|instruction| instruction.to_string())
        .collect::<Vec<_>>()
        .join("\n");

    let casm_formatted_instructions = instructions
        .iter()
        .map(|instruction| instruction.to_string())
        .collect();

    Ok(Json(RunnerResult {
        sierra_program_code: format!("{sierra_program}"),
        casm_program_code,
        cairo_lang_compiler_version: CAIRO_LANG_COMPILER_VERSION.to_string(),
        serialized_output,
        tracer_data,
        casm_formatted_instructions,
    }))
}
