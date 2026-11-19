const LODS = [
  { rank: 0, key: "lod:0", short: "LOD0", label: "LOD0 Proxy", res: 0.125, density: 0.35, lossless: false, sky: [0.09, 0.08, 0.12], fog: [0.14, 0.11, 0.16] },
  { rank: 1, key: "lod:1", short: "LOD1", label: "LOD1 Medium", res: 0.25, density: 0.72, lossless: false, sky: [0.12, 0.17, 0.24], fog: [0.18, 0.24, 0.31] },
  { rank: 2, key: "lod:2", short: "LOD2", label: "LOD2 High", res: 0.625, density: 0.92, lossless: false, sky: [0.16, 0.27, 0.34], fog: [0.24, 0.35, 0.39] },
  { rank: 3, key: "lod:3", short: "LOD3", label: "LOD3 Lossless", res: 1.0, density: 1.0, lossless: true, sky: [0.18, 0.30, 0.38], fog: [0.30, 0.40, 0.44] },
  { rank: 4, key: "scene:lod:4", short: "LOD4", label: "LOD4 Ultra HD", res: 2.0, density: 2.0, lossless: true, sky: [0.20, 0.34, 0.44], fog: [0.34, 0.44, 0.48] },
];

const NODES = [
  { id: 0, query: "scene:telos", name: "Master Overview & W5H Governor", target: [2.2, 3.8, 9.2], dist: 18.5, yawMilli: 460, pitchMilli: 260 },
  { id: 1, query: "scene:acoustics", name: "Parabolic Stone Arch Span", target: [7.5, 1.8, -4.0], dist: 13.5, yawMilli: 640, pitchMilli: 320 },
  { id: 2, query: "scene:character", name: "22-Joint Explorer & NavMesh", target: [0, 1.6, 11.5], dist: 6.2, possessExplorer: true },
  { id: 3, query: "scene:joints", name: "Articulated XPBD Rig & ECS Bay", target: [-6.5, 1.8, 9.0], dist: 10, yawMilli: -540, pitchMilli: 320 },
  { id: 4, query: "scene:ui", name: "South Terrace & Tropical UI", target: [6.0, 2.0, 8.0], dist: 11, yawMilli: 520, pitchMilli: 300 },
  { id: 5, query: "scene:narrative", name: "Story Podium & Conservation", target: [1.5, 2.2, -1.0], dist: 8.5, yawMilli: 350, pitchMilli: 260 },
  { id: 6, query: "scene:lawful", name: "Suspension Mover Track", target: [16.5, 1.2, 0], dist: 11, followMover: true },
  { id: 7, query: "scene:uorc", name: "Pavilion Vault & Semiring Dome", target: [0.0, 5.2, 2.0], dist: 12.5, yawMilli: -420, pitchMilli: 220 },
];

const RUNTIME_STRATA = [
  { key: "telos", label: "01 · Ontological Telos", tag: "W5H 8/8", nodeIdx: 0, query: "scene:telos", roleKey: "dashboard" },
  { key: "uorc", label: "02 · UORC & Semirings", tag: "15 Kernels", nodeIdx: 7, query: "scene:uorc", roleKey: "code" },
  { key: "lawful", label: "03 · Swept BLAS & XPBD", tag: "Zero-Tunnel", nodeIdx: 3, query: "scene:lawful", roleKey: "code" },
  { key: "character", label: "04 · 22-Joint IK & Nav", tag: "4,608 Polys", nodeIdx: 2, query: "scene:character", roleKey: "gameplay" },
  { key: "acoustics", label: "05 · Sabine Acoustics", tag: "RT60 + ITD", nodeIdx: 1, query: "scene:acoustics", roleKey: "audio" },
  { key: "ui", label: "06 · Tropical Spatial UI", tag: "DTCG + AA", nodeIdx: 4, query: "scene:ui", roleKey: "design" },
  { key: "narrative", label: "07 · Story & Stitch P2P", tag: "2-of-3 XOR", nodeIdx: 5, query: "scene:narrative", roleKey: "narrative" },
];

const STUDIO_ROLES = [
  { key: "dashboard", label: "Dashboard", nodeIdx: 0, stratumKey: "telos" },
  { key: "code", label: "Code", nodeIdx: 3, stratumKey: "lawful" },
  { key: "design", label: "Design", nodeIdx: 4, stratumKey: "ui" },
  { key: "audio", label: "Audio", nodeIdx: 1, stratumKey: "acoustics" },
  { key: "narrative", label: "Narrative", nodeIdx: 5, stratumKey: "narrative" },
  { key: "director", label: "Director", nodeIdx: 1, stratumKey: "telos" },
  { key: "gameplay", label: "Gameplay", nodeIdx: 2, stratumKey: "character" },
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
  const [lodBtn, uorcBtn, renderBtn, importBtn, glbIn, studioBtn, meshBtn, simBtn, camBtn, audioBtn, glbBtn, snapBtn, promptForm, promptInput, hudLod, hudStats, tele, touchPad, interactBtn, lockBtn] =
    ["lod-btn", "uorc-btn", "render-btn", "import-btn", "glb-file", "studio-btn", "mesh-btn", "sim-btn", "cam-btn", "audio-btn", "glb-btn", "snap-btn", "prompt-form", "prompt-input", "hud-lod", "hud-stats", "telemetry", "touch-pad", "interact-btn", "lock-btn"].map((k) => $(`ve-${k}`));

  let lodIdx = 2, simLive = false, camPreset = "landmark", renderMode = "webgl2", fMov = false;
  let activeProject = "Sunspire Harbor Regatta", activeStratumKey = "telos", activeRoleKey = "director";
  let aSlot = 0, gazeMs = 300, pYaw = 0, pPit = 0, pDst = 0, jumpQueued = false;
  let sEye = [10.2, 8.5, 25.3], sTgt = [2.2, 3.8, 9.2], dragging = false, rafId = 0;
  let lTris = 0, lVis = 24, lClr = 290, isSw = false, offOk = false;
  let nextReqId = 1, workerFramesInFlight = 0, lSent = 0, lSentW = 0, lSentH = 0, fbR = null, fbM = null, stM = null;
  const organModes = new Uint8Array(11);
  const stratumButtons = new Map();

  const isOrganAllowed = (idx, allowGesture = false) =>
    organModes[idx] === 2 ? false : Boolean(organModes[idx] === 1 || ORGAN_DEFAULTS[idx] === 1 || allowGesture);
  const refuseOrgan = (idx) => {
    chip(statusEl, `Refused: ContainerOrganDenied(${ORGAN_SLUGS[idx] || idx})`, "warn");
  };
  const syncPossessedUi = () => {
    vpEl?.classList.toggle("possessed", camPreset !== "landmark");
  };
  const syncHudTitle = (customSuffix = null) => {
    if (!hudLod) return;
    const backendName = renderMode === "tile" ? "Wasm 16x16 Tile" : "WebGL2 PBR";
    hudLod.textContent = customSuffix
      ? `Verdant Engine · ${customSuffix}`
      : `Verdant Engine · ${backendName} · ${LODS[lodIdx].label}`;
  };
  const syncStratumHighlight = (stratumKey) => {
    if (RUNTIME_STRATA.some((s) => s.key === stratumKey)) {
      activeStratumKey = stratumKey;
    }
    for (const [k, btn] of stratumButtons.entries()) {
      btn.setAttribute("aria-current", k === activeStratumKey ? "true" : "false");
    }
  };
  const syncRoleHighlight = (roleKey) => {
    const found = STUDIO_ROLES.find((r) => r.key === roleKey);
    if (found) {
      activeRoleKey = found.key;
      syncStratumHighlight(found.stratumKey);
    }
  };

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
    const pad = isOrganAllowed(5) ? navigator.getGamepads?.()?.[0] : null;
    const dz = (v) => (Math.abs(v) > 0.16 ? v : 0);
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
    const triTag = lTris > 0 ? `${lTris.toLocaleString()} tris · ` : "";
    return `${fps ? `${fps} · ` : ""}Holo Verified · 15 Semirings · ${triTag}Clear ${lClr}mm`;
  };

  let eW = null;
  if (typeof Worker === "function" && canvas) {
    try {
      eW = preW || new Worker("./engine-worker.mjs", { type: "module" });
      preW = null;
      eW.onmessage = ({ data: msg }) => {
        if (!msg) return;
        if (msg.id && pCalls.has(msg.id)) { const r = pCalls.get(msg.id); pCalls.delete(msg.id); r(msg); return; }
        if (msg.kind === "frame-dropped") {
          workerFramesInFlight = Math.max(0, workerFramesInFlight - 1);
          const d = pxDims();
          if ((d.w !== lSentW || d.h !== lSentH) && rafId === 0) reqR();
          return;
        }
        if (msg.kind === "telemetry") {
          workerFramesInFlight = Math.max(0, workerFramesInFlight - 1);
          lTris = msg.tris || lTris; lVis = msg.bvhVisible ?? lVis; lClr = msg.camClearanceMm ?? lClr;
          if (Array.isArray(msg.eye)) sEye = msg.eye;
          if (Array.isArray(msg.lookTarget)) sTgt = msg.lookTarget;
          stM?.pushStudioTelemetry(msg);
          stM?.syncModalAudioSpatial(msg.binauralPan, msg.binauralGain, msg.binauralItdMicros, msg.pavilionContact, msg.attendedSlot);
          if (!isAnim() && hudStats) hudStats.textContent = hudStr("");
          const d = pxDims();
          if ((pYaw || pPit || pDst || jumpQueued || d.w !== lSentW || d.h !== lSentH) && rafId === 0) reqR();
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
    if (simBtn) {
      simBtn.setAttribute("aria-pressed", String(simLive));
      simBtn.textContent = simLive ? "Pause Preview" : "Play Preview";
    }
    reqR();
  }

  const apCam = (r) => {
    if (r?.camera) { sEye = r.camera.eye; sTgt = r.camera.lookTarget; lClr = r.camera.clearanceMm ?? lClr; }
    if (r?.tris) lTris = r.tris;
  };

  async function cycCam(forcePreset = null) {
    const order = ["landmark", "third", "first"];
    camPreset = forcePreset || order[(order.indexOf(camPreset) + 1) % 3]; fMov = false;
    syncPossessedUi();
    const spec = camPreset === "landmark"
      ? ["Camera: Orbit", "camera-focus-landmark", { target: [2.2, 3.8, 9.2], dist: 18.5, yawMilli: 460, pitchMilli: 260 }, "Orbit Camera · Near-Sphere Guarded (>=290mm)"]
      : camPreset === "third"
        ? ["Camera: 3rd IK", "camera-set-mode", { mode: 0, simClockSec: sClk }, "3rd-Person Character Rig · WASD + Foot IK"]
        : ["Camera: 1st Eye", "camera-set-mode", { mode: 1, simClockSec: sClk }, "1st-Person Eye Camera · Head Masked"];
    if (camBtn) camBtn.textContent = spec[0];
    apCam(await cW(spec[1], spec[2])); chip(statusEl, spec[3], "good");
    await syncFb(); reqR();
  }

  async function doStory() {
    const r = await cW("story-interact");
    if (!r?.ok) return null;
    chip(statusEl, r.advanced ? `StoryAutomaton -> ${r.title} · Conserved ${r.supply}` : `${r.title} · ${r.objective}`, r.advanced ? "good" : "wait");
    return r;
  }

  const qVer = async (q) => { try { return app ? show(await app.dispatch(parse(q))) : null; } catch { return null; } };

  async function syncB(query = LODS[lodIdx].key, customPrefix = null) {
    if (!app) { chip(statusEl, reason ? "Verifier unavailable" : "Runtime offline", "bad"); return; }
    const res = await qVer(query);
    if (!res) return;
    if (res.startsWith("conformant:")) {
      const lod = LODS[lodIdx];
      chip(statusEl, customPrefix ? `${customPrefix} · ${query} Conformant` : `Runtime Conformant · ${lod.label} · 15 Semirings`, "good");
    } else chip(statusEl, res.startsWith("refused:") ? res.replace("refused:", "Refused: ") : res, res.startsWith("refused:") ? "warn" : "bad");
  }

  async function syncFb() {
    if (offOk || !fbR || !eW) return;
    const r = await cW("get-packed-buffers", { simClockSec: sClk });
    if (r?.ok) { fbR.uploadPackedBuffers(r.pos, r.norm, r.col, r.idx); lTris = r.tris || 0; apCam(r); }
  }

  async function togRnd(force = null) {
    renderMode = force || (renderMode === "webgl2" ? "tile" : "webgl2");
    if (renderBtn) renderBtn.textContent = renderMode === "tile" ? "Raster: Wasm Tile" : "Raster: WebGL2";
    const r = await cW("set-render-mode", { mode: renderMode, simClockSec: sClk });
    syncHudTitle();
    if (hudStats) hudStats.textContent = hudStr("");
    chip(statusEl, renderMode === "tile" ? `Wasm 16x16 Software Tile (${r?.tris ?? lTris} tris)` : `WebGL2 3-Pass PBR (${r?.tris ?? lTris} tris)`, "good");
    reqR();
  }

  async function impGlb(fileName, rawBytes) {
    chip(statusEl, `Importing ${fileName}…`, "wait");
    const copy = new Uint8Array(rawBytes);
    const r = await cW("import-glb", { bytes: copy }, [copy.buffer]);
    if (!r?.ok) { chip(statusEl, `Refused invalid .glb (${fileName})`, "warn"); return; }
    lTris = r.tris || lTris;
    const mod = await getStudio();
    let opfsSaved = false;
    if (r.uorcBytes?.byteLength && isOrganAllowed(4, true)) {
      const cRes = await mod.cacheBytesInOpfs(`${fileName}.uorc`, r.uorcBytes);
      opfsSaved = Boolean(cRes?.cached);
    }
    await syncFb();
    syncHudTitle(`Asset ${fileName} (${r.tris} tris)`);
    chip(statusEl, `Imported ${fileName} · ${r.tris} tris${opfsSaved ? " · OPFS cached" : ""}`, "good");
    reqR();
  }

  async function mountOpfsUorc(name, bytes) {
    if (!isOrganAllowed(4, true)) { refuseOrgan(4); return false; }
    const targetRank = Math.min(3, Math.max(2, lodIdx));
    const lod = LODS[targetRank];
    const r = await cW("stream-uorc", { targetRank, customStream: bytes, lod, simClockSec: sClk });
    if (r?.ok) {
      lodIdx = targetRank;
      if (lodBtn) lodBtn.textContent = `Fidelity: ${lod.short}`;
      syncHudTitle(`OPFS ${name} (${r.tris} tris)`);
      lTris = r.tris || lTris;
      await syncFb(); reqR();
      chip(statusEl, `Mounted OPFS ${name} · ${r.tris} tris`, "good");
      return true;
    }
    chip(statusEl, `Refused invalid .uorc (${name})`, "warn");
    return false;
  }

  async function rstGlb() {
    const r = await cW("reset-glb");
    if (r?.tris) lTris = r.tris;
    syncHudTitle();
    await syncFb();
    await syncB("summary");
    reqR();
  }

  const toggleGuardedAudio = async () => {
    if (!isOrganAllowed(3, true)) { refuseOrgan(3); return; }
    await (await getStudio()).toggleModalAudio(audioBtn, ACOUST, () => sEye);
  };

  const openStitchGuarded = async () =>
    (await import("./stitch-mesh.mjs")).openStitchMeshModal({
      callWorker: cW,
      queryVerifier: qVer,
      isOrganAllowed,
      onRefuseOrgan: refuseOrgan,
      onRestoreSnapshot: rstSnap,
    });

  const execP = async (text) =>
    (await getStudio()).routeStudioPrompt(text, {
      callWorker: cW, queryVerifier: qVer, chip, statusEl, streamUorcToRank: strUorc, cycleCameraMode: cycCam,
      interactChronicle: doStory, focusNodePreset: focNd, setSimLive: setSim, toggleRenderBackend: togRnd,
      restoreRawSnapshot: rstSnap, launchStudioModal: openSt, openStitchModal: openStitchGuarded,
    });

  async function focPt(target, dist = 12, label = "Entity", yawMilli = 520, pitchMilli = 300) {
    camPreset = "landmark"; fMov = false; syncPossessedUi();
    if (camBtn) camBtn.textContent = "Camera: Orbit";
    apCam(await cW("camera-focus-landmark", { target, dist, yawMilli, pitchMilli, simClockSec: sClk }));
    await syncFb(); reqR();
    chip(statusEl, `Framed ${label} · Guarded Rig`, "good");
  }

  async function setWorldSeed(seed) {
    const r = await cW("set-seed", { seed });
    if (r?.ok) {
      apCam(r); await syncFb(); reqR();
      chip(statusEl, `Runtime Seed ${r.seed} · ${LODS[lodIdx].label}`, "good");
    }
    return r;
  }

  async function openSt(initialTab = null) {
    if (typeof initialTab === "string") syncRoleHighlight(initialTab);
    const mod = await getStudio();
    mod.openStudioModal({
      initialTab: typeof initialTab === "string" ? initialTab : activeRoleKey,
      callWorker: cW, queryVerifier: qVer,
      onPickGlb: () => glbIn?.click(), onResetGlb: rstGlb,
      onExportGlb: () => mod.exportGlbDownload(cW, LODS[lodIdx].short),
      onShareSnapshot: () => mod.shareUorcSnapshot(cW, chip, statusEl),
      onToggleRenderMode: togRnd, onSetLod: strUorc,
      onSetCameraMode: cycCam, onSetSimLive: setSim,
      onToggleAudio: toggleGuardedAudio,
      onFocusPoint: focPt, onFocusPreset: focNd, onSetSeed: setWorldSeed,
      onOpenStitchMesh: openStitchGuarded,
      onRoutePrompt: execP, onInteractStory: doStory,
      onMountOpfsUorc: mountOpfsUorc,
      onRoleChange: (rk) => syncRoleHighlight(rk),
      onProjectChange: (projTitle) => {
        if (projTitle) { activeProject = projTitle; syncHudTitle(); }
      },
      getActiveProject: () => activeProject,
      getAcousticSpec: () => ACOUST,
      isOrganAllowed,
      getOrganMode: (idx) => organModes[idx] ?? 0,
      onOrganGrantChange: (idx, nextMode) => {
        organModes[idx] = nextMode;
        if (idx === 3 && nextMode === 2) stM?.suspendModalAudio?.(audioBtn);
      },
      onRefuseOrgan: refuseOrgan,
      getRenderMode: () => renderMode, getLodRank: () => lodIdx,
    });
  }

  async function setLod(nextIdx, updateBanner = true) {
    lodIdx = ((nextIdx % LODS.length) + LODS.length) % LODS.length;
    const lod = LODS[lodIdx];
    if (lodBtn) lodBtn.textContent = `Fidelity: ${lod.short}`;
    syncHudTitle();
    if (lod.rank < 2) fMov = false;
    if (eW) { eW.postMessage({ kind: "set-lod", lod, simClockSec: sClk }); await syncFb(); }
    reqR(); if (updateBanner) syncB(lod.key);
  }

  async function focNd(idx, customTitle = null, customQuery = null) {
    const node = NODES[idx];
    if (!node) return;
    aSlot = Math.min(6, node.id); gazeMs = 1400;
    if (lodIdx < 3) {
      lodIdx = 3;
      if (lodBtn) lodBtn.textContent = `Fidelity: ${LODS[lodIdx].short}`;
      syncHudTitle();
    }
    fMov = Boolean(node.followMover);
    if (fMov || node.id === 3) setSim(true);
    camPreset = node.possessExplorer ? "third" : "landmark";
    syncPossessedUi();
    if (camBtn) camBtn.textContent = node.possessExplorer ? "Camera: 3rd IK" : "Camera: Orbit";
    apCam(await cW("camera-focus-landmark", {
      possessExplorer: Boolean(node.possessExplorer), target: node.target, dist: node.dist,
      yawMilli: node.yawMilli ?? 620, pitchMilli: node.pitchMilli ?? 350, lod: LODS[lodIdx], simClockSec: sClk,
    }));
    if (hudStats) hudStats.textContent = hudStr("");
    await syncFb(); if (isAnim() || !offOk) reqR();
    await syncB(customQuery || node.query, customTitle || node.name);
  }

  async function strUorc(targetRank) {
    const clamped = Math.max(0, Math.min(LODS.length - 1, targetRank | 0));
    lodIdx = clamped;
    const lod = LODS[lodIdx];
    if (lodBtn) lodBtn.textContent = `Fidelity: ${lod.short}`;
    if (uorcBtn) uorcBtn.textContent = clamped >= 4 ? "Stream LOD0 Proxy" : "Stream LOD4 HD";
    syncHudTitle();
    if (lod.rank < 2) fMov = false;
    chip(statusEl, `Streaming .uorc -> ${lod.label}…`, "wait");
    if (eW) {
      const r = await cW("stream-uorc", { targetRank: clamped, lod, simClockSec: sClk });
      if (r?.tris) lTris = r.tris;
      await syncFb();
      reqR();
      if (r?.ok) { chip(statusEl, `Streamed ${lod.label} (${lTris} tris · Σ=${r.codedChecksum})`, "good"); return r; }
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
  onClick(meshBtn, openStitchGuarded);
  onClick(simBtn, () => setSim(!simLive));
  onClick(camBtn, () => cycCam());
  promptForm?.addEventListener("submit", async (e) => { e.preventDefault(); await execP(promptInput?.value); if (promptInput) promptInput.value = ""; });
  vpEl?.addEventListener("dragover", (e) => { e.preventDefault(); vpEl.classList.add("drag-over"); });
  vpEl?.addEventListener("dragleave", () => vpEl.classList.remove("drag-over"));
  vpEl?.addEventListener("drop", async (e) => { e.preventDefault(); vpEl.classList.remove("drag-over"); const f = e.dataTransfer?.files?.[0]; if (f) await impGlb(f.name, await f.arrayBuffer()); });
  onClick(audioBtn, toggleGuardedAudio);
  onClick(glbBtn, async () => (await getStudio()).exportGlbDownload(cW, LODS[lodIdx].short));
  onClick(snapBtn, async () => (await getStudio()).shareUorcSnapshot(cW, chip, statusEl));
  onClick(interactBtn, () => doStory());
  onClick(lockBtn, () => {
    if (document.pointerLockElement === canvas) document.exitPointerLock?.();
    else canvas?.requestPointerLock?.();
  });
  document.addEventListener("pointerlockchange", () => {
    const locked = document.pointerLockElement === canvas;
    lockBtn?.classList.toggle("active", locked);
    if (lockBtn) lockBtn.textContent = locked ? "Unlock" : "Look";
  });
  touchPad?.querySelectorAll("button[data-move]").forEach((btn) => {
    const code = btn.dataset.move;
    const press = (e) => {
      e.preventDefault();
      keys.add(code);
      btn.classList.add("active");
      if (camPreset === "landmark") cycCam("third");
      else reqR();
    };
    const release = () => { keys.delete(code); btn.classList.remove("active"); };
    btn.addEventListener("pointerdown", press);
    btn.addEventListener("pointerup", release);
    btn.addEventListener("pointercancel", release);
    btn.addEventListener("pointerleave", release);
  });

  async function rstSnap(bytes, label = "UORC") {
    const r = await cW("restore-snapshot", { bytes });
    if (r?.ok) { await setLod(r.lodRank ?? 3, false); chip(statusEl, `Restored ${label} · ${LODS[lodIdx].label} (${r.byteLen}B)`, "good"); return true; }
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
    const locked = document.pointerLockElement === canvas;
    if (!dragging && !locked) return;
    const dx = locked ? (e.movementX || 0) : e.clientX - lx;
    const dy = locked ? (e.movementY || 0) : e.clientY - ly;
    lx = e.clientX; ly = e.clientY; gazeMs = Math.max(80, gazeMs - 40);
    pYaw -= dx * 0.007; pPit += camPreset === "first" ? -dy * 0.0055 : dy * 0.006; reqR();
  });
  onC("pointerup", () => { dragging = false; });
  onC("pointercancel", () => { dragging = false; });
  onC("wheel", (e) => { e.preventDefault(); pDst += e.deltaY * 0.018; reqR(); }, { passive: false });
  addEventListener("deviceorientation", (e) => {
    if (!isOrganAllowed(6) || e.gamma == null || e.beta == null) return;
    pYaw -= Math.max(-1, Math.min(1, e.gamma / 45)) * 0.012;
    reqR();
  });
  addEventListener("keydown", (e) => {
    if (/^(INPUT|TEXTAREA|SELECT)$/.test(document.activeElement?.tagName ?? "")) return;
    if (e.code === "KeyE" && !e.repeat) { doStory(); return; }
    if (e.code === "KeyG" && !e.repeat) { openSt("code"); return; }
    if (e.code === "Space") jumpQueued = true;
    keys.add(e.code);
    if (hasMv() && camPreset === "landmark") {
      cycCam("third");
      return;
    }
    reqR();
  });
  addEventListener("keyup", (e) => keys.delete(e.code));
  addEventListener("blur", () => keys.clear());
  addEventListener("resize", reqR);
  document.addEventListener("visibilitychange", () => {
    if (document.hidden) keys.clear();
    else { prevNow = performance.now(); reqR(); }
  });
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
      workerFramesInFlight++; lSent = now; lSentW = w; lSentH = h;
      eW.postMessage({
        kind: "frame", width: w, height: h, dt, stepPhysics: stepping, followMover: fMov,
        fwd: intent.fwd, strafe: intent.strafe, speedScale: intent.speedScale, jump: intent.jump,
        deltaYaw: intent.dYaw, deltaPitch: intent.dPitch, deltaDist: intent.dDist,
        gazedSlot: aSlot, dwellMs: gazeMs, angSpeedMilli: dragging ? 600 : 60, lod, simClockSec: sClk,
      });
    } else if (fbR && fbM && canvas) {
      if (canvas.width !== w || canvas.height !== h) { canvas.width = w; canvas.height = h; }
      const proj = fbM.mat4Perspective(Math.PI / 3.4, Math.max(0.2, w / h), 0.25, 140.0);
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

  const stratumBadges = [];
  const el = (t, c, x) => { const n = document.createElement(t); if (c) n.className = c; if (x) n.textContent = x; return n; };
  if (tele) {
    tele.replaceChildren();
    for (const stratum of RUNTIME_STRATA) {
      const li = el("li"), b = el("button", "again"), bg = el("span", "ve-stratum-sub", stratum.tag);
      b.type = "button";
      b.id = `ve-stratum-${stratum.key}-btn`;
      b.dataset.stratumKey = stratum.key;
      b.setAttribute("aria-current", stratum.key === activeStratumKey ? "true" : "false");
      b.title = `Frame ${stratum.label} in the 3D stage and verify ${stratum.query}`;
      b.append(el("span", 0, stratum.label), bg);
      b.onclick = async () => {
        syncStratumHighlight(stratum.key);
        activeRoleKey = stratum.roleKey;
        await focNd(stratum.nodeIdx, stratum.label, stratum.query);
      };
      li.append(b);
      tele.append(li);
      stratumButtons.set(stratum.key, b);
      stratumBadges.push({ stratum, badge: bg, btn: b });
    }
  }

  const apInit = (r) => {
    if (!r) return;
    apCam(r);
    if (r.audioSpec) Object.assign(ACOUST, r.audioSpec);
  };
  const iD = pxDims(); let cXfer = false;
  if (eW && canvas && typeof canvas.transferControlToOffscreen === "function") {
    try {
      const oc = canvas.transferControlToOffscreen(); cXfer = true;
      const r = await cW("init", { canvas: oc, width: iD.w, height: iD.h, lod: LODS[lodIdx] }, [oc]);
      if (r?.kind === "ready") { offOk = true; lSentW = iD.w; lSentH = iD.h; isSw = Boolean(r.isSoftware); apInit(r); if (hudStats) hudStats.textContent = hudStr(""); }
    } catch { offOk = false; }
  }
  if (!offOk && !cXfer && canvas) {
    fbM = await import("./scene-renderer.mjs"); fbR = await fbM.createBestRenderer(canvas);
    isSw = Boolean(fbR?.isSoftware);
    if (eW) {
      apInit(await cW("init", { canvas: null, width: iD.w, height: iD.h, lod: LODS[lodIdx] }));
      await syncFb();
    }
  }

  syncHudTitle();
  const bootSnap = await rstHash();
  if (location.hash.startsWith("#studio=")) openSt(location.hash.slice(8).trim());
  if (!bootSnap) chip(statusEl, `Runtime Conformant · ${LODS[lodIdx].label} · 15 Semirings`, "good");
  reqR();
  (async () => {
    if (!app) { if (!bootSnap) chip(statusEl, reason ? "Verifier unavailable" : "Runtime offline", "bad"); return; }
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
    for (const { stratum, badge, btn } of stratumBadges) {
      await yieldTask();
      const sf = pConf(await qVer(stratum.query));
      if (!sf) continue;
      badge.className = "ve-stratum-sub";
      badge.textContent = stratum.tag;
      btn.dataset.holoConformant = "true";
      btn.title = `${stratum.label} (${stratum.tag}) — ${stratum.query} verified`;
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
