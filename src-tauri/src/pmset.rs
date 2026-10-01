//! `pmset -<scope> <setting> <value>` の定義と現在値の取得。
//! 管理者権限が必要なのは設定変更のみ。`pmset -g` は権限不要。
//!
//! 渡せる対象・項目・値は許可リストに限る。schedule や sleepnow のような
//! 即時操作、パス指定、サポート外の項目は受け付けない。

use serde::Serialize;

use crate::privileged;
use crate::spec::AdminCommand;
use crate::types::CommandResult;

const PROGRAM: &str = "/usr/bin/pmset";

/// 分単位タイマーの上限 (24時間)。0 は無効。
const MAX_MINUTES: u32 = 1440;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ValueKind {
    /// 0 または 1
    Bool,
    /// 0..=MAX_MINUTES
    Minutes,
    /// `man pmset` の hibernatemode: 0 / 3 / 25
    Hibernate,
}

struct SettingDef {
    name: &'static str,
    kind: ValueKind,
}

const SETTINGS: &[SettingDef] = &[
    SettingDef { name: "disablesleep", kind: ValueKind::Bool },
    SettingDef { name: "displaysleep", kind: ValueKind::Minutes },
    SettingDef { name: "disksleep", kind: ValueKind::Minutes },
    SettingDef { name: "sleep", kind: ValueKind::Minutes },
    SettingDef { name: "womp", kind: ValueKind::Bool },
    SettingDef { name: "ring", kind: ValueKind::Bool },
    SettingDef { name: "powernap", kind: ValueKind::Bool },
    SettingDef { name: "proximitywake", kind: ValueKind::Bool },
    SettingDef { name: "autorestart", kind: ValueKind::Bool },
    SettingDef { name: "autorestartatconnect", kind: ValueKind::Bool },
    SettingDef { name: "lidwake", kind: ValueKind::Bool },
    SettingDef { name: "acwake", kind: ValueKind::Bool },
    SettingDef { name: "lessbright", kind: ValueKind::Bool },
    SettingDef { name: "halfdim", kind: ValueKind::Bool },
    SettingDef { name: "sms", kind: ValueKind::Bool },
    SettingDef { name: "ttyskeepawake", kind: ValueKind::Bool },
    SettingDef { name: "destroyfvkeyonstandby", kind: ValueKind::Bool },
    SettingDef { name: "hibernatemode", kind: ValueKind::Hibernate },
];

const SCOPES: &[&str] = &["a", "b", "c", "u"];

/// `pmset -g custom` で読めた1項目。Frontend の `PmsetValue` と一致させること。
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct PmsetValue {
    pub name: String,
    pub value: String,
}

/// 現在の電源設定。Frontend の `PmsetState` と一致させること。
#[derive(Debug, Clone, Serialize)]
pub struct PmsetState {
    pub battery: Vec<PmsetValue>,
    pub ac: Vec<PmsetValue>,
    pub ups: Vec<PmsetValue>,
    /// `pmset -g` の SleepDisabled。disablesleep の現在値 (0 / 1)。
    pub sleep_disabled: Option<String>,
}

/// `sudo pmset -<scope> <setting> <value>`
pub struct Pmset {
    scope: String,
    setting: String,
    value: String,
}

impl Pmset {
    pub fn new(scope: &str, setting: &str, value: &str) -> Result<Self, String> {
        let scope = scope.trim().to_string();
        let setting = setting.trim().to_string();
        let value = canonicalize_value(&setting, value)?;
        if !SCOPES.contains(&scope.as_str()) {
            return Err(format!("電源の対象が不正です: {scope}"));
        }
        Ok(Self { scope, setting, value })
    }

    pub fn preview(&self) -> String {
        format!("sudo pmset -{} {} {}", self.scope, self.setting, self.value)
    }
}

impl AdminCommand for Pmset {
    const ID: &'static str = "pmset.apply";
    const PROGRAM: &'static str = PROGRAM;

    fn args(&self) -> Vec<String> {
        vec![
            format!("-{}", self.scope),
            self.setting.clone(),
            self.value.clone(),
        ]
    }

    fn preview(&self) -> String {
        Pmset::preview(self)
    }
}

pub fn apply(scope: String, setting: String, value: String) -> Result<CommandResult, String> {
    #[cfg(not(target_os = "macos"))]
    {
        let _ = (scope, setting, value);
        return Err("pmset is only supported on macOS".to_string());
    }

    #[cfg(target_os = "macos")]
    return Pmset::new(&scope, &setting, &value)?.run();
}

/// 現在値を取得する (管理者権限不要)。
pub fn current() -> Result<PmsetState, String> {
    #[cfg(not(target_os = "macos"))]
    return Err("pmset is only supported on macOS".to_string());

    #[cfg(target_os = "macos")]
    {
        let custom = privileged::execute_plain(PROGRAM, &["-g", "custom"], "pmset -g custom".into())?;
        if !custom.success {
            return Err(format!(
                "pmset -g custom failed (exit {}): {}",
                custom.exit_code,
                custom.stderr.trim()
            ));
        }
        let system = privileged::execute_plain(PROGRAM, &["-g"], "pmset -g".into())?;
        if !system.success {
            return Err(format!(
                "pmset -g failed (exit {}): {}",
                system.exit_code,
                system.stderr.trim()
            ));
        }
        Ok(PmsetState {
            battery: parse_section(&custom.stdout, "Battery Power:"),
            ac: parse_section(&custom.stdout, "AC Power:"),
            ups: parse_section(&custom.stdout, "UPS Power:"),
            sleep_disabled: parse_sleep_disabled(&system.stdout),
        })
    }
}

fn setting_def(name: &str) -> Result<&'static SettingDef, String> {
    SETTINGS
        .iter()
        .find(|def| def.name == name)
        .ok_or_else(|| format!("設定項目が不正です: {name}"))
}

fn canonicalize_value(setting: &str, value: &str) -> Result<String, String> {
    let def = setting_def(setting)?;
    let value = value.trim();
    match def.kind {
        ValueKind::Bool => {
            if value == "0" || value == "1" {
                Ok(value.to_string())
            } else {
                Err("値は 0 か 1 です".to_string())
            }
        }
        ValueKind::Minutes => {
            let minutes = value
                .parse::<u32>()
                .map_err(|_| format!("値は 0〜{MAX_MINUTES} の整数（分）です"))?;
            if minutes > MAX_MINUTES {
                return Err(format!("値は 0〜{MAX_MINUTES} の整数（分）です"));
            }
            if value.starts_with('+') || value.starts_with('-') {
                return Err(format!("値は 0〜{MAX_MINUTES} の整数（分）です"));
            }
            Ok(minutes.to_string())
        }
        ValueKind::Hibernate => {
            if matches!(value, "0" | "3" | "25") {
                Ok(value.to_string())
            } else {
                Err("hibernatemode は 0、3、25 のいずれかです".to_string())
            }
        }
    }
}

/// `header` のセクション内から許可リストの項目だけを拾う。
/// 値は最初のトークンだけ使う (`sleep 1 (sleep prevented by ...)` → `1`)。
fn parse_section(output: &str, header: &str) -> Vec<PmsetValue> {
    let mut in_section = false;
    let mut values = Vec::new();
    for line in output.lines() {
        let trimmed = line.trim();
        if trimmed.ends_with(':') {
            in_section = trimmed == header;
            continue;
        }
        if !in_section {
            continue;
        }
        let mut parts = trimmed.split_whitespace();
        let Some(name) = parts.next() else { continue };
        let Some(value) = parts.next() else { continue };
        if setting_def(name).is_ok() {
            values.push(PmsetValue {
                name: name.to_string(),
                value: value.to_string(),
            });
        }
    }
    values
}

fn parse_sleep_disabled(output: &str) -> Option<String> {
    for line in output.lines() {
        let lower = line.to_lowercase();
        if lower.contains("sleepdisabled") || lower.contains("disablesleep") {
            let token = line.split_whitespace().last()?;
            if token == "0" || token == "1" {
                return Some(token.to_string());
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::{parse_section, parse_sleep_disabled, Pmset};

    #[test]
    fn previews_selected_setting() {
        assert_eq!(
            Pmset::new("a", "disablesleep", "1").unwrap().preview(),
            "sudo pmset -a disablesleep 1"
        );
        assert_eq!(
            Pmset::new("b", "displaysleep", "10").unwrap().preview(),
            "sudo pmset -b displaysleep 10"
        );
        assert_eq!(
            Pmset::new(" c ", "hibernatemode", "25").unwrap().preview(),
            "sudo pmset -c hibernatemode 25"
        );
    }

    #[test]
    fn rejects_unknown_options_and_values() {
        assert!(Pmset::new("-a", "disablesleep", "1").is_err());
        assert!(Pmset::new("a", "sleepnow", "1").is_err());
        assert!(Pmset::new("a", "hibernatefile", "/var/vm/sleepimage").is_err());
        assert!(Pmset::new("a", "networkoversleep", "0").is_err());
        assert!(Pmset::new("a", "disablesleep", "2").is_err());
        assert!(Pmset::new("a", "sleep", "1441").is_err());
        assert!(Pmset::new("a", "sleep", "-1").is_err());
        assert!(Pmset::new("a", "hibernatemode", "1").is_err());
        assert!(Pmset::new("a", "displaysleep", "0").is_ok());
    }

    #[test]
    fn parses_custom_and_sleep_disabled() {
        let custom = "\
Battery Power:
 displaysleep         2
 sleep                1 (sleep prevented by sharingd)
 womp                 0
 hibernatefile        /var/vm/sleepimage
AC Power:
 displaysleep         60
 Sleep On Power Button 1
UPS Power:
 disksleep            10
";
        assert_eq!(
            parse_section(custom, "Battery Power:")
                .iter()
                .map(|v| (v.name.as_str(), v.value.as_str()))
                .collect::<Vec<_>>(),
            vec![("displaysleep", "2"), ("sleep", "1"), ("womp", "0")]
        );
        assert_eq!(
            parse_section(custom, "AC Power:").len(),
            1
        );
        assert_eq!(
            parse_section(custom, "UPS Power:")[0].name,
            "disksleep"
        );
        assert_eq!(
            parse_sleep_disabled("System-wide power settings:\n SleepDisabled\t\t0\n"),
            Some("0".to_string())
        );
        assert_eq!(parse_sleep_disabled("displaysleep 10\n"), None);
    }
}
