import { useEffect, useState } from "react";
import { PageHeader } from "../components/PageHeader";
import { ErrorState } from "../components/ErrorState";
import { GITHUB_REPO_URL, useSettings } from "../i18n";
import {
  checkForUpdates,
  downloadInstaller,
  getAppVersion,
  openExternal,
  type UpdateCheck,
} from "../updates";

export function SettingsPage() {
  const { preferences, t, setLanguage, setTheme } = useSettings();
  const [version, setVersion] = useState(__FORGE_VERSION__);
  const [update, setUpdate] = useState<UpdateCheck | null>(null);
  const [updateError, setUpdateError] = useState<string | null>(null);
  const [checking, setChecking] = useState(false);
  const [downloading, setDownloading] = useState(false);
  const [downloadedPath, setDownloadedPath] = useState<string | null>(null);

  useEffect(() => {
    void getAppVersion().then(setVersion);
  }, []);

  async function onCheck() {
    setChecking(true);
    setUpdateError(null);
    setDownloadedPath(null);
    try {
      setUpdate(await checkForUpdates());
    } catch (error) {
      setUpdate(null);
      const code = error instanceof Error ? error.message : "failed";
      setUpdateError(
        code === "network"
          ? t("settings.update.network")
          : code === "invalid"
            ? t("settings.update.invalid")
            : t("settings.update.failed"),
      );
    } finally {
      setChecking(false);
    }
  }

  async function onDownload() {
    if (!update?.downloadUrl) {
      return;
    }
    setDownloading(true);
    setUpdateError(null);
    try {
      setDownloadedPath(await downloadInstaller(update.downloadUrl));
    } catch {
      setUpdateError(t("settings.update.downloadFailed"));
    } finally {
      setDownloading(false);
    }
  }

  return (
    <div className="stack">
      <PageHeader kicker={t("settings.kicker")} title={t("settings.title")} />

      <section className="panel stack">
        <h2 className="section-title">{t("settings.general")}</h2>
        <div className="form-grid">
          <div className="field">
            <label htmlFor="settings-language">{t("settings.language")}</label>
            <select
              id="settings-language"
              value={preferences.language}
              onChange={(event) =>
                setLanguage(event.target.value as typeof preferences.language)
              }
            >
              <option value="system">{t("settings.language.system")}</option>
              <option value="zh">{t("settings.language.zh")}</option>
              <option value="en">{t("settings.language.en")}</option>
            </select>
          </div>
          <div className="field">
            <label htmlFor="settings-theme">{t("settings.theme")}</label>
            <select
              id="settings-theme"
              value={preferences.theme}
              onChange={(event) => setTheme(event.target.value as typeof preferences.theme)}
            >
              <option value="system">{t("settings.theme.system")}</option>
              <option value="dark">{t("settings.theme.dark")}</option>
            </select>
          </div>
        </div>
      </section>

      <section className="panel stack">
        <h2 className="section-title">{t("settings.about")}</h2>
        <p>
          <strong>Forge</strong>
        </p>
        <p>
          {t("settings.version")}: {version}
        </p>
        <p>
          {t("settings.license")}: {t("settings.licenseValue")}
        </p>
        <p className="muted">{GITHUB_REPO_URL}</p>
        <div className="row">
          <button
            type="button"
            className="btn"
            onClick={() => void openExternal(GITHUB_REPO_URL)}
          >
            {t("settings.githubAction")}
          </button>
        </div>
      </section>

      <section className="panel stack">
        <h2 className="section-title">{t("settings.update")}</h2>
        <div className="row">
          <button type="button" className="btn" disabled={checking} onClick={() => void onCheck()}>
            {checking ? t("settings.update.checking") : t("settings.update.check")}
          </button>
        </div>
        {updateError ? <ErrorState message={updateError} /> : null}
        {update && update.upToDate ? <p>{t("settings.update.latest")}</p> : null}
        {update && !update.upToDate ? (
          <div className="stack">
            <p>{t("settings.update.available")}</p>
            <p className="muted">{t("settings.update.current", { version: update.currentVersion })}</p>
            <p className="muted">
              {t("settings.update.latestVersion", { version: update.latestVersion })}
            </p>
            {update.notes ? (
              <div>
                <h3 className="section-title">{t("settings.update.notes")}</h3>
                <pre className="release-notes">{update.notes}</pre>
              </div>
            ) : null}
            {update.downloadUrl ? (
              <div className="row">
                <button
                  type="button"
                  className="btn btn-primary"
                  disabled={downloading}
                  onClick={() => void onDownload()}
                >
                  {downloading ? t("settings.update.downloading") : t("settings.update.download")}
                </button>
              </div>
            ) : (
              <p className="muted">{t("settings.update.noWindowsAsset")}</p>
            )}
          </div>
        ) : null}
        {downloadedPath ? (
          <div className="stack">
            <p>{t("settings.update.downloaded", { path: downloadedPath })}</p>
            <div className="row">
              <button
                type="button"
                className="btn"
                onClick={() => void openExternal(downloadedPath)}
              >
                {t("settings.update.openFolder")}
              </button>
            </div>
          </div>
        ) : null}
      </section>
    </div>
  );
}
