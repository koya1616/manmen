import { useTranslation } from "react-i18next";

export function SystemProfilerAbout() {
  const { t } = useTranslation();

  return (
    <section className="about-body">
      <h2>{t("about.title")}</h2>
      <h3>{t("spAbout.heading")}</h3>
      <p className="muted">{t("spAbout.body")}</p>
      <h3>{t("spAbout.usageTitle")}</h3>
      <p className="muted">{t("spAbout.usage")}</p>
    </section>
  );
}
