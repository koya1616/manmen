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

pub struct AppState {
    pub registry: Mutex<CommandRegistry>,
    pub storage: Mutex<Storage>,
}

#[tauri::command]
pub fn get_commands(state: State<AppState>) -> Result<Vec<CommandSummaryDTO>, String> {
    let registry = state.registry.lock().map_err(|e| e.to_string())?;
    let commands = registry.get_all_commands();

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
    let registry = state.registry.lock().map_err(|e| e.to_string())?;
    let cmd = registry
        .get_command(&id)
        .ok_or_else(|| format!("Command '{}' not found", id))?;

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
    let registry = state.registry.lock().map_err(|e| e.to_string())?;
    let results = registry.search(&query);

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
    let registry = state.registry.lock().map_err(|e| e.to_string())?;
    let cmd = registry
        .get_command(&command_id)
        .ok_or_else(|| format!("Command '{}' not found", command_id))?;

    let result = Validator::validate(cmd, &arguments);

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
    let registry = state.registry.lock().map_err(|e| e.to_string())?;
    let cmd = registry
        .get_command(&command_id)
        .ok_or_else(|| format!("Command '{}' not found", command_id))?;

    // Validate first
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

    Ok(BuildCommandResponseDTO {
        program,
        args,
        full_command,
    })
}

#[tauri::command]
pub fn execute_command(
    state: State<AppState>,
    request: CommandExecutionRequest,
) -> Result<CommandExecutionResponseDTO, String> {
    let registry = state.registry.lock().map_err(|e| e.to_string())?;
    let cmd = registry
        .get_command(&request.command_id)
        .ok_or_else(|| format!("Command '{}' not found", request.command_id))?;

    // Validate
    let validation = Validator::validate(cmd, &request.arguments);
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

    // Build command
    let (program, args) = CommandBuilder::build(cmd, &request.arguments)?;

    // Execute
    let result = Executor::execute(&program, &args, cmd.timeout_ms)?;

    // Save to history
    let generated_command = {
        let mut s = program.clone();
        for arg in &args {
            s.push(' ');
            s.push_str(arg);
        }
        s
    };

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

    let storage = state.storage.lock().map_err(|e| e.to_string())?;
    storage.save_execution(&record)?;

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
