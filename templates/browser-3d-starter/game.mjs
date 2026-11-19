// Sunspire Beacon Trials — Standalone Playable 3D Coastal Action-Adventure Trial.
// Powered 100% by Prism-verified `/app.holo`, `/verdant-uorc.wasm` (22-joint LBS + Foot IK),
// `/scene-renderer.mjs`, and `/prism-bridge-meta.mjs` (zero handwritten JS terrain or physics bypasses).

import {
  PRISM_CHAMBER_SABINE_RT60_MS,
  PRISM_CSM_SPLIT_METERS,
  PRISM_DEFAULT_WORLD_SEED,
  PRISM_MAX_LOD_RANK,
  PRISM_MODAL_BAR_DENOM,
  PRISM_MODAL_BAR_NUMERS,
} from "../prism-bridge-meta.mjs";
import {
  createBestRenderer,
  mat4LookAt,
  mat4Mul,
  mat4Perspective,
} from "../scene-renderer.mjs";
import { load } from "../worker-runtime/index.mjs";

const CSM_FAR_METERS = PRISM_CSM_SPLIT_METERS[3];

const LOD3_SPEC = {
  rank: PRISM_MAX_LOD_RANK,
  sky: [0.18, 0.30, 0.38],
  fog: [0.30, 0.40, 0.44],
};

// 3 Relay Beacons placed across Citadel courtyard, Stone Arch Bridge, and North Terrace:
const BEACON_COORDS = [
  [-5.5, 4.5],
  [7.5, -4.0],
  [3.0, -7.5],
];

// Unit crystalline octahedron mesh for hardware-instanced Relay Beacons:
const OCTA_POS = new Float32Array([
  0, 1.35, 0,  0.65, 0, 0,  0, 0, 0.65,
  0, 1.35, 0,  0, 0, 0.65, -0.65, 0, 0,
  0, 1.35, 0, -0.65, 0, 0,  0, 0, -0.65,
  0, 1.35, 0,  0, 0, -0.65, 0.65, 0, 0,
  0, -0.95, 0, 0, 0, 0.65,  0.65, 0, 0,
  0, -0.95, 0, -0.65, 0, 0, 0, 0, 0.65,
  0, -0.95, 0, 0, 0, -0.65, -0.65, 0, 0,
  0, -0.95, 0, 0.65, 0, 0,  0, 0, -0.65,
]);
const OCTA_NORM = new Float32Array(OCTA_POS.length);
const OCTA_COL = new Float32Array(OCTA_POS.length);
const OCTA_IDX = new Uint32Array(24);
for (let t = 0; t < 8; t++) {
  const b = t * 9;
  const ux = OCTA_POS[b + 3] - OCTA_POS[b], uy = OCTA_POS[b + 4] - OCTA_POS[b + 1], uz = OCTA_POS[b + 5] - OCTA_POS[b + 2];
  const vx = OCTA_POS[b + 6] - OCTA_POS[b], vy = OCTA_POS[b + 7] - OCTA_POS[b + 1], vz = OCTA_POS[b + 8] - OCTA_POS[b + 2];
  let nx = uy * vz - uz * vy, ny = uz * vx - ux * vz, nz = ux * vy - uy * vx;
  const nl = Math.hypot(nx, ny, nz) || 1;
  nx /= nl; ny /= nl; nz /= nl;
  for (let k = 0; k < 3; k++) {
    OCTA_NORM[b + k * 3] = nx;
    OCTA_NORM[b + k * 3 + 1] = ny;
    OCTA_NORM[b + k * 3 + 2] = nz;
    // Material class 2.0 (metallic emissive crystal) encoded in integer part of R channel:
    OCTA_COL[b + k * 3] = 2.92;
    OCTA_COL[b + k * 3 + 1] = 0.88;
    OCTA_COL[b + k * 3 + 2] = 0.72;
    OCTA_IDX[t * 3 + k] = t * 3 + k;
  }
}

const canvas = document.getElementById("game-canvas");
const holoBadge = document.getElementById("holo-badge");
const hudBeacons = document.getElementById("hud-beacons");
const hudChronicle = document.getElementById("hud-chronicle");
const hudTime = document.getElementById("hud-time");
const hudSub = document.getElementById("hud-sub");
const btnStart = document.getElementById("start-btn");
const btnChronicle = document.getElementById("btn-chronicle");
const btnAudio = document.getElementById("btn-audio");
const btnCam = document.getElementById("btn-cam");
const btnPause = document.getElementById("btn-pause");
const btnResume = document.getElementById("btn-resume");
const btnRestart = document.getElementById("btn-restart");
const btnTitle = document.getElementById("btn-title");
const modalTitle = document.getElementById("modal-title");
const modalStats = document.getElementById("modal-stats");
const touchPad = document.getElementById("starter-touch-pad");

let activeWorldSeed = PRISM_DEFAULT_WORLD_SEED;
let activeWorldTitle = "Sunspire Beacon Trials";
let activeWorldSubtitle = "";
let wasm = null;
let renderer = null;
let gameState = "title";
let firstPerson = false;
let audioEnabled = false;
let audioCtx = null;
let beaconsHarmonized = [false, false, false];
let beaconWorldPositions = [];
let beaconEpoch = 1;
let runSeconds = 0;
let droneDeflections = 0;
let chronicleSupply = 12;
let chronicleTitle = "Arrival";
let emaFrameMicros = 8000;
let dragYaw = 0;
let dragPitch = 0;
let dragging = false;
let lastX = 0;
let lastY = 0;
let jumpQueued = false;
const keys = new Set();

function setGameState(next) {
  gameState = next;
  document.body.dataset.state = next;
}

function playModalChime(modeIdx, pan = 0) {
  if (!audioEnabled) return;
  const Ctx = window.AudioContext || window.webkitAudioContext;
  if (!Ctx) return;
  audioCtx ??= new Ctx();
  audioCtx.resume?.();
  const numer = PRISM_MODAL_BAR_NUMERS[Math.abs(modeIdx) % PRISM_MODAL_BAR_NUMERS.length];
  const freq = Math.min(1560, (132 * numer) / PRISM_MODAL_BAR_DENOM);
  const rtSec = Math.max(0.5, PRISM_CHAMBER_SABINE_RT60_MS / 1600);
  const now = audioCtx.currentTime;
  const osc = audioCtx.createOscillator();
  const gain = audioCtx.createGain();
  osc.type = "sine";
  osc.frequency.setValueAtTime(freq, now);
  gain.gain.setValueAtTime(0.0001, now);
  gain.gain.exponentialRampToValueAtTime(0.11, now + 0.018);
  gain.gain.exponentialRampToValueAtTime(0.0001, now + rtSec);
  if (typeof audioCtx.createStereoPanner === "function") {
    const panner = audioCtx.createStereoPanner();
    panner.pan.setValueAtTime(Math.max(-1, Math.min(1, pan)), now);
    osc.connect(gain).connect(panner).connect(audioCtx.destination);
  } else {
    osc.connect(gain).connect(audioCtx.destination);
  }
  osc.start(now);
  osc.stop(now + rtSec + 0.04);
}

function computeBeaconInstanceViews(spinRad) {
  const data = new Float32Array(BEACON_COORDS.length * 16);
  const c = Math.cos(spinRad), s = Math.sin(spinRad);
  for (let i = 0; i < BEACON_COORDS.length; i++) {
    const [bx, bz] = BEACON_COORDS[i];
    const by = beaconWorldPositions[i]?.[1] ?? 2.2;
    const bob = Math.sin(spinRad * 1.7 + i * 2.1) * 0.18;
    const lit = beaconsHarmonized[i];
    const cr = lit ? 0.32 : 0.98;
    const cg = lit ? 0.96 : 0.62;
    const cb = lit ? 0.66 : 0.22;
    const o = i * 16;
    data[o] = c;      data[o + 1] = 0; data[o + 2] = -s; data[o + 3] = cr;
    data[o + 4] = 0;  data[o + 5] = 1; data[o + 6] = 0;  data[o + 7] = cg;
    data[o + 8] = s;  data[o + 9] = 0; data[o + 10] = c; data[o + 11] = cb;
    data[o + 12] = bx; data[o + 13] = by + bob; data[o + 14] = bz; data[o + 15] = 1;
  }
  beaconEpoch = (beaconEpoch + 1) >>> 0;
  return {
    epoch: beaconEpoch,
    instCount: BEACON_COORDS.length,
    pos: OCTA_POS,
    norm: OCTA_NORM,
    col: OCTA_COL,
    idx: OCTA_IDX,
    data,
  };
}

function readWasmSplitViews() {
  const buf = wasm.memory.buffer;
  const sVerts = wasm.verdant_wasm_static_vert_count() >>> 0;
  const sIdx = wasm.verdant_wasm_static_idx_count() >>> 0;
  const staticViews = sVerts > 0 && sIdx > 0 ? {
    epoch: wasm.verdant_wasm_static_epoch() >>> 0,
    pos: new Float32Array(buf, wasm.verdant_wasm_scene_pos_ptr() >>> 0, sVerts * 3),
    norm: new Float32Array(buf, wasm.verdant_wasm_scene_norm_ptr() >>> 0, sVerts * 3),
    col: new Float32Array(buf, wasm.verdant_wasm_scene_col_ptr() >>> 0, sVerts * 3),
    idx: new Uint32Array(buf, wasm.verdant_wasm_scene_idx_ptr() >>> 0, sIdx),
  } : null;

  const dVerts = wasm.verdant_wasm_dyn_vert_count() >>> 0;
  const dIdx = wasm.verdant_wasm_dyn_idx_count() >>> 0;
  const dynViews = dVerts > 0 && dIdx > 0 ? {
    pos: new Float32Array(buf, wasm.verdant_wasm_dyn_pos_ptr() >>> 0, dVerts * 3),
    norm: new Float32Array(buf, wasm.verdant_wasm_dyn_norm_ptr() >>> 0, dVerts * 3),
    col: new Float32Array(buf, wasm.verdant_wasm_dyn_col_ptr() >>> 0, dVerts * 3),
    idx: new Uint32Array(buf, wasm.verdant_wasm_dyn_idx_ptr() >>> 0, dIdx),
  } : null;

  return { staticViews, dynViews };
}

function readChronicleStatus() {
  if (!wasm || typeof wasm.verdant_wasm_story_status_aux !== "function") return;
  const len = wasm.verdant_wasm_story_status_aux() >>> 0;
  const ptr = wasm.verdant_wasm_aux_ptr() >>> 0;
  if (len === 0 || ptr === 0) return;
  const raw = new TextDecoder().decode(new Uint8Array(wasm.memory.buffer, ptr, len));
  const [, title, supplyStr] = raw.split("|");
  if (title) chronicleTitle = title;
  const parsedSupply = Number(supplyStr);
  if (Number.isFinite(parsedSupply) && parsedSupply > 0) chronicleSupply = parsedSupply;
}

function stepChronicleAtPlayer() {
  if (!wasm || typeof wasm.verdant_wasm_story_interact_at !== "function") return;
  const px = wasm.verdant_wasm_player_x();
  const py = wasm.verdant_wasm_player_y() - 1.55;
  const pz = wasm.verdant_wasm_player_z();
  const advanced = wasm.verdant_wasm_story_interact_at(px, py, pz) === 1;
  readChronicleStatus();
  if (advanced) playModalChime(3, 0);
  syncHud();
}

function syncHud() {
  const litCount = beaconsHarmonized.filter(Boolean).length;
  if (hudBeacons) hudBeacons.textContent = `${litCount} / ${BEACON_COORDS.length}`;
  if (hudChronicle) hudChronicle.textContent = `${chronicleTitle} (${chronicleSupply})`;
  if (hudTime) hudTime.textContent = `${Math.floor(runSeconds)}s`;
  const clr = wasm ? (wasm.verdant_wasm_camera_clearance_milli() >>> 0) : 290;
  if (hudSub) hudSub.textContent = `View: ${firstPerson ? "1st Eye" : "3rd Person"} · ${clr}mm`;
  if (btnCam) btnCam.textContent = firstPerson ? "1st View" : "3rd View";
}

function resetArenaRun(enterPlaying = true) {
  if (enterPlaying) {
    document.activeElement?.blur?.();
    keys.clear();
    dragging = false;
    setGameState("playing");
  }
  if (!wasm) return;
  wasm.verdant_wasm_init(BigInt(activeWorldSeed));
  wasm.verdant_wasm_set_lod_rank(PRISM_MAX_LOD_RANK);
  wasm.verdant_wasm_load_domain_pack?.(0);
  readChronicleStatus();
  beaconWorldPositions = BEACON_COORDS.map(([bx, bz]) => [
    bx,
    wasm.verdant_wasm_terrain_height(bx, bz, 1.0) + 1.65,
    bz,
  ]);
  beaconsHarmonized = [false, false, false];
  runSeconds = 0;
  droneDeflections = 0;
  keys.clear();

  const aspectMilli = Math.max(400, Math.round((canvas.width / Math.max(1, canvas.height)) * 1000)) >>> 0;
  if (enterPlaying) {
    wasm.verdant_wasm_camera_set_mode(firstPerson ? 1 : 0);
    wasm.verdant_wasm_camera_set_pose_intent(-580, 280, 6200, aspectMilli);
    wasm.verdant_wasm_scene_rebuild_buffers(0);
    setGameState("playing");
    playModalChime(0, 0);
  } else {
    document.exitPointerLock?.();
    wasm.verdant_wasm_camera_set_anchor(1, 2.2, 3.8, 9.2);
    wasm.verdant_wasm_camera_set_pose_intent(460, 260, 18500, aspectMilli);
    wasm.verdant_wasm_scene_rebuild_buffers(0);
    setGameState("title");
  }
  syncHud();
}

function startOrResume() {
  document.activeElement?.blur?.();
  if (gameState === "paused") {
    if (btnResume) btnResume.textContent = "Resume";
    keys.clear();
    setGameState("playing");
    return;
  }
  resetArenaRun(true);
}

function togglePause() {
  if (gameState === "playing") {
    document.exitPointerLock?.();
    if (modalTitle) modalTitle.textContent = "Trials Paused";
    if (modalStats) {
      const lit = beaconsHarmonized.filter(Boolean).length;
      modalStats.textContent =
        `Beacons Harmonized: ${lit} / ${BEACON_COORDS.length} · Chronicle: ${chronicleTitle} (${chronicleSupply} relics)\n` +
        `Elapsed: ${Math.floor(runSeconds)}s · Patrol Deflections: ${droneDeflections}`;
    }
    if (btnResume) btnResume.textContent = "Resume";
    keys.clear();
    dragging = false;
    setGameState("paused");
  } else if (gameState === "paused") {
    document.activeElement?.blur?.();
    setGameState("playing");
  }
}

btnStart?.addEventListener("click", (e) => {
  e.currentTarget.blur?.();
  startOrResume();
});
btnChronicle?.addEventListener("click", (e) => {
  e.currentTarget.blur?.();
  stepChronicleAtPlayer();
});
btnResume?.addEventListener("click", (e) => {
  e.currentTarget.blur?.();
  startOrResume();
});
btnRestart?.addEventListener("click", () => resetArenaRun(true));
btnTitle?.addEventListener("click", () => resetArenaRun(false));
btnPause?.addEventListener("click", (e) => {
  e.currentTarget.blur?.();
  togglePause();
});
btnCam?.addEventListener("click", (e) => {
  e.currentTarget.blur?.();
  if (!wasm) return;
  firstPerson = !firstPerson;
  wasm.verdant_wasm_camera_set_mode(firstPerson ? 1 : 0);
  syncHud();
});
btnAudio?.addEventListener("click", (e) => {
  e.currentTarget.blur?.();
  audioEnabled = !audioEnabled;
  btnAudio.setAttribute("aria-pressed", String(audioEnabled));
  btnAudio.textContent = audioEnabled ? "Audio: On" : "Audio";
  if (audioEnabled) playModalChime(2, 0);
});

touchPad?.querySelectorAll("button[data-move]").forEach((b) => {
  const code = b.dataset.move;
  const down = (e) => {
    e.preventDefault();
    if (code === "Space") jumpQueued = true;
    keys.add(code);
  };
  const up = () => keys.delete(code);
  b.addEventListener("pointerdown", down);
  b.addEventListener("pointerup", up);
  b.addEventListener("pointercancel", up);
  b.addEventListener("pointerleave", up);
});

canvas?.addEventListener("pointerdown", (e) => {
  dragging = true;
  lastX = e.clientX;
  lastY = e.clientY;
  canvas.setPointerCapture?.(e.pointerId);
});
canvas?.addEventListener("pointermove", (e) => {
  if (!dragging) return;
  const dx = e.clientX - lastX;
  const dy = e.clientY - lastY;
  lastX = e.clientX;
  lastY = e.clientY;
  dragYaw -= dx * 0.0065;
  dragPitch += firstPerson ? -dy * 0.005 : dy * 0.0055;
});
canvas?.addEventListener("pointerup", () => { dragging = false; });
canvas?.addEventListener("pointercancel", () => { dragging = false; });

addEventListener("keydown", (e) => {
  if (gameState === "title" && (e.code === "Enter" || e.code === "Space")) {
    e.preventDefault();
    startOrResume();
    return;
  }
  if ((e.code === "KeyP" || e.code === "Escape") && !e.repeat) {
    if (gameState === "playing" || gameState === "paused") {
      e.preventDefault();
      togglePause();
      return;
    }
  }
  if (e.code === "KeyR" && !e.repeat && gameState !== "title") {
    resetArenaRun(true);
    return;
  }
  if (e.code === "KeyC" && !e.repeat && gameState === "playing" && wasm) {
    firstPerson = !firstPerson;
    wasm.verdant_wasm_camera_set_mode(firstPerson ? 1 : 0);
    syncHud();
    return;
  }
  if (e.code === "KeyE" && !e.repeat && gameState === "playing") {
    stepChronicleAtPlayer();
    return;
  }
  if (e.code === "KeyM" && !e.repeat) {
    btnAudio?.click();
    return;
  }
  if (e.code === "Space") jumpQueued = true;
  keys.add(e.code);
});
addEventListener("keyup", (e) => keys.delete(e.code));
addEventListener("blur", () => keys.clear());
document.addEventListener("visibilitychange", () => {
  if (document.hidden) keys.clear();
});
canvas?.addEventListener("webglcontextlost", (e) => e.preventDefault());
canvas?.addEventListener("webglcontextrestored", async () => {
  renderer = await createBestRenderer(canvas);
  if (wasm) {
    const csmSplits = [0, 1, 2, 3].map((i) => wasm.verdant_wasm_csm_split_meters(0.25, CSM_FAR_METERS, i));
    renderer?.setCsmSplits?.(csmSplits);
  }
});

let prevNow = performance.now();
function scheduleNextFrame() {
  if (renderer?.isSoftware) {
    setTimeout(() => requestAnimationFrame(loop), 24);
  } else {
    requestAnimationFrame(loop);
  }
}

function loop(now) {
  if (!wasm || !renderer || document.hidden) {
    scheduleNextFrame();
    return;
  }
  const dt = Math.min(0.05, Math.max(0.001, (now - prevNow) * 0.001));
  prevNow = now;

  const dpr = Math.min(window.devicePixelRatio || 1, renderer.isSoftware ? 1 : 2);
  const w = Math.max(1, Math.floor(canvas.clientWidth * dpr));
  const h = Math.max(1, Math.floor(canvas.clientHeight * dpr));
  if (canvas.width !== w || canvas.height !== h) {
    canvas.width = w;
    canvas.height = h;
  }
  const aspect = Math.max(0.2, w / h);
  const aspectMilli = Math.round(aspect * 1000) >>> 0;
  const dtMicros = Math.round(dt * 1_000_000) >>> 0;

  if (gameState === "title") {
    wasm.verdant_wasm_camera_intent(
      Math.round((dragYaw + dt * 0.22) * 1000) | 0,
      Math.round(dragPitch * 1000) | 0,
      0,
      aspectMilli,
      dtMicros,
    );
    dragYaw = 0;
    dragPitch = 0;
  } else if (gameState === "playing") {
    runSeconds += dt;
    wasm.verdant_wasm_camera_intent(
      Math.round(dragYaw * 1000) | 0,
      Math.round(dragPitch * 1000) | 0,
      0,
      aspectMilli,
      dtMicros,
    );
    dragYaw = 0;
    dragPitch = 0;

    const fwd = (keys.has("KeyW") || keys.has("ArrowUp") ? 1 : 0) - (keys.has("KeyS") || keys.has("ArrowDown") ? 1 : 0);
    const strafe = (keys.has("KeyD") || keys.has("ArrowRight") ? 1 : 0) - (keys.has("KeyA") || keys.has("ArrowLeft") ? 1 : 0);
    const speedScale = keys.has("ShiftLeft") || keys.has("ShiftRight") ? 1550 : 1000;
    const jump = jumpQueued || keys.has("Space") ? 1 : 0;
    jumpQueued = false;

    const ex0 = wasm.verdant_wasm_camera_eye_x(), ez0 = wasm.verdant_wasm_camera_eye_z();
    const tx0 = wasm.verdant_wasm_camera_target_x(), tz0 = wasm.verdant_wasm_camera_target_z();
    const lookAz = Math.atan2(tx0 - ex0, tz0 - ez0);

    wasm.verdant_wasm_step_lawful(
      dtMicros,
      fwd * 1000,
      strafe * 1000,
      Math.round(lookAz * 1000) | 0,
      speedScale,
      jump,
    );
    wasm.verdant_wasm_scene_rebuild_buffers(runSeconds);

    const px = wasm.verdant_wasm_player_x();
    const py = wasm.verdant_wasm_player_y() - 1.55;
    const pz = wasm.verdant_wasm_player_z();

    // Check Patrol Mover proximity repulsion:
    const moverScale = 16.5 / 26.0;
    const mx = wasm.verdant_wasm_mover_x() * moverScale;
    const mz = wasm.verdant_wasm_mover_z() * moverScale;
    if (Math.hypot(px - mx, pz - mz) < 2.2) {
      droneDeflections += 1;
      const pushAz = Math.atan2(px - mx, pz - mz);
      wasm.verdant_wasm_step_lawful(45000, 1000, 0, Math.round(pushAz * 1000) | 0, 1800, 1);
      playModalChime(4, -0.4);
    }

    // Check Relay Beacon harmonization:
    const fdx = tx0 - ex0, fdz = tz0 - ez0, fl = Math.hypot(fdx, fdz) || 1;
    for (let i = 0; i < beaconWorldPositions.length; i++) {
      if (beaconsHarmonized[i]) continue;
      const [bx, by, bz] = beaconWorldPositions[i];
      if (Math.hypot(px - bx, pz - bz) < 3.4 && Math.abs(py - by) < 3.6) {
        beaconsHarmonized[i] = true;
        const pan = wasm.verdant_wasm_binaural_pan(bx, by, bz, ex0, wasm.verdant_wasm_camera_eye_y(), ez0, fdx / fl, 0, fdz / fl);
        playModalChime(i + 1, pan);
      }
      if (beaconsHarmonized[i] && i === 0) stepChronicleAtPlayer();
    }

    syncHud();
    if (beaconsHarmonized.every(Boolean)) {
      document.exitPointerLock?.();
      if (modalTitle) modalTitle.textContent = "All Sunspire Beacons Harmonized";
      if (modalStats) {
        modalStats.textContent =
          `Clear Time: ${Math.round(runSeconds)}s · Beacons: ${BEACON_COORDS.length} / ${BEACON_COORDS.length}\n` +
          `Chronicle: ${chronicleTitle} (${chronicleSupply} relics) · Patrol Deflections: ${droneDeflections}`;
      }
      if (btnResume) btnResume.textContent = "Play Again";
      keys.clear();
      dragging = false;
      setGameState("gameover");
    }
  }

  const tRender0 = performance.now();
  const eye = [wasm.verdant_wasm_camera_eye_x(), wasm.verdant_wasm_camera_eye_y(), wasm.verdant_wasm_camera_eye_z()];
  const target = [wasm.verdant_wasm_camera_target_x(), wasm.verdant_wasm_camera_target_y(), wasm.verdant_wasm_camera_target_z()];
  const { staticViews, dynViews } = readWasmSplitViews();
  const instViews = computeBeaconInstanceViews(now * 0.0014);
  renderer.uploadSplitBuffers(staticViews, dynViews, instViews);

  const fov = Math.PI / 3.4;
  const proj = mat4Perspective(fov, aspect, 0.25, CSM_FAR_METERS);
  const view = mat4LookAt(eye, target, [0, 1, 0]);
  renderer.render(mat4Mul(proj, view), eye, LOD3_SPEC, now * 0.001, target, fov);
  const frameMicros = Math.max(50, Math.round((performance.now() - tRender0) * 1000));
  emaFrameMicros = Math.round(emaFrameMicros * 0.85 + frameMicros * 0.15);
  const scalePermille = wasm.verdant_wasm_governor_scale_permille(emaFrameMicros >>> 0) >>> 0;
  renderer.setRenderScale?.(scalePermille);
  scheduleNextFrame();
}

async function boot() {
  renderer = await createBestRenderer(canvas);
  const [holoResp, wasmResp] = await Promise.all([
    fetch(new URL("../app.holo", import.meta.url)),
    fetch(new URL("../verdant-uorc.wasm", import.meta.url)),
  ]);
  if (!holoResp.ok || !wasmResp.ok) {
    throw new Error(`Asset fetch failed: holo=${holoResp.status} wasm=${wasmResp.status}`);
  }
  const [holoBuf, wasmBuf] = await Promise.all([
    holoResp.arrayBuffer(),
    wasmResp.arrayBuffer(),
  ]);

  const holoApp = await load(new Uint8Array(holoBuf));
  const utf8Enc = new TextEncoder();
  const utf8Dec = new TextDecoder();
  const summaryVerdict = utf8Dec.decode(await holoApp.dispatch(utf8Enc.encode("summary")));
  const lawfulVerdict = utf8Dec.decode(await holoApp.dispatch(utf8Enc.encode("scene:lawful")));
  const lod3Verdict = utf8Dec.decode(await holoApp.dispatch(utf8Enc.encode(`scene:lod:${PRISM_MAX_LOD_RANK}`)));
  if (!summaryVerdict.startsWith("conformant:") || !lawfulVerdict.startsWith("conformant:") || !lod3Verdict.startsWith("conformant:")) {
    throw new Error(`Prism Holo refused: ${summaryVerdict} | ${lawfulVerdict} | ${lod3Verdict}`);
  }
  for (const part of lod3Verdict.slice(11).split(",")) {
    const [k, v] = part.split("=");
    if ((k === "sky" || k === "fog") && v) {
      const rgb = v.split("/").map(Number);
      if (rgb.length === 3 && rgb.every(Number.isFinite)) LOD3_SPEC[k] = rgb.map((n) => n / 100);
    }
  }

  const { instance } = await WebAssembly.instantiate(wasmBuf, {});
  wasm = instance.exports;
  const csmSplits = [0, 1, 2, 3].map((i) => wasm.verdant_wasm_csm_split_meters(0.25, CSM_FAR_METERS, i));
  renderer?.setCsmSplits?.(csmSplits);

  const params = new URLSearchParams(location.search);
  const reqDomain = params.get("domain");
  if (reqDomain) {
    try {
      const wpResp = await fetch(new URL("../generated/world-differentiation-profiles.json", import.meta.url));
      if (wpResp.ok) {
        const wpJson = await wpResp.json();
        const found = (wpJson?.worlds || []).find((w) => w.slug === reqDomain || w.domainId === reqDomain);
        if (found) {
          activeWorldSeed = Number(found.seed) || PRISM_DEFAULT_WORLD_SEED;
          activeWorldTitle = found.title || activeWorldTitle;
          activeWorldSubtitle = `${found.environmentName} · ${found.defaultAppGenreLabel}`;
          if (Array.isArray(found.skyRgb) && found.skyRgb.length === 3) LOD3_SPEC.sky = found.skyRgb;
          if (Array.isArray(found.fogRgb) && found.fogRgb.length === 3) LOD3_SPEC.fog = found.fogRgb;
          document.title = `${activeWorldTitle} · 3D Playable World`;
          const h1 = document.querySelector(".title-h1");
          if (h1) h1.textContent = activeWorldTitle;
        }
      }
    } catch {
      // Fallback gracefully to default Sunspire Beacon Trials profile.
    }
  }

  const autostart = params.get("autostart") === "1";
  resetArenaRun(autostart);

  if (holoBadge) {
    holoBadge.textContent = activeWorldSubtitle
      ? `${activeWorldSubtitle} · Harmonize ${BEACON_COORDS.length} Beacons`
      : `Harmonize ${BEACON_COORDS.length} Coastal Beacons Across the Stone Arch Basin`;
  }

  prevNow = performance.now();
  requestAnimationFrame(loop);
}

boot().catch((err) => {
  if (holoBadge) holoBadge.textContent = `Basin unavailable: ${err?.message || err}`;
  console.error("Sunspire Beacon Trials boot error:", err);
});
