import { useTranslation } from "react-i18next";

function GlossaryItem({ term, text }: { term: string; text: string }) {
  return (
    <>
      <dt>
        <code>{term}</code>
      </dt>
      <dd>{text}</dd>
    </>
  );
}

export function PmsetAbout() {
  const { t } = useTranslation();

  return (
    <section className="card">
      <h2>{t("about.title")}</h2>
      <h3>{t("about.pmsetTitle")}</h3>
      <p className="muted">{t("about.pmset")}</p>
      <h3>{t("about.argsTitle")}</h3>
      <dl className="glossary">
        <GlossaryItem term="sudo" text={t("about.sudo")} />
        <GlossaryItem term="-a / -b / -c / -u" text={t("about.scopes")} />
        <GlossaryItem term="setting value" text={t("about.value")} />
        <GlossaryItem term="disablesleep" text={t("about.disablesleep")} />
      </dl>
      <h3>{t("about.onoffTitle")}</h3>
      <dl className="glossary">
        <GlossaryItem term="ON (1)" text={t("about.on")} />
        <GlossaryItem term="OFF (0)" text={t("about.off")} />
      </dl>
    </section>
  );
}
