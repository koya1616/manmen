import { useTranslation } from "react-i18next";

export function LsofAbout() {
  const { t } = useTranslation();

  return (
    <section className="about-body">
      <h2>{t("lsofAbout.title")}</h2>
      <p className="muted">{t("lsofAbout.lsof")}</p>
      <h3>{t("lsofAbout.usageTitle")}</h3>
      <p className="muted">{t("lsofAbout.usage")}</p>
    </section>
  );
}
