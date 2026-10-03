import { useTranslation } from "react-i18next";

export function SshAbout() {
  const { t } = useTranslation();

  return (
    <section className="about-body">
      <h2>{t("sshAbout.title")}</h2>
      <p className="muted">{t("sshAbout.ssh")}</p>
      <h3>{t("sshAbout.usageTitle")}</h3>
      <p className="muted">{t("sshAbout.usage")}</p>
    </section>
  );
}
