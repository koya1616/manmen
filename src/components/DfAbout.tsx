import { useTranslation } from "react-i18next";

export function DfAbout() {
  const { t } = useTranslation();

  return (
    <section className="about-body">
      <h2>{t("about.title")}</h2>
      <h3>{t("dfAbout.heading")}</h3>
      <p className="muted">{t("dfAbout.body")}</p>
      <h3>{t("dfAbout.usageTitle")}</h3>
      <p className="muted">{t("dfAbout.usage")}</p>
    </section>
  );
}
