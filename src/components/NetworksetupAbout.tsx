import { useTranslation } from "react-i18next";

export function NetworksetupAbout() {
  const { t } = useTranslation();

  return (
    <section className="about-body">
      <h2>{t("about.title")}</h2>
      <h3>{t("networksetupAbout.heading")}</h3>
      <p className="muted">{t("networksetupAbout.body")}</p>
      <h3>{t("networksetupAbout.usageTitle")}</h3>
      <p className="muted">{t("networksetupAbout.usage")}</p>
    </section>
  );
}
