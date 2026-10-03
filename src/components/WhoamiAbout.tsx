import { useTranslation } from "react-i18next";

export function WhoamiAbout() {
  const { t } = useTranslation();

  return (
    <section className="about-body">
      <h2>{t("about.title")}</h2>
      <h3>{t("whoamiAbout.heading")}</h3>
      <p className="muted">{t("whoamiAbout.body")}</p>
      <h3>{t("whoamiAbout.usageTitle")}</h3>
      <p className="muted">{t("whoamiAbout.usage")}</p>
    </section>
  );
}
