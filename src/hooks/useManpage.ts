import { useState } from "react";
import { api } from "../api";
import type { ManpageDocument } from "../types";
import { DEFAULT_MAN_TOPIC } from "../commands/manTopics";
import { tok, tokensToString } from "../commands/tokens";
import { useRunner } from "./useRunner";

function buildTokens(topic: string) {
  const name = topic.trim();
  return [tok("man", "cmd"), name ? tok(name, "value", "topic") : tok("…", "placeholder", "topic")];
}

export function useManpage() {
  const [topic, setTopic] = useState(DEFAULT_MAN_TOPIC);
  const runner = useRunner<ManpageDocument>();

  const tokens = buildTokens(topic);
  const preview = tokensToString(tokens);

  // 一覧から選んだときはその場で表示するので、選んだ名前を直接受け取れるようにする
  async function execute(next?: string) {
    const name = (next ?? topic).trim();
    if (next !== undefined) setTopic(next);
    await runner.run(() => api.getManpage(name), tokensToString(buildTokens(name)));
  }

  return { topic, setTopic, runner, tokens, preview, execute };
}
