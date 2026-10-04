//! `scutil` の参照系の定義・実行・出力パース。管理者権限は不要。
//! 対象は `--dns` / `--nwi` / `--get <名前>` のみ。`--set` (書き込み) と
//! `--proxy` は受け付けない。引数は固定で、自由入力は一切ない。

use serde::{Deserialize, Serialize};

use crate::privileged;

/// 許可するサブコマンド。Frontend の `ScutilSub` と一致させること。
pub const ALLOWED_SUBS: &[&str] = &["dns", "nwi", "names"];

/// `--get` で読む名前キー (固定)。
const NAME_KEYS: &[&str] = &["ComputerName", "LocalHostName", "HostName"];

/// Frontend の `getScutil` 引数とフィールドを一致させること。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScutilQuery {
    #[serde(default = "default_sub")]
    pub sub: String,
}

fn default_sub() -> String {
    "dns".to_string()
}

/// `key : value` の1行。Frontend の `ScutilEntry` とフィールドを一致させること。
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct ScutilEntry {
    pub key: String,
    pub value: String,
}

/// `resolver #N` の塊。Frontend の `DnsResolver` と一致させること。
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct DnsResolver {
    pub name: String,
    pub entries: Vec<ScutilEntry>,
}

/// `DNS configuration ...` の区切り。Frontend の `DnsSection` と一致させること。
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct DnsSection {
    pub title: String,
    pub resolvers: Vec<DnsResolver>,
}

/// `get_scutil dns` の返却値。Frontend の `DnsSnapshot` と一致させること。
#[derive(Debug, Clone, Serialize)]
pub struct DnsSnapshot {
    pub success: bool,
    pub exit_code: i32,
    pub command: String,
    pub sections: Vec<DnsSection>,
    pub stderr: String,
}

/// nwi の1行。継続行は直前の iface を引き継ぐ。Frontend の `NwiRow` と一致させること。
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct NwiRow {
    pub iface: String,
    pub key: String,
    pub value: String,
}

/// `IPv4 ... information` の区切り。Frontend の `NwiSection` と一致させること。
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct NwiSection {
    pub title: String,
    pub rows: Vec<NwiRow>,
}

/// `get_scutil nwi` の返却値。Frontend の `NwiSnapshot` と一致させること。
#[derive(Debug, Clone, Serialize)]
pub struct NwiSnapshot {
    pub success: bool,
    pub exit_code: i32,
    pub command: String,
    pub sections: Vec<NwiSection>,
    pub footer: String,
    pub stderr: String,
}

/// `--get` の1項目。未設定は value が空。Frontend の `NameEntry` と一致させること。
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct NameEntry {
    pub key: String,
    pub value: String,
}

/// `get_scutil names` の返却値。Frontend の `NamesSnapshot` と一致させること。
#[derive(Debug, Clone, Serialize)]
pub struct NamesSnapshot {
    pub success: bool,
    pub exit_code: i32,
    pub command: String,
    pub names: Vec<NameEntry>,
    pub stderr: String,
}

pub enum ScutilSnapshot {
    Dns(DnsSnapshot),
    Nwi(NwiSnapshot),
    Names(NamesSnapshot),
}

pub fn get_snapshot(query: ScutilQuery) -> Result<ScutilResult, String> {
    #[cfg(not(target_os = "macos"))]
    {
        let _ = query;
        return Err("scutil is only supported on macOS".to_string());
    }

    #[cfg(target_os = "macos")]
    return run_snapshot(query);
}

/// Frontend へ返す共用体。`sub` ごとに中身が変わる。
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum ScutilResult {
    Dns(DnsSnapshot),
    Nwi(NwiSnapshot),
    Names(NamesSnapshot),
}

#[cfg(target_os = "macos")]
fn run_snapshot(query: ScutilQuery) -> Result<ScutilResult, String> {
    let sub = query.sub.trim().to_string();
    if !ALLOWED_SUBS.contains(&sub.as_str()) {
        return Err(format!("サブコマンドが不正です: {sub}"));
    }
    match sub.as_str() {
        "dns" => run_dns().map(ScutilResult::Dns),
        "nwi" => run_nwi().map(ScutilResult::Nwi),
        _ => run_names().map(ScutilResult::Names),
    }
}

/// `scutil --dns`
#[cfg(target_os = "macos")]
fn run_dns() -> Result<DnsSnapshot, String> {
    let preview = "scutil --dns".to_string();
    let output = privileged::execute_plain("scutil", &["--dns"], preview.clone())?;
    let sections = parse_dns(&output.stdout);
    Ok(DnsSnapshot {
        success: output.success,
        exit_code: output.exit_code,
        command: preview,
        sections,
        stderr: output.stderr.trim().to_string(),
    })
}

/// 空行で区切り、`resolver #N` を塊の開始、`key : value` を項目として読む。
/// `DNS configuration ...` の見出し行はセクション名になる。
fn parse_dns(stdout: &str) -> Vec<DnsSection> {
    let mut sections: Vec<DnsSection> = Vec::new();
    let mut current: Option<DnsSection> = None;
    let mut resolver: Option<DnsResolver> = None;

    let flush_resolver = |current: &mut Option<DnsSection>, resolver: &mut Option<DnsResolver>| {
        if let Some(done) = resolver.take() {
            if let Some(section) = current.as_mut() {
                section.resolvers.push(done);
            }
        }
    };

    for line in stdout.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if let Some(name) = trimmed.strip_prefix("resolver #") {
            flush_resolver(&mut current, &mut resolver);
            resolver = Some(DnsResolver {
                name: format!("resolver #{name}"),
                entries: Vec::new(),
            });
            continue;
        }
        if let Some((key, value)) = trimmed.split_once(':') {
            if let Some(active) = resolver.as_mut() {
                active.entries.push(ScutilEntry {
                    key: key.trim().to_string(),
                    value: value.trim().to_string(),
                });
            }
            continue;
        }
        // `:` のない行はセクション見出し (`DNS configuration ...`)
        flush_resolver(&mut current, &mut resolver);
        if let Some(done) = current.take() {
            sections.push(done);
        }
        current = Some(DnsSection {
            title: trimmed.to_string(),
            resolvers: Vec::new(),
        });
    }
    flush_resolver(&mut current, &mut resolver);
    if let Some(done) = current.take() {
        sections.push(done);
    }
    // 見出しなしで始まることはないが、念のため空セクションは落とす
    sections
        .into_iter()
        .filter(|section| !section.resolvers.is_empty())
        .collect()
}

/// `scutil --nwi`
#[cfg(target_os = "macos")]
fn run_nwi() -> Result<NwiSnapshot, String> {
    let preview = "scutil --nwi".to_string();
    let output = privileged::execute_plain("scutil", &["--nwi"], preview.clone())?;
    let (sections, footer) = parse_nwi(&output.stdout);
    Ok(NwiSnapshot {
        success: output.success,
        exit_code: output.exit_code,
        command: preview,
        sections,
        footer,
        stderr: output.stderr.trim().to_string(),
    })
}

/// 行頭非空白・`:` なしの行をセクション見出し、`a : b : c` を
/// (iface, key, value)、`Network interfaces: ...` をフッタとして読む。
/// 継続行 (iface 空欄) は直前の iface を引き継ぐ。
fn parse_nwi(stdout: &str) -> (Vec<NwiSection>, String) {
    let mut sections: Vec<NwiSection> = Vec::new();
    let mut current: Option<NwiSection> = None;
    let mut footer = String::new();
    let mut iface = String::new();

    for line in stdout.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if let Some(list) = trimmed.strip_prefix("Network interfaces:") {
            footer = list.trim().to_string();
            continue;
        }
        let indented = line.starts_with(' ') || line.starts_with('\t');
        if !indented && !trimmed.contains(':') {
            if let Some(done) = current.take() {
                sections.push(done);
            }
            current = Some(NwiSection {
                title: trimmed.to_string(),
                rows: Vec::new(),
            });
            iface.clear();
            continue;
        }
        let parts: Vec<&str> = trimmed.split(':').map(str::trim).collect();
        let row = match parts.as_slice() {
            [a, b, c] => {
                if !a.is_empty() {
                    iface = (*a).to_string();
                }
                NwiRow {
                    iface: iface.clone(),
                    key: (*b).to_string(),
                    value: (*c).to_string(),
                }
            }
            [a, b] => NwiRow {
                iface: iface.clone(),
                key: (*a).to_string(),
                value: (*b).to_string(),
            },
            _ => continue,
        };
        if let Some(section) = current.as_mut() {
            section.rows.push(row);
        }
    }
    if let Some(done) = current.take() {
        sections.push(done);
    }
    // 先頭の `Network information` など行だけの見出しは空になるので落とす
    let sections: Vec<NwiSection> = sections
        .into_iter()
        .filter(|section| !section.rows.is_empty())
        .collect();
    (sections, footer)
}

/// `scutil --get <名前>` を3回叩く。未設定 (`... not set` / 空) は空文字。
#[cfg(target_os = "macos")]
fn run_names() -> Result<NamesSnapshot, String> {
    let preview = NAME_KEYS
        .iter()
        .map(|key| format!("scutil --get {key}"))
        .collect::<Vec<_>>()
        .join("; ");
    let mut names = Vec::new();
    let mut stderr = Vec::new();
    let mut exit_code = 0;
    let mut success = true;
    for key in NAME_KEYS {
        let output = privileged::execute_plain("scutil", &["--get", key], preview.clone())?;
        let (value, unset) = interpret_get(&output.stdout, &output.stderr);
        if unset {
            // 未設定は正常系。終了コード・stderr を失敗として扱わない
            names.push(NameEntry {
                key: (*key).to_string(),
                value,
            });
            continue;
        }
        if !output.success {
            success = false;
            exit_code = output.exit_code;
        }
        let err = output.stderr.trim();
        if !err.is_empty() {
            stderr.push(format!("{key}: {err}"));
        }
        names.push(NameEntry {
            key: (*key).to_string(),
            value,
        });
    }
    Ok(NamesSnapshot {
        success,
        exit_code,
        command: preview,
        names,
        stderr: stderr.join("\n"),
    })
}

/// 1行目を取り出す。空・`... not set` は未設定として空にする。
fn first_value(stdout: &str) -> String {
    let line = stdout.lines().map(str::trim).find(|l| !l.is_empty()).unwrap_or("");
    if line.is_empty() || line.ends_with("not set") {
        return String::new();
    }
    line.to_string()
}

/// `--get` 1件分の解釈。未設定は正常系として空値と true を返す。
fn interpret_get(stdout: &str, stderr: &str) -> (String, bool) {
    let value = first_value(stdout);
    let err = stderr.trim();
    if value.is_empty() && (err.is_empty() || err.ends_with("not set")) {
        return (String::new(), true);
    }
    (value, false)
}

#[cfg(test)]
mod tests {
    use super::{first_value, interpret_get, parse_dns, parse_nwi};

    const DNS_SAMPLE: &str = "\
DNS configuration

resolver #1
  nameserver[0] : 100.64.0.2
  flags    : Request A records

resolver #2
  domain   : local
  options  : mdns

DNS configuration (for scoped queries)

resolver #1
  search domain[0] : example.ne.jp
  nameserver[0] : 192.168.40.1
";

    #[test]
    fn parses_dns_sections() {
        let sections = parse_dns(DNS_SAMPLE);
        assert_eq!(sections.len(), 2);
        assert_eq!(sections[0].title, "DNS configuration");
        assert_eq!(sections[0].resolvers.len(), 2);
        assert_eq!(sections[0].resolvers[0].name, "resolver #1");
        assert_eq!(sections[0].resolvers[0].entries[0].key, "nameserver[0]");
        assert_eq!(sections[0].resolvers[0].entries[0].value, "100.64.0.2");
        assert_eq!(sections[1].title, "DNS configuration (for scoped queries)");
        assert_eq!(sections[1].resolvers[0].entries[0].key, "search domain[0]");
    }

    const NWI_SAMPLE: &str = "\
Network information

IPv4 network interface information
   utun7 : flags      : 0x7 (IPv4,IPv6,DNS)
           address    : 10.5.0.2
           reach      : 0x00000002 (Reachable)

   REACH : flags 0x00000002 (Reachable)

Network interfaces: utun7 en0
";

    #[test]
    fn parses_nwi_rows() {
        let (sections, footer) = parse_nwi(NWI_SAMPLE);
        assert_eq!(footer, "utun7 en0");
        assert_eq!(sections.len(), 1);
        let rows = &sections[0].rows;
        assert_eq!(rows[0].iface, "utun7");
        assert_eq!(rows[0].key, "flags");
        // 継続行は直前の iface を引き継ぐ
        assert_eq!(rows[1].iface, "utun7");
        assert_eq!(rows[1].key, "address");
        assert_eq!(rows[1].value, "10.5.0.2");
        assert_eq!(rows[3].iface, "utun7");
        assert_eq!(rows[3].key, "REACH");
    }

    #[test]
    fn treats_not_set_as_empty() {
        assert_eq!(first_value("青山煌矢のMacBook Pro\n"), "青山煌矢のMacBook Pro");
        assert_eq!(first_value("HostName: not set\n"), "");
        assert_eq!(first_value(""), "");
    }

    #[test]
    fn treats_unset_host_as_ok() {
        // 未設定は終了コード非ゼロ・stderr でも正常系
        assert_eq!(interpret_get("", "HostName: not set\n"), (String::new(), true));
        assert_eq!(interpret_get("", ""), (String::new(), true));
        assert_eq!(
            interpret_get("my-mac\n", ""),
            ("my-mac".to_string(), false)
        );
        // not set 以外の stderr は異常系のまま
        assert_eq!(interpret_get("", "boom"), (String::new(), false));
    }

    #[test]
    fn runs_dns_snapshot() {
        let output = crate::privileged::execute_plain(
            "scutil",
            &["--dns"],
            "scutil --dns".to_string(),
        )
        .expect("scutil");
        assert!(output.success, "stderr: {}", output.stderr);
        let sections = parse_dns(&output.stdout);
        assert!(!sections.is_empty());
    }

    #[test]
    fn runs_nwi_snapshot() {
        let output = crate::privileged::execute_plain(
            "scutil",
            &["--nwi"],
            "scutil --nwi".to_string(),
        )
        .expect("scutil");
        assert!(output.success, "stderr: {}", output.stderr);
        let (sections, _) = parse_nwi(&output.stdout);
        assert!(!sections.is_empty());
    }

    #[test]
    fn runs_names_snapshot() {
        // HostName 未設定の実機でも success になること
        let result = super::run_names().expect("scutil names");
        assert!(result.success, "stderr: {}", result.stderr);
        assert_eq!(result.names.len(), 3);
        assert!(result.stderr.is_empty(), "stderr: {}", result.stderr);
    }
}
