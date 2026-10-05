//! `curl` の定義・実行・出力パース。管理者権限は不要。
//! 参照系に限定するため GET / HEAD のみを扱い、`-d`/`-F`/`-T` (送信)、`-o`/`-O` (ファイル書込)、
//! `-K` (設定ファイル)、`-u`/`--cookie` 等の認証・状態系は扱わない。
//! 無限待ちを避けるため `-m max-time` は必須 (1〜120 秒)。
//! 安全のため常に `-q` (~/.curlrc を読まない)、`-g` (URL グロブ無効)、
//! `--proto`/`--proto-redir` (http/https 限定)、`--max-filesize` を付ける。
//! 計測値は `-w '%{stderr}…%{json}'` で stderr 末尾に書かせて本文と分離する。

use serde::{Deserialize, Serialize};

use crate::privileged;

pub const DEFAULT_MAX_TIME: u32 = 30;
pub const MIN_MAX_TIME: u32 = 1;
pub const MAX_MAX_TIME: u32 = 120;
pub const MIN_CONNECT_TIMEOUT: u32 = 1;
pub const MAX_CONNECT_TIMEOUT: u32 = 60;
pub const DEFAULT_MAX_REDIRS: u32 = 10;
pub const MAX_MAX_REDIRS: u32 = 20;
pub const MAX_URL_LEN: usize = 2048;
pub const MAX_HEADERS: usize = 5;
pub const MAX_HEADER_LEN: usize = 1024;
/// これを超えるダウンロードは curl 側で打ち切る (exit 63)。
pub const MAX_FILESIZE: u64 = 20 * 1024 * 1024;
/// 画面に返す本文の上限バイト数。
pub const MAX_BODY_BYTES: usize = 64 * 1024;

const WRITE_OUT_MARKER: &str = "__MANMEN_CURL__";

/// Frontend の `getCurl` 引数とフィールドを一致させること。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurlQuery {
    pub url: String,
    #[serde(default)]
    pub head: bool,
    #[serde(default)]
    pub headers: Vec<String>,
    #[serde(default)]
    pub follow: bool,
    #[serde(default = "default_max_redirs")]
    pub max_redirs: u32,
    #[serde(default = "default_max_time")]
    pub max_time: u32,
    #[serde(default)]
    pub connect_timeout: Option<u32>,
    /// "" (既定) / "1.1" / "2"
    #[serde(default)]
    pub http_version: String,
    /// "" (自動) / "4" / "6"
    #[serde(default)]
    pub ip_version: String,
    #[serde(default)]
    pub insecure: bool,
    #[serde(default)]
    pub compressed: bool,
}

fn default_max_redirs() -> u32 {
    DEFAULT_MAX_REDIRS
}

fn default_max_time() -> u32 {
    DEFAULT_MAX_TIME
}

/// ヘッダ1行。Frontend の `CurlHeader` とフィールドを一致させること。
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct CurlHeader {
    pub name: String,
    pub value: String,
}

/// レスポンス1件 (リダイレクトを追うと複数)。Frontend の `CurlResponse` とフィールドを一致させること。
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct CurlResponse {
    pub status_line: String,
    pub http_version: String,
    pub status_code: Option<u32>,
    pub headers: Vec<CurlHeader>,
}

/// `-w '%{json}'` のうち使う項目。Frontend の `CurlInfo` とフィールドを一致させること。
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq)]
#[serde(default)]
pub struct CurlInfo {
    pub http_code: u32,
    pub http_version: String,
    pub method: Option<String>,
    pub scheme: Option<String>,
    pub remote_ip: Option<String>,
    pub remote_port: Option<u32>,
    pub url_effective: Option<String>,
    pub num_redirects: u32,
    pub content_type: Option<String>,
    pub size_download: u64,
    pub size_header: u64,
    pub speed_download: u64,
    pub ssl_verify_result: i64,
    pub time_namelookup: f64,
    pub time_connect: f64,
    pub time_appconnect: f64,
    pub time_pretransfer: f64,
    pub time_redirect: f64,
    pub time_starttransfer: f64,
    pub time_total: f64,
    pub errormsg: Option<String>,
}

/// `get_curl` の返却値。Frontend の `CurlSnapshot` とフィールドを一致させること。
#[derive(Debug, Clone, Serialize)]
pub struct CurlSnapshot {
    pub success: bool,
    pub exit_code: i32,
    pub command: String,
    pub responses: Vec<CurlResponse>,
    pub body: String,
    pub body_truncated: bool,
    pub body_binary: bool,
    pub info: Option<CurlInfo>,
    pub stderr: String,
}

pub struct Curl {
    /// 表示用 (ユーザーが選んだオプション + URL)
    shown: Vec<String>,
    /// 実際に渡す引数 (安全用の固定オプションを含む)
    args: Vec<String>,
}

impl Curl {
    pub fn new(query: CurlQuery) -> Result<Self, String> {
        let shown = build_shown_args(&query)?;
        let mut args: Vec<String> = [
            "-q",
            "-g",
            "--proto",
            "=http,https",
            "--proto-redir",
            "=http,https",
            "--max-filesize",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        args.push(MAX_FILESIZE.to_string());
        args.push("-w".to_string());
        args.push(format!("%{{stderr}}\n{WRITE_OUT_MARKER}%{{json}}"));
        args.extend(shown.iter().cloned());
        Ok(Self { shown, args })
    }

    /// 安全用の固定オプションと `-w` は表示から省く。
    pub fn preview(&self) -> String {
        format!("curl {}", self.shown.join(" "))
    }

    pub fn run(&self) -> Result<CurlSnapshot, String> {
        let preview = self.preview();
        let refs: Vec<&str> = self.args.iter().map(String::as_str).collect();
        let output = privileged::execute_plain("/usr/bin/curl", &refs, preview.clone())?;
        let (stderr, info) = split_write_out(&output.stderr);
        let (responses, body) = parse_stdout(&output.stdout);
        let body_binary = body.contains('\0') || body.contains('\u{FFFD}');
        let (body, body_truncated) = if body_binary {
            (String::new(), false)
        } else {
            truncate_body(body)
        };
        Ok(CurlSnapshot {
            success: output.success,
            exit_code: output.exit_code,
            command: preview,
            responses,
            body,
            body_truncated,
            body_binary,
            info,
            stderr,
        })
    }
}

pub fn get_snapshot(query: CurlQuery) -> Result<CurlSnapshot, String> {
    Curl::new(query).and_then(|cmd| cmd.run())
}

fn build_shown_args(query: &CurlQuery) -> Result<Vec<String>, String> {
    let url = query.url.trim();
    validate_url(url)?;
    if query.headers.len() > MAX_HEADERS {
        return Err(format!("ヘッダは {MAX_HEADERS} 件までです"));
    }
    for header in &query.headers {
        validate_header(header.trim())?;
    }
    if query.max_redirs > MAX_MAX_REDIRS {
        return Err(format!("リダイレクト上限は 0〜{MAX_MAX_REDIRS} です"));
    }
    if !(MIN_MAX_TIME..=MAX_MAX_TIME).contains(&query.max_time) {
        return Err(format!(
            "最大時間は {MIN_MAX_TIME}〜{MAX_MAX_TIME} 秒です"
        ));
    }
    if let Some(timeout) = query.connect_timeout {
        if !(MIN_CONNECT_TIMEOUT..=MAX_CONNECT_TIMEOUT).contains(&timeout) {
            return Err(format!(
                "接続タイムアウトは {MIN_CONNECT_TIMEOUT}〜{MAX_CONNECT_TIMEOUT} 秒です"
            ));
        }
    }
    let http_flag = match query.http_version.as_str() {
        "" => None,
        "1.1" => Some("--http1.1"),
        "2" => Some("--http2"),
        other => return Err(format!("HTTP バージョンが不正です: {other}")),
    };
    let ip_flag = match query.ip_version.as_str() {
        "" => None,
        "4" => Some("-4"),
        "6" => Some("-6"),
        other => return Err(format!("IP バージョンが不正です: {other}")),
    };

    let mut args: Vec<String> = vec!["-sS".to_string()];
    args.push(if query.head { "-I" } else { "-i" }.to_string());
    if query.follow {
        args.push("-L".to_string());
        args.push("--max-redirs".to_string());
        args.push(query.max_redirs.to_string());
    }
    args.push("-m".to_string());
    args.push(query.max_time.to_string());
    if let Some(timeout) = query.connect_timeout {
        args.push("--connect-timeout".to_string());
        args.push(timeout.to_string());
    }
    if let Some(flag) = http_flag {
        args.push(flag.to_string());
    }
    if let Some(flag) = ip_flag {
        args.push(flag.to_string());
    }
    if query.insecure {
        args.push("-k".to_string());
    }
    if query.compressed {
        args.push("--compressed".to_string());
    }
    for header in &query.headers {
        args.push("-H".to_string());
        args.push(header.trim().to_string());
    }
    args.push(url.to_string());
    Ok(args)
}

/// http/https のみ。空白・制御文字を含む値はオプション誤解釈や複数URL化を招くので拒否する。
fn validate_url(url: &str) -> Result<(), String> {
    if url.is_empty() {
        return Err("URL を入力してください".to_string());
    }
    if url.len() > MAX_URL_LEN {
        return Err("URL が長すぎます".to_string());
    }
    if url.chars().any(|c| c.is_whitespace() || c.is_control()) {
        return Err(format!("URL が不正です: {url}"));
    }
    let lower = url.to_ascii_lowercase();
    let rest = lower
        .strip_prefix("http://")
        .or_else(|| lower.strip_prefix("https://"))
        .ok_or_else(|| "URL は http:// か https:// で始めてください".to_string())?;
    let authority = rest.split(['/', '?', '#']).next().unwrap_or("");
    let host = authority.rsplit('@').next().unwrap_or("");
    if host.is_empty() || host.starts_with(':') {
        return Err(format!("URL のホストが空です: {url}"));
    }
    Ok(())
}

/// `Name: value` 形式のみ。`@file` 読み込みや改行による別ヘッダ注入を防ぐ。
fn validate_header(header: &str) -> Result<(), String> {
    if header.len() > MAX_HEADER_LEN {
        return Err("ヘッダが長すぎます".to_string());
    }
    let (name, value) = header
        .split_once(':')
        .ok_or_else(|| format!("ヘッダは「名前: 値」の形式です: {header}"))?;
    let name_ok = !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || "!#$%&'*+-.^_`|~".contains(c));
    if !name_ok {
        return Err(format!("ヘッダ名が不正です: {name}"));
    }
    if value.chars().any(|c| c.is_control() && c != '\t') {
        return Err(format!("ヘッダ値が不正です: {name}"));
    }
    Ok(())
}

/// stderr から `-w` の JSON を取り出し、残り (curl のエラー文) と分ける。
fn split_write_out(stderr: &str) -> (String, Option<CurlInfo>) {
    match stderr.rfind(WRITE_OUT_MARKER) {
        Some(pos) => {
            let json = stderr[pos + WRITE_OUT_MARKER.len()..].trim();
            let info = serde_json::from_str::<CurlInfo>(json).ok();
            (stderr[..pos].trim().to_string(), info)
        }
        None => (stderr.trim().to_string(), None),
    }
}

/// `-i`/`-I` の出力を先頭から `HTTP/` ブロック単位で切り出し、残りを本文とする。
fn parse_stdout(stdout: &str) -> (Vec<CurlResponse>, String) {
    let mut responses = Vec::new();
    let mut rest = stdout;
    while rest.starts_with("HTTP/") {
        let (block, next) = match (rest.find("\r\n\r\n"), rest.find("\n\n")) {
            (Some(crlf), Some(lf)) if lf < crlf => (&rest[..lf], &rest[lf + 2..]),
            (Some(crlf), _) => (&rest[..crlf], &rest[crlf + 4..]),
            (None, Some(lf)) => (&rest[..lf], &rest[lf + 2..]),
            (None, None) => (rest, ""),
        };
        responses.push(parse_header_block(block));
        rest = next;
    }
    (responses, rest.to_string())
}

/// `HTTP/2 200` + `name: value` 行の塊をパースする。
fn parse_header_block(block: &str) -> CurlResponse {
    let mut lines = block.lines().map(|line| line.trim_end_matches('\r'));
    let status_line = lines.next().unwrap_or("").trim().to_string();
    let mut parts = status_line.split_whitespace();
    let http_version = parts
        .next()
        .unwrap_or("")
        .trim_start_matches("HTTP/")
        .to_string();
    let status_code = parts.next().and_then(|code| code.parse::<u32>().ok());
    let headers = lines
        .filter_map(|line| {
            let (name, value) = line.split_once(':')?;
            Some(CurlHeader {
                name: name.trim().to_string(),
                value: value.trim().to_string(),
            })
        })
        .collect();
    CurlResponse {
        status_line,
        http_version,
        status_code,
        headers,
    }
}

fn truncate_body(body: String) -> (String, bool) {
    if body.len() <= MAX_BODY_BYTES {
        return (body, false);
    }
    let mut end = MAX_BODY_BYTES;
    while !body.is_char_boundary(end) {
        end -= 1;
    }
    (body[..end].to_string(), true)
}

#[cfg(test)]
mod tests {
    use super::{Curl, CurlQuery};

    const REDIRECT_THEN_OK: &str = "HTTP/1.1 301 Moved Permanently\r\n\
Content-Length: 0\r\n\
Location: https://example.com/\r\n\
\r\n\
HTTP/2 200 \r\n\
content-type: text/html; charset=utf-8\r\n\
etag: \"abc\"\r\n\
\r\n\
<html>hello</html>";

    const STDERR_WITH_JSON: &str = "curl: (6) Could not resolve host: nope.invalid\n\
\n__MANMEN_CURL__{\"http_code\":0,\"http_version\":\"0\",\"exitcode\":6,\
\"errormsg\":\"Could not resolve host: nope.invalid\",\"time_total\":0.01,\
\"remote_ip\":\"\",\"num_redirects\":0}";

    fn base_query() -> CurlQuery {
        CurlQuery {
            url: "https://example.com".to_string(),
            head: false,
            headers: Vec::new(),
            follow: false,
            max_redirs: 10,
            max_time: 30,
            connect_timeout: None,
            http_version: String::new(),
            ip_version: String::new(),
            insecure: false,
            compressed: false,
        }
    }

    #[test]
    fn previews_minimal_query() {
        let curl = Curl::new(base_query()).unwrap();
        assert_eq!(curl.preview(), "curl -sS -i -m 30 https://example.com");
    }

    #[test]
    fn previews_full_options() {
        let mut query = base_query();
        query.head = true;
        query.follow = true;
        query.max_redirs = 5;
        query.max_time = 10;
        query.connect_timeout = Some(3);
        query.http_version = "2".to_string();
        query.ip_version = "4".to_string();
        query.insecure = true;
        query.compressed = true;
        query.headers = vec!["Accept: application/json".to_string()];
        let curl = Curl::new(query).unwrap();
        assert_eq!(
            curl.preview(),
            "curl -sS -I -L --max-redirs 5 -m 10 --connect-timeout 3 --http2 -4 -k --compressed -H Accept: application/json https://example.com"
        );
    }

    #[test]
    fn always_adds_safety_flags_first() {
        let curl = Curl::new(base_query()).unwrap();
        assert_eq!(curl.args[0], "-q");
        assert!(curl.args.contains(&"-g".to_string()));
        assert!(curl.args.contains(&"=http,https".to_string()));
        assert_eq!(curl.args.last().unwrap(), "https://example.com");
    }

    #[test]
    fn rejects_unsafe_input() {
        for url in [
            "",
            "file:///etc/passwd",
            "ftp://example.com",
            "-o/tmp/x",
            "https://example.com -o /tmp/x",
            "https://",
            "https://exa\nmple.com",
        ] {
            let mut query = base_query();
            query.url = url.to_string();
            assert!(Curl::new(query).is_err(), "url should be rejected: {url:?}");
        }

        for header in ["@/etc/passwd", "NoColon", "Bad Name: x", "X-A: a\r\nX-B: b"] {
            let mut query = base_query();
            query.headers = vec![header.to_string()];
            assert!(Curl::new(query).is_err(), "header should be rejected: {header:?}");
        }

        let mut query = base_query();
        query.headers = vec!["A: b".to_string(); 6];
        assert!(Curl::new(query).is_err());

        let mut query = base_query();
        query.max_time = 0;
        assert!(Curl::new(query).is_err());

        let mut query = base_query();
        query.max_redirs = 21;
        assert!(Curl::new(query).is_err());

        let mut query = base_query();
        query.http_version = "3".to_string();
        assert!(Curl::new(query).is_err());
    }

    #[test]
    fn accepts_ipv6_and_userinfo_urls() {
        for url in ["http://[::1]:8080/path", "HTTPS://user@example.com/?q=1"] {
            let mut query = base_query();
            query.url = url.to_string();
            assert!(Curl::new(query).is_ok(), "url should be accepted: {url:?}");
        }
    }

    #[test]
    fn parses_redirect_chain_and_body() {
        let (responses, body) = super::parse_stdout(REDIRECT_THEN_OK);
        assert_eq!(responses.len(), 2);
        assert_eq!(responses[0].status_code, Some(301));
        assert_eq!(responses[0].http_version, "1.1");
        assert_eq!(responses[0].headers[1].name, "Location");
        assert_eq!(responses[0].headers[1].value, "https://example.com/");
        assert_eq!(responses[1].status_code, Some(200));
        assert_eq!(responses[1].http_version, "2");
        assert_eq!(responses[1].headers.len(), 2);
        assert_eq!(body, "<html>hello</html>");
    }

    #[test]
    fn parses_head_only_output() {
        let (responses, body) = super::parse_stdout("HTTP/2 204 \r\nserver: x\r\n\r\n");
        assert_eq!(responses.len(), 1);
        assert_eq!(responses[0].status_code, Some(204));
        assert_eq!(body, "");
    }

    #[test]
    fn splits_write_out_from_errors() {
        let (stderr, info) = super::split_write_out(STDERR_WITH_JSON);
        assert_eq!(stderr, "curl: (6) Could not resolve host: nope.invalid");
        let info = info.expect("json");
        assert_eq!(info.http_code, 0);
        assert_eq!(
            info.errormsg.as_deref(),
            Some("Could not resolve host: nope.invalid")
        );
        assert_eq!(info.time_total, 0.01);
    }

    #[test]
    fn truncates_long_body_on_char_boundary() {
        let body = "あ".repeat(super::MAX_BODY_BYTES);
        let (out, truncated) = super::truncate_body(body);
        assert!(truncated);
        assert!(out.len() <= super::MAX_BODY_BYTES);
    }

    #[test]
    fn reports_connection_refused() {
        let mut query = base_query();
        query.url = "http://127.0.0.1:1/".to_string();
        query.max_time = 5;
        let result = super::get_snapshot(query).expect("curl");
        assert!(!result.success);
        assert_eq!(result.exit_code, 7);
        assert!(result.responses.is_empty());
        assert!(result.info.is_some(), "stderr: {}", result.stderr);
    }
}
