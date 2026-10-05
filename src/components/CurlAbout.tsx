import { useTranslation } from "react-i18next";

export function CurlAbout() {
  const { t } = useTranslation();

  return (
    <section className="about-body">
      <h2>{t("about.title")}</h2>
      <h3>{t("curlAbout.heading")}</h3>
      <p className="muted">{t("curlAbout.body")}</p>
      <h3>{t("curlAbout.usageTitle")}</h3>
      <p className="muted">{t("curlAbout.usage")}</p>
      <h3>{t("curlAbout.safetyTitle")}</h3>
      <p className="muted">{t("curlAbout.safety")}</p>
    </section>
  );
}
