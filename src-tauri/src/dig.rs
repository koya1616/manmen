//! `dig` の定義・実行・出力パース。管理者権限は不要。
//! DNS問い合わせをGUIから安全に行うため、名前・タイプ・サーバ・主要オプションだけを許可する。
//! 対話モードやバッチファイル (`-f`)、TSIG (`-y`/`-k`)、バインドアドレス (`-b`) は扱わない。

use serde::{Deserialize, Serialize};

use crate::privileged;

/// 許可する問い合わせタイプ。`man dig` の q-type のうち日常的なものに限る。
pub const ALLOWED_TYPES: &[&str] = &[
    "A", "AAAA", "CNAME", "MX", "NS", "SOA", "TXT", "SRV", "CAA", "PTR",
];
/// 許可する問い合わせクラス。
pub const ALLOWED_CLASSES: &[&str] = &["IN", "CH", "HS"];

pub const DEFAULT_TYPE: &str = "A";
pub const DEFAULT_CLASS: &str = "IN";
const MAX_NAME_LEN: usize = 253;
const MAX_SERVER_LEN: usize = 253;

/// Frontend の `getDig` 引数とフィールドを一致させること。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DigQuery {
    pub name: String,
    #[serde(default = "default_qtype")]
    pub qtype: String,
    #[serde(default = "default_qclass")]
    pub qclass: String,
    #[serde(default)]
    pub server: String,
    #[serde(default)]
    pub short: bool,
    #[serde(default)]
    pub tcp: bool,
    #[serde(default)]
    pub dnssec: bool,
    #[serde(default)]
    pub trace: bool,
    #[serde(default)]
    pub no_recurse: bool,
    #[serde(default)]
    pub reverse: bool,
    #[serde(default)]
    pub transport: String,
}

fn default_qtype() -> String {
    DEFAULT_TYPE.to_string()
}

fn default_qclass() -> String {
    DEFAULT_CLASS.to_string()
}

/// ANSWER SECTION の1行。Frontend の `DigRecord` とフィールドを一致させること。
#[derive(Debug, Clone, Default, Serialize, PartialEq)]
pub struct DigRecord {
    pub name: String,
    pub ttl: Option<u32>,
    pub class: String,
    pub dtype: String,
    pub value: String,
}

/// `get_dig` の返却値。Frontend の `DigSnapshot` とフィールドを一致させること。
#[derive(Debug, Clone, Serialize)]
pub struct DigSnapshot {
    pub success: bool,
    pub exit_code: i32,
    pub command: String,
    pub answers: Vec<DigRecord>,
    pub query_time: String,
    pub server: String,
    pub when: String,
    pub msg_size: String,
    pub raw: String,
    pub stderr: String,
}

/// `dig [@server] name [type] [class] [queryopt...]`
pub struct Dig {
    args: Vec<String>,
    query_name: String,
    qtype: String,
    short: bool,
}

impl Dig {
    pub fn new(query: DigQuery) -> Result<Self, String> {
        let args = build_args(&query)?;
        Ok(Self {
            args,
            query_name: query.name.trim().to_string(),
            qtype: query.qtype.trim().to_uppercase(),
            short: query.short,
        })
    }

    pub fn preview(&self) -> String {
        format!("dig {}", self.args.join(" "))
    }

    pub fn run(&self) -> Result<DigSnapshot, String> {
        let preview = self.preview();
        let refs: Vec<&str> = self.args.iter().map(String::as_str).collect();
        let output = privileged::execute_plain("/usr/bin/dig", &refs, preview.clone())?;
        let parsed = parse_output(&output.stdout, &self.query_name, &self.qtype, self.short);
        Ok(DigSnapshot {
            success: output.success,
            exit_code: output.exit_code,
            command: preview,
            answers: parsed.answers,
            query_time: parsed.query_time,
            server: parsed.server,
            when: parsed.when,
            msg_size: parsed.msg_size,
            raw: output.stdout.trim_end().to_string(),
            stderr: output.stderr.trim().to_string(),
        })
    }
}

pub fn get_snapshot(query: DigQuery) -> Result<DigSnapshot, String> {
    Dig::new(query).and_then(|cmd| cmd.run())
}

fn build_args(query: &DigQuery) -> Result<Vec<String>, String> {
    let name = query.name.trim();
    let qtype = query.qtype.trim().to_uppercase();
    let qclass = query.qclass.trim().to_uppercase();
    let server = query.server.trim();
    validate_transport(&query.transport)?;
    if query.reverse {
        validate_ip(name)?;
    } else {
        validate_name(name)?;
    }
    validate_type(&qtype)?;
    validate_class(&qclass)?;
    if !server.is_empty() {
        validate_server(server)?;
    }
    if query.trace && !server.is_empty() {
        return Err("+trace はサーバ指定と同時に使えません".to_string());
    }

    let mut args: Vec<String> = Vec::new();
    // トランスポート (-4 / -6) は先頭に置く
    if query.transport == "4" {
        args.push("-4".to_string());
    } else if query.transport == "6" {
        args.push("-6".to_string());
    }
    // サーバは @server 形式で名前より前に置く
    if !server.is_empty() {
        let bare = server.strip_prefix('@').unwrap_or(server);
        args.push(format!("@{bare}"));
    }
    if query.reverse {
        args.push("-x".to_string());
        args.push(name.to_string());
    } else {
        args.push(name.to_string());
        args.push(qtype);
        if qclass != DEFAULT_CLASS {
            args.push(qclass);
        }
    }
    if query.short {
        args.push("+short".to_string());
    }
    if query.tcp {
        args.push("+tcp".to_string());
    }
    if query.dnssec {
        args.push("+dnssec".to_string());
    }
    if query.trace {
        args.push("+trace".to_string());
    }
    if query.no_recurse {
        args.push("+norecurse".to_string());
    }
    if args.is_empty() {
        return Err("問い合わせ内容が空です".to_string());
    }
    Ok(args)
}

fn validate_transport(transport: &str) -> Result<(), String> {
    if matches!(transport, "" | "auto" | "4" | "6") {
        Ok(())
    } else {
        Err(format!("トランスポートが不正です: {transport}"))
    }
}

/// dig のオプション解釈を防ぐため、名前をホスト名相当に制限する。
/// 先頭 `-` / `+` / `@` 禁止、英数字と `._-` のみ、末尾ドット1つまで許可。
fn validate_name(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("名前を入力してください".to_string());
    }
    if name.len() > MAX_NAME_LEN {
        return Err("名前が長すぎます".to_string());
    }
    if name.starts_with('-') || name.starts_with('+') || name.starts_with('@') {
        return Err(format!("名前が不正です: {name}"));
    }
    let stripped = name.strip_suffix('.').unwrap_or(name);
    if stripped.is_empty() {
        return Err(format!("名前が不正です: {name}"));
    }
    let ok = stripped
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-'));
    if !ok {
        return Err(format!("名前が不正です: {name}"));
    }
    if stripped.contains("..") {
        return Err(format!("名前が不正です: {name}"));
    }
    Ok(())
}

fn validate_server(server: &str) -> Result<(), String> {
    let bare = server.strip_prefix('@').unwrap_or(server);
    if bare.is_empty() || bare.len() > MAX_SERVER_LEN {
        return Err(format!("サーバが不正です: {server}"));
    }
    if bare.starts_with('-') || bare.starts_with('+') {
        return Err(format!("サーバが不正です: {server}"));
    }
    // IP アドレスかホスト名のどちらか
    if is_ip(bare) {
        return Ok(());
    }
    validate_name(bare).map_err(|_| format!("サーバが不正です: {server}"))
}

fn validate_type(qtype: &str) -> Result<(), String> {
    if ALLOWED_TYPES.contains(&qtype) {
        Ok(())
    } else {
        Err(format!("タイプが不正です: {qtype}"))
    }
}

fn validate_class(qclass: &str) -> Result<(), String> {
    if ALLOWED_CLASSES.contains(&qclass) {
        Ok(())
    } else {
        Err(format!("クラスが不正です: {qclass}"))
    }
}

fn validate_ip(ip: &str) -> Result<(), String> {
    if ip.is_empty() {
        return Err("IPアドレスを入力してください".to_string());
    }
    if is_ip(ip) {
        Ok(())
    } else {
        Err(format!("IPアドレスが不正です: {ip}"))
    }
}

fn is_ip(value: &str) -> bool {
    value.parse::<std::net::IpAddr>().is_ok()
}

struct ParsedOutput {
    answers: Vec<DigRecord>,
    query_time: String,
    server: String,
    when: String,
    msg_size: String,
}

/// stdout 全体を ANSWER と統計情報に分けてパースする。
/// `+short` のときは各行がそのまま答えになる。
fn parse_output(stdout: &str, query_name: &str, qtype: &str, short: bool) -> ParsedOutput {
    let mut query_time = String::new();
    let mut server = String::new();
    let mut when = String::new();
    let mut msg_size = String::new();
    for line in stdout.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix(";; Query time:") {
            query_time = rest.trim().to_string();
        } else if let Some(rest) = trimmed.strip_prefix(";; SERVER:") {
            server = rest.trim().to_string();
        } else if let Some(rest) = trimmed.strip_prefix(";; WHEN:") {
            when = rest.trim().to_string();
        } else if let Some(rest) = trimmed.strip_prefix(";; MSG SIZE") {
            let rest = rest.trim().strip_prefix("rcvd:").unwrap_or(rest.trim());
            msg_size = rest.trim().to_string();
        }
    }

    let answers = if short {
        parse_short(stdout, query_name, qtype)
    } else {
        parse_answer_section(stdout)
    };
    ParsedOutput {
        answers,
        query_time,
        server,
        when,
        msg_size,
    }
}

fn parse_short(stdout: &str, query_name: &str, qtype: &str) -> Vec<DigRecord> {
    stdout
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with(';'))
        .map(|line| DigRecord {
            name: query_name.to_string(),
            ttl: None,
            class: String::new(),
            dtype: qtype.to_string(),
            value: line.to_string(),
        })
        .collect()
}

/// `;; ANSWER SECTION:` 以降の表をパースする。次の `;;` セクションで打ち切る。
fn parse_answer_section(stdout: &str) -> Vec<DigRecord> {
    let mut in_answer = false;
    let mut out = Vec::new();
    for line in stdout.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with(";; ANSWER SECTION:") {
            in_answer = true;
            continue;
        }
        if in_answer && trimmed.starts_with(";;") {
            break;
        }
        if !in_answer || trimmed.is_empty() || trimmed.starts_with(';') {
            continue;
        }
        if let Some(record) = parse_record_line(trimmed) {
            out.push(record);
        }
    }
    out
}

/// `name [TTL] class type RDATA` の1行をパースする。TTL省略形にも対応する。
fn parse_record_line(line: &str) -> Option<DigRecord> {
    let mut parts = line.split_whitespace();
    let name = parts.next()?.to_string();
    let second = parts.next()?;
    // TTL があるかないかでずらす
    let (ttl, class, dtype, value_start) = if second.chars().all(|c| c.is_ascii_digit()) {
        let ttl: u32 = second.parse().ok()?;
        let class = parts.next()?.to_string();
        let dtype = parts.next()?.to_string();
        (Some(ttl), class, dtype, 4usize)
    } else {
        (None, second.to_string(), parts.next()?.to_string(), 3usize)
    };
    let tokens: Vec<&str> = line.split_whitespace().collect();
    if tokens.len() <= value_start {
        return None;
    }
    let value = tokens[value_start..].join(" ");
    Some(DigRecord {
        name,
        ttl,
        class,
        dtype,
        value,
    })
}

#[cfg(test)]
mod tests {
    use super::{parse_answer_section, parse_output, parse_short, Dig, DigQuery};

    const FULL: &str = "; <<>> DiG 9.10.6 <<>> example.com A\n\
        ;; global options: +cmd\n\
        ;; Got answer:\n\
        ;; ->>HEADER<<- opcode: QUERY, status: NOERROR, id: 1\n\
        \n\
        ;; QUESTION SECTION:\n\
        ;example.com.\t\t\tIN\tA\n\
        \n\
        ;; ANSWER SECTION:\n\
        example.com.\t\t30\tIN\tA\t93.184.215.14\n\
        example.com.\t\t30\tIN\tA\t93.184.216.34\n\
        \n\
        ;; Query time: 12 msec\n\
        ;; SERVER: 192.168.1.1#53(192.168.1.1)\n\
        ;; WHEN: Fri Oct 02 10:00:00 JST 2026\n\
        ;; MSG SIZE  rcvd: 61\n";

    fn base_query() -> DigQuery {
        DigQuery {
            name: "example.com".to_string(),
            qtype: "A".to_string(),
            qclass: "IN".to_string(),
            server: String::new(),
            short: false,
            tcp: false,
            dnssec: false,
            trace: false,
            no_recurse: false,
            reverse: false,
            transport: "auto".to_string(),
        }
    }

    #[test]
    fn previews_basic_query() {
        let dig = Dig::new(base_query()).unwrap();
        assert_eq!(dig.preview(), "dig example.com A");
    }

    #[test]
    fn previews_full_options() {
        let mut query = base_query();
        query.name = "example.com.".to_string();
        query.qtype = "mx".to_string();
        query.qclass = "IN".to_string();
        query.server = "8.8.8.8".to_string();
        query.short = true;
        query.tcp = true;
        query.dnssec = true;
        query.no_recurse = true;
        query.transport = "4".to_string();
        let dig = Dig::new(query).unwrap();
        assert_eq!(
            dig.preview(),
            "dig -4 @8.8.8.8 example.com. MX +short +tcp +dnssec +norecurse"
        );
    }

    #[test]
    fn previews_reverse_lookup() {
        let mut query = base_query();
        query.name = "8.8.8.8".to_string();
        query.reverse = true;
        let dig = Dig::new(query).unwrap();
        assert_eq!(dig.preview(), "dig -x 8.8.8.8");
    }

    #[test]
    fn rejects_option_like_input() {
        let mut query = base_query();
        query.name = "-f /etc/passwd".to_string();
        assert!(Dig::new(query).is_err());

        let mut query = base_query();
        query.name = "example.com; rm -rf /".to_string();
        assert!(Dig::new(query).is_err());

        let mut query = base_query();
        query.name = String::new();
        assert!(Dig::new(query).is_err());

        let mut query = base_query();
        query.qtype = "ANY;rm".to_string();
        assert!(Dig::new(query).is_err());

        let mut query = base_query();
        query.server = "-f".to_string();
        assert!(Dig::new(query).is_err());

        let mut query = base_query();
        query.server = "8.8.8.8".to_string();
        query.trace = true;
        assert!(Dig::new(query).is_err());

        let mut query = base_query();
        query.transport = "x".to_string();
        assert!(Dig::new(query).is_err());

        let mut query = base_query();
        query.name = "not-an-ip".to_string();
        query.reverse = true;
        assert!(Dig::new(query).is_err());
    }

    #[test]
    fn parses_answer_and_stats() {
        let parsed = parse_output(FULL, "example.com", "A", false);
        assert_eq!(parsed.answers.len(), 2);
        assert_eq!(parsed.answers[0].name, "example.com.");
        assert_eq!(parsed.answers[0].ttl, Some(30));
        assert_eq!(parsed.answers[0].class, "IN");
        assert_eq!(parsed.answers[0].dtype, "A");
        assert_eq!(parsed.answers[0].value, "93.184.215.14");
        assert_eq!(parsed.query_time, "12 msec");
        assert!(parsed.server.contains("192.168.1.1"));
        assert!(parsed.when.contains("2026"));
        assert_eq!(parsed.msg_size, "61");
    }

    #[test]
    fn parses_short_output() {
        let short = "93.184.215.14\n93.184.216.34\n";
        let answers = parse_short(short, "example.com", "A");
        assert_eq!(answers.len(), 2);
        assert_eq!(answers[0].value, "93.184.215.14");
        assert_eq!(answers[0].ttl, None);
    }

    #[test]
    fn tolerates_empty_answer() {
        let answers = parse_answer_section(";; QUESTION SECTION:\n;example.com. IN A\n");
        assert!(answers.is_empty());
    }
}
