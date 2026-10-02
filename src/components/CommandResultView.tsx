import { useTranslation } from "react-i18next";
import type { CommandResult } from "../types";

// 全コマンド共通の標準出力表示。状態・終了コード・stderr は OutputPane 側で出す。
export function CommandResultView({ result }: { result: CommandResult }) {
  const { t } = useTranslation();
  return <pre className="term">{result.stdout || t("result.noOutput")}</pre>;
}
