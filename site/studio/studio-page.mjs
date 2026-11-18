// Standalone Verdant Studio Atelier Controller (`site/studio/studio-page.mjs`).
// Connects a dedicated live 3D authoring viewport (`/engine-worker.mjs`) and `/app.holo`
// directly to `mountInlineStudio` (`site/studio.mjs`) across all 7 functional roles.

import { load } from "/worker-runtime/index.mjs";
import {
  STARTER_ARCHETYPES,
  STUDIO_ROLE_KEYS,
  base64UrlToBytes,
  buildPosixTarArchive,
  cacheBytesInOpfs,
  exportGlbDownload,
  generateStarterProjectFiles,
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
const exportTarBtn = $("studio-export-tar-btn");
const glbFileIn = $("studio-glb-file");
const promptForm = $("studio-prompt-form");
const promptInput = $("studio-prompt-input");

const chip = (el, text) => {
  if (el) el.textContent = text;
};

let holoApp = null;
let lodIdx = 3;
let simLive = false;
let camPreset = "landmark";
let renderMode = "webgl2";
let followMover = false;
let activeProject = STARTER_ARCHETYPES[0].title;
let activeRoleKey = "dashboard";
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

function syncHud() {
  const roleLabel = activeRoleKey.charAt(0).toUpperCase() + activeRoleKey.slice(1);
  if (hudProjectEl) hudProjectEl.textContent = `${activeProject} · ${roleLabel}`;
  if (hudMetricsEl) {
    const triStr = lTris > 0 ? `${lTris.toLocaleString()} tris · ` : "";
    hudMetricsEl.textContent = `${LODS[lodIdx].label} · ${triStr}Clear ${lClr}mm`;
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
    worker = new Worker("/engine-worker.mjs", { type: "module" });
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
  const lod = LODS[lodIdx];
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
  const lod = LODS[lodIdx];
  if (lodBtn) lodBtn.textContent = `Fidelity: ${lod.short}`;
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
    lod: LODS[lodIdx],
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

exportTarBtn?.addEventListener("click", () => {
  const arch = STARTER_ARCHETYPES.find((a) => a.title === activeProject) || STARTER_ARCHETYPES[0];
  const files = generateStarterProjectFiles({
    slug: arch.slug,
    archetypeIdx: arch.idx,
    seed: arch.seed,
    lodRank: lodIdx,
    cameraMode: camPreset,
  });
  const tar = buildPosixTarArchive(arch.slug, files);
  const url = URL.createObjectURL(new Blob([tar], { type: "application/x-tar" }));
  const a = document.createElement("a");
  a.href = url;
  a.download = `${arch.slug}.tar`;
  a.click();
  setTimeout(() => URL.revokeObjectURL(url), 1500);
  chip(statusEl, `Exported ${arch.slug}.tar (${tar.byteLength}B POSIX ustar archive)`);
});

addEventListener("hashchange", () => {
  const rk = resolveInitialRole();
  if (rk) studioHandle?.selectTab(rk);
});

// Boot Holo + Worker + Inline 7-Role Atelier
(async () => {
  try {
    const holoRes = await fetch("/app.holo");
    if (holoRes.ok) {
      holoApp = await load(new Uint8Array(await holoRes.arrayBuffer()));
      const acc = await holoApp.acceptance();
      if (holoStatusEl) {
        holoStatusEl.textContent = `Atelier Ready · Prism Holo Verified (${acc.passed}/${acc.total} vectors) · 7 Lifecycle Roles`;
      }
    }
  } catch {
    if (holoStatusEl) holoStatusEl.textContent = "Atelier Ready · Local Authoring Mode";
  }

  const { w, h } = pxDims();
  if (worker && canvas && typeof canvas.transferControlToOffscreen === "function") {
    try {
      const oc = canvas.transferControlToOffscreen();
      const r = await callWorker("init", { canvas: oc, width: w, height: h, lod: LODS[lodIdx] }, [oc]);
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
      const r = await callWorker("init", { canvas: null, width: w, height: h, lod: LODS[lodIdx] });
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
      onOpenStitchMesh: openStitchGuarded,
      onRoutePrompt: executePrompt,
      onInteractStory: async () => callWorker("story-interact"),
      onMountOpfsUorc: async (name, bytes) => {
        if (!isOrganAllowed(4, true)) {
          refuseOrgan(4);
          return false;
        }
        const r = await callWorker("stream-uorc", { targetRank: 3, customStream: bytes, lod: LODS[3], simClockSec });
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
        if (location.hash !== `#${rk}`) history.replaceState(null, "", `#${rk}`);
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
  chip(statusEl, `Atelier Ready · ${activeProject} · ${LODS[lodIdx].label} · 7 Role Contracts Active`);
  requestRender();
})();
