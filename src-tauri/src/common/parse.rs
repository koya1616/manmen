//! ps / lsof で重複していた表パースの共通実装。
//! 先頭の見出し行を除き、列数に合わせて行を切る。
//! 末尾列 (ps の `command` / lsof の `NAME`) は空白を含むため原文から取り直す。

/// 先頭の見出し行を除き、列数に合わせて行を切る。
/// 末尾列は空白を含むため残り全部を1セルにする。列数に満たない行は捨てる。
pub fn parse_table(stdout: &str, width: usize) -> Vec<Vec<String>> {
    let mut lines = stdout.lines();
    // 見出し行は列名の表示に使わない (要求キーワードをそのまま返すため読み飛ばす)
    let _ = lines.next();
    lines
        .filter_map(|line| {
            if line.trim().is_empty() {
                return None;
            }
            let cells: Vec<String> = line.split_whitespace().map(str::to_string).collect();
            if cells.len() < width {
                return None;
            }
            let mut row: Vec<String> = cells.into_iter().take(width).collect();
            if row.len() == width {
                // split_whitespace では末尾列が切れてしまうので原文から取り直す
                let tail = nth_field_start(line, width);
                if let Some(tail) = tail {
                    row[width - 1] = tail.to_string();
                }
            }
            Some(row)
        })
        .collect()
}

/// 空白区切りの n 番目 (1始まり) フィールドの開始位置以降を返す。
pub fn nth_field_start(line: &str, n: usize) -> Option<&str> {
    let mut index = 0;
    let bytes = line.as_bytes();
    let mut field = 0;
    while index < bytes.len() {
        while index < bytes.len() && bytes[index].is_ascii_whitespace() {
            index += 1;
        }
        if index >= bytes.len() {
            break;
        }
        field += 1;
        if field == n {
            return Some(line[index..].trim_end());
        }
        while index < bytes.len() && !bytes[index].is_ascii_whitespace() {
            index += 1;
        }
    }
    None
}
