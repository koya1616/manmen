use tauri::State;
use std::collections::HashMap;
use std::sync::Mutex;

use crate::registry::CommandRegistry;
use crate::storage::Storage;
use crate::models::dto::*;
use crate::models::execution::ExecutionRecord;
use crate::builder::CommandBuilder;
use crate::validator::Validator;
use crate::executor::Executor;
use log::{info, warn};

pub struct AppState {
    pub registry: Mutex<CommandRegistry>,
    pub storage: Mutex<Storage>,
}

#[tauri::command]
pub fn get_commands(state: State<AppState>) -> Result<Vec<CommandSummaryDTO>, String> {
    info!("IPC: get_commands");
    let registry = state.registry.lock().map_err(|e| e.to_string())?;
    let commands = registry.get_all_commands();
    info!("Found {} commands", commands.len());

    Ok(commands
        .into_iter()
        .map(|cmd| CommandSummaryDTO {
            id: cmd.id.clone(),
            name: cmd.name.clone(),
            description: cmd.description.clone(),
            category: cmd.category.clone(),
            requires_admin: cmd.requires_admin,
            dangerous: cmd.dangerous,
            tags: cmd.tags.clone(),
        })
        .collect())
}

#[tauri::command]
pub fn get_command(state: State<AppState>, id: String) -> Result<CommandDetailDTO, String> {
    info!("IPC: get_command id={}", id);
    let registry = state.registry.lock().map_err(|e| e.to_string())?;
    let cmd = registry
        .get_command(&id)
        .ok_or_else(|| {
            warn!("Command '{}' not found", id);
            format!("Command '{}' not found", id)
        })?;

    Ok(CommandDetailDTO {
        id: cmd.id.clone(),
        command: cmd.command.clone(),
        name: cmd.name.clone(),
        category: cmd.category.clone(),
        description: cmd.description.clone(),
        requires_admin: cmd.requires_admin,
        dangerous: cmd.dangerous,
        arguments: cmd
            .arguments
            .iter()
            .map(|arg| ArgumentDTO {
                id: arg.id.clone(),
                name: arg.name.clone(),
                description: arg.description.clone(),
                arg_type: format!("{:?}", arg.arg_type).to_lowercase(),
                required: arg.required,
                default: arg.default.clone(),
                min: arg.min,
                max: arg.max,
                unit: arg.unit.clone(),
                options: arg.options.clone(),
            })
            .collect(),
        timeout_ms: cmd.timeout_ms,
        tags: cmd.tags.clone(),
    })
}

#[tauri::command]
pub fn search_commands(state: State<AppState>, query: String) -> Result<Vec<CommandSummaryDTO>, String> {
    info!("IPC: search_commands query='{}'", query);
    let registry = state.registry.lock().map_err(|e| e.to_string())?;
    let results = registry.search(&query);
    info!("Search returned {} results", results.len());

    Ok(results
        .into_iter()
        .map(|cmd| CommandSummaryDTO {
            id: cmd.id.clone(),
            name: cmd.name.clone(),
            description: cmd.description.clone(),
            category: cmd.category.clone(),
            requires_admin: cmd.requires_admin,
            dangerous: cmd.dangerous,
            tags: cmd.tags.clone(),
        })
        .collect())
}

#[tauri::command]
pub fn get_categories(state: State<AppState>) -> Result<Vec<CommandCategoryDTO>, String> {
    info!("IPC: get_categories");
    let registry = state.registry.lock().map_err(|e| e.to_string())?;
    let categories = registry.get_categories();

    Ok(categories
        .iter()
        .map(|cat| CommandCategoryDTO {
            id: cat.id.clone(),
            name: cat.name.clone(),
            description: cat.description.clone(),
        })
        .collect())
}

#[tauri::command]
pub fn validate_command(
    state: State<AppState>,
    command_id: String,
    arguments: HashMap<String, serde_json::Value>,
) -> Result<CommandValidationResultDTO, String> {
    info!("IPC: validate_command id={}", command_id);
    let registry = state.registry.lock().map_err(|e| e.to_string())?;
    let cmd = registry
        .get_command(&command_id)
        .ok_or_else(|| format!("Command '{}' not found", command_id))?;

    let result = Validator::validate(cmd, &arguments);
    info!("Validation result: valid={}", result.valid);

    Ok(CommandValidationResultDTO {
        valid: result.valid,
        errors: result
            .errors
            .into_iter()
            .map(|e| ValidationErrorDTO {
                field: e.field,
                code: e.code,
                message: e.message,
            })
            .collect(),
    })
}

#[tauri::command]
pub fn build_command(
    state: State<AppState>,
    command_id: String,
    arguments: HashMap<String, serde_json::Value>,
) -> Result<BuildCommandResponseDTO, String> {
    info!("IPC: build_command id={}", command_id);
    let registry = state.registry.lock().map_err(|e| e.to_string())?;
    let cmd = registry
        .get_command(&command_id)
        .ok_or_else(|| format!("Command '{}' not found", command_id))?;

    let validation = Validator::validate(cmd, &arguments);
    if !validation.valid {
        return Err(format!(
            "Validation failed: {}",
            validation
                .errors
                .iter()
                .map(|e| e.message.clone())
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }

    let (program, args) = CommandBuilder::build(cmd, &arguments)?;

    let mut full_command = program.clone();
    for arg in &args {
        full_command.push(' ');
        full_command.push_str(arg);
    }

    info!("Built command: {}", full_command);

    Ok(BuildCommandResponseDTO {
        program,
        args,
        full_command,
    })
}

#[tauri::command]
pub async fn execute_command(
    state: State<'_, AppState>,
    request: CommandExecutionRequest,
) -> Result<CommandExecutionResponseDTO, String> {
    info!("IPC: execute_command id={} args={:?}", request.command_id, request.arguments);

    // 1. Lock registry, clone what we need, drop lock immediately
    let (cmd_def, timeout_ms) = {
        let registry = state.registry.lock().map_err(|e| e.to_string())?;
        let cmd = registry
            .get_command(&request.command_id)
            .ok_or_else(|| format!("Command '{}' not found", request.command_id))?;
        (cmd.clone(), cmd.timeout_ms)
    };

    // 2. Validate (no lock needed)
    let validation = Validator::validate(&cmd_def, &request.arguments);
    if !validation.valid {
        let msg = validation
            .errors
            .iter()
            .map(|e| e.message.clone())
            .collect::<Vec<_>>()
            .join(", ");
        warn!("Validation failed: {}", msg);
        return Err(format!("Validation failed: {}", msg));
    }

    // 3. Build command (no lock needed)
    let (program, args) = CommandBuilder::build(&cmd_def, &request.arguments)?;

    let generated_command = {
        let mut s = program.clone();
        for arg in &args {
            s.push(' ');
            s.push_str(arg);
        }
        s
    };
    info!("Executing: {}", generated_command);

    // 4. Execute process (no lock, async)
    let resolved_program = Executor::resolve_program(&cmd_def)?;
    let resolved_program = resolved_program
        .to_str()
        .ok_or_else(|| format!("Command path is not valid UTF-8: {:?}", resolved_program))?
        .to_string();
    let execution_args = args.clone();
    let requires_admin = cmd_def.requires_admin;
    let result = tauri::async_runtime::spawn_blocking(move || {
        if requires_admin {
            Executor::execute_as_admin(&resolved_program, &execution_args, timeout_ms)
        } else {
            Executor::execute(&resolved_program, &execution_args, timeout_ms)
        }
    })
    .await
    .map_err(|error| format!("Execution task failed: {}", error))??;

    info!(
        "Execution complete: id={} success={} exit_code={} duration={}ms",
        result.execution_id, result.success, result.exit_code, result.duration_ms
    );
    if !result.stdout.is_empty() {
        info!("stdout: {}", &result.stdout[..result.stdout.len().min(500)]);
    }
    if !result.stderr.is_empty() {
        warn!("stderr: {}", &result.stderr[..result.stderr.len().min(500)]);
    }

    // 5. Save to history
    let record = ExecutionRecord {
        id: result.execution_id.clone(),
        command_id: request.command_id,
        command: program,
        arguments_json: serde_json::to_string(&request.arguments).unwrap_or_default(),
        generated_command,
        exit_code: Some(result.exit_code),
        success: result.success,
        stdout: result.stdout.clone(),
        stderr: result.stderr.clone(),
        duration_ms: Some(result.duration_ms),
        started_at: result.started_at,
        finished_at: Some(result.finished_at),
    };

    {
        let storage = state.storage.lock().map_err(|e| e.to_string())?;
        storage.save_execution(&record)?;
        info!("Execution history saved: id={}", record.id);
    }

    Ok(CommandExecutionResponseDTO {
        execution_id: result.execution_id,
        success: result.success,
        exit_code: result.exit_code,
        stdout: result.stdout,
        stderr: result.stderr,
        duration_ms: result.duration_ms,
    })
}

#[tauri::command]
pub fn get_execution_history(
    state: State<AppState>,
    limit: Option<i32>,
) -> Result<Vec<HistoryDTO>, String> {
    info!("IPC: get_execution_history limit={:?}", limit);
    let storage = state.storage.lock().map_err(|e| e.to_string())?;
    let records = storage.get_executions(limit.unwrap_or(50))?;

    Ok(records
        .into_iter()
        .map(|r| HistoryDTO {
            id: r.id,
            command_id: r.command_id,
            command_name: r.command,
            generated_command: r.generated_command,
            success: r.success,
            exit_code: r.exit_code,
            duration_ms: r.duration_ms,
            started_at: r.started_at.to_rfc3339(),
        })
        .collect())
}

#[tauri::command]
pub fn get_execution(
    state: State<AppState>,
    id: String,
) -> Result<HistoryDetailDTO, String> {
    info!("IPC: get_execution id={}", id);
    let storage = state.storage.lock().map_err(|e| e.to_string())?;
    let record = storage
        .get_execution(&id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Execution '{}' not found", id))?;

    Ok(HistoryDetailDTO {
        id: record.id,
        command_id: record.command_id,
        command_name: record.command,
        generated_command: record.generated_command,
        success: record.success,
        exit_code: record.exit_code,
        stdout: record.stdout,
        stderr: record.stderr,
        duration_ms: record.duration_ms,
        started_at: record.started_at.to_rfc3339(),
    })
}
