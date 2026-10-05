//! `kill` の定義・実行・出力パース。
//! 自分のプロセスは権限不要で、他ユーザーのプロセスは `sudo` (AdminCommand の定型フロー) で送る。
//! 誤操作で OS やアプリ自身を止めないよう、PID 0 (プロセスグループ全体)・1 (launchd)・
//! このアプリ自身の PID は拒否する。シグナルは代表的なものだけに絞り、`-l` 等は扱わない。

use serde::{Deserialize, Serialize};

use crate::common::validate::parse_pids;
use crate::privileged;
use crate::spec::AdminCommand;
use crate::types::CommandResult;

pub const MAX_PIDS: usize = 10;
pub const SIGNALS: &[&str] = &["TERM", "INT", "HUP", "QUIT", "KILL", "STOP", "CONT"];

/// Frontend の `kill` 引数とフィールドを一致させること。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KillQuery {
    pub pids: String,
    pub signal: String,
    #[serde(default)]
    pub sudo: bool,
}

/// 送信前に見せる対象プロセス。Frontend の `KillTarget` とフィールドを一致させること。
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct KillTarget {
    pub pid: u32,
    pub found: bool,
    pub user: String,
    pub command: String,
}

/// PID ごとの送信結果。Frontend の `KillPidResult` とフィールドを一致させること。
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct KillPidResult {
    pub pid: u32,
    pub ok: bool,
    pub error: String,
}

/// `kill` の返却値。Frontend の `KillSnapshot` とフィールドを一致させること。
#[derive(Debug, Clone, Serialize)]
pub struct KillSnapshot {
    pub success: bool,
    pub exit_code: i32,
    pub command: String,
    pub signal: String,
    pub results: Vec<KillPidResult>,
    pub stderr: String,
}

pub struct Kill {
    signal: String,
    pids: Vec<String>,
    sudo: bool,
}

impl AdminCommand for Kill {
    const ID: &'static str = "kill.signal";
    const PROGRAM: &'static str = "/bin/kill";

    fn args(&self) -> Vec<String> {
        let mut args = vec!["-s".to_string(), self.signal.clone()];
        args.extend(self.pids.iter().cloned());
        args
    }

    fn preview(&self) -> String {
        format!("sudo kill {}", self.args().join(" "))
    }
}

impl Kill {
    pub fn new(query: KillQuery) -> Result<Self, String> {
        let signal = query.signal.trim().to_ascii_uppercase();
        if !SIGNALS.contains(&signal.as_str()) {
            return Err(format!("シグナルが不正です: {}", query.signal));
        }
        let pids = validate_pids(&query.pids)?;
        Ok(Self {
            signal,
            pids,
            sudo: query.sudo,
        })
    }

    pub fn shown_preview(&self) -> String {
        if self.sudo {
            AdminCommand::preview(self)
        } else {
            format!("kill {}", self.args().join(" "))
        }
    }

    pub fn execute(&self) -> Result<KillSnapshot, String> {
        let output: CommandResult = if self.sudo {
            AdminCommand::run(self)?
        } else {
            let args = self.args();
            let refs: Vec<&str> = args.iter().map(String::as_str).collect();
            privileged::execute_plain(Self::PROGRAM, &refs, self.shown_preview())?
        };
        let pids: Vec<u32> = self.pids.iter().filter_map(|p| p.parse().ok()).collect();
        Ok(KillSnapshot {
            success: output.success,
            exit_code: output.exit_code,
            command: self.shown_preview(),
            signal: self.signal.clone(),
            results: parse_results(&pids, &output.stderr),
            stderr: output.stderr.trim().to_string(),
        })
    }
}

pub fn get_snapshot(query: KillQuery) -> Result<KillSnapshot, String> {
    Kill::new(query).and_then(|cmd| cmd.execute())
}

/// 送信前の確認用に `ps -A -o pid=,user=,comm=` から対象を引く。権限不要。
/// `-p` 指定だと上限を超える PID が1つあるだけで ps 全体が失敗するため、全件から絞る。
pub fn get_targets(pids: String) -> Result<Vec<KillTarget>, String> {
    let pids = validate_pids(&pids)?;
    let output = privileged::execute_plain(
        "/bin/ps",
        &["-A", "-o", "pid=,user=,comm="],
        "ps -A -o pid=,user=,comm=".to_string(),
    )?;
    if !output.success {
        return Err(output.stderr.trim().to_string());
    }
    let found = parse_ps_targets(&output.stdout);
    Ok(pids
        .iter()
        .filter_map(|p| p.parse::<u32>().ok())
        .map(|pid| {
            found
                .iter()
                .find(|t| t.pid == pid)
                .cloned()
                .unwrap_or(KillTarget {
                    pid,
                    found: false,
                    user: String::new(),
                    command: String::new(),
                })
        })
        .collect())
}

fn validate_pids(raw: &str) -> Result<Vec<String>, String> {
    let pids = parse_pids(raw, MAX_PIDS)?;
    if pids.is_empty() {
        return Err("プロセスIDを入力してください".to_string());
    }
    let own = std::process::id();
    for pid in &pids {
        let n: u32 = pid.parse().map_err(|_| format!("プロセスIDが不正です: {pid}"))?;
        if n == 0 || n == 1 {
            return Err(format!("PID {n} には送信できません"));
        }
        if n == own {
            return Err("このアプリ自身には送信できません".to_string());
        }
    }
    Ok(pids)
}

fn parse_ps_targets(stdout: &str) -> Vec<KillTarget> {
    stdout
        .lines()
        .filter_map(|line| {
            let mut parts = line.split_whitespace();
            let pid = parts.next()?.parse::<u32>().ok()?;
            let user = parts.next()?.to_string();
            let command = parts.collect::<Vec<_>>().join(" ");
            Some(KillTarget {
                pid,
                found: true,
                user,
                command,
            })
        })
        .collect()
}

/// `kill: 123: No such process` の行がある PID を失敗、それ以外を成功とする。
fn parse_results(pids: &[u32], stderr: &str) -> Vec<KillPidResult> {
    pids.iter()
        .map(|&pid| {
            let prefix = format!("kill: {pid}: ");
            let error = stderr
                .lines()
                .find_map(|line| line.trim().strip_prefix(&prefix))
                .unwrap_or("")
                .to_string();
            KillPidResult {
                pid,
                ok: error.is_empty(),
                error,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{Kill, KillQuery};

    fn query(pids: &str, signal: &str) -> KillQuery {
        KillQuery {
            pids: pids.to_string(),
            signal: signal.to_string(),
            sudo: false,
        }
    }

    #[test]
    fn previews_plain_and_sudo() {
        let kill = Kill::new(query("123, 456", "term")).unwrap();
        assert_eq!(kill.shown_preview(), "kill -s TERM 123 456");

        let mut q = query("123", "KILL");
        q.sudo = true;
        let kill = Kill::new(q).unwrap();
        assert_eq!(kill.shown_preview(), "sudo kill -s KILL 123");
    }

    #[test]
    fn rejects_dangerous_input() {
        for (pids, signal) in [
            ("", "TERM"),
            ("0", "TERM"),
            ("1", "TERM"),
            ("-1", "TERM"),
            ("123 abc", "TERM"),
            ("1 2 3 4 5 6 7 8 9 10 11", "TERM"),
            ("123", "SEGV"),
            ("123", "-9"),
        ] {
            assert!(Kill::new(query(pids, signal)).is_err(), "{pids:?} {signal:?}");
        }
        let own = std::process::id().to_string();
        assert!(Kill::new(query(&own, "TERM")).is_err());
    }

    #[test]
    fn parses_ps_targets() {
        let targets = super::parse_ps_targets("  1 root  /sbin/launchd\n4242 me /Applications/My App.app/x\n");
        assert_eq!(targets.len(), 2);
        assert_eq!(targets[0].pid, 1);
        assert_eq!(targets[0].user, "root");
        assert_eq!(targets[1].command, "/Applications/My App.app/x");
    }

    #[test]
    fn parses_per_pid_errors() {
        let results = super::parse_results(
            &[100, 200, 300],
            "kill: 200: No such process\nkill: 300: Operation not permitted\n",
        );
        assert!(results[0].ok);
        assert!(!results[1].ok);
        assert_eq!(results[1].error, "No such process");
        assert_eq!(results[2].error, "Operation not permitted");
    }

    #[test]
    fn looks_up_and_signals_child_process() {
        let mut child = std::process::Command::new("/bin/sleep")
            .arg("30")
            .spawn()
            .expect("spawn sleep");
        let pid = child.id().to_string();

        let targets = super::get_targets(format!("{pid} 999999")).expect("targets");
        assert!(targets[0].found);
        assert!(targets[0].command.contains("sleep"));
        assert!(!targets[1].found);

        let result = super::get_snapshot(query(&pid, "TERM")).expect("kill");
        assert!(result.success, "stderr: {}", result.stderr);
        assert!(result.results[0].ok);
        let _ = child.wait();
    }
}
