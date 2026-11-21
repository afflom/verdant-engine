// Standalone Verdant Studio IDE Controller (`site/studio/studio-page.mjs`).
// Unifies:
//   1. Live HD 3D WebGL2/Wasm stage (`../engine-worker.mjs`) + `../app.holo`
//   2. Unified Multi-Stream Live Internet Catalog Discovery (`../catalog-streams.mjs`)
//   3. Wasm `ObserverPrismHolon` Dynamic Model Admission, Editing, Immutable `kappa`
//      Versioning (`v1 -> v2 -> ...`), Non-Destructive Revert, & Removal
//   4. 4-Stage Prism Composition & Quality Gate + 7 Lifecycle Role Workspaces (`../studio.mjs`)

import { load } from "../worker-runtime/index.mjs";
import { initStudioOutliner } from "./outliner.mjs";
import { initStudioObserverPanel } from "./observer-panel.mjs";
import { initStudioProjectInspector } from "./project-inspector.mjs";
import { bootStudioWorkspace } from "./studio-boot.mjs";
import { createStudioViewportController } from "./studio-viewport.mjs";
import { DOMAIN_DEFAULT_GENRE, initStudioDiscoveryPanel } from "./discovery-panel.mjs";
import { initStudioStageAssets, initStudioDomainWizard } from "./stage-assets.mjs";
import {
  STARTER_ARCHETYPES,
  STUDIO_ROLE_KEYS,
  base64UrlToBytes,
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

const {
  renderStudioOutliner,
  selectStudioObject,
  loadBlankStudioStage,
  addStudioPrimitive,
  updateSelectedStudioObject,
  removeStudioObject,
  clearStudioObjects,
} = initStudioOutliner({
  studioObjCountEl,
  studioOutlinerEl,
  objInspCard,
  objInspTitle,
  objPosX,
  objPosY,
  objPosZ,
  objSclX,
  objSclY,
  objSclZ,
  objYawDeg,
  objMatSurface,
  objSwatches,
  studioClearObjsBtn,
  objInspDelBtn,
  statusEl,
  callWorker: (...args) => callWorker(...args),
  chip: (...args) => chip(...args),
  currentLodSpec: () => currentLodSpec(),
  getSimClockSec: () => simClockSec,
  getStudioObjects: () => studioObjects,
  setStudioObjects: (next) => { studioObjects = next; },
  getSelectedStudioObjId: () => selectedStudioObjId,
  setSelectedStudioObjId: (next) => { selectedStudioObjId = next; },
  setRightRailTab: (tab) => setRightRailTab(tab),
  setUserSelectedDomain: (v) => { userSelectedDomain = v; },
  setActiveStageAsset: (v) => { wfState.activeStageAsset = v; },
  setActiveProject: (v) => { activeProject = v; },
  setActiveSkyFog: (sky, fog) => { activeSky = sky; activeFog = fog; },
  setTris: (v) => { lTris = v; },
  setCameraState: (cam) => { sEye = cam.eye; sTgt = cam.lookTarget; lClr = cam.clearanceMm ?? lClr; },
  syncTemplatePills: (k) => syncTemplatePills(k),
  syncFallbackBuffers: () => syncFallbackBuffers(),
  syncHud: () => syncHud(),
  requestRender: () => requestRender(),
});

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
let lastDiscoveryItems = [];
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

const {
  pxDims,
  syncFallbackBuffers,
  requestRender,
  setSimLive,
  cycleCameraMode,
  streamLod,
  focusPoint,
  focusPreset,
  setWorldSeed,
  toggleRenderBackend,
} = createStudioViewportController({
  canvas,
  simBtn,
  camBtn,
  statusEl,
  LODS,
  NODES,
  callWorker,
  chip,
  currentLodSpec,
  syncHud,
  state: {
    getOffOk: () => offOk,
    getFbR: () => fbR,
    getFbM: () => fbM,
    getWorker: () => worker,
    getSimClockSec: () => simClockSec,
    setSimClockSec: (v) => { simClockSec = v; },
    getLTris: () => lTris,
    setLTris: (v) => { lTris = v; },
    getLClr: () => lClr,
    setCamera: (eye, tgt, clr) => { sEye = eye; sTgt = tgt; lClr = clr ?? lClr; },
    getSEye: () => sEye,
    getSTgt: () => sTgt,
    getRafId: () => rafId,
    setRafId: (v) => { rafId = v; },
    getPrevNow: () => prevNow,
    setPrevNow: (v) => { prevNow = v; },
    getSimLive: () => simLive,
    setSimLive: (v) => { simLive = v; },
    getFollowMover: () => followMover,
    setFollowMover: (v) => { followMover = v; },
    getDragging: () => dragging,
    getCamPreset: () => camPreset,
    setCamPreset: (v) => { camPreset = v; },
    getWorkerFramesInFlight: () => workerFramesInFlight,
    incWorkerFramesInFlight: () => { workerFramesInFlight++; },
    consumePointerDeltas: () => { const out = { dYaw: pYaw, dPit: pPit, dDst: pDst }; pYaw = pPit = pDst = 0; return out; },
    getExperienceDesign: () => experienceDesign,
    getLodIdx: () => lodIdx,
    setLodIdx: (v) => { lodIdx = v; },
    getActiveProject: () => activeProject,
    getRenderMode: () => renderMode,
    setRenderMode: (v) => { renderMode = v; },
  },
});

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

const {
  loadRemoteGlbIntoStage,
  composeRemoteGlbIntoStage,
  applyRemoteEnvSky,
  ensureRemoteFontLoaded,
  applyProjectModelLive,
} = initStudioStageAssets({
  statusEl, hudProjectEl, hudMetricsEl, loadedFontUrls, ACOUST,
  callWorker: (...a) => callWorker(...a),
  chip: (...a) => chip(...a),
  getLastCognition: () => lastCognition,
  getSimLive: () => simLive,
  setSimLive: (v) => setSimLive(v),
  getTris: () => lTris,
  setTris: (v) => { lTris = v; },
  setCameraState: (c) => { sEye = c.eye; sTgt = c.lookTarget; lClr = c.clearanceMm ?? lClr; },
  setLastRt60Ms: (v) => { lastRt60Ms = v; },
  getDomainPacks: () => domainPacks,
  syncFallbackBuffers: () => syncFallbackBuffers(),
  syncHud: () => syncHud(),
  requestRender: () => requestRender(),
  refreshObserverCognition: () => refreshObserverCognition(),
  renderObserverCognition: (c) => renderObserverCognition(c),
  loadCatalogDomainPack: (...a) => loadCatalogDomainPack(...a),
});

const {
  renderObserverCognition,
  refreshObserverCognition,
  autoRefineObserverFromStreams,
} = initStudioObserverPanel({
  cogHostEnvelopeEl,
  cogPartsListEl,
  cogStreamsListEl,
  discStreamSelect,
  cogLogEl,
  cogClearSkyBtn,
  cogAddStreamForm,
  cogStreamIdInput,
  cogStreamLabelInput,
  cogStreamAdapterSel,
  cogStreamUrlInput,
  cogStreamPartSel,
  statusEl,
  callWorker: (...args) => callWorker(...args),
  chip: (...args) => chip(...args),
  saveStreamPrefs: (c) => saveStreamPrefs(c),
  setLastCognition: (c) => { lastCognition = c; },
  getLastDiscoveryItems: () => lastDiscoveryItems,
  runUnifiedDiscovery: () => runUnifiedDiscovery(),
  loadRemoteGlbIntoStage: (u, l) => loadRemoteGlbIntoStage(u, l),
  applyRemoteEnvSky: (u, l) => applyRemoteEnvSky(u, l),
});

// ==================== PROJECT MODEL TREE + INSPECTOR / VERSIONING ====================

const {
  renderProjectTree,
  renderInspector,
  selectProjectModel,
  refreshProjectModelsFromWasm,
  importCatalogItemToProject,
  saveEditedProjectModel,
  revertProjectModel,
  removeProjectModel,
} = initStudioProjectInspector({
  projectTreeEl,
  projModelCountEl,
  inspEmptyState,
  inspEditor,
  inspMetaEl,
  inspTitleEl,
  inspTagsEl,
  inspPropsEl,
  inspNoteEl,
  inspSaveBtn,
  inspApplyBtn,
  inspRemoveBtn,
  inspRevisionsEl,
  statusEl,
  callWorker: (...args) => callWorker(...args),
  chip: (...args) => chip(...args),
  syncHud: () => syncHud(),
  setRightRailTab: (tab) => setRightRailTab(tab),
  getProjectModels: () => projectModels,
  setProjectModels: (next) => { projectModels = next; },
  getSelectedProjectModelId: () => selectedProjectModelId,
  setSelectedProjectModelId: (next) => { selectedProjectModelId = next; },
  wfState,
  syncWorkflowRibbon: () => syncWorkflowRibbon(),
  applyProjectModelLive: (m) => applyProjectModelLive(m),
});

// ==================== UNIFIED LIVE INTERNET CATALOG DISCOVERY ====================
const {
  renderContentTypePills,
  openPreviewDock,
  renderDiscoveryResults,
  runUnifiedDiscovery,
  runFullPrismQualityGate,
  publishActiveAppToStitchCatalog,
  setActiveContentType,
} = initStudioDiscoveryPanel({
  $, discSearchForm, discSearchInput, discStreamSelect, discTypePills, discStreamStatus,
  discCountBadge, discResultsGrid, discPreviewDock, discPrevTitle, discPrevMedia, discPrevMeta,
  discPrevAudioHost, discPrevImportBtn, discPrevLiveBtn, discPrevComposeBtn, discPrevSkyBtn,
  discPrevSourceLink, discPrevCloseBtn, topbarSearchForm, topbarSearchInput, wizChkUi, wizChkUorc,
  wizChkHolo, wizChkObs, wizAuditLog, stitchAppTitleInput, stitchAppGenreSelect, statusEl, wfState,
  callWorker: (...a) => callWorker(...a),
  queryVerifier: (...a) => queryVerifier(...a),
  chip: (...a) => chip(...a),
  syncWorkflowRibbon: () => syncWorkflowRibbon(),
  friendlySlotLabel: (s) => friendlySlotLabel(s),
  loadStreamPrefs: () => loadStreamPrefs(),
  getLastCognition: () => lastCognition,
  getDomainPacks: () => domainPacks,
  getActiveDomainIdx: () => activeDomainIdx,
  getActiveProject: () => activeProject,
  getProjectModels: () => projectModels,
  getLiveTris: () => lTris,
  getHoloSummary: () => holoSummary,
  setLastDiscoveryItems: (items) => { lastDiscoveryItems = items; },
  renderFeaturedStageGlbs: (items) => renderFeaturedStageGlbs(items),
  ensureRemoteFontLoaded: (...a) => ensureRemoteFontLoaded(...a),
  importCatalogItemToProject: (item) => importCatalogItemToProject(item),
  loadRemoteGlbIntoStage: (...a) => loadRemoteGlbIntoStage(...a),
  composeRemoteGlbIntoStage: (...a) => composeRemoteGlbIntoStage(...a),
  applyRemoteEnvSky: (...a) => applyRemoteEnvSky(...a),
});

// ==================== 4-STAGE PRISM COMPOSITION WIZARD ====================
const {
  setWizardStep,
  renderDomainControlsForActivePack,
  renderNpcDirectivesAndPrompts,
  runNpcDialogue,
  loadCatalogDomainPack,
  renderDomainPackCards,
} = initStudioDomainWizard({
  manDomainEl, wizNpcSelect, wizDirectivesRow, wizComposeLog, wizPromptChips, wizDlgLog,
  wizDomainLog, wizDomainGrid, stageModeDomainBtn, stitchAppTitleInput, stitchAppGenreSelect,
  statusEl, wfState, ACOUST, STARTER_ARCHETYPES, DOMAIN_DEFAULT_GENRE,
  callWorker: (...a) => callWorker(...a),
  chip: (...a) => chip(...a),
  currentLodSpec: () => currentLodSpec(),
  syncTemplatePills: (k) => syncTemplatePills(k),
  syncFallbackBuffers: () => syncFallbackBuffers(),
  syncHud: () => syncHud(),
  requestRender: () => requestRender(),
  getDomainPacks: () => domainPacks,
  getActiveDomainIdx: () => activeDomainIdx,
  setActiveDomainIdx: (v) => { activeDomainIdx = v; },
  getActiveProject: () => activeProject,
  setActiveProject: (v) => { activeProject = v; },
  setUserSelectedDomain: (v) => { userSelectedDomain = v; },
  setActiveSkyFog: (sky, fog) => { activeSky = sky; activeFog = fog; },
  getTris: () => lTris,
  setTris: (v) => { lTris = v; },
  setCameraState: (c) => { sEye = c.eye; sTgt = c.lookTarget; lClr = c.clearanceMm ?? lClr; },
  getLastRt60Ms: () => lastRt60Ms,
  setLastRt60Ms: (v) => { lastRt60Ms = v; },
  getSimClockSec: () => simClockSec,
  getExperienceDesign: () => experienceDesign,
  getStudioHandle: () => studioHandle,
});

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

  await bootStudioWorkspace({
    $,
    LODS,
    STUDIO_ROLE_KEYS,
    ACOUST,
    canvas,
    worker,
    inlineHost,
    glbFileIn,
    statusEl,
    scaleBadgeEl,
    audioBtn,
    catalogBanner,
    catalogBannerText,
    stitchAppTitleInput,
    stitchAppGenreSelect,
    discStreamSelect,
    discSearchInput,
    stageModeDomainBtn,
    wfState,
    pxDims,
    callWorker,
    queryVerifier,
    currentLodSpec,
    syncFallbackBuffers,
    loadStreamPrefs,
    refreshObserverCognition,
    mountInlineStudio,
    exportGlbDownload,
    shareUorcSnapshot,
    toggleRenderBackend,
    streamLod,
    cycleCameraMode,
    setSimLive,
    toggleGuardedAudio,
    focusPoint,
    focusPreset,
    setWorldSeed,
    loadCatalogDomainPack,
    loadBlankStudioStage,
    loadRemoteGlbIntoStage,
    applyRemoteEnvSky,
    openStitchGuarded,
    executePrompt,
    isOrganAllowed,
    refuseOrgan,
    suspendModalAudio,
    setActiveContentType,
    runUnifiedDiscovery,
    refreshProjectModelsFromWasm,
    syncWorkflowRibbon,
    setRightRailTab,
    base64UrlToBytes,
    hydrateExperienceDesignSystem,
    clearStudioObjects,
    renderStudioOutliner,
    syncHud,
    chip,
    requestRender,
    state: {
      getOffOk: () => offOk,
      setOffOk: (v) => { offOk = v; },
      setActiveRenderBackend: (v) => { activeRenderBackend = v; },
      getLTris: () => lTris,
      setLTris: (v) => { lTris = v; },
      setFallbackRenderer: (m, r) => { fbM = m; fbR = r; },
      getActiveRoleKey: () => activeRoleKey,
      setActiveRoleKey: (v) => { activeRoleKey = v; },
      getActiveProject: () => activeProject,
      setActiveProject: (v) => { activeProject = v; },
      getActiveDomainIdx: () => activeDomainIdx,
      getDomainPacks: () => domainPacks,
      getUserSelectedDomain: () => userSelectedDomain,
      getStudioObjects: () => studioObjects,
      setStudioObjects: (v) => { studioObjects = v; },
      getLodIdx: () => lodIdx,
      getSimClockSec: () => simClockSec,
      getOrganMode: (idx) => organModes[idx] ?? 0,
      setOrganMode: (idx, nextMode) => { organModes[idx] = nextMode; },
      getRenderMode: () => renderMode,
      setStudioHandle: (h) => { studioHandle = h; },
      getExperienceDesign: () => experienceDesign,
    },
  });
})();
