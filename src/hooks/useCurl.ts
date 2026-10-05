import { useState } from "react";
import { api } from "../api";
import {
  CURL_DEFAULT_MAX_REDIRS,
  CURL_DEFAULT_MAX_TIME,
  CURL_MAX_HEADERS,
  isValidCurlConnectTimeout,
  isValidCurlHeader,
  isValidCurlMaxRedirs,
  isValidCurlMaxTime,
  isValidCurlUrl,
  type CurlHttpVersion,
  type CurlIpVersion,
} from "../commands/curlOptions";
import { tok, tokensToString, type CmdToken } from "../commands/tokens";
import type { CurlSnapshot } from "../types";
import { useRunner } from "./useRunner";

export function useCurl() {
  const [url, setUrl] = useState("https://example.com");
  const [head, setHead] = useState(false);
  const [headers, setHeaders] = useState<string[]>([""]);
  const [follow, setFollow] = useState(false);
  const [maxRedirs, setMaxRedirs] = useState(String(CURL_DEFAULT_MAX_REDIRS));
  const [maxTime, setMaxTime] = useState(String(CURL_DEFAULT_MAX_TIME));
  const [connectTimeout, setConnectTimeout] = useState("");
  const [httpVersion, setHttpVersion] = useState<CurlHttpVersion>("");
  const [ipVersion, setIpVersion] = useState<CurlIpVersion>("");
  const [insecure, setInsecure] = useState(false);
  const [compressed, setCompressed] = useState(false);
  const runner = useRunner<CurlSnapshot>();

  const urlOk = isValidCurlUrl(url);
  const headerOks = headers.map(isValidCurlHeader);
  const headersOk = headerOks.every(Boolean);
  const maxRedirsOk = !follow || isValidCurlMaxRedirs(maxRedirs);
  const maxTimeOk = isValidCurlMaxTime(maxTime);
  const connectTimeoutOk = isValidCurlConnectTimeout(connectTimeout);
  const valid = urlOk && headersOk && maxRedirsOk && maxTimeOk && connectTimeoutOk;

  const headerValues = headers.map((h) => h.trim()).filter((h) => h !== "");
  const maxRedirsNum = isValidCurlMaxRedirs(maxRedirs)
    ? Number(maxRedirs.trim())
    : CURL_DEFAULT_MAX_REDIRS;
  const maxTimeNum = maxTimeOk ? Number(maxTime.trim()) : CURL_DEFAULT_MAX_TIME;
  const connectTimeoutNum = connectTimeout.trim() === "" ? null : Number(connectTimeout.trim());

  const query = {
    url: url.trim(),
    head,
    headers: headerValues,
    follow,
    maxRedirs: maxRedirsNum,
    maxTime: maxTimeNum,
    connectTimeout: connectTimeoutNum,
    httpVersion,
    ipVersion,
    insecure,
    compressed,
  };
  const tokens = buildTokens(query);
  const preview = tokensToString(tokens);

  function setHeaderAt(index: number, value: string) {
    setHeaders((prev) => prev.map((h, i) => (i === index ? value : h)));
  }

  function addHeader() {
    setHeaders((prev) => (prev.length >= CURL_MAX_HEADERS ? prev : [...prev, ""]));
  }

  function removeHeader(index: number) {
    setHeaders((prev) => (prev.length <= 1 ? [""] : prev.filter((_, i) => i !== index)));
  }

  async function execute() {
    if (!valid) return;
    await runner.run(() => api.getCurl(query), preview);
  }

  return {
    url,
    setUrl,
    head,
    setHead,
    headers,
    setHeaderAt,
    addHeader,
    removeHeader,
    follow,
    setFollow,
    maxRedirs,
    setMaxRedirs,
    maxTime,
    setMaxTime,
    connectTimeout,
    setConnectTimeout,
    httpVersion,
    setHttpVersion,
    ipVersion,
    setIpVersion,
    insecure,
    setInsecure,
    compressed,
    setCompressed,
    runner,
    tokens,
    preview,
    valid,
    urlOk,
    headerOks,
    headersOk,
    maxRedirsOk,
    maxTimeOk,
    connectTimeoutOk,
    execute,
  };
}

// Rust の Curl::preview と同じ並びにすること (安全用の固定オプションは表示しない)。
function buildTokens(input: {
  url: string;
  head: boolean;
  headers: string[];
  follow: boolean;
  maxRedirs: number;
  maxTime: number;
  connectTimeout: number | null;
  httpVersion: CurlHttpVersion;
  ipVersion: CurlIpVersion;
  insecure: boolean;
  compressed: boolean;
}): CmdToken[] {
  const out: CmdToken[] = [tok("curl", "cmd"), tok("-sS", "fixed")];
  out.push(input.head ? tok("-I", "flag", "method") : tok("-i", "fixed", "method"));
  if (input.follow) {
    out.push(
      tok("-L", "flag", "follow"),
      tok("--max-redirs", "flag", "maxRedirs"),
      tok(String(input.maxRedirs), "value", "maxRedirs"),
    );
  }
  out.push(tok("-m", "flag", "maxTime"), tok(String(input.maxTime), "value", "maxTime"));
  if (input.connectTimeout !== null) {
    out.push(
      tok("--connect-timeout", "flag", "connectTimeout"),
      tok(String(input.connectTimeout), "value", "connectTimeout"),
    );
  }
  if (input.httpVersion === "1.1") out.push(tok("--http1.1", "flag", "httpVersion"));
  if (input.httpVersion === "2") out.push(tok("--http2", "flag", "httpVersion"));
  if (input.ipVersion === "4") out.push(tok("-4", "flag", "ipVersion"));
  if (input.ipVersion === "6") out.push(tok("-6", "flag", "ipVersion"));
  if (input.insecure) out.push(tok("-k", "flag", "insecure"));
  if (input.compressed) out.push(tok("--compressed", "flag", "compressed"));
  for (const header of input.headers) {
    out.push(tok("-H", "flag", "headers"), tok(header, "value", "headers"));
  }
  out.push(tok(input.url || "…", input.url ? "value" : "placeholder", "url"));
  return out;
}
