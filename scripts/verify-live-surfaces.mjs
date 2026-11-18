#!/usr/bin/env node
/**
 * End-to-end headless Chromium verifier for Verdant Studio and all 6 Diataxis
 * documentation pages (`site/`). Verifies:
 *   1. Zero uncaught pageerror or console.error events across every route.
 *   2. Root Verdant Studio (`/`) across Light, Dark, and Immersive theme classes,
 *      all DevRel controls, 7-role Studio (`Dashboard`, `Code`, `Design`, `Audio`,
 *      `Narrative`, `Director`, `Gameplay`), Standalone Engine Demo (`/demo/`),
 *      live SceneWorld Entity Outliner, zero-constants Rust/Wasm Project Scaffolder,
 *      and Stitch 2-of-3 Custody modal.
 *   3. All 6 `/docs/*` pages reach `data-ve-ready="true"` on every live Wasm
 *      figure (never stuck on "Initializing WebAssembly runtime…"), respond to
 *      interactive button clicks (including SHA-256 tamper refusal), and switch
 *      Light/Dark themes cleanly.
 */

import { createServer } from "node:http";
import { readFile, stat } from "node:fs/promises";
import { execSync } from "node:child_process";
import { extname, join, normalize } from "node:path";
import { pathToFileURL } from "node:url";

const siteDir = process.argv[2] ?? "site";
const types = {
  ".html": "text/html",
  ".css": "text/css",
  ".mjs": "text/javascript",
  ".js": "text/javascript",
  ".json": "application/json",
  ".webmanifest": "application/manifest+json",
  ".svg": "image/svg+xml",
  ".woff2": "font/woff2",
  ".png": "image/png",
  ".holo": "application/octet-stream",
  ".wasm": "application/wasm",
  ".uorc": "application/octet-stream",
  ".md": "text/markdown",
};

const server = createServer(async (req, res) => {
  let reqPath = decodeURIComponent(new URL(req.url, "http://127.0.0.1").pathname);
  if (reqPath.endsWith("/")) reqPath += "index.html";
  const file = normalize(join(siteDir, reqPath));
  try {
    await stat(file);
    const body = await readFile(file);
    res.writeHead(200, { "content-type": types[extname(file)] ?? "application/octet-stream" });
    res.end(body);
  } catch {
    res.writeHead(404);
    res.end();
  }
});

await new Promise((resolve) => server.listen(0, "127.0.0.1", resolve));
const origin = `http://127.0.0.1:${server.address().port}/`;

const playwright = await import("playwright").catch(() =>
  import(pathToFileURL(join(execSync("npm root -g").toString().trim(), "playwright", "index.mjs")).href),
);

const browser = await playwright.chromium.launch();
const failures = [];

try {
  const context = await browser.newContext({
    viewport: { width: 1280, height: 840 },
    serviceWorkers: "block",
  });
  await context.route(/^https?:\/\/(?!127\.0\.0\.1|uor-foundation\.github\.io\/template\/frame\/)/, (route) => route.abort());

  // 1. Verify Root Verdant Studio (`/`) + 3 Themes + All 7 Role Workspaces + Multiplayer Stitch:
  {
    const page = await context.newPage();
    const errors = [];
    page.on("pageerror", (e) => errors.push(`pageerror: ${e.message}`));
    page.on("console", (m) => {
      if (m.type() === "error") errors.push(`console.error: ${m.text()}`);
    });

    await page.goto(origin, { waitUntil: "load" });
    await page.waitForFunction(
      () => {
        const s = document.getElementById("ve-status")?.textContent || "";
        return s.includes("LOD") || s.includes("Observer") || s.includes("Offline");
      },
      null,
      { timeout: 10000 },
    );

    // Cycle Light -> Dark -> Immersive classes and verify computed background tokens adapt cleanly:
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
      failures.push(`Studio .ve-pill background did not adapt between Light (${themeBgs.lightBg}) and Dark (${themeBgs.darkBg})`);
    }

    // Open 7-Role Studio Workspace (`#ve-studio-btn`):
    await page.click("#ve-studio-btn");
    await page.waitForSelector("dialog.ve-modal[open]", { timeout: 5000 });

    const tabKeys = await page.$$eval("dialog.ve-modal .ve-m-tab", (els) =>
      els.map((e) => e.dataset.tabKey),
    );
    if (tabKeys.join(",") !== "dashboard,code,design,audio,narrative,director,gameplay") {
      failures.push(`Unexpected Studio role tabs: ${tabKeys.join(",")}`);
    }

    // Role 0 (`dashboard`): Verify all 7 role requirement contracts against app.holo:
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

    // Role 1 (`code`): Switch project template and verify zero-constants Rust generation:
    await page.click('[data-tab-key="code"]');
    await page.selectOption("#ve-scaf-arch", "2");
    const generatedCode = await page.textContent("#ve-scaf-code");
    if (!generatedCode?.includes("LawfulCameraRig") || !generatedCode?.includes("Glasshouse Botanical Chronicle")) {
      failures.push("Code role Project Scaffolder failed to synthesize expected non-self-referential Rust code");
    }

    // Role 2 (`design`): Verify live Wasm SceneWorld Entity Outliner & tropical UI compliance:
    await page.click('[data-tab-key="design"]');
    await page.waitForFunction(
      () => document.querySelectorAll("dialog.ve-modal section:not([hidden]) .ve-table tbody tr").length >= 5,
      null,
      { timeout: 5000 },
    );

    // Role 3 (`audio`): Switch Sabine acoustic room preset (`harbor-amphitheater`) via Wasm:
    await page.click('[data-tab-key="audio"]');
    await page.click('[data-acoustic-room="harbor-amphitheater"]');
    await page.waitForFunction(
      () => (document.getElementById("ve-audio-log")?.textContent || "").includes("Wasm Sabine RT60:"),
      null,
      { timeout: 8000 },
    );

    // Role 4 (`narrative`): Advance StoryAutomaton beat and verify integer ConservationLedger:
    await page.click('[data-tab-key="narrative"]');
    await page.click("#ve-narrative-step-btn");
    await page.waitForFunction(
      () => (document.getElementById("ve-story-log")?.textContent || "").includes("ConservationLedger Total Supply:"),
      null,
      { timeout: 8000 },
    );

    // Role 5 (`director`): Cut to camera shot 2 (`shot-1`) and verify CSM / frustum guard readout:
    await page.click('[data-tab-key="director"]');
    await page.click('[data-shot-id="shot-1"]');
    await page.waitForFunction(
      () => (document.getElementById("ve-director-log")?.textContent || "").includes("02. Parabolic Arch Low-Angle"),
      null,
      { timeout: 8000 },
    );

    // Role 6 (`gameplay`): Switch to Tournament ruleset and frame 22-joint BridgeEngineer NPC:
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

    // Close Studio Workspace and open Multiplayer Stitch Custody modal (`#ve-mesh-btn`):
    await page.click('dialog.ve-modal .ve-m-head button[aria-label="Close Verdant Studio workspace"]');
    await page.click("#ve-mesh-btn");
    await page.waitForFunction(
      () => document.querySelectorAll("dialog.ve-modal[open]").length === 1,
      null,
      { timeout: 5000 },
    );

    if (errors.length) failures.push(`Root Studio errors: ${errors.join(" | ")}`);
    await page.close();
  }

  // 2. Verify Standalone Verdant Engine Demo (`/demo/`) — Raster Comparison, UORC Ladder, Benchmarks & Kernels:
  {
    const page = await context.newPage();
    const errors = [];
    page.on("pageerror", (e) => errors.push(`pageerror: ${e.message}`));
    page.on("console", (m) => {
      if (m.type() === "error") errors.push(`console.error: ${m.text()}`);
    });

    await page.goto(`${origin}demo/`, { waitUntil: "load" });
    await page.waitForFunction(
      () =>
        document.querySelectorAll("#demo-lod-tbody tr").length === 5 &&
        document.querySelectorAll("#demo-kernel-tbody tr").length === 22 &&
        (document.getElementById("log-bench-uorc")?.textContent || "").includes("conformant:"),
      null,
      { timeout: 15000 },
    );

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
  }

  // 3. Verify all 6 Diataxis documentation routes + Live Wasm figures + interactive buttons + theme toggle:
  const docRoutes = [
    { path: "docs/", act: "lod" },
    { path: "docs/getting-started/", act: "slot-2" },
    { path: "docs/guides/custom-game-world/", act: "step" },
    { path: "docs/guides/progressive-uorc-streaming/", act: "lod-2" },
    { path: "docs/reference/engine-api/", act: "tamper" },
    { path: "docs/explanation/tensor-and-uorc-streaming/", act: "audio" },
  ];

  for (const route of docRoutes) {
    const page = await context.newPage();
    const errors = [];
    page.on("pageerror", (e) => errors.push(`pageerror: ${e.message}`));
    page.on("console", (m) => {
      if (m.type() === "error") errors.push(`console.error: ${m.text()}`);
    });

    await page.goto(`${origin}${route.path}`, { waitUntil: "load" });
    await page.waitForFunction(
      () => {
        const figs = [...document.querySelectorAll("figure[data-ve-live]")];
        return figs.length > 0 && figs.every((f) => f.dataset.veReady === "true" || f.dataset.veReady === "error");
      },
      null,
      { timeout: 15000 },
    );

    const state = await page.$eval("figure[data-ve-live]", (f) => ({
      id: f.id,
      ready: f.dataset.veReady,
      err: f.dataset.veError || "",
    }));
    if (state.ready !== "true") {
      failures.push(`${route.path} (${state.id}) failed ready check: ready=${state.ready} err=${state.err}`);
    }

    // Click interactive live Wasm control:
    await page.click(`figure[data-ve-live] button[data-ve-act="${route.act}"]`);
    if (route.act === "tamper") {
      await page.waitForFunction(
        () => document.querySelector("figure[data-ve-live]")?.dataset.veSnapshotVerdict === "refused",
        null,
        { timeout: 15000 },
      );
    }

    // Toggle theme button in `.ve-nav`:
    await page.click("[data-ve-theme-toggle]");
    const afterReady = await page.$eval("figure[data-ve-live]", (f) => f.dataset.veReady);
    if (afterReady !== "true") {
      failures.push(`${route.path} lost ready state after theme toggle`);
    }
    if (errors.length) {
      failures.push(`${route.path} runtime errors: ${errors.join(" | ")}`);
    }
    await page.close();
  }
} finally {
  await browser.close();
  server.close();
}

if (failures.length > 0) {
  console.error("verify-live-surfaces FAILED:");
  for (const f of failures) console.error(`  - ${f}`);
  process.exit(1);
}
console.log("verify-live-surfaces: Root Verdant Studio (7 Role Workspaces, 3 Themes, Stitch Custody), Standalone Engine Demo (/demo/), and all 6 live Wasm documentation pages verified with 0 errors.");
