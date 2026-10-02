import { useTranslation } from "react-i18next";

export function ManpageAbout() {
  const { t } = useTranslation();

  return (
    <section className="about-body">
      <h2>{t("manAbout.title")}</h2>
      <p className="muted">{t("manAbout.man")}</p>
      <h3>{t("manAbout.usageTitle")}</h3>
      <p className="muted">{t("manAbout.usage")}</p>
    </section>
  );
}
