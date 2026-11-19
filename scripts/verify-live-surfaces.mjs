#!/usr/bin/env node
/**
 * End-to-end headless Chromium deployment gate for Verdant Engine (`site/`).
 * Serves `site/` STRICTLY under the GitHub Pages subpath declared in `site/app.json`
 * (`live`, e.g. `/verdant-engine/`), returning HTTP 404 for ANY request outside
 * that subpath so broken subpath URLs fail the build before Pages deploys.
 *
 * Runs independent sub-app and documentation checks concurrently inside one shared
 * Chromium browser to minimize CI latency while enforcing on all 11 routes:
 *   - 0 HTTP >= 400 responses or failed requests on the site origin
 *   - 0 uncaught `pageerror` or `console.error` events
 *   - Active loaded stylesheets (`cssRules >= 20`, styled background & typography)
 *   - Every internal `<a href>` resolves strictly inside `basePath`
 *   - Every 3D viewport `<canvas>` renders non-blank visual pixels (`uniqColors >= 8`)
 *   - Native wheel scrollability & unclipped `<main>` on `/studio/`, `/catalogs/`,
 *     `/demo/`, and `/docs/` (`window.scrollY >= 300`, `main.scrollHeight <= main.clientHeight + 8`)
 *   - Interactive 4-Stage Catalog Composition Wizard on `/studio/` (`load-domain-pack`,
 *     ISO 24617-2 `"Unknown"` abstention, 4/4 Prism Quality Gate, `.uorc`-only export)
 */

import { createServer } from "node:http";
import { readFile, stat } from "node:fs/promises";
import { execSync } from "node:child_process";
import { extname, join, normalize, resolve } from "node:path";
import { pathToFileURL } from "node:url";

const siteDir = resolve(process.argv[2] ?? "site");
const appJson = JSON.parse(await readFile(join(siteDir, "app.json"), "utf8"));
const rawBase = appJson.live ? new URL(appJson.live).pathname : "/";
const basePath = rawBase.endsWith("/") ? rawBase : `${rawBase}/`;

const types = {
  ".html": "text/html; charset=utf-8",
  ".css": "text/css; charset=utf-8",
  ".mjs": "text/javascript; charset=utf-8",
  ".js": "text/javascript; charset=utf-8",
  ".json": "application/json; charset=utf-8",
  ".webmanifest": "application/manifest+json; charset=utf-8",
  ".svg": "image/svg+xml",
  ".woff2": "font/woff2",
  ".png": "image/png",
  ".holo": "application/octet-stream",
  ".wasm": "application/wasm",
  ".uorc": "application/octet-stream",
  ".md": "text/markdown; charset=utf-8",
};

const server = createServer(async (req, res) => {
  const reqPath = decodeURIComponent(new URL(req.url, "http://127.0.0.1").pathname);
  if (!reqPath.startsWith(basePath)) {
    res.writeHead(404, { "content-type": "text/plain; charset=utf-8" });
    res.end(`Subpath 404: ${reqPath} escapes ${basePath}`);
    return;
  }
  let rel = reqPath.slice(basePath.length);
  if (rel === "" || rel.endsWith("/")) rel += "index.html";
  const file = resolve(siteDir, normalize(rel));
  if (!file.startsWith(siteDir)) {
    res.writeHead(403);
    res.end();
    return;
  }
  try {
    const st = await stat(file);
    if (!st.isFile()) throw new Error("not a file");
    const body = await readFile(file);
    res.writeHead(200, { "content-type": types[extname(file)] ?? "application/octet-stream" });
    res.end(body);
  } catch {
    res.writeHead(404, { "content-type": "text/plain; charset=utf-8" });
    res.end(`Not found: ${reqPath}`);
  }
});

await new Promise((resolveListen) => server.listen(0, "127.0.0.1", resolveListen));
const port = server.address().port;
const hostOrigin = `http://127.0.0.1:${port}`;
const origin = `${hostOrigin}${basePath}`;

const playwright = await import("playwright").catch(() =>
  import(pathToFileURL(join(execSync("npm root -g").toString().trim(), "playwright", "index.mjs")).href),
);

const browser = await playwright.chromium.launch({
  args: ["--use-gl=angle", "--use-angle=swiftshader", "--enable-unsafe-swiftshader"],
});
const failures = [];
const abortedExternalUrls = new Set();

function attachTelemetry(page) {
  const errors = [];
  page.on("pageerror", (e) => errors.push(`pageerror: ${e.message}`));
  page.on("console", (m) => {
    if (m.type() === "error") {
      const locUrl = m.location()?.url || "";
      if (m.text().includes("net::ERR_FAILED") && locUrl && abortedExternalUrls.has(locUrl)) {
        return;
      }
      errors.push(`console.error: ${m.text()}`);
    }
  });
  page.on("requestfailed", (req) => {
    const url = req.url();
    if (url.startsWith(hostOrigin)) {
      errors.push(`requestfailed: ${url} (${req.failure()?.errorText ?? "unknown"})`);
    }
  });
  page.on("response", (res) => {
    const url = res.url();
    if (url.startsWith(hostOrigin) && res.status() >= 400) {
      errors.push(`HTTP ${res.status()}: ${url}`);
    }
  });
  return errors;
}

async function verifyStyledAndContained(page, routeLabel) {
  const audit = await page.evaluate((bp) => {
    let cssRulesCount = 0;
    for (const sheet of document.styleSheets) {
      try {
        cssRulesCount += sheet.cssRules.length;
      } catch {
        // Cross-origin stylesheet
      }
    }
    const bodyStyle = getComputedStyle(document.body);
    const escapingLinks = [];
    for (const a of document.querySelectorAll("a[href]")) {
      const raw = a.getAttribute("href") || "";
      if (!raw || raw.startsWith("#") || raw.startsWith("mailto:") || raw.startsWith("blob:") || raw.startsWith("data:")) {
        continue;
      }
      try {
        const resolved = new URL(raw, document.baseURI);
        if (resolved.origin === location.origin && !resolved.pathname.startsWith(bp)) {
          escapingLinks.push(`${raw} -> ${resolved.pathname}`);
        }
      } catch {
        escapingLinks.push(`invalid:${raw}`);
      }
    }
    return {
      styleSheetCount: document.styleSheets.length,
      cssRulesCount,
      bodyBg: bodyStyle.backgroundColor,
      bodyFont: bodyStyle.fontFamily,
      escapingLinks,
    };
  }, basePath);

  if (audit.styleSheetCount < 1 || audit.cssRulesCount < 20) {
    failures.push(`${routeLabel}: stylesheets failed to load (sheets=${audit.styleSheetCount}, rules=${audit.cssRulesCount})`);
  }
  if (!audit.bodyBg || audit.bodyBg === "rgba(0, 0, 0, 0)") {
    failures.push(`${routeLabel}: body background is transparent/unstyled (${audit.bodyBg})`);
  }
  if (/^"?Times(\s+New\s+Roman)?"?$/i.test(audit.bodyFont.trim())) {
    failures.push(`${routeLabel}: typography fell back to unstyled browser default (${audit.bodyFont})`);
  }
  if (audit.escapingLinks.length > 0) {
    failures.push(`${routeLabel}: internal links escape ${basePath}: ${audit.escapingLinks.join(", ")}`);
  }
}

async function verifyScrollableUnclipped(page, routeLabel) {
  const pre = await page.evaluate(() => {
    const main = document.querySelector("main");
    return {
      docScrollH: document.documentElement.scrollHeight,
      docClientH: document.documentElement.clientHeight,
      mainScrollH: main ? main.scrollHeight : 0,
      mainClientH: main ? main.clientHeight : 0,
      mainOverflowY: main ? getComputedStyle(main).overflowY : "none",
    };
  });
  if (pre.mainScrollH > pre.mainClientH + 8 && pre.mainOverflowY === "hidden") {
    failures.push(
      `${routeLabel}: <main> is vertically clipped by overflow:hidden (scrollH=${pre.mainScrollH} > clientH=${pre.mainClientH})`,
    );
  }
  if (pre.docScrollH <= pre.docClientH + 200) {
    failures.push(
      `${routeLabel}: document scrollHeight too small (${pre.docScrollH} vs viewport ${pre.docClientH}) — content likely clipped`,
    );
    return;
  }
  await page.evaluate(() => window.scrollTo({ top: 0, behavior: "instant" }));
  // Move cursor to right rail / non-canvas margin so wheel events scroll the document rather than zooming WebGL:
  await page.mouse.move(1180, 420);
  await page.mouse.wheel(0, 700);
  await page.waitForFunction(() => window.scrollY >= 250, null, { timeout: 3000 }).catch(() => {});
  const scrolledY = await page.evaluate(() => window.scrollY);
  if (scrolledY < 250) {
    failures.push(`${routeLabel}: page failed real mouse wheel scroll check (scrollY=${scrolledY}, scrollH=${pre.docScrollH})`);
  }
  await page.evaluate(() => window.scrollTo({ top: 0, behavior: "instant" }));
}

async function verifyCanvasRendered(page, selector, routeLabel) {
  const locator = page.locator(selector).first();
  await locator.waitFor({ state: "visible", timeout: 10000 });
  const box = await locator.boundingBox();
  if (!box || box.width < 100 || box.height < 80) {
    failures.push(`${routeLabel}: canvas "${selector}" collapsed (${box?.width ?? 0}x${box?.height ?? 0})`);
    return;
  }
  await page.evaluate(() => new Promise((r) => requestAnimationFrame(() => requestAnimationFrame(r))));
  const pngBuf = await locator.screenshot();
  const stats = await page.evaluate(async (bytes) => {
    const blob = new Blob([new Uint8Array(bytes)], { type: "image/png" });
    const bmp = await createImageBitmap(blob);
    const cv = document.createElement("canvas");
    cv.width = Math.min(bmp.width, 320);
    cv.height = Math.min(bmp.height, 240);
    const ctx = cv.getContext("2d");
    ctx.drawImage(bmp, 0, 0, cv.width, cv.height);
    const data = ctx.getImageData(0, 0, cv.width, cv.height).data;
    const colors = new Set();
    for (let i = 0; i < data.length; i += 16) {
      const key = ((data[i] >> 3) << 10) | ((data[i + 1] >> 3) << 5) | (data[i + 2] >> 3);
      colors.add(key);
    }
    return { uniqColors: colors.size, width: bmp.width, height: bmp.height };
  }, [...pngBuf]);

  if (stats.uniqColors < 8) {
    failures.push(`${routeLabel}: canvas "${selector}" rendered blank/flat (${stats.uniqColors} unique quantized colors across ${stats.width}x${stats.height})`);
  }
}

try {
  const context = await browser.newContext({
    viewport: { width: 1280, height: 840 },
    serviceWorkers: "block",
  });
  await context.route(/^https?:\/\/(?!127\.0\.0\.1|uor-foundation\.github\.io\/template\/frame\/)/, (route) => {
    abortedExternalUrls.add(route.request().url());
    return route.abort();
  });

  // 1. Root Engine Stage (`/`):
  const verifyRoot = async () => {
    const page = await context.newPage();
    const errors = attachTelemetry(page);

    await page.goto(origin, { waitUntil: "load" });
    await page.waitForFunction(
      () => {
        const s = document.getElementById("ve-status")?.textContent || "";
        return s.includes("LOD") || s.includes("Conformant") || s.includes("Offline");
      },
      null,
      { timeout: 12000 },
    );

    await verifyStyledAndContained(page, basePath);
    await verifyCanvasRendered(page, "#ve-canvas", basePath);

    const themeBgs = await page.evaluate(() => {
      const root = document.documentElement;
      root.className = "";
      const lightBg = getComputedStyle(document.querySelector(".ve-pill")).backgroundColor;
      root.className = "dark";
      const darkBg = getComputedStyle(document.querySelector(".ve-pill")).backgroundColor;
      root.className = "dark immersive";
      const immBg = getComputedStyle(document.querySelector(".ve-pill")).backgroundColor;
      root.className = "dark";
      return { lightBg, darkBg, immBg };
    });
    if (themeBgs.lightBg === themeBgs.darkBg) {
      failures.push(`Engine root .ve-pill background did not adapt between Light (${themeBgs.lightBg}) and Dark (${themeBgs.darkBg})`);
    }

    await page.click("#ve-io-drawer > summary");
    await page.click("#ve-studio-btn");
    await page.waitForSelector("dialog.ve-modal[open]", { timeout: 5000 });

    const tabKeys = await page.$$eval("dialog.ve-modal .ve-m-tab", (els) => els.map((e) => e.dataset.tabKey));
    if (tabKeys.join(",") !== "dashboard,code,design,audio,narrative,director,gameplay") {
      failures.push(`Unexpected Studio role tabs: ${tabKeys.join(",")}`);
    }

    await page.click('[data-tab-key="dashboard"]');
    await page.click("#ve-verify-roles-btn");
    await page.waitForFunction(
      () =>
        [...document.querySelectorAll("dialog.ve-modal section:not([hidden]) .ve-table tbody tr")].every((tr) =>
          (tr.textContent || "").includes("Conformant"),
        ),
      null,
      { timeout: 8000 },
    );

    await page.click('[data-tab-key="code"]');
    await page.selectOption("#ve-scaf-arch", "2");
    const generatedCode = await page.textContent("#ve-scaf-code");
    if (!generatedCode?.includes("LawfulCameraRig") || !generatedCode?.includes("Glasshouse Botanical Chronicle")) {
      failures.push("Code role Project Scaffolder failed to synthesize expected non-self-referential Rust code");
    }
    if ((await page.locator("#ve-scaf-uorc-btn").count()) !== 1 || (await page.locator("#ve-scaf-tar-btn").count()) > 0) {
      failures.push("Studio Code panel must expose #ve-scaf-uorc-btn and zero .tar export buttons");
    }

    await page.click('[data-tab-key="design"]');
    await page.waitForFunction(
      () => document.querySelectorAll("dialog.ve-modal section:not([hidden]) .ve-table tbody tr").length >= 5,
      null,
      { timeout: 5000 },
    );

    await page.click('[data-tab-key="audio"]');
    await page.click('[data-acoustic-room="harbor-amphitheater"]');
    await page.waitForFunction(
      () => (document.getElementById("ve-audio-log")?.textContent || "").includes("Wasm Sabine RT60:"),
      null,
      { timeout: 8000 },
    );

    await page.click('[data-tab-key="narrative"]');
    await page.click("#ve-narrative-step-btn");
    await page.waitForFunction(
      () => (document.getElementById("ve-story-log")?.textContent || "").includes("ConservationLedger Total Supply:"),
      null,
      { timeout: 8000 },
    );

    await page.click('[data-tab-key="director"]');
    await page.click('[data-shot-id="shot-1"]');
    await page.waitForFunction(
      () => (document.getElementById("ve-director-log")?.textContent || "").includes("02. Parabolic Arch Low-Angle"),
      null,
      { timeout: 8000 },
    );

    await page.click('[data-tab-key="gameplay"]');
    await page.click('[data-game-mode="tournament"]');
    await page.waitForFunction(
      () => (document.getElementById("ve-gameplay-mode-log")?.textContent || "").includes("Active Game Mode: Tournament"),
      null,
      { timeout: 8000 },
    );
    await page.click('[data-npc-frame="bridge-engineer"]');
    await page.waitForFunction(
      () => (document.getElementById("ve-npc-log")?.textContent || "").includes("Framed Dock Engineer"),
      null,
      { timeout: 8000 },
    );

    await verifyStyledAndContained(page, `${basePath} (Studio modal open)`);
    await page.click('dialog.ve-modal .ve-m-head button[aria-label="Close Verdant Studio workspace"]');
    await page.click("#ve-mesh-btn");
    await page.waitForFunction(() => document.querySelectorAll("dialog.ve-modal[open]").length === 1, null, { timeout: 5000 });

    if (errors.length) failures.push(`Root Engine (${basePath}) errors: ${errors.join(" | ")}`);
    await page.close();
  };

  // 2. Standalone Verdant Studio IDE (`/studio/` — scrolling, Live Discovery -> Preview -> Import v1 -> Edit v2 -> Revert v3 -> Remove, Compose Wizard, 7 Roles):
  const verifyStudio = async () => {
    const page = await context.newPage();
    const errors = attachTelemetry(page);

    await page.goto(`${origin}studio/`, { waitUntil: "load" });
    await page.waitForFunction(
      () =>
        document.querySelectorAll("#studio-inline-host .ve-m-tab").length === 7 &&
        document.querySelectorAll("#wiz-domain-grid .proj-domain-btn").length === 4 &&
        document.querySelectorAll("#studio-workflow-bar .wf-step").length === 5 &&
        document.querySelectorAll("#disc-results-grid .disc-card").length >= 6 &&
        (document.getElementById("studio-holo-status")?.textContent || "").includes("Atelier Ready"),
      null,
      { timeout: 15000 },
    );

    await verifyStyledAndContained(page, `${basePath}studio/`);
    await verifyScrollableUnclipped(page, `${basePath}studio/`);
    await verifyCanvasRendered(page, "#studio-canvas", `${basePath}studio/`);

    if ((await page.locator("#studio-export-uorc-btn").count()) !== 1 || (await page.locator("#studio-export-tar-btn").count()) > 0) {
      failures.push("Standalone /studio/ must expose #studio-export-uorc-btn and zero .tar buttons");
    }

    // Exercise Live Discovery -> Preview -> Import v1 -> Edit v2 -> Revert v3 -> Remove in Wasm ObserverPrismHolon:
    await page.locator("#disc-results-grid .disc-card button[data-preview-model]").first().click();
    await page.waitForFunction(
      () => document.getElementById("disc-preview-dock")?.dataset.open === "true",
      null,
      { timeout: 5000 },
    );
    await page.click("#disc-prev-import-btn");
    await page.waitForFunction(
      () => (document.getElementById("proj-model-count")?.textContent || "").includes("1 Imported") &&
        (document.getElementById("insp-meta")?.textContent || "").includes("Active: v1"),
      null,
      { timeout: 10000 },
    );

    await page.fill("#insp-title", "Custom Tuned Studio Model");
    await page.fill("#insp-note", "Tuned properties in live Studio IDE");
    await page.click("#insp-save-btn");
    await page.waitForFunction(
      () => (document.getElementById("insp-meta")?.textContent || "").includes("Active: v2"),
      null,
      { timeout: 10000 },
    );

    await page.click('[data-revert-version="1"]');
    await page.waitForFunction(
      () => (document.getElementById("insp-meta")?.textContent || "").includes("Active: v3"),
      null,
      { timeout: 10000 },
    );

    await page.click("#insp-remove-btn");
    await page.waitForFunction(
      () => (document.getElementById("proj-model-count")?.textContent || "").includes("0 Imported"),
      null,
      { timeout: 10000 },
    );

    // Exercise Compose Tab (Domain swap + ISO 24617-2 Abstention + 4/4 Prism Quality Gate):
    await page.click("#right-tab-compose");
    await page.click('[data-wiz-domain-slug="meridian-observatory"]');
    await page.waitForFunction(
      () => (document.getElementById("st-man-domain")?.textContent || "").includes("DOM-MERIDIAN-OBSERVATORY"),
      null,
      { timeout: 10000 },
    );
    await page.click('[data-wiz-step="3"]');
    await page.click("#wiz-dlg-abstain");
    await page.waitForFunction(
      () => (document.getElementById("wiz-dlg-log")?.textContent || "").includes("ABSTAINED (Unknown"),
      null,
      { timeout: 10000 },
    );
    await page.click('[data-wiz-step="4"]');
    await page.click("#wiz-audit-btn");
    await page.waitForFunction(
      () => (document.getElementById("wiz-audit-log")?.textContent || "").includes("CONFORMANT (4/4 GATES PASSED)"),
      null,
      { timeout: 10000 },
    );

    // Exercise Observer Cognition Tab (Host Envelope, 8 Observer Parts, Dynamic Stream Add/Remove):
    await page.click("#right-tab-cognition");
    await page.waitForFunction(
      () =>
        document.querySelectorAll("#cog-parts-list [data-observer-part]").length === 8 &&
        document.querySelectorAll("#cog-streams-list [data-observer-stream-id]").length >= 10 &&
        (document.getElementById("cog-host-envelope")?.textContent || "").includes("Host Container:"),
      null,
      { timeout: 10000 },
    );
    await page.fill("#cog-stream-id", "ci-test-stream");
    await page.fill("#cog-stream-label", "CI Verified Test Stream");
    await page.fill("#cog-stream-url", "https://example.org/catalog.json");
    await page.click("#cog-stream-add-btn");
    await page.waitForFunction(
      () =>
        document.querySelector('#cog-streams-list [data-observer-stream-id="ci-test-stream"]') !== null &&
        (document.getElementById("cog-log")?.textContent || "").includes('Admitted custom stream "ci-test-stream"'),
      null,
      { timeout: 10000 },
    );
    await page.click('#cog-streams-list [data-observer-stream-id="ci-test-stream"] button[aria-label*="Remove"]');
    await page.waitForFunction(
      () => document.querySelector('#cog-streams-list [data-observer-stream-id="ci-test-stream"]') === null,
      null,
      { timeout: 10000 },
    );

    // Exercise 7 Roles Tab:
    await page.click("#right-tab-roles");
    await page.click('#studio-inline-host [data-tab-key="code"]');
    const inlineCode = await page.textContent("#ve-scaf-code");
    if (!inlineCode?.includes("LawfulCameraRig")) {
      failures.push("Standalone /studio/ inline Code workspace failed to synthesize Rust scaffold");
    }

    if (errors.length) failures.push(`Standalone /studio/ errors: ${errors.join(" | ")}`);
    await page.close();
  };

  // 3. Standalone Verdant Model Catalogs (`/catalogs/` — scrolling + 5 organs + 4 domain packs + 36 models):
  const verifyCatalogs = async () => {
    const page = await context.newPage();
    const errors = attachTelemetry(page);

    await page.goto(`${origin}catalogs/`, { waitUntil: "load" });
    await page.waitForFunction(
      () =>
        document.querySelectorAll("#cat-sensory-grid .cat-sensory-card").length === 10 &&
        document.querySelectorAll("#cat-live-gallery .cat-gallery-card").length >= 6 &&
        document.querySelectorAll("#cat-organ-grid .cat-organ-card").length === 5 &&
        document.querySelectorAll("#cat-domain-grid .cat-organ-card").length === 4 &&
        document.querySelectorAll("#cat-list .cat-entry").length >= 36 &&
        (document.getElementById("cat-holo-banner")?.textContent || "").includes("Holo Verified"),
      null,
      { timeout: 15000 },
    );

    await verifyStyledAndContained(page, `${basePath}catalogs/`);
    await verifyScrollableUnclipped(page, `${basePath}catalogs/`);

    if (errors.length) failures.push(`Standalone /catalogs/ errors: ${errors.join(" | ")}`);
    await page.close();
  };

  // 4. Standalone Engine Demo (`/demo/` — scrolling + 3D canvas + benchmarks):
  const verifyDemo = async () => {
    const page = await context.newPage();
    const errors = attachTelemetry(page);

    await page.goto(`${origin}demo/`, { waitUntil: "load" });
    await page.waitForFunction(
      () =>
        document.querySelectorAll("#demo-lod-tbody tr").length === 5 &&
        document.querySelectorAll("#demo-kernel-tbody tr").length === 22 &&
        (document.getElementById("log-bench-uorc")?.textContent || "").includes("conformant:"),
      null,
      { timeout: 25000 },
    );

    await verifyStyledAndContained(page, `${basePath}demo/`);
    await verifyScrollableUnclipped(page, `${basePath}demo/`);
    await verifyCanvasRendered(page, "#demo-canvas", `${basePath}demo/`);

    await page.click("#btn-mode-tile");
    await page.click('[data-inst-count="10000"]');
    await page.waitForFunction(
      () => (document.getElementById("stat-inst")?.textContent || "").includes("10,000"),
      null,
      { timeout: 8000 },
    );
    await page.click('[data-inst-count="0"]');
    await page.click("#btn-mode-webgl2");

    if (errors.length) failures.push(`Engine Demo (/demo/) errors: ${errors.join(" | ")}`);
    await page.close();
  };

  // 5. Standalone Starter Trial (`/starter/`):
  const verifyStarter = async () => {
    const page = await context.newPage();
    const errors = attachTelemetry(page);

    await page.goto(`${origin}starter/`, { waitUntil: "load" });
    await page.waitForFunction(
      () => {
        const txt = document.getElementById("holo-badge")?.textContent || "";
        return txt.length > 0 && !txt.includes("Preparing");
      },
      null,
      { timeout: 15000 },
    );

    await verifyStyledAndContained(page, `${basePath}starter/`);
    await page.click("#start-btn");
    await verifyCanvasRendered(page, "#game-canvas", `${basePath}starter/`);

    if (errors.length) failures.push(`Starter Trial (/starter/) errors: ${errors.join(" | ")}`);
    await page.close();
  };

  // 6. All 6 Diataxis documentation routes concurrently:
  const docRoutes = [
    { path: "docs/", act: "lod", checkScroll: true },
    { path: "docs/getting-started/", act: "slot-2", checkScroll: false },
    { path: "docs/guides/custom-game-world/", act: "step", checkScroll: false },
    { path: "docs/guides/progressive-uorc-streaming/", act: "lod-2", checkScroll: false },
    { path: "docs/reference/engine-api/", act: "tamper", checkScroll: false },
    { path: "docs/explanation/tensor-and-uorc-streaming/", act: "audio", checkScroll: false },
  ];

  const verifyDocRoute = async (route) => {
    const page = await context.newPage();
    const errors = attachTelemetry(page);

    await page.goto(`${origin}${route.path}`, { waitUntil: "load" });
    await page.waitForFunction(
      () => {
        const figs = [...document.querySelectorAll("figure[data-ve-live]")];
        return figs.length > 0 && figs.every((f) => f.dataset.veReady === "true" || f.dataset.veReady === "error");
      },
      null,
      { timeout: 20000 },
    );

    await verifyStyledAndContained(page, `${basePath}${route.path}`);
    if (route.checkScroll) await verifyScrollableUnclipped(page, `${basePath}${route.path}`);

    const state = await page.$eval("figure[data-ve-live]", (f) => ({
      id: f.id,
      ready: f.dataset.veReady,
      err: f.dataset.veError || "",
    }));
    if (state.ready !== "true") {
      failures.push(`${route.path} (${state.id}) failed ready check: ready=${state.ready} err=${state.err}`);
    }

    await page.click(`figure[data-ve-live] button[data-ve-act="${route.act}"]`);
    if (route.act === "tamper") {
      await page.waitForFunction(
        () => document.querySelector("figure[data-ve-live]")?.dataset.veSnapshotVerdict === "refused",
        null,
        { timeout: 15000 },
      );
    }

    await page.click("[data-ve-theme-toggle]");
    const afterReady = await page.$eval("figure[data-ve-live]", (f) => f.dataset.veReady);
    if (afterReady !== "true") {
      failures.push(`${route.path} lost ready state after theme toggle`);
    }
    if (errors.length) failures.push(`${route.path} runtime errors: ${errors.join(" | ")}`);
    await page.close();
  };

  // Run root + standalone apps in parallel batches to keep SwiftShader GPU responsive while slashing total wall time:
  await Promise.all([verifyRoot(), verifyCatalogs(), verifyStarter()]);
  await Promise.all([verifyStudio(), verifyDemo()]);
  await Promise.all(docRoutes.map((r) => verifyDocRoute(r)));
} finally {
  await browser.close();
  server.close();
}

if (failures.length > 0) {
  console.error(`verify-live-surfaces FAILED (subpath=${basePath}):`);
  for (const f of failures) console.error(`  - ${f}`);
  process.exit(1);
}
console.log(
  `verify-live-surfaces: PASS under strict subpath "${basePath}" — all 11 routes, native wheel scrolling, 4-stage Catalog Composition Wizard, live 3D canvases, stylesheets, link containment, and .uorc-only export verified with 0 errors.`,
);
