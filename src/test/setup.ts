import "@testing-library/jest-dom/vitest";
import { beforeEach } from "vitest";

Object.defineProperty(window.navigator, "language", {
  configurable: true,
  value: "zh-CN",
});

beforeEach(() => {
  localStorage.clear();
  document.documentElement.dataset.theme = "system";
  document.documentElement.lang = "zh-CN";
});
