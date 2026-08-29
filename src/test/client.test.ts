import { afterEach, expect, test, vi } from "vitest";
import { ApiClientError, request } from "../api/client";

afterEach(() => {
  vi.unstubAllGlobals();
});

test("parses the API error envelope into a readable message", async () => {
  vi.stubGlobal(
    "fetch",
    vi.fn().mockResolvedValue({
      ok: false,
      status: 404,
      text: async () =>
        JSON.stringify({
          error: { code: "not_found", message: "task `abc` was not found" },
        }),
    }),
  );

  await expect(request("/api/v1/tasks/abc")).rejects.toMatchObject({
    name: "ApiClientError",
    status: 404,
    code: "not_found",
    message: "未找到该内容。",
  } satisfies Partial<ApiClientError>);
});
