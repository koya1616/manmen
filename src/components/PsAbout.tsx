import { useTranslation } from "react-i18next";

export function PsAbout() {
  const { t } = useTranslation();

  return (
    <section className="about-body">
      <h2>{t("psAbout.title")}</h2>
      <p className="muted">{t("psAbout.ps")}</p>
      <h3>{t("psAbout.usageTitle")}</h3>
      <p className="muted">{t("psAbout.usage")}</p>
    </section>
  );
}
