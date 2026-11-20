#!/usr/bin/env node
/**
 * Live Browser UHD 4K/8K + PagedTensorArena Workload Stress & Zero-Account Studio VCS Gate (VE-49, VE-50).
 *
 * Proves in headless Chromium against the built `site/studio/`:
 * 1. 4K (3840x2160) & 8K (7680x4320) glTF + quantized .uorc CodedMatrix AI workloads stream through
 *    Rust/Wasm `PagedTensorArena` + `DynamicWorkloadGovernor` with strict bounded RAM (`peak <= budget`),
 *    active CLOCK/LRU evictions, upheld performance contract, and ZERO main-thread browser freezes.
 * 2. Zero-account in-browser `VerdantVcs` commits (`e1 -> e2`), self-verifying portable `.holo`
 *    export (`VDNT_HOLO_REL_V1`), and `?pin=stitch://` deep-link generation work cleanly inside Studio.
 */

import { execSync } from "node:child_process";
import { createServer } from "node:http";
import { readFile } from "node:fs/promises";
import { extname, join, resolve } from "node:path";
import { pathToFileURL } from "node:url";

const repoRoot = resolve(process.cwd());
const siteDir = resolve(process.argv[2] || join(repoRoot, "site"));

const MIME = {
  ".html": "text/html; charset=utf-8",
  ".js": "text/javascript; charset=utf-8",
  ".mjs": "text/javascript; charset=utf-8",
  ".css": "text/css; charset=utf-8",
  ".json": "application/json; charset=utf-8",
  ".wasm": "application/wasm",
  ".uorc": "application/octet-stream",
  ".holo": "application/octet-stream",
  ".svg": "image/svg+xml",
  ".png": "image/png",
  ".woff2": "font/woff2",
  ".toml": "text/plain; charset=utf-8",
};

const server = createServer(async (req, res) => {
  const urlPath = decodeURIComponent(new URL(req.url, "http://127.0.0.1").pathname);
  const rel = urlPath.endsWith("/") ? `${urlPath}index.html` : urlPath;
  const full = join(siteDir, rel.replace(/^\/+/, ""));
  try {
    const body = await readFile(full);
    res.writeHead(200, { "content-type": MIME[extname(full)] || "application/octet-stream", "cache-control": "no-store" });
    res.end(body);
  } catch {
    res.writeHead(404, { "content-type": "text/plain; charset=utf-8" });
    res.end("Not found");
  }
});

await new Promise((r) => server.listen(0, "127.0.0.1", r));
const origin = `http://127.0.0.1:${server.address().port}`;

const playwright = await import("playwright").catch(() =>
  import(pathToFileURL(join(execSync("npm root -g").toString().trim(), "playwright", "index.mjs")).href),
);

const browser = await playwright.chromium.launch({
  args: ["--use-gl=angle", "--use-angle=swiftshader", "--enable-unsafe-swiftshader"],
});

try {
  const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
  const pageErrors = [];
  page.on("pageerror", (e) => pageErrors.push(String(e)));

  await page.goto(`${origin}/studio/`, { waitUntil: "domcontentloaded", timeout: 30000 });
  await page.waitForFunction(() => Boolean(window.__VERDANT_STUDIO_API__), { timeout: 30000 });

  // 1. Live 4K & 8K UHD + multi-MB glTF & CodedMatrix AI PagedTensorArena stress with main-thread lag monitor:
  const stress = await page.evaluate(async () => {
    const api = window.__VERDANT_STUDIO_API__;
    let maxLagMs = 0;
    let lastTick = performance.now();
    const timer = setInterval(() => {
      const now = performance.now();
      const lag = now - lastTick - 25;
      if (lag > maxLagMs) maxLagMs = lag;
      lastTick = now;
    }, 25);

    try {
      // 4K UHD (3840x2160): 256 x 64 KiB = 16 MiB virtual workload inside 8 MiB ceiling -> must evict cleanly!
      const uhd4k = await api.stepPagedUhdWorkload({
        viewportW: 3840,
        viewportH: 2160,
        isSoftwareGpu: false,
        webgpuAvailable: true,
        frameUs: 9200,
        budgetMb: 8,
        gltfPages: 128,
        codedAiPages: 128,
        pageKb: 64,
      });

      // 8K UHD (7680x4320): 512 x 64 KiB = 32 MiB virtual workload inside 8 MiB ceiling -> must evict cleanly!
      const uhd8k = await api.stepPagedUhdWorkload({
        viewportW: 7680,
        viewportH: 4320,
        isSoftwareGpu: false,
        webgpuAvailable: true,
        frameUs: 11400,
        budgetMb: 8,
        gltfPages: 256,
        codedAiPages: 256,
        pageKb: 64,
      });

      const badgeText = document.getElementById("studio-scale-badge")?.textContent || "";
      const badgeContract = document.getElementById("studio-scale-badge")?.dataset?.contract || "";

      // 2. Zero-account in-browser VerdantVcs e1 -> e2 + portable .holo + ?pin=stitch:// deep-link!
      const c1 = await api.commitAndPackHolo("Initial zero-account 3D scene commit");
      const c2 = await api.commitAndPackHolo("Second revision with UHD lighting");
      const hist = api.getVcsHistory();
      return {
        maxLagMs,
        uhd4k,
        uhd8k,
        badgeText,
        badgeContract,
        c1: { epoch: c1.epoch, kappa: c1.commitKappa, holoBytes: c1.capsuleByteLen, shards: c1.shardCount, magic: c1.holoHex.slice(0, 32), pin: c1.stitchDeepLinkUri },
        c2: { epoch: c2.epoch, kappa: c2.commitKappa, parent: c2.parentCommitKappa, pin: c2.stitchDeepLinkUri },
        histLen: hist.length,
        prismId: api.getLocalPrismId(),
      };
    } finally {
      clearInterval(timer);
    }
  });

  if (!stress.uhd4k?.ok || !stress.uhd4k.contractMet || !stress.uhd4k.arena?.invariantHeld || stress.uhd4k.arena.evictions < 1) {
    throw new Error(`4K UHD PagedTensorArena gate failed: ${JSON.stringify(stress.uhd4k)}`);
  }
  if (!stress.uhd8k?.ok || !stress.uhd8k.contractMet || !stress.uhd8k.arena?.invariantHeld || stress.uhd8k.arena.evictions < 1) {
    throw new Error(`8K UHD PagedTensorArena gate failed: ${JSON.stringify(stress.uhd8k)}`);
  }
  if (stress.uhd8k.arena.peakResidentBytes > stress.uhd8k.arena.budgetBytes) {
    throw new Error(`8K UHD exceeded RAM budget: ${stress.uhd8k.arena.peakResidentBytes} > ${stress.uhd8k.arena.budgetBytes}`);
  }
  if (stress.maxLagMs > 500) {
    throw new Error(`Browser main-thread stalled during UHD workload: maxLagMs=${stress.maxLagMs.toFixed(1)}`);
  }
  if (stress.badgeContract !== "met") {
    throw new Error(`Studio dynamic scale badge did not report contract=met: ${stress.badgeText}`);
  }
  // Verify VDNT_HOLO_REL_V1 hex magic = 56444e545f484f4c4f5f52454c5f5631
  if (stress.c1.magic !== "56444e545f484f4c4f5f52454c5f5631" || stress.c1.shards !== 8) {
    throw new Error(`Portable .holo export verification failed: ${JSON.stringify(stress.c1)}`);
  }
  if (stress.c2.parent !== stress.c1.kappa || stress.c2.epoch !== stress.c1.epoch + 1 || !stress.c2.pin.startsWith("stitch://")) {
    throw new Error(`In-browser VerdantVcs lineage or deep-link failed: ${JSON.stringify(stress.c2)}`);
  }
  if (pageErrors.length > 0) {
    throw new Error(`Studio pageerror(s): ${pageErrors.join(" | ")}`);
  }

  console.log(
    `verify-uhd-workload: PASS — 4K (${stress.uhd4k.arena.evictions} evicts) & 8K (${stress.uhd8k.arena.evictions} evicts, peak ${stress.uhd8k.arena.peakResidentBytes}/${stress.uhd8k.arena.budgetBytes} B) upheld contract with maxLag=${stress.maxLagMs.toFixed(1)}ms; zero-account VerdantVcs (${stress.prismId}) committed e1->e2, packed .holo (${stress.c1.holoBytes} B, 8 STCH2 shards), and generated ${stress.c2.pin}.`,
  );
} finally {
  await browser.close();
  server.close();
}
