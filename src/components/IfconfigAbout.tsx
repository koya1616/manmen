import { useTranslation } from "react-i18next";

export function IfconfigAbout() {
  const { t } = useTranslation();

  return (
    <section className="about-body">
      <h2>{t("about.title")}</h2>
      <h3>{t("ifconfigAbout.heading")}</h3>
      <p className="muted">{t("ifconfigAbout.body")}</p>
      <h3>{t("ifconfigAbout.usageTitle")}</h3>
      <p className="muted">{t("ifconfigAbout.usage")}</p>
    </section>
  );
}
