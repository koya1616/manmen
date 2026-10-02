// コマンドバーに表示するトークン。opt はオプション行の id と対応させ、
// トークンとオプション行を相互にハイライトするのに使う。
export type TokenKind =
  | "sudo"
  | "cmd"
  | "sub"
  | "flag"
  | "value"
  | "fixed"
  | "placeholder";

export interface CmdToken {
  text: string;
  kind: TokenKind;
  opt?: string;
}

export function tok(text: string, kind: TokenKind, opt?: string): CmdToken {
  return { text, kind, opt };
}

export function tokensToString(tokens: CmdToken[]): string {
  return tokens.map((token) => token.text).join(" ");
}
