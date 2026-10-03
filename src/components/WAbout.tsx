import { useTranslation } from "react-i18next";

export function WAbout() {
  const { t } = useTranslation();

  return (
    <section className="about-body">
      <h2>{t("about.title")}</h2>
      <h3>{t("wAbout.heading")}</h3>
      <p className="muted">{t("wAbout.body")}</p>
      <h3>{t("wAbout.usageTitle")}</h3>
      <p className="muted">{t("wAbout.usage")}</p>
    </section>
  );
}
