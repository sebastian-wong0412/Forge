export function normalizeVersion(value: string): string {
  return value.trim().replace(/^v/i, "");
}

export function compareSemver(left: string, right: string): number {
  const a = normalizeVersion(left).split(".").map((part) => Number.parseInt(part, 10) || 0);
  const b = normalizeVersion(right).split(".").map((part) => Number.parseInt(part, 10) || 0);
  const length = Math.max(a.length, b.length);
  for (let i = 0; i < length; i += 1) {
    const delta = (a[i] ?? 0) - (b[i] ?? 0);
    if (delta !== 0) {
      return delta > 0 ? 1 : -1;
    }
  }
  return 0;
}

export function isWindowsX64Installer(name: string): boolean {
  const lower = name.toLowerCase();
  return lower.endsWith(".exe") && lower.includes("x64") && lower.includes("setup");
}

export interface UpdateCheck {
  currentVersion: string;
  latestVersion: string;
  notes: string;
  upToDate: boolean;
  assetName: string | null;
  downloadUrl: string | null;
}

export interface GitHubReleaseAsset {
  name: string;
  browser_download_url: string;
}

export interface GitHubRelease {
  tag_name: string;
  body?: string | null;
  assets?: GitHubReleaseAsset[];
}

export function releaseToUpdateCheck(currentVersion: string, release: GitHubRelease): UpdateCheck {
  const latestVersion = normalizeVersion(release.tag_name);
  const asset = (release.assets ?? []).find((item) => isWindowsX64Installer(item.name)) ?? null;
  return {
    currentVersion: normalizeVersion(currentVersion),
    latestVersion,
    notes: (release.body ?? "").trim(),
    upToDate: compareSemver(currentVersion, latestVersion) >= 0,
    assetName: asset?.name ?? null,
    downloadUrl: asset?.browser_download_url ?? null,
  };
}
