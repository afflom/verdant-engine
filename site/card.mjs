const STREAMING_LODS = [
  { rank: 0, key: "lod:0", short: "LOD0", label: "LOD0 · Coarse Proxy (0.125)", res: 0.125, density: 0.35, lossless: false, sky: [0.09, 0.08, 0.12], fog: [0.14, 0.11, 0.16] },
  { rank: 1, key: "lod:1", short: "LOD1", label: "LOD1 · Medium Detail (0.25)", res: 0.25, density: 0.72, lossless: false, sky: [0.12, 0.17, 0.24], fog: [0.18, 0.24, 0.31] },
  { rank: 2, key: "lod:2", short: "LOD2", label: "LOD2 · High Detail (0.625)", res: 0.625, density: 0.92, lossless: false, sky: [0.16, 0.27, 0.34], fog: [0.24, 0.35, 0.39] },
  { rank: 3, key: "lod:3", short: "LOD3", label: "LOD3 · Lossless UORC (1.00)", res: 1.0, density: 1.0, lossless: true, sky: [0.18, 0.30, 0.38], fog: [0.30, 0.40, 0.44] },
  { rank: 4, key: "scene:lod:4", short: "LOD4", label: "LOD4 · Ultra HD Citadel (2.00)", res: 2.0, density: 2.0, lossless: true, sky: [0.19, 0.33, 0.43], fog: [0.32, 0.43, 0.48] },
];

const NODE_KERNELS = [
  { id: 0, query: "kernel:0", name: "Vaulted Citadel & Observer", semiring: "LinearExact", target: [2.2, 3.8, 9.2], dist: 18.5, yawMilli: 460, pitchMilli: 260 },
  { id: 1, query: "kernel:12", name: "Parabolic Stone Arch Bridge", semiring: "TropicalMinPlus", target: [7.5, 1.8, -4.0], dist: 14, yawMilli: 560, pitchMilli: 340 },
  { id: 2, query: "kernel:13", name: "22-Joint Explorer & Foot IK", semiring: "LinearExact", target: [0, 1.6, 11.5], dist: 6.2, possessExplorer: true },
  { id: 3, query: "kernel:6", name: "XPBD Articulated Rig Bay", semiring: "LinearExact", target: [-6.5, 1.8, 9.0], dist: 10, yawMilli: -540, pitchMilli: 320 },
  { id: 4, query: "kernel:9", name: "Diegetic UI & South Terrace", semiring: "TropicalMinPlus", target: [6.0, 2.0, 8.0], dist: 11, yawMilli: 520, pitchMilli: 300 },
  { id: 5, query: "kernel:14", name: "Citadel Chronicle", semiring: "TropicalMinPlus", target: [1.5, 2.2, -1.0], dist: 8.5, yawMilli: 350, pitchMilli: 260 },
  { id: 6, query: "kernel:3", name: "4-Wheel Suspension Vehicle", semiring: "LinearExact", target: [16.5, 1.2, 0], dist: 11, followMover: true },
];

const ACOUSTIC_SPEC = { quantumFrames: 128, stereoChannels: 2, fundamentalHz: 110.0, modeNumers: [9, 25, 49, 81, 121], modeDenom: 9, rt60Ms: 3019 };

function parseConformantFields(line) {
  if (!line?.startsWith("conformant:")) return null;
  const fields = {};
  for (const part of line.slice(11).split(",")) {
    const eq = part.indexOf("=");
    if (eq > 0) fields[part.slice(0, eq)] = part.slice(eq + 1);
  }
  return fields;
}

function parseRatioPair(raw, fb) {
  if (!raw) return fb;
  const [n, d] = raw.split("/").map(Number);
  return Number.isFinite(n) && Number.isFinite(d) && d !== 0 ? n / d : fb;
}

function parseTriplet(raw, scale, fb) {
  if (!raw) return fb;
  const p = raw.split("/").map(Number);
  return p.length === 3 && p.every(Number.isFinite) ? p.map((v) => v / scale) : fb;
}

const yieldTask = () => new Promise((r) => setTimeout(r, 0));
let prewarmedWorker = null;
try {
  if (typeof Worker === "function") prewarmedWorker = new Worker("./engine-worker.mjs", { type: "module" });
} catch {
  prewarmedWorker = null;
}

export default async function start({ app, reason, $, chip, show, parse }) {
  const canvas = $("ve-canvas"), viewportEl = canvas?.parentElement, statusEl = $("ve-status");
  const lodBtn = $("ve-lod-btn"), uorcBtn = $("ve-uorc-btn"), renderBtn = $("ve-render-btn");
  const importBtn = $("ve-import-btn"), glbFileInput = $("ve-glb-file"), studioBtn = $("ve-studio-btn");
  const meshBtn = $("ve-mesh-btn"), simBtn = $("ve-sim-btn"), camBtn = $("ve-cam-btn");
  const audioBtn = $("ve-audio-btn"), glbBtn = $("ve-glb-btn"), snapBtn = $("ve-snap-btn");
  const promptForm = $("ve-prompt-form"), promptInput = $("ve-prompt-input");
  const hudLod = $("ve-hud-lod"), hudStats = $("ve-hud-stats"), telemetryList = $("ve-telemetry");

  let lodIdx = 2, simLive = false, camPreset = "landmark", renderMode = "webgl2", followMover = false;
  let activeNodeSlot = 0, gazeDwellMs = 300;
  let pendingYaw = 0, pendingPitch = 0, pendingDist = 0, jumpQueued = false;
  let solvedEye = [10.2, 8.5, 25.3], solvedTarget = [2.2, 3.8, 9.2], universeCell = [0, 0, 0];
  let dragging = false, rafId = 0;
  let lastTris = 0, lastBvhVis = 24, lastKernelCount = 15;
  let workerBackend = "webgl2", isSoftwareGpu = false, offscreenActive = false, nextReqId = 1, workerFramesInFlight = 0, lastFrameSentAt = 0;
  let fallbackRenderer = null, fallbackMod = null, studioMod = null;
  const keys = new Set(), pendingCalls = new Map();
  const getStudio = async () => (studioMod ??= await import("./studio.mjs"));

  function measureCanvasPixels() {
    if (!canvas) return { w: 1, h: 1 };
    const dpr = Math.min(window.devicePixelRatio || 1, isSoftwareGpu ? 1 : 2);
    return { w: Math.max(1, Math.floor((canvas.clientWidth || 320) * dpr)), h: Math.max(1, Math.floor((canvas.clientHeight || 240) * dpr)) };
  }

  function readMoveIntent(dt) {
    const pad = navigator.getGamepads?.()?.[0];
    const dz = (v) => (Math.abs(v) > 0.16 ? v : 0);
    const gx = pad ? dz(pad.axes[0] || 0) : 0, gy = pad ? -dz(pad.axes[1] || 0) : 0;
    const rx = pad ? dz(pad.axes[2] || 0) : 0, ry = pad ? dz(pad.axes[3] || 0) : 0;
    const kf = (keys.has("KeyW") || keys.has("ArrowUp") ? 1 : 0) - (keys.has("KeyS") || keys.has("ArrowDown") ? 1 : 0);
    const ks = (keys.has("KeyD") || keys.has("ArrowRight") ? 1 : 0) - (keys.has("KeyA") || keys.has("ArrowLeft") ? 1 : 0);
    const fwd = Math.max(-1, Math.min(1, kf + gy));
    const strafe = Math.max(-1, Math.min(1, ks + gx));
    const sprint = keys.has("ShiftLeft") || keys.has("ShiftRight") ? 1.65 : 1.0;
    const jump = jumpQueued || keys.has("Space") || Boolean(pad?.buttons?.[0]?.pressed);
    jumpQueued = false;
    let dYaw = pendingYaw - rx * dt * 1.6;
    let dPitch = pendingPitch + ry * dt * 1.1;
    if (simLive && !dragging && !followMover && camPreset === "landmark" && fwd === 0 && strafe === 0) {
      dYaw += dt * 0.14;
    }
    const dDist = pendingDist;
    pendingYaw = 0; pendingPitch = 0; pendingDist = 0;
    return { fwd, strafe, speedScale: sprint, jump, dYaw, dPitch, dDist, moving: fwd !== 0 || strafe !== 0 || jump };
  }

  const formatHudStats = (fps) => {
    const b = renderMode === "tile" ? "TILE 16x16" : workerBackend.toUpperCase();
    const sec = `S(${universeCell[0]},${universeCell[2]})`;
    return `${fps ? `${fps} · ${b}` : b} · ${lastTris} tris · ${lastBvhVis} vis · ${sec}`;
  };

  const hasMoveKeys = () =>
    keys.has("KeyW") || keys.has("KeyA") || keys.has("KeyS") || keys.has("KeyD") ||
    keys.has("ArrowUp") || keys.has("ArrowDown") || keys.has("ArrowLeft") || keys.has("ArrowRight") || keys.has("Space");

  const isAnimating = () => simLive || dragging || followMover || hasMoveKeys();

  let engineWorker = null;
  if (typeof Worker === "function" && canvas) {
    try {
      engineWorker = prewarmedWorker || new Worker("./engine-worker.mjs", { type: "module" });
      prewarmedWorker = null;
      engineWorker.onmessage = ({ data: msg }) => {
        if (!msg) return;
        if (msg.id && pendingCalls.has(msg.id)) {
          const r = pendingCalls.get(msg.id);
          pendingCalls.delete(msg.id);
          r(msg);
          return;
        }
        if (msg.kind === "frame-dropped") {
          workerFramesInFlight = Math.max(0, workerFramesInFlight - 1);
          return;
        }
        if (msg.kind === "telemetry") {
          workerFramesInFlight = Math.max(0, workerFramesInFlight - 1);
          lastTris = msg.tris || lastTris;
          lastBvhVis = msg.bvhVisible ?? lastBvhVis;
          if (Array.isArray(msg.eye)) solvedEye = msg.eye;
          if (Array.isArray(msg.lookTarget)) solvedTarget = msg.lookTarget;
          if (Array.isArray(msg.universeCell)) universeCell = msg.universeCell;
          studioMod?.pushStudioTelemetry(msg);
          studioMod?.syncModalAudioSpatial(msg.binauralPan, msg.binauralGain);
          if (!isAnimating() && hudStats) hudStats.textContent = formatHudStats("");
          if ((pendingYaw !== 0 || pendingPitch !== 0 || pendingDist !== 0 || jumpQueued) && rafId === 0) requestRender();
        }
      };
      engineWorker.onerror = () => {
        workerFramesInFlight = 0;
        for (const r of pendingCalls.values()) r(null);
        pendingCalls.clear();
      };
    } catch {
      engineWorker = null;
    }
  }

  const callWorker = (kind, payload = {}, transfer = []) =>
    !engineWorker ? Promise.resolve(null) : new Promise((res) => {
      const id = nextReqId++;
      const wd = setTimeout(() => { if (pendingCalls.delete(id)) res(null); }, 15000);
      pendingCalls.set(id, (v) => { clearTimeout(wd); res(v); });
      engineWorker.postMessage({ kind, id, ...payload }, transfer);
    });

  let prevNow = performance.now(), simClockSec = 0, frames = 0, lastFpsStamp = prevNow;
  const requestRender = () => {
    if (rafId === 0) { prevNow = performance.now(); rafId = requestAnimationFrame(frame); }
  };

  function setSimLive(next) {
    simLive = Boolean(next);
    if (simBtn) { simBtn.setAttribute("aria-pressed", String(simLive)); simBtn.textContent = simLive ? "Sim: Live" : "Sim: Step"; }
    requestRender();
  }

  const applyCamReply = (r) => {
    if (r?.camera) { solvedEye = r.camera.eye; solvedTarget = r.camera.lookTarget; }
    if (r?.tris) lastTris = r.tris;
  };

  async function cycleCameraMode(forcePreset = null) {
    const order = ["landmark", "third", "first"];
    camPreset = forcePreset || order[(order.indexOf(camPreset) + 1) % order.length];
    followMover = false;
    if (camPreset === "landmark") {
      if (camBtn) camBtn.textContent = "Cam: Orbit";
      applyCamReply(await callWorker("camera-focus-landmark", { target: [2.2, 3.8, 9.2], dist: 18.5, yawMilli: 460, pitchMilli: 260 }));
      chip(statusEl, "Lawful Orbit · Near-Sphere Guarded", "good");
    } else if (camPreset === "third") {
      if (camBtn) camBtn.textContent = "Cam: 3rd IK";
      applyCamReply(await callWorker("camera-set-mode", { mode: 0, simClockSec }));
      chip(statusEl, "3rd-Person Explorer · WASD + Foot IK", "good");
    } else {
      if (camBtn) camBtn.textContent = "Cam: 1st Eye";
      applyCamReply(await callWorker("camera-set-mode", { mode: 1, simClockSec }));
      chip(statusEl, "1st-Person Possessed · Head Masked", "good");
    }
    await syncFallbackBuffers();
    requestRender();
  }

  async function interactChronicle() {
    const r = await callWorker("story-interact");
    if (!r?.ok) return null;
    chip(
      statusEl,
      r.advanced
        ? `Chronicle -> ${r.title} · Supply ${r.supply}`
        : `${r.title} · ${r.objective}`,
      r.advanced ? "good" : "wait",
    );
    return r;
  }

  const queryVerifier = async (q) => {
    if (!app) return null;
    try { return show(await app.dispatch(parse(q))); } catch { return null; }
  };

  async function syncVerifierBanner(query = STREAMING_LODS[lodIdx].key) {
    if (!app) { chip(statusEl, reason ? "Verifier unavailable" : "Engine offline", "bad"); return; }
    const res = await queryVerifier(query);
    if (!res) return;
    if (res.startsWith("conformant:")) {
      const lod = STREAMING_LODS[lodIdx];
      chip(statusEl, `${lod.short} · ${lod.lossless ? `${lastKernelCount}/15 Kernels` : "UORC Stream OK"}`, "good");
    } else if (res.startsWith("refused:")) chip(statusEl, res.replace("refused:", "Refused: "), "warn");
    else chip(statusEl, res, "bad");
  }

  async function syncFallbackBuffers() {
    if (offscreenActive || !fallbackRenderer || !engineWorker) return;
    const r = await callWorker("get-packed-buffers", { simClockSec });
    if (r?.ok) {
      fallbackRenderer.uploadPackedBuffers(r.pos, r.norm, r.col, r.idx);
      lastTris = r.tris || 0;
      if (r.camera) { solvedEye = r.camera.eye; solvedTarget = r.camera.lookTarget; }
    }
  }

  async function toggleRenderBackend(force = null) {
    renderMode = force || (renderMode === "webgl2" ? "tile" : "webgl2");
    if (renderBtn) renderBtn.textContent = renderMode === "tile" ? "Tile 16x16" : "WebGL2";
    const r = await callWorker("set-render-mode", { mode: renderMode, simClockSec });
    if (hudStats) hudStats.textContent = formatHudStats("");
    chip(statusEl, renderMode === "tile" ? `Wasm 16x16 Tile PBR (${r?.tris ?? lastTris} tris)` : `3-Pass WebGL2 HD (${r?.tris ?? lastTris} tris)`, "good");
    requestRender();
  }

  async function importGlbBytes(fileName, rawBytes) {
    chip(statusEl, `Importing ${fileName}…`, "wait");
    const copy = new Uint8Array(rawBytes);
    const r = await callWorker("import-glb", { bytes: copy }, [copy.buffer]);
    if (!r?.ok) { chip(statusEl, `Refused invalid .glb (${fileName})`, "warn"); return; }
    lastTris = r.tris || lastTris;
    const mod = await getStudio();
    if (r.vdntBytes?.byteLength) await mod.cacheBytesInOpfs(`${fileName}.vdnt`, r.vdntBytes);
    await syncFallbackBuffers();
    if (hudLod) hudLod.textContent = `GLB · ${fileName} (${r.tris} tris)`;
    chip(statusEl, `Imported ${fileName} · ${r.tris} tris · OPFS cached`, "good");
    requestRender();
  }

  async function resetImportedGlb() {
    const r = await callWorker("reset-glb");
    if (r?.tris) lastTris = r.tris;
    if (hudLod) hudLod.textContent = STREAMING_LODS[lodIdx].label;
    await syncFallbackBuffers();
    await syncVerifierBanner("summary");
    requestRender();
  }

  const executePromptIntent = async (text) =>
    (await getStudio()).routeStudioPrompt(text, {
      callWorker, chip, statusEl, streamUorcToRank, cycleCameraMode,
      interactChronicle, focusNodePreset, setSimLive, toggleRenderBackend,
      restoreRawSnapshot, launchStudioModal,
    });

  async function focusWorldPoint(target, dist = 12, label = "Entity") {
    camPreset = "landmark"; followMover = false;
    if (camBtn) camBtn.textContent = "Cam: Orbit";
    applyCamReply(await callWorker("camera-focus-landmark", { target, dist, yawMilli: 520, pitchMilli: 300, simClockSec }));
    await syncFallbackBuffers(); requestRender();
    chip(statusEl, `Framed ${label} · Guarded Orbit`, "good");
  }

  async function setWorldSeed(seed) {
    const r = await callWorker("set-seed", { seed });
    if (r?.ok) {
      applyCamReply(r); await syncFallbackBuffers(); requestRender();
      chip(statusEl, `Seed ${r.seed} · ${STREAMING_LODS[lodIdx].short} (${lastTris} tris)`, "good");
    }
    return r;
  }

  async function launchStudioModal(initialTab = null) {
    const mod = await getStudio();
    mod.openStudioModal({
      initialTab: typeof initialTab === "string" ? initialTab : null, callWorker, queryVerifier,
      onPickGlb: () => glbFileInput?.click(), onResetGlb: resetImportedGlb,
      onExportGlb: () => mod.exportGlbDownload(callWorker, STREAMING_LODS[lodIdx].short),
      onShareSnapshot: () => mod.shareVdntSnapshot(callWorker, chip, statusEl),
      onToggleRenderMode: toggleRenderBackend, onSetLod: streamUorcToRank,
      onSetCameraMode: cycleCameraMode, onSetSimLive: setSimLive,
      onToggleAudio: () => mod.toggleModalAudio(audioBtn, ACOUSTIC_SPEC, () => solvedEye),
      onFocusPoint: focusWorldPoint, onFocusPreset: focusNodePreset, onSetSeed: setWorldSeed,
      onOpenStitchMesh: async () => (await import("./stitch-mesh.mjs")).openStitchMeshModal({ callWorker, onRestoreSnapshot: restoreRawSnapshot }),
      onRoutePrompt: executePromptIntent, onInteractStory: interactChronicle,
      getRenderMode: () => renderMode, getLodRank: () => lodIdx,
    });
  }

  async function setLod(nextIdx, updateBanner = true) {
    lodIdx = ((nextIdx % STREAMING_LODS.length) + STREAMING_LODS.length) % STREAMING_LODS.length;
    const lod = STREAMING_LODS[lodIdx];
    if (lodBtn) lodBtn.textContent = lod.short;
    if (hudLod) hudLod.textContent = lod.label;
    if (lod.rank < 2) followMover = false;
    if (engineWorker) { engineWorker.postMessage({ kind: "set-lod", lod, simClockSec }); await syncFallbackBuffers(); }
    requestRender(); if (updateBanner) syncVerifierBanner(lod.key);
  }

  async function focusNodePreset(idx) {
    const node = NODE_KERNELS[idx];
    if (!node) return;
    activeNodeSlot = node.id; gazeDwellMs = 1400;
    if (lodIdx < 3) await setLod(3, false);
    followMover = Boolean(node.followMover);
    if (followMover || node.id === 3) setSimLive(true);
    camPreset = node.possessExplorer ? "third" : "landmark";
    if (camBtn) camBtn.textContent = node.possessExplorer ? "Cam: 3rd IK" : "Cam: Orbit";
    applyCamReply(await callWorker("camera-focus-landmark", {
      possessExplorer: Boolean(node.possessExplorer), target: node.target, dist: node.dist,
      yawMilli: node.yawMilli ?? 620, pitchMilli: node.pitchMilli ?? 350, simClockSec,
    }));
    await syncFallbackBuffers(); requestRender(); await syncVerifierBanner(node.query);
  }

  async function streamUorcToRank(targetRank) {
    const clamped = Math.max(0, Math.min(STREAMING_LODS.length - 1, targetRank | 0));
    lodIdx = clamped;
    const lod = STREAMING_LODS[lodIdx];
    if (lodBtn) lodBtn.textContent = lod.short;
    if (hudLod) hudLod.textContent = lod.label;
    if (lod.rank < 2) followMover = false;
    chip(statusEl, `Streaming .uorc -> ${lod.short}…`, "wait");
    if (engineWorker) {
      const r = await callWorker("stream-uorc", { targetRank: clamped, lod, simClockSec });
      if (r?.tris) lastTris = r.tris;
      await syncFallbackBuffers();
      requestRender();
      if (r?.ok) { chip(statusEl, `UORC Wasm · ${lod.short} (${lastTris} tris · Σ=${r.codedChecksum})`, "good"); return r; }
    }
    await setLod(clamped, true);
    return null;
  }

  lodBtn?.addEventListener("click", () => setLod(lodIdx + 1));
  uorcBtn?.addEventListener("click", () => streamUorcToRank(lodIdx < 4 ? 4 : 0));
  renderBtn?.addEventListener("click", () => toggleRenderBackend());
  importBtn?.addEventListener("click", () => glbFileInput?.click());
  glbFileInput?.addEventListener("change", async () => {
    const f = glbFileInput.files?.[0];
    if (f) { await importGlbBytes(f.name, await f.arrayBuffer()); glbFileInput.value = ""; }
  });
  studioBtn?.addEventListener("click", () => launchStudioModal());
  meshBtn?.addEventListener("click", async () => (await import("./stitch-mesh.mjs")).openStitchMeshModal({ callWorker, onRestoreSnapshot: restoreRawSnapshot }));
  simBtn?.addEventListener("click", () => setSimLive(!simLive));
  camBtn?.addEventListener("click", () => cycleCameraMode());
  promptForm?.addEventListener("submit", async (e) => { e.preventDefault(); await executePromptIntent(promptInput?.value); if (promptInput) promptInput.value = ""; });
  viewportEl?.addEventListener("dragover", (e) => { e.preventDefault(); viewportEl.classList.add("drag-over"); });
  viewportEl?.addEventListener("dragleave", () => viewportEl.classList.remove("drag-over"));
  viewportEl?.addEventListener("drop", async (e) => { e.preventDefault(); viewportEl.classList.remove("drag-over"); const f = e.dataTransfer?.files?.[0]; if (f) await importGlbBytes(f.name, await f.arrayBuffer()); });

  audioBtn?.addEventListener("click", async () => (await getStudio()).toggleModalAudio(audioBtn, ACOUSTIC_SPEC, () => solvedEye));
  glbBtn?.addEventListener("click", async () => (await getStudio()).exportGlbDownload(callWorker, STREAMING_LODS[lodIdx].short));

  async function restoreRawSnapshot(bytes, label = "VDNT") {
    const r = await callWorker("restore-snapshot", { bytes });
    if (r?.ok) { await setLod(r.lodRank ?? 3, false); chip(statusEl, `Restored ${label} · ${STREAMING_LODS[lodIdx].short} (${r.byteLen}B)`, "good"); return true; }
    chip(statusEl, `Refused tampered ${label} snapshot`, "warn");
    return false;
  }

  async function restoreHashSnapshotIfPresent() {
    if (!location.hash.startsWith("#vdnt=")) return false;
    const mod = await getStudio();
    const bytes = mod.base64UrlToBytes(location.hash.slice(6).trim());
    if (!bytes?.byteLength) { chip(statusEl, "Refused malformed VDNT snapshot", "warn"); return true; }
    await restoreRawSnapshot(bytes, "VDNT");
    return true;
  }

  snapBtn?.addEventListener("click", async () => (await getStudio()).shareVdntSnapshot(callWorker, chip, statusEl));

  addEventListener("hashchange", () => {
    if (location.hash.startsWith("#vdnt=")) restoreHashSnapshotIfPresent();
    else if (location.hash.startsWith("#studio=")) launchStudioModal(location.hash.slice(8).trim());
  });

  let lastX = 0, lastY = 0;
  canvas?.addEventListener("pointerdown", (e) => { dragging = true; lastX = e.clientX; lastY = e.clientY; canvas.setPointerCapture?.(e.pointerId); requestRender(); });
  canvas?.addEventListener("pointermove", (e) => {
    if (!dragging) return;
    const dx = e.clientX - lastX, dy = e.clientY - lastY;
    lastX = e.clientX; lastY = e.clientY;
    gazeDwellMs = Math.max(80, gazeDwellMs - 40);
    pendingYaw -= dx * 0.007;
    pendingPitch += camPreset === "first" ? -dy * 0.0055 : dy * 0.006;
    requestRender();
  });
  canvas?.addEventListener("pointerup", () => { dragging = false; });
  canvas?.addEventListener("pointercancel", () => { dragging = false; });
  canvas?.addEventListener("wheel", (e) => { e.preventDefault(); pendingDist += e.deltaY * 0.018; requestRender(); }, { passive: false });
  addEventListener("keydown", (e) => {
    if (/^(INPUT|TEXTAREA|SELECT)$/.test(document.activeElement?.tagName ?? "")) return;
    if (e.code === "KeyE" && !e.repeat) { interactChronicle(); return; }
    if (e.code === "KeyG" && !e.repeat) { launchStudioModal("create"); return; }
    if (e.code === "Space") jumpQueued = true;
    keys.add(e.code);
    if (hasMoveKeys() && camPreset === "landmark") {
      camPreset = "third";
      followMover = false;
      if (camBtn) camBtn.textContent = "Cam: 3rd IK";
    }
    requestRender();
  });
  addEventListener("keyup", (e) => keys.delete(e.code));
  addEventListener("resize", requestRender);
  document.addEventListener("visibilitychange", () => { if (!document.hidden) { prevNow = performance.now(); requestRender(); } });
  if (typeof ResizeObserver !== "undefined" && viewportEl) new ResizeObserver(requestRender).observe(viewportEl);

  function frame(now) {
    rafId = 0;
    if (document.hidden) return;
    if (offscreenActive && engineWorker && workerFramesInFlight >= 2) {
      if (now - lastFrameSentAt > 1500) workerFramesInFlight = 0;
      else { if (isAnimating()) rafId = requestAnimationFrame(frame); return; }
    }
    const dt = Math.min(0.05, Math.max(0.001, (now - prevNow) * 0.001));
    prevNow = now;
    const intent = readMoveIntent(dt);
    const stepping = simLive || followMover || intent.moving;
    if (stepping) simClockSec += dt;
    if (!dragging) gazeDwellMs = Math.min(3000, gazeDwellMs + Math.round(dt * 1000));
    const lod = STREAMING_LODS[lodIdx], { w, h } = measureCanvasPixels();

    if (offscreenActive && engineWorker) {
      workerFramesInFlight++; lastFrameSentAt = now;
      engineWorker.postMessage({
        kind: "frame", width: w, height: h, dt, stepPhysics: stepping, followMover,
        fwd: intent.fwd, strafe: intent.strafe, speedScale: intent.speedScale, jump: intent.jump,
        deltaYaw: intent.dYaw, deltaPitch: intent.dPitch, deltaDist: intent.dDist,
        gazedSlot: activeNodeSlot, dwellMs: gazeDwellMs, angSpeedMilli: dragging ? 600 : 60, lod, simClockSec,
      });
    } else if (fallbackRenderer && fallbackMod && canvas) {
      if (canvas.width !== w || canvas.height !== h) { canvas.width = w; canvas.height = h; }
      const proj = fallbackMod.mat4Perspective(Math.PI / 3.4, Math.max(0.2, w / h), 0.25, 160.0);
      fallbackRenderer.render(fallbackMod.mat4Mul(proj, fallbackMod.mat4LookAt(solvedEye, solvedTarget, [0, 1, 0])), solvedEye, lod, simClockSec, solvedTarget, Math.PI / 3.4);
    }

    frames++;
    if (!isAnimating()) { if (hudStats && lastTris > 0) hudStats.textContent = formatHudStats(""); }
    else if (now - lastFpsStamp >= 600) {
      const fps = Math.max(1, Math.round((frames * 1000) / (now - lastFpsStamp)));
      frames = 0; lastFpsStamp = now;
      if (hudStats) hudStats.textContent = formatHudStats(`${fps} fps`);
    }
    if (isAnimating()) rafId = requestAnimationFrame(frame);
  }

  const fmtSemiring = (s) => s.replace(/([a-z])([A-Z])/g, "$1 $2");
  const badges = [];
  const makeRailRow = (label, tagText, onClick, id = "") => {
    const li = document.createElement("li"), btn = document.createElement("button");
    btn.type = "button"; btn.className = "again"; if (id) btn.id = id;
    const lbl = document.createElement("span"); lbl.textContent = label;
    const badge = document.createElement("span"); badge.className = "tag good"; badge.textContent = tagText;
    btn.append(lbl, badge); btn.addEventListener("click", onClick); li.append(btn); telemetryList.append(li);
    return badge;
  };
  if (telemetryList) {
    telemetryList.replaceChildren();
    makeRailRow("Guided Journeys & Create Game", "7 Personas", () => launchStudioModal("journeys"), "ve-journeys-rail-btn");
    for (const node of NODE_KERNELS) {
      badges.push({ node, badge: makeRailRow(node.name, fmtSemiring(node.semiring), () => focusNodePreset(node.id)) });
    }
  }

  const applyInitReply = (r) => {
    if (!r) return;
    applyCamReply(r);
    if (r.audioSpec) Object.assign(ACOUSTIC_SPEC, r.audioSpec);
    if (r.tensorKernels) lastKernelCount = r.tensorKernels;
    if (Array.isArray(r.universeCell)) universeCell = r.universeCell;
  };
  const initialDims = measureCanvasPixels();
  if (engineWorker && canvas && typeof canvas.transferControlToOffscreen === "function") {
    try {
      const offscreen = canvas.transferControlToOffscreen();
      const r = await callWorker("init", { canvas: offscreen, width: initialDims.w, height: initialDims.h, lod: STREAMING_LODS[lodIdx] }, [offscreen]);
      if (r?.kind === "ready") {
        offscreenActive = true; workerBackend = r.backend || "webgl2"; isSoftwareGpu = Boolean(r.isSoftware);
        applyInitReply(r); if (hudStats) hudStats.textContent = formatHudStats("");
      }
    } catch { offscreenActive = false; }
  }

  if (!offscreenActive && canvas) {
    fallbackMod = await import("./scene-renderer.mjs");
    fallbackRenderer = await fallbackMod.createBestRenderer(canvas);
    workerBackend = fallbackRenderer?.backend || "webgl2";
    isSoftwareGpu = Boolean(fallbackRenderer?.isSoftware);
    if (engineWorker) {
      applyInitReply(await callWorker("init", { canvas: null, width: initialDims.w, height: initialDims.h, lod: STREAMING_LODS[lodIdx] }));
      await syncFallbackBuffers();
    }
  }

  const bootSnap = await restoreHashSnapshotIfPresent();
  if (location.hash.startsWith("#studio=")) launchStudioModal(location.hash.slice(8).trim());
  if (!bootSnap) chip(statusEl, `${STREAMING_LODS[lodIdx].short} · ${lastKernelCount}/15 Kernels · Lawful CCD`, "good");
  requestRender();
  (async () => {
    if (!app) { if (!bootSnap) chip(statusEl, reason ? "Verifier unavailable" : "Engine offline", "bad"); return; }
    for (let w = 0; w < 40; w++) {
      const ex = document.getElementById("frame")?.getAttribute("examples") || "";
      if (/^\d+\/\d+$/.test(ex)) break;
      await new Promise((r) => setTimeout(r, 50));
    }
    if (!bootSnap) await syncVerifierBanner("summary");
    for (let i = 0; i < STREAMING_LODS.length; i++) {
      await yieldTask();
      const f = parseConformantFields(await queryVerifier(`scene:lod:${i}`));
      if (!f) continue;
      const lod = STREAMING_LODS[i];
      lod.res = parseRatioPair(f.res, lod.res); lod.density = parseRatioPair(f.density, lod.density);
      lod.lossless = f.lossless === "1"; lod.sky = parseTriplet(f.sky, 100, lod.sky); lod.fog = parseTriplet(f.fog, 100, lod.fog);
    }
    for (const { node, badge } of badges) {
      await yieldTask();
      const kf = parseConformantFields(await queryVerifier(node.query));
      if (!kf) continue;
      if (kf.semiring) node.semiring = kf.semiring;
      badge.className = "tag good";
      badge.textContent = fmtSemiring(node.semiring);
    }
    await yieldTask();
    const af = parseConformantFields(await queryVerifier("scene:acoustics"));
    if (af?.modes && af?.denom) {
      const nums = af.modes.split("/").map(Number).filter(Number.isFinite);
      if (nums.length) { ACOUSTIC_SPEC.modeNumers = nums; ACOUSTIC_SPEC.modeDenom = Number(af.denom) || 9; }
    }
    requestRender();
  })();
}
