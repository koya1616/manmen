import { useTranslation } from "react-i18next";

export function TopAbout() {
  const { t } = useTranslation();

  return (
    <section className="card">
      <h2>{t("topAbout.title")}</h2>
      <p className="muted">{t("topAbout.top")}</p>
      <h3>{t("topAbout.usageTitle")}</h3>
      <p className="muted">{t("topAbout.usage")}</p>
    </section>
  );
}
