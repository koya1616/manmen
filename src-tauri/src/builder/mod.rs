use crate::models::CommandDefinition;
use std::collections::HashMap;

pub struct CommandBuilder;

impl CommandBuilder {
    pub fn build(
        command_def: &CommandDefinition,
        arguments: &HashMap<String, serde_json::Value>,
    ) -> Result<(String, Vec<String>), String> {
        let mut args: Vec<String> = Vec::new();

        for arg_def in &command_def.arguments {
            if let Some(value) = arguments.get(&arg_def.id) {
                let arg_strings = Self::build_argument(arg_def, value)?;
                args.extend(arg_strings);
            } else if let Some(default) = &arg_def.default {
                let arg_strings = Self::build_argument(arg_def, default)?;
                args.extend(arg_strings);
            } else if arg_def.required {
                return Err(format!("Required argument '{}' is missing", arg_def.id));
            }
        }

        Ok((command_def.command.clone(), args))
    }

    fn build_argument(
        arg_def: &crate::models::ArgumentDefinition,
        value: &serde_json::Value,
    ) -> Result<Vec<String>, String> {
        let mut result = Vec::new();

        match &arg_def.arg_type {
            crate::models::ArgumentType::Boolean => {
                let b = value.as_bool().unwrap_or(false);
                if b {
                    result.push(arg_def.id.clone());
                }
            }
            crate::models::ArgumentType::Integer => {
                let n = value.as_i64().ok_or_else(|| {
                    format!("Argument '{}' must be an integer", arg_def.id)
                })?;

                if let Some(min) = arg_def.min {
                    if (n as f64) < min {
                        return Err(format!(
                            "Argument '{}' must be >= {}",
                            arg_def.id, min
                        ));
                    }
                }

                if let Some(max) = arg_def.max {
                    if (n as f64) > max {
                        return Err(format!(
                            "Argument '{}' must be <= {}",
                            arg_def.id, max
                        ));
                    }
                }

                result.push(arg_def.id.clone());
                result.push(n.to_string());
            }
            crate::models::ArgumentType::Number => {
                let n = value.as_f64().ok_or_else(|| {
                    format!("Argument '{}' must be a number", arg_def.id)
                })?;

                if let Some(min) = arg_def.min {
                    if n < min {
                        return Err(format!(
                            "Argument '{}' must be >= {}",
                            arg_def.id, min
                        ));
                    }
                }

                if let Some(max) = arg_def.max {
                    if n > max {
                        return Err(format!(
                            "Argument '{}' must be <= {}",
                            arg_def.id, max
                        ));
                    }
                }

                result.push(arg_def.id.clone());
                result.push(n.to_string());
            }
            crate::models::ArgumentType::String | crate::models::ArgumentType::Path | crate::models::ArgumentType::File | crate::models::ArgumentType::Directory => {
                let s = value.as_str().ok_or_else(|| {
                    format!("Argument '{}' must be a string", arg_def.id)
                })?;

                result.push(arg_def.id.clone());
                result.push(s.to_string());
            }
            crate::models::ArgumentType::Enum => {
                let s = value.as_str().ok_or_else(|| {
                    format!("Argument '{}' must be a string", arg_def.id)
                })?;

                if let Some(options) = &arg_def.options {
                    if !options.contains(&s.to_string()) {
                        return Err(format!(
                            "Argument '{}' must be one of: {}",
                            arg_def.id,
                            options.join(", ")
                        ));
                    }
                }

                result.push(arg_def.id.clone());
                result.push(s.to_string());
            }
            crate::models::ArgumentType::Duration => {
                let n = value.as_f64().ok_or_else(|| {
                    format!("Argument '{}' must be a number (seconds)", arg_def.id)
                })?;

                result.push(arg_def.id.clone());
                result.push(n.to_string());
            }
            crate::models::ArgumentType::Multiple => {
                if let Some(arr) = value.as_array() {
                    for item in arr {
                        if let Some(s) = item.as_str() {
                            result.push(arg_def.id.clone());
                            result.push(s.to_string());
                        }
                    }
                } else if let Some(s) = value.as_str() {
                    result.push(arg_def.id.clone());
                    result.push(s.to_string());
                }
            }
        }

        Ok(result)
    }
}
