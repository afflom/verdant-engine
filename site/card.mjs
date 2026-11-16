const STREAMING_LODS = [
  { rank: 0, key: "lod:0", short: "LOD0", label: "LOD0 · Coarse Proxy (0.125)", res: 0.125, density: 0.35, lossless: false, sky: [0.09, 0.08, 0.12], fog: [0.14, 0.11, 0.16] },
  { rank: 1, key: "lod:1", short: "LOD1", label: "LOD1 · Medium Detail (0.25)", res: 0.25, density: 0.72, lossless: false, sky: [0.12, 0.17, 0.24], fog: [0.18, 0.24, 0.31] },
  { rank: 2, key: "lod:2", short: "LOD2", label: "LOD2 · High Detail (0.625)", res: 0.625, density: 0.92, lossless: false, sky: [0.16, 0.27, 0.34], fog: [0.24, 0.35, 0.39] },
  { rank: 3, key: "lod:3", short: "LOD3", label: "LOD3 · Lossless UORC (1.00)", res: 1.0, density: 1.0, lossless: true, sky: [0.18, 0.30, 0.38], fog: [0.30, 0.40, 0.44] },
];

const NODE_KERNELS = [
  { id: 0, query: "kernel:0", sceneQuery: "scene:node:0", name: "Central Pavilion & Observer", semiring: "LinearExact", target: [0, 2.2, 0], dist: 16 },
  { id: 1, query: "kernel:1", sceneQuery: "scene:node:1", name: "Axial Skinning Spire", semiring: "LinearExact", target: [9.5, 1.2, -3.5], dist: 10 },
  { id: 2, query: "kernel:2", sceneQuery: "scene:node:2", name: "Modal Acoustics Frame", semiring: "TropicalMaxPlus", target: [-9.0, 1.8, -4.0], dist: 10 },
  { id: 3, query: "kernel:3", sceneQuery: "scene:node:3", name: "XPBD Articulated Rig Bay", semiring: "LinearExact", target: [-6.5, 1.8, 9.0], dist: 10 },
  { id: 4, query: "kernel:4", sceneQuery: "scene:node:4", name: "Elevated Deck & Diegetic UI", semiring: "TropicalMaxPlus", target: [5.5, 1.2, 8.5], dist: 12 },
  { id: 5, query: "kernel:5", sceneQuery: "scene:node:5", name: "Tropical NavMesh Corridor", semiring: "TropicalMinPlus", target: [0, 0.8, 5.0], dist: 18 },
  { id: 6, query: "kernel:6", sceneQuery: "scene:node:6", name: "4-Wheel Suspension Vehicle", semiring: "LinearExact", target: [16.5, 1.2, 0], dist: 11, followMover: true },
];

const ACOUSTIC_SPEC = { quantumFrames: 128, stereoChannels: 2, fundamentalHz: 110.0, modeNumers: [9, 25, 49, 81, 121], modeDenom: 9, rt60Ms: 3019 };

function bytesToBase64Url(bytes) {
  let bin = "";
  for (let i = 0; i < bytes.byteLength; i++) bin += String.fromCharCode(bytes[i]);
  return btoa(bin).replace(/\+/g, "-").replace(/\//g, "_").replace(/=+$/g, "");
}

function base64UrlToBytes(raw) {
  if (!raw || /[^A-Za-z0-9\-_]/.test(raw)) return null;
  try {
    const bin = atob(raw.replace(/-/g, "+").replace(/_/g, "/") + "===".slice((raw.length + 3) % 4));
    return Uint8Array.from(bin, (c) => c.charCodeAt(0));
  } catch {
    return null;
  }
}

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

export default async function start({ app, reason, $, chip, show, parse }) {
  const canvas = $("ve-canvas"), viewportEl = canvas?.parentElement, statusEl = $("ve-status");
  const lodBtn = $("ve-lod-btn"), uorcBtn = $("ve-uorc-btn"), renderBtn = $("ve-render-btn");
  const importBtn = $("ve-import-btn"), glbFileInput = $("ve-glb-file"), studioBtn = $("ve-studio-btn");
  const meshBtn = $("ve-mesh-btn"), simBtn = $("ve-sim-btn"), camBtn = $("ve-cam-btn");
  const audioBtn = $("ve-audio-btn"), glbBtn = $("ve-glb-btn"), snapBtn = $("ve-snap-btn");
  const promptForm = $("ve-prompt-form"), promptInput = $("ve-prompt-input");
  const hudLod = $("ve-hud-lod"), hudStats = $("ve-hud-stats"), telemetryList = $("ve-telemetry");

  let lodIdx = 3, simLive = false, camMode = "orbit", renderMode = "webgl2", followMover = false;
  let activeNodeSlot = 0, gazeDwellMs = 300, orbitYaw = 0.68, orbitPitch = 0.42, orbitDist = 24.0;
  let target = [0, 1.8, 1.5], walkPos = [0, 2.2, 13.5], walkYaw = Math.PI, walkPitch = -0.08;
  let dragging = false, rafId = 0, currentEye = [16, 11, 18], lastMoverPos = [16.5, 0.5, 0];
  let lastTris = 0, lastBvhVis = 32, lastKernelCount = 12, lastAttendedSlot = 0;
  let workerBackend = "webgl2", isSoftwareGpu = false, offscreenActive = false, nextReqId = 1;
  let audioCtx = null, workletNode = null, audioTimer = 0, audioOn = false, modalStrikeIndex = 0;
  let fallbackRenderer = null, fallbackMod = null, studioMod = null;
  const keys = new Set(), pendingCalls = new Map();

  function computeCameraPose(lod, dt) {
    const pad = navigator.getGamepads?.()?.[0];
    const dz = (v) => (Math.abs(v) > 0.16 ? v : 0);
    const gx = pad ? dz(pad.axes[0] || 0) : 0, gy = pad ? -dz(pad.axes[1] || 0) : 0, lx = pad ? dz(pad.axes[2] || 0) : 0;
    if (camMode === "orbit") {
      if (simLive && !dragging && !followMover) orbitYaw += dt * 0.12;
      orbitYaw -= lx * dt * 1.4;
      if (followMover && lod.rank >= 2) target = [...lastMoverPos];
      const cp = Math.cos(orbitPitch), sp = Math.sin(orbitPitch);
      currentEye = [target[0] + Math.sin(orbitYaw) * cp * orbitDist, Math.max(0.8, target[1] + sp * orbitDist), target[2] + Math.cos(orbitYaw) * cp * orbitDist];
      return { eye: currentEye, lookTarget: target, fwd: gy, strafe: gx, yaw: orbitYaw };
    }
    const kf = (keys.has("KeyW") || keys.has("ArrowUp") ? 1 : 0) - (keys.has("KeyS") || keys.has("ArrowDown") ? 1 : 0);
    const ks = (keys.has("KeyD") || keys.has("ArrowRight") ? 1 : 0) - (keys.has("KeyA") || keys.has("ArrowLeft") ? 1 : 0);
    const fwd = Math.max(-1, Math.min(1, kf + gy)), strafe = Math.max(-1, Math.min(1, ks + gx));
    walkYaw -= lx * dt * 1.6;
    const sp = 7.5 * dt;
    walkPos = [
      Math.max(-25, Math.min(25, walkPos[0] + (Math.sin(walkYaw) * fwd - Math.cos(walkYaw) * strafe) * sp)),
      2.1,
      Math.max(-25, Math.min(25, walkPos[2] + (Math.cos(walkYaw) * fwd + Math.sin(walkYaw) * strafe) * sp)),
    ];
    currentEye = [...walkPos];
    return {
      eye: currentEye,
      lookTarget: [walkPos[0] + Math.sin(walkYaw) * Math.cos(walkPitch), walkPos[1] + Math.sin(walkPitch), walkPos[2] + Math.cos(walkYaw) * Math.cos(walkPitch)],
      fwd, strafe, yaw: walkYaw,
    };
  }

  function measureCanvasPixels() {
    if (!canvas) return { w: 1, h: 1 };
    const dpr = Math.min(window.devicePixelRatio || 1, isSoftwareGpu ? 1 : 2);
    return { w: Math.max(1, Math.floor((canvas.clientWidth || 320) * dpr)), h: Math.max(1, Math.floor((canvas.clientHeight || 240) * dpr)) };
  }

  const formatHudStats = (fps) => {
    const b = renderMode === "tile" ? "TILE 16x16" : workerBackend.toUpperCase();
    return `${fps ? `${fps} · ${b}` : b} · ${lastTris} tris · ${lastBvhVis} vis · N${lastAttendedSlot}`;
  };

  const isAnimating = () => simLive || dragging || followMover || (camMode === "walk" && keys.size > 0);

  let engineWorker = null;
  if (typeof Worker === "function" && canvas) {
    try {
      engineWorker = new Worker("./engine-worker.mjs", { type: "module" });
      engineWorker.onmessage = ({ data: msg }) => {
        if (!msg) return;
        if (msg.id && pendingCalls.has(msg.id)) {
          const r = pendingCalls.get(msg.id);
          pendingCalls.delete(msg.id);
          r(msg);
          return;
        }
        if (msg.kind === "telemetry") {
          lastTris = msg.tris || lastTris;
          lastBvhVis = msg.bvhVisible ?? lastBvhVis;
          lastAttendedSlot = msg.attendedSlot ?? lastAttendedSlot;
          if (Array.isArray(msg.moverPos)) lastMoverPos = msg.moverPos;
          studioMod?.pushStudioTelemetry(msg);
          if (workletNode && audioOn) workletNode.port.postMessage({ kind: "spatial", pan: msg.binauralPan, gain: msg.binauralGain });
          if (!isAnimating() && hudStats) hudStats.textContent = formatHudStats("");
        }
      };
      engineWorker.onerror = () => {
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
      pendingCalls.set(id, res);
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
      chip(statusEl, `${lod.short} · ${lod.lossless ? `${lastKernelCount}/12 Kernels` : "UORC Stream OK"}`, "good");
    } else if (res.startsWith("refused:")) chip(statusEl, res.replace("refused:", "Refused: "), "warn");
    else chip(statusEl, res, "bad");
  }

  async function syncFallbackBuffers() {
    if (offscreenActive || !fallbackRenderer || !engineWorker) return;
    const r = await callWorker("get-packed-buffers", { simClockSec });
    if (r?.ok) { fallbackRenderer.uploadPackedBuffers(r.pos, r.norm, r.col, r.idx); lastTris = r.tris || 0; }
  }

  async function toggleRenderBackend(force = null) {
    renderMode = force || (renderMode === "webgl2" ? "tile" : "webgl2");
    if (renderBtn) renderBtn.textContent = renderMode === "tile" ? "Tile 16x16" : "WebGL2";
    const pose = computeCameraPose(STREAMING_LODS[lodIdx], 0.016);
    const r = await callWorker("set-render-mode", { mode: renderMode, eye: pose.eye, lookTarget: pose.lookTarget, simClockSec });
    if (hudStats) hudStats.textContent = formatHudStats("");
    chip(statusEl, renderMode === "tile" ? `Wasm 16x16 Tile PBR (${r?.tris ?? lastTris} tris)` : `WebGL2 Hardware (${r?.tris ?? lastTris} tris)`, "good");
    requestRender();
  }

  async function importGlbBytes(fileName, rawBytes) {
    chip(statusEl, `Importing ${fileName}…`, "wait");
    const pose = computeCameraPose(STREAMING_LODS[lodIdx], 0.016);
    const copy = new Uint8Array(rawBytes);
    const r = await callWorker("import-glb", { bytes: copy, eye: pose.eye, lookTarget: pose.lookTarget }, [copy.buffer]);
    if (!r?.ok) { chip(statusEl, `Refused invalid .glb (${fileName})`, "warn"); return; }
    lastTris = r.tris || lastTris;
    studioMod ??= await import("./studio.mjs");
    if (r.vdntBytes?.byteLength) await studioMod.cacheBytesInOpfs(`${fileName}.vdnt`, r.vdntBytes);
    await syncFallbackBuffers();
    if (hudLod) hudLod.textContent = `GLB · ${fileName} (${r.tris} tris)`;
    chip(statusEl, `Imported ${fileName} · ${r.tris} tris · OPFS cached`, "good");
    requestRender();
  }

  async function resetImportedGlb() {
    const pose = computeCameraPose(STREAMING_LODS[lodIdx], 0.016);
    const r = await callWorker("reset-glb", { eye: pose.eye, lookTarget: pose.lookTarget });
    if (r?.tris) lastTris = r.tris;
    if (hudLod) hudLod.textContent = STREAMING_LODS[lodIdx].label;
    await syncFallbackBuffers();
    await syncVerifierBanner("summary");
    requestRender();
  }

  async function executePromptIntent(text) {
    const prompt = String(text || "").trim();
    if (!prompt) return null;
    const r = await callWorker("route-prompt", { prompt });
    if (!r?.ok) { chip(statusEl, "Refused empty intent", "warn"); return r; }
    const s = r.slug;
    if (s === "focus-observer") { await focusNodePreset(0); setSimLive(true); }
    else if (s === "drive-vehicle") { await focusNodePreset(6); setSimLive(true); }
    else if (s === "render-tile-wasm") await toggleRenderBackend("tile");
    else if (s === "render-webgl2") await toggleRenderBackend("webgl2");
    else if (s === "stream-uorc-lod3") await streamUorcToRank(3);
    else if (s === "stitch-shard-sync") (await import("./stitch-mesh.mjs")).openStitchMeshModal({ callWorker, onRestoreSnapshot: restoreRawSnapshot });
    else if (s === "inspect-formal-ui") await launchStudioModal();
    else if (s === "step-xpbd-physics") { await focusNodePreset(3); setSimLive(true); }
    chip(statusEl, `SLM -> ${s} (${(r.scoreMilli / 10).toFixed(0)}% coded_gemm)`, "good");
    return r;
  }

  async function launchStudioModal() {
    studioMod ??= await import("./studio.mjs");
    studioMod.openStudioModal({
      callWorker,
      onPickGlb: () => glbFileInput?.click(),
      onResetGlb: resetImportedGlb,
      onToggleRenderMode: () => toggleRenderBackend(),
      onRoutePrompt: executePromptIntent,
      getRenderMode: () => renderMode,
    });
  }

  async function setLod(nextIdx, updateBanner = true) {
    lodIdx = ((nextIdx % STREAMING_LODS.length) + STREAMING_LODS.length) % STREAMING_LODS.length;
    const lod = STREAMING_LODS[lodIdx];
    if (lodBtn) lodBtn.textContent = `LOD: ${lod.short}`;
    if (hudLod) hudLod.textContent = lod.label;
    if (lod.rank < 2) followMover = false;
    const pose = computeCameraPose(lod, 0.016);
    if (engineWorker) {
      engineWorker.postMessage({ kind: "set-lod", lod, eye: pose.eye, lookTarget: pose.lookTarget, simClockSec });
      await syncFallbackBuffers();
    }
    requestRender();
    if (updateBanner) syncVerifierBanner(lod.key);
  }

  async function focusNodePreset(idx) {
    const node = NODE_KERNELS[idx];
    if (!node) return;
    activeNodeSlot = node.id;
    gazeDwellMs = 1400;
    if (lodIdx !== 3) await setLod(3, false);
    camMode = "orbit";
    if (camBtn) camBtn.textContent = "Orbit";
    followMover = Boolean(node.followMover);
    if (followMover || node.id === 3) setSimLive(true);
    target = [...node.target];
    orbitDist = node.dist;
    requestRender();
    await syncVerifierBanner(node.query);
  }

  async function streamUorcToRank(targetRank) {
    const clamped = Math.max(0, Math.min(STREAMING_LODS.length - 1, targetRank | 0));
    lodIdx = clamped;
    const lod = STREAMING_LODS[lodIdx];
    if (lodBtn) lodBtn.textContent = `LOD: ${lod.short}`;
    if (hudLod) hudLod.textContent = lod.label;
    if (lod.rank < 2) followMover = false;
    const pose = computeCameraPose(lod, 0.016);
    chip(statusEl, `Streaming .uorc -> ${lod.short}…`, "wait");
    if (engineWorker) {
      const r = await callWorker("stream-uorc", { targetRank: clamped, lod, eye: pose.eye, lookTarget: pose.lookTarget, simClockSec });
      await syncFallbackBuffers();
      requestRender();
      if (r?.ok) { chip(statusEl, `UORC Wasm · ${lod.short} (${r.streamBytes}B · Σ=${r.codedChecksum})`, "good"); return r; }
    }
    await setLod(clamped, true);
    return null;
  }

  lodBtn?.addEventListener("click", () => setLod(lodIdx + 1));
  uorcBtn?.addEventListener("click", () => streamUorcToRank((lodIdx + 1) % STREAMING_LODS.length));
  renderBtn?.addEventListener("click", () => toggleRenderBackend());
  importBtn?.addEventListener("click", () => glbFileInput?.click());
  glbFileInput?.addEventListener("change", async () => {
    const f = glbFileInput.files?.[0];
    if (f) { await importGlbBytes(f.name, await f.arrayBuffer()); glbFileInput.value = ""; }
  });
  studioBtn?.addEventListener("click", launchStudioModal);
  meshBtn?.addEventListener("click", async () => (await import("./stitch-mesh.mjs")).openStitchMeshModal({ callWorker, onRestoreSnapshot: restoreRawSnapshot }));
  simBtn?.addEventListener("click", () => setSimLive(!simLive));
  camBtn?.addEventListener("click", () => { camMode = camMode === "orbit" ? "walk" : "orbit"; camBtn.textContent = camMode === "orbit" ? "Orbit" : "Walk"; requestRender(); });
  promptForm?.addEventListener("submit", async (e) => { e.preventDefault(); await executePromptIntent(promptInput?.value); if (promptInput) promptInput.value = ""; });
  viewportEl?.addEventListener("dragover", (e) => { e.preventDefault(); viewportEl.classList.add("drag-over"); });
  viewportEl?.addEventListener("dragleave", () => viewportEl.classList.remove("drag-over"));
  viewportEl?.addEventListener("drop", async (e) => { e.preventDefault(); viewportEl.classList.remove("drag-over"); const f = e.dataTransfer?.files?.[0]; if (f) await importGlbBytes(f.name, await f.arrayBuffer()); });

  function triggerModalStrike() {
    if (!audioCtx || !audioOn) return;
    if (workletNode) { workletNode.port.postMessage({ kind: "strike", modeIndex: modalStrikeIndex++ }); return; }
    const dist = Math.hypot(currentEye[0] + 9.0, currentEye[1] - 2.0, currentEye[2] + 4.0);
    const atten = Math.min(0.14, 0.9 / (1.0 + 0.18 * dist));
    const idx = (modalStrikeIndex++) % ACOUSTIC_SPEC.modeNumers.length;
    const freq = Math.min(1320.0, (ACOUSTIC_SPEC.fundamentalHz * ACOUSTIC_SPEC.modeNumers[idx]) / ACOUSTIC_SPEC.modeDenom);
    const rt = Math.max(0.5, ACOUSTIC_SPEC.rt60Ms / 1000.0), now = audioCtx.currentTime;
    const osc = audioCtx.createOscillator(), gain = audioCtx.createGain();
    osc.frequency.setValueAtTime(freq, now);
    gain.gain.setValueAtTime(0.0001, now);
    gain.gain.exponentialRampToValueAtTime(atten, now + 0.02);
    gain.gain.exponentialRampToValueAtTime(0.0001, now + rt);
    osc.connect(gain).connect(audioCtx.destination);
    osc.start(now); osc.stop(now + rt + 0.05);
  }

  audioBtn?.addEventListener("click", async () => {
    const Ctx = window.AudioContext || window.webkitAudioContext;
    if (!Ctx) return;
    audioOn = !audioOn;
    audioBtn.setAttribute("aria-pressed", String(audioOn));
    audioBtn.textContent = audioOn ? "Audio: On" : "Audio";
    if (audioOn) {
      audioCtx ??= new Ctx();
      await audioCtx.resume?.();
      if (!workletNode && audioCtx.audioWorklet && typeof AudioWorkletNode === "function") {
        try {
          await audioCtx.audioWorklet.addModule("./audio-worklet.mjs");
          workletNode = new AudioWorkletNode(audioCtx, "verdant-modal-processor", {
            numberOfOutputs: 1, outputChannelCount: [ACOUSTIC_SPEC.stereoChannels],
            processorOptions: { quantumFrames: ACOUSTIC_SPEC.quantumFrames, modeNumers: ACOUSTIC_SPEC.modeNumers, modeDenom: ACOUSTIC_SPEC.modeDenom, rt60Ms: ACOUSTIC_SPEC.rt60Ms, fundamentalHz: ACOUSTIC_SPEC.fundamentalHz },
          });
          workletNode.connect(audioCtx.destination);
        } catch { workletNode = null; }
      }
      clearInterval(audioTimer);
      audioTimer = window.setInterval(triggerModalStrike, 1350);
      triggerModalStrike();
    } else clearInterval(audioTimer);
  });

  glbBtn?.addEventListener("click", async () => {
    const r = await callWorker("export-glb");
    if (!r?.buffer) return;
    const url = URL.createObjectURL(new Blob([r.buffer], { type: "model/gltf-binary" }));
    const a = document.createElement("a");
    a.href = url; a.download = `verdant-engine-${STREAMING_LODS[lodIdx].short.toLowerCase()}.glb`; a.click();
    setTimeout(() => URL.revokeObjectURL(url), 1500);
  });

  async function restoreRawSnapshot(bytes, label = "VDNT") {
    const r = await callWorker("restore-snapshot", { bytes });
    if (r?.ok) { await setLod(r.lodRank ?? 3, false); chip(statusEl, `Restored ${label} · ${STREAMING_LODS[lodIdx].short} (${r.byteLen}B)`, "good"); return true; }
    chip(statusEl, `Refused tampered ${label} snapshot`, "warn");
    return false;
  }

  async function restoreHashSnapshotIfPresent() {
    if (!location.hash.startsWith("#vdnt=")) return false;
    const bytes = base64UrlToBytes(location.hash.slice(6).trim());
    if (!bytes?.byteLength) { chip(statusEl, "Refused malformed VDNT snapshot", "warn"); return true; }
    await restoreRawSnapshot(bytes, "VDNT");
    return true;
  }

  snapBtn?.addEventListener("click", async () => {
    const r = await callWorker("encode-snapshot");
    if (!r?.ok || !r.bytes) { chip(statusEl, "Snapshot unavailable", "warn"); return; }
    const bytes = r.bytes instanceof Uint8Array ? r.bytes : new Uint8Array(r.bytes);
    history.replaceState(null, "", `${location.pathname}${location.search}#vdnt=${bytesToBase64Url(bytes)}`);
    try { await navigator.clipboard?.writeText(location.href); } catch {}
    chip(statusEl, `VDNT · ${bytes.byteLength}B SHA-256 (#vdnt=)`, "good");
  });

  addEventListener("hashchange", () => { if (location.hash.startsWith("#vdnt=")) restoreHashSnapshotIfPresent(); });

  let lastX = 0, lastY = 0;
  canvas?.addEventListener("pointerdown", (e) => { dragging = true; lastX = e.clientX; lastY = e.clientY; canvas.setPointerCapture?.(e.pointerId); requestRender(); });
  canvas?.addEventListener("pointermove", (e) => {
    if (!dragging) return;
    const dx = e.clientX - lastX, dy = e.clientY - lastY;
    lastX = e.clientX; lastY = e.clientY;
    gazeDwellMs = Math.max(80, gazeDwellMs - 40);
    if (camMode === "orbit") { orbitYaw -= dx * 0.0075; orbitPitch = Math.max(0.08, Math.min(1.38, orbitPitch + dy * 0.0065)); }
    else { walkYaw -= dx * 0.0055; walkPitch = Math.max(-1.1, Math.min(1.1, walkPitch - dy * 0.0055)); }
    requestRender();
  });
  canvas?.addEventListener("pointerup", () => { dragging = false; });
  canvas?.addEventListener("pointercancel", () => { dragging = false; });
  canvas?.addEventListener("wheel", (e) => { e.preventDefault(); orbitDist = Math.max(5.0, Math.min(42.0, orbitDist + e.deltaY * 0.02)); requestRender(); }, { passive: false });
  addEventListener("keydown", (e) => { if (!/^(INPUT|TEXTAREA|SELECT)$/.test(document.activeElement?.tagName ?? "")) { keys.add(e.code); requestRender(); } });
  addEventListener("keyup", (e) => keys.delete(e.code));
  addEventListener("resize", requestRender);
  if (typeof ResizeObserver !== "undefined" && viewportEl) new ResizeObserver(requestRender).observe(viewportEl);

  function frame(now) {
    rafId = 0;
    const dt = Math.min(0.05, Math.max(0.001, (now - prevNow) * 0.001));
    prevNow = now;
    const stepping = simLive || followMover || (camMode === "walk" && keys.size > 0);
    if (simLive || followMover) simClockSec += dt;
    if (!dragging) gazeDwellMs = Math.min(3000, gazeDwellMs + Math.round(dt * 1000));
    const lod = STREAMING_LODS[lodIdx], { w, h } = measureCanvasPixels(), pose = computeCameraPose(lod, dt);

    if (offscreenActive && engineWorker) {
      engineWorker.postMessage({
        kind: "frame", width: w, height: h, dt, stepPhysics: stepping, fwd: pose.fwd, strafe: pose.strafe,
        walkYaw, playerYaw: pose.yaw, gazedSlot: activeNodeSlot, dwellMs: gazeDwellMs,
        angSpeedMilli: dragging ? 600 : 60, eye: pose.eye, lookTarget: pose.lookTarget, lod, simClockSec,
      });
    } else if (fallbackRenderer && fallbackMod && canvas) {
      if (canvas.width !== w || canvas.height !== h) { canvas.width = w; canvas.height = h; }
      const proj = fallbackMod.mat4Perspective(Math.PI / 3.4, Math.max(0.2, w / h), 0.25, 160.0);
      fallbackRenderer.render(fallbackMod.mat4Mul(proj, fallbackMod.mat4LookAt(pose.eye, pose.lookTarget, [0, 1, 0])), pose.eye, lod, simClockSec);
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

  const badges = [];
  if (telemetryList) {
    telemetryList.replaceChildren();
    for (const node of NODE_KERNELS) {
      const li = document.createElement("li"), btn = document.createElement("button");
      btn.type = "button"; btn.className = "again";
      const lbl = document.createElement("span"); lbl.textContent = node.name;
      const badge = document.createElement("span"); badge.className = "tag"; badge.textContent = node.semiring;
      btn.append(lbl, badge);
      btn.addEventListener("click", () => focusNodePreset(node.id));
      li.append(btn); telemetryList.append(li);
      badges.push({ node, badge });
    }
  }

  const initialDims = measureCanvasPixels(), initialPose = computeCameraPose(STREAMING_LODS[lodIdx], 0.016);
  if (engineWorker && canvas && typeof canvas.transferControlToOffscreen === "function") {
    try {
      const offscreen = canvas.transferControlToOffscreen();
      const r = await callWorker("init", { canvas: offscreen, width: initialDims.w, height: initialDims.h, lod: STREAMING_LODS[lodIdx], eye: initialPose.eye, lookTarget: initialPose.lookTarget }, [offscreen]);
      if (r?.kind === "ready") {
        offscreenActive = true; workerBackend = r.backend || "webgl2"; isSoftwareGpu = Boolean(r.isSoftware);
        lastTris = r.tris || 0; if (r.tensorKernels) lastKernelCount = r.tensorKernels;
        if (r.audioSpec) Object.assign(ACOUSTIC_SPEC, r.audioSpec);
      }
    } catch { offscreenActive = false; }
  }

  if (!offscreenActive && canvas) {
    fallbackMod = await import("./scene-renderer.mjs");
    fallbackRenderer = await fallbackMod.createBestRenderer(canvas);
    workerBackend = fallbackRenderer?.backend || "webgl2";
    isSoftwareGpu = Boolean(fallbackRenderer?.isSoftware);
    if (engineWorker) {
      const r = await callWorker("init", { canvas: null, width: initialDims.w, height: initialDims.h, lod: STREAMING_LODS[lodIdx], eye: initialPose.eye, lookTarget: initialPose.lookTarget });
      if (r?.tensorKernels) lastKernelCount = r.tensorKernels;
      if (r?.audioSpec) Object.assign(ACOUSTIC_SPEC, r.audioSpec);
      await syncFallbackBuffers();
    }
  }

  const bootSnap = await restoreHashSnapshotIfPresent();
  requestRender();
  setTimeout(async () => {
    if (!bootSnap) await syncVerifierBanner("summary");
    if (!app) return;
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
      const sf = parseConformantFields(await queryVerifier(node.sceneQuery));
      if (sf) { node.target = parseTriplet(sf.pos, 10, node.target); const d = Number(sf.dist); if (d > 0) node.dist = d; node.followMover = sf.mover === "1"; }
      const kf = parseConformantFields(await queryVerifier(node.query));
      if (kf?.semiring) node.semiring = kf.semiring;
      badge.className = kf ? "tag good" : "tag";
      badge.textContent = kf ? node.semiring.replace(/([a-z])([A-Z])/g, "$1 $2") : "Offline";
    }
    await yieldTask();
    const af = parseConformantFields(await queryVerifier("scene:acoustics"));
    if (af?.modes && af?.denom) {
      const nums = af.modes.split("/").map(Number).filter(Number.isFinite);
      if (nums.length) { ACOUSTIC_SPEC.modeNumers = nums; ACOUSTIC_SPEC.modeDenom = Number(af.denom) || 9; }
    }
    requestRender();
  }, 60);
}
