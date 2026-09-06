import { useTranslation } from "react-i18next";
import type { CommandResult } from "../types";

// 全コマンド共通の実行結果表示。コマンド固有の知識は持たせない。
export function CommandResultView({
  result,
  error,
}: {
  result: CommandResult | null;
  error: string | null;
}) {
  const { t } = useTranslation();

  if (error) {
    return (
      <section className="card error">
        <h3>{t("result.failed")}</h3>
        <pre>{error}</pre>
      </section>
    );
  }

  if (!result) return null;

  return (
    <section className={`card ${result.success ? "success" : "error"}`}>
      <h3>{result.success ? t("result.completed") : t("result.failed")}</h3>
      <p className="muted">
        {t("result.exitCode")}: {result.exit_code}
      </p>
      <div className="output-block">
        <h4>{t("result.output")}</h4>
        <pre>{result.stdout || t("result.noOutput")}</pre>
      </div>
      <div className="output-block">
        <h4>{t("result.error")}</h4>
        <pre>{result.stderr || t("result.noOutput")}</pre>
      </div>
    </section>
  );
}
