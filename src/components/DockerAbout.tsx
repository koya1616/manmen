import { useTranslation } from "react-i18next";

export function DockerAbout() {
  const { t } = useTranslation();

  return (
    <section className="about-body">
      <h2>{t("dockerAbout.title")}</h2>
      <p className="muted">{t("dockerAbout.docker")}</p>
      <h3>{t("dockerAbout.usageTitle")}</h3>
      <p className="muted">{t("dockerAbout.usage")}</p>
    </section>
  );
}
