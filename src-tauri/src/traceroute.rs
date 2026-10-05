//! `traceroute` の定義・実行・出力パース。管理者権限は不要 (/usr/sbin/traceroute は setuid)。
//! 実行が長引かないよう、最大ホップ数 × 試行回数 × 待ち秒数の最悪値を MAX_WORST_SECS 以内に制限する。
//! 送信元指定 (`-s`/`-i`)・ゲートウェイ (`-g`)・ルーティング無視 (`-r`)・
//! ファイアウォール回避 (`-e`)・TOS/ECN・チェックサム操作など低レベルなフラグは扱わない。

use serde::{Deserialize, Serialize};

use crate::ping::validate_host;
use crate::privileged;

pub const DEFAULT_MAX_TTL: u32 = 20;
pub const MAX_MAX_TTL: u32 = 30;
pub const DEFAULT_QUERIES: u32 = 3;
pub const MAX_QUERIES: u32 = 3;
pub const DEFAULT_WAIT: u32 = 2;
pub const MAX_WAIT: u32 = 5;
pub const MAX_WORST_SECS: u32 = 150;

/// Frontend の `getTraceroute` 引数とフィールドを一致させること。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TracerouteQuery {
    pub host: String,
    #[serde(default = "default_max_ttl")]
    pub max_ttl: u32,
    #[serde(default)]
    pub first_ttl: Option<u32>,
    #[serde(default = "default_queries")]
    pub queries: u32,
    #[serde(default = "default_wait")]
    pub wait: u32,
    #[serde(default)]
    pub icmp: bool,
    #[serde(default)]
    pub numeric: bool,
    #[serde(default)]
    pub as_lookup: bool,
}

fn default_max_ttl() -> u32 {
    DEFAULT_MAX_TTL
}

fn default_queries() -> u32 {
    DEFAULT_QUERIES
}

fn default_wait() -> u32 {
    DEFAULT_WAIT
}

/// ホップ内で応答した機器1台分。Frontend の `TracerouteResponder` とフィールドを一致させること。
#[derive(Debug, Clone, Default, Serialize, PartialEq)]
pub struct TracerouteResponder {
    pub host: String,
    pub ip: String,
    pub asn: String,
    pub rtts_ms: Vec<f64>,
    /// `!H` (ホスト到達不能) などの注記
    pub annotations: Vec<String>,
}

/// 1ホップ。Frontend の `TracerouteHop` とフィールドを一致させること。
#[derive(Debug, Clone, Default, Serialize, PartialEq)]
pub struct TracerouteHop {
    pub ttl: u32,
    pub responders: Vec<TracerouteResponder>,
    pub timeouts: u32,
}

/// `get_traceroute` の返却値。Frontend の `TracerouteSnapshot` とフィールドを一致させること。
#[derive(Debug, Clone, Serialize)]
pub struct TracerouteSnapshot {
    pub success: bool,
    pub exit_code: i32,
    pub command: String,
    pub target: String,
    pub resolved_ip: String,
    pub max_hops: u32,
    pub hops: Vec<TracerouteHop>,
    pub reached: bool,
    pub raw: String,
    pub stderr: String,
}

pub struct Traceroute {
    args: Vec<String>,
}

impl Traceroute {
    pub fn new(query: TracerouteQuery) -> Result<Self, String> {
        let args = build_args(&query)?;
        Ok(Self { args })
    }

    pub fn preview(&self) -> String {
        format!("traceroute {}", self.args.join(" "))
    }

    pub fn run(&self) -> Result<TracerouteSnapshot, String> {
        let preview = self.preview();
        let refs: Vec<&str> = self.args.iter().map(String::as_str).collect();
        let output = privileged::execute_plain("/usr/sbin/traceroute", &refs, preview.clone())?;
        // 見出し行 (`traceroute to …`) は stderr に出るため、取り出したうえで stderr から外す。
        let (target, resolved_ip, max_hops, stderr) = split_header(&output.stderr);
        let hops = parse_hops(&output.stdout);
        let reached = !resolved_ip.is_empty()
            && hops
                .last()
                .map(|hop| hop.responders.iter().any(|r| r.ip == resolved_ip))
                .unwrap_or(false);
        Ok(TracerouteSnapshot {
            success: output.success,
            exit_code: output.exit_code,
            command: preview,
            target,
            resolved_ip,
            max_hops,
            hops,
            reached,
            raw: output.stdout.trim_end().to_string(),
            stderr,
        })
    }
}

pub fn get_snapshot(query: TracerouteQuery) -> Result<TracerouteSnapshot, String> {
    Traceroute::new(query).and_then(|cmd| cmd.run())
}

fn build_args(query: &TracerouteQuery) -> Result<Vec<String>, String> {
    let host = query.host.trim();
    validate_host(host)?;
    if query.max_ttl == 0 || query.max_ttl > MAX_MAX_TTL {
        return Err(format!("最大ホップ数は 1〜{MAX_MAX_TTL} です"));
    }
    if let Some(first) = query.first_ttl {
        if first == 0 || first > query.max_ttl {
            return Err("開始ホップは 1〜最大ホップ数 です".to_string());
        }
    }
    if query.queries == 0 || query.queries > MAX_QUERIES {
        return Err(format!("試行回数は 1〜{MAX_QUERIES} です"));
    }
    if query.wait == 0 || query.wait > MAX_WAIT {
        return Err(format!("待ち時間は 1〜{MAX_WAIT} 秒です"));
    }
    let first = query.first_ttl.unwrap_or(1);
    let worst = (query.max_ttl - first + 1) * query.queries * query.wait;
    if worst > MAX_WORST_SECS {
        return Err(format!(
            "最悪の所要時間が {worst} 秒になります。ホップ数・試行回数・待ち時間を減らしてください (上限 {MAX_WORST_SECS} 秒)"
        ));
    }

    let mut args: Vec<String> = vec!["-m".to_string(), query.max_ttl.to_string()];
    if let Some(first) = query.first_ttl {
        args.push("-f".to_string());
        args.push(first.to_string());
    }
    args.push("-q".to_string());
    args.push(query.queries.to_string());
    args.push("-w".to_string());
    args.push(query.wait.to_string());
    if query.icmp {
        args.push("-I".to_string());
    }
    if query.numeric {
        args.push("-n".to_string());
    }
    if query.as_lookup {
        args.push("-a".to_string());
    }
    args.push(host.to_string());
    Ok(args)
}

/// `traceroute to example.com (104.20.23.154), 6 hops max, 40 byte packets` を取り出す。
fn split_header(stderr: &str) -> (String, String, u32, String) {
    let mut target = String::new();
    let mut resolved_ip = String::new();
    let mut max_hops = 0;
    let mut rest = Vec::new();
    for line in stderr.lines() {
        let trimmed = line.trim();
        if let Some(body) = trimmed.strip_prefix("traceroute to ") {
            target = body.split_whitespace().next().unwrap_or("").to_string();
            resolved_ip = body
                .find('(')
                .and_then(|s| body[s + 1..].find(')').map(|e| body[s + 1..s + 1 + e].to_string()))
                .unwrap_or_default();
            max_hops = body
                .split(',')
                .find_map(|part| part.trim().strip_suffix(" hops max"))
                .and_then(|n| n.trim().parse().ok())
                .unwrap_or(0);
        } else if !trimmed.is_empty() {
            rest.push(trimmed);
        }
    }
    (target, resolved_ip, max_hops, rest.join("\n"))
}

/// ホップ行 (` 2  host (ip)  1.2 ms`) と、同じホップの別応答元を示す継続行 (`    host (ip)  1.3 ms`) を読む。
fn parse_hops(stdout: &str) -> Vec<TracerouteHop> {
    let mut hops: Vec<TracerouteHop> = Vec::new();
    for line in stdout.lines() {
        let mut tokens = line.split_whitespace().peekable();
        let Some(first) = tokens.peek().copied() else {
            continue;
        };
        // 先頭が整数ならホップ行、そうでなければ直前ホップの継続行
        if let Ok(ttl) = first.parse::<u32>() {
            tokens.next();
            hops.push(TracerouteHop {
                ttl,
                ..Default::default()
            });
        }
        let Some(hop) = hops.last_mut() else {
            continue;
        };
        parse_probe_tokens(hop, tokens.collect());
    }
    for hop in &mut hops {
        for responder in &mut hop.responders {
            if responder.ip.is_empty() {
                responder.ip = responder.host.clone();
            }
        }
    }
    hops
}

fn parse_probe_tokens(hop: &mut TracerouteHop, tokens: Vec<&str>) {
    let mut pending_asn = String::new();
    let mut i = 0;
    while i < tokens.len() {
        let token = tokens[i];
        if token == "*" {
            hop.timeouts += 1;
        } else if token.starts_with('[') && token.ends_with(']') {
            pending_asn = token.trim_matches(|c| c == '[' || c == ']').to_string();
        } else if token.starts_with('(') && token.ends_with(')') {
            if let Some(responder) = hop.responders.last_mut() {
                responder.ip = token.trim_matches(|c| c == '(' || c == ')').to_string();
            }
        } else if token.starts_with('!') {
            if let Some(responder) = hop.responders.last_mut() {
                responder.annotations.push(token.to_string());
            }
        } else if tokens.get(i + 1) == Some(&"ms") && token.parse::<f64>().is_ok() {
            if let Some(responder) = hop.responders.last_mut() {
                responder.rtts_ms.push(token.parse().unwrap_or(0.0));
            }
            i += 1;
        } else {
            hop.responders.push(TracerouteResponder {
                host: token.to_string(),
                asn: std::mem::take(&mut pending_asn),
                ..Default::default()
            });
        }
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::{Traceroute, TracerouteQuery};

    const MIXED: &str = " 1  10.5.0.1 (10.5.0.1)  22.580 ms  19.798 ms
 2  187.14.63.3 (187.14.63.3)  20.002 ms
    187.14.63.2 (187.14.63.2)  19.531 ms
 3  * *
 4  router.example.net (162.158.4.25)  24.321 ms *
 5  104.20.23.154 (104.20.23.154)  20.926 ms !H  21.733 ms !H
";

    const STDERR: &str = "traceroute: Warning: example.com has multiple addresses; using 104.20.23.154
traceroute to example.com (104.20.23.154), 6 hops max, 40 byte packets
";

    fn base_query() -> TracerouteQuery {
        TracerouteQuery {
            host: "example.com".to_string(),
            max_ttl: 20,
            first_ttl: None,
            queries: 3,
            wait: 2,
            icmp: false,
            numeric: false,
            as_lookup: false,
        }
    }

    #[test]
    fn previews_minimal_and_full_query() {
        let cmd = Traceroute::new(base_query()).unwrap();
        assert_eq!(cmd.preview(), "traceroute -m 20 -q 3 -w 2 example.com");

        let mut query = base_query();
        query.max_ttl = 15;
        query.first_ttl = Some(3);
        query.queries = 2;
        query.wait = 1;
        query.icmp = true;
        query.numeric = true;
        query.as_lookup = true;
        let cmd = Traceroute::new(query).unwrap();
        assert_eq!(
            cmd.preview(),
            "traceroute -m 15 -f 3 -q 2 -w 1 -I -n -a example.com"
        );
    }

    #[test]
    fn rejects_bad_input() {
        let cases: Vec<Box<dyn Fn(&mut TracerouteQuery)>> = vec![
            Box::new(|q| q.host = "-g evil".to_string()),
            Box::new(|q| q.host = String::new()),
            Box::new(|q| q.max_ttl = 0),
            Box::new(|q| q.max_ttl = 31),
            Box::new(|q| q.first_ttl = Some(21)),
            Box::new(|q| q.queries = 4),
            Box::new(|q| q.wait = 6),
            // 30 hops × 3 × 5 秒 = 450 秒は上限超え
            Box::new(|q| {
                q.max_ttl = 30;
                q.wait = 5;
            }),
        ];
        for mutate in cases {
            let mut query = base_query();
            mutate(&mut query);
            assert!(Traceroute::new(query).is_err());
        }
    }

    #[test]
    fn parses_hops_with_multiple_responders_and_timeouts() {
        let hops = super::parse_hops(MIXED);
        assert_eq!(hops.len(), 5);
        assert_eq!(hops[0].responders[0].rtts_ms, vec![22.580, 19.798]);
        assert_eq!(hops[1].responders.len(), 2);
        assert_eq!(hops[1].responders[1].ip, "187.14.63.2");
        assert_eq!(hops[2].timeouts, 2);
        assert!(hops[2].responders.is_empty());
        assert_eq!(hops[3].responders[0].host, "router.example.net");
        assert_eq!(hops[3].responders[0].ip, "162.158.4.25");
        assert_eq!(hops[3].timeouts, 1);
        assert_eq!(hops[4].responders[0].annotations, vec!["!H", "!H"]);
        assert_eq!(hops[4].responders[0].rtts_ms.len(), 2);
    }

    #[test]
    fn parses_numeric_and_as_output() {
        let hops = super::parse_hops(" 1  [AS13335] 127.0.0.1  0.433 ms  0.093 ms\n");
        assert_eq!(hops[0].responders[0].host, "127.0.0.1");
        assert_eq!(hops[0].responders[0].ip, "127.0.0.1");
        assert_eq!(hops[0].responders[0].asn, "AS13335");
    }

    #[test]
    fn splits_header_from_warnings() {
        let (target, ip, max_hops, rest) = super::split_header(STDERR);
        assert_eq!(target, "example.com");
        assert_eq!(ip, "104.20.23.154");
        assert_eq!(max_hops, 6);
        assert!(rest.starts_with("traceroute: Warning"));
    }

    #[test]
    fn runs_traceroute_localhost() {
        let mut query = base_query();
        query.host = "127.0.0.1".to_string();
        query.max_ttl = 2;
        query.queries = 1;
        query.wait = 1;
        query.numeric = true;
        let result = super::get_snapshot(query).expect("traceroute");
        assert!(result.success, "stderr: {}", result.stderr);
        assert!(result.reached);
        assert_eq!(result.hops.len(), 1);
        assert_eq!(result.resolved_ip, "127.0.0.1");
    }
}
