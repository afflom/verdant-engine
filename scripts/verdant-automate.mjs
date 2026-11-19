#!/usr/bin/env node
/**
 * Verdant Self-Automation & Kappa-Memoized Verification Orchestrator.
 * Uses Verdant's own native runtime (`verdant-cli automate-check`) + content-addressed
 * SHA-256 input fingerprinting to skip redundant Wasm rebuilds and run browser &
 * contract gates concurrently, eliminating development and CI pipeline latency.
 *
 * Usage:
 *   node scripts/verdant-automate.mjs [--skip-rust] [--force-wasm]
 */

import { createHash } from "node:crypto";
import { spawn } from "node:child_process";
import { existsSync, mkdirSync, readdirSync, readFileSync, rmSync, statSync, writeFileSync } from "node:fs";
import { join, relative } from "node:path";
import { fileURLToPath } from "node:url";

const root = fileURLToPath(new URL("../", import.meta.url));
const args = new Set(process.argv.slice(2));
const skipRust = args.has("--skip-rust");
const forceWasm = args.has("--force-wasm");
const cacheDir = join(root, ".verdant-cache");
const wasmKappaPath = join(cacheDir, "wasm-inputs.sha256");

function walkFiles(dir, out = []) {
  if (!existsSync(dir)) return out;
  for (const entry of readdirSync(dir).sort()) {
    const p = join(dir, entry);
    const st = statSync(p);
    if (st.isDirectory()) walkFiles(p, out);
    else out.push(p);
  }
  return out;
}

function computeWasmInputKappa() {
  const hash = createHash("sha256");
  const inputs = [
    join(root, "Cargo.toml"),
    join(root, "Cargo.lock"),
    ...walkFiles(join(root, "crates", "verdant-core", "src")),
    ...walkFiles(join(root, "crates", "verdant-tensor", "src")),
    ...walkFiles(join(root, "crates", "verdant-wasm", "src")),
    ...walkFiles(join(root, "catalogs")),
  ];
  for (const file of inputs) {
    if (!existsSync(file)) continue;
    hash.update(relative(root, file));
    hash.update("\0");
    hash.update(readFileSync(file));
    hash.update("\0");
  }
  return hash.digest("hex");
}

function runAsync(label, cmd, cmdArgs) {
  const t0 = Date.now();
  return new Promise((resolve, reject) => {
    const child = spawn(cmd, cmdArgs, { cwd: root, stdio: ["ignore", "pipe", "pipe"] });
    let out = "";
    let err = "";
    child.stdout.on("data", (d) => { out += d; });
    child.stderr.on("data", (d) => { err += d; });
    child.on("close", (code) => {
      const ms = Date.now() - t0;
      if (code === 0) {
        const lastLine = out.trim().split(/\r?\n/).filter(Boolean).pop() || "OK";
        console.log(`[PASS ${ms}ms] ${label}: ${lastLine}`);
        resolve({ label, ms, out });
      } else {
        console.error(`[FAIL ${ms}ms] ${label} (exit ${code}):\n${out}\n${err}`);
        reject(new Error(`${label} failed with exit ${code}`));
      }
    });
  });
}

const tAll = Date.now();
mkdirSync(cacheDir, { recursive: true });

// 1. Kappa-memoized Wasm build: skip cargo wasm32 compilation if inputs & outputs are unchanged.
const currentKappa = computeWasmInputKappa();
const cachedKappa = existsSync(wasmKappaPath) ? readFileSync(wasmKappaPath, "utf8").trim() : "";
const wasmPresent = existsSync(join(root, "site", "verdant-wasm.wasm")) && existsSync(join(root, "site", "verdant-uorc.wasm"));

if (!skipRust && (forceWasm || currentKappa !== cachedKappa || !wasmPresent)) {
  await runAsync("sync-prism-wasm", process.execPath, ["scripts/sync-prism-wasm.mjs"]);
  writeFileSync(wasmKappaPath, `${currentKappa}\n`);
} else {
  console.log(`[SKIP 0ms] sync-prism-wasm: kappa hit (${currentKappa.slice(0, 12)}…)`);
}

// 2. Assemble site & run Verdant's native self-audit in parallel:
const stage2 = [runAsync("build-site", process.execPath, ["scripts/build-site.mjs"])];
if (!skipRust) {
  stage2.push(runAsync("verdant-cli automate-check", "cargo", ["run", "--locked", "-p", "verdant-cli", "--", "automate-check"]));
}
await Promise.all(stage2);

// 3. Execute Prism contract suite & all browser deployment gates in parallel:
await Promise.all([
  runAsync("prism-contracts.test.ts", "npx", ["--yes", "tsx", "--test", "prism-contracts.test.ts"]),
  runAsync("verify-live-surfaces", process.execPath, ["scripts/verify-live-surfaces.mjs", "site"]),
  runAsync("content/check", process.execPath, ["scripts/content/check.mjs", "site"]),
  runAsync("weight (FR-03)", process.execPath, ["bootstrap/weight.mjs", "site"]),
]);
rmSync(join(root, "site", "content-report.json"), { force: true });
rmSync(join(root, "site", "content-report.md"), { force: true });

console.log(`verdant-automate: ALL GATES PASSED in ${((Date.now() - tAll) / 1000).toFixed(1)}s`);
