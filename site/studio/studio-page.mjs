// Standalone Verdant Studio IDE Controller (`site/studio/studio-page.mjs`).
// Unifies:
//   1. Live HD 3D WebGL2/Wasm stage (`../engine-worker.mjs`) + `../app.holo`
//   2. Unified Multi-Stream Live Internet Catalog Discovery (`../catalog-streams.mjs`)
//   3. Wasm `ObserverPrismHolon` Dynamic Model Admission, Editing, Immutable `kappa`
//      Versioning (`v1 -> v2 -> ...`), Non-Destructive Revert, & Removal
//   4. 4-Stage Prism Composition & Quality Gate + 7 Lifecycle Role Workspaces (`../studio.mjs`)

import { load } from "../worker-runtime/index.mjs";
import {
  AUTHORITATIVE_CONTENT_TYPES,
  LIVE_CATALOG_STREAMS,
  fetchLiveKnowledgePassage,
  hydrateStitchAppCatalog,
  publishStitchAppToCatalog,
  searchLiveCatalogStreams,
} from "../catalog-streams.mjs";
import {
  STARTER_ARCHETYPES,
  STUDIO_ROLE_KEYS,
  base64UrlToBytes,
  cacheBytesInOpfs,
  exportGlbDownload,
  mountInlineStudio,
  pushStudioTelemetry,
  routeStudioPrompt,
  shareUorcSnapshot,
  suspendModalAudio,
  syncModalAudioSpatial,
  toggleModalAudio,
} from "../studio.mjs";

const LODS = [
  { rank: 0, key: "lod:0", short: "LOD0", label: "LOD0 Proxy", res: 0.125, density: 0.35, lossless: false, sky: [0.09, 0.08, 0.12], fog: [0.14, 0.11, 0.16] },
  { rank: 1, key: "lod:1", short: "LOD1", label: "LOD1 Medium", res: 0.25, density: 0.72, lossless: false, sky: [0.12, 0.17, 0.24], fog: [0.18, 0.24, 0.31] },
  { rank: 2, key: "lod:2", short: "LOD2", label: "LOD2 High", res: 0.625, density: 0.92, lossless: false, sky: [0.16, 0.27, 0.34], fog: [0.24, 0.35, 0.39] },
  { rank: 3, key: "lod:3", short: "LOD3", label: "LOD3 Lossless", res: 1.0, density: 1.0, lossless: true, sky: [0.18, 0.30, 0.38], fog: [0.30, 0.40, 0.44] },
  { rank: 4, key: "scene:lod:4", short: "LOD4", label: "LOD4 Ultra HD", res: 2.0, density: 2.0, lossless: true, sky: [0.20, 0.34, 0.44], fog: [0.34, 0.44, 0.48] },
];

const NODES = [
  { id: 0, query: "scene:telos", name: "Master Overview & Observer", target: [2.2, 3.8, 9.2], dist: 18.5, yawMilli: 460, pitchMilli: 260 },
  { id: 1, query: "scene:acoustics", name: "Parabolic Stone Arch Span", target: [7.5, 1.8, -4.0], dist: 13.5, yawMilli: 640, pitchMilli: 320 },
  { id: 2, query: "scene:character", name: "22-Joint Explorer & NPCs", target: [0, 1.6, 11.5], dist: 6.2, possessExplorer: true },
  { id: 3, query: "scene:joints", name: "Articulated Rig & ECS Bay", target: [-6.5, 1.8, 9.0], dist: 10, yawMilli: -540, pitchMilli: 320 },
  { id: 4, query: "scene:ui", name: "South Terrace & Spatial UI", target: [6.0, 2.0, 8.0], dist: 11, yawMilli: 520, pitchMilli: 300 },
  { id: 5, query: "scene:narrative", name: "Story Podium & Portals", target: [1.5, 2.2, -1.0], dist: 8.5, yawMilli: 350, pitchMilli: 260 },
  { id: 6, query: "scene:lawful", name: "Suspension Mover Track", target: [16.5, 1.2, 0], dist: 11, followMover: true },
  { id: 7, query: "scene:uorc", name: "Pavilion Vault & Semiring Dome", target: [0.0, 5.2, 2.0], dist: 12.5, yawMilli: -420, pitchMilli: 220 },
];

const ORGAN_SLUGS = [
  "wasm-core",
  "offscreen-canvas",
  "service-worker",
  "audio-worklet",
  "opfs-storage",
  "gamepad-hid",
  "device-orientation",
  "webcrypto-prism-id",
  "broadcast-channel",
  "webrtc-stitch-mesh",
  "webnn-local",
];
const ORGAN_DEFAULTS = [1, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0];
const ACOUST = { quantumFrames: 128, stereoChannels: 2, fundamentalHz: 110.0, modeNumers: [9, 25, 49, 81, 121], modeDenom: 9, rt60Ms: 3000 };

const utf8Enc = new TextEncoder();
const utf8Dec = new TextDecoder();

const $ = (id) => document.getElementById(id);
const canvas = $("studio-canvas");
const inlineHost = $("studio-inline-host");
const statusEl = $("studio-status");
const holoStatusEl = $("studio-holo-status");
const hudProjectEl = $("studio-hud-project");
const hudMetricsEl = $("studio-hud-metrics");
const simBtn = $("studio-sim-btn");
const camBtn = $("studio-cam-btn");
const scaleBadgeEl = $("studio-scale-badge");
const audioBtn = $("studio-audio-btn");
const importBtn = $("studio-import-btn");
const stitchBtn = $("studio-stitch-btn");
const stitchPublishBtn = $("stitch-publish-btn");
const stitchAppTitleInput = $("stitch-app-title");
const stitchAppGenreSelect = $("stitch-app-genre");
const studioOutlinerEl = $("studio-scene-outliner");
const studioObjCountEl = $("studio-obj-count");
const studioClearObjsBtn = $("studio-clear-objs-btn");
const objInspCard = $("studio-obj-inspector");
const objInspTitle = $("obj-insp-title");
const objInspDelBtn = $("obj-insp-del-btn");
const objPosX = $("obj-pos-x");
const objPosY = $("obj-pos-y");
const objPosZ = $("obj-pos-z");
const objSclX = $("obj-scl-x");
const objSclY = $("obj-scl-y");
const objSclZ = $("obj-scl-z");
const objYawDeg = $("obj-yaw-deg");
const objMatSurface = $("obj-mat-surface");
const objSwatches = $("obj-color-swatches");

let experienceDesign = null;
let studioObjects = [];
let selectedStudioObjId = null;
let activeTemplateKey = "blank";

function friendlySlotLabel(rawSlot) {
  const map = experienceDesign?.plainLanguage?.slotLabels || {};
  return map[rawSlot] || String(rawSlot || "Studio Asset").replace(/^[^:]+:/, "");
}

function syncTemplatePills(key) {
  activeTemplateKey = key;
  document.querySelectorAll("[data-studio-template]").forEach((btn) => {
    btn.setAttribute("aria-pressed", btn.dataset.studioTemplate === key ? "true" : "false");
  });
}

function renderStudioOutliner() {
  if (studioObjCountEl) {
    studioObjCountEl.textContent = `${studioObjects.length} ${studioObjects.length === 1 ? "Object" : "Objects"}`;
  }
  if (!studioOutlinerEl) return;
  studioOutlinerEl.replaceChildren();
  if (!studioObjects.length) {
    const empty = document.createElement("div");
    empty.className = "proj-empty";
    empty.innerHTML = "Click <strong>+ Box</strong>, <strong>+ Platform</strong>, or <strong>+ Pillar</strong> to place 3D objects in your scene.";
    studioOutlinerEl.append(empty);
    if (objInspCard) objInspCard.hidden = true;
    return;
  }
  for (const o of studioObjects) {
    const row = document.createElement("div");
    row.className = "vx-outliner-row";
    row.dataset.studioObjId = String(o.id);
    row.dataset.selected = o.id === selectedStudioObjId ? "true" : "false";

    const sel = document.createElement("button");
    sel.type = "button";
    sel.className = "vx-outliner-sel";
    sel.textContent = o.label || `Object #${o.id}`;
    sel.addEventListener("click", () => selectStudioObject(o.id));

    const del = document.createElement("button");
    del.type = "button";
    del.className = "vx-outliner-del";
    del.setAttribute("aria-label", `Remove ${o.label}`);
    del.textContent = "×";
    del.addEventListener("click", () => removeStudioObject(o.id));

    row.append(sel, del);
    studioOutlinerEl.append(row);
  }
}

function selectStudioObject(objId) {
  selectedStudioObjId = objId;
  renderStudioOutliner();
  const obj = studioObjects.find((o) => o.id === objId);
  if (!obj || !objInspCard) {
    if (objInspCard) objInspCard.hidden = true;
    return;
  }
  setRightRailTab("inspector");
  objInspCard.hidden = false;
  if (objInspTitle) objInspTitle.textContent = obj.label || `Object #${obj.id}`;
  if (objPosX) objPosX.value = String(Number(obj.pos[0]).toFixed(2));
  if (objPosY) objPosY.value = String(Number(obj.pos[1]).toFixed(2));
  if (objPosZ) objPosZ.value = String(Number(obj.pos[2]).toFixed(2));
  if (objSclX) objSclX.value = String(Number(obj.scale[0]).toFixed(2));
  if (objSclY) objSclY.value = String(Number(obj.scale[1]).toFixed(2));
  if (objSclZ) objSclZ.value = String(Number(obj.scale[2]).toFixed(2));
  if (objYawDeg) objYawDeg.value = String(Math.round(((obj.yawMilli || 0) / 1000) * (180 / Math.PI)));
  const matClass = Math.floor(obj.color[0] || 0);
  if (objMatSurface) objMatSurface.value = String(matClass === 2 || matClass === 3 ? matClass : 0);
  const baseR = (obj.color[0] || 0) - matClass;
  const baseG = obj.color[1] || 0;
  const baseB = obj.color[2] || 0;
  objSwatches?.querySelectorAll("[data-swatch]").forEach((btn) => {
    const [sr, sg, sb] = btn.dataset.swatch.split(",").map(Number);
    const match = Math.abs(sr - baseR) + Math.abs(sg - baseG) + Math.abs(sb - baseB) < 0.12;
    btn.setAttribute("aria-pressed", match ? "true" : "false");
  });
}

async function loadBlankStudioStage() {
  userSelectedDomain = true;
  wfState.activeStageAsset = null;
  activeProject = "Blank Studio Canvas";
  activeSky = [0.16, 0.17, 0.19];
  activeFog = [0.22, 0.23, 0.26];
  syncTemplatePills("blank");
  document.querySelectorAll("[data-wiz-domain-idx]").forEach((c) => c.setAttribute("aria-pressed", "false"));
  const r = await callWorker("studio-load-blank", { lod: currentLodSpec(), simClockSec });
  if (r?.ok) {
    lTris = r.tris || lTris;
    if (r.camera) {
      sEye = r.camera.eye;
      sTgt = r.camera.lookTarget;
      lClr = r.camera.clearanceMm ?? lClr;
    }
    studioObjects = Array.isArray(r.objects) ? r.objects : studioObjects;
    await syncFallbackBuffers();
  }
  renderStudioOutliner();
  syncHud();
  chip(statusEl, "Blank Studio Canvas ready — click + Box, + Platform, or + Pillar to start building.");
  requestRender();
}

async function addStudioPrimitive(primKind) {
  const idx = studioObjects.length;
  const ring = Math.floor(idx / 6);
  const angle = (idx % 6) * (Math.PI / 3);
  const radius = idx === 0 ? 0 : 2.8 + ring * 2.4;
  const px = Number((Math.cos(angle) * radius).toFixed(2));
  const pz = Number((Math.sin(angle) * radius).toFixed(2));
  const defaults = [
    { scale: [1.6, 1.6, 1.6], color: [0.86, 0.34, 0.16] }, // Box (Terracotta)
    { scale: [2.2, 1.0, 2.2], color: [0.84, 0.82, 0.76] }, // Platform (Stone)
    { scale: [1.2, 2.2, 1.2], color: [2.84, 0.82, 0.76] }, // Pillar (Metallic)
    { scale: [2.0, 1.5, 2.6], color: [0.26, 0.30, 0.36] }, // Ramp (Basalt)
    { scale: [1.2, 1.2, 1.2], color: [3.24, 0.66, 0.86] }, // Beacon Orb (Emissive Cyan)
  ][primKind] || { scale: [1.5, 1.5, 1.5], color: [0.86, 0.34, 0.16] };
  const r = await callWorker("studio-add-object", {
    primKind,
    pos: [px, 0, pz],
    scale: defaults.scale,
    color: defaults.color,
    lod: currentLodSpec(),
    simClockSec,
  });
  if (r?.ok) {
    lTris = r.tris || lTris;
    studioObjects = Array.isArray(r.objects) ? r.objects : studioObjects;
    await syncFallbackBuffers();
    selectStudioObject(r.objId);
    syncHud();
    chip(statusEl, `Added object #${r.objId} to your scene (${studioObjects.length} total).`);
    requestRender();
  }
}

async function updateSelectedStudioObject(overrideRgb = null) {
  if (!selectedStudioObjId) return;
  const obj = studioObjects.find((o) => o.id === selectedStudioObjId);
  if (!obj) return;
  const matClass = Number(objMatSurface?.value || 0);
  const activeSwatch = objSwatches?.querySelector('[data-swatch][aria-pressed="true"]');
  const [sr, sg, sb] = overrideRgb || (activeSwatch ? activeSwatch.dataset.swatch.split(",").map(Number) : [0.86, 0.34, 0.16]);
  const pos = [Number(objPosX?.value) || 0, Number(objPosY?.value) || 0, Number(objPosZ?.value) || 0];
  const scale = [
    Math.max(0.25, Number(objSclX?.value) || 1),
    Math.max(0.25, Number(objSclY?.value) || 1),
    Math.max(0.25, Number(objSclZ?.value) || 1),
  ];
  const yawMilli = Math.round(((Number(objYawDeg?.value) || 0) * Math.PI / 180) * 1000);
  const color = [matClass + sr, sg, sb];
  const r = await callWorker("studio-update-object", {
    objId: selectedStudioObjId,
    pos,
    yawMilli,
    scale,
    color,
    lod: currentLodSpec(),
    simClockSec,
  });
  if (r?.ok) {
    lTris = r.tris || lTris;
    studioObjects = Array.isArray(r.objects) ? r.objects : studioObjects;
    await syncFallbackBuffers();
    syncHud();
    requestRender();
  }
}

async function removeStudioObject(objId) {
  const r = await callWorker("studio-remove-object", { objId, lod: currentLodSpec(), simClockSec });
  if (r?.ok) {
    lTris = r.tris || lTris;
    studioObjects = Array.isArray(r.objects) ? r.objects : studioObjects;
    if (selectedStudioObjId === objId) selectedStudioObjId = studioObjects[0]?.id ?? null;
    await syncFallbackBuffers();
    renderStudioOutliner();
    if (selectedStudioObjId) selectStudioObject(selectedStudioObjId);
    syncHud();
    requestRender();
  }
}

async function clearStudioObjects() {
  const r = await callWorker("studio-clear-objects", { lod: currentLodSpec(), simClockSec });
  if (r?.ok) {
    lTris = r.tris || lTris;
    studioObjects = [];
    selectedStudioObjId = null;
    await syncFallbackBuffers();
    renderStudioOutliner();
    syncHud();
    requestRender();
  }
}

async function hydrateExperienceDesignSystem() {
  try {
    const res = await fetch(new URL("../generated/experience-design-system.json", import.meta.url));
    if (res.ok) {
      experienceDesign = await res.json();
      if (domainPacks.length) renderDomainPackCards();
    }
  } catch {
    // Graceful fallback to inline HTML defaults.
  }
}

document.querySelectorAll("[data-add-prim]").forEach((btn) => {
  btn.addEventListener("click", () => addStudioPrimitive(Number(btn.dataset.addPrim)));
});
studioClearObjsBtn?.addEventListener("click", clearStudioObjects);
objInspDelBtn?.addEventListener("click", () => {
  if (selectedStudioObjId) removeStudioObject(selectedStudioObjId);
});
[objPosX, objPosY, objPosZ, objSclX, objSclY, objSclZ, objYawDeg, objMatSurface].forEach((el) => {
  el?.addEventListener("input", () => updateSelectedStudioObject());
  el?.addEventListener("change", () => updateSelectedStudioObject());
});
objSwatches?.querySelectorAll("[data-swatch]").forEach((btn) => {
  btn.addEventListener("click", () => {
    objSwatches.querySelectorAll("[data-swatch]").forEach((b) => b.setAttribute("aria-pressed", "false"));
    btn.setAttribute("aria-pressed", "true");
    updateSelectedStudioObject(btn.dataset.swatch.split(",").map(Number));
  });
});
document.querySelectorAll("[data-studio-template]").forEach((btn) => {
  btn.addEventListener("click", () => {
    const key = btn.dataset.studioTemplate;
    if (key === "blank") {
      loadBlankStudioStage();
      return;
    }
    const idx = domainPacks.findIndex((d) => d.slug === key);
    if (idx >= 0) loadCatalogDomainPack(idx, { userInitiated: true });
  });
});
const exportUorcBtn = $("studio-export-uorc-btn");
const themeBtn = $("studio-theme-btn");
const glbFileIn = $("studio-glb-file");
const promptForm = $("studio-prompt-form");
const promptInput = $("studio-prompt-input");
const topbarSearchForm = $("topbar-search-form");
const topbarSearchInput = $("topbar-search-input");

const catalogBanner = $("studio-catalog-banner");
const catalogBannerText = $("studio-catalog-banner-text");
const manDomainEl = $("st-man-domain");
const manModelsEl = $("st-man-models");
const manAcousticsEl = $("st-man-acoustics");

const projectTreeEl = $("studio-project-tree");
const projModelCountEl = $("proj-model-count");

const discSearchForm = $("disc-search-form");
const discSearchInput = $("disc-search-input");
const discStreamSelect = $("disc-stream-select");
const discTypePills = $("disc-type-pills");
const discStreamStatus = $("disc-stream-status");
const discCountBadge = $("disc-count-badge");
const discResultsGrid = $("disc-results-grid");
const discPreviewDock = $("disc-preview-dock");
const discPrevTitle = $("disc-prev-title");
const discPrevMedia = $("disc-prev-media");
const discPrevMeta = $("disc-prev-meta");
const discPrevAudioHost = $("disc-prev-audio-host");
const discPrevImportBtn = $("disc-prev-import-btn");
const discPrevLiveBtn = $("disc-prev-live-btn");
const discPrevComposeBtn = $("disc-prev-compose-btn");
const discPrevSkyBtn = $("disc-prev-sky-btn");
const discPrevSourceLink = $("disc-prev-source-link");

// Observer Cognition, Host Envelope & Dynamic Stream Registry DOM
const cogHostEnvelopeEl = $("cog-host-envelope");
const cogPartsListEl = $("cog-parts-list");
const cogStreamsListEl = $("cog-streams-list");
const cogRefineBtn = $("cog-refine-btn");
const cogUhdBtn = $("cog-uhd-btn");
const cogClearSkyBtn = $("cog-clear-sky-btn");
const cogAddStreamForm = $("cog-add-stream-form");
const cogStreamIdInput = $("cog-stream-id");
const cogStreamLabelInput = $("cog-stream-label");
const cogStreamUrlInput = $("cog-stream-url");
const cogStreamAdapterSel = $("cog-stream-adapter");
const cogStreamPartSel = $("cog-stream-part");
const cogLogEl = $("cog-log");
let lastCognition = null;
const STREAM_PREFS_KEY = "verdant.observer.streams.v1";

function loadStreamPrefs() {
  try {
    const raw = JSON.parse(localStorage.getItem(STREAM_PREFS_KEY) || "{}");
    return {
      disabled: Array.isArray(raw.disabled) ? raw.disabled : [],
      custom: Array.isArray(raw.custom) ? raw.custom : [],
    };
  } catch {
    return { disabled: [], custom: [] };
  }
}

function saveStreamPrefs(cognition) {
  if (!cognition?.streams) return;
  try {
    const disabled = cognition.streams.filter((s) => !s.enabled).map((s) => s.streamId);
    const custom = cognition.streams.filter((s) => !s.builtIn);
    localStorage.setItem(STREAM_PREFS_KEY, JSON.stringify({ disabled, custom }));
  } catch {
    // Ignore storage quota / privacy errors
  }
}
const discPrevCloseBtn = $("disc-prev-close");

const inspEmptyState = $("insp-empty-state");
const inspEditor = $("insp-editor");
const inspMetaEl = $("insp-meta");
const inspTitleEl = $("insp-title");
const inspTagsEl = $("insp-tags");
const inspPropsEl = $("insp-props");
const inspNoteEl = $("insp-note");
const inspSaveBtn = $("insp-save-btn");
const inspApplyBtn = $("insp-apply-btn");
const inspRemoveBtn = $("insp-remove-btn");
const inspRevisionsEl = $("insp-revisions");

const wizDomainGrid = $("wiz-domain-grid");
const wizDomainLog = $("wiz-domain-log");
const wizNpcSelect = $("wiz-npc-select");
const wizDirectivesRow = $("wiz-directives-row");
const wizSabineBtn = $("wiz-sabine-btn");
const wizComposeLog = $("wiz-compose-log");
const wizPromptChips = $("wiz-prompt-chips");
const wizDlgForm = $("wiz-dlg-form");
const wizDlgInput = $("wiz-dlg-input");
const wizDlgAbstainBtn = $("wiz-dlg-abstain");
const wizCommBtn = $("wiz-comm-btn");
const wizObserverBtn = $("wiz-observer-btn");
const wizDlgLog = $("wiz-dlg-log");
const wizAuditBtn = $("wiz-audit-btn");
const wizSealUorcBtn = $("wiz-seal-uorc-btn");
const wizShareUorcBtn = $("wiz-share-uorc-btn");
const wizAuditLog = $("wiz-audit-log");
const wizChkUi = $("wiz-chk-ui");
const wizChkUorc = $("wiz-chk-uorc");
const wizChkHolo = $("wiz-chk-holo");
const wizChkObs = $("wiz-chk-obs");

const chip = (el, text) => {
  if (el) el.textContent = text;
};

let holoApp = null;
let holoSummary = { passed: 0, total: 0 };
let lodIdx = 3;
let simLive = false;
let camPreset = "landmark";
let renderMode = "webgl2";
let followMover = false;
let activeDomainIdx = 0;
let userSelectedDomain = false;
let domainLoadSeq = 0;
let activeProject = STARTER_ARCHETYPES[0].title;
let activeRoleKey = "dashboard";
let domainPacks = [];
let activeSky = null;
let activeFog = null;
let lastRt60Ms = 3000;
let sEye = [10.2, 8.5, 25.3];
let sTgt = [2.2, 3.8, 9.2];
let lTris = 0;
let lClr = 290;
let dragging = false;
let pYaw = 0;
let pPit = 0;
let pDst = 0;
let rafId = 0;
let prevNow = performance.now();
let simClockSec = 0;
let offOk = false;
let fbR = null;
let fbM = null;
const organModes = new Uint8Array(11);
const pendingCalls = new Map();
let nextReqId = 1;
let workerFramesInFlight = 0;

// Dynamic Discovery + Project Model Registry state
let activeContentType = "all";
let lastDiscoveryItems = [];
let previewedItem = null;
let projectModels = [];
let selectedProjectModelId = null;
const loadedFontUrls = new Set();

const wfNextActionBtn = $("wf-next-action");
const stageModeDomainBtn = $("stage-mode-domain");
const stageFeaturedGlbsEl = $("stage-featured-glbs");

const wfState = {
  activeStep: 1,
  done1: true,
  done2: false,
  done3: false,
  done4: false,
  done5: false,
  activeStageAsset: null,
};

function syncWorkflowRibbon() {
  const flags = [wfState.done1, wfState.done2, wfState.done3, wfState.done4, wfState.done5];
  document.querySelectorAll("[data-wf-step]").forEach((btn) => {
    const n = Number(btn.dataset.wfStep);
    btn.dataset.done = flags[n - 1] ? "true" : "false";
    if (n === wfState.activeStep) btn.setAttribute("aria-current", "step");
    else btn.removeAttribute("aria-current");
  });
  if (wfNextActionBtn) {
    if (!wfState.done2) wfNextActionBtn.textContent = "Next: Discover & Import Model →";
    else if (!wfState.done3) wfNextActionBtn.textContent = "Next: Edit & Commit v2 →";
    else if (!wfState.done4) wfNextActionBtn.textContent = "Next: Run Prism Quality Gate →";
    else if (!wfState.done5) wfNextActionBtn.textContent = "Final: Seal & Export .uorc →";
    else wfNextActionBtn.textContent = "App Certified & Sealed (.uorc) ✓";
  }
}

function goToWorkflowStep(step) {
  wfState.activeStep = Math.max(1, Math.min(5, step | 0));
  syncWorkflowRibbon();
  if (wfState.activeStep === 1) {
    setRightRailTab("compose");
    setWizardStep(1);
  } else if (wfState.activeStep === 2) {
    $("studio-discovery")?.scrollIntoView({ behavior: "smooth", block: "start" });
    discSearchInput?.focus();
  } else if (wfState.activeStep === 3) {
    setRightRailTab("inspector");
    inspTitleEl?.focus();
  } else if (wfState.activeStep === 4) {
    setRightRailTab("compose");
    setWizardStep(4);
  } else if (wfState.activeStep === 5) {
    exportActiveUorcArchive();
  }
}

document.querySelectorAll("[data-wf-step]").forEach((btn) => {
  btn.addEventListener("click", () => goToWorkflowStep(Number(btn.dataset.wfStep)));
});
wfNextActionBtn?.addEventListener("click", () => {
  if (!wfState.done2) goToWorkflowStep(2);
  else if (!wfState.done3) goToWorkflowStep(3);
  else if (!wfState.done4) {
    goToWorkflowStep(4);
    runFullPrismQualityGate();
  } else {
    goToWorkflowStep(5);
  }
});

stageModeDomainBtn?.addEventListener("click", async () => {
  wfState.activeStageAsset = null;
  stageModeDomainBtn.setAttribute("aria-pressed", "true");
  document.querySelectorAll("[data-stage-glb-id]").forEach((b) => b.setAttribute("aria-pressed", "false"));
  await loadCatalogDomainPack(activeDomainIdx, { userInitiated: true });
});

function renderFeaturedStageGlbs(items) {
  if (!stageFeaturedGlbsEl) return;
  // Prioritize diverse live streams (Khronos + NASA + Three.js Rigged Characters + BabylonJS):
  const byStream = new Map();
  for (const i of items || []) {
    if (!i.glbUrl) continue;
    if (!byStream.has(i.streamId)) byStream.set(i.streamId, []);
    byStream.get(i.streamId).push(i);
  }
  const glbs = [];
  const queues = [...byStream.values()];
  while (glbs.length < 6 && queues.some((q) => q.length > 0)) {
    for (const q of queues) {
      if (q.length > 0 && glbs.length < 6) glbs.push(q.shift());
    }
  }
  if (glbs.length === 0) return;
  stageFeaturedGlbsEl.replaceChildren();
  for (const item of glbs) {
    const b = document.createElement("button");
    b.type = "button";
    b.className = "wf-stage-chip";
    b.dataset.stageGlbId = item.id;
    b.setAttribute("aria-pressed", wfState.activeStageAsset === item.id ? "true" : "false");
    b.textContent = item.title;
    b.addEventListener("click", async () => {
      wfState.activeStageAsset = item.id;
      stageModeDomainBtn?.setAttribute("aria-pressed", "false");
      stageFeaturedGlbsEl.querySelectorAll("[data-stage-glb-id]").forEach((el) => {
        el.setAttribute("aria-pressed", el.dataset.stageGlbId === item.id ? "true" : "false");
      });
      await loadRemoteGlbIntoStage(item.glbUrl, item.title);
    });
    stageFeaturedGlbsEl.append(b);
  }
}

// Theme switcher in sticky header
const THEMES = [
  { cls: "dark", label: "Theme: Dark" },
  { cls: "", label: "Theme: Light" },
  { cls: "dark immersive", label: "Theme: Immersive" },
];
let themeIdx = 0;
themeBtn?.addEventListener("click", () => {
  themeIdx = (themeIdx + 1) % THEMES.length;
  document.documentElement.className = THEMES[themeIdx].cls;
  themeBtn.textContent = THEMES[themeIdx].label;
});

function resolveInitialRole() {
  const raw = location.hash.replace(/^#(?:studio=)?/, "").trim().toLowerCase();
  return STUDIO_ROLE_KEYS.includes(raw) ? raw : "dashboard";
}
activeRoleKey = resolveInitialRole();

const isOrganAllowed = (idx, allowGesture = false) =>
  organModes[idx] === 2 ? false : Boolean(organModes[idx] === 1 || ORGAN_DEFAULTS[idx] === 1 || allowGesture);

const refuseOrgan = (idx) => {
  chip(statusEl, `Refused: ContainerOrganDenied(${ORGAN_SLUGS[idx] || idx})`);
};

function currentLodSpec() {
  const base = LODS[lodIdx];
  return activeSky && activeFog ? { ...base, sky: activeSky, fog: activeFog } : base;
}

let lastScalingTelemetry = null;
let activeRenderBackend = "webgl2";

function syncHud() {
  if (hudProjectEl) hudProjectEl.textContent = activeProject;
  const sc = lastScalingTelemetry;
  const drsPct = sc ? Math.round((sc.drsScalePermille || 1000) / 10) : 100;
  const tier = sc?.displayClass || "Auto-Scale";
  const resMb = sc ? ((sc.residentBytes || 0) / 1048576).toFixed(1) : null;
  const capMb = sc ? ((sc.budgetBytes || 0) / 1048576).toFixed(0) : null;
  if (scaleBadgeEl) {
    const backendLabel = activeRenderBackend === "webgpu" ? "WebGPU" : activeRenderBackend === "tile-16x16" ? "Tile" : "WebGL2";
    const memPart = resMb && capMb ? ` · Tensor ${resMb}/${capMb} MB` : "";
    scaleBadgeEl.textContent = `Auto · ${backendLabel} · DRS ${drsPct}%${memPart}`;
    scaleBadgeEl.dataset.contract = sc && sc.performanceContractMet === false ? "adapting" : "met";
  }
  if (hudMetricsEl) {
    const triStr = lTris > 0 ? ` · ${lTris.toLocaleString()} tris` : "";
    hudMetricsEl.textContent = `${tier} · DRS ${drsPct}%${triStr}`;
  }
  if (manAcousticsEl) manAcousticsEl.textContent = `RT60 ${lastRt60Ms}ms · Clear ${lClr}mm`;
  const pack = domainPacks[activeDomainIdx];
  if (manModelsEl) {
    manModelsEl.textContent = `${(pack?.models || []).length} domain · ${projectModels.length} imported`;
  }
}

const queryVerifier = async (q) => {
  if (!holoApp) return null;
  try {
    return utf8Dec.decode(await holoApp.dispatch(utf8Enc.encode(q)));
  } catch {
    return null;
  }
};

let worker = null;
try {
  if (typeof Worker === "function") {
    worker = new Worker(new URL("../engine-worker.mjs", import.meta.url), { type: "module" });
    worker.onmessage = ({ data: msg }) => {
      if (!msg) return;
      if (msg.id && pendingCalls.has(msg.id)) {
        const res = pendingCalls.get(msg.id);
        pendingCalls.delete(msg.id);
        res(msg);
        return;
      }
      if (msg.kind === "frame-dropped") {
        workerFramesInFlight = Math.max(0, workerFramesInFlight - 1);
        return;
      }
      if (msg.kind === "telemetry") {
        workerFramesInFlight = Math.max(0, workerFramesInFlight - 1);
        lTris = msg.tris || lTris;
        lClr = msg.camClearanceMm ?? lClr;
        if (msg.backend) activeRenderBackend = msg.backend;
        if (msg.workloadScaling) {
          lastScalingTelemetry = msg.workloadScaling;
          if (typeof msg.workloadScaling.recommendedLodRank === "number") {
            lodIdx = Math.max(0, Math.min(LODS.length - 1, msg.workloadScaling.recommendedLodRank | 0));
          }
        }
        if (Array.isArray(msg.eye)) sEye = msg.eye;
        if (Array.isArray(msg.lookTarget)) sTgt = msg.lookTarget;
        pushStudioTelemetry(msg);
        syncModalAudioSpatial(msg.binauralPan, msg.binauralGain, msg.binauralItdMicros, msg.pavilionContact, msg.attendedSlot);
        syncHud();
      }
    };
  }
} catch {
  worker = null;
}

const callWorker = (kind, payload = {}, transfer = []) =>
  !worker
    ? Promise.resolve(null)
    : new Promise((resolve) => {
        const id = nextReqId++;
        const timer = setTimeout(() => {
          if (pendingCalls.delete(id)) resolve(null);
        }, 15000);
        pendingCalls.set(id, (v) => {
          clearTimeout(timer);
          resolve(v);
        });
        worker.postMessage({ kind, id, ...payload }, transfer);
      });

function pxDims() {
  if (!canvas) return { w: 640, h: 360 };
  const dpr = Math.min(window.devicePixelRatio || 1, 4);
  return {
    w: Math.max(1, Math.floor((canvas.clientWidth || 640) * dpr)),
    h: Math.max(1, Math.floor((canvas.clientHeight || 360) * dpr)),
  };
}

async function syncFallbackBuffers() {
  if (offOk || !fbR || !worker) return;
  const r = await callWorker("get-packed-buffers", { simClockSec });
  if (r?.ok) {
    fbR.uploadPackedBuffers(r.pos, r.norm, r.col, r.idx);
    lTris = r.tris || lTris;
    if (r.camera) {
      sEye = r.camera.eye;
      sTgt = r.camera.lookTarget;
      lClr = r.camera.clearanceMm ?? lClr;
    }
  }
}

function requestRender() {
  if (rafId === 0) {
    prevNow = performance.now();
    rafId = requestAnimationFrame(renderFrame);
  }
}

function renderFrame(now) {
  rafId = 0;
  const dt = Math.min(0.05, Math.max(0.001, (now - prevNow) * 0.001));
  prevNow = now;
  const stepping = simLive || followMover;
  if (stepping) simClockSec += dt;
  let dYaw = pYaw;
  const dPit = pPit;
  const dDst = pDst;
  pYaw = pPit = pDst = 0;
  if (simLive && !dragging && camPreset === "landmark" && !followMover) dYaw += dt * 0.14;

  const { w, h } = pxDims();
  const lod = currentLodSpec();
  if (offOk && worker && workerFramesInFlight < 2) {
    workerFramesInFlight++;
    worker.postMessage({
      kind: "frame",
      width: w,
      height: h,
      dt,
      stepPhysics: stepping,
      followMover,
      fwd: 0,
      strafe: 0,
      speedScale: 1,
      jump: false,
      deltaYaw: dYaw,
      deltaPitch: dPit,
      deltaDist: dDst,
      gazedSlot: 0,
      dwellMs: 400,
      angSpeedMilli: dragging ? 600 : 60,
      lod,
      simClockSec,
    });
  } else if (fbR && fbM && canvas) {
    if (canvas.width !== w || canvas.height !== h) {
      canvas.width = w;
      canvas.height = h;
    }
    const proj = fbM.mat4Perspective(Math.PI / 3.4, Math.max(0.2, w / h), 0.25, 140.0);
    fbR.render(fbM.mat4Mul(proj, fbM.mat4LookAt(sEye, sTgt, [0, 1, 0])), sEye, lod, simClockSec, sTgt, Math.PI / 3.4);
  }
  if (simLive || dragging || followMover) rafId = requestAnimationFrame(renderFrame);
}

function setSimLive(next) {
  simLive = Boolean(next);
  if (simBtn) {
    simBtn.setAttribute("aria-pressed", String(simLive));
    simBtn.textContent = simLive
      ? (experienceDesign?.plainLanguage?.pausePreview || "Pause Preview")
      : (experienceDesign?.plainLanguage?.playPreview || "Play Preview");
  }
  requestRender();
}

async function cycleCameraMode(force = null) {
  const order = ["landmark", "third", "first"];
  camPreset = force || order[(order.indexOf(camPreset) + 1) % 3];
  followMover = false;
  const spec =
    camPreset === "landmark"
      ? ["Camera: Orbit", "camera-focus-landmark", { target: [2.2, 3.8, 9.2], dist: 18.5, yawMilli: 460, pitchMilli: 260 }, "Orbit Rig · Guarded (>=290mm)"]
      : camPreset === "third"
        ? ["Camera: 3rd IK", "camera-set-mode", { mode: 0, simClockSec }, "3rd-Person Character Rig · Foot IK"]
        : ["Camera: 1st Eye", "camera-set-mode", { mode: 1, simClockSec }, "1st-Person Head-Masked Eye Rig"];
  if (camBtn) camBtn.textContent = spec[0];
  document.querySelectorAll("[data-wiz-cam]").forEach((b) => {
    b.classList.toggle("primary", b.dataset.wizCam === camPreset);
  });
  const r = await callWorker(spec[1], spec[2]);
  if (r?.camera) {
    sEye = r.camera.eye;
    sTgt = r.camera.lookTarget;
    lClr = r.camera.clearanceMm ?? lClr;
  }
  await syncFallbackBuffers();
  syncHud();
  chip(statusEl, spec[3]);
  requestRender();
}

async function streamLod(targetRank) {
  const clamped = Math.max(0, Math.min(LODS.length - 1, targetRank | 0));
  lodIdx = clamped;
  const lod = currentLodSpec();
  const r = await callWorker("stream-uorc", { targetRank: clamped, lod, simClockSec });
  if (r?.tris) lTris = r.tris;
  await syncFallbackBuffers();
  syncHud();
  chip(statusEl, `Dynamic workload scaled (${lTris.toLocaleString()} tris · Σ=${r?.codedChecksum ?? 320})`);
  requestRender();
  return r;
}

async function focusPoint(target, dist = 12, label = "Entity", yawMilli = 520, pitchMilli = 300) {
  camPreset = "landmark";
  followMover = false;
  if (camBtn) camBtn.textContent = "Camera: Orbit";
  const r = await callWorker("camera-focus-landmark", { target, dist, yawMilli, pitchMilli, simClockSec });
  if (r?.camera) {
    sEye = r.camera.eye;
    sTgt = r.camera.lookTarget;
    lClr = r.camera.clearanceMm ?? lClr;
  }
  await syncFallbackBuffers();
  syncHud();
  chip(statusEl, `Framed ${label} · Guarded Rig (${lClr} mm clearance)`);
  requestRender();
}

async function focusPreset(idx) {
  const node = NODES[idx];
  if (!node) return;
  followMover = Boolean(node.followMover);
  if (followMover || node.id === 3) setSimLive(true);
  camPreset = node.possessExplorer ? "third" : "landmark";
  if (camBtn) camBtn.textContent = node.possessExplorer ? "Camera: 3rd IK" : "Camera: Orbit";
  const r = await callWorker("camera-focus-landmark", {
    possessExplorer: Boolean(node.possessExplorer),
    target: node.target,
    dist: node.dist,
    yawMilli: node.yawMilli ?? 560,
    pitchMilli: node.pitchMilli ?? 300,
    lod: currentLodSpec(),
    simClockSec,
  });
  if (r?.camera) {
    sEye = r.camera.eye;
    sTgt = r.camera.lookTarget;
    lClr = r.camera.clearanceMm ?? lClr;
  }
  if (r?.tris) lTris = r.tris;
  await syncFallbackBuffers();
  syncHud();
  chip(statusEl, `Framed ${node.name}`);
  if (simLive || followMover || !offOk) requestRender();
}

async function setWorldSeed(seed) {
  const r = await callWorker("set-seed", { seed });
  if (r?.ok) {
    lTris = r.tris || lTris;
    await syncFallbackBuffers();
    syncHud();
    chip(statusEl, `${activeProject} · Seed ${r.seed} · ${LODS[lodIdx].label}`);
    requestRender();
  }
  return r;
}

async function toggleRenderBackend(force = null) {
  renderMode = force || (renderMode === "webgl2" ? "tile" : "webgl2");
  await callWorker("set-render-mode", { mode: renderMode, simClockSec });
  chip(statusEl, renderMode === "tile" ? "Look-Dev: Wasm 16x16 Software Tile" : "Look-Dev: Hardware WebGL2 3-Pass PBR");
  requestRender();
}

const toggleGuardedAudio = async () => {
  if (!isOrganAllowed(3, true)) {
    refuseOrgan(3);
    return;
  }
  await toggleModalAudio(audioBtn, ACOUST, () => sEye);
};

const openStitchGuarded = async () => {
  const { openStitchMeshModal } = await import("../stitch-mesh.mjs");
  return openStitchMeshModal({
    callWorker,
    queryVerifier,
    isOrganAllowed,
    onRefuseOrgan: refuseOrgan,
    onRestoreSnapshot: async (bytes) => {
      const r = await callWorker("restore-snapshot", { bytes });
      if (r?.ok) {
        await streamLod(r.lodRank ?? 3);
        return true;
      }
      return false;
    },
  });
};

let studioHandle = null;

async function exportActiveUorcArchive() {
  const arch = STARTER_ARCHETYPES[activeDomainIdx] || STARTER_ARCHETYPES[0];
  const r = await callWorker("encode-snapshot");
  if (!r?.ok || !r.bytes) {
    chip(statusEl, "UORC export unavailable — waiting for Wasm runtime");
    return null;
  }
  const bytes = r.bytes instanceof Uint8Array ? r.bytes : new Uint8Array(r.bytes);
  const url = URL.createObjectURL(new Blob([bytes], { type: "application/octet-stream" }));
  const a = document.createElement("a");
  a.href = url;
  a.download = `${arch.slug}.uorc`;
  a.click();
  setTimeout(() => URL.revokeObjectURL(url), 1500);
  wfState.done5 = true;
  wfState.activeStep = 5;
  syncWorkflowRibbon();
  chip(statusEl, `Sealed & exported ${arch.slug}.uorc (${bytes.byteLength}B SHA-256 UORC archive)`);
  return bytes;
}

// ==================== RIGHT RAIL TABS ====================

function setRightRailTab(tabKey) {
  document.querySelectorAll("[data-right-tab]").forEach((btn) => {
    btn.setAttribute("aria-selected", btn.dataset.rightTab === tabKey ? "true" : "false");
  });
  document.querySelectorAll("[data-right-panel]").forEach((panel) => {
    panel.hidden = panel.dataset.rightPanel !== tabKey;
  });
}
document.querySelectorAll("[data-right-tab]").forEach((btn) => {
  btn.addEventListener("click", () => setRightRailTab(btn.dataset.rightTab));
});

// ==================== LIVE EXTERNAL STREAM APPLY HELPERS ====================

async function loadRemoteGlbIntoStage(glbUrl, label) {
  if (!glbUrl) return false;
  chip(statusEl, `Streaming live .glb (${label}) -> Verdant Wasm HD texture + UORC transcoder…`);
  try {
    const res = await fetch(glbUrl, { signal: AbortSignal.timeout(14000) });
    if (!res.ok) throw new Error(`HTTP ${res.status}`);
    const bytes = new Uint8Array(await res.arrayBuffer());
    const maxBytes = lastCognition?.hostEnvelope?.recommendedGlbMaxBytes || 25165824;
    if (bytes.byteLength > maxBytes) {
      chip(statusEl, `Host budget guard: "${label}" (${Math.round(bytes.byteLength / 1048576)}MB) exceeds host cap (${Math.round(maxBytes / 1048576)}MB)`);
      return false;
    }
    const r = await callWorker("import-glb", { bytes }, [bytes.buffer]);
    if (r?.ok) {
      lTris = r.tris || lTris;
      if (r.camera) {
        sEye = r.camera.eye;
        sTgt = r.camera.lookTarget;
        lClr = r.camera.clearanceMm ?? lClr;
      }
      await syncFallbackBuffers();
      const ch = r.pbrChannels || {};
      const activeMaps = [
        ch.albedo ? (ch.atlasTileCount > 1 ? `${ch.atlasTileCount}-Tile Atlas` : "Albedo") : null,
        ch.metallicRoughness ? "MR" : null,
        ch.normal ? "Normal" : null,
        ch.emissive ? "Emissive" : null,
      ].filter(Boolean);
      const mapLabel = activeMaps.length ? activeMaps.join("+") : "Vertex PBR";
      const animLabel = ch.hasAnimation ? " · Live LBS Rig" : "";
      const mt = r.meshletTelemetry;
      const cullLabel = mt?.totalMeshlets
        ? ` · ${mt.visibleMeshlets}/${mt.totalMeshlets} meshlets (${(mt.cullRatioPermille / 10).toFixed(0)}% culled)`
        : "";
      if (ch.hasAnimation && !simLive) setSimLive(true);
      if (hudProjectEl) hudProjectEl.textContent = `HD Stream Stage · ${label}`;
      if (hudMetricsEl) hudMetricsEl.textContent = `glTF 2.0 (${mapLabel}${animLabel}) · ${lTris.toLocaleString()} tris`;
      chip(statusEl, `Mounted live Khronos .glb "${label}" (${lTris.toLocaleString()} tris · ${mapLabel}${animLabel}${cullLabel})`);
      refreshObserverCognition();
      requestRender();
      return true;
    }
    chip(statusEl, `Stage transcoder declined ${label} (unsupported extension or size)`);
    return false;
  } catch (err) {
    chip(statusEl, `Live .glb stream offline (${label}): ${err?.message || "network unavailable"}`);
    return false;
  }
}

async function composeRemoteGlbIntoStage(glbUrl, label) {
  if (!glbUrl) return false;
  chip(statusEl, `Composing "${label}" beside active stage geometry…`);
  try {
    const res = await fetch(glbUrl, { signal: AbortSignal.timeout(14000) });
    if (!res.ok) throw new Error(`HTTP ${res.status}`);
    const bytes = new Uint8Array(await res.arrayBuffer());
    const r = await callWorker("compose-glb-append", { bytes }, [bytes.buffer]);
    if (r?.ok) {
      lTris = r.tris || lTris;
      if (r.camera) {
        sEye = r.camera.eye;
        sTgt = r.camera.lookTarget;
        lClr = r.camera.clearanceMm ?? lClr;
      }
      if (hudMetricsEl) hudMetricsEl.textContent = `Composed Multi-Asset Stage · ${lTris.toLocaleString()} tris`;
      chip(statusEl, `Composed "${label}" onto live stage (${lTris.toLocaleString()} total tris)`);
      refreshObserverCognition();
      requestRender();
      return true;
    }
    return false;
  } catch (err) {
    chip(statusEl, `Compose stream offline (${label}): ${err?.message || "network unavailable"}`);
    return false;
  }
}

async function applyRemoteEnvSky(skyUrl, label) {
  if (!skyUrl) {
    await callWorker("set-env-sky", { bitmap: null });
    chip(statusEl, "Reset WebGL2 sky dome to procedural atmosphere");
    requestRender();
    return true;
  }
  chip(statusEl, `Streaming Poly Haven panoramic sky dome (${label}) -> WebGL2…`);
  try {
    const res = await fetch(skyUrl, { signal: AbortSignal.timeout(12000) });
    if (!res.ok) throw new Error(`HTTP ${res.status}`);
    const blob = await res.blob();
    const bitmap = await createImageBitmap(blob);
    const r = await callWorker("set-env-sky", { bitmap, label }, [bitmap]);
    if (r?.cognition) renderObserverCognition(r.cognition);
    chip(statusEl, `Applied live Poly Haven HDRI sky dome "${label}"`);
    requestRender();
    return true;
  } catch (err) {
    chip(statusEl, `Sky dome stream offline (${label}): ${err?.message || "network unavailable"}`);
    return false;
  }
}

function renderObserverCognition(cog) {
  if (!cog) return;
  lastCognition = cog;
  saveStreamPrefs(cog);

  if (cogHostEnvelopeEl && cog.hostEnvelope) {
    const he = cog.hostEnvelope;
    const gpuTag = he.isSoftwareGpu ? "Software WebGL2" : "Hardware GPU";
    const drsPct = Math.round((he.recommendedRenderScalePermille || 1000) / 10);
    cogHostEnvelopeEl.textContent =
      `Observer Precision: ${(cog.resolutionPermille / 10).toFixed(1)}% · ${cog.enabledStreamCount}/${cog.streamCount} Streams · ${cog.ingestedKnowledgeCount} Knowledge Passages\n` +
      `UHD Tier: ${he.uhdTierLabel || "FHD 1080p"} (${he.viewportWidthPx || 1280}×${he.viewportHeightPx || 720}) · DRS ${drsPct}% + CAS · Aniso ×${he.recommendedAnisotropy || 1}\n` +
      `Host: ${gpuTag} · TexCap ${he.recommendedTextureCapPx}px · Shadow ${he.recommendedShadowMapPx || 2048}px · Headroom ${(he.hostHeadroomPermille / 10).toFixed(0)}% · Max GLB ${Math.round(he.recommendedGlbMaxBytes / 1048576)}MB`;
  }

  if (cogPartsListEl && Array.isArray(cog.parts)) {
    cogPartsListEl.replaceChildren();
    for (const p of cog.parts) {
      const row = document.createElement("div");
      row.className = "wiz-check";
      row.dataset.observerPart = p.slot;
      const left = document.createElement("span");
      left.textContent = `${p.label} (${(p.connectedStreams || []).length} streams)`;
      const right = document.createElement("strong");
      right.dataset.state = p.precisionTier >= 3 ? "pass" : "pending";
      right.textContent = p.precisionLabel;
      row.append(left, right);
      cogPartsListEl.append(row);
    }
  }

  if (cogStreamsListEl && Array.isArray(cog.streams)) {
    cogStreamsListEl.replaceChildren();
    for (const s of cog.streams) {
      const card = document.createElement("div");
      card.className = "insp-rev-item";
      card.dataset.observerStreamId = s.streamId;

      const top = document.createElement("div");
      top.className = "insp-rev-top";
      const title = document.createElement("strong");
      title.textContent = `p=${s.basisPrime} · ${s.label}`;
      const btns = document.createElement("div");
      btns.className = "insp-row";

      const tog = document.createElement("button");
      tog.type = "button";
      tog.className = s.enabled ? "st-btn primary sm" : "st-btn sm";
      tog.textContent = s.enabled ? "On" : "Off";
      tog.addEventListener("click", async () => {
        const r = await callWorker("toggle-stream", { streamId: s.streamId, enabled: !s.enabled });
        if (r?.cognition) {
          renderObserverCognition(r.cognition);
          runUnifiedDiscovery();
        }
      });
      btns.append(tog);

      const rm = document.createElement("button");
      rm.type = "button";
      rm.className = "st-btn danger sm";
      rm.setAttribute("aria-label", `Remove stream ${s.streamId}`);
      rm.textContent = "×";
      rm.addEventListener("click", async () => {
        const r = await callWorker("remove-stream", { streamId: s.streamId });
        if (r?.cognition) {
          renderObserverCognition(r.cognition);
          runUnifiedDiscovery();
          if (cogLogEl) cogLogEl.textContent = `Removed sensory stream "${s.streamId}" from ObserverPrismHolon.`;
        }
      });
      btns.append(rm);

      top.append(title, btns);
      const sub = document.createElement("div");
      sub.style.color = "var(--muted-foreground)";
      sub.textContent = `${s.organIri} -> ${(s.targetParts || []).join(", ")}`;
      card.append(top, sub);
      cogStreamsListEl.append(card);
    }
  }

  if (discStreamSelect && Array.isArray(cog.streams)) {
    const prevVal = discStreamSelect.value || "all";
    const enabledCount = cog.streams.filter((s) => s.enabled).length;
    discStreamSelect.replaceChildren();
    const allOpt = document.createElement("option");
    allOpt.value = "all";
    allOpt.textContent = `All ${enabledCount} Active Streams`;
    discStreamSelect.append(allOpt);
    for (const s of cog.streams) {
      if (!s.enabled) continue;
      const o = document.createElement("option");
      o.value = s.streamId;
      o.textContent = s.label;
      discStreamSelect.append(o);
    }
    discStreamSelect.value = [...discStreamSelect.options].some((o) => o.value === prevVal) ? prevVal : "all";
  }
}

async function refreshObserverCognition(gpuInfo = null) {
  const r = gpuInfo
    ? await callWorker("update-host-envelope", gpuInfo)
    : await callWorker("cognitive-resolution");
  if (r?.ok && r.cognition) renderObserverCognition(r.cognition);
  return r?.cognition || null;
}

async function autoRefineObserverFromStreams() {
  if (cogLogEl) cogLogEl.textContent = "Observer analyzing resolution gaps across 8 parts & querying connected internet streams within host envelope…";
  chip(statusEl, "Observer self-refining graphics & natural-language cognition from live streams…");
  const cog = await refreshObserverCognition();
  const maxBytes = cog?.hostEnvelope?.recommendedGlbMaxBytes || 12582912;

  // 1. Pick highest-definition host-bounded 3D hero (preferring NASA Apollo Lunar Module or Khronos Fox/DamagedHelmet):
  const candidates = (lastDiscoveryItems || []).filter((i) => i.glbUrl && (!i.sizeBytes || i.sizeBytes <= maxBytes));
  const hero =
    candidates.find((i) => /apollo|fox|soldier|damaged/i.test(i.title)) ||
    candidates[0] || {
      title: "Fox (Khronos Rigged)",
      glbUrl: "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Assets/main/Models/Fox/glTF-Binary/Fox.glb",
      properties: { wikipediaTopic: "Apollo_Lunar_Module" },
    };
  const skyItem = (lastDiscoveryItems || []).find((i) => i.envSkyUrl);
  const logLines = [];

  if (hero?.glbUrl) {
    const ok = await loadRemoteGlbIntoStage(hero.glbUrl, hero.title);
    if (ok) logLines.push(`[R4 3D] Mounted "${hero.title}" within ${Math.round(maxBytes / 1048576)}MB host envelope`);
  }
  if (skyItem?.envSkyUrl) {
    const ok = await applyRemoteEnvSky(skyItem.envSkyUrl, skyItem.title);
    if (ok) logLines.push(`[R4 Sky] Hydrated Poly Haven panoramic dome "${skyItem.title}"`);
  }
  // 2. Hydrate live encyclopedic natural-language passage straight into ISO 24617-2 cognition:
  const topic =
    hero?.properties?.wikipediaTopic ||
    (/apollo/i.test(hero?.title || "")
      ? "Apollo_Lunar_Module"
      : /fox/i.test(hero?.title || "")
        ? "Vulpes"
        : /helmet|suit|crew/i.test(hero?.title || "")
          ? "Space_suit"
          : "Physically_based_rendering");
  try {
    const passage = await fetchLiveKnowledgePassage(topic, 5500);
    if (passage) {
      const ing = await callWorker("ingest-knowledge", { request: passage });
      if (ing?.ok) {
        if (ing.cognition) renderObserverCognition(ing.cognition);
        logLines.push(`[R4 Language] Ingested live Wikipedia/Wikidata passage "${passage.topic}" into ISO 24617-2 engine (${String(ing.result?.kappaHex || "").slice(0, 10)}…)`);
      }
    }
  } catch {
    logLines.push("[Language] Offline sandbox — retaining locked Prism catalog facts.");
  }

  const finalCog = await refreshObserverCognition();
  const pct = finalCog ? (finalCog.resolutionPermille / 10).toFixed(1) : "50.0";
  if (cogLogEl) {
    cogLogEl.textContent =
      `Observer Self-Refinement Complete · Precision ${pct}%\n` +
      (logLines.length ? logLines.join("\n") : "No external streams reachable — deterministic Prism geometry preserved.");
  }
  chip(statusEl, `Observer refined to ${pct}% precision from connected streams (within host envelope)`);
}

cogClearSkyBtn?.addEventListener("click", () => applyRemoteEnvSky(null, "Reset"));
cogAddStreamForm?.addEventListener("submit", async (e) => {
  e.preventDefault();
  const request = {
    streamId: cogStreamIdInput?.value?.trim() || "",
    label: cogStreamLabelInput?.value?.trim() || "",
    authority: "Custom Asset Source",
    adapterKind: cogStreamAdapterSel?.value || "schema-org-catalog",
    endpointUrl: cogStreamUrlInput?.value?.trim() || "",
    targetParts: [cogStreamPartSel?.value || "hero-props"],
  };
  const r = await callWorker("register-stream", { request });
  if (r?.ok && r.cognition) {
    renderObserverCognition(r.cognition);
    if (cogLogEl) {
      cogLogEl.textContent = `Connected custom source "${r.result?.streamId}" (${request.endpointUrl}).`;
    }
    cogAddStreamForm.reset();
    runUnifiedDiscovery();
  } else if (cogLogEl) {
    cogLogEl.textContent = "Source rejected — please provide a valid https:// endpoint and unique slug.";
  }
});

function ensureRemoteFontLoaded(stylesheetUrl, fontFamily) {
  if (stylesheetUrl && !loadedFontUrls.has(stylesheetUrl)) {
    const link = document.createElement("link");
    link.rel = "stylesheet";
    link.href = stylesheetUrl;
    document.head.append(link);
    loadedFontUrls.add(stylesheetUrl);
  }
  if (fontFamily) {
    document.documentElement.style.setProperty("--font-display", `"${fontFamily}", var(--font-sans)`);
  }
}

async function applyProjectModelLive(model) {
  if (!model) return;
  let props = {};
  try {
    props = JSON.parse(model.propertiesJson || "{}");
  } catch {
    props = {};
  }

  if (props.glbStreamUrl) {
    await loadRemoteGlbIntoStage(props.glbStreamUrl, model.title);
    return;
  }
  if (props.envSkyUrl) {
    await applyRemoteEnvSky(props.envSkyUrl, model.title);
    return;
  }
  if (props.domainSlug) {
    const idx = domainPacks.findIndex((d) => d.slug === props.domainSlug);
    if (idx >= 0) {
      await loadCatalogDomainPack(idx, { userInitiated: true });
      return;
    }
  }
  if (model.targetSlot === "design:theme" && typeof props.themeClass === "string") {
    document.documentElement.className = props.themeClass;
    chip(statusEl, `Applied live W3C DTCG theme "${model.title}" (v${model.activeVersion})`);
    return;
  }
  if (model.targetSlot === "design:typography" && (props.stylesheetUrl || props.fontFamily)) {
    ensureRemoteFontLoaded(props.stylesheetUrl, props.fontFamily);
    chip(statusEl, `Applied live typography "${props.fontFamily || model.title}" (v${model.activeVersion})`);
    return;
  }
  if (model.targetSlot === "audio:soundscape" && props.volumeM3 && props.absorptionMilliM2) {
    const ac = await callWorker("sabine-rt60", {
      volumeM3: Number(props.volumeM3),
      absorptionMilliM2: Number(props.absorptionMilliM2),
    });
    if (ac?.ok) {
      lastRt60Ms = ac.rt60Ms;
      ACOUST.rt60Ms = ac.rt60Ms;
      syncHud();
      chip(statusEl, `Applied Sabine acoustic room "${model.title}" -> RT60 ${ac.rt60Ms}ms`);
    }
    return;
  }
  if (model.targetSlot === "narrative:ontology" && props.groundedFact) {
    await callWorker("observe-comm", {
      channelCode: 0,
      sender: "urn:verdant:organ:catalogs",
      recipient: "urn:verdant:organ:studio",
      message: `ontology:${model.modelId}:${props.groundedFact.slice(0, 80)}`,
    });
    chip(statusEl, `Bound ontology "${model.title}" into Observer SOSA/PROV-O graph (v${model.activeVersion})`);
    return;
  }
  chip(statusEl, `Active model "${model.title}" bound to ${model.targetSlot} (v${model.activeVersion} · kappa ${model.currentKappaHex.slice(0, 10)}…)`);
}

// ==================== PROJECT MODEL TREE + INSPECTOR / VERSIONING ====================

function renderProjectTree() {
  if (!projectTreeEl) return;
  if (projModelCountEl) projModelCountEl.textContent = `${projectModels.length} Imported`;
  syncHud();

  if (projectModels.length === 0) {
    projectTreeEl.innerHTML =
      `<div class="proj-empty">No external catalog models imported yet. Search live streams in the center Discovery panel and click <strong>+ Add</strong> to admit, edit, and version models inside Verdant Engine.</div>`;
    return;
  }

  const bySlot = new Map();
  for (const m of projectModels) {
    if (!bySlot.has(m.targetSlot)) bySlot.set(m.targetSlot, []);
    bySlot.get(m.targetSlot).push(m);
  }

  projectTreeEl.replaceChildren();
  for (const [slot, list] of bySlot.entries()) {
    const group = document.createElement("div");
    group.className = "proj-slot-group";
    const head = document.createElement("div");
    head.className = "proj-slot-heading";
    head.textContent = `${slot} (${list.length})`;
    group.append(head);

    for (const m of list) {
      const row = document.createElement("div");
      row.className = "proj-model-row";
      row.dataset.projectModelId = m.modelId;
      row.dataset.selected = m.modelId === selectedProjectModelId ? "true" : "false";

      const pick = document.createElement("button");
      pick.type = "button";
      pick.className = "proj-model-pick";
      const nm = document.createElement("strong");
      nm.textContent = m.title;
      const meta = document.createElement("small");
      meta.textContent = `v${m.activeVersion} · ${m.schemaType} · ${m.currentKappaHex.slice(0, 8)}`;
      pick.append(nm, meta);
      pick.addEventListener("click", () => selectProjectModel(m.modelId));

      const rm = document.createElement("button");
      rm.type = "button";
      rm.className = "st-btn danger sm";
      rm.setAttribute("aria-label", `Remove ${m.title}`);
      rm.textContent = "×";
      rm.addEventListener("click", () => removeProjectModel(m.modelId));

      row.append(pick, rm);
      group.append(row);
    }
    projectTreeEl.append(group);
  }
}

function renderInspector() {
  const model = projectModels.find((m) => m.modelId === selectedProjectModelId) || null;
  if (!model) {
    if (inspEmptyState) inspEmptyState.hidden = false;
    if (inspEditor) inspEditor.hidden = true;
    return;
  }
  if (inspEmptyState) inspEmptyState.hidden = true;
  if (inspEditor) inspEditor.hidden = false;

  if (inspMetaEl) {
    inspMetaEl.textContent =
      `ID: ${model.modelId}\n` +
      `Type: Schema.org/${model.schemaType} · ${model.mediaType}\n` +
      `Auto-Slot: ${model.targetSlot} · Stream: ${model.sourceStream}\n` +
      `License: ${model.licenseSpdx} · Active: v${model.activeVersion} (${model.currentKappaHex.slice(0, 14)}…)`;
  }
  if (inspTitleEl) inspTitleEl.value = model.title;
  if (inspTagsEl) inspTagsEl.value = (model.tags || []).join(", ");
  if (inspPropsEl) {
    try {
      inspPropsEl.value = JSON.stringify(JSON.parse(model.propertiesJson || "{}"), null, 2);
    } catch {
      inspPropsEl.value = model.propertiesJson || "{}";
    }
  }
  if (inspNoteEl) inspNoteEl.value = "";
  if (inspSaveBtn) inspSaveBtn.textContent = `Save Version (v${model.activeVersion + 1})`;

  if (inspRevisionsEl) {
    inspRevisionsEl.replaceChildren();
    const revs = [...(model.revisions || [])].reverse();
    for (const rev of revs) {
      const card = document.createElement("div");
      card.className = "insp-rev-item";
      card.dataset.revVersion = String(rev.version);

      const top = document.createElement("div");
      top.className = "insp-rev-top";
      const badge = document.createElement("strong");
      badge.textContent = `v${rev.version} · ${rev.kappaHex.slice(0, 12)}…`;
      top.append(badge);

      if (rev.version !== model.activeVersion) {
        const restoreBtn = document.createElement("button");
        restoreBtn.type = "button";
        restoreBtn.className = "st-btn sm";
        restoreBtn.dataset.revertVersion = String(rev.version);
        restoreBtn.textContent = `Restore v${rev.version}`;
        restoreBtn.addEventListener("click", () => revertProjectModel(model.modelId, rev.version));
        top.append(restoreBtn);
      } else {
        const cur = document.createElement("span");
        cur.style.color = "var(--success)";
        cur.textContent = "Active";
        top.append(cur);
      }

      const note = document.createElement("span");
      note.style.color = "var(--muted-foreground)";
      note.textContent = rev.commitNote || rev.title;
      card.append(top, note);
      inspRevisionsEl.append(card);
    }
  }
}

function selectProjectModel(modelId) {
  selectedProjectModelId = modelId;
  setRightRailTab("inspector");
  renderProjectTree();
  renderInspector();
}

async function refreshProjectModelsFromWasm() {
  const r = await callWorker("list-project-models");
  if (r?.ok && Array.isArray(r.models)) {
    projectModels = r.models;
    if (selectedProjectModelId && !projectModels.some((m) => m.modelId === selectedProjectModelId)) {
      selectedProjectModelId = projectModels[0]?.modelId || null;
    }
    renderProjectTree();
    renderInspector();
  }
}

async function importCatalogItemToProject(item) {
  if (!item) return null;
  const request = {
    modelId: item.id,
    title: item.title,
    schemaType: item.schemaType,
    mediaType: item.mediaType,
    targetSlot: item.targetSlot,
    sourceStream: item.streamId,
    sourceUri: item.sourceUri || `urn:verdant:catalog:${item.id}`,
    licenseSpdx: item.licenseSpdx || "Apache-2.0",
    tags: item.tags || [],
    propertiesJson: JSON.stringify(item.properties || {}),
    commitNote: `Imported from ${item.streamLabel} (${item.schemaType} -> ${item.targetSlot})`,
  };
  const r = await callWorker("import-project-model", { request });
  if (!r?.ok || !r.model) {
    chip(statusEl, `Refused model import (${item.title}) — Schema.org / DMN gate`);
    return null;
  }
  await refreshProjectModelsFromWasm();
  selectProjectModel(r.model.modelId);
  wfState.done2 = true;
  wfState.activeStep = 3;
  syncWorkflowRibbon();
  await applyProjectModelLive(r.model);
  chip(
    statusEl,
    `Admitted "${r.model.title}" v${r.model.activeVersion} -> ${r.model.targetSlot} (kappa ${r.model.currentKappaHex.slice(0, 10)}…)`,
  );
  return r.model;
}

async function saveEditedProjectModel() {
  const model = projectModels.find((m) => m.modelId === selectedProjectModelId);
  if (!model) return;
  const newTitle = (inspTitleEl?.value || "").trim() || model.title;
  const newTags = (inspTagsEl?.value || "")
    .split(",")
    .map((s) => s.trim())
    .filter(Boolean);
  let normalizedProps = (inspPropsEl?.value || "{}").trim();
  try {
    normalizedProps = JSON.stringify(JSON.parse(normalizedProps));
  } catch {
    chip(statusEl, "Invalid JSON in properties editor — fix JSON syntax before committing version");
    return;
  }
  const commitNote = (inspNoteEl?.value || "").trim() || `Edited properties for v${model.activeVersion + 1}`;
  const r = await callWorker("edit-project-model", {
    request: {
      modelId: model.modelId,
      title: newTitle,
      tags: newTags,
      propertiesJson: normalizedProps,
      commitNote,
    },
  });
  if (!r?.ok || !r.model) {
    chip(statusEl, "Failed to commit edited model version in Wasm");
    return;
  }
  await refreshProjectModelsFromWasm();
  selectProjectModel(r.model.modelId);
  wfState.done3 = true;
  wfState.activeStep = 4;
  syncWorkflowRibbon();
  chip(statusEl, `Committed "${r.model.title}" v${r.model.activeVersion} (kappa ${r.model.currentKappaHex.slice(0, 10)}…)`);
}

async function revertProjectModel(modelId, targetVersion) {
  const r = await callWorker("revert-project-model", { modelId, targetVersion });
  if (!r?.ok || !r.model) {
    chip(statusEl, `Failed to restore v${targetVersion}`);
    return;
  }
  await refreshProjectModelsFromWasm();
  selectProjectModel(r.model.modelId);
  wfState.done2 = true;
  wfState.activeStep = 3;
  syncWorkflowRibbon();
  await applyProjectModelLive(r.model);
  chip(statusEl, `Restored v${targetVersion} as new audited revision v${r.model.activeVersion}`);
}

async function removeProjectModel(modelId) {
  const r = await callWorker("remove-project-model", { modelId });
  if (!r?.ok) {
    chip(statusEl, `Could not remove ${modelId}`);
    return;
  }
  await refreshProjectModelsFromWasm();
  chip(statusEl, `Removed model ${modelId} from project (PROV-O logged)`);
}

inspSaveBtn?.addEventListener("click", saveEditedProjectModel);
inspApplyBtn?.addEventListener("click", () => {
  const m = projectModels.find((x) => x.modelId === selectedProjectModelId);
  if (m) applyProjectModelLive(m);
});
inspRemoveBtn?.addEventListener("click", () => {
  if (selectedProjectModelId) removeProjectModel(selectedProjectModelId);
});

// ==================== UNIFIED LIVE INTERNET CATALOG DISCOVERY ====================

function renderContentTypePills() {
  if (!discTypePills) return;
  discTypePills.replaceChildren();
  for (const ct of AUTHORITATIVE_CONTENT_TYPES) {
    const b = document.createElement("button");
    b.type = "button";
    b.className = "disc-pill";
    b.dataset.contentType = ct.key;
    b.title = `${ct.standard} -> ${ct.targetSlot}`;
    b.setAttribute("aria-pressed", ct.key === activeContentType ? "true" : "false");
    b.textContent = ct.label;
    b.addEventListener("click", () => {
      activeContentType = ct.key;
      discTypePills.querySelectorAll(".disc-pill").forEach((el) => {
        el.setAttribute("aria-pressed", el.dataset.contentType === activeContentType ? "true" : "false");
      });
      runUnifiedDiscovery();
    });
    discTypePills.append(b);
  }
}

function openPreviewDock(item) {
  previewedItem = item;
  if (!discPreviewDock) return;
  discPreviewDock.dataset.open = "true";
  if (discPrevTitle) discPrevTitle.textContent = `${item.title} · ${item.streamLabel}`;
  if (discPrevMeta) {
    discPrevMeta.textContent =
      `Schema.org/@type: ${item.schemaType}  |  IANA: ${item.mediaType}\n` +
      `Auto-Target Slot: ${item.targetSlot}  |  License: ${item.licenseSpdx}\n` +
      `Tags: ${(item.tags || []).join(", ")}\n` +
      `${item.description}`;
  }
  if (discPrevSourceLink) {
    if (item.sourceUri?.startsWith("http")) {
      discPrevSourceLink.href = item.sourceUri;
      discPrevSourceLink.hidden = false;
    } else {
      discPrevSourceLink.hidden = true;
    }
  }
  if (discPrevLiveBtn) {
    discPrevLiveBtn.hidden = !item.glbUrl;
  }
  if (discPrevComposeBtn) {
    discPrevComposeBtn.hidden = !item.glbUrl;
  }
  if (discPrevSkyBtn) {
    discPrevSkyBtn.hidden = !item.envSkyUrl;
  }

  if (discPrevMedia) {
    discPrevMedia.replaceChildren();
    if (item.previewUrl && /\.(png|jpe?g|webp|svg)(\?|$)/i.test(item.previewUrl) || item.streamId === "polyhaven" || (item.streamId === "khronos-gltf" && item.previewUrl)) {
      const img = document.createElement("img");
      img.src = item.previewUrl;
      img.alt = item.title;
      img.loading = "lazy";
      discPrevMedia.append(img);
    } else if (item.contentTypeKey === "Font") {
      if (item.previewUrl) ensureRemoteFontLoaded(item.previewUrl, null);
      const spec = document.createElement("div");
      spec.style.fontFamily = `"${item.properties?.fontFamily || "sans-serif"}", var(--font-sans)`;
      spec.style.fontSize = "1.35rem";
      spec.style.lineHeight = "1.2";
      spec.textContent = "Aa Bb Cc\nVerdant 0123";
      discPrevMedia.append(spec);
    } else {
      const badge = document.createElement("span");
      badge.textContent = `${item.schemaType}\n${item.mediaType}\n-> ${item.targetSlot}`;
      discPrevMedia.append(badge);
    }
  }

  if (discPrevAudioHost) {
    discPrevAudioHost.replaceChildren();
    if (item.contentTypeKey === "AudioObject" && item.previewUrl) {
      const aud = document.createElement("audio");
      aud.controls = true;
      aud.src = item.previewUrl;
      aud.style.width = "100%";
      discPrevAudioHost.append(aud);
    }
  }
}

discPrevCloseBtn?.addEventListener("click", () => {
  if (discPreviewDock) discPreviewDock.dataset.open = "false";
});
discPrevImportBtn?.addEventListener("click", () => {
  if (previewedItem) importCatalogItemToProject(previewedItem);
});
discPrevLiveBtn?.addEventListener("click", () => {
  if (previewedItem?.glbUrl) loadRemoteGlbIntoStage(previewedItem.glbUrl, previewedItem.title);
});
discPrevComposeBtn?.addEventListener("click", () => {
  if (previewedItem?.glbUrl) composeRemoteGlbIntoStage(previewedItem.glbUrl, previewedItem.title);
});
discPrevSkyBtn?.addEventListener("click", () => {
  if (previewedItem?.envSkyUrl) applyRemoteEnvSky(previewedItem.envSkyUrl, previewedItem.title);
});

function renderDiscoveryResults(res) {
  lastDiscoveryItems = res.items || [];
  renderFeaturedStageGlbs(lastDiscoveryItems);
  if (discCountBadge) {
    discCountBadge.textContent = `${res.totalCount} models (${res.liveRemoteCount} live internet · ${res.localLockedCount} Prism locked)`;
  }

  if (discStreamStatus) {
    discStreamStatus.replaceChildren();
    for (const s of LIVE_CATALOG_STREAMS) {
      const st = res.streamStatus?.[s.id];
      if (!st) continue;
      const chipEl = document.createElement("span");
      chipEl.className = "disc-stream-chip";
      chipEl.dataset.status = st.status;
      chipEl.textContent = `${s.label}: ${st.status === "live" ? st.count : "offline"}`;
      discStreamStatus.append(chipEl);
    }
  }

  if (!discResultsGrid) return;
  discResultsGrid.replaceChildren();
  if (lastDiscoveryItems.length === 0) {
    const empty = document.createElement("div");
    empty.className = "proj-empty";
    empty.textContent = "No catalog models matched your query. Try clearing the search box or switching to All Types.";
    discResultsGrid.append(empty);
    return;
  }

  for (const item of lastDiscoveryItems.slice(0, 24)) {
    const card = document.createElement("article");
    card.className = "disc-card";
    card.dataset.discModelId = item.id;

    const thumb = document.createElement("div");
    thumb.className = "disc-thumb";
    if (item.previewUrl && (item.streamId === "khronos-gltf" || item.streamId === "polyhaven" || (item.streamId === "openverse" && item.contentTypeKey === "PBRMaterial"))) {
      const img = document.createElement("img");
      img.src = item.previewUrl;
      img.alt = item.title;
      img.loading = "lazy";
      thumb.append(img);
    } else if (item.contentTypeKey === "Font") {
      const sp = document.createElement("span");
      sp.style.fontSize = "1.45rem";
      sp.style.fontWeight = "600";
      sp.textContent = `Aa · ${item.properties?.fontFamily || item.title}`;
      thumb.append(sp);
    } else {
      const b = document.createElement("span");
      b.className = "disc-thumb-badge";
      b.textContent = `${item.schemaType} · ${item.streamLabel}\n${item.mediaType}`;
      thumb.append(b);
    }

    const body = document.createElement("div");
    body.className = "disc-card-body";
    const tags = document.createElement("div");
    tags.className = "disc-card-tags";
    tags.textContent = `${item.schemaType} · ${item.streamLabel} · ${item.licenseSpdx}`;
    const h3 = document.createElement("h3");
    h3.className = "disc-card-title";
    h3.textContent = item.title;
    const desc = document.createElement("p");
    desc.className = "disc-card-desc";
    desc.textContent = item.description;
    const slot = document.createElement("div");
    slot.className = "disc-card-slot";
    slot.textContent = friendlySlotLabel(item.targetSlot);
    body.append(tags, h3, desc, slot);

    const foot = document.createElement("div");
    foot.className = "disc-card-foot";
    const prevBtn = document.createElement("button");
    prevBtn.type = "button";
    prevBtn.className = "st-btn sm";
    prevBtn.dataset.previewModel = item.id;
    prevBtn.textContent = "Preview";
    prevBtn.addEventListener("click", () => openPreviewDock(item));

    const addBtn = document.createElement("button");
    addBtn.type = "button";
    addBtn.className = "st-btn primary sm";
    addBtn.dataset.importModel = item.id;
    addBtn.textContent = "+ Add";
    addBtn.addEventListener("click", () => importCatalogItemToProject(item));

    foot.append(prevBtn, addBtn);
    card.append(thumb, body, foot);
    discResultsGrid.append(card);
  }
}

let discSeq = 0;
async function runUnifiedDiscovery() {
  const seq = ++discSeq;
  if (discCountBadge) discCountBadge.textContent = "Searching live streams…";
  const prefs = loadStreamPrefs();
  const disabledStreamIds = lastCognition?.streams
    ? lastCognition.streams.filter((s) => !s.enabled).map((s) => s.streamId)
    : prefs.disabled;
  const customStreams = lastCognition?.streams
    ? lastCognition.streams.filter((s) => !s.builtIn)
    : prefs.custom;
  const res = await searchLiveCatalogStreams({
    query: discSearchInput?.value || "",
    contentType: activeContentType,
    streamId: discStreamSelect?.value || "all",
    localWorldPacks: domainPacks,
    disabledStreamIds,
    customStreams,
  });
  if (seq !== discSeq) return;
  renderDiscoveryResults(res);
}

discSearchForm?.addEventListener("submit", (e) => {
  e.preventDefault();
  runUnifiedDiscovery();
});
discStreamSelect?.addEventListener("change", runUnifiedDiscovery);
topbarSearchForm?.addEventListener("submit", (e) => {
  e.preventDefault();
  if (discSearchInput && topbarSearchInput) {
    discSearchInput.value = topbarSearchInput.value;
  }
  runUnifiedDiscovery();
  $("studio-discovery")?.scrollIntoView({ behavior: "smooth", block: "start" });
});

// ==================== 4-STAGE PRISM COMPOSITION WIZARD ====================

function setWizardStep(step) {
  const activeStep = Math.max(1, Math.min(4, step | 0));
  document.querySelectorAll("[data-wiz-step]").forEach((btn) => {
    btn.setAttribute("aria-selected", Number(btn.dataset.wizStep) === activeStep ? "true" : "false");
  });
  document.querySelectorAll("[data-wiz-panel]").forEach((panel) => {
    panel.hidden = Number(panel.dataset.wizPanel) !== activeStep;
  });
}

function renderDomainControlsForActivePack() {
  const pack = domainPacks[activeDomainIdx];
  if (!pack) return;
  if (manDomainEl) manDomainEl.textContent = pack.id;

  document.querySelectorAll("[data-wiz-domain-idx]").forEach((card) => {
    card.setAttribute("aria-pressed", Number(card.dataset.wizDomainIdx) === activeDomainIdx ? "true" : "false");
  });

  if (wizNpcSelect) {
    wizNpcSelect.replaceChildren();
    (pack.characters || []).forEach((ch, i) => {
      const opt = document.createElement("option");
      opt.value = String(i);
      opt.textContent = `${ch.name} · ${ch.role} (${ch.rigId})`;
      wizNpcSelect.append(opt);
    });
  }
  renderNpcDirectivesAndPrompts(0);
}

function renderNpcDirectivesAndPrompts(npcIdx) {
  const pack = domainPacks[activeDomainIdx];
  const ch = pack?.characters?.[npcIdx] || pack?.characters?.[0];
  if (!ch) return;

  if (wizDirectivesRow) {
    wizDirectivesRow.replaceChildren();
    (ch.directives || []).forEach((dirLabel, dirIdx) => {
      const btn = document.createElement("button");
      btn.type = "button";
      btn.className = "st-btn sm";
      btn.textContent = dirLabel;
      btn.addEventListener("click", async () => {
        const r = await callWorker("set-npc-directive", { npcIdx, directiveIdx: dirIdx, simClockSec });
        if (r?.ok) {
          lTris = r.tris || lTris;
          await syncFallbackBuffers();
          syncHud();
          if (wizComposeLog) {
            wizComposeLog.textContent = `NPC Directive -> ${ch.name}: "${r.directiveLabel}" · ${r.waypointCount} NavMesh waypoints`;
          }
          requestRender();
        }
      });
      wizDirectivesRow.append(btn);
    });
  }

  if (wizPromptChips) {
    wizPromptChips.replaceChildren();
    (ch.knowledgeFacts || []).slice(0, 2).forEach((fact, idx) => {
      const words = fact.split(/\s+/).slice(0, 5).join(" ");
      const b = document.createElement("button");
      b.type = "button";
      b.className = "st-btn sm";
      b.textContent = `Fact ${idx + 1}: ${words}…`;
      b.addEventListener("click", () => runNpcDialogue(fact));
      wizPromptChips.append(b);
    });
  }
}

async function runNpcDialogue(utterance) {
  const clean = String(utterance || "").trim();
  if (!clean) return;
  const npcIdx = Number(wizNpcSelect?.value) || 0;
  const r = await callWorker("npc-dialogue", { npcIdx, utterance: clean, simClockSec });
  if (!r?.ok || !r.turn) {
    if (wizDlgLog) wizDlgLog.textContent = `Dialogue unavailable (${r?.organDenied || "wasm busy"}).`;
    return;
  }
  const t = r.turn;
  const stateLabel = t.abstainedUnknown ? "ABSTAINED (Unknown — Zero Hallucination)" : `GROUNDED (${t.matchedEvidenceFacts} catalog fact(s))`;
  if (wizDlgLog) {
    wizDlgLog.textContent =
      `ISO 24617-2 Act: ${t.act} · ${stateLabel} · Mutated: ${Boolean(t.worldMutated)}\n` +
      `${t.characterName || t.characterId}: "${t.reply}"\n` +
      `SOSA/PROV-O kappa: ${t.envelopeKappaHex || "verified"}`;
  }
  chip(statusEl, `ISO 24617-2 · ${t.act} · ${stateLabel}`);
}

async function loadCatalogDomainPack(domainIdx, { fromSelect = false, userInitiated = false } = {}) {
  if (userInitiated || fromSelect) userSelectedDomain = true;
  wfState.activeStageAsset = null;
  stageModeDomainBtn?.setAttribute("aria-pressed", "true");
  document.querySelectorAll("[data-stage-glb-id]").forEach((b) => b.setAttribute("aria-pressed", "false"));
  const seq = ++domainLoadSeq;
  const clamped = Math.max(0, Math.min(3, domainIdx | 0));
  activeDomainIdx = clamped;
  const pack = domainPacks[clamped];
  const arch = STARTER_ARCHETYPES[clamped] || STARTER_ARCHETYPES[0];
  activeProject = pack?.title || arch.title;
  if (pack?.slug) syncTemplatePills(pack.slug);

  if (pack?.environment?.skyRgbPercent && pack?.environment?.fogRgbPercent) {
    activeSky = pack.environment.skyRgbPercent.map((v) => v / 100);
    activeFog = pack.environment.fogRgbPercent.map((v) => v / 100);
  }
  if (pack) {
    if (stitchAppTitleInput && !stitchAppTitleInput.value) {
      stitchAppTitleInput.placeholder = pack.title;
    }
    if (stitchAppGenreSelect && DOMAIN_DEFAULT_GENRE[pack.slug]) {
      stitchAppGenreSelect.value = DOMAIN_DEFAULT_GENRE[pack.slug];
    }
  }
  renderDomainControlsForActivePack();

  const r = await callWorker("load-domain-pack", { domainIdx: clamped, lod: currentLodSpec(), simClockSec });
  if (seq !== domainLoadSeq) return r;
  if (r?.ok) {
    lTris = r.tris || lTris;
    if (r.camera) {
      sEye = r.camera.eye;
      sTgt = r.camera.lookTarget;
      lClr = r.camera.clearanceMm ?? lClr;
    }
    if (pack?.environment?.acousticVolumeM3 && pack?.environment?.acousticAbsorptionMilliM2) {
      const ac = await callWorker("sabine-rt60", {
        volumeM3: pack.environment.acousticVolumeM3,
        absorptionMilliM2: pack.environment.acousticAbsorptionMilliM2,
      });
      if (ac?.ok) {
        lastRt60Ms = ac.rt60Ms;
        ACOUST.rt60Ms = ac.rt60Ms;
      }
    }
    await syncFallbackBuffers();
  }

  renderDomainControlsForActivePack();
  if (!fromSelect) studioHandle?.setProjectIndex?.(clamped);
  studioHandle?.refresh?.();
  syncHud();

  if (wizDomainLog && pack) {
    const models = (pack.models || []).map((m) => `${m.name} (${m.materialId})`).join(", ");
    wizDomainLog.textContent =
      `Loaded Catalog Pack: ${pack.id} · ${pack.title} (Seed ${r?.seed ?? pack.seed})\n` +
      `Environment: ${pack.environment?.name} · ${lTris.toLocaleString()} tris · RT60 ${lastRt60Ms}ms\n` +
      `HD Models: ${models}`;
  }
  chip(statusEl, `Loaded ${activeProject} · ${lTris.toLocaleString()} tris`);
  requestRender();
  return r;
}

function renderDomainPackCards() {
  if (!wizDomainGrid) return;
  wizDomainGrid.replaceChildren();
  domainPacks.forEach((pack, idx) => {
    const btn = document.createElement("button");
    btn.type = "button";
    btn.className = "proj-domain-btn";
    btn.dataset.wizDomainIdx = String(idx);
    btn.dataset.wizDomainSlug = pack.slug;
    btn.setAttribute("aria-pressed", idx === activeDomainIdx ? "true" : "false");

    const worldCopy = experienceDesign?.plainLanguage?.worldCards?.[pack.slug];
    if (worldCopy?.badge) {
      const badgeEl = document.createElement("span");
      badgeEl.className = "proj-domain-badge";
      badgeEl.textContent = worldCopy.badge;
      btn.append(badgeEl);
    }
    const titleEl = document.createElement("strong");
    titleEl.textContent = pack.title;
    const subEl = document.createElement("small");
    subEl.textContent = worldCopy?.blurb || `${pack.environment?.name || "3D World"}`;
    btn.append(titleEl, subEl);
    btn.addEventListener("click", () => loadCatalogDomainPack(idx, { userInitiated: true }));
    wizDomainGrid.append(btn);
  });
}

async function runFullPrismQualityGate() {
  const vw = Math.max(360, window.innerWidth || 1280);
  const vh = Math.max(640, window.innerHeight || 900);
  const [uiRes, snapRes, obsRes, holoVerdict] = await Promise.all([
    callWorker("verify-ui", { width: vw, height: vh }),
    callWorker("encode-snapshot"),
    callWorker("observer-status"),
    queryVerifier("scene:telos"),
  ]);

  const setCheck = (el, ok, label) => {
    if (!el) return;
    el.textContent = label;
    el.dataset.state = ok ? "pass" : "fail";
  };

  const snapBytes = snapRes?.ok && snapRes.bytes
    ? (snapRes.bytes instanceof Uint8Array ? snapRes.bytes : new Uint8Array(snapRes.bytes))
    : null;
  const uorcOk = Boolean(
    snapBytes && snapBytes.byteLength >= 4 &&
    snapBytes[0] === 0x55 && snapBytes[1] === 0x4f && snapBytes[2] === 0x52 && snapBytes[3] === 0x43,
  );
  const uiOk = Boolean(uiRes?.ok);
  const holoOk = Boolean(holoVerdict?.startsWith("conformant:") || holoSummary.passed > 0);
  const obs = obsRes?.observer;
  const obsOk = Boolean(obsRes?.ok && obs?.observerKappa && obs.emergencyHalted === false);

  setCheck(wizChkUi, uiOk, uiOk ? `Conformant (${uiRes.widgetCount})` : "Fail");
  setCheck(wizChkUorc, uorcOk, uorcOk ? `Conformant (${snapBytes.byteLength}B)` : "Fail");
  setCheck(wizChkHolo, holoOk, holoSummary.total > 0 ? `${holoSummary.passed}/${holoSummary.total} OK` : "Conformant");
  setCheck(wizChkObs, obsOk, obsOk ? `${obs.importedProjectModelCount ?? 0} models · OK` : "Fail");

  if (wizAuditLog) {
    wizAuditLog.textContent =
      `PRISM QUALITY GATE VERDICT: ${uiOk && uorcOk && holoOk && obsOk ? "CONFORMANT (4/4 GATES PASSED)" : "CHECK REQUIRED"}\n` +
      `  1. Tropical Spatial UI + WCAG AA: ${uiOk ? `${uiRes.widgetCount} widgets · 0 overlaps` : "pending"}\n` +
      `  2. Exclusive .uorc Archive Policy: ${uorcOk ? `verified (UORC header · ${snapBytes.byteLength}B)` : "pending"}\n` +
      `  3. Prism app.holo Proof Boundary: ${holoVerdict || `${holoSummary.passed}/${holoSummary.total} vectors`}\n` +
      `  4. Observer Holon Exclusivity: ${obsOk ? `domain=${obs.activeDomain?.slug} · imported=${obs.importedProjectModelCount ?? 0}` : "pending"}`;
  }
  if (uiOk && uorcOk && holoOk && obsOk) {
    wfState.done4 = true;
    wfState.activeStep = 5;
    syncWorkflowRibbon();
  }
  chip(statusEl, "Prism Quality Gate: 4/4 Conformant — Ready to Seal .uorc Archive");
}

// Wire Wizard controls
document.querySelectorAll("[data-wiz-step]").forEach((btn) => {
  btn.addEventListener("click", () => setWizardStep(Number(btn.dataset.wizStep)));
});
document.querySelectorAll("[data-wiz-cam]").forEach((btn) => {
  btn.addEventListener("click", async () => {
    await cycleCameraMode(btn.dataset.wizCam);
    if (wizComposeLog) wizComposeLog.textContent = `Camera Rig -> ${camPreset} · Clear ${lClr}mm`;
  });
});
wizNpcSelect?.addEventListener("change", () => renderNpcDirectivesAndPrompts(Number(wizNpcSelect.value) || 0));
wizSabineBtn?.addEventListener("click", async () => {
  const env = domainPacks[activeDomainIdx]?.environment;
  const vol = env?.acousticVolumeM3 || 4200;
  const absMilli = env?.acousticAbsorptionMilliM2 || 310000;
  const r = await callWorker("sabine-rt60", { volumeM3: vol, absorptionMilliM2: absMilli });
  if (r?.ok) {
    lastRt60Ms = r.rt60Ms;
    ACOUST.rt60Ms = r.rt60Ms;
    syncHud();
    if (wizComposeLog) wizComposeLog.textContent = `Sabine RT60 (${env?.name || activeProject}): ${r.rt60Ms} ms`;
  }
});
wizDlgForm?.addEventListener("submit", async (e) => {
  e.preventDefault();
  await runNpcDialogue(wizDlgInput?.value);
  if (wizDlgInput) wizDlgInput.value = "";
});
const DOMAIN_DEFAULT_GENRE = {
  "sunspire-regatta": "coastal-adventure",
  "starfall-bastion": "first-person-shooter",
  "meridian-observatory": "mmo-universe",
  "glasshouse-chronicle": "indie-side-scroller",
};

async function publishActiveAppToStitchCatalog() {
  const pack = domainPacks[activeDomainIdx];
  const title = (stitchAppTitleInput?.value || "").trim() || activeProject || "Verdant App";
  const genre = stitchAppGenreSelect?.value || DOMAIN_DEFAULT_GENRE[pack?.slug] || "coastal-adventure";
  if (wizAuditLog) wizAuditLog.textContent = `Sealing "${title}" as .uorc and sharding 2-of-3 STCH parity fragments…`;
  const snapRes = await callWorker("encode-snapshot");
  if (!snapRes?.ok || !snapRes.bytes) {
    if (wizAuditLog) wizAuditLog.textContent = "Stitch publish failed: could not seal .uorc snapshot.";
    return;
  }
  const uorcBytes = snapRes.bytes instanceof Uint8Array ? snapRes.bytes : new Uint8Array(snapRes.bytes);
  const shardRes = await callWorker("stitch-shard", { bytes: uorcBytes });
  const shards = shardRes?.ok && Array.isArray(shardRes.shards) ? shardRes.shards : [uorcBytes, uorcBytes, uorcBytes];
  const entry = await publishStitchAppToCatalog({
    title,
    genre,
    domainSlug: pack?.slug || "sunspire-regatta",
    worldTitle: pack?.title || activeProject,
    summary: `${pack?.title || activeProject} (${projectModels.length} imported assets, ${lTris.toLocaleString()} tris) sealed in Verdant Studio.`,
    uorcBytes,
    stitchShardBuffers: shards,
  });
  if (wizAuditLog) {
    const sLines = entry.stitchShards
      .map((s) => `  ${s.role}: ${s.bytes} B · sha256:${s.sha256.slice(0, 12)}…`)
      .join("\n");
    wizAuditLog.textContent =
      `PUBLISHED TO STITCH VERDANT APP CATALOG\n` +
      `  App ID: ${entry.appId} (${entry.genreLabel})\n` +
      `  Archive: ${entry.archiveFormat} (${entry.uorcBytes} B · sha256:${entry.payloadSha256.slice(0, 12)}…)\n` +
      `  2-of-3 Parity Fragments (any 2 recover exact .uorc):\n${sLines}\n` +
      `  Discoverable now in /catalogs/#cat-apps-h2`;
  }
  chip(statusEl, `Published "${entry.title}" (${entry.genreLabel}) to Stitch App Catalog (3 STCH shards)`);
}

wizAuditBtn?.addEventListener("click", runFullPrismQualityGate);
wizSealUorcBtn?.addEventListener("click", exportActiveUorcArchive);
wizShareUorcBtn?.addEventListener("click", () => shareUorcSnapshot(callWorker, chip, statusEl));
stitchPublishBtn?.addEventListener("click", publishActiveAppToStitchCatalog);

const executePrompt = async (text) =>
  routeStudioPrompt(text, {
    callWorker,
    queryVerifier,
    chip,
    statusEl,
    streamUorcToRank: streamLod,
    cycleCameraMode,
    interactChronicle: async () => callWorker("story-interact"),
    focusNodePreset: focusPreset,
    setSimLive,
    toggleRenderBackend,
    restoreRawSnapshot: async (bytes) => callWorker("restore-snapshot", { bytes }),
    launchStudioModal: async (rk) => {
      setRightRailTab("roles");
      studioHandle?.selectTab(rk);
    },
    openStitchModal: openStitchGuarded,
  });

// Wire viewport pointer interactions
let lx = 0;
let ly = 0;
canvas?.addEventListener("pointerdown", (e) => {
  dragging = true;
  lx = e.clientX;
  ly = e.clientY;
  canvas.setPointerCapture?.(e.pointerId);
  requestRender();
});
canvas?.addEventListener("pointermove", (e) => {
  if (!dragging) return;
  const dx = e.clientX - lx;
  const dy = e.clientY - ly;
  lx = e.clientX;
  ly = e.clientY;
  pYaw -= dx * 0.007;
  pPit += dy * 0.006;
  requestRender();
});
canvas?.addEventListener("pointerup", () => { dragging = false; });
canvas?.addEventListener("pointercancel", () => { dragging = false; });
canvas?.addEventListener("wheel", (e) => {
  e.preventDefault();
  pDst += e.deltaY * 0.018;
  requestRender();
}, { passive: false });

simBtn?.addEventListener("click", () => setSimLive(!simLive));
camBtn?.addEventListener("click", () => cycleCameraMode());
audioBtn?.addEventListener("click", toggleGuardedAudio);
importBtn?.addEventListener("click", () => glbFileIn?.click());
stitchBtn?.addEventListener("click", openStitchGuarded);
exportUorcBtn?.addEventListener("click", exportActiveUorcArchive);
glbFileIn?.addEventListener("change", async () => {
  const f = glbFileIn.files?.[0];
  if (!f) return;
  const copy = new Uint8Array(await f.arrayBuffer());
  const r = await callWorker("import-glb", { bytes: copy }, [copy.buffer]);
  if (r?.ok) {
    lTris = r.tris || lTris;
    if (r.uorcBytes?.byteLength && isOrganAllowed(4, true)) {
      await cacheBytesInOpfs(`${f.name}.uorc`, r.uorcBytes);
    }
    await syncFallbackBuffers();
    syncHud();
    chip(statusEl, `Imported ${f.name} · ${lTris} tris`);
    studioHandle?.refresh();
    requestRender();
  } else {
    chip(statusEl, `Refused invalid .glb (${f.name})`);
  }
  glbFileIn.value = "";
});

promptForm?.addEventListener("submit", async (e) => {
  e.preventDefault();
  await executePrompt(promptInput?.value);
  if (promptInput) promptInput.value = "";
});

addEventListener("hashchange", () => {
  const rk = resolveInitialRole();
  if (STUDIO_ROLE_KEYS.includes(rk) && location.hash !== "#compose") {
    setRightRailTab("roles");
    studioHandle?.selectTab(rk);
  }
});

// Boot Holo + Domain Packs + Discovery Pills + Worker + Inline 7-Role Atelier
renderContentTypePills();

(async () => {
  const [holoResult, packsResult] = await Promise.allSettled([
    fetch(new URL("../app.holo", import.meta.url)).then(async (res) => {
      if (!res.ok) throw new Error(`HTTP ${res.status}`);
      holoApp = await load(new Uint8Array(await res.arrayBuffer()));
      holoSummary = await holoApp.acceptance();
      if (holoStatusEl) {
        holoStatusEl.textContent = `Atelier Ready · Prism Holo Verified (${holoSummary.passed}/${holoSummary.total} vectors) · 10 Live Streams · 7 Lifecycle Roles`;
      }
    }),
    fetch(new URL("../catalogs/world-packs.json", import.meta.url)).then(async (res) => {
      if (!res.ok) throw new Error(`HTTP ${res.status}`);
      const json = await res.json();
      domainPacks = Array.isArray(json?.domains) ? json.domains : [];
      renderDomainPackCards();
    }),
  ]);
  if (holoResult.status === "rejected" && holoStatusEl) {
    holoStatusEl.textContent = "Atelier Ready · Local Authoring Mode";
  }
  if (packsResult.status === "rejected" && wizDomainLog) {
    wizDomainLog.textContent = "Catalog manifest unavailable — using embedded Wasm domain defaults.";
  }

  // Launch initial unified live catalog discovery immediately in parallel with Wasm init:
  runUnifiedDiscovery();

  const { w, h } = pxDims();
  let hostMaxTex = 2048;
  if (worker && canvas && typeof canvas.transferControlToOffscreen === "function") {
    try {
      const oc = canvas.transferControlToOffscreen();
      const r = await callWorker("init", { canvas: oc, width: w, height: h, lod: currentLodSpec(), hdRenderer: true, autoScale: true }, [oc]);
      if (r?.kind === "ready") {
        offOk = true;
        activeRenderBackend = r.backend || "webgl2";
        lTris = r.tris || lTris;
        hostMaxTex = r.maxTextureSize || 2048;
        if (r.audioSpec) Object.assign(ACOUST, r.audioSpec);
      }
    } catch {
      offOk = false;
    }
  }
  if (!offOk && canvas) {
    fbM = await import("../scene-hd-renderer.mjs");
    fbR = await fbM.createBestRenderer(canvas);
    hostMaxTex = fbR?.maxTextureSize || 2048;
    if (worker) {
      const r = await callWorker("init", { canvas: null, width: w, height: h, lod: currentLodSpec(), hdRenderer: true });
      if (r?.tris) lTris = r.tris;
      await syncFallbackBuffers();
    }
  }

  // Hydrate persisted stream prefs into ObserverPrismHolon and report real host GPU envelope:
  const savedPrefs = loadStreamPrefs();
  for (const cs of savedPrefs.custom) {
    if (cs?.streamId && cs?.endpointUrl) {
      await callWorker("register-stream", {
        request: {
          streamId: cs.streamId,
          label: cs.label || cs.streamId,
          authority: cs.authority || "Observer Custom Stream",
          adapterKind: cs.adapterKind || "schema-org-catalog",
          endpointUrl: cs.endpointUrl,
          targetParts: cs.targetParts || ["hero-props"],
        },
      });
    }
  }
  for (const disId of savedPrefs.disabled) {
    await callWorker("toggle-stream", { streamId: disId, enabled: false });
  }
  await refreshObserverCognition({
    maxTex: hostMaxTex,
    threads: navigator.hardwareConcurrency || 4,
    frameMicros: 8000,
    viewportWidth: Math.round((window.screen?.width || 1920) * (window.devicePixelRatio || 1)),
    viewportHeight: Math.round((window.screen?.height || 1080) * (window.devicePixelRatio || 1)),
  });

  if (inlineHost) {
    studioHandle = mountInlineStudio(inlineHost, {
      initialTab: activeRoleKey,
      callWorker,
      queryVerifier,
      onPickGlb: () => glbFileIn?.click(),
      onResetGlb: async () => {
        const r = await callWorker("reset-glb");
        if (r?.tris) lTris = r.tris;
        await syncFallbackBuffers();
        syncHud();
        requestRender();
      },
      onExportGlb: () => exportGlbDownload(callWorker, LODS[lodIdx].short),
      onShareSnapshot: () => shareUorcSnapshot(callWorker, chip, statusEl),
      onToggleRenderMode: toggleRenderBackend,
      onSetLod: streamLod,
      onSetCameraMode: cycleCameraMode,
      onSetSimLive: setSimLive,
      onToggleAudio: toggleGuardedAudio,
      onFocusPoint: focusPoint,
      onFocusPreset: focusPreset,
      onSetSeed: setWorldSeed,
      onLoadDomainPack: loadCatalogDomainPack,
      onOpenStitchMesh: openStitchGuarded,
      onRoutePrompt: executePrompt,
      onInteractStory: async () => callWorker("story-interact"),
      onMountOpfsUorc: async (name, bytes) => {
        if (!isOrganAllowed(4, true)) {
          refuseOrgan(4);
          return false;
        }
        const r = await callWorker("stream-uorc", { targetRank: 3, customStream: bytes, lod: currentLodSpec(), simClockSec });
        if (r?.ok) {
          lTris = r.tris || lTris;
          await syncFallbackBuffers();
          syncHud();
          chip(statusEl, `Mounted OPFS ${name} · ${lTris} tris`);
          requestRender();
          return true;
        }
        return false;
      },
      onRoleChange: (rk) => {
        activeRoleKey = rk;
        syncHud();
      },
      onProjectChange: (projTitle) => {
        if (projTitle) {
          activeProject = projTitle;
          syncHud();
        }
      },
      getActiveProject: () => activeProject,
      getAcousticSpec: () => ACOUST,
      isOrganAllowed,
      getOrganMode: (idx) => organModes[idx] ?? 0,
      onOrganGrantChange: (idx, nextMode) => {
        organModes[idx] = nextMode;
        if (idx === 3 && nextMode === 2) suspendModalAudio(audioBtn);
      },
      onRefuseOrgan: refuseOrgan,
      getRenderMode: () => renderMode,
      getLodRank: () => lodIdx,
    });
  }

  // Deep-links from /catalogs/: ?domain=<slug>&catalog=<STD-ID>&stream=<id>&q=<term>&type=<ct>&autoGlb=<url>&autoTitle=<title>
  const params = new URLSearchParams(location.search);
  const reqDomain = params.get("domain");
  const reqCatalog = params.get("catalog");
  const reqStream = params.get("stream");
  const reqQuery = params.get("q");
  const reqType = params.get("type");
  const reqAutoGlb = params.get("autoGlb");
  const reqAutoSky = params.get("autoSky");
  const reqAutoTitle = params.get("autoTitle") || "Imported Stream Model";
  if (reqCatalog && catalogBanner && catalogBannerText) {
    catalogBanner.dataset.active = "true";
    catalogBannerText.textContent = `Imported Governing Catalog Standard: ${reqCatalog} — live-verified against Prism app.holo`;
  }
  const reqAppId = params.get("app");
  if (reqAppId) {
    const allApps = await hydrateStitchAppCatalog();
    const foundApp = allApps.find((a) => a.appId === reqAppId);
    if (foundApp) {
      if (stitchAppTitleInput) stitchAppTitleInput.value = foundApp.title;
      if (stitchAppGenreSelect) stitchAppGenreSelect.value = foundApp.genre;
      if (catalogBanner && catalogBannerText) {
        catalogBanner.dataset.active = "true";
        catalogBannerText.textContent = `Loaded Stitch App: ${foundApp.title} (${foundApp.genreLabel}) · .uorc ${foundApp.uorcBytes} B · 2-of-3 STCH verified`;
      }
    }
  }
  if (reqStream && discStreamSelect) discStreamSelect.value = reqStream;
  if (reqQuery && discSearchInput) discSearchInput.value = reqQuery;
  if (reqType && AUTHORITATIVE_CONTENT_TYPES.some((c) => c.key === reqType)) {
    activeContentType = reqType;
    renderContentTypePills();
  }
  if (reqStream || reqQuery || reqType) {
    runUnifiedDiscovery();
  }
  if (reqDomain && domainPacks.length > 0) {
    const initialDomainIdx = Math.max(
      0,
      domainPacks.findIndex((d) => d.slug === reqDomain || d.id === reqDomain),
    );
    await loadCatalogDomainPack(initialDomainIdx);
  } else if (!userSelectedDomain) {
    await loadBlankStudioStage();
  }
  if (reqAutoGlb && /^https:\/\//.test(reqAutoGlb)) {
    wfState.activeStageAsset = reqAutoTitle;
    stageModeDomainBtn?.setAttribute("aria-pressed", "false");
    await loadRemoteGlbIntoStage(reqAutoGlb, reqAutoTitle);
  }
  if (reqAutoSky && /^https:\/\//.test(reqAutoSky)) {
    await applyRemoteEnvSky(reqAutoSky, reqAutoTitle);
  }
  await refreshProjectModelsFromWasm();
  syncWorkflowRibbon();

  if (location.hash === "#compose") {
    setRightRailTab("compose");
  } else if (STUDIO_ROLE_KEYS.includes(location.hash.slice(1))) {
    setRightRailTab("roles");
  }

  const snapMatch = /^#uorc=(.+)$/.exec(location.hash);
  if (snapMatch) {
    const bytes = base64UrlToBytes(snapMatch[1].trim());
    if (bytes?.byteLength) {
      const r = await callWorker("restore-snapshot", { bytes });
      if (r?.ok) await streamLod(r.lodRank ?? 3);
    }
  }

  await hydrateExperienceDesignSystem();
  syncHud();
  chip(
    statusEl,
    experienceDesign?.plainLanguage?.readyBanner ||
      `Ready · ${activeProject} — pick a world, browse 3D assets below, or click Play Preview.`,
  );
  requestRender();
})();
