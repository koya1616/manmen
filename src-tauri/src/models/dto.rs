use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandSummaryDTO {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub requires_admin: bool,
    pub dangerous: bool,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandDetailDTO {
    pub id: String,
    pub command: String,
    pub name: String,
    pub category: String,
    pub description: String,
    pub requires_admin: bool,
    pub dangerous: bool,
    pub arguments: Vec<ArgumentDTO>,
    pub timeout_ms: Option<u64>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArgumentDTO {
    pub id: String,
    pub name: String,
    pub description: String,
    #[serde(rename = "type")]
    pub arg_type: String,
    pub required: bool,
    pub default: Option<serde_json::Value>,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub unit: Option<String>,
    pub options: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandCategoryDTO {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandValidationResultDTO {
    pub valid: bool,
    pub errors: Vec<ValidationErrorDTO>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationErrorDTO {
    pub field: String,
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildCommandResponseDTO {
    pub program: String,
    pub args: Vec<String>,
    pub full_command: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandExecutionRequest {
    pub command_id: String,
    pub arguments: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandExecutionResponseDTO {
    pub execution_id: String,
    pub success: bool,
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
    pub duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryDTO {
    pub id: String,
    pub command_id: String,
    pub command_name: String,
    pub generated_command: String,
    pub success: bool,
    pub exit_code: Option<i32>,
    pub duration_ms: Option<u64>,
    pub started_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryDetailDTO {
    pub id: String,
    pub command_id: String,
    pub command_name: String,
    pub generated_command: String,
    pub success: bool,
    pub exit_code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
    pub duration_ms: Option<u64>,
    pub started_at: String,
}
