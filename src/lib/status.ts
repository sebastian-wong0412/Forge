import { statusMessageKey, tCurrent } from "../i18n";

export function statusLabel(status: string): string {
  const key = statusMessageKey(status);
  return key ? tCurrent(key) : status;
}
