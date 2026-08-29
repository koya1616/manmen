use crate::models::{CommandDefinition, ArgumentDefinition};
use crate::models::execution::{CommandValidationResult, ValidationFieldError};
use std::collections::HashMap;

pub struct Validator;

impl Validator {
    pub fn validate(
        command_def: &CommandDefinition,
        arguments: &HashMap<String, serde_json::Value>,
    ) -> CommandValidationResult {
        let mut errors = Vec::new();

        for arg_def in &command_def.arguments {
            let value = arguments.get(&arg_def.id);

            if arg_def.required {
                if value.is_none() {
                    errors.push(ValidationFieldError {
                        field: arg_def.id.clone(),
                        code: "REQUIRED".to_string(),
                        message: format!("'{}' is required", arg_def.name),
                    });
                    continue;
                }
            }

            if let Some(value) = value {
                Self::validate_argument(arg_def, value, &mut errors);
            }
        }

        CommandValidationResult {
            valid: errors.is_empty(),
            errors,
        }
    }

    fn validate_argument(
        arg_def: &ArgumentDefinition,
        value: &serde_json::Value,
        errors: &mut Vec<ValidationFieldError>,
    ) {
        match &arg_def.arg_type {
            crate::models::ArgumentType::Boolean => {
                if !value.is_boolean() {
                    errors.push(ValidationFieldError {
                        field: arg_def.id.clone(),
                        code: "INVALID_TYPE".to_string(),
                        message: format!("'{}' must be a boolean", arg_def.name),
                    });
                }
            }
            crate::models::ArgumentType::Integer => {
                if let Some(n) = value.as_i64() {
                    if let Some(min) = arg_def.min {
                        if (n as f64) < min {
                            errors.push(ValidationFieldError {
                                field: arg_def.id.clone(),
                                code: "INVALID_RANGE".to_string(),
                                message: format!(
                                    "'{}' must be greater than or equal to {}",
                                    arg_def.name, min
                                ),
                            });
                        }
                    }
                    if let Some(max) = arg_def.max {
                        if (n as f64) > max {
                            errors.push(ValidationFieldError {
                                field: arg_def.id.clone(),
                                code: "INVALID_RANGE".to_string(),
                                message: format!(
                                    "'{}' must be less than or equal to {}",
                                    arg_def.name, max
                                ),
                            });
                        }
                    }
                } else {
                    errors.push(ValidationFieldError {
                        field: arg_def.id.clone(),
                        code: "INVALID_TYPE".to_string(),
                        message: format!("'{}' must be an integer", arg_def.name),
                    });
                }
            }
            crate::models::ArgumentType::Number => {
                if let Some(n) = value.as_f64() {
                    if let Some(min) = arg_def.min {
                        if n < min {
                            errors.push(ValidationFieldError {
                                field: arg_def.id.clone(),
                                code: "INVALID_RANGE".to_string(),
                                message: format!(
                                    "'{}' must be greater than or equal to {}",
                                    arg_def.name, min
                                ),
                            });
                        }
                    }
                    if let Some(max) = arg_def.max {
                        if n > max {
                            errors.push(ValidationFieldError {
                                field: arg_def.id.clone(),
                                code: "INVALID_RANGE".to_string(),
                                message: format!(
                                    "'{}' must be less than or equal to {}",
                                    arg_def.name, max
                                ),
                            });
                        }
                    }
                } else {
                    errors.push(ValidationFieldError {
                        field: arg_def.id.clone(),
                        code: "INVALID_TYPE".to_string(),
                        message: format!("'{}' must be a number", arg_def.name),
                    });
                }
            }
            crate::models::ArgumentType::String => {
                if !value.is_string() {
                    errors.push(ValidationFieldError {
                        field: arg_def.id.clone(),
                        code: "INVALID_TYPE".to_string(),
                        message: format!("'{}' must be a string", arg_def.name),
                    });
                }
            }
            crate::models::ArgumentType::Enum => {
                if let Some(s) = value.as_str() {
                    if let Some(options) = &arg_def.options {
                        if !options.contains(&s.to_string()) {
                            errors.push(ValidationFieldError {
                                field: arg_def.id.clone(),
                                code: "INVALID_ENUM".to_string(),
                                message: format!(
                                    "'{}' must be one of: {}",
                                    arg_def.name,
                                    options.join(", ")
                                ),
                            });
                        }
                    }
                } else {
                    errors.push(ValidationFieldError {
                        field: arg_def.id.clone(),
                        code: "INVALID_TYPE".to_string(),
                        message: format!("'{}' must be a string", arg_def.name),
                    });
                }
            }
            crate::models::ArgumentType::Path
            | crate::models::ArgumentType::File
            | crate::models::ArgumentType::Directory => {
                if let Some(s) = value.as_str() {
                    let path = std::path::Path::new(s);

                    match &arg_def.arg_type {
                        crate::models::ArgumentType::File => {
                            if !path.exists() {
                                errors.push(ValidationFieldError {
                                    field: arg_def.id.clone(),
                                    code: "FILE_NOT_FOUND".to_string(),
                                    message: format!("'{}' does not exist", arg_def.name),
                                });
                            }
                        }
                        crate::models::ArgumentType::Directory => {
                            if !path.is_dir() {
                                errors.push(ValidationFieldError {
                                    field: arg_def.id.clone(),
                                    code: "DIRECTORY_NOT_FOUND".to_string(),
                                    message: format!("'{}' is not a directory", arg_def.name),
                                });
                            }
                        }
                        _ => {}
                    }
                } else {
                    errors.push(ValidationFieldError {
                        field: arg_def.id.clone(),
                        code: "INVALID_TYPE".to_string(),
                        message: format!("'{}' must be a string path", arg_def.name),
                    });
                }
            }
            crate::models::ArgumentType::Duration => {
                if let Some(n) = value.as_f64() {
                    if n < 0.0 {
                        errors.push(ValidationFieldError {
                            field: arg_def.id.clone(),
                            code: "INVALID_RANGE".to_string(),
                            message: format!(
                                "'{}' must be a non-negative number",
                                arg_def.name
                            ),
                        });
                    }
                } else {
                    errors.push(ValidationFieldError {
                        field: arg_def.id.clone(),
                        code: "INVALID_TYPE".to_string(),
                        message: format!("'{}' must be a number", arg_def.name),
                    });
                }
            }
            crate::models::ArgumentType::Multiple => {
                if !value.is_array() && !value.is_string() {
                    errors.push(ValidationFieldError {
                        field: arg_def.id.clone(),
                        code: "INVALID_TYPE".to_string(),
                        message: format!("'{}' must be an array or string", arg_def.name),
                    });
                }
            }
        }
    }
}
