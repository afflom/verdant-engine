/**
 * ISO/IEC/IEEE 42010:2022 Viewpoint Purity, Subpath Deployment & Immutability Seal Contract Gates
 * (`enforce-prism-viewpoints.mjs`, owned by `mgr-verification-observatory`).
 */

import { createHash } from "node:crypto";
import { existsSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const ROOT_DIR = join(dirname(fileURLToPath(import.meta.url)), "..");

export function verifySubpathAndUorcOnlyDeploymentContract({
  surfaceSources = [],
  studioSources = [],
}) {
  const rootAbsPattern = /(?:href|src)\s*=\s*["']\/(?!\/)|(?:from|import)\s*["']\/(?!\/)|(?:fetch|Worker|URL)\s*\(\s*["']\/(?!\/)/;
  for (const [label, src] of surfaceSources) {
    const m = rootAbsPattern.exec(src);
    if (m) {
      throw new Error(
        `Subpath violation in ${label}: root-absolute URL "${m[0]}" breaks GitHub Pages subpath deployment (use relative "./" / "../" or new URL(..., import.meta.url))`,
      );
    }
  }

  const tarForbiddenPattern = /buildPosixTarArchive|application\/x-tar|POSIX[^.\n]{0,20}\.tar|\bExport Project \(\.tar\)|Download \.tar/i;
  for (const [label, src] of studioSources) {
    const m = tarForbiddenPattern.exec(src);
    if (m) {
      throw new Error(
        `Archive policy violation in ${label}: forbidden .tar export token "${m[0]}" (Verdant exclusively uses .uorc archives)`,
      );
    }
  }

  const studioJoined = studioSources.map(([, s]) => s).join("\n");
  for (const req of ["studio-export-uorc-btn", "ve-scaf-uorc-btn", "encode-snapshot"]) {
    if (!studioJoined.includes(req)) {
      throw new Error(`Verdant Studio missing required .uorc export integration "${req}"`);
    }
  }
}

export function verifyContextPurityAndIntentionalInterfaceContract({
  rootDir = ROOT_DIR,
  cardHtml,
  studioPageHtml,
  catalogsHtml,
  demoHtml,
  demoSource,
  starterHtml = "",
}) {
  const vsmCat = JSON.parse(
    readFileSync(join(rootDir, "catalogs", "governance", "vsm-viable-system.json"), "utf8"),
  );
  const vpPath = join(rootDir, "site", "generated", "viewpoint-contracts.json");
  const vpCat = existsSync(vpPath)
    ? JSON.parse(readFileSync(vpPath, "utf8"))
    : JSON.parse(readFileSync(join(rootDir, "catalogs", "governance", "iso42010-viewpoints.json"), "utf8"));
  if (!vsmCat?.antiContrivanceRule?.requireManagerOwnership || !Array.isArray(vpCat?.managers)) {
    throw new Error("Gate C13: locked VSM / ISO 42010 governance catalogs missing or malformed");
  }
  if (existsSync(vpPath)) {
    if (vpCat.spec !== "verdant/manager-viewpoint-contracts/1" || vpCat.totalOwnershipComplete !== true) {
      throw new Error("Gate C13: site/generated/viewpoint-contracts.json failed Manager ownership contract");
    }
  }
  const worldsPath = join(rootDir, "site", "generated", "world-differentiation-profiles.json");
  if (existsSync(worldsPath)) {
    const wp = JSON.parse(readFileSync(worldsPath, "utf8"));
    if (wp.spec !== "verdant/world-differentiation-profiles/1" || !Array.isArray(wp.worlds) || wp.worlds.length < 4) {
      throw new Error("Gate C13 (mgr-playable-world): site/generated/world-differentiation-profiles.json must define all 4 differentiated worlds");
    }
  }
  const stitchPath = join(rootDir, "site", "generated", "stitch-app-catalog.json");
  if (existsSync(stitchPath)) {
    const sc = JSON.parse(readFileSync(stitchPath, "utf8"));
    if (sc.spec !== "verdant/stitch-app-catalog/1" || !Array.isArray(sc.apps) || sc.apps.length < 4) {
      throw new Error("Gate C13 (mgr-catalog-curator): site/generated/stitch-app-catalog.json must publish >= 4 Stitch 2-of-3 Verdant Apps");
    }
  }
  const expJsonPath = join(rootDir, "site", "generated", "experience-design-system.json");
  if (existsSync(expJsonPath)) {
    const exp = JSON.parse(readFileSync(expJsonPath, "utf8"));
    if (
      exp.spec !== "verdant/experience-design-system/1" ||
      exp.owningManager !== "mgr-design-coordinator" ||
      !Array.isArray(exp.studioJourney?.steps) ||
      exp.studioJourney.steps.length !== 3
    ) {
      throw new Error("Gate C13 (mgr-design-coordinator): site/generated/experience-design-system.json must define the 3-step guided Studio journey");
    }
  }
  const expCssPath = join(rootDir, "site", "generated", "studio-experience.css");
  if (existsSync(expCssPath)) {
    const expCss = readFileSync(expCssPath, "utf8");
    if (/#[0-9a-fA-F]{3,8}\b/.test(expCss)) {
      throw new Error("Gate C13 (mgr-design-coordinator): site/generated/studio-experience.css must use DTCG var(--...) tokens (zero raw #hex)");
    }
    if (/backdrop-filter\s*:\s*blur/i.test(expCss)) {
      throw new Error("Gate C13 (mgr-design-coordinator): site/generated/studio-experience.css must not use backdrop-filter blur");
    }
  }
  const sealPath = join(rootDir, "site", "generated", "immutability-seal.json");
  if (existsSync(sealPath)) {
    const seal = JSON.parse(readFileSync(sealPath, "utf8"));
    if (seal.spec !== "verdant/view-immutability-seal/1" || !Array.isArray(seal.files) || seal.files.length < 11) {
      throw new Error("Gate C13 (mgr-observer-executive): site/generated/immutability-seal.json malformed or incomplete");
    }
    const sorted = [...seal.files].sort((a, b) => a.path.localeCompare(b.path));
    const rootInput = sorted.map((f) => `${f.path}|${f.owningManagerId}|${f.managerKappaHex}|${f.sha256Hex}\n`).join("");
    const computedRoot = createHash("sha256").update(rootInput).digest("hex");
    if (computedRoot !== seal.sealRootSha256Hex) {
      throw new Error(`Gate C13 (mgr-observer-executive): immutability seal root tamper detected (expected ${computedRoot}, found ${seal.sealRootSha256Hex})`);
    }
    for (const f of seal.files) {
      const abs = join(rootDir, f.path);
      if (!existsSync(abs)) {
        throw new Error(`Gate C13 (immutability-seal): missing sealed view "${f.path}"`);
      }
      const obsSha = createHash("sha256").update(readFileSync(abs)).digest("hex");
      if (obsSha !== f.sha256Hex) {
        throw new Error(
          `Gate C13 (immutability-seal): out-of-band manual edit detected on immutable view "${f.path}" (owner ${f.owningManagerId}) — modify only via Verdant API/MCP`,
        );
      }
    }
  }
  const htmlByViewpoint = {
    "runtime-stage": ["site/card.html", cardHtml],
    "creator-studio": ["site/studio/index.html", studioPageHtml],
    "asset-catalogs": ["site/catalogs/index.html", catalogsHtml],
    "verification-observatory": ["site/demo/index.html", demoHtml],
    ...(starterHtml ? { "playable-world": ["templates/browser-3d-starter/index.html", starterHtml] } : {}),
  };
  for (const rawMgr of vpCat.managers) {
    const mgr = rawMgr.spec || rawMgr;
    const entry = htmlByViewpoint[mgr.viewpointId];
    if (!entry) continue;
    const [label, html] = entry;
    for (const req of mgr.requiredElements || []) {
      if (!html.includes(req)) {
        throw new Error(
          `Gate C13 (${mgr.managerId}): ${label} missing required Viewpoint element "${req}"`,
        );
      }
    }
    for (const term of mgr.forbiddenCrossContamination || []) {
      if (html.includes(term)) {
        throw new Error(
          `Gate C13 (${mgr.managerId}): ${label} contains forbidden cross-contamination "${term}"`,
        );
      }
    }
  }
  const studioSearchInputs = (studioPageHtml.match(/type\s*=\s*["']search["']/gi) || []).length;
  if (studioSearchInputs !== 1) {
    throw new Error(
      `Gate C13 (mgr-studio-alm): site/studio/index.html must expose exactly 1 unified search input (found ${studioSearchInputs})`,
    );
  }
  if (
    !demoSource.includes("verdant_wasm_uhd_cull_meshlets") ||
    !demoSource.includes("verdant_wasm_frame_stability_aux")
  ) {
    throw new Error(
      "Gate C13 (mgr-verification-observatory): UHD 4K/8K, Morton meshlet, LOD pyramid & Frame Stability Oracle benchmark must live in site/demo/demo.mjs",
    );
  }
}
