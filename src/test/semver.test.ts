import { compareSemver, isWindowsX64Installer, releaseToUpdateCheck } from "../updates/semver";

test("compareSemver treats v-prefix as equal", () => {
  expect(compareSemver("0.3.0", "v0.3.0")).toBe(0);
  expect(compareSemver("0.3.0", "0.2.1")).toBe(1);
  expect(compareSemver("0.2.1", "0.3.0")).toBe(-1);
});

test("only Windows x64 setup.exe assets are installers", () => {
  expect(isWindowsX64Installer("Forge_0.3.0_x64-setup.exe")).toBe(true);
  expect(isWindowsX64Installer("Forge_0.3.0_x64_en-US.msi")).toBe(false);
  expect(isWindowsX64Installer("Forge_0.3.0.dmg")).toBe(false);
});

test("releaseToUpdateCheck maps a GitHub latest payload", () => {
  const check = releaseToUpdateCheck("0.3.0", {
    tag_name: "v0.2.1",
    body: "notes",
    assets: [
      {
        name: "Forge_0.2.1_x64-setup.exe",
        browser_download_url: "https://github.com/example/Forge/releases/download/v0.2.1/Forge_0.2.1_x64-setup.exe",
      },
    ],
  });
  expect(check.upToDate).toBe(true);
  expect(check.latestVersion).toBe("0.2.1");
  expect(check.downloadUrl).toContain("Forge_0.2.1_x64-setup.exe");
});
