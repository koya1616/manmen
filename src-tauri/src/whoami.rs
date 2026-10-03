//! `whoami` の実行。管理者権限は不要。
//! 引数なしで現在のユーザ名を返すだけ。

use crate::privileged;
use crate::types::CommandResult;

/// `whoami`
pub fn current() -> Result<CommandResult, String> {
    privileged::execute_plain("/usr/bin/whoami", &[], "whoami".to_string())
}

#[cfg(test)]
mod tests {
    use super::current;

    #[test]
    fn returns_current_user() {
        let result = current().expect("whoami");
        assert!(result.success, "stderr: {}", result.stderr);
        assert_eq!(result.command, "whoami");
        assert!(!result.stdout.trim().is_empty());
    }
}
