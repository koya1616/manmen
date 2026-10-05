import { useTranslation } from "react-i18next";

export function KillAbout() {
  const { t } = useTranslation();

  return (
    <section className="about-body">
      <h2>{t("about.title")}</h2>
      <h3>{t("killAbout.heading")}</h3>
      <p className="muted">{t("killAbout.body")}</p>
      <h3>{t("killAbout.usageTitle")}</h3>
      <p className="muted">{t("killAbout.usage")}</p>
      <h3>{t("killAbout.safetyTitle")}</h3>
      <p className="muted">{t("killAbout.safety")}</p>
    </section>
  );
}
