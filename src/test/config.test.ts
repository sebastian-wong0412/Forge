import { afterEach, expect, test, vi } from "vitest";
import { API_BASE_URL, DESKTOP_API_URL, isTauriShell } from "../config";

afterEach(() => {
  vi.unstubAllEnvs();
});

test("browser and unit tests use the developer API port", () => {
  expect(isTauriShell()).toBe(false);
  expect(API_BASE_URL).toBe("http://127.0.0.1:8080");
  expect(DESKTOP_API_URL).toBe("http://127.0.0.1:17340");
});
