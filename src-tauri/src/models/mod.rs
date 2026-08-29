pub mod dto;
pub mod execution;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandDefinition {
    pub schema_version: Option<u32>,
    pub id: String,
    pub command: String,
    pub name: String,
    pub category: String,
    pub description: String,
    #[serde(default = "default_platform")]
    pub platform: String,
    #[serde(default)]
    pub requires_admin: bool,
    #[serde(default)]
    pub dangerous: bool,
    #[serde(default)]
    pub arguments: Vec<ArgumentDefinition>,
    pub binary: Option<BinaryDefinition>,
    pub timeout_ms: Option<u64>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub aliases: Vec<String>,
    pub supported_os: Option<OSVersionRequirement>,
}

fn default_platform() -> String {
    "macos".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArgumentDefinition {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(rename = "type")]
    pub arg_type: ArgumentType,
    #[serde(default)]
    pub required: bool,
    pub default: Option<serde_json::Value>,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub unit: Option<String>,
    pub options: Option<Vec<String>>,
    pub multiple: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ArgumentType {
    String,
    Integer,
    Number,
    Boolean,
    Enum,
    Path,
    File,
    Directory,
    Duration,
    Multiple,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinaryDefinition {
    pub name: String,
    pub paths: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OSVersionRequirement {
    pub min: Option<String>,
    pub max: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandCategory {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}
