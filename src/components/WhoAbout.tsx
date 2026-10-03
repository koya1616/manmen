import { useTranslation } from "react-i18next";

export function WhoAbout() {
  const { t } = useTranslation();

  return (
    <section className="about-body">
      <h2>{t("about.title")}</h2>
      <h3>{t("whoAbout.heading")}</h3>
      <p className="muted">{t("whoAbout.body")}</p>
      <h3>{t("whoAbout.usageTitle")}</h3>
      <p className="muted">{t("whoAbout.usage")}</p>
    </section>
  );
}
