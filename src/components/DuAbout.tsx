import { useTranslation } from "react-i18next";

export function DuAbout() {
  const { t } = useTranslation();

  return (
    <section className="about-body">
      <h2>{t("about.title")}</h2>
      <h3>{t("duAbout.heading")}</h3>
      <p className="muted">{t("duAbout.body")}</p>
      <h3>{t("duAbout.usageTitle")}</h3>
      <p className="muted">{t("duAbout.usage")}</p>
    </section>
  );
}
