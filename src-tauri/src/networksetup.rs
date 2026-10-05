//! `networksetup` の参照系 (`-list…` / `-get…`) の定義・実行・出力パース。管理者権限は不要。
//! `-set…` / `-create…` / `-remove…` / `-switchtolocation` などの書き込み系は一切受け付けない。
//! サービス名は自由入力にせず、`-listallnetworkservices` に実在する名前だけを許可する。
//! 1つのサブコマンドで複数の取得系を続けて実行する場合は、実行した全コマンドを `commands` で返す。

use serde::{Deserialize, Serialize};

use crate::privileged;
use crate::types::CommandResult;

const PROGRAM: &str = "/usr/sbin/networksetup";

/// 許可するサブコマンド。Frontend の `NetworksetupSub` と一致させること。
pub const ALLOWED_SUBS: &[&str] = &["services", "info", "wifi", "locations"];

/// Frontend の `getNetworksetup` 引数とフィールドを一致させること。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworksetupQuery {
    pub sub: String,
    /// info で使うサービス名
    #[serde(default)]
    pub service: String,
    /// wifi で使うデバイス名。空なら Wi-Fi のハードウェアポートから探す
    #[serde(default)]
    pub device: String,
}

/// `key: value` の1行。Frontend の `NetworksetupEntry` とフィールドを一致させること。
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct NetworksetupEntry {
    pub key: String,
    pub value: String,
}

/// サービス1件。Frontend の `NetworkService` とフィールドを一致させること。
#[derive(Debug, Clone, Default, Serialize, PartialEq)]
pub struct NetworkService {
    pub order: Option<u32>,
    pub name: String,
    pub enabled: bool,
    pub hardware_port: String,
    pub device: String,
}

/// ハードウェアポート1件 (`-listallhardwareports`)。ifconfig でも使う。
#[derive(Debug, Clone, Default, Serialize, PartialEq)]
pub struct HardwarePort {
    pub port: String,
    pub device: String,
    pub mac: String,
}

/// プロキシ設定1種類。Frontend の `ProxySetting` とフィールドを一致させること。
#[derive(Debug, Clone, Default, Serialize, PartialEq)]
pub struct ProxySetting {
    /// "web" / "secureweb" / "socks"
    pub kind: String,
    pub enabled: bool,
    pub server: String,
    pub port: String,
    pub authenticated: bool,
}

/// 実行結果の共通部分。Frontend の各 Snapshot と一致させること。
#[derive(Debug, Clone, Serialize)]
pub struct RunMeta {
    pub success: bool,
    pub exit_code: i32,
    pub command: String,
    pub commands: Vec<String>,
    pub stderr: String,
}

/// `services` の返却値。
#[derive(Debug, Clone, Serialize)]
pub struct ServicesSnapshot {
    #[serde(flatten)]
    pub meta: RunMeta,
    pub services: Vec<NetworkService>,
}

/// `info` の返却値。
#[derive(Debug, Clone, Serialize)]
pub struct InfoSnapshot {
    #[serde(flatten)]
    pub meta: RunMeta,
    pub service: String,
    pub info: Vec<NetworksetupEntry>,
    pub dns_servers: Vec<String>,
    pub search_domains: Vec<String>,
    pub proxies: Vec<ProxySetting>,
    pub auto_proxy_discovery: bool,
    pub auto_proxy_url: String,
    pub auto_proxy_enabled: bool,
    pub bypass_domains: Vec<String>,
}

/// `wifi` の返却値。
#[derive(Debug, Clone, Serialize)]
pub struct WifiSnapshot {
    #[serde(flatten)]
    pub meta: RunMeta,
    pub device: String,
    pub power: String,
    /// 接続中のネットワーク名。未接続または位置情報の権限が無く取れない場合は空
    pub network: String,
    pub network_message: String,
}

/// `locations` の返却値。
#[derive(Debug, Clone, Serialize)]
pub struct LocationsSnapshot {
    #[serde(flatten)]
    pub meta: RunMeta,
    pub current: String,
    pub locations: Vec<String>,
}

/// Frontend へ返す共用体。`sub` ごとに中身が変わる。
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum NetworksetupResult {
    Services(ServicesSnapshot),
    Info(InfoSnapshot),
    Wifi(WifiSnapshot),
    Locations(LocationsSnapshot),
}

pub fn get_snapshot(query: NetworksetupQuery) -> Result<NetworksetupResult, String> {
    let sub = query.sub.trim();
    if !ALLOWED_SUBS.contains(&sub) {
        return Err(format!("サブコマンドが不正です: {sub}"));
    }
    match sub {
        "services" => run_services().map(NetworksetupResult::Services),
        "info" => run_info(query.service.trim()).map(NetworksetupResult::Info),
        "wifi" => run_wifi(query.device.trim()).map(NetworksetupResult::Wifi),
        _ => run_locations().map(NetworksetupResult::Locations),
    }
}

/// サービス詳細の選択肢に使う名前一覧 (`-listallnetworkservices`)。無効なサービスも含む。
pub fn list_service_names() -> Result<Vec<String>, String> {
    let output = run(&["-listallnetworkservices"])?;
    if !output.success {
        return Err(output.stdout.trim().to_string());
    }
    Ok(parse_service_names(&output.stdout))
}

/// `-listallhardwareports` の対応表。
pub fn hardware_ports() -> Result<Vec<HardwarePort>, String> {
    let output = run(&["-listallhardwareports"])?;
    Ok(parse_hardware_ports(&output.stdout))
}

fn run(args: &[&str]) -> Result<CommandResult, String> {
    privileged::execute_plain(PROGRAM, args, preview_of(args))
}

fn preview_of(args: &[&str]) -> String {
    let quoted: Vec<String> = args
        .iter()
        .map(|a| {
            if a.contains(' ') {
                format!("\"{a}\"")
            } else {
                a.to_string()
            }
        })
        .collect();
    format!("networksetup {}", quoted.join(" "))
}

/// 複数コマンドの結果を1つにまとめる。表示用の `command` は先頭のコマンド。
fn meta_of(outputs: &[CommandResult]) -> RunMeta {
    let failed = outputs.iter().find(|o| !o.success);
    RunMeta {
        success: failed.is_none(),
        exit_code: failed.map(|o| o.exit_code).unwrap_or(0),
        command: outputs.first().map(|o| o.command.clone()).unwrap_or_default(),
        commands: outputs.iter().map(|o| o.command.clone()).collect(),
        // networksetup はエラーも stdout に出すため、失敗したものは stdout も含める
        stderr: outputs
            .iter()
            .filter(|o| !o.success)
            .map(|o| format!("{}\n{}", o.stdout.trim(), o.stderr.trim()).trim().to_string())
            .collect::<Vec<_>>()
            .join("\n"),
    }
}

fn run_services() -> Result<ServicesSnapshot, String> {
    let output = run(&["-listnetworkserviceorder"])?;
    let services = parse_service_order(&output.stdout);
    Ok(ServicesSnapshot {
        meta: meta_of(&[output]),
        services,
    })
}

fn run_info(service: &str) -> Result<InfoSnapshot, String> {
    if service.is_empty() {
        return Err("サービスを選んでください".to_string());
    }
    if !list_service_names()?.iter().any(|name| name == service) {
        return Err(format!("サービスが見つかりません: {service}"));
    }
    let getters = [
        "-getinfo",
        "-getdnsservers",
        "-getsearchdomains",
        "-getwebproxy",
        "-getsecurewebproxy",
        "-getsocksfirewallproxy",
        "-getproxyautodiscovery",
        "-getautoproxyurl",
        "-getproxybypassdomains",
    ];
    let mut outputs = Vec::new();
    for getter in getters {
        outputs.push(run(&[getter, service])?);
    }
    let text = |i: usize| outputs[i].stdout.as_str();
    let proxies = [("web", 3), ("secureweb", 4), ("socks", 5)]
        .iter()
        .map(|(kind, i)| parse_proxy(kind, text(*i)))
        .collect();
    let auto_url = parse_entries(text(7));
    Ok(InfoSnapshot {
        service: service.to_string(),
        info: parse_entries(text(0)),
        dns_servers: parse_list(text(1)),
        search_domains: parse_list(text(2)),
        proxies,
        auto_proxy_discovery: parse_entries(text(6))
            .iter()
            .any(|e| e.key == "Auto Proxy Discovery" && e.value == "On"),
        auto_proxy_url: entry_value(&auto_url, "URL")
            .filter(|v| v != "(null)")
            .unwrap_or_default(),
        auto_proxy_enabled: entry_value(&auto_url, "Enabled").as_deref() == Some("Yes"),
        bypass_domains: parse_list(text(8)),
        meta: meta_of(&outputs),
    })
}

fn run_wifi(device: &str) -> Result<WifiSnapshot, String> {
    let device = if device.is_empty() {
        hardware_ports()?
            .into_iter()
            .find(|p| p.port == "Wi-Fi")
            .map(|p| p.device)
            .ok_or_else(|| "Wi-Fi のハードウェアポートが見つかりません".to_string())?
    } else {
        validate_device(device)?;
        device.to_string()
    };
    let power = run(&["-getairportpower", &device])?;
    let network = run(&["-getairportnetwork", &device])?;
    let network_line = network.stdout.trim().to_string();
    Ok(WifiSnapshot {
        device: device.clone(),
        power: power
            .stdout
            .trim()
            .rsplit(": ")
            .next()
            .unwrap_or("")
            .to_string(),
        network: network_line
            .strip_prefix("Current Wi-Fi Network: ")
            .unwrap_or("")
            .to_string(),
        network_message: network_line,
        meta: meta_of(&[power, network]),
    })
}

fn run_locations() -> Result<LocationsSnapshot, String> {
    let current = run(&["-getcurrentlocation"])?;
    let list = run(&["-listlocations"])?;
    Ok(LocationsSnapshot {
        current: current.stdout.trim().to_string(),
        locations: parse_list(&list.stdout),
        meta: meta_of(&[current, list]),
    })
}

/// `en0` のような英小文字 + 数字の名前に限る。
fn validate_device(name: &str) -> Result<(), String> {
    let letters = name.trim_end_matches(|c: char| c.is_ascii_digit());
    let ok = !letters.is_empty()
        && letters.len() < name.len()
        && name.len() <= 20
        && letters.chars().all(|c| c.is_ascii_lowercase());
    if ok {
        Ok(())
    } else {
        Err(format!("デバイス名が不正です: {name}"))
    }
}

/// 先頭の注意書きを除き、無効サービスの `*` を外して名前だけにする。
fn parse_service_names(stdout: &str) -> Vec<String> {
    stdout
        .lines()
        .filter(|line| !line.starts_with("An asterisk"))
        .map(|line| line.trim_start_matches('*').trim().to_string())
        .filter(|line| !line.is_empty())
        .collect()
}

/// `(1) Wi-Fi` / `(*) Name` の行と、続く `(Hardware Port: Wi-Fi, Device: en0)` の行を組にする。
fn parse_service_order(stdout: &str) -> Vec<NetworkService> {
    let mut services: Vec<NetworkService> = Vec::new();
    for line in stdout.lines().map(str::trim) {
        if let Some(rest) = line.strip_prefix("(Hardware Port: ") {
            if let Some(service) = services.last_mut() {
                let inner = rest.trim_end_matches(')');
                let (port, device) = inner.rsplit_once(", Device:").unwrap_or((inner, ""));
                service.hardware_port = port.trim().to_string();
                service.device = device.trim().to_string();
            }
            continue;
        }
        let Some(rest) = line.strip_prefix('(') else {
            continue;
        };
        let Some((marker, name)) = rest.split_once(") ") else {
            continue;
        };
        services.push(NetworkService {
            order: marker.parse().ok(),
            enabled: marker != "*",
            name: name.trim().to_string(),
            ..Default::default()
        });
    }
    services
}

fn parse_hardware_ports(stdout: &str) -> Vec<HardwarePort> {
    let mut ports: Vec<HardwarePort> = Vec::new();
    for line in stdout.lines().map(str::trim) {
        if let Some(port) = line.strip_prefix("Hardware Port: ") {
            ports.push(HardwarePort {
                port: port.to_string(),
                ..Default::default()
            });
        } else if let Some(port) = ports.last_mut() {
            if let Some(device) = line.strip_prefix("Device: ") {
                port.device = device.to_string();
            } else if let Some(mac) = line.strip_prefix("Ethernet Address: ") {
                port.mac = mac.to_string();
            }
        }
    }
    ports
}

fn parse_entries(stdout: &str) -> Vec<NetworksetupEntry> {
    stdout
        .lines()
        .filter_map(|line| {
            let (key, value) = line.split_once(':')?;
            Some(NetworksetupEntry {
                key: key.trim().to_string(),
                value: value.trim().to_string(),
            })
        })
        .collect()
}

fn entry_value(entries: &[NetworksetupEntry], key: &str) -> Option<String> {
    entries.iter().find(|e| e.key == key).map(|e| e.value.clone())
}

/// DNS サーバー・検索ドメイン・除外ドメインなど1行1件の一覧。
/// 未設定時の `There aren't any …` は空として扱う。
fn parse_list(stdout: &str) -> Vec<String> {
    stdout
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with("There aren't any"))
        .map(str::to_string)
        .collect()
}

fn parse_proxy(kind: &str, stdout: &str) -> ProxySetting {
    let entries = parse_entries(stdout);
    let server = entry_value(&entries, "Server").unwrap_or_default();
    let port = entry_value(&entries, "Port").unwrap_or_default();
    ProxySetting {
        kind: kind.to_string(),
        enabled: entry_value(&entries, "Enabled").as_deref() == Some("Yes"),
        // 未設定時は Server: 0 / Port: 0 と出るので空にそろえる
        server: if server == "0" { String::new() } else { server },
        port: if port == "0" { String::new() } else { port },
        authenticated: entry_value(&entries, "Authenticated Proxy Enabled").as_deref() == Some("1"),
    }
}

#[cfg(test)]
mod tests {
    use super::{get_snapshot, NetworksetupQuery, NetworksetupResult};

    const ORDER: &str = "\
An asterisk (*) denotes that a network service is disabled.
(1) Thunderbolt Bridge
(Hardware Port: Thunderbolt Bridge, Device: bridge0)

(2) Wi-Fi
(Hardware Port: Wi-Fi, Device: en0)

(*) Old VPN (Office)
(Hardware Port: L2TP, Device: )

(3) Tailscale
(Hardware Port: io.tailscale.ipn.macsys, Device: )
";

    fn query(sub: &str, service: &str, device: &str) -> NetworksetupQuery {
        NetworksetupQuery {
            sub: sub.to_string(),
            service: service.to_string(),
            device: device.to_string(),
        }
    }

    #[test]
    fn parses_service_order() {
        let services = super::parse_service_order(ORDER);
        assert_eq!(services.len(), 4);
        assert_eq!(services[1].name, "Wi-Fi");
        assert_eq!(services[1].order, Some(2));
        assert_eq!(services[1].device, "en0");
        assert!(services[1].enabled);
        assert_eq!(services[2].name, "Old VPN (Office)");
        assert!(!services[2].enabled);
        assert_eq!(services[2].order, None);
        assert_eq!(services[3].hardware_port, "io.tailscale.ipn.macsys");
        assert_eq!(services[3].device, "");
    }

    #[test]
    fn parses_service_names() {
        let names = super::parse_service_names(
            "An asterisk (*) denotes that a network service is disabled.\nWi-Fi\n*Old VPN\n",
        );
        assert_eq!(names, vec!["Wi-Fi", "Old VPN"]);
    }

    #[test]
    fn parses_hardware_ports() {
        let ports = super::parse_hardware_ports(
            "\nHardware Port: Wi-Fi\nDevice: en0\nEthernet Address: 10:9f:41:b6:6a:7f\n\nHardware Port: Thunderbolt Bridge\nDevice: bridge0\nEthernet Address: N/A\n",
        );
        assert_eq!(ports.len(), 2);
        assert_eq!(ports[0].port, "Wi-Fi");
        assert_eq!(ports[0].device, "en0");
        assert_eq!(ports[1].mac, "N/A");
    }

    #[test]
    fn parses_proxy_and_lists() {
        let off = super::parse_proxy("web", "Enabled: No\nServer: 0\nPort: 0\nAuthenticated Proxy Enabled: 0\n");
        assert!(!off.enabled);
        assert_eq!(off.server, "");
        assert_eq!(off.port, "");

        let on = super::parse_proxy(
            "secureweb",
            "Enabled: Yes\nServer: proxy.example.com\nPort: 8080\nAuthenticated Proxy Enabled: 1\n",
        );
        assert!(on.enabled);
        assert_eq!(on.server, "proxy.example.com");
        assert!(on.authenticated);

        assert!(super::parse_list("There aren't any DNS Servers set on Wi-Fi.\n").is_empty());
        assert_eq!(super::parse_list("1.1.1.1\n8.8.8.8\n"), vec!["1.1.1.1", "8.8.8.8"]);
    }

    #[test]
    fn rejects_bad_input() {
        assert!(get_snapshot(query("setdnsservers", "", "")).is_err());
        assert!(get_snapshot(query("info", "", "")).is_err());
        assert!(get_snapshot(query("info", "No Such Service manmen", "")).is_err());
        for device in ["-x", "en0 ;", "EN0", "en"] {
            assert!(get_snapshot(query("wifi", "", device)).is_err(), "{device}");
        }
    }

    #[test]
    fn runs_services_and_locations() {
        match get_snapshot(query("services", "", "")).expect("services") {
            NetworksetupResult::Services(s) => {
                assert!(s.meta.success, "stderr: {}", s.meta.stderr);
                assert_eq!(s.meta.command, "networksetup -listnetworkserviceorder");
            }
            _ => panic!("unexpected kind"),
        }
        match get_snapshot(query("locations", "", "")).expect("locations") {
            NetworksetupResult::Locations(l) => {
                assert!(l.meta.success, "stderr: {}", l.meta.stderr);
                assert!(!l.current.is_empty());
                assert_eq!(l.meta.commands.len(), 2);
            }
            _ => panic!("unexpected kind"),
        }
    }

    #[test]
    fn runs_info_for_first_service() {
        let names = super::list_service_names().expect("names");
        let Some(first) = names.first() else {
            return;
        };
        match get_snapshot(query("info", first, "")).expect("info") {
            NetworksetupResult::Info(info) => {
                assert_eq!(info.meta.commands.len(), 9);
                assert_eq!(info.proxies.len(), 3);
            }
            _ => panic!("unexpected kind"),
        }
    }
}
