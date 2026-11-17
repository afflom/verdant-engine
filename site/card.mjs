const LODS = [
  { rank: 0, key: "lod:0", short: "LOD0", label: "LOD0 · Coarse Proxy (0.125)", res: 0.125, density: 0.35, lossless: false, sky: [0.09, 0.08, 0.12], fog: [0.14, 0.11, 0.16] },
  { rank: 1, key: "lod:1", short: "LOD1", label: "LOD1 · Medium Detail (0.25)", res: 0.25, density: 0.72, lossless: false, sky: [0.12, 0.17, 0.24], fog: [0.18, 0.24, 0.31] },
  { rank: 2, key: "lod:2", short: "LOD2", label: "LOD2 · High Detail (0.625)", res: 0.625, density: 0.92, lossless: false, sky: [0.16, 0.27, 0.34], fog: [0.24, 0.35, 0.39] },
  { rank: 3, key: "lod:3", short: "LOD3", label: "LOD3 · Lossless UORC (1.00)", res: 1.0, density: 1.0, lossless: true, sky: [0.18, 0.30, 0.38], fog: [0.30, 0.40, 0.44] },
  { rank: 4, key: "scene:lod:4", short: "LOD4", label: "LOD4 · Ultra HD Citadel (2.00)", res: 2.0, density: 2.0, lossless: true, sky: [0.19, 0.33, 0.43], fog: [0.32, 0.43, 0.48] },
];

const NODES = [
  { id: 0, query: "kernel:0", name: "Vaulted Citadel & Observer", semiring: "LinearExact", target: [2.2, 3.8, 9.2], dist: 18.5, yawMilli: 460, pitchMilli: 260 },
  { id: 1, query: "kernel:12", name: "Parabolic Stone Arch Bridge", semiring: "TropicalMinPlus", target: [7.5, 1.8, -4.0], dist: 14, yawMilli: 560, pitchMilli: 340 },
  { id: 2, query: "kernel:13", name: "22-Joint Explorer & Foot IK", semiring: "LinearExact", target: [0, 1.6, 11.5], dist: 6.2, possessExplorer: true },
  { id: 3, query: "kernel:6", name: "XPBD Articulated Rig Bay", semiring: "LinearExact", target: [-6.5, 1.8, 9.0], dist: 10, yawMilli: -540, pitchMilli: 320 },
  { id: 4, query: "kernel:9", name: "Diegetic UI & South Terrace", semiring: "TropicalMinPlus", target: [6.0, 2.0, 8.0], dist: 11, yawMilli: 520, pitchMilli: 300 },
  { id: 5, query: "kernel:14", name: "Citadel Chronicle", semiring: "TropicalMinPlus", target: [1.5, 2.2, -1.0], dist: 8.5, yawMilli: 350, pitchMilli: 260 },
  { id: 6, query: "kernel:3", name: "4-Wheel Suspension Vehicle", semiring: "LinearExact", target: [16.5, 1.2, 0], dist: 11, followMover: true },
];

const ACOUST = { quantumFrames: 128, stereoChannels: 2, fundamentalHz: 110.0, modeNumers: [9, 25, 49, 81, 121], modeDenom: 9, rt60Ms: 3019 };

function pConf(line) {
  if (!line?.startsWith("conformant:")) return null;
  const fields = {};
  for (const part of line.slice(11).split(",")) {
    const eq = part.indexOf("=");
    if (eq > 0) fields[part.slice(0, eq)] = part.slice(eq + 1);
  }
  return fields;
}

function pRat(raw, fb) {
  if (!raw) return fb;
  const [n, d] = raw.split("/").map(Number);
  return Number.isFinite(n) && Number.isFinite(d) && d !== 0 ? n / d : fb;
}

function pTri(raw, scale, fb) {
  if (!raw) return fb;
  const p = raw.split("/").map(Number);
  return p.length === 3 && p.every(Number.isFinite) ? p.map((v) => v / scale) : fb;
}

const yieldTask = () => new Promise((r) => setTimeout(r, 0));
let preW = null;
try {
  if (typeof Worker === "function") preW = new Worker("./engine-worker.mjs", { type: "module" });
} catch {
  preW = null;
}

export default async function start({ app, reason, $, chip, show, parse }) {
  const canvas = $("ve-canvas"), vpEl = canvas?.parentElement, statusEl = $("ve-status");
  const [lodBtn, uorcBtn, renderBtn, importBtn, glbIn, studioBtn, meshBtn, simBtn, camBtn, audioBtn, glbBtn, snapBtn, promptForm, promptInput, hudLod, hudStats, tele] =
    ["lod-btn", "uorc-btn", "render-btn", "import-btn", "glb-file", "studio-btn", "mesh-btn", "sim-btn", "cam-btn", "audio-btn", "glb-btn", "snap-btn", "prompt-form", "prompt-input", "hud-lod", "hud-stats", "telemetry"].map((k) => $(`ve-${k}`));

  let lodIdx = 2, simLive = false, camPreset = "landmark", renderMode = "webgl2", fMov = false;
  let aSlot = 0, gazeMs = 300, pYaw = 0, pPit = 0, pDst = 0, jumpQueued = false;
  let sEye = [10.2, 8.5, 25.3], sTgt = [2.2, 3.8, 9.2], uCell = [0, 0, 0], dragging = false, rafId = 0;
  let lTris = 0, lVis = 24, lKern = 15, wBk = "webgl2", isSw = false, offOk = false;
  let nextReqId = 1, workerFramesInFlight = 0, lSent = 0, fbR = null, fbM = null, stM = null;
  const keys = new Set(), pCalls = new Map(), getStudio = async () => (stM ??= await import("./studio.mjs"));
  const hK = (...a) => a.some((k) => keys.has(k));
  const hasMv = () => hK("KeyW", "KeyA", "KeyS", "KeyD", "ArrowUp", "ArrowDown", "ArrowLeft", "ArrowRight", "Space");
  const isAnim = () => simLive || dragging || fMov || hasMv();

  function pxDims() {
    if (!canvas) return { w: 1, h: 1 };
    const dpr = Math.min(window.devicePixelRatio || 1, isSw ? 1 : 2);
    return { w: Math.max(1, Math.floor((canvas.clientWidth || 320) * dpr)), h: Math.max(1, Math.floor((canvas.clientHeight || 240) * dpr)) };
  }

  function mvInt(dt) {
    const pad = navigator.getGamepads?.()?.[0], dz = (v) => (Math.abs(v) > 0.16 ? v : 0);
    const gx = pad ? dz(pad.axes[0] || 0) : 0, gy = pad ? -dz(pad.axes[1] || 0) : 0;
    const rx = pad ? dz(pad.axes[2] || 0) : 0, ry = pad ? dz(pad.axes[3] || 0) : 0;
    const kf = (hK("KeyW", "ArrowUp") ? 1 : 0) - (hK("KeyS", "ArrowDown") ? 1 : 0);
    const ks = (hK("KeyD", "ArrowRight") ? 1 : 0) - (hK("KeyA", "ArrowLeft") ? 1 : 0);
    const fwd = Math.max(-1, Math.min(1, kf + gy)), strafe = Math.max(-1, Math.min(1, ks + gx));
    const speedScale = hK("ShiftLeft", "ShiftRight") ? 1.65 : 1.0;
    const jump = jumpQueued || keys.has("Space") || Boolean(pad?.buttons?.[0]?.pressed);
    jumpQueued = false;
    let dYaw = pYaw - rx * dt * 1.6, dPitch = pPit + ry * dt * 1.1;
    if (simLive && !dragging && !fMov && camPreset === "landmark" && !fwd && !strafe) dYaw += dt * 0.14;
    const dDist = pDst;
    pYaw = pPit = pDst = 0;
    return { fwd, strafe, speedScale, jump, dYaw, dPitch, dDist, moving: Boolean(fwd || strafe || jump) };
  }

  const hudStr = (fps) => {
    const b = renderMode === "tile" ? "TILE 16x16" : wBk.toUpperCase();
    return `${fps ? `${fps} · ${b}` : b} · ${lTris} tris · ${lVis} vis · S(${uCell[0]},${uCell[2]})`;
  };

  let eW = null;
  if (typeof Worker === "function" && canvas) {
    try {
      eW = preW || new Worker("./engine-worker.mjs", { type: "module" });
      preW = null;
      eW.onmessage = ({ data: msg }) => {
        if (!msg) return;
        if (msg.id && pCalls.has(msg.id)) { const r = pCalls.get(msg.id); pCalls.delete(msg.id); r(msg); return; }
        if (msg.kind === "frame-dropped") { workerFramesInFlight = Math.max(0, workerFramesInFlight - 1); return; }
        if (msg.kind === "telemetry") {
          workerFramesInFlight = Math.max(0, workerFramesInFlight - 1);
          lTris = msg.tris || lTris; lVis = msg.bvhVisible ?? lVis;
          if (Array.isArray(msg.eye)) sEye = msg.eye;
          if (Array.isArray(msg.lookTarget)) sTgt = msg.lookTarget;
          if (Array.isArray(msg.universeCell)) uCell = msg.universeCell;
          stM?.pushStudioTelemetry(msg); stM?.syncModalAudioSpatial(msg.binauralPan, msg.binauralGain);
          if (!isAnim() && hudStats) hudStats.textContent = hudStr("");
          if ((pYaw || pPit || pDst || jumpQueued) && rafId === 0) reqR();
        }
      };
      eW.onerror = () => { workerFramesInFlight = 0; for (const r of pCalls.values()) r(null); pCalls.clear(); };
    } catch { eW = null; }
  }

  const cW = (kind, payload = {}, transfer = []) =>
    !eW ? Promise.resolve(null) : new Promise((res) => {
      const id = nextReqId++;
      const wd = setTimeout(() => { if (pCalls.delete(id)) res(null); }, 15000);
      pCalls.set(id, (v) => { clearTimeout(wd); res(v); });
      eW.postMessage({ kind, id, ...payload }, transfer);
    });

  let prevNow = performance.now(), sClk = 0, frames = 0, lastFpsStamp = prevNow;
  const reqR = () => {
    if (rafId === 0) { prevNow = performance.now(); rafId = requestAnimationFrame(frame); }
  };

  function setSim(next) {
    simLive = Boolean(next);
    if (simBtn) { simBtn.setAttribute("aria-pressed", String(simLive)); simBtn.textContent = simLive ? "Sim: Live" : "Sim: Step"; }
    reqR();
  }

  const apCam = (r) => {
    if (r?.camera) { sEye = r.camera.eye; sTgt = r.camera.lookTarget; }
    if (r?.tris) lTris = r.tris;
  };

  async function cycCam(forcePreset = null) {
    const order = ["landmark", "third", "first"];
    camPreset = forcePreset || order[(order.indexOf(camPreset) + 1) % 3]; fMov = false;
    const spec = camPreset === "landmark"
      ? ["Cam: Orbit", "camera-focus-landmark", { target: [2.2, 3.8, 9.2], dist: 18.5, yawMilli: 460, pitchMilli: 260 }, "Lawful Orbit · Near-Sphere Guarded"]
      : camPreset === "third"
        ? ["Cam: 3rd IK", "camera-set-mode", { mode: 0, simClockSec: sClk }, "3rd-Person Explorer · WASD + Foot IK"]
        : ["Cam: 1st Eye", "camera-set-mode", { mode: 1, simClockSec: sClk }, "1st-Person Possessed · Head Masked"];
    if (camBtn) camBtn.textContent = spec[0];
    apCam(await cW(spec[1], spec[2])); chip(statusEl, spec[3], "good");
    await syncFb(); reqR();
  }

  async function doStory() {
    const r = await cW("story-interact");
    if (!r?.ok) return null;
    chip(statusEl, r.advanced ? `Chronicle -> ${r.title} · Supply ${r.supply}` : `${r.title} · ${r.objective}`, r.advanced ? "good" : "wait");
    return r;
  }

  const qVer = async (q) => { try { return app ? show(await app.dispatch(parse(q))) : null; } catch { return null; } };

  async function syncB(query = LODS[lodIdx].key) {
    if (!app) { chip(statusEl, reason ? "Verifier unavailable" : "Engine offline", "bad"); return; }
    const res = await qVer(query);
    if (!res) return;
    if (res.startsWith("conformant:")) {
      const lod = LODS[lodIdx];
      chip(statusEl, `${lod.short} · ${lod.lossless ? `${lKern}/15 Kernels` : "UORC Stream OK"}`, "good");
    } else chip(statusEl, res.startsWith("refused:") ? res.replace("refused:", "Refused: ") : res, res.startsWith("refused:") ? "warn" : "bad");
  }

  async function syncFb() {
    if (offOk || !fbR || !eW) return;
    const r = await cW("get-packed-buffers", { simClockSec: sClk });
    if (r?.ok) { fbR.uploadPackedBuffers(r.pos, r.norm, r.col, r.idx); lTris = r.tris || 0; apCam(r); }
  }

  async function togRnd(force = null) {
    renderMode = force || (renderMode === "webgl2" ? "tile" : "webgl2");
    if (renderBtn) renderBtn.textContent = renderMode === "tile" ? "Tile 16x16" : "WebGL2";
    const r = await cW("set-render-mode", { mode: renderMode, simClockSec: sClk });
    if (hudStats) hudStats.textContent = hudStr("");
    chip(statusEl, renderMode === "tile" ? `Wasm 16x16 Tile PBR (${r?.tris ?? lTris} tris)` : `3-Pass WebGL2 HD (${r?.tris ?? lTris} tris)`, "good");
    reqR();
  }

  async function impGlb(fileName, rawBytes) {
    chip(statusEl, `Importing ${fileName}…`, "wait");
    const copy = new Uint8Array(rawBytes);
    const r = await cW("import-glb", { bytes: copy }, [copy.buffer]);
    if (!r?.ok) { chip(statusEl, `Refused invalid .glb (${fileName})`, "warn"); return; }
    lTris = r.tris || lTris;
    const mod = await getStudio();
    if (r.uorcBytes?.byteLength) await mod.cacheBytesInOpfs(`${fileName}.uorc`, r.uorcBytes);
    await syncFb();
    if (hudLod) hudLod.textContent = `GLB · ${fileName} (${r.tris} tris)`;
    chip(statusEl, `Imported ${fileName} · ${r.tris} tris · .uorc cached`, "good");
    reqR();
  }

  async function rstGlb() {
    const r = await cW("reset-glb");
    if (r?.tris) lTris = r.tris;
    if (hudLod) hudLod.textContent = LODS[lodIdx].label;
    await syncFb();
    await syncB("summary");
    reqR();
  }

  const execP = async (text) =>
    (await getStudio()).routeStudioPrompt(text, {
      callWorker: cW, chip, statusEl, streamUorcToRank: strUorc, cycleCameraMode: cycCam,
      interactChronicle: doStory, focusNodePreset: focNd, setSimLive: setSim, toggleRenderBackend: togRnd,
      restoreRawSnapshot: rstSnap, launchStudioModal: openSt,
    });

  async function focPt(target, dist = 12, label = "Entity") {
    camPreset = "landmark"; fMov = false;
    if (camBtn) camBtn.textContent = "Cam: Orbit";
    apCam(await cW("camera-focus-landmark", { target, dist, yawMilli: 520, pitchMilli: 300, simClockSec: sClk }));
    await syncFb(); reqR();
    chip(statusEl, `Framed ${label} · Guarded Orbit`, "good");
  }

  async function setWorldSeed(seed) {
    const r = await cW("set-seed", { seed });
    if (r?.ok) {
      apCam(r); await syncFb(); reqR();
      chip(statusEl, `Seed ${r.seed} · ${LODS[lodIdx].short} (${lTris} tris)`, "good");
    }
    return r;
  }

  async function openSt(initialTab = null) {
    const mod = await getStudio();
    mod.openStudioModal({
      initialTab: typeof initialTab === "string" ? initialTab : null, callWorker: cW, queryVerifier: qVer,
      onPickGlb: () => glbIn?.click(), onResetGlb: rstGlb,
      onExportGlb: () => mod.exportGlbDownload(cW, LODS[lodIdx].short),
      onShareSnapshot: () => mod.shareVdntSnapshot(cW, chip, statusEl),
      onToggleRenderMode: togRnd, onSetLod: strUorc,
      onSetCameraMode: cycCam, onSetSimLive: setSim,
      onToggleAudio: () => mod.toggleModalAudio(audioBtn, ACOUST, () => sEye),
      onFocusPoint: focPt, onFocusPreset: focNd, onSetSeed: setWorldSeed,
      onOpenStitchMesh: async () => (await import("./stitch-mesh.mjs")).openStitchMeshModal({ callWorker: cW, onRestoreSnapshot: rstSnap }),
      onRoutePrompt: execP, onInteractStory: doStory,
      getRenderMode: () => renderMode, getLodRank: () => lodIdx,
    });
  }

  async function setLod(nextIdx, updateBanner = true) {
    lodIdx = ((nextIdx % LODS.length) + LODS.length) % LODS.length;
    const lod = LODS[lodIdx];
    if (lodBtn) lodBtn.textContent = lod.short;
    if (hudLod) hudLod.textContent = lod.label;
    if (lod.rank < 2) fMov = false;
    if (eW) { eW.postMessage({ kind: "set-lod", lod, simClockSec: sClk }); await syncFb(); }
    reqR(); if (updateBanner) syncB(lod.key);
  }

  async function focNd(idx) {
    const node = NODES[idx];
    if (!node) return;
    aSlot = node.id; gazeMs = 1400;
    if (lodIdx < 3) await setLod(3, false);
    fMov = Boolean(node.followMover);
    if (fMov || node.id === 3) setSim(true);
    camPreset = node.possessExplorer ? "third" : "landmark";
    if (camBtn) camBtn.textContent = node.possessExplorer ? "Cam: 3rd IK" : "Cam: Orbit";
    apCam(await cW("camera-focus-landmark", {
      possessExplorer: Boolean(node.possessExplorer), target: node.target, dist: node.dist,
      yawMilli: node.yawMilli ?? 620, pitchMilli: node.pitchMilli ?? 350, sClk,
    }));
    await syncFb(); reqR(); await syncB(node.query);
  }

  async function strUorc(targetRank) {
    const clamped = Math.max(0, Math.min(LODS.length - 1, targetRank | 0));
    lodIdx = clamped;
    const lod = LODS[lodIdx];
    if (lodBtn) lodBtn.textContent = lod.short;
    if (hudLod) hudLod.textContent = lod.label;
    if (lod.rank < 2) fMov = false;
    chip(statusEl, `Streaming .uorc -> ${lod.short}…`, "wait");
    if (eW) {
      const r = await cW("stream-uorc", { targetRank: clamped, lod, simClockSec: sClk });
      if (r?.tris) lTris = r.tris;
      await syncFb();
      reqR();
      if (r?.ok) { chip(statusEl, `UORC Wasm · ${lod.short} (${lTris} tris · Σ=${r.codedChecksum})`, "good"); return r; }
    }
    await setLod(clamped, true);
    return null;
  }

  const onClick = (el, fn) => el?.addEventListener("click", fn);
  onClick(lodBtn, () => setLod(lodIdx + 1));
  onClick(uorcBtn, () => strUorc(lodIdx < 4 ? 4 : 0));
  onClick(renderBtn, () => togRnd());
  onClick(importBtn, () => glbIn?.click());
  glbIn?.addEventListener("change", async () => { const f = glbIn.files?.[0]; if (f) { await impGlb(f.name, await f.arrayBuffer()); glbIn.value = ""; } });
  onClick(studioBtn, () => openSt());
  onClick(meshBtn, async () => (await import("./stitch-mesh.mjs")).openStitchMeshModal({ callWorker: cW, onRestoreSnapshot: rstSnap }));
  onClick(simBtn, () => setSim(!simLive));
  onClick(camBtn, () => cycCam());
  promptForm?.addEventListener("submit", async (e) => { e.preventDefault(); await execP(promptInput?.value); if (promptInput) promptInput.value = ""; });
  vpEl?.addEventListener("dragover", (e) => { e.preventDefault(); vpEl.classList.add("drag-over"); });
  vpEl?.addEventListener("dragleave", () => vpEl.classList.remove("drag-over"));
  vpEl?.addEventListener("drop", async (e) => { e.preventDefault(); vpEl.classList.remove("drag-over"); const f = e.dataTransfer?.files?.[0]; if (f) await impGlb(f.name, await f.arrayBuffer()); });
  onClick(audioBtn, async () => (await getStudio()).toggleModalAudio(audioBtn, ACOUST, () => sEye));
  onClick(glbBtn, async () => (await getStudio()).exportGlbDownload(cW, LODS[lodIdx].short));
  onClick(snapBtn, async () => (await getStudio()).shareVdntSnapshot(cW, chip, statusEl));

  async function rstSnap(bytes, label = "VDNT") {
    const r = await cW("restore-snapshot", { bytes });
    if (r?.ok) { await setLod(r.lodRank ?? 3, false); chip(statusEl, `Restored ${label} · ${LODS[lodIdx].short} (${r.byteLen}B)`, "good"); return true; }
    chip(statusEl, `Refused tampered ${label} snapshot`, "warn");
    return false;
  }

  async function rstHash() {
    const m = /^#(uorc|vdnt)=(.+)$/.exec(location.hash);
    if (!m) return false;
    const bytes = (await getStudio()).base64UrlToBytes(m[2].trim());
    if (!bytes?.byteLength) { chip(statusEl, "Refused malformed UORC snapshot", "warn"); return true; }
    await rstSnap(bytes, "UORC");
    return true;
  }

  addEventListener("hashchange", () => {
    if (/^#(uorc|vdnt)=/.test(location.hash)) rstHash();
    else if (location.hash.startsWith("#studio=")) openSt(location.hash.slice(8).trim());
  });

  let lx = 0, ly = 0;
  const onC = (ev, fn, opt) => canvas?.addEventListener(ev, fn, opt);
  onC("pointerdown", (e) => { dragging = true; lx = e.clientX; ly = e.clientY; canvas.setPointerCapture?.(e.pointerId); reqR(); });
  onC("pointermove", (e) => {
    if (!dragging) return;
    const dx = e.clientX - lx, dy = e.clientY - ly;
    lx = e.clientX; ly = e.clientY; gazeMs = Math.max(80, gazeMs - 40);
    pYaw -= dx * 0.007; pPit += camPreset === "first" ? -dy * 0.0055 : dy * 0.006; reqR();
  });
  onC("pointerup", () => { dragging = false; });
  onC("pointercancel", () => { dragging = false; });
  onC("wheel", (e) => { e.preventDefault(); pDst += e.deltaY * 0.018; reqR(); }, { passive: false });
  addEventListener("keydown", (e) => {
    if (/^(INPUT|TEXTAREA|SELECT)$/.test(document.activeElement?.tagName ?? "")) return;
    if (e.code === "KeyE" && !e.repeat) { doStory(); return; }
    if (e.code === "KeyG" && !e.repeat) { openSt("create"); return; }
    if (e.code === "Space") jumpQueued = true;
    keys.add(e.code);
    if (hasMv() && camPreset === "landmark") {
      camPreset = "third";
      fMov = false;
      if (camBtn) camBtn.textContent = "Cam: 3rd IK";
    }
    reqR();
  });
  addEventListener("keyup", (e) => keys.delete(e.code));
  addEventListener("resize", reqR);
  document.addEventListener("visibilitychange", () => { if (!document.hidden) { prevNow = performance.now(); reqR(); } });
  if (typeof ResizeObserver !== "undefined" && vpEl) new ResizeObserver(reqR).observe(vpEl);

  function frame(now) {
    rafId = 0;
    if (document.hidden) return;
    if (offOk && eW && workerFramesInFlight >= 2) {
      if (now - lSent > 1500) workerFramesInFlight = 0;
      else { if (isAnim()) rafId = requestAnimationFrame(frame); return; }
    }
    const dt = Math.min(0.05, Math.max(0.001, (now - prevNow) * 0.001));
    prevNow = now;
    const intent = mvInt(dt);
    const stepping = simLive || fMov || intent.moving;
    if (stepping) sClk += dt;
    if (!dragging) gazeMs = Math.min(3000, gazeMs + Math.round(dt * 1000));
    const lod = LODS[lodIdx], { w, h } = pxDims();

    if (offOk && eW) {
      workerFramesInFlight++; lSent = now;
      eW.postMessage({
        kind: "frame", width: w, height: h, dt, stepPhysics: stepping, followMover: fMov,
        fwd: intent.fwd, strafe: intent.strafe, speedScale: intent.speedScale, jump: intent.jump,
        deltaYaw: intent.dYaw, deltaPitch: intent.dPitch, deltaDist: intent.dDist,
        gazedSlot: aSlot, dwellMs: gazeMs, angSpeedMilli: dragging ? 600 : 60, lod, simClockSec: sClk,
      });
    } else if (fbR && fbM && canvas) {
      if (canvas.width !== w || canvas.height !== h) { canvas.width = w; canvas.height = h; }
      const proj = fbM.mat4Perspective(Math.PI / 3.4, Math.max(0.2, w / h), 0.25, 160.0);
      fbR.render(fbM.mat4Mul(proj, fbM.mat4LookAt(sEye, sTgt, [0, 1, 0])), sEye, lod, sClk, sTgt, Math.PI / 3.4);
    }

    frames++;
    if (!isAnim()) { if (hudStats && lTris > 0) hudStats.textContent = hudStr(""); }
    else if (now - lastFpsStamp >= 600) {
      const fps = Math.max(1, Math.round((frames * 1000) / (now - lastFpsStamp)));
      frames = 0; lastFpsStamp = now;
      if (hudStats) hudStats.textContent = hudStr(`${fps} fps`);
    }
    if (isAnim()) rafId = requestAnimationFrame(frame);
  }

  const fmtS = (s) => s.replace(/([a-z])([A-Z])/g, "$1 $2"), badges = [];
  const el = (t, c, x) => { const n = document.createElement(t); if (c) n.className = c; if (x) n.textContent = x; return n; };
  const mkRow = (l, t, fn, id) => {
    const li = el("li"), b = el("button", "again"), bg = el("span", "tag good", t);
    b.type = "button"; if (id) b.id = id; b.append(el("span", 0, l), bg); b.onclick = fn; li.append(b); tele.append(li); return bg;
  };
  if (tele) {
    tele.replaceChildren();
    mkRow("Guided Journeys & Create Game", "7 Personas", () => openSt("journeys"), "ve-journeys-rail-btn");
    for (const node of NODES) badges.push({ node, badge: mkRow(node.name, fmtS(node.semiring), () => focNd(node.id)) });
  }
  const apInit = (r) => {
    if (!r) return;
    apCam(r);
    if (r.audioSpec) Object.assign(ACOUST, r.audioSpec);
    if (r.tensorKernels) lKern = r.tensorKernels;
    if (Array.isArray(r.universeCell)) uCell = r.universeCell;
  };
  const iD = pxDims(); let cXfer = false;
  if (eW && canvas && typeof canvas.transferControlToOffscreen === "function") {
    try {
      const oc = canvas.transferControlToOffscreen(); cXfer = true;
      const r = await cW("init", { canvas: oc, width: iD.w, height: iD.h, lod: LODS[lodIdx] }, [oc]);
      if (r?.kind === "ready") { offOk = true; wBk = r.backend || "webgl2"; isSw = Boolean(r.isSoftware); apInit(r); if (hudStats) hudStats.textContent = hudStr(""); }
    } catch { offOk = false; }
  }
  if (!offOk && !cXfer && canvas) {
    fbM = await import("./scene-renderer.mjs"); fbR = await fbM.createBestRenderer(canvas);
    wBk = fbR?.backend || "webgl2"; isSw = Boolean(fbR?.isSoftware);
    if (eW) { apInit(await cW("init", { canvas: null, width: iD.w, height: iD.h, lod: LODS[lodIdx] })); await syncFb(); }
  }

  const bootSnap = await rstHash();
  if (location.hash.startsWith("#studio=")) openSt(location.hash.slice(8).trim());
  if (!bootSnap) chip(statusEl, `${LODS[lodIdx].short} · ${lKern}/15 Kernels · Lawful CCD`, "good");
  reqR();
  (async () => {
    if (!app) { if (!bootSnap) chip(statusEl, reason ? "Verifier unavailable" : "Engine offline", "bad"); return; }
    for (let w = 0; w < 40; w++) {
      const ex = document.getElementById("frame")?.getAttribute("examples") || "";
      if (/^\d+\/\d+$/.test(ex)) break;
      await new Promise((r) => setTimeout(r, 50));
    }
    if (!bootSnap) await syncB("summary");
    for (let i = 0; i < LODS.length; i++) {
      await yieldTask();
      const f = pConf(await qVer(`scene:lod:${i}`));
      if (!f) continue;
      const lod = LODS[i];
      lod.res = pRat(f.res, lod.res); lod.density = pRat(f.density, lod.density);
      lod.lossless = f.lossless === "1"; lod.sky = pTri(f.sky, 100, lod.sky); lod.fog = pTri(f.fog, 100, lod.fog);
    }
    for (const { node, badge } of badges) {
      await yieldTask();
      const kf = pConf(await qVer(node.query));
      if (!kf) continue;
      if (kf.semiring) node.semiring = kf.semiring;
      badge.className = "tag good";
      badge.textContent = fmtS(node.semiring);
    }
    await yieldTask();
    const af = pConf(await qVer("scene:acoustics"));
    if (af?.modes && af?.denom) {
      const nums = af.modes.split("/").map(Number).filter(Number.isFinite);
      if (nums.length) { ACOUST.modeNumers = nums; ACOUST.modeDenom = Number(af.denom) || 9; }
    }
    reqR();
  })();
}
