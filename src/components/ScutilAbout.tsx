import { useTranslation } from "react-i18next";

export function ScutilAbout() {
  const { t } = useTranslation();

  return (
    <section className="about-body">
      <h2>{t("about.title")}</h2>
      <h3>{t("scutilAbout.heading")}</h3>
      <p className="muted">{t("scutilAbout.body")}</p>
      <h3>{t("scutilAbout.usageTitle")}</h3>
      <p className="muted">{t("scutilAbout.usage")}</p>
    </section>
  );
}
