//! `system_profiler` の定義・実行・JSON パース。管理者権限は不要。
//! 引数は `-json <DataType>` のみ。DataType は許可リストで検証する。
//! 全文スキャンは遅いため1種類ずつ取得し、`-json` の出力をそのまま返す。
//! 型ごとにスキーマが違うため、Frontend は汎用ツリーで表示する。

use serde::{Deserialize, Serialize};

use crate::privileged;

/// 取得を許可するデータ型 (`system_profiler -listDataTypes` の実在名のみ)。
/// アプリ・フォント等の巨大・低速な型は外す。
/// Frontend の `SYSTEM_PROFILER_TYPES` と一致させること。
pub const ALLOWED_DATA_TYPES: &[&str] = &[
    "SPHardwareDataType",
    "SPSoftwareDataType",
    "SPMemoryDataType",
    "SPStorageDataType",
    "SPDisplaysDataType",
    "SPPowerDataType",
    "SPNetworkDataType",
    "SPAirPortDataType",
    "SPBluetoothDataType",
    "SPUSBHostDataType",
    "SPThunderboltDataType",
    "SPAudioDataType",
    "SPCameraDataType",
    "SPPrintersDataType",
    "SPFirewallDataType",
    "SPNetworkLocationDataType",
    "SPInstallHistoryDataType",
    "SPStartupItemDataType",
    "SPConfigurationProfileDataType",
    "SPDeveloperToolsDataType",
    "SPDiagnosticsDataType",
    "SPNVMeDataType",
];

pub const DEFAULT_DATA_TYPE: &str = "SPHardwareDataType";

/// Frontend の `getSystemProfiler` 引数とフィールドを一致させること。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemProfilerQuery {
    #[serde(default = "default_data_type")]
    pub data_type: String,
}

fn default_data_type() -> String {
    DEFAULT_DATA_TYPE.to_string()
}

/// `get_system_profiler` の返却値。Frontend の `SystemProfilerSnapshot` と一致させること。
/// `value` は `-json` 出力の当該データ型の中身 (通常は配列)。
#[derive(Debug, Clone, Serialize)]
pub struct SystemProfilerSnapshot {
    pub success: bool,
    pub exit_code: i32,
    pub command: String,
    pub data_type: String,
    pub value: serde_json::Value,
    pub stderr: String,
}

/// `system_profiler -json <data_type>`
pub struct SystemProfiler {
    data_type: String,
}

impl SystemProfiler {
    pub fn new(query: SystemProfilerQuery) -> Result<Self, String> {
        let data_type = query.data_type.trim().to_string();
        validate_data_type(&data_type)?;
        Ok(Self { data_type })
    }

    pub fn preview(&self) -> String {
        format!("system_profiler -json {}", self.data_type)
    }

    pub fn run(&self) -> Result<SystemProfilerSnapshot, String> {
        let preview = self.preview();
        let output = privileged::execute_plain(
            "system_profiler",
            &["-json", &self.data_type],
            preview.clone(),
        )?;
        if !output.success {
            return Ok(failed(
                preview,
                self.data_type.clone(),
                output.exit_code,
                prefer_output(&output.stderr, &output.stdout),
            ));
        }
        let parsed: serde_json::Value = serde_json::from_str(&output.stdout)
            .map_err(|e| format!("system_profiler の JSON を解釈できませんでした: {e}"))?;
        // `{"SPHardwareDataType": [...]}` の中身を取り出す。形が違えば全体を返す。
        let value = parsed
            .get(&self.data_type)
            .cloned()
            .unwrap_or(parsed);
        Ok(SystemProfilerSnapshot {
            success: true,
            exit_code: output.exit_code,
            command: preview,
            data_type: self.data_type.clone(),
            value,
            stderr: output.stderr.trim().to_string(),
        })
    }
}

fn failed(command: String, data_type: String, exit_code: i32, stderr: String) -> SystemProfilerSnapshot {
    SystemProfilerSnapshot {
        success: false,
        exit_code,
        command,
        data_type,
        value: serde_json::Value::Null,
        stderr,
    }
}

fn prefer_output(stderr: &str, stdout: &str) -> String {
    let stderr = stderr.trim();
    if !stderr.is_empty() {
        return stderr.to_string();
    }
    let stdout = stdout.trim();
    if !stdout.is_empty() {
        return stdout.to_string();
    }
    "システム情報を取得できませんでした".to_string()
}

/// 許可リストにある型のみ通す。`-` 始まりや未知の型は受け付けない。
fn validate_data_type(data_type: &str) -> Result<(), String> {
    if ALLOWED_DATA_TYPES.contains(&data_type) {
        Ok(())
    } else {
        Err(format!("データ型が不正です: {data_type}"))
    }
}

pub fn get_snapshot(query: SystemProfilerQuery) -> Result<SystemProfilerSnapshot, String> {
    #[cfg(not(target_os = "macos"))]
    {
        let _ = query;
        return Err("system_profiler is only supported on macOS".to_string());
    }

    #[cfg(target_os = "macos")]
    return SystemProfiler::new(query).and_then(|cmd| cmd.run());
}

#[cfg(test)]
mod tests {
    use super::{get_snapshot, validate_data_type, SystemProfiler, SystemProfilerQuery};

    fn query(data_type: &str) -> SystemProfilerQuery {
        SystemProfilerQuery {
            data_type: data_type.to_string(),
        }
    }

    #[test]
    fn previews_json_command() {
        let cmd = SystemProfiler::new(query("SPHardwareDataType")).unwrap();
        assert_eq!(cmd.preview(), "system_profiler -json SPHardwareDataType");
    }

    #[test]
    fn rejects_unknown_types() {
        assert!(SystemProfiler::new(query("SPHardwareDataType")).is_ok());
        assert!(SystemProfiler::new(query("")).is_err());
        assert!(SystemProfiler::new(query("-json")).is_err());
        assert!(SystemProfiler::new(query("SPApplicationsDataType")).is_err());
        assert!(SystemProfiler::new(query("SPHardwareDataType; rm -rf /")).is_err());
        assert!(validate_data_type("SPFontsDataType").is_err());
    }

    #[test]
    fn runs_hardware_profiler() {
        let result = get_snapshot(query("SPHardwareDataType")).expect("system_profiler");
        assert!(result.success, "stderr: {}", result.stderr);
        assert!(result.value.is_array());
        assert!(!result.value.as_array().unwrap().is_empty());
    }
}
