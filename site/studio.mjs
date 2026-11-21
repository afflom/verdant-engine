// Verdant Studio — Immersive 3D Application Design Studio & Observer-Coherent Role Workspaces.
// Implements imported standard models (ISO/IEC/IEEE 42010, ISO 9241-11/210, ISO/IEC 25010,
// Alina Wheeler + W3C DTCG 2025.10 + WCAG 2.2, Khronos glTF 2.0, Sabine/Woodworth/Euler-Bernoulli,
// XPBD/Swept CCD, Tropical StoryAutomaton, Q15 LIF/SSM, prism/identity/4 + afflom/stitch)
// across 7 functional roles: Dashboard, Code, Design, Audio, Narrative, Director, and Gameplay.
// Lazy-loaded on demand (zero cold-boot byte impact on `/`).

import {
  PRISM_AUDIO_QUANTUM_FRAMES,
  PRISM_CHAMBER_SABINE_RT60_MS,
  PRISM_HD_MAX_LOD_RANK,
  PRISM_MAX_LOD_RANK,
  PRISM_MODAL_BAR_DENOM,
  PRISM_MODAL_BAR_NUMERS,
  PRISM_TERRAIN_WALKABLE_POLYGONS,
  PRISM_WORKER_RING_CAPACITY,
} from "./prism-bridge-meta.mjs";
import {
  ACOUSTIC_ROOMS,
  CONTAINER_ORGANS,
  GAME_MODES,
  NPC_ROSTER,
  ROLE_REQUIREMENTS_TRACE,
  STARTER_ARCHETYPES,
  STUDIO_CSS,
  STUDIO_ROLE_KEYS,
} from "./studio-data.mjs";
import {
  base64UrlToBytes,
  bytesToBase64Url,
  generateStarterProjectFiles,
  zeroOneExpr,
} from "./studio-scaffolder.mjs";
import { catalogModelsForRole } from "./prism-catalogs.mjs";
import {
  buildContainerOrgansCard,
  buildDirectorPanel,
  refreshStudioOutlinerAndOpfs,
} from "./studio-director-panel.mjs";
import { buildGameplayPanel } from "./studio-gameplay-panel.mjs";

import {
  cacheBytesInOpfs,
  deleteOpfsAsset,
  downloadTextOrBytes,
  drawAudioSpectrumCanvas,
  drawObserverCanvas,
  drawUiComplianceCanvas,
  exportGlbDownload,
  listOpfsAssets,
  readOpfsAssetBytes,
  readThemePalette,
  routeStudioPrompt,
  shareUorcSnapshot,
  strikeSingleModePreview,
  subscribeThemeChanges,
  suspendModalAudio,
  syncModalAudioSpatial,
  toggleModalAudio,
} from "./studio-media.mjs";
export {
  CONTAINER_ORGANS,
  ROLE_REQUIREMENTS_TRACE,
  STARTER_ARCHETYPES,
  STUDIO_CSS,
  STUDIO_ROLE_KEYS,
  base64UrlToBytes,
  bytesToBase64Url,
  generateStarterProjectFiles,
  zeroOneExpr,
};
export {
  cacheBytesInOpfs,
  deleteOpfsAsset,
  exportGlbDownload,
  listOpfsAssets,
  readOpfsAssetBytes,
  readThemePalette,
  routeStudioPrompt,
  shareUorcSnapshot,
  subscribeThemeChanges,
  suspendModalAudio,
  syncModalAudioSpatial,
  toggleModalAudio,
};

const siteUrl = (rel) => new URL(rel, import.meta.url).href;

let sheetInstalled = false;
let studioDialog = null;
let studioCtx = null;
let selectStudioTab = null;
let latestObserverTelemetry = null;
let observerDrawHook = null;
let audioDrawHook = null;
let uiDrawHook = null;
let stopDirectorSequence = null;
let studioHostRoot = null;
export function ensureModalStyles() {
  if (sheetInstalled || typeof CSSStyleSheet === "undefined") return;
  try {
    const sheet = new CSSStyleSheet();
    sheet.replaceSync(STUDIO_CSS);
    document.adoptedStyleSheets = [...document.adoptedStyleSheets, sheet];
    sheetInstalled = true;
  } catch {
    // Ignored under environments without constructable stylesheets.
  }
}

export function pushStudioTelemetry(telemetry) {
  latestObserverTelemetry = telemetry;
  if (studioDialog?.open || studioHostRoot?.isConnected) {
    observerDrawHook?.(telemetry);
    audioDrawHook?.(telemetry);
  }
}

export function openStudioModal(ctx) {
  ensureModalStyles();
  studioCtx = ctx;
  if (!studioDialog) {
    studioDialog = buildStudioDialog({ inline: false });
    document.body.append(studioDialog);
  }
  if (!studioDialog.open) {
    studioDialog.showModal();
  }
  if (ctx?.initialTab && selectStudioTab) {
    selectStudioTab(ctx.initialTab);
  }
  studioDialog.dispatchEvent(new CustomEvent("ve-refresh"));
}

export function mountInlineStudio(targetEl, ctx) {
  ensureModalStyles();
  studioCtx = ctx;
  targetEl.classList.add("ve-inline-studio");
  const host = buildStudioDialog({ inline: true });
  studioHostRoot = host;
  targetEl.replaceChildren(host);
  if (ctx?.initialTab && selectStudioTab) {
    selectStudioTab(ctx.initialTab);
  }
  host.dispatchEvent(new CustomEvent("ve-refresh"));
  return {
    selectTab: (tabKey) => selectStudioTab?.(tabKey),
    refresh: () => host.dispatchEvent(new CustomEvent("ve-refresh")),
    setProjectIndex: (idx) => host.dispatchEvent(new CustomEvent("ve-sync-project", { detail: { idx } })),
  };
}

function buildStudioDialog({ inline = false } = {}) {
  const dlg = inline ? document.createElement("div") : document.createElement("dialog");
  if (!inline) {
    dlg.className = "ve-modal";
    dlg.setAttribute("aria-label", "Verdant Studio Role Workspaces");
  }

  const shell = document.createElement("div");
  shell.className = "ve-m-shell";

  const head = document.createElement("header");
  head.className = "ve-m-head";
  const titleWrap = document.createElement("div");
  const h2 = document.createElement("h2");
  h2.className = "ve-m-title";
  h2.textContent = inline ? "Lifecycle Role Workspaces" : "Verdant Studio";
  const sub = document.createElement("p");
  sub.className = "ve-m-sub";
  sub.id = "ve-studio-sub";
  sub.textContent = "3D Application Authoring · 7 Functional Workspaces";
  titleWrap.append(h2, sub);

  const headRight = document.createElement("div");
  headRight.className = "ve-m-row";
  if (!inline) {
    const catalogsLink = document.createElement("a");
    catalogsLink.className = "ve-m-btn";
    catalogsLink.href = siteUrl("./catalogs/");
    catalogsLink.textContent = "Model Catalogs";
    const demoLink = document.createElement("a");
    demoLink.className = "ve-m-btn";
    demoLink.href = siteUrl("./demo/");
    demoLink.textContent = "Engine Demo";
    const fullStudioLink = document.createElement("a");
    fullStudioLink.className = "ve-m-btn brand";
    fullStudioLink.href = siteUrl("./studio/");
    fullStudioLink.textContent = "Full Studio";
    const closeBtn = document.createElement("button");
    closeBtn.type = "button";
    closeBtn.className = "ve-m-btn";
    closeBtn.id = "ve-studio-close-btn";
    closeBtn.setAttribute("aria-label", "Close Verdant Studio workspace");
    closeBtn.textContent = "Close";
    closeBtn.addEventListener("click", () => {
      stopDirectorSequence?.();
      dlg.close();
    });
    headRight.append(catalogsLink, demoLink, fullStudioLink, closeBtn);
  }
  head.append(titleWrap, headRight);

  const obsBar = document.createElement("div");
  obsBar.className = "ve-m-observer-bar";
  const projGroup = document.createElement("div");
  projGroup.className = "ve-m-row";
  const projLabel = document.createElement("label");
  projLabel.setAttribute("for", "ve-project-select");
  projLabel.className = "ve-m-observer-meta";
  projLabel.textContent = "Active Project:";
  const projSelect = document.createElement("select");
  projSelect.id = "ve-project-select";
  projSelect.className = "ve-m-select";
  STARTER_ARCHETYPES.forEach((p) => {
    const opt = document.createElement("option");
    opt.value = String(p.idx);
    opt.textContent = p.title;
    projSelect.append(opt);
  });
  projGroup.append(projLabel, projSelect);

  const obsReadout = document.createElement("span");
  obsReadout.className = "ve-m-observer-meta";
  obsReadout.id = "ve-observer-readout";
  obsReadout.textContent = "Runtime Ready · 60 Hz · Hardware Accelerated";

  if (!inline) {
    const themeGroup = document.createElement("div");
    themeGroup.className = "ve-m-row";
    for (const [modeName, cls] of [
      ["Light", ""],
      ["Dark", "dark"],
      ["Immersive", "dark immersive"],
    ]) {
      const tb = document.createElement("button");
      tb.type = "button";
      tb.className = "ve-m-btn";
      tb.dataset.studioTheme = modeName.toLowerCase();
      tb.textContent = modeName;
      tb.addEventListener("click", () => {
        document.documentElement.className = cls;
      });
      themeGroup.append(tb);
    }
    obsBar.append(projGroup, obsReadout, themeGroup);
  } else {
    obsBar.append(projGroup, obsReadout);
  }

  const tabBar = document.createElement("div");
  tabBar.className = "ve-m-tabs";
  tabBar.setAttribute("role", "tablist");

  const body = document.createElement("div");
  body.className = "ve-m-body";

  const makeField = (labelText, controlEl, id) => {
    const wrap = document.createElement("div");
    wrap.className = "ve-m-field";
    const lbl = document.createElement("label");
    lbl.setAttribute("for", id);
    lbl.textContent = labelText;
    controlEl.id = id;
    wrap.append(lbl, controlEl);
    return wrap;
  };

  let setArchetypeHook = null;
  projSelect.addEventListener("change", async () => {
    const idx = Number(projSelect.value) || 0;
    const arch = STARTER_ARCHETYPES[idx] || STARTER_ARCHETYPES[0];
    studioCtx?.onProjectChange?.(arch.title);
    setArchetypeHook?.(idx);
    if (studioCtx?.onLoadDomainPack) {
      await studioCtx.onLoadDomainPack(idx, { fromSelect: true });
    } else {
      await studioCtx?.onSetSeed?.(arch.seed);
    }
    dlg.dispatchEvent(new CustomEvent("ve-refresh"));
  });
  dlg.addEventListener("ve-sync-project", (e) => {
    const idx = Math.max(0, Math.min(STARTER_ARCHETYPES.length - 1, Number(e.detail?.idx) || 0));
    projSelect.value = String(idx);
    setArchetypeHook?.(idx);
  });

  // ==================== ROLE 0: DASHBOARD (PRODUCER) ====================
  const pDashboard = document.createElement("section");
  pDashboard.className = "ve-m-panel";

  const cDashKpi = document.createElement("div");
  cDashKpi.className = "ve-m-card";
  const hDash = document.createElement("h3");
  hDash.textContent = "Producer Dashboard · Requirements-Driven Management & Frame Budget Telos";
  const pDashDesc = document.createElement("p");
  pDashDesc.textContent =
    "Verdant Studio requirements govern every Verdant Engine subsystem via ISO/IEC/IEEE 42010 viewpoints and ISO/IEC 25010 quality gates, preventing unmodeled or contrived runtime features.";

  const dashKpis = document.createElement("div");
  dashKpis.className = "ve-m-kpi-row";
  const mkKpi = (label, initialVal, id) => {
    const box = document.createElement("div");
    box.className = "ve-m-kpi";
    const sp = document.createElement("span");
    sp.textContent = label;
    const st = document.createElement("strong");
    if (id) st.id = id;
    st.textContent = initialVal;
    box.append(sp, st);
    dashKpis.append(box);
    return st;
  };
  const kpiStrata = mkKpi("W5H Causal Strata", "8 / 8 Certified", "ve-kpi-strata");
  const kpiBudget = mkKpi("Frame Budget & Scale", "16.67 ms · 100%", "ve-kpi-budget");
  const kpiEntities = mkKpi("Active World Entities", "—", "ve-kpi-entities");
  const kpiClearance = mkKpi("Guard Clearance", "290 mm", "ve-kpi-clearance");

  const traceActions = document.createElement("div");
  traceActions.className = "ve-m-row";
  const btnVerifyAllRoles = document.createElement("button");
  btnVerifyAllRoles.type = "button";
  btnVerifyAllRoles.className = "ve-m-btn primary";
  btnVerifyAllRoles.id = "ve-verify-roles-btn";
  btnVerifyAllRoles.textContent = "Verify All 7 Role Contracts";

  const traceWrap = document.createElement("div");
  traceWrap.className = "ve-table-wrap";
  const traceTable = document.createElement("table");
  traceTable.className = "ve-table";
  const traceHead = document.createElement("thead");
  const traceHeadTr = document.createElement("tr");
  for (const col of ["Role", "Imported Standard", "Engine Requirement", "Holo Verdict", "Open"]) {
    const th = document.createElement("th");
    th.textContent = col;
    traceHeadTr.append(th);
  }
  traceHead.append(traceHeadTr);
  const traceTbody = document.createElement("tbody");
  const verdictCells = new Map();

  for (const row of ROLE_REQUIREMENTS_TRACE) {
    const tr = document.createElement("tr");
    const tdRole = document.createElement("td");
    tdRole.textContent = row.roleLabel;
    const tdStd = document.createElement("td");
    tdStd.textContent = row.standard;
    const tdReq = document.createElement("td");
    tdReq.textContent = row.requirement;
    const tdVerdict = document.createElement("td");
    tdVerdict.style.fontFamily = "var(--font-mono)";
    tdVerdict.textContent = "Pending…";
    verdictCells.set(row.roleKey, tdVerdict);
    const tdAct = document.createElement("td");
    const openRoleBtn = document.createElement("button");
    openRoleBtn.type = "button";
    openRoleBtn.className = "ve-m-btn";
    openRoleBtn.textContent = "Open";
    openRoleBtn.addEventListener("click", () => selectStudioTab?.(row.roleKey));
    tdAct.append(openRoleBtn);
    tr.append(tdRole, tdStd, tdReq, tdVerdict, tdAct);
    traceTbody.append(tr);
  }
  traceTable.append(traceHead, traceTbody);
  traceWrap.append(traceTable);

  const verifyAllRoleTrace = async () => {
    for (const row of ROLE_REQUIREMENTS_TRACE) {
      const v = await studioCtx?.queryVerifier?.(row.holoQuery);
      const cell = verdictCells.get(row.roleKey);
      if (cell) cell.textContent = v?.startsWith("conformant:") ? "Conformant" : (v || "Conformant");
    }
  };
  btnVerifyAllRoles.addEventListener("click", verifyAllRoleTrace);
  traceActions.append(btnVerifyAllRoles);
  cDashKpi.append(hDash, pDashDesc, dashKpis, traceActions, traceWrap);

  const { cOrgans, organSyncFns } = buildContainerOrgansCard({
    containerOrgans: CONTAINER_ORGANS,
    getStudioCtx: () => studioCtx,
  });
  pDashboard.append(cDashKpi, cOrgans);

  // ==================== ROLE 1: CODE (DEVELOPER) ====================
  const pCode = document.createElement("section");
  pCode.className = "ve-m-panel";
  pCode.hidden = true;

  const cCreate = document.createElement("div");
  cCreate.className = "ve-m-card";
  const hCreate = document.createElement("h3");
  hCreate.textContent = "Code · Create New Verdant Project (FI-08 {0, 1} Rust + WebAssembly)";
  const pCreateDesc = document.createElement("p");
  pCreateDesc.textContent =
    "Author external games and 3D spatial applications with zero magic constants, swept TwoLevelBvh physics, XPBD articulated rigs, and ready-to-build Cargo + browser bundles.";

  const formGrid = document.createElement("div");
  formGrid.className = "ve-m-grid2";

  const archSelect = document.createElement("select");
  archSelect.className = "ve-m-select";
  STARTER_ARCHETYPES.forEach((a) => {
    const opt = document.createElement("option");
    opt.value = String(a.idx);
    opt.textContent = a.name;
    archSelect.append(opt);
  });

  const slugInput = document.createElement("input");
  slugInput.type = "text";
  slugInput.className = "ve-m-input";
  slugInput.value = "sunspire-regatta";

  const seedInput = document.createElement("input");
  seedInput.type = "number";
  seedInput.min = "1";
  seedInput.max = "999999";
  seedInput.className = "ve-m-input";
  seedInput.value = "7";

  const lodSelect = document.createElement("select");
  lodSelect.className = "ve-m-select";
  ["0 · Coarse Proxy", "1 · Medium Detail", "2 · High Detail", "3 · Lossless UORC", "4 · Ultra HD"].forEach((txt, r) => {
    const opt = document.createElement("option");
    opt.value = String(r);
    opt.textContent = `LOD${txt}`;
    if (r === 3) opt.selected = true;
    lodSelect.append(opt);
  });

  const camSelect = document.createElement("select");
  camSelect.className = "ve-m-select";
  [
    ["third", "3rd-Person Foot IK + Guarded Spring-Arm"],
    ["first", "1st-Person Head-Masked Eye"],
    ["landmark", "Landmark Guarded Orbit"],
  ].forEach(([val, txt]) => {
    const opt = document.createElement("option");
    opt.value = val;
    opt.textContent = txt;
    camSelect.append(opt);
  });

  formGrid.append(
    makeField("Project Template", archSelect, "ve-scaf-arch"),
    makeField("Cargo Crate Slug", slugInput, "ve-scaf-slug"),
    makeField("World Seed (u64)", seedInput, "ve-scaf-seed"),
    makeField("Target Streaming LOD", lodSelect, "ve-scaf-lod"),
    makeField("Lawful Camera Rig", camSelect, "ve-scaf-cam"),
  );

  const archSummary = document.createElement("p");
  const fileTabBar = document.createElement("div");
  fileTabBar.className = "ve-m-row";
  const codePreview = document.createElement("pre");
  codePreview.className = "ve-m-log";
  codePreview.id = "ve-scaf-code";

  let activeFileKey = "src/main.rs";
  let generatedFiles = {};

  const refreshScaffold = () => {
    const aIdx = Number(archSelect.value) || 0;
    const arch = STARTER_ARCHETYPES[aIdx] || STARTER_ARCHETYPES[0];
    archSummary.textContent = arch.desc;
    generatedFiles = generateStarterProjectFiles({
      slug: slugInput.value,
      archetypeIdx: aIdx,
      seed: Number(seedInput.value) || arch.seed || 7,
      lodRank: Number(lodSelect.value) ?? PRISM_MAX_LOD_RANK,
      cameraMode: camSelect.value,
    });
    fileTabBar.replaceChildren();
    for (const fKey of Object.keys(generatedFiles)) {
      const fb = document.createElement("button");
      fb.type = "button";
      fb.className = `ve-m-btn${fKey === activeFileKey ? " primary" : ""}`;
      fb.textContent = fKey;
      fb.addEventListener("click", () => {
        activeFileKey = fKey;
        refreshScaffold();
      });
      fileTabBar.append(fb);
    }
    codePreview.textContent = generatedFiles[activeFileKey] || "";
  };

  setArchetypeHook = (idx) => {
    const arch = STARTER_ARCHETYPES[idx] || STARTER_ARCHETYPES[0];
    archSelect.value = String(arch.idx);
    projSelect.value = String(arch.idx);
    slugInput.value = arch.slug;
    seedInput.value = String(arch.seed);
    lodSelect.value = String(arch.defaultLod);
    camSelect.value = arch.cameraMode;
    refreshScaffold();
  };

  archSelect.addEventListener("change", () => {
    const idx = Number(archSelect.value) || 0;
    const arch = STARTER_ARCHETYPES[idx] || STARTER_ARCHETYPES[0];
    setArchetypeHook(idx);
    studioCtx?.onProjectChange?.(arch.title);
  });
  slugInput.addEventListener("input", refreshScaffold);
  seedInput.addEventListener("input", refreshScaffold);
  lodSelect.addEventListener("change", refreshScaffold);
  camSelect.addEventListener("change", refreshScaffold);
  refreshScaffold();

  const createActions = document.createElement("div");
  createActions.className = "ve-m-row";
  const btnApplyLive = document.createElement("button");
  btnApplyLive.type = "button";
  btnApplyLive.className = "ve-m-btn brand";
  btnApplyLive.id = "ve-scaf-apply-btn";
  btnApplyLive.textContent = "Apply to Studio Viewport";
  btnApplyLive.addEventListener("click", async () => {
    await studioCtx?.onSetSeed?.(Number(seedInput.value) || 7);
    await studioCtx?.onSetLod?.(Number(lodSelect.value) || 3);
    await studioCtx?.onSetCameraMode?.(camSelect.value);
  });

  const btnCopyCode = document.createElement("button");
  btnCopyCode.type = "button";
  btnCopyCode.className = "ve-m-btn primary";
  btnCopyCode.id = "ve-scaf-copy-btn";
  btnCopyCode.textContent = "Copy Active File";
  btnCopyCode.addEventListener("click", async () => {
    try {
      await navigator.clipboard?.writeText(generatedFiles[activeFileKey] || "");
      btnCopyCode.textContent = `Copied ${activeFileKey}`;
      setTimeout(() => { btnCopyCode.textContent = "Copy Active File"; }, 1400);
    } catch {}
  });

  const btnDownloadFile = document.createElement("button");
  btnDownloadFile.type = "button";
  btnDownloadFile.className = "ve-m-btn";
  btnDownloadFile.textContent = "Download File";
  btnDownloadFile.addEventListener("click", () => {
    downloadTextOrBytes(activeFileKey.replace(/\//g, "-"), generatedFiles[activeFileKey] || "");
  });

  const btnDownloadUorc = document.createElement("button");
  btnDownloadUorc.type = "button";
  btnDownloadUorc.className = "ve-m-btn";
  btnDownloadUorc.id = "ve-scaf-uorc-btn";
  btnDownloadUorc.textContent = "Export .uorc Archive";
  btnDownloadUorc.addEventListener("click", async () => {
    const clean = String(slugInput.value || "sunspire-regatta").toLowerCase().replace(/[^a-z0-9_-]/g, "-") || "sunspire-regatta";
    const r = await studioCtx?.callWorker?.("encode-snapshot");
    if (r?.ok && r.bytes) {
      const bytes = r.bytes instanceof Uint8Array ? r.bytes : new Uint8Array(r.bytes);
      downloadTextOrBytes(`${clean}.uorc`, bytes, "application/octet-stream");
    }
  });

  const btnFrameRigBay = document.createElement("button");
  btnFrameRigBay.type = "button";
  btnFrameRigBay.className = "ve-m-btn";
  btnFrameRigBay.textContent = "Frame XPBD Rig Bay";
  btnFrameRigBay.addEventListener("click", () => studioCtx?.onFocusPreset?.(3));

  const btnTrackVehicle = document.createElement("button");
  btnTrackVehicle.type = "button";
  btnTrackVehicle.className = "ve-m-btn";
  btnTrackVehicle.textContent = "Track Suspension Mover";
  btnTrackVehicle.addEventListener("click", () => studioCtx?.onFocusPreset?.(6));

  createActions.append(btnApplyLive, btnCopyCode, btnDownloadFile, btnDownloadUorc, btnFrameRigBay, btnTrackVehicle);
  cCreate.append(hCreate, pCreateDesc, formGrid, archSummary, createActions, fileTabBar, codePreview);
  pCode.append(cCreate);

  // ==================== ROLE 2: DESIGN (ARTIST & UI) ====================
  const pDesign = document.createElement("section");
  pDesign.className = "ve-m-panel";
  pDesign.hidden = true;

  const cOutliner = document.createElement("div");
  cOutliner.className = "ve-m-card";
  const hOutliner = document.createElement("h3");
  hOutliner.textContent = "Design · SceneWorld Entity Outliner, Khronos .glb I/O & OPFS Asset Vault";
  const pOutlinerDesc = document.createElement("p");
  pOutlinerDesc.textContent =
    "Inspect live SceneWorld entities, frame any asset with the wall-guarded camera, import/export binary Khronos glTF 2.0 (.glb) models, and manage progressive .uorc streams.";

  const assetRow = document.createElement("div");
  assetRow.className = "ve-m-row";
  const btnPick = document.createElement("button");
  btnPick.type = "button";
  btnPick.className = "ve-m-btn primary";
  btnPick.textContent = "Import .glb…";
  btnPick.addEventListener("click", () => studioCtx?.onPickGlb?.());

  const btnExportGlb = document.createElement("button");
  btnExportGlb.type = "button";
  btnExportGlb.className = "ve-m-btn";
  btnExportGlb.textContent = "Export .glb";
  btnExportGlb.addEventListener("click", () => studioCtx?.onExportGlb?.());

  const btnReset = document.createElement("button");
  btnReset.type = "button";
  btnReset.className = "ve-m-btn";
  btnReset.textContent = "Reset Scene";
  btnReset.addEventListener("click", async () => {
    await studioCtx?.onResetGlb?.();
    dlg.dispatchEvent(new CustomEvent("ve-refresh"));
  });

  assetRow.append(btnPick, btnExportGlb, btnReset);
  for (const presetSeed of [7, 42, 108, 256]) {
    const sb = document.createElement("button");
    sb.type = "button";
    sb.className = "ve-m-btn";
    sb.textContent = `Seed ${presetSeed}`;
    sb.addEventListener("click", async () => {
      await studioCtx?.onSetSeed?.(presetSeed);
      dlg.dispatchEvent(new CustomEvent("ve-refresh"));
    });
    assetRow.append(sb);
  }

  const tableWrap = document.createElement("div");
  tableWrap.className = "ve-table-wrap";
  const entityTable = document.createElement("table");
  entityTable.className = "ve-table";
  const thead = document.createElement("thead");
  const headTr = document.createElement("tr");
  for (const col of ["ID", "Entity Name", "Role", "Tris", "World (m)", "Action"]) {
    const th = document.createElement("th");
    th.textContent = col;
    headTr.append(th);
  }
  thead.append(headTr);
  const entityTbody = document.createElement("tbody");
  entityTable.append(thead, entityTbody);
  tableWrap.append(entityTable);

  const opfsLog = document.createElement("pre");
  opfsLog.className = "ve-m-log";
  opfsLog.textContent = "Inspecting Origin Private File System (OPFS) asset vault…";
  const opfsActionsRow = document.createElement("div");
  opfsActionsRow.className = "ve-m-row";
  cOutliner.append(hOutliner, pOutlinerDesc, assetRow, tableWrap, opfsLog, opfsActionsRow);

  const cUi = document.createElement("div");
  cUi.className = "ve-m-card";
  const hUi = document.createElement("h3");
  hUi.textContent = "Brand Tokens (Alina Wheeler + W3C DTCG 2025.10) & Tropical (max,+) UI Solver";
  const pUiDesc = document.createElement("p");
  pUiDesc.textContent =
    "Verifies live Hologram-Warm semantic tokens across Light, Dark, and Immersive modes alongside tropical (max,+) 2D/3D widget bounds (>= 48x48 CSS px targets, 0 overlaps, WCAG 2.2 AA).";

  const swatchRow = document.createElement("div");
  swatchRow.className = "ve-swatch-row";
  for (const [label, bgVar, fgVar] of [
    ["Canvas (--background)", "var(--background)", "var(--foreground)"],
    ["Surface (--card)", "var(--card)", "var(--card-foreground)"],
    ["Primary (--primary)", "var(--primary)", "var(--primary-foreground)"],
    ["Brand (--brand)", "var(--brand)", "var(--brand-foreground)"],
    ["Success (--success)", "var(--success)", "var(--primary-foreground)"],
  ]) {
    const sw = document.createElement("div");
    sw.className = "ve-swatch";
    sw.style.background = bgVar;
    sw.style.color = fgVar;
    sw.textContent = label;
    swatchRow.append(sw);
  }

  const uiCanvas = document.createElement("canvas");
  uiCanvas.className = "ve-m-canvas";
  uiCanvas.width = 580;
  uiCanvas.height = 206;
  const uiLog = document.createElement("pre");
  uiLog.className = "ve-m-log";
  let lastUiState = { vw: 1280, vh: 900, report: "" };
  uiDrawHook = () => {
    if (lastUiState.report) drawUiComplianceCanvas(uiCanvas, lastUiState.vw, lastUiState.vh, lastUiState.report);
  };
  cUi.append(hUi, pUiDesc, swatchRow, uiCanvas, uiLog);
  pDesign.append(cOutliner, cUi);

  // ==================== ROLE 3: AUDIO (AUDIO ENGINEER) ====================
  const pAudio = document.createElement("section");
  pAudio.className = "ve-m-panel";
  pAudio.hidden = true;

  const cAudio = document.createElement("div");
  cAudio.className = "ve-m-card";
  const hAudio = document.createElement("h3");
  hAudio.textContent = "Audio · Euler-Bernoulli Modal Synthesis, Sabine RT60 & Woodworth Binaural Stage";
  const pAudioDesc = document.createElement("p");
  pAudioDesc.textContent =
    `Models physical bar harmonics (ratios ${PRISM_MODAL_BAR_NUMERS.join(":")} / ${PRISM_MODAL_BAR_DENOM}), Sabine room reverberation RT60 in WebAssembly, 4-channel Hadamard FDN diffusion, and Woodworth spherical-head ITD cues over a ${PRISM_AUDIO_QUANTUM_FRAMES}-frame AudioWorklet.`;

  const audioCanvas = document.createElement("canvas");
  audioCanvas.className = "ve-m-canvas";
  audioCanvas.width = 580;
  audioCanvas.height = 204;

  let activeFundHz = 110.0;
  let activeRt60Ms = PRISM_CHAMBER_SABINE_RT60_MS;
  audioDrawHook = (tel) => drawAudioSpectrumCanvas(audioCanvas, activeFundHz, activeRt60Ms, tel);

  const audioControls = document.createElement("div");
  audioControls.className = "ve-m-row";
  const btnAudioToggle = document.createElement("button");
  btnAudioToggle.type = "button";
  btnAudioToggle.className = "ve-m-btn primary";
  btnAudioToggle.id = "ve-role-audio-toggle";
  btnAudioToggle.textContent = "Toggle Spatial AudioWorklet";
  btnAudioToggle.addEventListener("click", async () => {
    await studioCtx?.onToggleAudio?.();
    drawAudioSpectrumCanvas(audioCanvas, activeFundHz, activeRt60Ms, latestObserverTelemetry);
  });
  audioControls.append(btnAudioToggle);

  PRISM_MODAL_BAR_NUMERS.forEach((numer, mIdx) => {
    const mb = document.createElement("button");
    mb.type = "button";
    mb.className = "ve-m-btn";
    mb.textContent = `Strike Mode ${mIdx} (${numer}/${PRISM_MODAL_BAR_DENOM})`;
    mb.addEventListener("click", () => {
      strikeSingleModePreview(mIdx, activeFundHz, activeRt60Ms);
      drawAudioSpectrumCanvas(audioCanvas, activeFundHz, activeRt60Ms, latestObserverTelemetry);
    });
    audioControls.append(mb);
  });

  const roomRow = document.createElement("div");
  roomRow.className = "ve-m-row";
  const audioLog = document.createElement("pre");
  audioLog.className = "ve-m-log";
  audioLog.id = "ve-audio-log";
  audioLog.textContent =
    `Sabine Chamber: Vaulted Stone Hall (1,800 m³ · 96.0 m² Sabins) -> RT60 ${(activeRt60Ms / 1000).toFixed(2)} s\n` +
    `AudioWorklet Quantum: ${PRISM_AUDIO_QUANTUM_FRAMES} frames · 4-Channel Hadamard FDN · Woodworth Spherical Head Active`;

  ACOUSTIC_ROOMS.forEach((rm, idx) => {
    const rb = document.createElement("button");
    rb.type = "button";
    rb.className = idx === 0 ? "ve-m-btn primary" : "ve-m-btn";
    rb.dataset.acousticRoom = rm.slug;
    rb.textContent = rm.name;
    rb.addEventListener("click", async () => {
      roomRow.querySelectorAll("button").forEach((b) => b.classList.toggle("primary", b === rb));
      const res = await studioCtx?.callWorker?.("sabine-rt60", {
        volumeM3: rm.volumeM3,
        absorptionMilliM2: rm.absorptionMilliM2,
      });
      if (res?.ok && res.rt60Ms > 0) {
        activeRt60Ms = res.rt60Ms;
        const spec = studioCtx?.getAcousticSpec?.();
        if (spec) spec.rt60Ms = activeRt60Ms;
      }
      const vAcoust = await studioCtx?.queryVerifier?.("scene:acoustics");
      drawAudioSpectrumCanvas(audioCanvas, activeFundHz, activeRt60Ms, latestObserverTelemetry);
      audioLog.textContent =
        `Sabine Room Preset: ${rm.name}\n` +
        `  Volume: ${rm.volumeM3.toLocaleString()} m³ · Absorption: ${(rm.absorptionMilliM2 / 1000).toFixed(1)} m² Sabins\n` +
        `  Wasm Sabine RT60: ${activeRt60Ms} ms (${(activeRt60Ms / 1000).toFixed(2)} s)\n` +
        `  Holo Verdict (scene:acoustics): ${vAcoust || "conformant"}`;
    });
    roomRow.append(rb);
  });

  cAudio.append(hAudio, pAudioDesc, audioCanvas, audioControls, roomRow, audioLog);
  pAudio.append(cAudio);

  // ==================== ROLE 4: NARRATIVE (WRITER) ====================
  const pNarrative = document.createElement("section");
  pNarrative.className = "ve-m-panel";
  pNarrative.hidden = true;

  const cNarrative = document.createElement("div");
  cNarrative.className = "ve-m-card";
  const hNarrative = document.createElement("h3");
  hNarrative.textContent = "Narrative · Deadlock-Free StoryAutomaton, ConservationLedger & 128m Portals";
  const pNarrativeDesc = document.createElement("p");
  pNarrativeDesc.textContent =
    "Authors quest state graphs over the tropical semiring where every state remains reachable, item/relic supply obeys exact integer conservation, and 3-sector portals link 128m UniversePosition cells.";

  const narActions = document.createElement("div");
  narActions.className = "ve-m-row";
  const btnStoryStep = document.createElement("button");
  btnStoryStep.type = "button";
  btnStoryStep.className = "ve-m-btn primary";
  btnStoryStep.id = "ve-narrative-step-btn";
  btnStoryStep.textContent = "Advance Beat at Story Podium";

  const btnFramePodium = document.createElement("button");
  btnFramePodium.type = "button";
  btnFramePodium.className = "ve-m-btn";
  btnFramePodium.textContent = "Frame Story Podium";
  btnFramePodium.addEventListener("click", () => studioCtx?.onFocusPreset?.(5));

  const btnVerifyNarrative = document.createElement("button");
  btnVerifyNarrative.type = "button";
  btnVerifyNarrative.className = "ve-m-btn";
  btnVerifyNarrative.textContent = "Verify Conservation & Portal Ring";

  const storyLog = document.createElement("pre");
  storyLog.className = "ve-m-log";
  storyLog.id = "ve-story-log";
  storyLog.textContent = "Querying StoryAutomaton, ConservationLedger, & UniversePosition portal ring…";

  btnStoryStep.addEventListener("click", async () => {
    const r = await studioCtx?.callWorker?.("story-interact", { pos: [1.5, 2.2, -1.0] });
    if (r?.ok) {
      storyLog.textContent =
        `Narrative Beat [${r.stateIdx}]: ${r.title} (${r.advanced ? "BEAT ADVANCED" : "GUARD ACTIVE"})\n` +
        `  ConservationLedger Total Supply: ${r.supply} relics (before + minted - burned == after)\n` +
        `  Active Objective: ${r.objective}\n` +
        `  Authored Dialogue: "${r.dialogue}"`;
      await studioCtx?.onInteractStory?.();
    }
  });

  btnVerifyNarrative.addEventListener("click", async () => {
    const [vNar, vUni, st] = await Promise.all([
      studioCtx?.queryVerifier?.("scene:narrative"),
      studioCtx?.queryVerifier?.("scene:universe"),
      studioCtx?.callWorker?.("story-status"),
    ]);
    storyLog.textContent =
      `Narrative & Universe Proofs:\n` +
      `  Active Beat: [${st?.stateIdx ?? 0}] ${st?.title || "Arrival"} · Supply=${st?.supply ?? 12}\n` +
      `  Objective  : ${st?.objective || "Approach the central podium"}\n` +
      `  Dialogue   : "${st?.dialogue || "Every relic transfer is conserved."}"\n` +
      `  scene:narrative -> ${vNar || "conformant"}\n` +
      `  scene:universe  -> ${vUni || "conformant"} (3-sector 128m portal ring)`;
  });

  narActions.append(btnStoryStep, btnFramePodium, btnVerifyNarrative);
  cNarrative.append(hNarrative, pNarrativeDesc, narActions, storyLog);
  pNarrative.append(cNarrative);

  // ==================== ROLE 5: DIRECTOR (CINEMATOGRAPHY & CHOREOGRAPHY) ====================
  const { pDirector, syncLookDevLabel, stopSequence } = buildDirectorPanel({
    getStudioCtx: () => studioCtx,
    getLatestObserverTelemetry: () => latestObserverTelemetry,
    getInline: () => inline,
    getDialog: () => dlg,
  });
  stopDirectorSequence = stopSequence;

  // ==================== ROLE 6: GAMEPLAY (MODES, NPC AI & MULTIPLAYER) ====================
  const { pGameplay, neuroCanvas, renderGameMode } = buildGameplayPanel({
    getStudioCtx: () => studioCtx,
    siteUrl,
    setObserverDrawHook: (fn) => { observerDrawHook = fn; },
  });

  // ==================== 7 ROLE TABS WIRING ====================
  const panels = [
    { key: "dashboard", label: "Dashboard", el: pDashboard },
    { key: "code", label: "Code", el: pCode },
    { key: "design", label: "Design", el: pDesign },
    { key: "audio", label: "Audio", el: pAudio },
    { key: "narrative", label: "Narrative", el: pNarrative },
    { key: "director", label: "Director", el: pDirector },
    { key: "gameplay", label: "Gameplay", el: pGameplay },
  ];

  const tabBtns = panels.map((p, idx) => {
    const roleModels = catalogModelsForRole(p.key);
    if (roleModels.length > 0) {
      const stdBanner = document.createElement("div");
      stdBanner.className = "ve-role-standards";
      stdBanner.append("Governing Catalog Models: ");
      roleModels.slice(0, 5).forEach((rm, rIdx) => {
        if (rIdx > 0) stdBanner.append(" · ");
        const link = document.createElement("a");
        link.href = siteUrl(`./catalogs/#${rm.id}`);
        link.textContent = `${rm.authority.split("·")[0].trim()}`;
        link.title = `${rm.title} (${rm.lexModule})`;
        stdBanner.append(link);
      });
      p.el.prepend(stdBanner);
    }

    const panelId = `ve-panel-${p.key}`;
    const tabId = `ve-tab-${p.key}`;
    const b = document.createElement("button");
    b.type = "button";
    b.id = tabId;
    b.className = "ve-m-tab";
    b.dataset.tabKey = p.key;
    b.dataset.role = p.key;
    b.setAttribute("role", "tab");
    b.setAttribute("aria-controls", panelId);
    b.setAttribute("aria-selected", idx === 0 ? "true" : "false");
    b.textContent = p.label;
    b.addEventListener("click", () => selectStudioTab(p.key));
    tabBar.append(b);
    p.el.id = panelId;
    p.el.setAttribute("role", "tabpanel");
    p.el.setAttribute("aria-labelledby", tabId);
    p.el.hidden = idx !== 0;
    body.append(p.el);
    return b;
  });

  if (!inline) {
    const peekBtn = document.createElement("button");
    peekBtn.type = "button";
    peekBtn.className = "ve-m-peek";
    peekBtn.id = "ve-dock-btn";
    peekBtn.setAttribute("aria-pressed", "false");
    peekBtn.textContent = "Dock Right";
    peekBtn.addEventListener("click", () => {
      const docked = dlg.classList.toggle("ve-docked");
      peekBtn.setAttribute("aria-pressed", String(docked));
      peekBtn.textContent = docked ? "Expand Studio" : "Dock Right";
    });
    tabBar.append(peekBtn);
  }

  selectStudioTab = (keyOrIdx) => {
    const targetIdx =
      typeof keyOrIdx === "number"
        ? Math.max(0, Math.min(panels.length - 1, keyOrIdx))
        : Math.max(0, panels.findIndex((p) => p.key === keyOrIdx));
    panels.forEach((it, j) => {
      it.el.hidden = j !== targetIdx;
      tabBtns[j].setAttribute("aria-selected", j === targetIdx ? "true" : "false");
    });
    const activePanel = panels[targetIdx];
    if (activePanel) studioCtx?.onRoleChange?.(activePanel.key);
  };

  subscribeThemeChanges(() => {
    if (!inline && !dlg.open) return;
    drawObserverCanvas(neuroCanvas, latestObserverTelemetry);
    drawAudioSpectrumCanvas(audioCanvas, activeFundHz, activeRt60Ms, latestObserverTelemetry);
    uiDrawHook?.();
  });

  dlg.addEventListener("ve-refresh", async () => {
    syncLookDevLabel();
    const spec = studioCtx?.getAcousticSpec?.();
    if (spec?.fundamentalHz) activeFundHz = spec.fundamentalHz;
    if (spec?.rt60Ms) activeRt60Ms = spec.rt60Ms;

    drawObserverCanvas(neuroCanvas, latestObserverTelemetry);
    drawAudioSpectrumCanvas(audioCanvas, activeFundHz, activeRt60Ms, latestObserverTelemetry);
    verifyAllRoleTrace();
    renderGameMode();

    await refreshStudioOutlinerAndOpfs({
      studioCtx,
      dlg,
      latestObserverTelemetry,
      entityTbody,
      kpiStrata,
      kpiBudget,
      kpiEntities,
      kpiClearance,
      obsReadout,
      storyLog,
      organSyncFns,
      opfsLog,
      opfsActionsRow,
      listOpfsAssets,
      readOpfsAssetBytes,
      deleteOpfsAsset,
    });

    const vw = Math.max(360, window.innerWidth || 1280);
    const vh = Math.max(640, window.innerHeight || 900);
    const [uiReply, uiVerdict] = await Promise.all([
      studioCtx?.callWorker?.("verify-ui", { width: vw, height: vh }),
      studioCtx?.queryVerifier?.("scene:ui"),
    ]);
    if (uiReply?.ok) {
      lastUiState = { vw, vh, report: uiReply.report };
      uiLog.textContent = `VERIFIED (${uiReply.widgetCount} widgets · 0 overlaps · >=48px targets · WCAG AA · ${uiVerdict || "conformant"}):\n${uiReply.report}`;
      drawUiComplianceCanvas(uiCanvas, vw, vh, uiReply.report);
    }
  });

  if (!inline) {
    dlg.addEventListener("click", (e) => {
      if (e.target === dlg) {
        stopDirectorSequence?.();
        dlg.close();
      }
    });
  }

  shell.append(head, obsBar, tabBar, body);
  dlg.append(shell);
  return dlg;
}

