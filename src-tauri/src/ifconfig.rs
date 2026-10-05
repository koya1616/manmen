//! `ifconfig` の参照系の定義・実行・出力パース。管理者権限は不要。
//! 表示のみを扱い、アドレス設定・up/down・create/destroy・vlan/bond 等の書き込み系は受け付けない。
//! インターフェース名だけでは用途が分からないため、`networksetup -listallhardwareports` の
//! 対応表 (en0 = Wi-Fi など) を結果に添える。

use serde::{Deserialize, Serialize};

use crate::networksetup;
use crate::privileged;

pub const FAMILIES: &[&str] = &["inet", "inet6"];

/// Frontend の `getIfconfig` 引数とフィールドを一致させること。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IfconfigQuery {
    /// 空なら全インターフェース (`-a`)
    #[serde(default)]
    pub interface: String,
    /// UP のものだけ (`-u`)。全インターフェース表示のときのみ有効
    #[serde(default)]
    pub up_only: bool,
    /// "" / "inet" / "inet6"
    #[serde(default)]
    pub family: String,
}

/// IPv4 アドレス1件。Frontend の `IfconfigInet` とフィールドを一致させること。
#[derive(Debug, Clone, Default, Serialize, PartialEq)]
pub struct IfconfigInet {
    pub address: String,
    /// `255.255.255.0` 形式に直したもの
    pub netmask: String,
    pub prefix_len: Option<u32>,
    pub broadcast: String,
    /// point-to-point の相手 (`-->` の右側)
    pub destination: String,
}

/// IPv6 アドレス1件。Frontend の `IfconfigInet6` とフィールドを一致させること。
#[derive(Debug, Clone, Default, Serialize, PartialEq)]
pub struct IfconfigInet6 {
    pub address: String,
    pub prefix_len: Option<u32>,
    /// `fe80::` で始まるリンクローカルかどうか
    pub link_local: bool,
    /// `secured` / `temporary` / `autoconf` などの属性
    pub attributes: Vec<String>,
}

/// 1インターフェース。Frontend の `IfconfigInterface` とフィールドを一致させること。
#[derive(Debug, Clone, Default, Serialize, PartialEq)]
pub struct IfconfigInterface {
    pub name: String,
    pub flags: Vec<String>,
    pub mtu: Option<u32>,
    pub mac: String,
    pub inet: Vec<IfconfigInet>,
    pub inet6: Vec<IfconfigInet6>,
    pub status: String,
    pub media: String,
    /// networksetup のハードウェアポート名 (`Wi-Fi` など)。対応が無ければ空
    pub hardware_port: String,
}

/// `get_ifconfig` の返却値。Frontend の `IfconfigSnapshot` とフィールドを一致させること。
#[derive(Debug, Clone, Serialize)]
pub struct IfconfigSnapshot {
    pub success: bool,
    pub exit_code: i32,
    pub command: String,
    pub interfaces: Vec<IfconfigInterface>,
    pub stderr: String,
}

pub struct Ifconfig {
    args: Vec<String>,
}

impl Ifconfig {
    pub fn new(query: IfconfigQuery) -> Result<Self, String> {
        let interface = query.interface.trim();
        let family = query.family.trim();
        if !family.is_empty() && !FAMILIES.contains(&family) {
            return Err(format!("アドレスファミリーが不正です: {family}"));
        }
        let mut args: Vec<String> = Vec::new();
        if interface.is_empty() {
            args.push("-a".to_string());
            if query.up_only {
                args.push("-u".to_string());
            }
        } else {
            validate_interface(interface)?;
            args.push(interface.to_string());
        }
        if !family.is_empty() {
            args.push(family.to_string());
        }
        Ok(Self { args })
    }

    pub fn preview(&self) -> String {
        format!("ifconfig {}", self.args.join(" "))
    }

    pub fn run(&self) -> Result<IfconfigSnapshot, String> {
        let preview = self.preview();
        let refs: Vec<&str> = self.args.iter().map(String::as_str).collect();
        let output = privileged::execute_plain("/sbin/ifconfig", &refs, preview.clone())?;
        let mut interfaces = parse_interfaces(&output.stdout);
        // 対応表が取れなくても ifconfig の結果は返す
        let ports = networksetup::hardware_ports().unwrap_or_default();
        for iface in &mut interfaces {
            if let Some(port) = ports.iter().find(|p| p.device == iface.name) {
                iface.hardware_port = port.port.clone();
            }
        }
        Ok(IfconfigSnapshot {
            success: output.success,
            exit_code: output.exit_code,
            command: preview,
            interfaces,
            stderr: output.stderr.trim().to_string(),
        })
    }
}

pub fn get_snapshot(query: IfconfigQuery) -> Result<IfconfigSnapshot, String> {
    Ifconfig::new(query).and_then(|cmd| cmd.run())
}

/// `en0` / `utun4` / `bridge0` のような英小文字 + 数字の名前に限る。
fn validate_interface(name: &str) -> Result<(), String> {
    let letters = name.trim_end_matches(|c: char| c.is_ascii_digit());
    let ok = !letters.is_empty()
        && letters.len() <= 15
        && name.len() - letters.len() <= 5
        && letters.chars().all(|c| c.is_ascii_lowercase());
    if ok {
        Ok(())
    } else {
        Err(format!("インターフェース名が不正です: {name}"))
    }
}

/// 行頭から始まる `en0: flags=8863<UP,...> mtu 1500` がインターフェースの区切り。
/// インデントされた行はそのインターフェースの属性として読む。
fn parse_interfaces(stdout: &str) -> Vec<IfconfigInterface> {
    let mut interfaces: Vec<IfconfigInterface> = Vec::new();
    for line in stdout.lines() {
        if line.is_empty() {
            continue;
        }
        if !line.starts_with(char::is_whitespace) {
            if let Some(iface) = parse_header(line) {
                interfaces.push(iface);
            }
            continue;
        }
        let Some(iface) = interfaces.last_mut() else {
            continue;
        };
        parse_attribute(iface, line.trim());
    }
    interfaces
}

fn parse_header(line: &str) -> Option<IfconfigInterface> {
    let (name, rest) = line.split_once(": ")?;
    let flags = rest
        .find('<')
        .and_then(|s| rest[s + 1..].find('>').map(|e| &rest[s + 1..s + 1 + e]))
        .map(|inner| {
            inner
                .split(',')
                .filter(|f| !f.is_empty())
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default();
    let mtu = rest
        .split_whitespace()
        .skip_while(|t| *t != "mtu")
        .nth(1)
        .and_then(|v| v.parse().ok());
    Some(IfconfigInterface {
        name: name.to_string(),
        flags,
        mtu,
        ..Default::default()
    })
}

fn parse_attribute(iface: &mut IfconfigInterface, line: &str) {
    let tokens: Vec<&str> = line.split_whitespace().collect();
    match tokens.first().copied() {
        Some("ether") => iface.mac = tokens.get(1).unwrap_or(&"").to_string(),
        Some("inet") => iface.inet.push(parse_inet(&tokens)),
        Some("inet6") => iface.inet6.push(parse_inet6(&tokens)),
        Some("status:") => iface.status = tokens[1..].join(" "),
        Some("media:") => iface.media = tokens[1..].join(" "),
        _ => {}
    }
}

/// `inet 192.168.1.2 netmask 0xffffff00 broadcast 192.168.1.255`
/// または `inet 10.5.0.2 --> 10.5.0.2 netmask 0xffff0000`
fn parse_inet(tokens: &[&str]) -> IfconfigInet {
    let mut inet = IfconfigInet {
        address: tokens.get(1).unwrap_or(&"").to_string(),
        ..Default::default()
    };
    let mut i = 2;
    while i < tokens.len() {
        let value = tokens.get(i + 1).copied().unwrap_or("");
        match tokens[i] {
            "-->" => inet.destination = value.to_string(),
            "netmask" => {
                if let Some(mask) = parse_hex_netmask(value) {
                    inet.netmask = format!(
                        "{}.{}.{}.{}",
                        mask >> 24,
                        (mask >> 16) & 0xff,
                        (mask >> 8) & 0xff,
                        mask & 0xff
                    );
                    inet.prefix_len = Some(mask.count_ones());
                } else {
                    inet.netmask = value.to_string();
                }
            }
            "broadcast" => inet.broadcast = value.to_string(),
            _ => {
                i += 1;
                continue;
            }
        }
        i += 2;
    }
    inet
}

fn parse_hex_netmask(value: &str) -> Option<u32> {
    u32::from_str_radix(value.strip_prefix("0x")?, 16).ok()
}

/// `inet6 fe80::1%lo0 prefixlen 64 secured scopeid 0x1`
fn parse_inet6(tokens: &[&str]) -> IfconfigInet6 {
    let address = tokens.get(1).unwrap_or(&"").to_string();
    let mut inet6 = IfconfigInet6 {
        link_local: address.to_ascii_lowercase().starts_with("fe80:"),
        address,
        ..Default::default()
    };
    let mut i = 2;
    while i < tokens.len() {
        match tokens[i] {
            "prefixlen" => {
                inet6.prefix_len = tokens.get(i + 1).and_then(|v| v.parse().ok());
                i += 2;
            }
            // scopeid の値は表示しない
            "scopeid" => i += 2,
            other => {
                inet6.attributes.push(other.to_string());
                i += 1;
            }
        }
    }
    inet6
}

#[cfg(test)]
mod tests {
    use super::{Ifconfig, IfconfigQuery};

    const OUTPUT: &str = "\
lo0: flags=8049<UP,LOOPBACK,RUNNING,MULTICAST> mtu 16384
\toptions=1203<RXCSUM,TXCSUM,TXSTATUS,SW_TIMESTAMP>
\tinet 127.0.0.1 netmask 0xff000000
\tinet6 ::1 prefixlen 128
\tinet6 fe80::1%lo0 prefixlen 64 scopeid 0x1
stf0: flags=0<> mtu 1280
en0: flags=8863<UP,BROADCAST,SMART,RUNNING,SIMPLEX,MULTICAST> mtu 1500
\tether ea:cf:16:9c:02:5d
\tinet 192.168.40.223 netmask 0xffffff00 broadcast 192.168.40.255
\tinet6 2001:db8::1 prefixlen 64 autoconf secured
\tmedia: autoselect
\tstatus: active
bridge0: flags=8863<UP,BROADCAST> mtu 1500
\tConfiguration:
\t\tid 0:0:0:0:0:0 priority 0 hellotime 0 fwddelay 0
\tmember: en1 flags=3<LEARNING,DISCOVER>
\tstatus: inactive
utun4: flags=8051<UP,POINTOPOINT,RUNNING,MULTICAST> mtu 1280
\tinet 10.5.0.2 --> 10.5.0.2 netmask 0xffff0000
";

    fn query(interface: &str, up_only: bool, family: &str) -> IfconfigQuery {
        IfconfigQuery {
            interface: interface.to_string(),
            up_only,
            family: family.to_string(),
        }
    }

    #[test]
    fn previews_queries() {
        assert_eq!(Ifconfig::new(query("", false, "")).unwrap().preview(), "ifconfig -a");
        assert_eq!(
            Ifconfig::new(query("", true, "inet")).unwrap().preview(),
            "ifconfig -a -u inet"
        );
        // 個別指定では -u は使わない
        assert_eq!(
            Ifconfig::new(query("en0", true, "inet6")).unwrap().preview(),
            "ifconfig en0 inet6"
        );
    }

    #[test]
    fn rejects_bad_input() {
        for name in ["-a", "en0 down", "EN0", "en0;ls", "a1b2", "en123456"] {
            assert!(Ifconfig::new(query(name, false, "")).is_err(), "{name}");
        }
        assert!(Ifconfig::new(query("", false, "link")).is_err());
    }

    #[test]
    fn parses_interfaces() {
        let ifaces = super::parse_interfaces(OUTPUT);
        assert_eq!(ifaces.len(), 5);

        let lo = &ifaces[0];
        assert_eq!(lo.name, "lo0");
        assert_eq!(lo.mtu, Some(16384));
        assert!(lo.flags.contains(&"LOOPBACK".to_string()));
        assert_eq!(lo.inet[0].netmask, "255.0.0.0");
        assert_eq!(lo.inet[0].prefix_len, Some(8));
        assert_eq!(lo.inet6.len(), 2);
        assert!(lo.inet6[1].link_local);
        assert!(lo.inet6[1].attributes.is_empty());

        assert!(ifaces[1].flags.is_empty());

        let en0 = &ifaces[2];
        assert_eq!(en0.mac, "ea:cf:16:9c:02:5d");
        assert_eq!(en0.inet[0].address, "192.168.40.223");
        assert_eq!(en0.inet[0].prefix_len, Some(24));
        assert_eq!(en0.inet[0].broadcast, "192.168.40.255");
        assert_eq!(en0.inet6[0].attributes, vec!["autoconf", "secured"]);
        assert_eq!(en0.status, "active");
        assert_eq!(en0.media, "autoselect");

        // bridge の Configuration / member 行は無視される
        assert_eq!(ifaces[3].status, "inactive");
        assert!(ifaces[3].inet.is_empty());

        assert_eq!(ifaces[4].inet[0].destination, "10.5.0.2");
        assert_eq!(ifaces[4].inet[0].prefix_len, Some(16));
    }

    #[test]
    fn runs_ifconfig_lo0() {
        let result = super::get_snapshot(query("lo0", false, "inet")).expect("ifconfig");
        assert!(result.success, "stderr: {}", result.stderr);
        assert_eq!(result.interfaces.len(), 1);
        assert_eq!(result.interfaces[0].inet[0].address, "127.0.0.1");
    }
}
