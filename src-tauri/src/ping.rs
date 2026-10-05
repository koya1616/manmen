//! `ping` の定義・実行・出力パース。管理者権限は不要。
//! 無限実行を避けるため `-c count` は必須 (1〜20)。flood (`-f`)、preload (`-l`)、
//! sweep (`-G`/`-g`/`-h`)、バイパス (`-r`)、送信元指定 (`-S`)、パターン (`-p`)、
//! IPsec (`-P`)、マルチキャスト専用 (`-I`/`-T`/`-L`)、Apple専用フラグは扱わない。

use serde::{Deserialize, Serialize};

use crate::privileged;

pub const DEFAULT_COUNT: u32 = 4;
pub const MAX_COUNT: u32 = 20;
pub const DEFAULT_INTERVAL: f64 = 1.0;
pub const MIN_INTERVAL: f64 = 0.2;
pub const MAX_INTERVAL: f64 = 10.0;
pub const DEFAULT_SIZE: u32 = 56;
pub const MAX_SIZE: u32 = 1472;
pub const MAX_HOST_LEN: usize = 253;
pub const MIN_WAIT_MS: u32 = 100;
pub const MAX_WAIT_MS: u32 = 10_000;
pub const MIN_TIMEOUT: u32 = 1;
pub const MAX_TIMEOUT: u32 = 120;
pub const MIN_TTL: u32 = 1;
pub const MAX_TTL: u32 = 255;

/// Frontend の `getPing` 引数とフィールドを一致させること。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PingQuery {
    pub host: String,
    #[serde(default = "default_count")]
    pub count: u32,
    #[serde(default = "default_interval")]
    pub interval: f64,
    #[serde(default)]
    pub timeout: Option<u32>,
    #[serde(default)]
    pub wait_ms: Option<u32>,
    #[serde(default = "default_size")]
    pub size: u32,
    #[serde(default)]
    pub ttl: Option<u32>,
    #[serde(default)]
    pub numeric: bool,
    #[serde(default)]
    pub no_fragment: bool,
}

fn default_count() -> u32 {
    DEFAULT_COUNT
}

fn default_interval() -> f64 {
    DEFAULT_INTERVAL
}

fn default_size() -> u32 {
    DEFAULT_SIZE
}

/// 応答1件。Frontend の `PingReply` とフィールドを一致させること。
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct PingReply {
    pub seq: u32,
    pub bytes: Option<u32>,
    pub from: String,
    pub ttl: Option<u32>,
    pub time_ms: Option<f64>,
    pub timeout: bool,
}

/// 集計。Frontend の `PingStats` とフィールドを一致させること。
#[derive(Debug, Clone, Default, Serialize, PartialEq)]
pub struct PingStats {
    pub transmitted: u32,
    pub received: u32,
    pub loss_percent: f64,
    pub min_ms: Option<f64>,
    pub avg_ms: Option<f64>,
    pub max_ms: Option<f64>,
    pub stddev_ms: Option<f64>,
}

/// `get_ping` の返却値。Frontend の `PingSnapshot` とフィールドを一致させること。
#[derive(Debug, Clone, Serialize)]
pub struct PingSnapshot {
    pub success: bool,
    pub exit_code: i32,
    pub command: String,
    pub target: String,
    pub resolved_ip: String,
    pub replies: Vec<PingReply>,
    pub timeout_count: usize,
    pub stats: PingStats,
    pub raw: String,
    pub stderr: String,
}

pub struct Ping {
    args: Vec<String>,
}

impl Ping {
    pub fn new(query: PingQuery) -> Result<Self, String> {
        let args = build_args(&query)?;
        Ok(Self { args })
    }

    pub fn preview(&self) -> String {
        format!("ping {}", self.args.join(" "))
    }

    pub fn run(&self) -> Result<PingSnapshot, String> {
        let preview = self.preview();
        let refs: Vec<&str> = self.args.iter().map(String::as_str).collect();
        let output = privileged::execute_plain("/sbin/ping", &refs, preview.clone())?;
        let parsed = parse_output(&output.stdout);
        Ok(PingSnapshot {
            success: output.success,
            exit_code: output.exit_code,
            command: preview,
            target: parsed.target,
            resolved_ip: parsed.resolved_ip,
            timeout_count: parsed
                .replies
                .iter()
                .filter(|reply| reply.timeout)
                .count(),
            replies: parsed.replies,
            stats: parsed.stats,
            raw: output.stdout.trim_end().to_string(),
            stderr: output.stderr.trim().to_string(),
        })
    }
}

pub fn get_snapshot(query: PingQuery) -> Result<PingSnapshot, String> {
    Ping::new(query).and_then(|cmd| cmd.run())
}

fn build_args(query: &PingQuery) -> Result<Vec<String>, String> {
    let host = query.host.trim();
    validate_host(host)?;
    if query.count == 0 || query.count > MAX_COUNT {
        return Err(format!("回数は 1〜{MAX_COUNT} です"));
    }
    if !(MIN_INTERVAL..=MAX_INTERVAL).contains(&query.interval) {
        return Err("間隔は 0.2〜10 秒です".to_string());
    }
    if let Some(timeout) = query.timeout {
        if !(MIN_TIMEOUT..=MAX_TIMEOUT).contains(&timeout) {
            return Err(format!("タイムアウトは {MIN_TIMEOUT}〜{MAX_TIMEOUT} 秒です"));
        }
    }
    if let Some(wait_ms) = query.wait_ms {
        if !(MIN_WAIT_MS..=MAX_WAIT_MS).contains(&wait_ms) {
            return Err(format!(
                "応答待ちは {MIN_WAIT_MS}〜{MAX_WAIT_MS} ミリ秒です"
            ));
        }
    }
    if query.size > MAX_SIZE {
        return Err(format!("サイズは 0〜{MAX_SIZE} です"));
    }
    if let Some(ttl) = query.ttl {
        if !(MIN_TTL..=MAX_TTL).contains(&ttl) {
            return Err(format!("TTL は {MIN_TTL}〜{MAX_TTL} です"));
        }
    }

    let mut args: Vec<String> = Vec::new();
    args.push("-c".to_string());
    args.push(query.count.to_string());
    if (query.interval - DEFAULT_INTERVAL).abs() > f64::EPSILON {
        args.push("-i".to_string());
        args.push(format_interval(query.interval));
    }
    if let Some(timeout) = query.timeout {
        args.push("-t".to_string());
        args.push(timeout.to_string());
    }
    if let Some(wait_ms) = query.wait_ms {
        args.push("-W".to_string());
        args.push(wait_ms.to_string());
    }
    if query.size != DEFAULT_SIZE {
        args.push("-s".to_string());
        args.push(query.size.to_string());
    }
    if let Some(ttl) = query.ttl {
        args.push("-m".to_string());
        args.push(ttl.to_string());
    }
    if query.numeric {
        args.push("-n".to_string());
    }
    if query.no_fragment {
        args.push("-D".to_string());
    }
    args.push(host.to_string());
    Ok(args)
}

fn format_interval(interval: f64) -> String {
    if (interval - interval.round()).abs() < f64::EPSILON {
        format!("{}", interval.round() as u32)
    } else {
        format!("{interval}")
    }
}

/// dig と同様、オプション解釈を防ぐためホスト名相当かIPに制限する。
pub fn validate_host(host: &str) -> Result<(), String> {
    if host.is_empty() {
        return Err("ホストを入力してください".to_string());
    }
    if host.len() > MAX_HOST_LEN {
        return Err("ホストが長すぎます".to_string());
    }
    if host.starts_with('-') || host.starts_with('+') || host.starts_with('@') {
        return Err(format!("ホストが不正です: {host}"));
    }
    if host.contains(char::is_whitespace) || host.contains(';') || host.contains('&')
        || host.contains('|') || host.contains('`') || host.contains('$')
        || host.contains('(') || host.contains(')') || host.contains('<')
        || host.contains('>') || host.contains('\'') || host.contains('"')
        || host.contains('\\')
    {
        return Err(format!("ホストが不正です: {host}"));
    }
    // ゾーン付き IPv6 (`fe80::1%en0`) は `%` 以降を外して判定する
    let bare = host.split('%').next().unwrap_or(host);
    if bare.parse::<std::net::IpAddr>().is_ok() {
        return Ok(());
    }
    let stripped = bare.strip_suffix('.').unwrap_or(bare);
    if stripped.is_empty() || stripped.contains("..") {
        return Err(format!("ホストが不正です: {host}"));
    }
    let ok = stripped
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-' | ':'));
    if !ok {
        return Err(format!("ホストが不正です: {host}"));
    }
    Ok(())
}

struct ParsedOutput {
    target: String,
    resolved_ip: String,
    replies: Vec<PingReply>,
    stats: PingStats,
}

fn parse_output(stdout: &str) -> ParsedOutput {
    let mut lines = stdout.lines();
    let first = lines.next().unwrap_or("").trim();
    let (target, resolved_ip) = parse_header(first);
    let mut replies = Vec::new();
    let mut stats = PingStats::default();
    for line in stdout.lines() {
        let trimmed = line.trim();
        if let Some(reply) = parse_reply_line(trimmed) {
            replies.push(reply);
            continue;
        }
        if let Some(parsed) = parse_stats_line(trimmed) {
            stats.transmitted = parsed.0;
            stats.received = parsed.1;
            stats.loss_percent = parsed.2;
        }
        if let Some(rtt) = parse_rtt_line(trimmed) {
            stats.min_ms = Some(rtt.0);
            stats.avg_ms = Some(rtt.1);
            stats.max_ms = Some(rtt.2);
            stats.stddev_ms = Some(rtt.3);
        }
    }
    ParsedOutput {
        target,
        resolved_ip,
        replies,
        stats,
    }
}

/// `PING example.com (93.184.215.14): 56 data bytes` から表示名と解決先を抜く。
fn parse_header(first: &str) -> (String, String) {
    let rest = first.strip_prefix("PING ").unwrap_or(first).trim();
    let target = rest
        .split_whitespace()
        .next()
        .unwrap_or("")
        .trim_end_matches(':')
        .to_string();
    let resolved_ip = rest
        .find('(')
        .and_then(|start| rest[start + 1..].find(')').map(|end| rest[start + 1..start + 1 + end].to_string()))
        .unwrap_or_default();
    (target, resolved_ip)
}

/// `64 bytes from 127.0.0.1: icmp_seq=0 ttl=64 time=0.085 ms`
/// または `Request timeout for icmp_seq 0` をパースする。
fn parse_reply_line(line: &str) -> Option<PingReply> {
    if let Some(rest) = line.strip_prefix("Request timeout for icmp_seq ") {
        let seq = rest.split_whitespace().next()?.parse::<u32>().ok()?;
        return Some(PingReply {
            seq,
            bytes: None,
            from: String::new(),
            ttl: None,
            time_ms: None,
            timeout: true,
        });
    }
    let (left, right) = line.split_once(':')?;
    if !right.contains("icmp_seq=") {
        return None;
    }
    let mut left_parts = left.split_whitespace();
    let bytes = left_parts.next()?.parse::<u32>().ok()?;
    if left_parts.next() != Some("bytes") || left_parts.next() != Some("from") {
        return None;
    }
    let from = left_parts.next()?.to_string();
    let mut seq = None;
    let mut ttl = None;
    let mut time_ms = None;
    for token in right.split_whitespace() {
        if let Some(value) = token.strip_prefix("icmp_seq=") {
            seq = value.parse::<u32>().ok();
        } else if let Some(value) = token.strip_prefix("ttl=") {
            ttl = value.parse::<u32>().ok();
        } else if let Some(value) = token.strip_prefix("time=") {
            time_ms = value.parse::<f64>().ok();
        }
    }
    Some(PingReply {
        seq: seq?,
        bytes: Some(bytes),
        from,
        ttl,
        time_ms,
        timeout: false,
    })
}

/// `2 packets transmitted, 2 packets received, 0.0% packet loss` をパースする。
fn parse_stats_line(line: &str) -> Option<(u32, u32, f64)> {
    let (sent_part, rest) = line.split_once("packets transmitted,")?;
    let transmitted = sent_part.trim().split_whitespace().last()?.parse::<u32>().ok()?;
    let (recv_part, loss_part) = rest.split_once("packets received,")?;
    let received = recv_part.trim().split_whitespace().last()?.parse::<u32>().ok()?;
    let loss = loss_part
        .trim()
        .split_whitespace()
        .next()?
        .trim_end_matches('%')
        .parse::<f64>()
        .ok()?;
    Some((transmitted, received, loss))
}

/// `round-trip min/avg/max/stddev = 0.085/0.098/0.111/0.013 ms` をパースする。
fn parse_rtt_line(line: &str) -> Option<(f64, f64, f64, f64)> {
    let rest = line.split('=').nth(1)?.trim();
    let values: Vec<&str> = rest.trim_end_matches(" ms").trim().split('/').collect();
    if values.len() != 4 {
        return None;
    }
    Some((
        values[0].parse::<f64>().ok()?,
        values[1].parse::<f64>().ok()?,
        values[2].parse::<f64>().ok()?,
        values[3].parse::<f64>().ok()?,
    ))
}

#[cfg(test)]
mod tests {
    use super::{Ping, PingQuery};

    const SUCCESS: &str = "\
PING 127.0.0.1 (127.0.0.1): 56 data bytes
64 bytes from 127.0.0.1: icmp_seq=0 ttl=64 time=0.085 ms
64 bytes from 127.0.0.1: icmp_seq=1 ttl=64 time=0.111 ms

--- 127.0.0.1 ping statistics ---
2 packets transmitted, 2 packets received, 0.0% packet loss
round-trip min/avg/max/stddev = 0.085/0.098/0.111/0.013 ms
";

    const WITH_TIMEOUT: &str = "\
PING 192.0.2.1 (192.0.2.1): 56 data bytes
Request timeout for icmp_seq 0
64 bytes from 192.0.2.1: icmp_seq=1 ttl=64 time=12.5 ms

--- 192.0.2.1 ping statistics ---
2 packets transmitted, 1 packets received, 50.0% packet loss
round-trip min/avg/max/stddev = 12.500/12.500/12.500/0.000 ms
";

    fn base_query() -> PingQuery {
        PingQuery {
            host: "example.com".to_string(),
            count: 4,
            interval: 1.0,
            timeout: None,
            wait_ms: None,
            size: 56,
            ttl: None,
            numeric: false,
            no_fragment: false,
        }
    }

    #[test]
    fn previews_minimal_query() {
        let ping = Ping::new(base_query()).unwrap();
        assert_eq!(ping.preview(), "ping -c 4 example.com");
    }

    #[test]
    fn previews_full_options() {
        let mut query = base_query();
        query.count = 2;
        query.interval = 0.5;
        query.timeout = Some(10);
        query.wait_ms = Some(2000);
        query.size = 64;
        query.ttl = Some(64);
        query.numeric = true;
        query.no_fragment = true;
        let ping = Ping::new(query).unwrap();
        assert_eq!(
            ping.preview(),
            "ping -c 2 -i 0.5 -t 10 -W 2000 -s 64 -m 64 -n -D example.com"
        );
    }

    #[test]
    fn rejects_option_like_input() {
        let mut query = base_query();
        query.host = "-f".to_string();
        assert!(Ping::new(query).is_err());

        let mut query = base_query();
        query.host = "example.com; rm -rf /".to_string();
        assert!(Ping::new(query).is_err());

        let mut query = base_query();
        query.host = String::new();
        assert!(Ping::new(query).is_err());

        let mut query = base_query();
        query.count = 0;
        assert!(Ping::new(query).is_err());

        let mut query = base_query();
        query.count = 21;
        assert!(Ping::new(query).is_err());

        let mut query = base_query();
        query.interval = 0.1;
        assert!(Ping::new(query).is_err());

        let mut query = base_query();
        query.size = 9000;
        assert!(Ping::new(query).is_err());

        let mut query = base_query();
        query.ttl = Some(0);
        assert!(Ping::new(query).is_err());
    }

    #[test]
    fn parses_success_output() {
        let parsed = super::parse_output(SUCCESS);
        assert_eq!(parsed.target, "127.0.0.1");
        assert_eq!(parsed.resolved_ip, "127.0.0.1");
        assert_eq!(parsed.replies.len(), 2);
        assert_eq!(parsed.replies[0].seq, 0);
        assert_eq!(parsed.replies[0].time_ms, Some(0.085));
        assert_eq!(parsed.replies[0].ttl, Some(64));
        assert!(!parsed.replies[0].timeout);
        assert_eq!(parsed.stats.transmitted, 2);
        assert_eq!(parsed.stats.received, 2);
        assert_eq!(parsed.stats.loss_percent, 0.0);
        assert_eq!(parsed.stats.avg_ms, Some(0.098));
    }

    #[test]
    fn parses_timeout_output() {
        let parsed = super::parse_output(WITH_TIMEOUT);
        assert_eq!(parsed.replies.len(), 2);
        assert!(parsed.replies[0].timeout);
        assert_eq!(parsed.replies[0].seq, 0);
        assert_eq!(parsed.replies[0].time_ms, None);
        assert_eq!(parsed.stats.loss_percent, 50.0);
    }

    #[test]
    fn tolerates_empty_output() {
        let parsed = super::parse_output("");
        assert!(parsed.replies.is_empty());
        assert_eq!(parsed.stats.transmitted, 0);
    }

    #[test]
    fn runs_ping_localhost() {
        let mut query = base_query();
        query.host = "127.0.0.1".to_string();
        query.count = 1;
        let result = super::get_snapshot(query).expect("ping");
        assert!(result.success, "stderr: {}", result.stderr);
        assert_eq!(result.command, "ping -c 1 127.0.0.1");
        assert_eq!(result.replies.len(), 1);
        assert_eq!(result.stats.received, 1);
    }
}
