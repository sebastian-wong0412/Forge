import { API_BASE_URL, isTauriShell } from "../config";
import type { ApiErrorBody } from "./types";

export class ApiClientError extends Error {
  readonly status: number;
  readonly code: string;

  constructor(status: number, code: string, message: string) {
    super(message);
    this.name = "ApiClientError";
    this.status = status;
    this.code = code;
  }
}

function localizeMessage(code: string): string {
  switch (code) {
    case "not_found":
      return "未找到该内容。";
    case "bad_request":
      return "输入有误，请检查后重试。";
    case "domain":
    case "conflict":
      return "当前状态不允许此操作。";
    case "persistence":
      return "保存失败，请稍后重试。";
    case "unreachable":
      return isTauriShell()
        ? "无法连接到 Forge。"
        : "无法连接到 Forge。请确认本地服务已启动。";
    default:
      return "操作失败，请稍后重试。";
  }
}

function humanMessage(status: number, body: unknown): { code: string; message: string } {
  if (status === 0) {
    return { code: "unreachable", message: localizeMessage("unreachable") };
  }

  if (body && typeof body === "object" && "error" in body) {
    const error = (body as ApiErrorBody).error;
    const code = error?.code || "request_failed";
    return { code, message: localizeMessage(code) };
  }

  return { code: "request_failed", message: localizeMessage("request_failed") };
}

export async function request<T>(path: string, init: RequestInit = {}): Promise<T> {
  const headers = new Headers(init.headers);
  if (init.body !== undefined && !headers.has("content-type")) {
    headers.set("content-type", "application/json");
  }

  let response: Response;
  try {
    response = await fetch(`${API_BASE_URL}${path}`, {
      ...init,
      headers,
    });
  } catch {
    throw new ApiClientError(0, "unreachable", localizeMessage("unreachable"));
  }

  const text = await response.text();
  const body = text ? safeJson(text) : null;

  if (!response.ok) {
    const { code, message } = humanMessage(response.status, body);
    throw new ApiClientError(response.status, code, message);
  }

  return (body as T) ?? (undefined as T);
}

function safeJson(text: string): unknown {
  try {
    return JSON.parse(text);
  } catch {
    return null;
  }
}

export function toQuery(params: Record<string, string>): string {
  return new URLSearchParams(params).toString();
}
