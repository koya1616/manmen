import { useTranslation } from "react-i18next";
import { useDocker, type DockerReadKind } from "../hooks/useDocker";
import { CommandResultView } from "./CommandResultView";
import { DockerDuResultView } from "./DockerDuResultView";
import { DockerInspectResultView } from "./DockerInspectResultView";
import { DockerLsResultView } from "./DockerLsResultView";
import { DockerVersionResultView } from "./DockerVersionResultView";

const READ_KINDS: DockerReadKind[] = ["du", "ls", "inspect", "version"];

export function DockerCard() {
  const { t } = useTranslation();
  const {
    force,
    setForce,
    all,
    setAll,
    name,
    setName,
    nameOk,
    executing,
    dockerResult,
    error,
    preview,
    inspectPreview,
    execute,
    executeRead,
  } = useDocker();

  return (
    <>
      <section className="card">
        <h2>{t("docker.title")}</h2>
        <p className="muted">{t("docker.description")}</p>

        <h3>{t("docker.readTitle")}</h3>
        <div className="choice-row">
          {READ_KINDS.filter((kind) => kind !== "inspect").map((kind) => (
            <button
              key={kind}
              type="button"
              className="btn-secondary"
              onClick={() => executeRead(kind)}
              disabled={executing}
            >
              {t(`dockerRead.${kind}`)}
            </button>
          ))}
        </div>

        <label className="field">
          <span className="field-label">{t("docker.name")}</span>
          <input
            type="text"
            value={name}
            placeholder={t("docker.namePlaceholder")}
            onChange={(e) => setName(e.target.value)}
          />
          <span className="field-hint">{t("docker.hintName")}</span>
        </label>
        {!nameOk ? <p className="muted">{t("docker.nameInvalid")}</p> : null}

        <div className="topic-row">
          <button
            type="button"
            className="btn-secondary"
            onClick={() => executeRead("inspect")}
            disabled={executing || !nameOk}
          >
            {executing ? t("docker.readExecuting") : t("dockerRead.inspect")}
          </button>
        </div>

        <div className="preview">
          <span className="preview-label">{t("docker.preview")}</span>
          <code>{inspectPreview}</code>
        </div>
      </section>

      <details className="card">
        <summary>{t("docker.pruneTitle")}</summary>
        <p className="muted">{t("docker.pruneSummary")}</p>
        <div className="choice-stack">
          <div className="flag-item">
            <label className="check-row">
              <input
                type="checkbox"
                checked={force}
                onChange={(e) => setForce(e.target.checked)}
              />
              <span>{t("dockerFlags.force")}</span>
            </label>
            <span className="field-hint">{t("docker.hintForce")}</span>
          </div>
          <div className="flag-item">
            <label className="check-row">
              <input
                type="checkbox"
                checked={all}
                onChange={(e) => setAll(e.target.checked)}
              />
              <span>{t("dockerFlags.all")}</span>
            </label>
            <span className="field-hint">{t("docker.hintAll")}</span>
          </div>
        </div>

        <button className="btn-primary" onClick={execute} disabled={executing}>
          {executing ? t("docker.executing") : t("docker.execute")}
        </button>

        <div className="preview">
          <span className="preview-label">{t("docker.preview")}</span>
          <code>{preview}</code>
        </div>

        <p className="muted">{t("docker.note")}</p>
      </details>

      {dockerResult?.kind === "du" ? (
        <DockerDuResultView result={dockerResult.result} error={error} />
      ) : null}
      {dockerResult?.kind === "ls" ? (
        <DockerLsResultView result={dockerResult.result} error={error} />
      ) : null}
      {dockerResult?.kind === "version" ? (
        <DockerVersionResultView result={dockerResult.result} error={error} />
      ) : null}
      {dockerResult?.kind === "inspect" ? (
        <DockerInspectResultView result={dockerResult.result} error={error} />
      ) : null}
      {dockerResult?.kind === "prune" ? (
        <CommandResultView result={dockerResult.result} error={error} />
      ) : null}
      {!dockerResult && error ? (
        <CommandResultView result={null} error={error} />
      ) : null}
    </>
  );
}
