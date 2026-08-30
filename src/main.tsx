import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { BrowserRouter } from "react-router-dom";
import { App } from "./App";
import { API_BASE_URL, isTauriShell, waitForApi } from "./config";
import { SettingsProvider, applyTheme, loadPreferencesSync, tCurrent } from "./i18n";
import "./styles.css";

applyTheme(loadPreferencesSync().theme);

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
      rootElement.textContent = tCurrent("error.backendUnavailable");
      return;
    }
  }

  createRoot(rootElement).render(
    <StrictMode>
      <SettingsProvider>
        <BrowserRouter>
          <App />
        </BrowserRouter>
      </SettingsProvider>
    </StrictMode>,
  );
}

void boot();
