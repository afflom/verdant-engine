// Verdant Engine — Browser 3D Studio Controller (OffscreenCanvas Worker + AudioWorklet + VDNT).
// Pure ES module, zero inline style mutations (strict hash-locked CSP). Delegates simulation,
// SAH BVH culling, universal WebGL2 / software tensor rendering, .glb packing, and SHA-256 VDNT
// WorldSnapshot verification to site/engine-worker.mjs + site/verdant-wasm.wasm, and spatial
// modal synthesis to site/audio-worklet.mjs.

import {
  PRISM_AUDIO_QUANTUM_FRAMES,
  PRISM_AUDIO_STEREO_CHANNELS,
  PRISM_CHAMBER_SABINE_RT60_MS,
  PRISM_MODAL_BAR_DENOM,
  PRISM_MODAL_BAR_NUMERS,
} from "./prism-bridge-meta.mjs";

const STREAMING_LODS = [
  {
    rank: 0,
    key: "lod:0",
    short: "LOD0",
    label: "LOD0 · Coarse Proxy (0.125)",
    res: 0.125,
    density: 0.35,
    shadows: false,
    ibl: false,
    lossless: false,
    sky: [0.09, 0.08, 0.12],
    fog: [0.14, 0.11, 0.16],
  },
  {
    rank: 1,
    key: "lod:1",
    short: "LOD1",
    label: "LOD1 · Medium Detail (0.25)",
    res: 0.25,
    density: 0.72,
    shadows: true,
    ibl: false,
    lossless: false,
    sky: [0.12, 0.17, 0.24],
    fog: [0.18, 0.24, 0.31],
  },
  {
    rank: 2,
    key: "lod:2",
    short: "LOD2",
    label: "LOD2 · High Detail (0.625)",
    res: 0.625,
    density: 0.92,
    shadows: true,
    ibl: true,
    lossless: false,
    sky: [0.16, 0.27, 0.34],
    fog: [0.24, 0.35, 0.39],
  },
  {
    rank: 3,
    key: "lod:3",
    short: "LOD3",
    label: "LOD3 · Lossless UORC (1.00)",
    res: 1.0,
    density: 1.0,
    shadows: true,
    ibl: true,
    lossless: true,
    sky: [0.18, 0.30, 0.38],
    fog: [0.30, 0.40, 0.44],
  },
];

const NODE_KERNELS = [
  { id: 0, query: "kernel:0", sceneQuery: "scene:node:0", name: "Central Arch Pavilion", semiring: "LinearExact", target: [0, 2.2, 0], dist: 16 },
  { id: 1, query: "kernel:1", sceneQuery: "scene:node:1", name: "Axial Marker Tower", semiring: "LinearExact", target: [9.5, 1.2, -3.5], dist: 9 },
  { id: 2, query: "kernel:2", sceneQuery: "scene:node:2", name: "Modal Acoustics Frame", semiring: "TropicalMaxPlus", target: [-9.0, 1.8, -4.0], dist: 10 },
  { id: 3, query: "kernel:3", sceneQuery: "scene:node:3", name: "Articulated Rig Bay", semiring: "LinearExact", target: [-6.5, 1.8, 9.0], dist: 10 },
  { id: 4, query: "kernel:4", sceneQuery: "scene:node:4", name: "Elevated Deck Bridge", semiring: "LinearExact", target: [5.5, 0.6, 8.5], dist: 11 },
  { id: 5, query: "kernel:5", sceneQuery: "scene:node:5", name: "Navigable Path Grid", semiring: "ModularAudioRing", target: [0, 0.8, 5.0], dist: 18 },
  { id: 6, query: "kernel:6", sceneQuery: "scene:node:6", name: "Kinematic Spline Mover", semiring: "LinearExact", target: [18, 1.5, 0], dist: 11, followMover: true },
];

const ACOUSTIC_SPEC = {
  fundamentalHz: 110.0,
  modeNumers: [...PRISM_MODAL_BAR_NUMERS],
  modeDenom: PRISM_MODAL_BAR_DENOM,
  rt60Ms: PRISM_CHAMBER_SABINE_RT60_MS,
};

function terrainHeight(x, z, amp = 1.0) {
  const r = Math.hypot(x, z);
  const basin = -1.35 * Math.exp(-((x - 5.5) ** 2 + (z - 9.0) ** 2) / 52.0);
  const plateau = 0.55 * Math.exp(-(r * r) / 70.0);
  const rim = 2.6 * Math.max(0, (r - 17.5) / 14.0) ** 1.45;
  const ridge =
    0.42 * Math.sin(x * 0.19 + 0.4) * Math.cos(z * 0.17 - 0.2) +
    0.21 * Math.sin((x + z) * 0.33);
  return (basin + plateau + rim + ridge) * amp;
}

function bytesToBase64Url(bytes) {
  let bin = "";
  for (let i = 0; i < bytes.byteLength; i++) bin += String.fromCharCode(bytes[i]);
  return btoa(bin).replace(/\+/g, "-").replace(/\//g, "_").replace(/=+$/g, "");
}

function base64UrlToBytes(raw) {
  if (!raw || /[^A-Za-z0-9\-_]/.test(raw)) return null;
  try {
    const padded = raw.replace(/-/g, "+").replace(/_/g, "/") + "===".slice((raw.length + 3) % 4);
    const bin = atob(padded);
    const out = new Uint8Array(bin.length);
    for (let i = 0; i < bin.length; i++) out[i] = bin.charCodeAt(i);
    return out;
  } catch {
    return null;
  }
}

function parseConformantFields(line) {
  if (!line || !line.startsWith("conformant:")) return null;
  const fields = {};
  for (const part of line.slice("conformant:".length).split(",")) {
    const eq = part.indexOf("=");
    if (eq > 0) fields[part.slice(0, eq)] = part.slice(eq + 1);
  }
  return fields;
}

function parseRatioPair(raw, fallback) {
  if (!raw) return fallback;
  const [n, d] = raw.split("/").map(Number);
  return Number.isFinite(n) && Number.isFinite(d) && d !== 0 ? n / d : fallback;
}

function parseTriplet(raw, scale, fallback) {
  if (!raw) return fallback;
  const parts = raw.split("/").map(Number);
  return parts.length === 3 && parts.every(Number.isFinite) ? parts.map((v) => v / scale) : fallback;
}

const yieldTask = () => new Promise((resolve) => setTimeout(resolve, 0));

export default async function start({ app, reason, $, chip, show, parse }) {
  const canvas = $("ve-canvas");
  const statusEl = $("ve-status");
  const lodBtn = $("ve-lod-btn");
  const uorcBtn = $("ve-uorc-btn");
  const vizBtn = $("ve-viz-btn");
  const simBtn = $("ve-sim-btn");
  const camBtn = $("ve-cam-btn");
  const audioBtn = $("ve-audio-btn");
  const glbBtn = $("ve-glb-btn");
  const snapBtn = $("ve-snap-btn");
  const hudLod = $("ve-hud-lod");
  const hudStats = $("ve-hud-stats");
  const telemetryList = $("ve-telemetry");

  let lodIdx = 3;
  let simLive = false;
  let camMode = "orbit";
  let followMover = false;
  let orbitYaw = 0.68;
  let orbitPitch = 0.42;
  let orbitDist = 24.0;
  let target = [0, 1.8, 1.5];
  let walkPos = [0, 2.2, 13.5];
  let walkYaw = Math.PI;
  let walkPitch = -0.08;
  let dragging = false;
  let rafId = 0;
  const keys = new Set();

  let currentEye = [16, 11, 18];
  let lastMoverPos = [16.5, 0.5, 0];
  let lastTris = 0;
  let lastBvhVis = 47;
  let lastKernelCount = 9;
  let workerBackend = "webgl2";
  let isSoftwareGpu = false;
  let snapshotHandledOnBoot = false;

  let engineWorker = null;
  let offscreenActive = false;
  let nextReqId = 1;
  const pendingCalls = new Map();

  let audioCtx = null;
  let workletNode = null;
  let audioTimer = 0;
  let audioOn = false;
  let modalStrikeIndex = 0;

  let fallbackRenderer = null;
  let fallbackMod = null;
  let fallbackStatic = null;
  let fallbackDyn = null;

  function computeCameraPose(lod, dt) {
    if (camMode === "orbit") {
      if (simLive && !dragging && !followMover) orbitYaw += dt * 0.08;
      if (followMover && lod.rank >= 2) target = [...lastMoverPos];
      const cp = Math.cos(orbitPitch), sp = Math.sin(orbitPitch);
      currentEye = [
        target[0] + Math.sin(orbitYaw) * cp * orbitDist,
        Math.max(terrainHeight(target[0], target[2], lod.density) + 0.8, target[1] + sp * orbitDist),
        target[2] + Math.cos(orbitYaw) * cp * orbitDist,
      ];
      return { eye: currentEye, lookTarget: target, fwd: 0, strafe: 0 };
    }
    const fwd = (keys.has("KeyW") || keys.has("ArrowUp") ? 1 : 0) - (keys.has("KeyS") || keys.has("ArrowDown") ? 1 : 0);
    const strafe = (keys.has("KeyD") || keys.has("ArrowRight") ? 1 : 0) - (keys.has("KeyA") || keys.has("ArrowLeft") ? 1 : 0);
    const speed = 7.5 * dt;
    walkPos[0] += (Math.sin(walkYaw) * fwd - Math.cos(walkYaw) * strafe) * speed;
    walkPos[2] += (Math.cos(walkYaw) * fwd + Math.sin(walkYaw) * strafe) * speed;
    walkPos[0] = Math.max(-25, Math.min(25, walkPos[0]));
    walkPos[2] = Math.max(-25, Math.min(25, walkPos[2]));
    walkPos[1] = Math.max(lod.rank >= 1 ? -0.1 : -2.0, terrainHeight(walkPos[0], walkPos[2], lod.density)) + 1.72;
    currentEye = [...walkPos];
    const lookTarget = [
      walkPos[0] + Math.sin(walkYaw) * Math.cos(walkPitch),
      walkPos[1] + Math.sin(walkPitch),
      walkPos[2] + Math.cos(walkYaw) * Math.cos(walkPitch),
    ];
    return { eye: currentEye, lookTarget, fwd, strafe };
  }

  function measureCanvasPixels() {
    if (!canvas) return { w: 1, h: 1 };
    const maxDpr = isSoftwareGpu ? 1 : 2;
    const dpr = Math.min(window.devicePixelRatio || 1, maxDpr);
    return {
      w: Math.max(1, Math.floor((canvas.clientWidth || 320) * dpr)),
      h: Math.max(1, Math.floor((canvas.clientHeight || 240) * dpr)),
    };
  }

  function callWorker(kind, payload = {}, transfer = []) {
    if (!engineWorker) return Promise.resolve(null);
    return new Promise((resolve) => {
      const id = nextReqId++;
      pendingCalls.set(id, resolve);
      engineWorker.postMessage({ kind, id, ...payload }, transfer);
    });
  }

  function startEngineWorker() {
    if (typeof Worker !== "function" || !canvas) return null;
    try {
      const w = new Worker("./engine-worker.mjs", { type: "module" });
      w.onmessage = ({ data: msg }) => {
        if (!msg) return;
        if (msg.id && pendingCalls.has(msg.id)) {
          const resolve = pendingCalls.get(msg.id);
          pendingCalls.delete(msg.id);
          resolve(msg);
          return;
        }
        if (msg.kind === "telemetry") {
          lastTris = msg.tris || lastTris;
          lastBvhVis = msg.bvhVisible ?? lastBvhVis;
          if (Array.isArray(msg.moverPos)) lastMoverPos = msg.moverPos;
          if (workletNode && audioOn) {
            workletNode.port.postMessage({
              kind: "spatial",
              pan: msg.binauralPan,
              gain: msg.binauralGain,
            });
          }
          if (!isAnimating() && hudStats) {
            const lod = STREAMING_LODS[lodIdx];
            hudStats.textContent = `${workerBackend.toUpperCase()} · ${lastTris} tris · ${lastBvhVis} vis · ${lod.lossless ? `${lastKernelCount} kernels` : "UORC stream"}`;
          }
        }
      };
      w.onerror = () => {
        for (const resolve of pendingCalls.values()) resolve(null);
        pendingCalls.clear();
      };
      return w;
    } catch {
      return null;
    }
  }

  engineWorker = startEngineWorker();

  function isAnimating() {
    return simLive || dragging || followMover || (camMode === "walk" && keys.size > 0);
  }

  function setSimLive(next) {
    simLive = Boolean(next);
    if (simBtn) {
      simBtn.setAttribute("aria-pressed", String(simLive));
      simBtn.textContent = simLive ? "Sim: Live" : "Sim: Step";
    }
    requestRender();
  }

  let prevNow = performance.now();
  let simClockSec = 0;
  let frames = 0;
  let lastFpsStamp = performance.now();
  let shownFps = 60;

  function requestRender() {
    if (rafId !== 0) return;
    prevNow = performance.now();
    rafId = requestAnimationFrame(frame);
  }

  async function queryVerifier(reqText) {
    if (!app) return null;
    try {
      return show(await app.dispatch(parse(reqText)));
    } catch {
      return null;
    }
  }

  async function syncVerifierBanner(query = STREAMING_LODS[lodIdx].key) {
    if (!app) {
      chip(statusEl, reason ? "Verifier unavailable" : "Engine offline", "bad");
      return;
    }
    const res = await queryVerifier(query);
    if (!res) return;
    if (res.startsWith("conformant:")) {
      const lod = STREAMING_LODS[lodIdx];
      chip(statusEl, `${lod.short} · ${lod.lossless ? `${lastKernelCount}/9 Exact Kernels` : "UORC Stream OK"}`, "good");
    } else if (res.startsWith("refused:")) {
      chip(statusEl, res.replace("refused:", "Refused: "), "warn");
    } else {
      chip(statusEl, res, "bad");
    }
  }

  function populateKernelsShell() {
    if (!telemetryList) return [];
    telemetryList.replaceChildren();
    const badges = [];
    for (const node of NODE_KERNELS) {
      const li = document.createElement("li");
      const btn = document.createElement("button");
      btn.type = "button";
      btn.className = "again";
      const label = document.createElement("span");
      label.textContent = node.name;
      const badge = document.createElement("span");
      badge.className = "tag";
      badge.textContent = node.semiring;
      btn.append(label, badge);
      btn.addEventListener("click", async () => {
        if (lodIdx !== 3) setLod(3, false);
        camMode = "orbit";
        camBtn.textContent = "Cam: Orbit";
        followMover = Boolean(node.followMover);
        if (followMover) setSimLive(true);
        target = [...node.target];
        orbitDist = node.dist;
        requestRender();
        await syncVerifierBanner(node.query);
      });
      li.append(btn);
      telemetryList.append(li);
      badges.push({ node, badge });
    }
    return badges;
  }

  function setLod(nextIdx, updateBanner = true) {
    lodIdx = ((nextIdx % STREAMING_LODS.length) + STREAMING_LODS.length) % STREAMING_LODS.length;
    const lod = STREAMING_LODS[lodIdx];
    if (lodBtn) lodBtn.textContent = `LOD: ${lod.short}`;
    if (hudLod) hudLod.textContent = lod.label;
    if (lod.rank < 2) followMover = false;
    const pose = computeCameraPose(lod, 0.016);
    if (offscreenActive && engineWorker) {
      engineWorker.postMessage({
        kind: "set-lod",
        lod,
        eye: pose.eye,
        lookTarget: pose.lookTarget,
        simClockSec,
      });
    } else if (fallbackMod && fallbackRenderer) {
      fallbackStatic = fallbackMod.buildStaticWorld(lod, isSoftwareGpu ? 18 : 36);
      fallbackRenderer.uploadStatic(fallbackStatic);
    }
    requestRender();
    if (updateBanner) syncVerifierBanner(lod.key);
  }

  async function focusNodePreset(nodeIndex) {
    const node = NODE_KERNELS[nodeIndex];
    if (!node) return;
    if (lodIdx !== 3) setLod(3, false);
    camMode = "orbit";
    if (camBtn) camBtn.textContent = "Cam: Orbit";
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
    chip(statusEl, `Streaming .uorc -> ${lod.short} in Wasm…`, "wait");
    if (engineWorker) {
      const reply = await callWorker("stream-uorc", {
        targetRank: clamped,
        lod,
        eye: pose.eye,
        lookTarget: pose.lookTarget,
        simClockSec,
      });
      if (!offscreenActive && fallbackMod && fallbackRenderer) {
        fallbackStatic = fallbackMod.buildStaticWorld(lod, isSoftwareGpu ? 18 : 36);
        fallbackRenderer.uploadStatic(fallbackStatic);
      }
      requestRender();
      if (reply?.ok) {
        chip(
          statusEl,
          `UORC Wasm · ${lod.short} (${reply.streamBytes}B · coded Σ=${reply.codedChecksum})`,
          "good",
        );
        return reply;
      }
    }
    setLod(clamped, true);
    return null;
  }

  lodBtn?.addEventListener("click", () => setLod(lodIdx + 1));
  uorcBtn?.addEventListener("click", () => {
    const nextRank = (lodIdx + 1) % STREAMING_LODS.length;
    streamUorcToRank(nextRank);
  });
  vizBtn?.addEventListener("click", async () => {
    const { openVisualsModal } = await import("./visuals.mjs");
    openVisualsModal({
      onFocusNode: focusNodePreset,
      onStreamUorc: streamUorcToRank,
    });
  });
  simBtn?.addEventListener("click", () => setSimLive(!simLive));

  camBtn?.addEventListener("click", () => {
    camMode = camMode === "orbit" ? "walk" : "orbit";
    camBtn.textContent = camMode === "orbit" ? "Cam: Orbit" : "Cam: Walk";
    requestRender();
  });

  function strikeFallbackOscillator() {
    if (!audioCtx || !audioOn) return;
    const dist = Math.hypot(currentEye[0] + 9.0, currentEye[1] - 2.0, currentEye[2] + 4.0);
    const atten = Math.min(0.14, 0.9 / (1.0 + 0.18 * dist));
    const idx = (modalStrikeIndex++) % ACOUSTIC_SPEC.modeNumers.length;
    const ratio = ACOUSTIC_SPEC.modeNumers[idx] / ACOUSTIC_SPEC.modeDenom;
    const baseFreq = Math.min(1320.0, ACOUSTIC_SPEC.fundamentalHz * ratio);
    const rt60Sec = Math.max(0.5, ACOUSTIC_SPEC.rt60Ms / 1000.0);
    const now = audioCtx.currentTime;
    const osc = audioCtx.createOscillator();
    const gain = audioCtx.createGain();
    osc.type = "sine";
    osc.frequency.setValueAtTime(baseFreq, now);
    gain.gain.setValueAtTime(0.0001, now);
    gain.gain.exponentialRampToValueAtTime(atten, now + 0.02);
    gain.gain.exponentialRampToValueAtTime(0.0001, now + rt60Sec);
    osc.connect(gain).connect(audioCtx.destination);
    osc.start(now);
    osc.stop(now + rt60Sec + 0.05);
  }

  function triggerModalStrike() {
    if (!audioCtx || !audioOn) return;
    if (workletNode) {
      workletNode.port.postMessage({ kind: "strike", modeIndex: modalStrikeIndex++ });
    } else {
      strikeFallbackOscillator();
    }
  }

  audioBtn?.addEventListener("click", async () => {
    const Ctx = window.AudioContext || window.webkitAudioContext;
    if (!Ctx) return;
    audioOn = !audioOn;
    audioBtn.setAttribute("aria-pressed", String(audioOn));
    audioBtn.textContent = audioOn ? "Audio: On" : "Audio: Off";
    if (audioOn) {
      audioCtx ??= new Ctx();
      await audioCtx.resume?.();
      if (!workletNode && audioCtx.audioWorklet && typeof AudioWorkletNode === "function") {
        try {
          await audioCtx.audioWorklet.addModule("./audio-worklet.mjs");
          workletNode = new AudioWorkletNode(audioCtx, "verdant-modal-processor", {
            numberOfOutputs: 1,
            outputChannelCount: [PRISM_AUDIO_STEREO_CHANNELS],
            processorOptions: {
              quantumFrames: PRISM_AUDIO_QUANTUM_FRAMES,
              modeNumers: ACOUSTIC_SPEC.modeNumers,
              modeDenom: ACOUSTIC_SPEC.modeDenom,
              rt60Ms: ACOUSTIC_SPEC.rt60Ms,
              fundamentalHz: ACOUSTIC_SPEC.fundamentalHz,
            },
          });
          workletNode.connect(audioCtx.destination);
        } catch {
          workletNode = null;
        }
      }
      clearInterval(audioTimer);
      audioTimer = window.setInterval(triggerModalStrike, 1350);
      triggerModalStrike();
    } else {
      clearInterval(audioTimer);
    }
  });

  glbBtn?.addEventListener("click", async () => {
    let glbBuffer = null;
    if (offscreenActive && engineWorker) {
      const reply = await callWorker("export-glb");
      glbBuffer = reply?.buffer ?? null;
    } else if (fallbackMod && fallbackStatic && fallbackDyn) {
      glbBuffer = fallbackMod.buildGlbArrayBuffer(fallbackStatic, fallbackDyn.mb);
    }
    if (!glbBuffer) return;
    const blob = new Blob([glbBuffer], { type: "model/gltf-binary" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = `verdant-engine-${STREAMING_LODS[lodIdx].short.toLowerCase()}.glb`;
    a.click();
    setTimeout(() => URL.revokeObjectURL(url), 1500);
  });

  async function restoreHashSnapshotIfPresent() {
    if (!location.hash.startsWith("#vdnt=")) return false;
    const encoded = location.hash.slice("#vdnt=".length).trim();
    const bytes = base64UrlToBytes(encoded);
    if (!bytes || bytes.byteLength === 0) {
      chip(statusEl, "Refused malformed VDNT snapshot", "warn");
      return true;
    }
    const reply = await callWorker("restore-snapshot", { bytes });
    if (reply?.ok) {
      setLod(reply.lodRank ?? 3, false);
      chip(
        statusEl,
        `Restored VDNT · ${STREAMING_LODS[lodIdx].short} (${reply.byteLen}B SHA-256)`,
        "good",
      );
    } else {
      chip(statusEl, "Refused tampered VDNT snapshot", "warn");
    }
    return true;
  }

  snapBtn?.addEventListener("click", async () => {
    const reply = await callWorker("encode-snapshot");
    if (!reply?.ok || !reply.bytes) {
      chip(statusEl, "Snapshot unavailable", "warn");
      return;
    }
    const bytes = reply.bytes instanceof Uint8Array ? reply.bytes : new Uint8Array(reply.bytes);
    const b64 = bytesToBase64Url(bytes);
    history.replaceState(null, "", `${location.pathname}${location.search}#vdnt=${b64}`);
    try {
      await navigator.clipboard?.writeText(location.href);
    } catch {
      // Clipboard write requires user gesture in headless environments.
    }
    chip(statusEl, `VDNT · ${bytes.byteLength}B SHA-256 (#vdnt=)`, "good");
  });

  addEventListener("hashchange", () => {
    if (location.hash.startsWith("#vdnt=")) {
      restoreHashSnapshotIfPresent();
    }
  });

  let lastX = 0, lastY = 0;
  canvas?.addEventListener("pointerdown", (e) => {
    dragging = true;
    lastX = e.clientX;
    lastY = e.clientY;
    canvas.setPointerCapture?.(e.pointerId);
    requestRender();
  });
  canvas?.addEventListener("pointermove", (e) => {
    if (!dragging) return;
    const dx = e.clientX - lastX;
    const dy = e.clientY - lastY;
    lastX = e.clientX;
    lastY = e.clientY;
    if (camMode === "orbit") {
      orbitYaw -= dx * 0.0075;
      orbitPitch = Math.max(0.08, Math.min(1.38, orbitPitch + dy * 0.0065));
    } else {
      walkYaw -= dx * 0.0055;
      walkPitch = Math.max(-1.1, Math.min(1.1, walkPitch - dy * 0.0055));
    }
    requestRender();
  });
  const endDrag = () => { dragging = false; };
  canvas?.addEventListener("pointerup", endDrag);
  canvas?.addEventListener("pointercancel", endDrag);
  canvas?.addEventListener("wheel", (e) => {
    e.preventDefault();
    orbitDist = Math.max(5.0, Math.min(42.0, orbitDist + e.deltaY * 0.02));
    requestRender();
  }, { passive: false });

  addEventListener("keydown", (e) => {
    if (/^(INPUT|TEXTAREA|SELECT)$/.test(document.activeElement?.tagName ?? "")) return;
    keys.add(e.code);
    requestRender();
  });
  addEventListener("keyup", (e) => keys.delete(e.code));
  addEventListener("resize", requestRender);
  if (typeof ResizeObserver !== "undefined" && canvas?.parentElement) {
    new ResizeObserver(() => requestRender()).observe(canvas.parentElement);
  }

  function frame(now) {
    rafId = 0;
    const dt = Math.min(0.05, Math.max(0.001, (now - prevNow) * 0.001));
    prevNow = now;
    const stepping = simLive || followMover || (camMode === "walk" && keys.size > 0);
    if (simLive || followMover) simClockSec += dt;
    const lod = STREAMING_LODS[lodIdx];
    const { w, h } = measureCanvasPixels();
    const pose = computeCameraPose(lod, dt);

    if (offscreenActive && engineWorker) {
      engineWorker.postMessage({
        kind: "frame",
        width: w,
        height: h,
        dt,
        stepPhysics: stepping,
        fwd: pose.fwd,
        strafe: pose.strafe,
        walkYaw,
        eye: pose.eye,
        lookTarget: pose.lookTarget,
        lod,
        simClockSec,
      });
    } else if (fallbackRenderer && fallbackMod && fallbackStatic && canvas) {
      if (canvas.width !== w || canvas.height !== h) {
        canvas.width = w;
        canvas.height = h;
      }
      fallbackDyn = fallbackMod.buildDynamicActors(lod, simClockSec, null);
      fallbackRenderer.uploadDynamic(fallbackDyn.mb);
      const aspect = Math.max(0.2, w / h);
      const proj = fallbackMod.mat4Perspective(Math.PI / 3.4, aspect, 0.25, 140.0);
      const view = fallbackMod.mat4LookAt(pose.eye, pose.lookTarget, [0, 1, 0]);
      fallbackRenderer.render(fallbackMod.mat4Mul(proj, view), pose.eye, lod, simClockSec);
      lastTris = Math.round((fallbackStatic.idx.length + fallbackDyn.mb.idx.length) / 3);
      lastMoverPos = fallbackDyn.moverPos;
    }

    frames++;
    if (!isAnimating()) {
      if (hudStats && lastTris > 0) {
        hudStats.textContent = `${workerBackend.toUpperCase()} · ${lastTris} tris · ${lastBvhVis} vis · ${lod.lossless ? `${lastKernelCount} kernels` : "UORC stream"}`;
      }
    } else if (now - lastFpsStamp >= 600) {
      shownFps = Math.max(1, Math.round((frames * 1000) / (now - lastFpsStamp)));
      frames = 0;
      lastFpsStamp = now;
      if (hudStats) {
        hudStats.textContent = `${shownFps} fps · ${lastTris} tris · ${lastBvhVis} vis · ${lod.lossless ? `${lastKernelCount} kernels` : "UORC stream"}`;
      }
    }

    if (isAnimating()) {
      rafId = requestAnimationFrame(frame);
    }
  }

  async function hydrateSceneFromHolo(badges) {
    if (!app) return;
    for (let i = 0; i < STREAMING_LODS.length; i++) {
      await yieldTask();
      const fields = parseConformantFields(await queryVerifier(`scene:lod:${i}`));
      if (!fields) continue;
      const lod = STREAMING_LODS[i];
      lod.res = parseRatioPair(fields.res, lod.res);
      lod.density = parseRatioPair(fields.density, lod.density);
      lod.shadows = fields.shadows === "1";
      lod.ibl = fields.ibl === "1";
      lod.lossless = fields.lossless === "1";
      lod.sky = parseTriplet(fields.sky, 100, lod.sky);
      lod.fog = parseTriplet(fields.fog, 100, lod.fog);
    }
    for (const { node, badge } of badges) {
      await yieldTask();
      const sceneFields = parseConformantFields(await queryVerifier(node.sceneQuery));
      if (sceneFields) {
        node.target = parseTriplet(sceneFields.pos, 10, node.target);
        const dist = Number(sceneFields.dist);
        if (Number.isFinite(dist) && dist > 0) node.dist = dist;
        node.followMover = sceneFields.mover === "1";
      }
      const kernelFields = parseConformantFields(await queryVerifier(node.query));
      const ok = Boolean(kernelFields);
      if (kernelFields?.semiring) node.semiring = kernelFields.semiring;
      badge.className = ok ? "tag good" : "tag";
      badge.textContent = ok ? node.semiring.replace(/([a-z])([A-Z])/g, "$1 $2") : "Offline";
    }
    await yieldTask();
    const acFields = parseConformantFields(await queryVerifier("scene:acoustics"));
    if (acFields?.modes && acFields?.denom) {
      const denom = Number(acFields.denom) || 9;
      const nums = acFields.modes.split("/").map(Number).filter(Number.isFinite);
      if (nums.length > 0) {
        ACOUSTIC_SPEC.modeNumers = nums;
        ACOUSTIC_SPEC.modeDenom = denom;
      }
    }
    requestRender();
  }

  const badgeTasks = populateKernelsShell();
  const initialDims = measureCanvasPixels();
  const initialPose = computeCameraPose(STREAMING_LODS[lodIdx], 0.016);

  if (engineWorker && canvas && typeof canvas.transferControlToOffscreen === "function") {
    try {
      const offscreen = canvas.transferControlToOffscreen();
      const readyMsg = await callWorker(
        "init",
        {
          canvas: offscreen,
          width: initialDims.w,
          height: initialDims.h,
          lod: STREAMING_LODS[lodIdx],
          eye: initialPose.eye,
          lookTarget: initialPose.lookTarget,
        },
        [offscreen],
      );
      if (readyMsg && readyMsg.kind === "ready") {
        offscreenActive = true;
        workerBackend = readyMsg.backend || "webgl2";
        isSoftwareGpu = Boolean(readyMsg.isSoftware);
        lastTris = readyMsg.tris || 0;
        if (readyMsg.tensorKernels) lastKernelCount = readyMsg.tensorKernels;
      }
    } catch {
      offscreenActive = false;
    }
  }

  if (!offscreenActive && canvas) {
    fallbackMod = await import("./scene-renderer.mjs");
    fallbackRenderer = await fallbackMod.createBestRenderer(canvas);
    workerBackend = fallbackRenderer?.backend || "webgl2";
    isSoftwareGpu = Boolean(fallbackRenderer?.isSoftware);
    fallbackStatic = fallbackMod.buildStaticWorld(STREAMING_LODS[lodIdx], isSoftwareGpu ? 18 : 36);
    fallbackRenderer?.uploadStatic(fallbackStatic);
    if (engineWorker) {
      await callWorker("init", {
        canvas: null,
        width: initialDims.w,
        height: initialDims.h,
        lod: STREAMING_LODS[lodIdx],
        eye: initialPose.eye,
        lookTarget: initialPose.lookTarget,
      });
    }
  }

  snapshotHandledOnBoot = await restoreHashSnapshotIfPresent();
  requestRender();
  setTimeout(async () => {
    if (!snapshotHandledOnBoot) {
      await syncVerifierBanner("summary");
    }
    await hydrateSceneFromHolo(badgeTasks);
  }, 60);
}
