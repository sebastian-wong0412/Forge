import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { BrowserRouter } from "react-router-dom";
import { App } from "./App";
import { API_BASE_URL, isTauriShell, waitForApi } from "./config";
import "./styles.css";

function requireRoot(): HTMLElement {
  const element = document.getElementById("root");
  if (!element) {
    throw new Error("Forge UI root element was not found.");
  }
  return element;
}

const rootElement = requireRoot();

async function boot(): Promise<void> {
  if (isTauriShell()) {
    try {
      await waitForApi(API_BASE_URL);
    } catch {
      rootElement.textContent = "Forge 无法启动本地服务。请关闭后重试。";
      return;
    }
  }

  createRoot(rootElement).render(
    <StrictMode>
      <BrowserRouter>
        <App />
      </BrowserRouter>
    </StrictMode>,
  );
}

void boot();
