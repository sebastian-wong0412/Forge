import {
  copyFileSync,
  existsSync,
  mkdirSync,
  readFileSync,
  statSync,
} from "node:fs";
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

function requireFile(path, label) {
  if (!existsSync(path)) {
    throw new Error(`${label} does not exist: ${path}`);
  }
  if (statSync(path).size <= 0) {
    throw new Error(`${label} is empty: ${path}`);
  }
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

const tauriConfPath = join(root, "src-tauri", "tauri.conf.json");
const tauriConf = JSON.parse(readFileSync(tauriConfPath, "utf8"));
const externalBins = tauriConf.bundle?.externalBin ?? [];
if (!Array.isArray(externalBins) || externalBins.length !== 1) {
  throw new Error(
    `src-tauri/tauri.conf.json bundle.externalBin must contain exactly one entry, found ${JSON.stringify(externalBins)}`,
  );
}

const externalBin = externalBins[0];
if (typeof externalBin !== "string" || !externalBin.startsWith("binaries/")) {
  throw new Error(
    `Unexpected bundle.externalBin entry '${externalBin}'. Expected a path under binaries/.`,
  );
}

const srcName = process.platform === "win32" ? "forge.exe" : "forge";
const destName =
  process.platform === "win32"
    ? `${externalBin}-${triple}.exe`
    : `${externalBin}-${triple}`;

const src = join(root, "target", profile, srcName);
const dest = join(root, "src-tauri", destName);
const destFileName = dest.split(/[\\/]/).pop();
const expectedFileName =
  process.platform === "win32"
    ? `${externalBin.slice("binaries/".length)}-${triple}.exe`
    : `${externalBin.slice("binaries/".length)}-${triple}`;

if (destFileName !== expectedFileName) {
  throw new Error(
    `Sidecar destination '${destFileName}' does not match tauri.conf.json externalBin '${externalBin}' for ${triple}`,
  );
}

requireFile(
  src,
  `Sidecar source binary after \`cargo ${cargoArgs.join(" ")}\` (bin name is "forge")`,
);

mkdirSync(dirname(dest), { recursive: true });
copyFileSync(src, dest);
requireFile(dest, "Sidecar destination binary");
console.log(`Copied sidecar ${src} -> ${dest}`);
