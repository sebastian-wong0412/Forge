const DEFAULT_DEV_API_URL = "http://127.0.0.1:8080";
export const DESKTOP_API_URL = "http://127.0.0.1:17340";

export function isTauriShell(): boolean {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}

export const API_BASE_URL =
  import.meta.env.VITE_API_BASE_URL ??
  (isTauriShell() ? DESKTOP_API_URL : DEFAULT_DEV_API_URL);

export async function waitForApi(
  baseUrl: string = API_BASE_URL,
  timeoutMs = 15_000,
): Promise<void> {
  const deadline = Date.now() + timeoutMs;
  while (Date.now() < deadline) {
    try {
      const response = await fetch(`${baseUrl}/health`);
      if (response.ok) {
        return;
      }
    } catch {
      // The desktop shell starts forge-server on launch; retry until it answers.
    }
    await new Promise((resolve) => setTimeout(resolve, 150));
  }
  throw new Error("backend_unavailable");
}
