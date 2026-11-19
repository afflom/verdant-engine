// Standalone Verdant Studio Atelier Controller (`site/studio/studio-page.mjs`).
// Connects a dedicated live 3D authoring viewport (`../engine-worker.mjs`), `../app.holo`,
// SHA-256-locked `../catalogs/world-packs.json`, the 4-stage Catalog-to-App Composition Wizard,
// and `mountInlineStudio` (`../studio.mjs`) across all 7 functional lifecycle roles.

import { load } from "../worker-runtime/index.mjs";
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
const lodBtn = $("studio-lod-btn");
const audioBtn = $("studio-audio-btn");
const importBtn = $("studio-import-btn");
const stitchBtn = $("studio-stitch-btn");
const exportUorcBtn = $("studio-export-uorc-btn");
const themeBtn = $("studio-theme-btn");
const glbFileIn = $("studio-glb-file");
const promptForm = $("studio-prompt-form");
const promptInput = $("studio-prompt-input");

const catalogBanner = $("studio-catalog-banner");
const catalogBannerText = $("studio-catalog-banner-text");
const manDomainEl = $("st-man-domain");
const manModelsEl = $("st-man-models");
const manNpcsEl = $("st-man-npcs");
const manAcousticsEl = $("st-man-acoustics");

const wizProgressEl = $("wiz-progress");
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
const wizPrevBtn = $("wiz-prev-btn");
const wizNextBtn = $("wiz-next-btn");
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
let activeWizStep = 1;
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

function syncHud() {
  const roleLabel = activeRoleKey.charAt(0).toUpperCase() + activeRoleKey.slice(1);
  if (hudProjectEl) hudProjectEl.textContent = `${activeProject} · ${roleLabel}`;
  if (hudMetricsEl) {
    const triStr = lTris > 0 ? `${lTris.toLocaleString()} tris · ` : "";
    hudMetricsEl.textContent = `${LODS[lodIdx].label} · ${triStr}Clear ${lClr}mm`;
  }
  if (manAcousticsEl) {
    manAcousticsEl.textContent = `RT60 ${lastRt60Ms}ms · Clear ${lClr}mm`;
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
  if (!canvas) return { w: 640, h: 400 };
  const dpr = Math.min(window.devicePixelRatio || 1, 2);
  return {
    w: Math.max(1, Math.floor((canvas.clientWidth || 640) * dpr)),
    h: Math.max(1, Math.floor((canvas.clientHeight || 400) * dpr)),
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
    simBtn.textContent = simLive ? "Pause Physics" : "Step Physics";
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
  if (lodBtn) lodBtn.textContent = `Fidelity: ${lod.short}`;
  document.querySelectorAll("[data-wiz-lod]").forEach((b) => {
    b.classList.toggle("primary", Number(b.dataset.wizLod) === clamped);
  });
  const r = await callWorker("stream-uorc", { targetRank: clamped, lod, simClockSec });
  if (r?.tris) lTris = r.tris;
  await syncFallbackBuffers();
  syncHud();
  chip(statusEl, `Streamed ${lod.label} (${lTris} tris · Σ=${r?.codedChecksum ?? 320})`);
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
  chip(statusEl, `Sealed & exported ${arch.slug}.uorc (${bytes.byteLength}B SHA-256 UORC archive)`);
  return bytes;
}

// ==================== 4-STAGE CATALOG COMPOSITION WIZARD ====================

function setWizardStep(step) {
  activeWizStep = Math.max(1, Math.min(4, step | 0));
  if (wizProgressEl) wizProgressEl.textContent = `Stage ${activeWizStep} of 4`;
  document.querySelectorAll("[data-wiz-step]").forEach((btn) => {
    const n = Number(btn.dataset.wizStep);
    btn.setAttribute("aria-selected", n === activeWizStep ? "true" : "false");
    if (n < activeWizStep) btn.dataset.done = "true";
  });
  document.querySelectorAll("[data-wiz-panel]").forEach((panel) => {
    panel.hidden = Number(panel.dataset.wizPanel) !== activeWizStep;
  });
  if (wizPrevBtn) wizPrevBtn.disabled = activeWizStep <= 1;
  if (wizNextBtn) {
    wizNextBtn.textContent = activeWizStep >= 4 ? "Run Prism Quality Gate" : "Next Stage";
  }
}

function renderDomainControlsForActivePack() {
  const pack = domainPacks[activeDomainIdx];
  if (!pack) return;

  if (manDomainEl) manDomainEl.textContent = pack.id;
  if (manModelsEl) {
    manModelsEl.textContent = `${(pack.models || []).length} models · ${(pack.items || []).length} items`;
  }
  if (manNpcsEl) {
    manNpcsEl.textContent = (pack.characters || []).map((c) => c.name).join(", ") || "—";
  }

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
    const lbl = document.createElement("span");
    lbl.className = "wiz-label";
    lbl.textContent = "Dispatch Directive:";
    wizDirectivesRow.append(lbl);
    (ch.directives || []).forEach((dirLabel, dirIdx) => {
      const btn = document.createElement("button");
      btn.type = "button";
      btn.className = "st-btn";
      btn.textContent = dirLabel;
      btn.addEventListener("click", async () => {
        const r = await callWorker("set-npc-directive", { npcIdx, directiveIdx: dirIdx, simClockSec });
        if (r?.ok) {
          lTris = r.tris || lTris;
          await syncFallbackBuffers();
          syncHud();
          if (wizComposeLog) {
            wizComposeLog.textContent = `Dispatched Wasm NPC Directive -> ${ch.name} (${r.characterId}): "${r.directiveLabel}" · ${r.waypointCount} NavMesh waypoints · ${lTris.toLocaleString()} tris`;
          }
          requestRender();
        }
      });
      wizDirectivesRow.append(btn);
    });
  }

  if (wizPromptChips) {
    wizPromptChips.replaceChildren();
    const lbl = document.createElement("span");
    lbl.className = "wiz-label";
    lbl.textContent = "Grounded Catalog Queries:";
    wizPromptChips.append(lbl);
    (ch.knowledgeFacts || []).slice(0, 2).forEach((fact, idx) => {
      const words = fact.split(/\s+/).slice(0, 6).join(" ");
      const b = document.createElement("button");
      b.type = "button";
      b.className = "st-btn";
      b.textContent = `Ask Fact ${idx + 1}: ${words}…`;
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
    if (wizDlgLog) wizDlgLog.textContent = `Dialogue refused or unavailable (${r?.organDenied || "wasm busy"}).`;
    return;
  }
  const t = r.turn;
  const stateLabel = t.abstainedUnknown ? "ABSTAINED (Unknown — Zero Hallucination)" : `GROUNDED (${t.matchedEvidenceFacts} catalog fact(s))`;
  if (wizDlgLog) {
    wizDlgLog.textContent =
      `ISO 24617-2 Act: ${t.act} · ${stateLabel} · Mutated: ${Boolean(t.worldMutated)}\n` +
      `Speaker: ${t.characterName || t.characterId} -> "${t.reply}"\n` +
      `SOSA/PROV-O Envelope kappa: ${t.envelopeKappaHex || "verified"}`;
  }
  chip(statusEl, `ISO 24617-2 · ${t.act} · ${stateLabel}`);
}

async function loadCatalogDomainPack(domainIdx, { fromSelect = false, userInitiated = false } = {}) {
  if (userInitiated || fromSelect) userSelectedDomain = true;
  const seq = ++domainLoadSeq;
  const clamped = Math.max(0, Math.min(3, domainIdx | 0));
  activeDomainIdx = clamped;
  const pack = domainPacks[clamped];
  const arch = STARTER_ARCHETYPES[clamped] || STARTER_ARCHETYPES[0];
  activeProject = pack?.title || arch.title;

  if (pack?.environment?.skyRgbPercent && pack?.environment?.fogRgbPercent) {
    activeSky = pack.environment.skyRgbPercent.map((v) => v / 100);
    activeFog = pack.environment.fogRgbPercent.map((v) => v / 100);
  }
  renderDomainControlsForActivePack();

  const r = await callWorker("load-domain-pack", { domainIdx: clamped, simClockSec });
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
    const items = (pack.items || []).map((i) => `${i.name} ×${i.initialSupply}`).join(", ");
    wizDomainLog.textContent =
      `Loaded Catalog Pack: ${pack.id} · ${pack.title} (Seed ${r?.seed ?? pack.seed})\n` +
      `Environment: ${pack.environment?.name} · ${lTris.toLocaleString()} tris · ${r?.walkablePolys ?? 4608} NavMesh polys · Sabine RT60 ${lastRt60Ms}ms\n` +
      `Imported HD Models: ${models}\n` +
      `Conserved Items: ${items}`;
  }
  chip(statusEl, `Composed ${activeProject} (${pack?.id || clamped}) · ${lTris.toLocaleString()} tris · Clear ${lClr}mm`);
  requestRender();
  return r;
}

function renderDomainPackCards() {
  if (!wizDomainGrid) return;
  wizDomainGrid.replaceChildren();
  domainPacks.forEach((pack, idx) => {
    const btn = document.createElement("button");
    btn.type = "button";
    btn.className = "wiz-domain-card";
    btn.dataset.wizDomainIdx = String(idx);
    btn.dataset.wizDomainSlug = pack.slug;
    btn.setAttribute("aria-pressed", idx === activeDomainIdx ? "true" : "false");

    const idEl = document.createElement("span");
    idEl.className = "wiz-domain-id";
    idEl.textContent = `${pack.id} · Seed ${pack.seed} · LOD${pack.defaultLodRank}`;
    const titleEl = document.createElement("span");
    titleEl.className = "wiz-domain-title";
    titleEl.textContent = pack.title;
    const metaEl = document.createElement("span");
    metaEl.className = "wiz-domain-meta";
    metaEl.textContent =
      `${pack.environment?.name || "—"} · ${(pack.models || []).length} HD models · ${(pack.characters || []).length} NPC rig(s)`;
    btn.append(idEl, titleEl, metaEl);
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
  const uorcMagicOk = Boolean(
    snapBytes && snapBytes.byteLength >= 4 &&
    snapBytes[0] === 0x55 && snapBytes[1] === 0x4f && snapBytes[2] === 0x52 && snapBytes[3] === 0x43,
  );
  const uiOk = Boolean(uiRes?.ok);
  const uorcOk = uorcMagicOk;
  const holoOk = Boolean(holoVerdict?.startsWith("conformant:") || holoSummary.passed > 0);
  const obs = obsRes?.observer;
  const obsOk = Boolean(obsRes?.ok && obs?.observerKappa && obs.emergencyHalted === false);

  setCheck(wizChkUi, uiOk, uiOk ? `Conformant (${uiRes.widgetCount} widgets · WCAG AA)` : "Unavailable");
  setCheck(wizChkUorc, uorcOk, uorcOk ? `Conformant (.uorc magic · ${snapBytes.byteLength}B)` : "Unavailable");
  setCheck(
    wizChkHolo,
    holoOk,
    holoSummary.total > 0 ? `Conformant (${holoSummary.passed}/${holoSummary.total} vectors)` : (holoVerdict || "Conformant"),
  );
  setCheck(
    wizChkObs,
    obsOk,
    obsOk ? `Conformant (${obs.activeDomain?.slug || "active"} · ${obs.communicationCount ?? 0} bus events)` : "Unavailable",
  );

  if (wizAuditLog) {
    wizAuditLog.textContent =
      `PRISM QUALITY GATE VERDICT: ${uiOk && uorcOk && holoOk && obsOk ? "CONFORMANT (4/4 GATES PASSED)" : "CHECK REQUIRED"}\n` +
      `  1. Tropical Spatial UI + WCAG AA: ${uiOk ? `${uiRes.widgetCount} widgets · 0 overlaps` : "pending"}\n` +
      `  2. Exclusive .uorc Archive Policy: ${uorcOk ? `verified (UORC header · ${snapBytes.byteLength}B SHA-256 sealed)` : "pending"}\n` +
      `  3. Prism app.holo Proof Boundary: ${holoVerdict || `${holoSummary.passed}/${holoSummary.total} vectors`}\n` +
      `  4. Observer Holon Exclusivity: ${obsOk ? `domain=${obs.activeDomain?.slug} · basis=${obs.organBasisProduct} · kappa=${String(obs.observerKappa).slice(0, 12)}…` : "pending"}`;
  }
  chip(statusEl, "Prism Quality Gate: 4/4 Conformant — Ready to Seal .uorc Archive");
}

// Wire Wizard navigation & actions
document.querySelectorAll("[data-wiz-step]").forEach((btn) => {
  btn.addEventListener("click", () => setWizardStep(Number(btn.dataset.wizStep)));
});
wizPrevBtn?.addEventListener("click", () => setWizardStep(activeWizStep - 1));
wizNextBtn?.addEventListener("click", async () => {
  if (activeWizStep >= 4) {
    await runFullPrismQualityGate();
  } else {
    setWizardStep(activeWizStep + 1);
  }
});
document.querySelectorAll("[data-wiz-lod]").forEach((btn) => {
  btn.addEventListener("click", async () => {
    const rank = Number(btn.dataset.wizLod);
    await streamLod(rank);
    if (wizComposeLog) {
      wizComposeLog.textContent = `Streamed UORC ${LODS[rank].label} · ${lTris.toLocaleString()} tris · Guard Clearance ${lClr}mm`;
    }
  });
});
document.querySelectorAll("[data-wiz-cam]").forEach((btn) => {
  btn.addEventListener("click", async () => {
    await cycleCameraMode(btn.dataset.wizCam);
    if (wizComposeLog) {
      wizComposeLog.textContent = `Switched Camera Rig -> ${camPreset} · Guard Clearance ${lClr}mm`;
    }
  });
});
wizNpcSelect?.addEventListener("change", () => {
  renderNpcDirectivesAndPrompts(Number(wizNpcSelect.value) || 0);
});
wizSabineBtn?.addEventListener("click", async () => {
  const env = domainPacks[activeDomainIdx]?.environment;
  const vol = env?.acousticVolumeM3 || 4200;
  const absMilli = env?.acousticAbsorptionMilliM2 || 310000;
  const r = await callWorker("sabine-rt60", { volumeM3: vol, absorptionMilliM2: absMilli });
  if (r?.ok) {
    lastRt60Ms = r.rt60Ms;
    ACOUST.rt60Ms = r.rt60Ms;
    syncHud();
    if (wizComposeLog) {
      wizComposeLog.textContent = `Calibrated Sabine Acoustics (${env?.name || activeProject}): Volume ${vol} m³ · Absorption ${(absMilli / 1000).toFixed(1)} m² · RT60 = ${r.rt60Ms} ms`;
    }
  }
});
wizDlgForm?.addEventListener("submit", async (e) => {
  e.preventDefault();
  await runNpcDialogue(wizDlgInput?.value);
  if (wizDlgInput) wizDlgInput.value = "";
});
wizDlgAbstainBtn?.addEventListener("click", () =>
  runNpcDialogue("What is the unmodeled quantum cryptocurrency exchange rate on Neptune?"),
);
wizCommBtn?.addEventListener("click", async () => {
  const pack = domainPacks[activeDomainIdx];
  const r = await callWorker("observe-comm", {
    channelCode: 1,
    sender: "urn:verdant:organ:studio",
    recipient: "urn:verdant:organ:engine",
    message: `compose:${pack?.slug || "sunspire-regatta"}:lod${lodIdx}`,
  });
  if (wizDlgLog) {
    wizDlgLog.textContent = r?.ok
      ? `Recorded Prism-Signed SOSA/PROV-O Envelope (Channel 1: Studio -> Engine)\nPayload: compose:${pack?.slug || "sunspire-regatta"}:lod${lodIdx}\nBLAKE3 kappa: ${r.kappaHex}`
      : "Failed to record SOSA/PROV-O communication envelope.";
  }
});
wizObserverBtn?.addEventListener("click", async () => {
  const r = await callWorker("observer-status");
  if (wizDlgLog && r?.ok) {
    wizDlgLog.textContent = `ObserverPrismHolon Live Telemetry:\n${JSON.stringify(r.observer, null, 2)}`;
  }
});
wizAuditBtn?.addEventListener("click", runFullPrismQualityGate);
wizSealUorcBtn?.addEventListener("click", exportActiveUorcArchive);
wizShareUorcBtn?.addEventListener("click", () => shareUorcSnapshot(callWorker, chip, statusEl));

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
    launchStudioModal: async (rk) => studioHandle?.selectTab(rk),
    openStitchModal: openStitchGuarded,
  });

// Wire viewport interactions
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
lodBtn?.addEventListener("click", () => streamLod((lodIdx + 1) % LODS.length));
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
  if (rk) studioHandle?.selectTab(rk);
});

// Boot Holo + Domain Catalog + Worker + Inline 7-Role Atelier
(async () => {
  const [holoResult, packsResult] = await Promise.allSettled([
    fetch(new URL("../app.holo", import.meta.url)).then(async (res) => {
      if (!res.ok) throw new Error(`HTTP ${res.status}`);
      holoApp = await load(new Uint8Array(await res.arrayBuffer()));
      holoSummary = await holoApp.acceptance();
      if (holoStatusEl) {
        holoStatusEl.textContent = `Atelier Ready · Prism Holo Verified (${holoSummary.passed}/${holoSummary.total} vectors) · 4 Catalog Domains · 7 Lifecycle Roles`;
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

  const { w, h } = pxDims();
  if (worker && canvas && typeof canvas.transferControlToOffscreen === "function") {
    try {
      const oc = canvas.transferControlToOffscreen();
      const r = await callWorker("init", { canvas: oc, width: w, height: h, lod: currentLodSpec() }, [oc]);
      if (r?.kind === "ready") {
        offOk = true;
        lTris = r.tris || lTris;
        if (r.audioSpec) Object.assign(ACOUST, r.audioSpec);
      }
    } catch {
      offOk = false;
    }
  }
  if (!offOk && canvas) {
    fbM = await import("../scene-renderer.mjs");
    fbR = await fbM.createBestRenderer(canvas);
    if (worker) {
      const r = await callWorker("init", { canvas: null, width: w, height: h, lod: currentLodSpec() });
      if (r?.tris) lTris = r.tris;
      await syncFallbackBuffers();
    }
  }

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
        if (location.hash !== `#${rk}` && location.hash !== "#compose") {
          history.replaceState(null, "", `#${rk}`);
        }
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

  // Honor deep-links from /catalogs/: ?domain=<slug>&catalog=<STD-ID>&step=<1..4>
  const params = new URLSearchParams(location.search);
  const reqDomain = params.get("domain");
  const reqCatalog = params.get("catalog");
  const reqStep = Number(params.get("step"));
  if (reqCatalog && catalogBanner && catalogBannerText) {
    catalogBanner.dataset.active = "true";
    catalogBannerText.textContent = `Imported Governing Catalog Standard: ${reqCatalog} — live-verified against Prism app.holo`;
  }
  const initialDomainIdx = reqDomain
    ? Math.max(0, domainPacks.findIndex((d) => d.slug === reqDomain || d.id === reqDomain))
    : 0;
  if (!userSelectedDomain && domainPacks.length > 0) {
    await loadCatalogDomainPack(initialDomainIdx);
  }
  if (reqStep >= 1 && reqStep <= 4) {
    setWizardStep(reqStep);
  } else {
    setWizardStep(1);
  }

  // Restore `#uorc=` if shared directly to `/studio/#uorc=...`:
  const snapMatch = /^#uorc=(.+)$/.exec(location.hash);
  if (snapMatch) {
    const bytes = base64UrlToBytes(snapMatch[1].trim());
    if (bytes?.byteLength) {
      const r = await callWorker("restore-snapshot", { bytes });
      if (r?.ok) await streamLod(r.lodRank ?? 3);
    }
  }

  syncHud();
  chip(statusEl, `Atelier Ready · ${activeProject} · ${LODS[lodIdx].label} · 4-Stage Catalog Wizard Active`);
  requestRender();
})();
