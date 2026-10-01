//! `man <topic>` の定義と、マニュアル本文の構造化。
//! 管理者権限は不要。
//!
//! 整形済みテキスト (`man` の折り返し) は幅に依存して割りにくいので、
//! `man -w` でソースを見つけ、`mandoc -T markdown` の出力を
//! 見出し・段落・強調に分けて返す。Frontend はセクション UI で表示する。

use serde::Serialize;

use crate::privileged;

/// セクション本文の塊。`paragraph` は通常の段落、`quote` はオプション説明などの引用。
/// Frontend の `ManBlock` とフィールドを一致させること。
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct ManBlock {
    pub kind: String,
    pub inlines: Vec<ManInline>,
}

/// 強調などのインライン要素。`text` / `code` は `text` を使い、
/// `bold` / `italic` は `children` を使う。
/// Frontend の `ManInline` とフィールドを一致させること。
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct ManInline {
    pub kind: String,
    pub text: String,
    pub children: Vec<ManInline>,
}

/// マニュアルの1セクション (`NAME` や `DESCRIPTION` など)。
/// Frontend の `ManSection` とフィールドを一致させること。
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct ManSection {
    pub title: String,
    pub blocks: Vec<ManBlock>,
}

/// `get_manpage` の返却値。Frontend の `ManpageDocument` とフィールドを一致させること。
#[derive(Debug, Clone, Serialize)]
pub struct ManpageDocument {
    pub success: bool,
    pub exit_code: i32,
    pub command: String,
    pub title: String,
    pub sections: Vec<ManSection>,
    pub stderr: String,
}

/// `man <topic>`
pub struct Manpage {
    pub topic: String,
}

impl Manpage {
    pub fn new(topic: &str) -> Result<Self, String> {
        let topic = topic.trim().to_string();
        validate_topic(&topic)?;
        Ok(Self { topic })
    }

    pub fn preview(&self) -> String {
        format!("man {}", self.topic)
    }

    pub fn run(&self) -> Result<ManpageDocument, String> {
        let preview = self.preview();
        let located =
            privileged::execute_plain("/usr/bin/man", &["-w", &self.topic], preview.clone())?;
        if !located.success {
            let message = prefer_output(&located.stderr, &located.stdout);
            return Ok(failed(preview, located.exit_code, message));
        }
        let path = first_path(&located.stdout)?;
        let formatted = privileged::execute_plain(
            "/usr/bin/mandoc",
            &["-T", "markdown", path],
            preview.clone(),
        )?;
        let parsed = parse_markdown(&formatted.stdout);
        let stderr = formatted.stderr.trim().to_string();
        Ok(ManpageDocument {
            success: formatted.success && !parsed.sections.is_empty(),
            exit_code: formatted.exit_code,
            command: preview,
            title: parsed.title,
            sections: parsed.sections,
            stderr,
        })
    }
}

fn failed(command: String, exit_code: i32, stderr: String) -> ManpageDocument {
    ManpageDocument {
        success: false,
        exit_code,
        command,
        title: String::new(),
        sections: Vec::new(),
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
    "マニュアルが見つかりません".to_string()
}

/// `man -w` の1行目をソースパスとして使う。引数として渡すだけなのでシェル展開はしない。
fn first_path(stdout: &str) -> Result<&str, String> {
    let path = stdout
        .lines()
        .map(str::trim)
        .find(|line| !line.is_empty())
        .ok_or_else(|| "マニュアルのパスを取得できませんでした".to_string())?;
    if !path.starts_with('/') || path.chars().any(char::is_control) {
        return Err(format!("マニュアルのパスが不正です: {path}"));
    }
    Ok(path)
}

/// man 自体のオプション解釈を防ぐため、トピック名を制限する。
/// 先頭 `-` 禁止・英数字と `_-.+` のみ許可。
fn validate_topic(topic: &str) -> Result<(), String> {
    if topic.is_empty() {
        return Err("トピック名を入力してください".to_string());
    }
    let valid = !topic.starts_with('-')
        && topic
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '-' | '.' | '+'));
    if !valid {
        return Err(format!("トピック名が不正です: {topic}"));
    }
    Ok(())
}

struct ParsedMarkdown {
    title: String,
    sections: Vec<ManSection>,
}

/// `mandoc -T markdown` の出力を見出し単位のセクションに分ける。
fn parse_markdown(md: &str) -> ParsedMarkdown {
    let lines: Vec<&str> = md.lines().collect();
    let mut index = 0;
    let mut title_parts = Vec::new();
    while index < lines.len() && !is_heading(lines[index]) {
        let trimmed = lines[index].trim();
        if !trimmed.is_empty() {
            title_parts.push(trimmed);
        }
        index += 1;
    }
    let mut sections = Vec::new();
    while index < lines.len() {
        let title = lines[index].trim_start_matches('#').trim().to_string();
        index += 1;
        let start = index;
        while index < lines.len() && !is_heading(lines[index]) {
            index += 1;
        }
        let blocks = parse_blocks(&lines[start..index]);
        if !title.is_empty() && !blocks.is_empty() {
            sections.push(ManSection { title, blocks });
        }
    }
    ParsedMarkdown {
        title: title_parts.join(" "),
        sections,
    }
}

fn is_heading(line: &str) -> bool {
    line.starts_with("# ")
}

fn parse_blocks(lines: &[&str]) -> Vec<ManBlock> {
    let mut blocks = Vec::new();
    let mut buf: Vec<String> = Vec::new();
    let mut kind: Option<&str> = None;
    let mut hard_break = false;

    let flush = |kind: &mut Option<&str>, buf: &mut Vec<String>, blocks: &mut Vec<ManBlock>| {
        let Some(kind_name) = kind.take() else {
            buf.clear();
            return;
        };
        let text = buf.join(" ");
        buf.clear();
        if text.trim().is_empty() {
            return;
        }
        let inlines = parse_inlines(&text);
        if !inlines.is_empty() {
            blocks.push(ManBlock {
                kind: kind_name.to_string(),
                inlines,
            });
        }
    };

    for line in lines {
        if line.trim().is_empty() {
            flush(&mut kind, &mut buf, &mut blocks);
            hard_break = false;
            continue;
        }
        let (content, hard) = split_hard(line);
        let (next_kind, text) = match quote_body(&content) {
            Some(body) => ("quote", body),
            None => ("paragraph", content),
        };
        if text.is_empty() {
            flush(&mut kind, &mut buf, &mut blocks);
            hard_break = false;
            continue;
        }
        if kind != Some(next_kind) || hard_break {
            flush(&mut kind, &mut buf, &mut blocks);
        }
        kind = Some(next_kind);
        buf.push(text);
        hard_break = hard;
    }
    flush(&mut kind, &mut buf, &mut blocks);
    blocks
}

/// 末尾の連続スペース2つは markdown の強制改行。
fn split_hard(line: &str) -> (String, bool) {
    let hard = line.ends_with("  ");
    (line.trim_end().to_string(), hard)
}

fn quote_body(content: &str) -> Option<String> {
    if let Some(rest) = content.strip_prefix("> ") {
        return Some(rest.to_string());
    }
    if content == ">" {
        return Some(String::new());
    }
    None
}

fn parse_inlines(input: &str) -> Vec<ManInline> {
    let decoded = decode_entities(input);
    let chars: Vec<char> = decoded.chars().collect();
    let mut index = 0;
    let mut out = Vec::new();
    parse_seq(&chars, &mut index, &mut out, Stop::None);
    out
}

#[derive(Clone, Copy, PartialEq)]
enum Stop {
    None,
    Bold,
    Italic,
}

fn parse_seq(chars: &[char], index: &mut usize, out: &mut Vec<ManInline>, stop: Stop) {
    let mut buf = String::new();
    while *index < chars.len() {
        if stop == Stop::Bold && starts_with(chars, *index, &['*', '*']) {
            flush_text(&mut buf, out);
            return;
        }
        if stop == Stop::Italic && chars[*index] == '*' && !starts_with(chars, *index, &['*', '*'])
        {
            flush_text(&mut buf, out);
            return;
        }
        if chars[*index] == '\\' && *index + 1 < chars.len() {
            buf.push(chars[*index + 1]);
            *index += 2;
            continue;
        }
        if chars[*index] == '`' {
            flush_text(&mut buf, out);
            *index += 1;
            let mut code = String::new();
            while *index < chars.len() && chars[*index] != '`' {
                if chars[*index] == '\\' && *index + 1 < chars.len() {
                    code.push(chars[*index + 1]);
                    *index += 2;
                } else {
                    code.push(chars[*index]);
                    *index += 1;
                }
            }
            if *index < chars.len() && chars[*index] == '`' {
                *index += 1;
            }
            if !code.is_empty() {
                out.push(leaf("code", code));
            }
            continue;
        }
        if starts_with(chars, *index, &['*', '*']) {
            flush_text(&mut buf, out);
            *index += 2;
            let mut children = Vec::new();
            parse_seq(chars, index, &mut children, Stop::Bold);
            if starts_with(chars, *index, &['*', '*']) {
                *index += 2;
                push_wrapper(out, "bold", children);
            } else {
                out.push(leaf("text", "**".to_string()));
                out.append(&mut children);
            }
            continue;
        }
        if chars[*index] == '*' {
            flush_text(&mut buf, out);
            *index += 1;
            let mut children = Vec::new();
            parse_seq(chars, index, &mut children, Stop::Italic);
            if *index < chars.len()
                && chars[*index] == '*'
                && !starts_with(chars, *index, &['*', '*'])
            {
                *index += 1;
                push_wrapper(out, "italic", children);
            } else {
                out.push(leaf("text", "*".to_string()));
                out.append(&mut children);
            }
            continue;
        }
        buf.push(chars[*index]);
        *index += 1;
    }
    flush_text(&mut buf, out);
}

fn starts_with(chars: &[char], index: usize, marker: &[char]) -> bool {
    chars[index..].starts_with(marker)
}

fn flush_text(buf: &mut String, out: &mut Vec<ManInline>) {
    if !buf.is_empty() {
        out.push(leaf("text", std::mem::take(buf)));
    }
}

fn push_wrapper(out: &mut Vec<ManInline>, kind: &str, children: Vec<ManInline>) {
    if children.is_empty() {
        return;
    }
    out.push(ManInline {
        kind: kind.to_string(),
        text: String::new(),
        children,
    });
}

fn leaf(kind: &str, text: String) -> ManInline {
    ManInline {
        kind: kind.to_string(),
        text,
        children: Vec::new(),
    }
}

fn decode_entities(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut rest = input;
    while let Some(start) = rest.find('&') {
        out.push_str(&rest[..start]);
        rest = &rest[start..];
        let Some(end) = rest.find(';') else {
            out.push_str(rest);
            return out;
        };
        if let Some(ch) = decode_entity(&rest[1..end]) {
            out.push(ch);
            rest = &rest[end + 1..];
        } else {
            out.push('&');
            rest = &rest[1..];
        }
    }
    out.push_str(rest);
    out
}

fn decode_entity(entity: &str) -> Option<char> {
    match entity {
        "nbsp" => Some(' '),
        "lt" => Some('<'),
        "gt" => Some('>'),
        "amp" => Some('&'),
        "quot" => Some('"'),
        "apos" => Some('\''),
        _ => {
            let code = if let Some(hex) = entity
                .strip_prefix("#x")
                .or_else(|| entity.strip_prefix("#X"))
            {
                u32::from_str_radix(hex, 16).ok()?
            } else {
                let digits = entity.strip_prefix('#')?;
                digits.parse::<u32>().ok()?
            };
            char::from_u32(code)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_markdown, validate_topic, Manpage};

    fn text(value: &str) -> super::ManInline {
        super::leaf("text", value.to_string())
    }

    #[test]
    fn previews_man_command() {
        assert_eq!(Manpage::new("pmset").unwrap().preview(), "man pmset");
    }

    #[test]
    fn rejects_option_like_topics() {
        assert!(Manpage::new("-l").is_err());
        assert!(Manpage::new("").is_err());
        assert!(Manpage::new("   ").is_err());
        assert!(Manpage::new("a;rm -rf /").is_err());
        assert!(Manpage::new("pmset").is_ok());
        assert!(Manpage::new("caffeinate").is_ok());
        assert!(validate_topic("launchctl").is_ok());
    }

    #[test]
    fn parses_sections_markup_and_hard_breaks() {
        let md = "\
PMSET(1) - General Commands Manual

# NAME

**pmset** - manipulate power management settings

# SYNOPSIS

**pmset**
\\[**-a**&nbsp;|&nbsp;**-b**]
\\[...]  

**pmset**
**-g**

# DESCRIPTION

The following options are available:

**-a**

> Display the
> *value* (&#8220;POSIX&#8221;).

# SEE ALSO

caffeinate(8)
";
        let parsed = parse_markdown(md);
        assert_eq!(parsed.title, "PMSET(1) - General Commands Manual");
        let titles: Vec<_> = parsed.sections.iter().map(|s| s.title.as_str()).collect();
        assert_eq!(titles, ["NAME", "SYNOPSIS", "DESCRIPTION", "SEE ALSO"]);

        let name = &parsed.sections[0].blocks[0].inlines;
        assert_eq!(name[0].kind, "bold");
        assert_eq!(name[0].children, vec![text("pmset")]);
        assert_eq!(name[1], text(" - manipulate power management settings"));

        let synopsis = &parsed.sections[1].blocks;
        assert_eq!(synopsis.len(), 2);
        assert_eq!(synopsis[0].kind, "paragraph");
        assert_eq!(synopsis[0].inlines[0].children, vec![text("pmset")]);
        assert!(
            synopsis[0]
                .inlines
                .iter()
                .any(|span| span.text.contains('[')),
            "escaped brackets should render as brackets"
        );
        assert!(synopsis[0].inlines.iter().any(|span| {
            span.kind == "bold" && span.children == vec![text("-a")]
        }));
        assert_eq!(synopsis[1].inlines[0].children, vec![text("pmset")]);

        let description = &parsed.sections[2].blocks;
        assert_eq!(description[1].inlines[0].children, vec![text("-a")]);
        let quote = &description[2];
        assert_eq!(quote.kind, "quote");
        assert_eq!(quote.inlines[0], text("Display the "));
        assert_eq!(quote.inlines[1].kind, "italic");
        assert_eq!(quote.inlines[1].children, vec![text("value")]);
        assert!(quote
            .inlines
            .iter()
            .any(|span| span.text.contains("“POSIX”")));
    }

    #[test]
    fn parses_live_pmset_page() {
        let located = std::process::Command::new("/usr/bin/man")
            .args(["-w", "pmset"])
            .output()
            .expect("man");
        assert!(located.status.success(), "pmset のマニュアルが見つかりません");
        let path = String::from_utf8_lossy(&located.stdout);
        let path = path.lines().next().unwrap().trim();
        let formatted = std::process::Command::new("/usr/bin/mandoc")
            .args(["-T", "markdown", path])
            .output()
            .expect("mandoc");
        let parsed = parse_markdown(&String::from_utf8_lossy(&formatted.stdout));
        let titles: Vec<_> = parsed.sections.iter().map(|s| s.title.as_str()).collect();
        assert!(titles.contains(&"NAME"));
        assert!(titles.contains(&"SYNOPSIS"));
        assert!(titles.contains(&"DESCRIPTION"));
        assert!(!parsed.title.is_empty());
    }
}
