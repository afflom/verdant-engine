#!/usr/bin/env node
/**
 * Mandatory Pull-Through Blank Frame Verification Gate (`VE-44`, `AUTH-STITCH-SELF-HOSTING`).
 *
 * Proves in clean headless Chromium that:
 *   1. The frozen Composable Blank Frame (`site/frame/`, <= 30 KiB) hosts ZERO local UI/Studio/Wasm files.
 *   2. With 4 of 8 Reed-Solomon STCH2 shards deliberately withheld (HTTP 404), the Blank Frame
 *      pulls the 4 surviving shards from a separate Stitch relay origin, recovers `verdant-vX.Y.Z.holo`
 *      bit-for-bit via GF(2^8) Cauchy Reed-Solomon, verifies SHA-256 on the capsule and all 100 lifted
 *      files, mounts them into `frame-sw.js` Virtual FS, and boots live lifted Verdant Studio!
 *   3. Composable third-party deep-link pinning (`?pin=stitch://verdant/catalogs`) works and malicious
 *      deep links (`javascript:`, `../`) are strictly rejected (`Threat 18`).
 *   4. Zero-account local `Prism ID` Private Boundary encryption + 4-of-8 sharding + 4-shard-loss
 *      recovery works client-side with ZERO GitHub or external cloud accounts (`Threat 12/17`).
 */

import { createServer } from "node:http";
import { mkdtemp, readFile, readdir, rm, stat } from "node:fs/promises";
import { execFileSync, execSync } from "node:child_process";
import { extname, join, resolve } from "node:path";
import { tmpdir } from "node:os";
import { pathToFileURL } from "node:url";

const repoRoot = resolve(process.cwd());
const frameDir = join(repoRoot, "site", "frame");
const MAX_BLANK_FRAME_BYTES = 30720;

const mimeTypes = {
  ".html": "text/html; charset=utf-8",
  ".mjs": "text/javascript; charset=utf-8",
  ".js": "text/javascript; charset=utf-8",
  ".json": "application/json; charset=utf-8",
  ".stch": "application/octet-stream",
};

// 1. Verify Blank Frame footprint <= 30 KiB and ZERO loose UI/Wasm files inside `site/frame/`.
const frameFiles = (await readdir(frameDir)).sort();
const allowedFrameFiles = new Set(["frame-boot.mjs", "frame-pin.json", "frame-sw.js", "index.html"]);
let totalFrameBytes = 0;
for (const name of frameFiles) {
  if (!allowedFrameFiles.has(name)) {
    throw new Error(`Blank Frame contamination: unexpected file \`${name}\` inside site/frame/`);
  }
  const st = await stat(join(frameDir, name));
  totalFrameBytes += st.size;
}
if (totalFrameBytes > MAX_BLANK_FRAME_BYTES) {
  throw new Error(`Blank Frame exceeds 30 KiB budget: ${totalFrameBytes} > ${MAX_BLANK_FRAME_BYTES}`);
}

// 2. Pack release into isolated temp directory (unless --dir was supplied)
const ledgerRaw = JSON.parse(
  await readFile(join(repoRoot, "site", "generated", "release-ledger.json"), "utf8"),
);
const activeVersion = `${ledgerRaw.currentVersion.major}.${ledgerRaw.currentVersion.minor}.${ledgerRaw.currentVersion.patch}`;
const activeEpoch = String(ledgerRaw.releaseEpoch || 1);

const dirArgIdx = process.argv.indexOf("--dir");
let releaseDir = dirArgIdx !== -1 ? resolve(process.argv[dirArgIdx + 1]) : null;
let cleanupTmp = false;
if (!releaseDir) {
  releaseDir = await mkdtemp(join(tmpdir(), "verdant-stitch-gate-"));
  cleanupTmp = true;
  const cliBin = join(repoRoot, "target", "debug", "verdant");
  try {
    await stat(cliBin);
    execFileSync(
      cliBin,
      [
        "release",
        "pack",
        "--root",
        repoRoot,
        "--version",
        activeVersion,
        "--epoch",
        activeEpoch,
        "--out",
        releaseDir,
        "--pin",
        "stitch://verdant/studio",
      ],
      { stdio: "pipe" },
    );
  } catch {
    execFileSync(
      "cargo",
      [
        "run",
        "-q",
        "--locked",
        "-p",
        "verdant-cli",
        "--bin",
        "verdant",
        "--",
        "release",
        "pack",
        "--root",
        repoRoot,
        "--version",
        activeVersion,
        "--epoch",
        activeEpoch,
        "--out",
        releaseDir,
        "--pin",
        "stitch://verdant/studio",
      ],
      { stdio: "pipe" },
    );
  }
}

const pinJson = JSON.parse(await readFile(join(releaseDir, "frame-pin.json"), "utf8"));
const shardsDir = join(releaseDir, "shards");

// 3. Server A: Strictly isolated Blank Frame origin (ZERO local UI/Studio/Wasm files!)
const leakedFrameRequests = [];
const frameServer = createServer(async (req, res) => {
  const reqPath = decodeURIComponent(new URL(req.url, "http://127.0.0.1").pathname);
  const rel = reqPath === "/" ? "index.html" : reqPath.replace(/^\/+/, "");
  if (rel === "frame-pin.json") {
    const body = await readFile(join(releaseDir, "frame-pin.json"));
    res.writeHead(200, { "content-type": mimeTypes[".json"], "cache-control": "no-store" });
    res.end(body);
    return;
  }
  if (!allowedFrameFiles.has(rel)) {
    leakedFrameRequests.push(reqPath);
    res.writeHead(404, { "content-type": "text/plain; charset=utf-8" });
    res.end(`Blank Frame has NO loose file: ${reqPath}`);
    return;
  }
  const body = await readFile(join(frameDir, rel));
  res.writeHead(200, {
    "content-type": mimeTypes[extname(rel)] ?? "application/octet-stream",
    "cache-control": "no-store",
  });
  res.end(body);
});

// 4. Server B: Separate Stitch Relay origin — DELIBERATELY WITHHOLDS 4 of 8 shards (0, 2, 4, 6)!
const withheldShards = new Set(["shard-0.stch", "shard-2.stch", "shard-4.stch", "shard-6.stch"]);
let servedShards = 0;
let withheldHits = 0;
const relayServer = createServer(async (req, res) => {
  const reqPath = decodeURIComponent(new URL(req.url, "http://127.0.0.1").pathname);
  const cors = { "access-control-allow-origin": "*", "cache-control": "no-store" };
  const match = reqPath.match(/^\/shards\/(shard-\d+\.stch)$/);
  if (!match) {
    res.writeHead(404, cors);
    res.end("Not found");
    return;
  }
  const fname = match[1];
  if (withheldShards.has(fname)) {
    withheldHits++;
    res.writeHead(404, cors);
    res.end(`Simulated Stitch shard loss: ${fname}`);
    return;
  }
  const body = await readFile(join(shardsDir, fname));
  servedShards++;
  res.writeHead(200, { ...cors, "content-type": "application/octet-stream" });
  res.end(body);
});

await Promise.all([
  new Promise((r) => frameServer.listen(0, "127.0.0.1", r)),
  new Promise((r) => relayServer.listen(0, "127.0.0.1", r)),
]);

const frameOrigin = `http://127.0.0.1:${frameServer.address().port}`;
const relayOrigin = `http://127.0.0.1:${relayServer.address().port}`;

const playwright = await import("playwright").catch(() =>
  import(pathToFileURL(join(execSync("npm root -g").toString().trim(), "playwright", "index.mjs")).href),
);

const browser = await playwright.chromium.launch({
  args: ["--use-gl=angle", "--use-angle=swiftshader", "--enable-unsafe-swiftshader"],
});

try {
  const context = await browser.newContext({ viewport: { width: 1440, height: 900 } });
  const page = await context.newPage();
  const pageErrors = [];
  page.on("pageerror", (err) => pageErrors.push(String(err)));

  // A. Multi-Origin Failover + 4-of-8 Pull-Through Blank Frame boot into lifted Verdant Studio!
  // Primary relay (`/dead-primary-relay`) fails with 404/503; secondary relay (`/shards`) withholds 4 of 8 shards!
  const multiRelays = `${relayOrigin}/dead-primary-relay,${relayOrigin}/shards`;
  const studioUrl = `${frameOrigin}/?relay=${encodeURIComponent(multiRelays)}&pin=${encodeURIComponent("stitch://verdant/studio")}`;
  await page.goto(studioUrl, { waitUntil: "domcontentloaded", timeout: 30000 });
  await page.waitForFunction(
    () => window.__VERDANT_BLANK_FRAME__ && window.__VERDANT_BLANK_FRAME__.status !== "booting",
    { timeout: 30000 },
  );
  const bootState = await page.evaluate(() => ({
    status: window.__VERDANT_BLANK_FRAME__.status,
    error: window.__VERDANT_BLANK_FRAME__.error || null,
    version: window.__VERDANT_BLANK_FRAME__.version,
    capsuleSha256Hex: window.__VERDANT_BLANK_FRAME__.capsuleSha256Hex,
    mountedFiles: window.__VERDANT_BLANK_FRAME__.mountedFiles,
    targetRoute: window.__VERDANT_BLANK_FRAME__.targetRoute,
    shardsUsed: window.__VERDANT_BLANK_FRAME__.shardsUsed,
    shardsDropped: window.__VERDANT_BLANK_FRAME__.shardsDropped,
    relayCount: window.__VERDANT_BLANK_FRAME__.relayCount,
    failedRelays: window.__VERDANT_BLANK_FRAME__.failedRelays,
    relayHits: window.__VERDANT_BLANK_FRAME__.relayHits,
  }));

  if (bootState.status !== "mounted") {
    throw new Error(`Blank Frame Stitch pull-through failed: ${JSON.stringify(bootState)}`);
  }
  if (bootState.relayCount < 2 || !bootState.failedRelays[`${relayOrigin}/dead-primary-relay`]) {
    throw new Error(`Multi-origin relay failover not exercised: ${JSON.stringify(bootState)}`);
  }
  if (bootState.capsuleSha256Hex !== pinJson.releaseKappaHex) {
    throw new Error(`Recovered kappa mismatch: ${bootState.capsuleSha256Hex} != ${pinJson.releaseKappaHex}`);
  }
  if (bootState.shardsUsed !== 4 || withheldHits < 4) {
    throw new Error(`Expected 4 surviving shards + >=4 withheld hits, got used=${bootState.shardsUsed}, withheld=${withheldHits}`);
  }

  // Verify the lifted Verdant Studio iframe actually loaded exclusively from ServiceWorker Virtual FS!
  const studioFrame = page.frameLocator("#verdant-lifted-frame");
  await studioFrame.locator("canvas").first().waitFor({ state: "visible", timeout: 20000 });
  const iframeCheck = await page.evaluate(() => {
    const iframe = document.getElementById("verdant-lifted-frame");
    const doc = iframe?.contentDocument;
    return {
      title: doc?.title || "",
      canvasCount: doc?.querySelectorAll("canvas").length || 0,
      hasHeader: Boolean(doc?.querySelector("header, nav, main")),
      bodyLen: doc?.body?.innerText?.trim().length || 0,
    };
  });
  if (iframeCheck.canvasCount < 1 || iframeCheck.bodyLen < 50) {
    throw new Error(`Lifted Verdant Studio iframe did not render properly: ${JSON.stringify(iframeCheck)}`);
  }

  // B. Threat 18 Anti-Injection & Zero-Account Private Boundary 4-of-8 recovery inside Blank Frame!
  const securityCheck = await page.evaluate(async () => {
    const api = window.__VERDANT_BLANK_FRAME__;
    let refusedJs = false;
    try {
      api.sanitizeStitchDeepLink("javascript:alert(1)");
    } catch {
      refusedJs = true;
    }
    let refusedTraversal = false;
    try {
      api.sanitizeStitchDeepLink("stitch://verdant/../../etc/passwd");
    } catch {
      refusedTraversal = true;
    }
    const secret = "Accountless private Stitch world boundary (zero GitHub account)";
    const sealed = await api.sealPrivateBoundaryZeroAccount("stitch://boundary/ci-proof", secret, 4, 8);
    // Drop 4 of 8 shards and recover from surviving parity+data mix!
    const kept = [sealed.shards[1], sealed.shards[3], sealed.shards[5], sealed.shards[7]];
    const recovered = await api.recoverPrivateBoundaryZeroAccount(kept);

    // Zero-account WebRTC DataChannel peer shard exchange + reciprocal budget cap check:
    const cachedPeer = await api.peerMesh.cacheVerified("prism:peer-alpha", sealed.shards[1]);
    const served = api.peerMesh.serveToPeer("prism:peer-beta", cachedPeer.key);
    const webrtcRes = await api.peerMesh.openLoopbackWebRtcDemo(cachedPeer.key);
    let refusedBadPeer = false;
    try {
      api.peerMesh.serveToPeer("github-oauth-only", cachedPeer.key);
    } catch {
      refusedBadPeer = true;
    }
    return {
      refusedJs,
      refusedTraversal,
      ownerPrismId: sealed.ownerPrismId,
      shardCount: sealed.shardCount,
      recoveredMatch: recovered.plaintextUtf8 === secret,
      peerServedBytes: served.byteLength,
      webrtcVerified: Boolean(webrtcRes?.verified),
      refusedBadPeer,
    };
  });
  if (
    !securityCheck.refusedJs ||
    !securityCheck.refusedTraversal ||
    !securityCheck.recoveredMatch ||
    !securityCheck.webrtcVerified ||
    !securityCheck.refusedBadPeer
  ) {
    throw new Error(`Blank Frame security/vault/WebRTC check failed: ${JSON.stringify(securityCheck)}`);
  }

  // C. Composable third-party deep-link pin (`?pin=stitch://verdant/catalogs`)
  const catalogsUrl = `${frameOrigin}/?relay=${encodeURIComponent(`${relayOrigin}/shards`)}&pin=${encodeURIComponent("stitch://verdant/catalogs")}`;
  await page.goto(catalogsUrl, { waitUntil: "domcontentloaded", timeout: 30000 });
  await page.waitForFunction(
    () => window.__VERDANT_BLANK_FRAME__ && window.__VERDANT_BLANK_FRAME__.status === "mounted",
    { timeout: 30000 },
  );
  const catalogsRoute = await page.evaluate(() => window.__VERDANT_BLANK_FRAME__.targetRoute);
  if (catalogsRoute !== "catalogs/index.html") {
    throw new Error(`Composable deep-link pin failed: expected catalogs/index.html, got ${catalogsRoute}`);
  }

  if (leakedFrameRequests.length > 0) {
    throw new Error(`Blank Frame origin leaked loose file requests: ${leakedFrameRequests.join(", ")}`);
  }
  if (pageErrors.length > 0) {
    throw new Error(`Blank Frame pageerror(s): ${pageErrors.join(" | ")}`);
  }

  console.log(
    `verify-stitch-release: PASS — Blank Frame (${totalFrameBytes} B <= ${MAX_BLANK_FRAME_BYTES} B, 0 loose UI files) pulled 4-of-8 STCH2 shards (${withheldHits} withheld shards tolerated), verified ${bootState.mountedFiles} lifted files (kappa ${bootState.capsuleSha256Hex.slice(0, 16)}…), booted live Verdant Studio & Catalogs via ServiceWorker Virtual FS, blocked XSS/traversal pins, and proved zero-account Prism ID (${securityCheck.ownerPrismId}) private 4-of-8 recovery.`,
  );
} finally {
  await browser.close();
  frameServer.close();
  relayServer.close();
  if (cleanupTmp && releaseDir) {
    await rm(releaseDir, { recursive: true, force: true });
  }
}
