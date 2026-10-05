import { useTranslation } from "react-i18next";

export function PingAbout() {
  const { t } = useTranslation();

  return (
    <section className="about-body">
      <h2>{t("about.title")}</h2>
      <h3>{t("pingAbout.heading")}</h3>
      <p className="muted">{t("pingAbout.body")}</p>
      <h3>{t("pingAbout.usageTitle")}</h3>
      <p className="muted">{t("pingAbout.usage")}</p>
    </section>
  );
}
