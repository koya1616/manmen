use crate::models::execution::CommandResult;
use chrono::Utc;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{ChildStderr, ChildStdout, Command};
use std::thread::JoinHandle;
use std::time::Duration;
use uuid::Uuid;
use wait_timeout::ChildExt;

pub struct Executor;

enum ProcessError {
    TimedOut,
    Io(std::io::Error),
}

impl From<std::io::Error> for ProcessError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl Executor {
    #[cfg(target_os = "macos")]
    pub fn execute_as_admin(
        program: &str,
        args: &[String],
        timeout_ms: Option<u64>,
    ) -> Result<CommandResult, String> {
        let command = std::iter::once(program)
            .chain(args.iter().map(String::as_str))
            .map(Self::shell_quote)
            .collect::<Vec<_>>()
            .join(" ");
        let osascript_args = vec![
            "-e".to_string(),
            "on run argv".to_string(),
            "-e".to_string(),
            "do shell script (item 1 of argv) with administrator privileges".to_string(),
            "-e".to_string(),
            "end run".to_string(),
            command,
        ];

        // Authentication dialogs need enough time for the user to respond.
        let timeout_ms = timeout_ms.unwrap_or(30_000).max(120_000);
        Self::execute("/usr/bin/osascript", &osascript_args, Some(timeout_ms))
    }

    #[cfg(not(target_os = "macos"))]
    pub fn execute_as_admin(
        _program: &str,
        _args: &[String],
        _timeout_ms: Option<u64>,
    ) -> Result<CommandResult, String> {
        Err("Administrator execution is only supported on macOS".to_string())
    }

    fn shell_quote(value: &str) -> String {
        if value.is_empty() {
            return "''".to_string();
        }

        format!("'{}'", value.replace('\'', "'\"'\"'"))
    }

    pub fn execute(
        program: &str,
        args: &[String],
        timeout_ms: Option<u64>,
    ) -> Result<CommandResult, String> {
        let execution_id = Uuid::new_v4().to_string();
        let started_at = Utc::now();
        let timeout_duration = Duration::from_millis(timeout_ms.unwrap_or(30000));

        log::info!(
            "Executing: {} {} (timeout={}ms)",
            program,
            args.join(" "),
            timeout_duration.as_millis()
        );

        let (exit_code, success, stdout, stderr) =
            match Self::run_process_with_timeout(program, args, timeout_duration) {
                Ok((exit_code, success, stdout, stderr)) => {
                    log::info!(
                        "Command completed: exit_code={} success={}",
                        exit_code,
                        success
                    );
                    (exit_code, success, stdout, stderr)
                }
                Err(error) => {
                    if matches!(error, ProcessError::TimedOut) {
                        let msg = format!(
                            "Command timed out after {}ms",
                            timeout_duration.as_millis()
                        );
                        log::warn!("{}", msg);
                        (-1, false, String::new(), msg)
                    } else {
                        let ProcessError::Io(error) = error else {
                            unreachable!();
                        };
                        let msg = format!("Process failed: {}", error);
                        log::warn!("{}", msg);
                        (-1, false, String::new(), msg)
                    }
                }
            };

        let duration = Utc::now().signed_duration_since(started_at);
        let finished_at = Utc::now();

        if !success && Self::is_permission_error(&stderr, exit_code) {
            let msg = format!(
                "Permission denied: '{}' requires administrator privileges. {}",
                program,
                Self::get_permission_hint(program)
            );
            log::warn!("{}", msg);
            return Err(msg);
        }

        if !stderr.is_empty() && Self::is_benign_error(&stderr) {
            log::info!(
                "Benign stderr filtered: {}",
                &stderr[..stderr.len().min(100)]
            );
        }

        Ok(CommandResult {
            execution_id,
            success,
            exit_code,
            stdout,
            stderr,
            duration_ms: duration.num_milliseconds() as u64,
            started_at,
            finished_at,
        })
    }

    fn is_permission_error(stderr: &str, exit_code: i32) -> bool {
        let stderr_lower = stderr.to_lowercase();
        stderr_lower.contains("must be run as root")
            || stderr_lower.contains("permission denied")
            || stderr_lower.contains("operation not permitted")
            || stderr_lower.contains("not allowed")
            || exit_code == 126
            || exit_code == 127
    }

    fn is_benign_error(stderr: &str) -> bool {
        stderr.contains("error messaging the mach port for IMKCFRunLoopWakeUpReliable")
            || stderr.contains("IMKCFRunLoopWakeUpReliable")
    }

    fn get_permission_hint(program: &str) -> &str {
        let command_name = Path::new(program)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or(program);

        match command_name {
            "pmset" => "Use 'sudo pmset' in Terminal, or run this app as administrator.",
            "launchctl" => "Some launchctl operations require root privileges.",
            "defaults" => "Some defaults commands require root privileges for system-wide settings.",
            _ => "This command may require administrator privileges.",
        }
    }

    fn run_process_with_timeout(
        program: &str,
        args: &[String],
        timeout: Duration,
    ) -> Result<(i32, bool, String, String), ProcessError> {
        let mut cmd = Command::new(program);
        cmd.args(args);

        cmd.stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped());

        let mut child = cmd.spawn().map_err(ProcessError::Io)?;
        let stdout = child.stdout.take().ok_or_else(|| {
            ProcessError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                "failed to capture stdout",
            ))
        })?;
        let stderr = child.stderr.take().ok_or_else(|| {
            ProcessError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                "failed to capture stderr",
            ))
        })?;
        let stdout_reader = Self::read_stdout(stdout);
        let stderr_reader = Self::read_stderr(stderr);

        match child.wait_timeout(timeout).map_err(ProcessError::Io)? {
            Some(status) => {
                let (stdout, stderr) = Self::collect_output(stdout_reader, stderr_reader)?;
                let exit_code = status.code().unwrap_or(-1);
                let success = status.success();
                Ok((exit_code, success, stdout, stderr))
            }
            None => {
                let _ = child.kill();
                let _ = child.wait();
                let _ = Self::collect_output(stdout_reader, stderr_reader);
                Err(ProcessError::TimedOut)
            }
        }
    }

    fn read_stdout(mut stdout: ChildStdout) -> JoinHandle<std::io::Result<Vec<u8>>> {
        std::thread::spawn(move || {
            let mut bytes = Vec::new();
            stdout.read_to_end(&mut bytes)?;
            Ok(bytes)
        })
    }

    fn read_stderr(mut stderr: ChildStderr) -> JoinHandle<std::io::Result<Vec<u8>>> {
        std::thread::spawn(move || {
            let mut bytes = Vec::new();
            stderr.read_to_end(&mut bytes)?;
            Ok(bytes)
        })
    }

    fn collect_output(
        stdout_reader: JoinHandle<std::io::Result<Vec<u8>>>,
        stderr_reader: JoinHandle<std::io::Result<Vec<u8>>>,
    ) -> Result<(String, String), ProcessError> {
        let stdout = stdout_reader.join().map_err(|_| {
            ProcessError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                "stdout reader thread panicked",
            ))
        })??;
        let stderr = stderr_reader.join().map_err(|_| {
            ProcessError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                "stderr reader thread panicked",
            ))
        })??;

        Ok((
            String::from_utf8_lossy(&stdout).to_string(),
            String::from_utf8_lossy(&stderr).to_string(),
        ))
    }

    pub fn resolve_program(
        command_def: &crate::models::CommandDefinition,
    ) -> Result<PathBuf, String> {
        if let Some(binary) = &command_def.binary {
            if let Some(path) = binary
                .paths
                .iter()
                .map(PathBuf::from)
                .find(|path| path.is_file())
            {
                return Ok(path);
            }
        }

        which::which(&command_def.command)
            .map_err(|error| format!("Command '{}' not found: {}", command_def.command, error))
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

        which::which(&command_def.command)
            .map_err(|_| format!("Command '{}' not found in PATH", command_def.command))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Executor;

    #[test]
    fn captures_output_after_waiting_with_timeout() {
        let result = Executor::execute(
            "/usr/bin/printf",
            &["process completed".to_string()],
            Some(1_000),
        )
        .unwrap();

        assert!(result.success);
        assert_eq!(result.exit_code, 0);
        assert_eq!(result.stdout, "process completed");
        assert!(result.stderr.is_empty());
    }

    #[test]
    fn preserves_process_start_error() {
        let result = Executor::execute(
            "/path/that/does/not/exist",
            &[],
            Some(1_000),
        )
        .unwrap();

        assert!(!result.success);
        assert_eq!(result.exit_code, -1);
        assert!(result.stderr.starts_with("Process failed:"));
    }

    #[test]
    fn shell_quotes_privileged_command_arguments() {
        assert_eq!(Executor::shell_quote(""), "''");
        assert_eq!(Executor::shell_quote("plain"), "'plain'");
        assert_eq!(Executor::shell_quote("two words"), "'two words'");
        assert_eq!(Executor::shell_quote("it's"), "'it'\"'\"'s'");
        assert_eq!(
            Executor::shell_quote("$(touch /tmp/should-not-run)"),
            "'$(touch /tmp/should-not-run)'"
        );
    }
}
