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
import { cp, mkdir, readdir, readFile, rm, stat, writeFile } from "node:fs/promises";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { verify } from "../ci/kappa-cache.mjs";
import { verifyPrismHoloContract } from "./enforce-prism-contracts.mjs";

const root = fileURLToPath(new URL("../", import.meta.url));
const siteDir = join(root, "site");
const vendorSrc = join(root, "try", "vendor");
const vendorDest = join(siteDir, "vendor");
const holoDest = join(siteDir, "app.holo");
const stitchProofDest = join(siteDir, "generated", "stitch-proof.holo");

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
  const existingProof = await readFile(stitchProofDest).catch(() => null);
  const existingOk = existing && existingProof && existing[0] === 0x55 && existing[4] === 0x02 && existing[5] === 0x02
    ? await tryVerifyBuffer(existing)
    : null;
  if (existingOk) {
    console.log(`build-site: present Holo v6 site/app.holo (${existingOk.fingerprint.slice(0, 12)}…) + stitch-proof.holo`);
    return;
  }

  try {
    const { buildVerifiedHoloV6 } = await import("./build-holo-v6.mjs");
    const { appHolo, engineReproved, stitchProofHolo, stitchProofReproved } = await buildVerifiedHoloV6({ repoRoot: root });
    await writeFile(holoDest, appHolo.bytes);
    await mkdir(join(siteDir, "generated"), { recursive: true });
    await writeFile(stitchProofDest, stitchProofHolo.bytes);
    console.log(
      `build-site: built and offline-reproved Holo v6 site/app.holo (${appHolo.bytes.length} B, ${engineReproved.theoremsProved} theorems) + stitch-proof.holo (${stitchProofHolo.bytes.length} B, ${stitchProofReproved.theoremsProved} theorems)`,
    );
    return;
  } catch (error) {
    console.warn(`build-site: local Holo v6 build fallback (${error?.message ?? error})`);
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

async function syncStarter() {
  const starterSrc = join(root, "templates", "browser-3d-starter");
  const starterDest = join(siteDir, "starter");
  await mkdir(starterDest, { recursive: true });
  await cp(join(starterSrc, "index.html"), join(starterDest, "index.html"));
  await cp(join(starterSrc, "game.mjs"), join(starterDest, "game.mjs"));
  // Sync SHA-256-locked multi-domain world catalog so /catalogs/ and /studio/ hydrate directly from source of truth.
  await cp(
    join(root, "catalogs", "domains", "world-packs.json"),
    join(siteDir, "catalogs", "world-packs.json"),
  );
}

await ensureVerifiedHolo();
await syncVendor();
await syncStarter();
execFileSync(process.execPath, ["bootstrap/page.mjs", "site"], { cwd: root, stdio: "inherit" });
{
  const { minify } = await import("terser");
  const wrDir = join(siteDir, "worker-runtime");
  for (const name of await readdir(wrDir).catch(() => [])) {
    if (!name.endsWith(".mjs")) continue;
    const p = join(wrDir, name);
    const src = await readFile(p, "utf8");
    const out = await minify(src, { module: true, compress: { passes: 2 }, mangle: true });
    if (out.code) await writeFile(p, out.code);
  }
  const starterGame = join(siteDir, "starter", "game.mjs");
  const gameSrc = await readFile(starterGame, "utf8").catch(() => null);
  if (gameSrc) {
    const out = await minify(gameSrc, { module: true, compress: { passes: 2 }, mangle: true });
    if (out.code) await writeFile(starterGame, out.code);
  }
}
execFileSync(process.execPath, ["bootstrap/precache.mjs", "site"], { cwd: root, stdio: "inherit" });
execFileSync(process.execPath, ["scripts/enforce-prism-contracts.mjs"], { cwd: root, stdio: "inherit" });

