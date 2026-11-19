import test from "node:test";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import {
  runAllPrismContractChecks,
  verifyExternalOraclesAndRuntimeExclusivityContract,
  verifyPlatformDifferentiationAndCatalogsContract,
  verifyPrismHoloContract,
  verifyPrismKernelAlignmentAndCleanRefactorContract,
  verifyRoleDrivenStudioAndDemoSeparationContract,
  verifyRustZeroMagicConstants,
  verifyStarterAndScaffolderContract,
  verifyStudioSurfaceAndZeroHexContract,
  verifySubpathAndUorcOnlyDeploymentContract,
  verifyUnifiedHostAndAudioContract,
  verifyZeroSlmBypassContract,
} from "./scripts/enforce-prism-contracts.mjs";
import {
  PRISM_MODEL_CATALOGS,
  VERDANT_PLATFORM_ORGANS,
  catalogModelsForRole,
  filterCatalogModels,
  parseConformantKeyValues,
  summarizeCatalogCoverage,
} from "./site/prism-catalogs.mjs";
import {
  AUTHORITATIVE_CONTENT_TYPES,
  LIVE_CATALOG_STREAMS,
  searchLiveCatalogStreams,
} from "./site/catalog-streams.mjs";
import * as studioModule from "./site/studio.mjs";
import {
  STARTER_ARCHETYPES,
  STUDIO_ROLE_KEYS,
  generateStarterProjectFiles,
} from "./site/studio.mjs";
import {
  CONTAINER_ORGANS,
  STARTER_ARCHETYPES as DATA_ARCHETYPES,
  STUDIO_ROLE_KEYS as DATA_ROLE_KEYS,
} from "./site/studio-data.mjs";
import {
  base64UrlToBytes,
  bytesToBase64Url,
  zeroOneExpr,
} from "./site/studio-scaffolder.mjs";

const ROOT = fileURLToPath(new URL("./", import.meta.url));

test("C1-C10: all repository Prism component contracts pass end-to-end", async () => {
  const summary = await runAllPrismContractChecks(ROOT);
  assert.ok(summary.vectors >= 46, `expected >=46 Holo acceptance vectors, got ${summary.vectors}`);
  assert.equal(summary.holoQueriesVerified, 65);
  assert.equal(summary.holoRefusalsVerified, 4);
  assert.ok(summary.catalogModelsVerified >= 28);
  assert.equal(summary.defaultSeed, 7);
});

test("Planted Defect Gate C1: rejects tampered app.holo payload even when kappa cache is warm", async () => {
  const valid = Buffer.from(readFileSync(join(ROOT, "site", "app.holo")));
  valid[valid.length - 1] ^= 0xff;
  await assert.rejects(() => verifyPrismHoloContract(valid, { useCache: true, rootDir: ROOT }), /self-fingerprint fails/);
});

test("Planted Defect Gate C3: rejects handwritten JS SLM prompt bypass or spoofed logits", () => {
  const defectiveBypass = `
    export async function routeStudioPrompt(text, ctx) {
      if (text.includes("lod4")) {
        return { ok: true, slug: "stream-uorc-lod4", scoreMilli: 980, rawLogit: 42 };
      }
      return ctx.callWorker("route-prompt", { prompt: text, max: PRISM_MAX_LOD_RANK });
    }
  `;
  assert.throws(() => verifyZeroSlmBypassContract(defectiveBypass), /Bypass detected|Spoof detected/);
});

test("Planted Defect Gate C5: rejects coarse 2m shadow grid snapping and missing ITD/FDN audio", () => {
  assert.throws(
    () =>
      verifyUnifiedHostAndAudioContract({
        workerSource:
          "activeSeed walkJournal syncOrgansToWasm verdant_wasm_csm_split_meters verdant_wasm_binaural_itd_micros binauralItdMicros",
        rendererSource: "setCsmSplits; const sfx = Math.round(fx * 0.5) * 2;",
        audioWorkletSource: "PRISM_MODAL_BAR_NUMERS itdMicros fdnDelays",
      }),
    /2m world-snapped shadow frustum/,
  );
  assert.throws(
    () =>
      verifyUnifiedHostAndAudioContract({
        workerSource:
          "activeSeed walkJournal syncOrgansToWasm verdant_wasm_csm_split_meters verdant_wasm_binaural_itd_micros binauralItdMicros",
        rendererSource: "setCsmSplits;",
        audioWorkletSource: "PRISM_MODAL_BAR_NUMERS",
      }),
    /missing Prism spatial\/FDN acoustic token/,
  );
});

test("Planted Defect Gate C6: rejects handwritten JS starter terrain and Rust FI-08 magic constants", () => {
  assert.throws(
    () =>
      verifyStarterAndScaffolderContract({
        starterHtml: "<div></div>",
        starterGameSource: "function terrainHeight(x, z) { return 0; }",
      }),
    /handwritten JS terrainHeight bypass/,
  );
  assert.throws(
    () => verifyRustZeroMagicConstants("fn main() { let magic = 42_u32; }", "planted-defect.rs"),
    /FI-08 magic constant/,
  );
});

test("Planted Defect Gate C7: rejects raw #hex colors, backdrop-filter blur, live-embed hex colors, and missing DevRel controls", () => {
  const validHtml = readFileSync(join(ROOT, "site", "card.html"), "utf8");
  assert.throws(
    () =>
      verifyStudioSurfaceAndZeroHexContract({
        cardHtml: validHtml,
        cardCss: ".bad { color: #ff00aa; }",
        studioSource: "const STUDIO_CSS = `.ok { color: var(--fg); }`;",
        liveEmbedSource: "renderer?.setCsmSplits?.([]);",
      }),
    /raw #hex color literal/,
  );
  assert.throws(
    () =>
      verifyStudioSurfaceAndZeroHexContract({
        cardHtml: validHtml,
        cardCss: ".bad { backdrop-filter: blur(12px); }",
        studioSource: "const STUDIO_CSS = `.ok { color: var(--fg); }`;",
        liveEmbedSource: "renderer?.setCsmSplits?.([]);",
      }),
    /backdrop-filter: blur/,
  );
  assert.throws(
    () =>
      verifyStudioSurfaceAndZeroHexContract({
        cardHtml: validHtml,
        cardCss: ".ok { color: var(--fg); }",
        studioSource: "const STUDIO_CSS = `.ok { color: var(--fg); }`;",
        liveEmbedSource: "const pal = { accent: '#1f7a4d' }; renderer?.setCsmSplits?.([]);",
      }),
    /live-embed\.mjs contains raw #hex/,
  );
  assert.throws(
    () =>
      verifyStudioSurfaceAndZeroHexContract({
        cardHtml: validHtml.replace('id="ve-docs-link"', 'id="missing-link"'),
        cardCss: ".ok { color: var(--fg); }",
        studioSource: "const STUDIO_CSS = `.ok { color: var(--fg); }`;",
        liveEmbedSource: "renderer?.setCsmSplits?.([]);",
      }),
    /missing canonical DevRel control id="ve-docs-link"/,
  );
});

test("Studio Scaffolder generates FI-08 zero-constants files cleanly and exposes no .tar archive exporter", () => {
  const files = generateStarterProjectFiles({
    slug: "citadel-quest",
    archetypeIdx: 2,
    seed: 42,
    lodRank: 3,
    cameraMode: "third",
  });
  const paths = Object.keys(files);
  assert.ok(paths.includes("Cargo.toml"));
  assert.ok(paths.includes("src/main.rs"));
  assert.ok(paths.includes("web/index.html"));
  const webHtml = files["web/index.html"] ?? "";
  assert.ok(!webHtml.includes('href="/'), "Scaffolded web/index.html must not contain root-absolute hrefs");
  assert.equal("buildPosixTarArchive" in studioModule, false, "Verdant exclusively exports .uorc archives, not .tar");
});

test("Planted Defect Gate C8: rejects stakeholder/persona UI jargon, demo #hex colors, and self-referential starter names", () => {
  const baseInputs = {
    cardHtml: readFileSync(join(ROOT, "site", "card.html"), "utf8"),
    cardSource: readFileSync(join(ROOT, "site", "card.mjs"), "utf8"),
    studioSource: readFileSync(join(ROOT, "site", "studio.mjs"), "utf8"),
    stitchSource: readFileSync(join(ROOT, "site", "stitch-mesh.mjs"), "utf8"),
    demoHtml: readFileSync(join(ROOT, "site", "demo", "index.html"), "utf8"),
    demoSource: readFileSync(join(ROOT, "site", "demo", "demo.mjs"), "utf8"),
    starterHtml: readFileSync(join(ROOT, "templates", "browser-3d-starter", "index.html"), "utf8"),
    roleKeys: STUDIO_ROLE_KEYS,
    archetypes: STARTER_ARCHETYPES,
  };

  assert.throws(
    () =>
      verifyRoleDrivenStudioAndDemoSeparationContract({
        ...baseInputs,
        studioSource: `${baseInputs.studioSource}\nconst badLabel = "7 Stakeholder Personas";`,
      }),
    /forbidden "stakeholder"\/"persona" UI terminology/,
  );

  assert.throws(
    () =>
      verifyRoleDrivenStudioAndDemoSeparationContract({
        ...baseInputs,
        demoHtml: `${baseInputs.demoHtml}\n<style>.bad { color: #ff00aa; }</style>`,
      }),
    /demo\/index\.html contains raw #hex/,
  );

  assert.throws(
    () =>
      verifyRoleDrivenStudioAndDemoSeparationContract({
        ...baseInputs,
        starterHtml: "<title>Citadel Arena · Verdant Engine Starter</title><h1>Citadel Arena</h1>",
      }),
    /non-self-referential/,
  );

  assert.throws(
    () =>
      verifyRoleDrivenStudioAndDemoSeparationContract({
        ...baseInputs,
        archetypes: [
          ...STARTER_ARCHETYPES.slice(0, 3),
          { idx: 3, slug: "verdant-studio-clone", title: "Verdant Studio Demo", name: "Verdant Studio" },
        ],
      }),
    /self-referential/,
  );
});

test("Planted Defect Gate C9: rejects BooleanOrAnd drift, legacy aliases, 160.0 CSM drift, and nested .ve-m-log box borders", () => {
  const validC9 = {
    demoSource: readFileSync(join(ROOT, "site", "demo", "demo.mjs"), "utf8"),
    demoHtml: readFileSync(join(ROOT, "site", "demo", "index.html"), "utf8"),
    studioDataSource: readFileSync(join(ROOT, "site", "studio-data.mjs"), "utf8"),
    studioScaffolderSource: readFileSync(join(ROOT, "site", "studio-scaffolder.mjs"), "utf8"),
    studioSource: readFileSync(join(ROOT, "site", "studio.mjs"), "utf8"),
    cardSource: readFileSync(join(ROOT, "site", "card.mjs"), "utf8"),
    workerSource: readFileSync(join(ROOT, "site", "engine-worker.mjs"), "utf8"),
    starterGameSource: readFileSync(join(ROOT, "templates", "browser-3d-starter", "game.mjs"), "utf8"),
    liveEmbedSource: readFileSync(join(ROOT, "site", "docs", "live-embed.mjs"), "utf8"),
    docsIndexHtml: readFileSync(join(ROOT, "site", "docs", "index.html"), "utf8"),
  };

  assert.throws(
    () =>
      verifyPrismKernelAlignmentAndCleanRefactorContract({
        ...validC9,
        demoSource: `${validC9.demoSource}\nconst drift = "BooleanOrAnd";`,
      }),
    /BooleanOrAnd\/Boolean kernel drift/,
  );

  assert.throws(
    () =>
      verifyPrismKernelAlignmentAndCleanRefactorContract({
        ...validC9,
        studioSource: `${validC9.studioSource}\nconst LEGACY_TAB_ALIASES = {};`,
      }),
    /stale legacy alias/,
  );

  assert.throws(
    () =>
      verifyPrismKernelAlignmentAndCleanRefactorContract({
        ...validC9,
        starterGameSource: `${validC9.starterGameSource}\nconst far = 160.0;`,
      }),
    /160\.0m CSM\/perspective far-plane drift/,
  );

  assert.throws(
    () =>
      verifyPrismKernelAlignmentAndCleanRefactorContract({
        ...validC9,
        studioDataSource: validC9.studioDataSource.replace(
          "border: 0;",
          "border: var(--border-width) solid var(--border);",
        ),
      }),
    /violates single-elevation depth/,
  );

  assert.throws(
    () =>
      verifyPrismKernelAlignmentAndCleanRefactorContract({
        ...validC9,
        starterGameSource: validC9.starterGameSource.replace("scene:lod:${PRISM_MAX_LOD_RANK}", "static-lod-bypass"),
      }),
    /must hydrate LOD3_SPEC from app\.holo/,
  );
});

test("Decoupled studio-data and studio-scaffolder modules preserve FI-08 {0, 1} synthesis and base64url round-trips", () => {
  assert.deepEqual(DATA_ROLE_KEYS, STUDIO_ROLE_KEYS);
  assert.deepEqual(DATA_ARCHETYPES, STARTER_ARCHETYPES);
  assert.equal(CONTAINER_ORGANS.length, 11);

  for (const sample of [0, 1, 2, 7, 42, 108, 256]) {
    const expr = zeroOneExpr(sample);
    verifyRustZeroMagicConstants(`fn check() -> u64 { ${expr} }`, `zeroOneExpr(${sample})`);
  }

  const payload = new Uint8Array([0, 1, 2, 62, 63, 128, 254, 255]);
  const encoded = bytesToBase64Url(payload);
  assert.ok(!/[+/=]/.test(encoded));
  assert.deepEqual(base64UrlToBytes(encoded), payload);
  assert.equal(base64UrlToBytes("invalid$$$"), null);
});

test("Prism Catalogs pure module filters lifecycle stages, domains, roles, and parses Holo verdicts", () => {
  const cov = summarizeCatalogCoverage();
  assert.equal(cov.organCount, 5);
  assert.equal(cov.domainCount, 8);
  assert.equal(cov.lifecycleCount, 8);
  assert.ok(cov.totalModels >= 28);

  const policyModels = filterCatalogModels({ domain: "CAT-POLICY" });
  assert.ok(policyModels.some((m) => m.id === "STD-FK07-PRIVACY-ORGANS"));

  for (const rk of STUDIO_ROLE_KEYS) {
    assert.ok(catalogModelsForRole(rk).length >= 3, `expected >=3 governing models for role ${rk}`);
  }

  assert.deepEqual(parseConformantKeyValues("conformant:lod=3,lossless=1"), { lod: "3", lossless: "1" });
  assert.equal(parseConformantKeyValues("refused:zero-extent"), null);
});

test("Planted Defect Gate C10: rejects Engine/Studio conflation, cold-boot static imports, and unmodeled catalog entries", () => {
  const validC10 = {
    rootDir: ROOT,
    cardHtml: readFileSync(join(ROOT, "site", "card.html"), "utf8"),
    cardSource: readFileSync(join(ROOT, "site", "card.mjs"), "utf8"),
    studioPageHtml: readFileSync(join(ROOT, "site", "studio", "index.html"), "utf8"),
    studioPageSource: readFileSync(join(ROOT, "site", "studio", "studio-page.mjs"), "utf8"),
    catalogsHtml: readFileSync(join(ROOT, "site", "catalogs", "index.html"), "utf8"),
    catalogsPageSource: readFileSync(join(ROOT, "site", "catalogs", "catalogs.mjs"), "utf8"),
    organs: VERDANT_PLATFORM_ORGANS,
    catalogEntries: PRISM_MODEL_CATALOGS,
  };

  assert.throws(
    () =>
      verifyPlatformDifferentiationAndCatalogsContract({
        ...validC10,
        cardHtml: validC10.cardHtml.replace("Verdant Engine 3D runtime stage", "Verdant Studio 3D authoring viewport"),
      }),
    /Conflation detected/,
  );

  assert.throws(
    () =>
      verifyPlatformDifferentiationAndCatalogsContract({
        ...validC10,
        cardSource: `import { openStudioModal } from "./studio.mjs";\n${validC10.cardSource}`,
      }),
    /must use dynamic import\(\)/,
  );

  assert.throws(
    () =>
      verifyPlatformDifferentiationAndCatalogsContract({
        ...validC10,
        studioPageSource: validC10.studioPageSource.replace("../worker-runtime/index.mjs", "../vendor/prism-code/src/index.mjs"),
      }),
    /unshimmed vendor\/prism-code\//,
  );

  assert.throws(
    () =>
      verifyPlatformDifferentiationAndCatalogsContract({
        ...validC10,
        catalogEntries: [
          ...PRISM_MODEL_CATALOGS,
          {
            id: "STD-FAKE-UNMODELED",
            title: "Handwritten Spec",
            authority: "None",
            claimLevel: "Ad-hoc",
            domain: "CAT-ARCH",
            lifecycle: "architect",
            lexModule: "app/src/NonExistent.lex.tex",
            holoQueries: ["summary"],
            refusalQueries: [],
            studioRoles: ["code"],
            taxon: ["A", "B", "C"],
            invariant: "Unmodeled",
          },
        ],
      }),
    /non-existent LexLean module/,
  );
});

test("Planted Defect Gate C11: rejects tampered authority catalog SHA-256 and prohibited external model runtimes", () => {
  assert.throws(
    () =>
      verifyExternalOraclesAndRuntimeExclusivityContract({
        rootDir: ROOT,
        extraSources: [["planted-onnx.mjs", 'import { pipeline } from "@huggingface/transformers";']],
      }),
    /Prohibited external model runtime/,
  );

  const tamperedToml = readFileSync(join(ROOT, "model", "authorities.toml"), "utf8").replace(
    /[0-9a-f]{64}/,
    "0".repeat(64),
  );
  assert.throws(
    () =>
      verifyExternalOraclesAndRuntimeExclusivityContract({
        rootDir: ROOT,
        authoritiesToml: tamperedToml,
      }),
    /SHA-256 mismatch/,
  );
});

test("Planted Defect Gate C12: rejects root-absolute subpath violations and forbidden .tar project exports", () => {
  const validStudio = [
    ["site/studio/index.html", '<button id="studio-export-uorc-btn">Export Project (.uorc)</button>'],
    ["site/studio.mjs", 'btn.id = "ve-scaf-uorc-btn"; await callWorker("encode-snapshot", {});'],
  ];

  assert.throws(
    () =>
      verifySubpathAndUorcOnlyDeploymentContract({
        surfaceSources: [["planted-subpage.html", '<link rel="stylesheet" href="/brand/fonts.css">']],
        studioSources: validStudio,
      }),
    /Subpath violation/,
  );

  assert.throws(
    () =>
      verifySubpathAndUorcOnlyDeploymentContract({
        surfaceSources: [["planted-subpage.mjs", 'const r = await fetch("/app.holo");']],
        studioSources: validStudio,
      }),
    /Subpath violation/,
  );

  assert.throws(
    () =>
      verifySubpathAndUorcOnlyDeploymentContract({
        surfaceSources: [["planted-subpage.mjs", 'import { x } from "/worker-runtime/index.mjs";']],
        studioSources: validStudio,
      }),
    /Subpath violation/,
  );

  assert.throws(
    () =>
      verifySubpathAndUorcOnlyDeploymentContract({
        surfaceSources: [],
        studioSources: [
          ...validStudio,
          ["site/studio/index.html", '<button id="studio-export-tar-btn">Export Project (.tar)</button>'],
        ],
      }),
    /Archive policy violation/,
  );

  assert.throws(
    () =>
      verifySubpathAndUorcOnlyDeploymentContract({
        surfaceSources: [],
        studioSources: [
          ...validStudio,
          ["site/studio-scaffolder.mjs", "export function buildPosixTarArchive() { return new Uint8Array(512); }"],
        ],
      }),
    /Archive policy violation/,
  );
});

test("Live Catalog Streams define authoritative Schema.org/IANA types and auto-tag local/remote models cleanly", async () => {
  assert.ok(AUTHORITATIVE_CONTENT_TYPES.length >= 10, "expected 'all' plus 9 authoritative Schema.org/IANA content types");
  assert.ok(LIVE_CATALOG_STREAMS.length >= 7, "expected 6 live internet streams plus locked Verdant Prism stream");

  const worldPacks = JSON.parse(readFileSync(join(ROOT, "catalogs", "domains", "world-packs.json"), "utf8")).domains;
  const res = await searchLiveCatalogStreams({
    query: "",
    contentType: "all",
    streamId: "verdant-prism",
    localWorldPacks: worldPacks,
    includeRemote: false,
  });
  assert.ok(res.totalCount >= 20, `expected >=20 local auto-tagged catalog models, got ${res.totalCount}`);
  for (const item of res.items) {
    assert.ok(item.schemaType && item.mediaType && item.targetSlot, `item ${item.id} missing authoritative auto-tags`);
  }
});
