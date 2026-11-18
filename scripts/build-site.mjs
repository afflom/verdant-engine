#!/usr/bin/env node
// Assembles `site/` for local development and production serving in Node.js,
// mirroring the assembly steps in `.github/workflows/pages.yml`:
//   1. Ensures a cryptographically verified `site/app.holo` is present (reusing
//      an existing verified archive, a local `.prism/build` output, or the
//      published GitHub Pages archive verified via BLAKE3 + all acceptance vectors).
//   2. Copies `try/vendor` into `site/vendor` (excluding test fixtures).
//   3. Runs `bootstrap/page.mjs site` to package `worker-runtime` and render
//      `site/index.html`, `site/spec.html`, and `site/manifest.webmanifest`.
//   4. Runs `bootstrap/precache.mjs site` to generate `site/precache.json`.

import { execFileSync } from "node:child_process";
import { cp, readdir, readFile, rm, stat, writeFile } from "node:fs/promises";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { verify } from "../ci/kappa-cache.mjs";
import { verifyPrismHoloContract } from "./enforce-prism-contracts.mjs";

const root = fileURLToPath(new URL("../", import.meta.url));
const siteDir = join(root, "site");
const vendorSrc = join(root, "try", "vendor");
const vendorDest = join(siteDir, "vendor");
const holoDest = join(siteDir, "app.holo");

async function tryVerifyBuffer(buf) {
  if (!buf || buf.length < 64) return null;
  try {
    await verify(buf);
    return await verifyPrismHoloContract(buf, { useCache: true, rootDir: root });
  } catch {
    return null;
  }
}

async function findLocalBuildHolo() {
  const buildRoot = join(root, "app", ".prism", "build");
  const subdirs = await readdir(buildRoot, { withFileTypes: true }).catch(() => []);
  const candidates = [];
  for (const entry of subdirs) {
    if (!entry.isDirectory()) continue;
    const dirPath = join(buildRoot, entry.name);
    const files = await readdir(dirPath).catch(() => []);
    for (const f of files) {
      if (f.endsWith(".holo")) {
        const holoPath = join(dirPath, f);
        const st = await stat(holoPath).catch(() => null);
        if (st) candidates.push({ holoPath, mtimeMs: st.mtimeMs });
      }
    }
  }
  candidates.sort((a, b) => b.mtimeMs - a.mtimeMs);
  for (const { holoPath } of candidates) {
    const bytes = await readFile(holoPath).catch(() => null);
    if (bytes && (await tryVerifyBuffer(bytes))) return bytes;
  }
  return null;
}

async function ensureVerifiedHolo() {
  const existing = await readFile(holoDest).catch(() => null);
  const existingOk = await tryVerifyBuffer(existing);
  if (existingOk) {
    console.log(`build-site: present site/app.holo (${existingOk.fingerprint.slice(0, 12)}…)`);
    return;
  }

  const localHolo = await findLocalBuildHolo();
  if (localHolo) {
    await writeFile(holoDest, localHolo);
    console.log("build-site: copied verified local build holo to site/app.holo");
    return;
  }

  try {
    const appMeta = JSON.parse(await readFile(join(siteDir, "app.json"), "utf8"));
    const holoUrl = new URL("app.holo", appMeta.live).href;
    const response = await fetch(holoUrl, { signal: AbortSignal.timeout(15000) });
    if (!response.ok) throw new Error(`HTTP ${response.status} from ${holoUrl}`);
    const fetched = Buffer.from(await response.arrayBuffer());
    const checked = await verify(fetched);
    await writeFile(holoDest, fetched);
    console.log(
      `build-site: fetched and verified site/app.holo (${checked.vectors} vectors replayed, fingerprint ${checked.fingerprint.slice(0, 12)}…)`,
    );
  } catch (error) {
    console.warn(
      `build-site: continuing without site/app.holo (${error instanceof Error ? error.message : String(error)})`,
    );
  }
}

async function syncVendor() {
  await rm(vendorDest, { recursive: true, force: true });
  await cp(vendorSrc, vendorDest, { recursive: true });
  await rm(join(vendorDest, "prism-code", "fixtures"), { recursive: true, force: true });
}

await ensureVerifiedHolo();
await syncVendor();
execFileSync(process.execPath, ["bootstrap/page.mjs", "site"], { cwd: root, stdio: "inherit" });
execFileSync(process.execPath, ["bootstrap/precache.mjs", "site"], { cwd: root, stdio: "inherit" });
execFileSync(process.execPath, ["scripts/enforce-prism-contracts.mjs"], { cwd: root, stdio: "inherit" });

