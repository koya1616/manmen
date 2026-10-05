import type { ReactNode } from "react";
import { useTranslation } from "react-i18next";
import {
  CURL_MAX_CONNECT_TIMEOUT,
  CURL_MAX_HEADERS,
  CURL_MAX_MAX_REDIRS,
  CURL_MAX_MAX_TIME,
  CURL_MIN_MAX_TIME,
  type CurlHttpVersion,
  type CurlIpVersion,
} from "../commands/curlOptions";
import { useCurl } from "../hooks/useCurl";
import { CurlResultView } from "./CurlResultView";
import { RunnerOutput } from "./ui/RunnerOutput";
import { Workbench } from "./ui/Workbench";
import { FlagRow, OptionRow, Segmented, Stepper, TextField } from "./ui/controls";

export function CurlCard({ active, about }: { active: boolean; about: ReactNode }) {
  const { t } = useTranslation();
  const form = useCurl();
  const { runner } = form;

  const blocker = !form.urlOk
    ? t("curl.urlInvalid")
    : !form.headersOk
      ? t("curl.headerInvalid")
      : !form.maxRedirsOk
        ? t("curl.maxRedirsInvalid")
        : !form.maxTimeOk
          ? t("curl.maxTimeInvalid")
          : !form.connectTimeoutOk
            ? t("curl.connectTimeoutInvalid")
            : null;

  return (
    <Workbench
      active={active}
      title={t("curl.title")}
      description={t("curl.description")}
      tokens={form.tokens}
      onRun={form.execute}
      canRun={form.valid}
      running={runner.running}
      runLabel={t("curl.execute")}
      runningLabel={t("curl.executing")}
      blocker={blocker}
      about={about}
      options={
        <>
          <div className="group-label">{t("curl.groupRequest")}</div>
          <OptionRow
            id="url"
            label={t("curl.url")}
            on={form.url.trim() !== ""}
            hint={t("curl.hintUrl")}
            error={form.urlOk ? null : t("curl.urlInvalid")}
          >
            <TextField
              value={form.url}
              onChange={form.setUrl}
              placeholder={t("curl.urlPlaceholder")}
              invalid={!form.urlOk}
              onClear={() => form.setUrl("")}
            />
          </OptionRow>
          <OptionRow id="method" label={t("curl.method")} on hint={t("curl.hintMethod")}>
            <Segmented
              value={form.head ? "HEAD" : "GET"}
              onChange={(next) => form.setHead(next === "HEAD")}
              options={[
                { value: "GET", label: "GET", sub: "-i" },
                { value: "HEAD", label: "HEAD", sub: "-I" },
              ]}
            />
          </OptionRow>
          <OptionRow
            id="headers"
            label={t("curl.headers")}
            flag="-H"
            on={form.headers.some((h) => h.trim() !== "")}
            hint={t("curl.hintHeaders")}
            error={form.headersOk ? null : t("curl.headerInvalid")}
          >
            <div className="stepper-wrap">
              {form.headers.map((header, index) => (
                <TextField
                  key={index}
                  value={header}
                  onChange={(next) => form.setHeaderAt(index, next)}
                  placeholder={t("curl.headerPlaceholder")}
                  invalid={!form.headerOks[index]}
                  onClear={() => form.removeHeader(index)}
                />
              ))}
              {form.headers.length < CURL_MAX_HEADERS ? (
                <div>
                  <button type="button" className="link" onClick={form.addHeader}>
                    {t("curl.addHeader")}
                  </button>
                </div>
              ) : null}
            </div>
          </OptionRow>

          <div className="group-label">{t("curl.groupRedirect")}</div>
          <FlagRow
            id="follow"
            label={t("curlFlags.follow")}
            flag="-L"
            hint={t("curl.hintFollow")}
            checked={form.follow}
            onChange={form.setFollow}
          />
          {form.follow ? (
            <OptionRow
              id="maxRedirs"
              label={t("curl.maxRedirs")}
              flag="--max-redirs"
              on
              hint={t("curl.hintMaxRedirs")}
              error={form.maxRedirsOk ? null : t("curl.maxRedirsInvalid")}
            >
              <Stepper
                value={form.maxRedirs}
                onChange={form.setMaxRedirs}
                min={0}
                max={CURL_MAX_MAX_REDIRS}
                step={1}
              />
            </OptionRow>
          ) : null}

          <div className="group-label">{t("curl.groupTiming")}</div>
          <OptionRow
            id="maxTime"
            label={t("curl.maxTime")}
            flag="-m"
            on
            hint={t("curl.hintMaxTime")}
            error={form.maxTimeOk ? null : t("curl.maxTimeInvalid")}
          >
            <Stepper
              value={form.maxTime}
              onChange={form.setMaxTime}
              min={CURL_MIN_MAX_TIME}
              max={CURL_MAX_MAX_TIME}
              step={5}
              unit={t("curl.seconds")}
            />
          </OptionRow>
          <OptionRow
            id="connectTimeout"
            label={t("curl.connectTimeout")}
            flag="--connect-timeout"
            on={form.connectTimeout.trim() !== ""}
            hint={t("curl.hintConnectTimeout")}
            error={form.connectTimeoutOk ? null : t("curl.connectTimeoutInvalid")}
          >
            <TextField
              value={form.connectTimeout}
              onChange={form.setConnectTimeout}
              placeholder={t("curl.connectTimeoutPlaceholder", { max: CURL_MAX_CONNECT_TIMEOUT })}
              invalid={!form.connectTimeoutOk}
              onClear={() => form.setConnectTimeout("")}
            />
          </OptionRow>

          <div className="group-label">{t("curl.groupConnection")}</div>
          <OptionRow
            id="httpVersion"
            label={t("curl.httpVersion")}
            on={form.httpVersion !== ""}
            hint={t("curl.hintHttpVersion")}
          >
            <Segmented
              value={form.httpVersion}
              onChange={(next) => form.setHttpVersion(next as CurlHttpVersion)}
              options={[
                { value: "", label: t("curl.auto") },
                { value: "1.1", label: "HTTP/1.1", sub: "--http1.1" },
                { value: "2", label: "HTTP/2", sub: "--http2" },
              ]}
            />
          </OptionRow>
          <OptionRow
            id="ipVersion"
            label={t("curl.ipVersion")}
            on={form.ipVersion !== ""}
            hint={t("curl.hintIpVersion")}
          >
            <Segmented
              value={form.ipVersion}
              onChange={(next) => form.setIpVersion(next as CurlIpVersion)}
              options={[
                { value: "", label: t("curl.auto") },
                { value: "4", label: "IPv4", sub: "-4" },
                { value: "6", label: "IPv6", sub: "-6" },
              ]}
            />
          </OptionRow>
          <FlagRow
            id="compressed"
            label={t("curlFlags.compressed")}
            flag="--compressed"
            hint={t("curl.hintCompressed")}
            checked={form.compressed}
            onChange={form.setCompressed}
          />
          <FlagRow
            id="insecure"
            label={t("curlFlags.insecure")}
            flag="-k"
            hint={t("curl.hintInsecure")}
            checked={form.insecure}
            onChange={form.setInsecure}
          />
          <p className="note">{t("curl.note")}</p>
        </>
      }
      output={
        <RunnerOutput
          runner={runner}
          preview={form.preview}
          emptyHint={t("curl.empty")}
          result={runner.result}
        >
          {runner.result ? <CurlResultView result={runner.result} /> : null}
        </RunnerOutput>
      }
    />
  );
}
