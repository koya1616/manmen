use crate::models::execution::ExecutionRecord;
use rusqlite::{Connection, params};
use chrono::{DateTime, Utc};
use std::path::Path;
use log::info;

pub struct Storage {
    conn: Connection,
}

impl Storage {
    pub fn new(db_path: &Path) -> Result<Self, String> {
        let conn = Connection::open(db_path)
            .map_err(|e| format!("Failed to open database: {}", e))?;

        let storage = Self { conn };
        storage.init_tables()?;
        Ok(storage)
    }

    fn init_tables(&self) -> Result<(), String> {
        self.conn
            .execute_batch(
                "CREATE TABLE IF NOT EXISTS executions (
                    id TEXT PRIMARY KEY,
                    command_id TEXT NOT NULL,
                    command TEXT NOT NULL,
                    arguments_json TEXT NOT NULL,
                    generated_command TEXT NOT NULL,
                    exit_code INTEGER,
                    success INTEGER NOT NULL,
                    stdout TEXT NOT NULL,
                    stderr TEXT NOT NULL,
                    duration_ms INTEGER,
                    started_at TEXT NOT NULL,
                    finished_at TEXT
                );

                CREATE INDEX IF NOT EXISTS idx_executions_command_id ON executions(command_id);
                CREATE INDEX IF NOT EXISTS idx_executions_started_at ON executions(started_at);",
            )
            .map_err(|e| format!("Failed to create tables: {}", e))?;

        Ok(())
    }

    pub fn save_execution(&self, record: &ExecutionRecord) -> Result<(), String> {
        self.conn
            .execute(
                "INSERT INTO executions (
                    id, command_id, command, arguments_json, generated_command,
                    exit_code, success, stdout, stderr, duration_ms,
                    started_at, finished_at
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
                params![
                    record.id,
                    record.command_id,
                    record.command,
                    record.arguments_json,
                    record.generated_command,
                    record.exit_code,
                    record.success as i32,
                    record.stdout,
                    record.stderr,
                    record.duration_ms,
                    record.started_at.to_rfc3339(),
                    record.finished_at.map(|dt| dt.to_rfc3339()),
                ],
            )
            .map_err(|e| format!("Failed to save execution: {}", e))?;

        info!("Saved execution record: {}", record.id);
        Ok(())
    }

    pub fn get_executions(&self, limit: i32) -> Result<Vec<ExecutionRecord>, String> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, command_id, command, arguments_json, generated_command,
                        exit_code, success, stdout, stderr, duration_ms,
                        started_at, finished_at
                 FROM executions
                 ORDER BY started_at DESC
                 LIMIT ?1",
            )
            .map_err(|e| format!("Failed to prepare query: {}", e))?;

        let rows = stmt
            .query_map(params![limit], |row| {
                let started_at_str: String = row.get(10)?;
                let finished_at_str: Option<String> = row.get(11)?;

                Ok(ExecutionRecord {
                    id: row.get(0)?,
                    command_id: row.get(1)?,
                    command: row.get(2)?,
                    arguments_json: row.get(3)?,
                    generated_command: row.get(4)?,
                    exit_code: row.get(5)?,
                    success: row.get::<_, i32>(6)? != 0,
                    stdout: row.get(7)?,
                    stderr: row.get(8)?,
                    duration_ms: row.get(9)?,
                    started_at: DateTime::parse_from_rfc3339(&started_at_str)
                        .map(|dt| dt.with_timezone(&Utc))
                        .unwrap_or_else(|_| Utc::now()),
                    finished_at: finished_at_str.and_then(|s| {
                        DateTime::parse_from_rfc3339(&s)
                            .map(|dt| dt.with_timezone(&Utc))
                            .ok()
                    }),
                })
            })
            .map_err(|e| format!("Failed to query executions: {}", e))?;

        let mut records = Vec::new();
        for row in rows {
            records.push(row.map_err(|e| format!("Failed to read row: {}", e))?);
        }

        Ok(records)
    }

    pub fn get_execution(&self, id: &str) -> Result<Option<ExecutionRecord>, String> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, command_id, command, arguments_json, generated_command,
                        exit_code, success, stdout, stderr, duration_ms,
                        started_at, finished_at
                 FROM executions
                 WHERE id = ?1",
            )
            .map_err(|e| format!("Failed to prepare query: {}", e))?;

        let mut rows = stmt
            .query_map(params![id], |row| {
                let started_at_str: String = row.get(10)?;
                let finished_at_str: Option<String> = row.get(11)?;

                Ok(ExecutionRecord {
                    id: row.get(0)?,
                    command_id: row.get(1)?,
                    command: row.get(2)?,
                    arguments_json: row.get(3)?,
                    generated_command: row.get(4)?,
                    exit_code: row.get(5)?,
                    success: row.get::<_, i32>(6)? != 0,
                    stdout: row.get(7)?,
                    stderr: row.get(8)?,
                    duration_ms: row.get(9)?,
                    started_at: DateTime::parse_from_rfc3339(&started_at_str)
                        .map(|dt| dt.with_timezone(&Utc))
                        .unwrap_or_else(|_| Utc::now()),
                    finished_at: finished_at_str.and_then(|s| {
                        DateTime::parse_from_rfc3339(&s)
                            .map(|dt| dt.with_timezone(&Utc))
                            .ok()
                    }),
                })
            })
            .map_err(|e| format!("Failed to query execution: {}", e))?;

        match rows.next() {
            Some(row) => Ok(Some(row.map_err(|e| format!("Failed to read row: {}", e))?)),
            None => Ok(None),
        }
    }
}
