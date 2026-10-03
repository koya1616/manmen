import { useTranslation } from "react-i18next";

export function DigAbout() {
  const { t } = useTranslation();

  return (
    <section className="about-body">
      <h2>{t("digAbout.title")}</h2>
      <p className="muted">{t("digAbout.dig")}</p>
      <h3>{t("digAbout.usageTitle")}</h3>
      <p className="muted">{t("digAbout.usage")}</p>
    </section>
  );
}
