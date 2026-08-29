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

    fn get_cli_key(arg_def: &crate::models::ArgumentDefinition) -> &str {
        arg_def.cli_key.as_deref().unwrap_or(&arg_def.id)
    }

    fn build_argument(
        arg_def: &crate::models::ArgumentDefinition,
        value: &serde_json::Value,
    ) -> Result<Vec<String>, String> {
        let mut result = Vec::new();

        match &arg_def.arg_type {
            crate::models::ArgumentType::Boolean => {
                let b = value.as_bool().ok_or_else(|| {
                    format!("Argument '{}' must be a boolean", arg_def.id)
                })?;
                if let Some(flag) = &arg_def.cli_flag {
                    if b {
                        result.push(flag.clone());
                    }
                } else {
                    result.push(Self::get_cli_key(arg_def).to_string());
                    result.push(if b { "1" } else { "0" }.to_string());
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

                if let Some(flag) = &arg_def.cli_flag {
                    result.push(flag.clone());
                } else {
                    result.push(Self::get_cli_key(arg_def).to_string());
                }
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

                if let Some(flag) = &arg_def.cli_flag {
                    result.push(flag.clone());
                } else {
                    result.push(Self::get_cli_key(arg_def).to_string());
                }
                result.push(n.to_string());
            }
            crate::models::ArgumentType::String | crate::models::ArgumentType::Path | crate::models::ArgumentType::File | crate::models::ArgumentType::Directory => {
                let s = value.as_str().ok_or_else(|| {
                    format!("Argument '{}' must be a string", arg_def.id)
                })?;

                if let Some(flag) = &arg_def.cli_flag {
                    result.push(flag.clone());
                } else {
                    result.push(Self::get_cli_key(arg_def).to_string());
                }
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

                if let Some(flag) = &arg_def.cli_flag {
                    result.push(flag.clone());
                } else if arg_def.cli_key.is_some() {
                    result.push(Self::get_cli_key(arg_def).to_string());
                } else {
                    result.push(s.to_string());
                }
            }
            crate::models::ArgumentType::Duration => {
                let n = value.as_f64().ok_or_else(|| {
                    format!("Argument '{}' must be a number (seconds)", arg_def.id)
                })?;

                if let Some(flag) = &arg_def.cli_flag {
                    result.push(flag.clone());
                } else {
                    result.push(Self::get_cli_key(arg_def).to_string());
                }
                result.push(n.to_string());
            }
            crate::models::ArgumentType::Multiple => {
                if let Some(arr) = value.as_array() {
                    for item in arr {
                        if let Some(s) = item.as_str() {
                            if let Some(flag) = &arg_def.cli_flag {
                                result.push(flag.clone());
                            } else {
                                result.push(Self::get_cli_key(arg_def).to_string());
                            }
                            result.push(s.to_string());
                        }
                    }
                } else if let Some(s) = value.as_str() {
                    if let Some(flag) = &arg_def.cli_flag {
                        result.push(flag.clone());
                    } else {
                        result.push(Self::get_cli_key(arg_def).to_string());
                    }
                    result.push(s.to_string());
                }
            }
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::CommandBuilder;
    use crate::models::CommandDefinition;
    use serde_json::json;
    use std::collections::HashMap;

    fn command_with_argument(argument: serde_json::Value) -> CommandDefinition {
        serde_json::from_value(json!({
            "id": "test.command",
            "command": "test-command",
            "name": "Test",
            "category": "test",
            "description": "Test command",
            "arguments": [argument]
        }))
        .expect("command definition should deserialize")
    }

    #[test]
    fn boolean_key_uses_numeric_value() {
        let command = command_with_argument(json!({
            "id": "ttyskeepawake",
            "name": "TTY sleep prevention",
            "type": "boolean",
            "required": false,
            "default": null
        }));
        let arguments = HashMap::from([("ttyskeepawake".to_string(), json!(true))]);

        let (_, args) = CommandBuilder::build(&command, &arguments).unwrap();

        assert_eq!(args, vec!["ttyskeepawake", "1"]);
    }

    #[test]
    fn false_boolean_key_uses_zero() {
        let command = command_with_argument(json!({
            "id": "ttyskeepawake",
            "name": "TTY sleep prevention",
            "type": "boolean",
            "required": false,
            "default": null
        }));
        let arguments = HashMap::from([("ttyskeepawake".to_string(), json!(false))]);

        let (_, args) = CommandBuilder::build(&command, &arguments).unwrap();

        assert_eq!(args, vec!["ttyskeepawake", "0"]);
    }

    #[test]
    fn false_boolean_flag_is_omitted() {
        let command = command_with_argument(json!({
            "id": "display",
            "name": "Prevent display sleep",
            "type": "boolean",
            "required": false,
            "default": false,
            "cli_flag": "-d"
        }));
        let arguments = HashMap::from([("display".to_string(), json!(false))]);

        let (_, args) = CommandBuilder::build(&command, &arguments).unwrap();

        assert!(args.is_empty());
    }

    #[test]
    fn true_boolean_flag_is_emitted_without_value() {
        let command = command_with_argument(json!({
            "id": "display",
            "name": "Prevent display sleep",
            "type": "boolean",
            "required": false,
            "default": false,
            "cli_flag": "-d"
        }));
        let arguments = HashMap::from([("display".to_string(), json!(true))]);

        let (_, args) = CommandBuilder::build(&command, &arguments).unwrap();

        assert_eq!(args, vec!["-d"]);
    }
}
