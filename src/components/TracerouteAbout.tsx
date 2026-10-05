import { useTranslation } from "react-i18next";

export function TracerouteAbout() {
  const { t } = useTranslation();

  return (
    <section className="about-body">
      <h2>{t("about.title")}</h2>
      <h3>{t("tracerouteAbout.heading")}</h3>
      <p className="muted">{t("tracerouteAbout.body")}</p>
      <h3>{t("tracerouteAbout.usageTitle")}</h3>
      <p className="muted">{t("tracerouteAbout.usage")}</p>
    </section>
  );
}
