import { copyFileSync, mkdirSync } from "node:fs";
import { dirname, join } from "node:path";
import { execFileSync } from "node:child_process";
import { fileURLToPath } from "node:url";

const root = join(dirname(fileURLToPath(import.meta.url)), "..");
const profile = process.argv[2] === "debug" ? "debug" : "release";
const cargoArgs = ["build", "-p", "forge-server"];
if (profile === "release") {
  cargoArgs.push("--release");
}

function resolveWindowsExe(name) {
  const lines = execFileSync("where.exe", [name], { encoding: "utf8" })
    .split(/\r?\n/)
    .map((line) => line.trim())
    .filter(Boolean);
  const exe = lines.find((line) => line.toLowerCase().endsWith(".exe"));
  if (!exe) {
    throw new Error(`Could not find ${name}.exe on PATH`);
  }
  return exe;
}

const cargo = process.platform === "win32" ? resolveWindowsExe("cargo") : "cargo";
const rustc = process.platform === "win32" ? resolveWindowsExe("rustc") : "rustc";

execFileSync(cargo, cargoArgs, { cwd: root, stdio: "inherit" });

const triple = execFileSync(rustc, ["--print", "host-tuple"], {
  cwd: root,
  encoding: "utf8",
}).trim();

if (!triple) {
  throw new Error("Could not determine rustc host triple");
}

const srcName = process.platform === "win32" ? "forge.exe" : "forge";
const destName =
  process.platform === "win32"
    ? `forge-server-${triple}.exe`
    : `forge-server-${triple}`;

const src = join(root, "target", profile, srcName);
const destDir = join(root, "src-tauri", "binaries");
mkdirSync(destDir, { recursive: true });
const dest = join(destDir, destName);
copyFileSync(src, dest);
console.log(`Copied sidecar ${src} -> ${dest}`);
