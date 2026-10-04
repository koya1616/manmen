import { useTranslation } from "react-i18next";

export function GitAbout() {
  const { t } = useTranslation();

  return (
    <section className="about-body">
      <h2>{t("about.title")}</h2>
      <h3>{t("gitAbout.heading")}</h3>
      <p className="muted">{t("gitAbout.body")}</p>
      <h3>{t("gitAbout.usageTitle")}</h3>
      <p className="muted">{t("gitAbout.usage")}</p>
    </section>
  );
}
