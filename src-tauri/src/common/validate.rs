//! ps / top / lsof で重複していたユーザ名・PIDリスト検証の共通実装。
//! 上限はモジュールごとの MAX_PIDS に合わせるため引数で受ける。
//! エラーメッセージは従来の各モジュールと同一にすること (Frontend の表示とテストに影響するため)。

/// 空文字は「絞り込みなし」として許可する。
pub fn validate_user(user: &str) -> Result<(), String> {
    if user.is_empty() {
        return Ok(());
    }
    let ok = user.len() <= 32
        && user
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '.' | '-'))
        && !user.starts_with('-');
    if ok {
        Ok(())
    } else {
        Err(format!("ユーザ名が不正です: {user}"))
    }
}

/// `,` / 空白区切りのPIDリストをパースする。重複は除去する。
pub fn parse_pids(raw: &str, max: usize) -> Result<Vec<String>, String> {
    let mut pids = Vec::new();
    for token in raw.split(|c: char| c == ',' || c.is_whitespace()) {
        if token.is_empty() {
            continue;
        }
        if token.parse::<u32>().is_err() || token.starts_with('+') || token.starts_with('-') {
            return Err(format!("プロセスIDが不正です: {token}"));
        }
        if !pids.contains(&token.to_string()) {
            pids.push(token.to_string());
        }
    }
    if pids.len() > max {
        return Err(format!("プロセスIDは {max} 件までです"));
    }
    Ok(pids)
}
