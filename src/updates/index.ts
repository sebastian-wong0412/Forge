import { isTauriShell } from "../config";
import { GITHUB_REPO } from "../i18n";
import { releaseToUpdateCheck, type UpdateCheck } from "./semver";

export type { UpdateCheck } from "./semver";

async function currentAppVersion(): Promise<string> {
  if (isTauriShell()) {
    try {
      const { invoke } = await import("@tauri-apps/api/core");
      return await invoke<string>("app_version");
    } catch {
      return __FORGE_VERSION__;
    }
  }
  return __FORGE_VERSION__;
}

export async function getAppVersion(): Promise<string> {
  return currentAppVersion();
}

export async function checkForUpdates(): Promise<UpdateCheck> {
  const currentVersion = await currentAppVersion();
  if (isTauriShell()) {
    try {
      const { invoke } = await import("@tauri-apps/api/core");
      return await invoke<UpdateCheck>("check_for_updates");
    } catch (error) {
      throw toError(error, "network");
    }
  }

  let response: Response;
  try {
    response = await fetch(`https://api.github.com/repos/${GITHUB_REPO}/releases/latest`, {
      headers: { Accept: "application/vnd.github+json" },
    });
  } catch {
    throw new Error("network");
  }
  if (!response.ok) {
    throw new Error("failed");
  }
  try {
    const release = (await response.json()) as {
      tag_name: string;
      body?: string | null;
      assets?: { name: string; browser_download_url: string }[];
    };
    if (!release.tag_name) {
      throw new Error("invalid");
    }
    return releaseToUpdateCheck(currentVersion, release);
  } catch (error) {
    if (error instanceof Error && ["invalid", "failed", "network"].includes(error.message)) {
      throw error;
    }
    throw new Error("invalid");
  }
}

export async function downloadInstaller(url: string): Promise<string> {
  if (!isTauriShell()) {
    throw new Error("download-desktop-only");
  }
  try {
    const { invoke } = await import("@tauri-apps/api/core");
    return await invoke<string>("download_installer", { url });
  } catch (error) {
    throw toError(error, "download");
  }
}

export async function openExternal(target: string): Promise<void> {
  if (isTauriShell()) {
    const { invoke } = await import("@tauri-apps/api/core");
    await invoke("open_external", { target });
    return;
  }
  window.open(target, "_blank", "noopener,noreferrer");
}

function toError(error: unknown, fallback: string): Error {
  if (typeof error === "string" && error.length > 0) {
    return new Error(error);
  }
  if (error instanceof Error && error.message) {
    return error;
  }
  return new Error(fallback);
}
