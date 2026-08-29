use crate::models::execution::CommandResult;
use std::process::Command;
use std::time::{Duration, Instant};
use chrono::Utc;
use uuid::Uuid;

pub struct Executor;

impl Executor {
    pub fn execute(
        program: &str,
        args: &[String],
        timeout_ms: Option<u64>,
    ) -> Result<CommandResult, String> {
        let execution_id = Uuid::new_v4().to_string();
        let started_at = Utc::now();
        let start = Instant::now();

        let timeout = Duration::from_millis(timeout_ms.unwrap_or(30000));

        let output = Self::run_process(program, args, timeout)
            .map_err(|e| format!("Failed to execute command: {}", e))?;

        let duration = start.elapsed();
        let finished_at = Utc::now();

        let exit_code = output.status.code().unwrap_or(-1);
        let success = output.status.success();

        Ok(CommandResult {
            execution_id,
            success,
            exit_code,
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            duration_ms: duration.as_millis() as u64,
            started_at,
            finished_at,
        })
    }

    fn run_process(
        program: &str,
        args: &[String],
        _timeout: Duration,
    ) -> Result<std::process::Output, String> {
        let mut cmd = Command::new(program);
        cmd.args(args);

        let child = cmd
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to spawn process: {}", e))?;

        let output = child
            .wait_with_output()
            .map_err(|e| format!("Failed to wait for process: {}", e))?;

        Ok(output)
    }

    pub fn validate_binary(command_def: &crate::models::CommandDefinition) -> Result<(), String> {
        if let Some(binary) = &command_def.binary {
            for path in &binary.paths {
                if std::path::Path::new(path).exists() {
                    return Ok(());
                }
            }
            return Err(format!(
                "Binary '{}' not found at any expected path: {:?}",
                binary.name, binary.paths
            ));
        }

        // If no binary definition, check if command exists in PATH
        which::which(&command_def.command)
            .map_err(|_| format!("Command '{}' not found in PATH", command_def.command))?;

        Ok(())
    }
}
