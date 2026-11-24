// Verdant Engine — Dedicated WebAssembly Simulation, Tensor Compute, & OffscreenCanvas Worker.
// Executes crates/verdant-wasm (wasm32-unknown-unknown) off the main thread: LawfulWorld swept CCD
// + crease projection, LawfulCameraRig guard-sphere rigs, 22-joint LBS/IK characters, 3-pass HD
// WebGL2 + 16x16 software tile PBR, floating-origin UniversePosition portals, StoryAutomaton
// conservation ledgers, LIF/SSM observer, coded_gemm SLM, UI compliance, GLB, UORC, and Stitch.

import {
  createBestRenderer,
  mat4LookAt,
  mat4Mul,
  mat4Perspective,
} from "./scene-renderer.mjs";
import {
  getExtendedWorkerModule,
  handleExtendedWorkerMessage,
  handleStreamOrGrantWorkerMessage,
  readSolvedCamera,
  readSplitMeshViews,
  readUniverseCell,
} from "./engine-worker-codec.mjs";

const wasmUrl = new URL("./verdant-wasm.wasm", import.meta.url);
const uorcWasmUrl = new URL("./verdant-uorc.wasm", import.meta.url);
const uorcStreamUrl = new URL("./scene-stream.uorc", import.meta.url);

async function instantiateWasmModule(targetUrl) {
  const response = await fetch(targetUrl);
  if (!response.ok) throw new Error(`wasm HTTP ${response.status}`);
  if (typeof WebAssembly.instantiateStreaming === "function") {
    try {
      const { instance } = await WebAssembly.instantiateStreaming(response, {});
      return instance.exports;
    } catch {
      const fallbackResp = await fetch(targetUrl);
      const buf = await fallbackResp.arrayBuffer();
      const { instance } = await WebAssembly.instantiate(buf, {});
      return instance.exports;
    }
  }
  const buf = await response.arrayBuffer();
  const { instance } = await WebAssembly.instantiate(buf, {});
  return instance.exports;
}

let ringCapacity = 64;
let snapshotMaxBytes = 32768;
let tileEdgePx = 16;
let activeSeed = 7n;

const initCoreSceneWasm = () =>
  instantiateWasmModule(wasmUrl).then((ex) => {
    const seed = ex.verdant_wasm_default_seed();
    activeSeed = BigInt(seed);
    ex.verdant_wasm_init(seed);
    ringCapacity = ex.verdant_wasm_worker_ring_capacity() >>> 0;
    snapshotMaxBytes = ex.verdant_wasm_snapshot_max_bytes() >>> 0;
    tileEdgePx = ex.verdant_wasm_tile_edge_pixels() >>> 0;
    return ex;
  });
let wasmPromise = initCoreSceneWasm();

let uorcModPromise = null;
function getUorcWasm() {
  uorcModPromise ??= instantiateWasmModule(uorcWasmUrl).then((ex) => {
    ex.verdant_wasm_init(activeSeed);
    syncOrgansToWasm(ex);
    return ex;
  });
  return uorcModPromise;
}

let uorcStreamPromise = null;
function getCanonicalUorcStream() {
  uorcStreamPromise ??= fetch(uorcStreamUrl).then(async (resp) => {
    if (!resp.ok) throw new Error(`scene-stream.uorc HTTP ${resp.status}`);
    return new Uint8Array(await resp.arrayBuffer());
  });
  return uorcStreamPromise;
}

const CSM_FAR_METERS = 140.0;

let wasm = null;
let uorcWasm = null;
let customGlbActive = false;
let uorcSceneActive = false;
let lastAnchor = [2.2, 3.8, 9.2];
let lastAnchorActive = true;
let lastPose = { yaw: 460, pitch: 260, distMm: 18500, aspectMilli: 1600 };
let canvas = null;
let renderer = null;
let forceTileMode = false;
let currentLod = null;
let pendingCommands = 0;
let emaFrameMicros = 8000;
let activeInstancedCount = 0;
let autoScaleLodEnabled = false;
let activeMeshletSseTenths = 10;
let lastWorkloadScaling = null;
const slmKappaCache = new Map();
let slmKappaHits = 0;
const walkJournal = [];
const organModes = new Uint8Array(11);

function syncOrgansToWasm(uex, queryIdx = 0) {
  if (!uex || typeof uex.verdant_wasm_set_organ_grant !== "function") return true;
  uex.verdant_wasm_set_organ_grant(0, 0);
  for (let i = 0; i < organModes.length; i++) {
    const m = organModes[i];
    if (m === 1 || m === 2) uex.verdant_wasm_set_organ_grant(i, m);
  }
  return uex.verdant_wasm_set_organ_grant(queryIdx >>> 0, 99) === 1;
}

function simHost() {
  return activeSceneHost();
}

function activeSceneHost() {
  return (customGlbActive || uorcSceneActive) && uorcWasm ? uorcWasm : wasm;
}

function syncCoreStateToUorc(uex) {
  for (const step of walkJournal) {
    uex.verdant_wasm_step_lawful(step.dtMicros, step.fwdM, step.strafeM, step.yawM, step.speedM, step.jump);
  }
  const liveYaw = wasm.verdant_wasm_camera_yaw_milli() | 0;
  if (lastAnchorActive) {
    uex.verdant_wasm_camera_set_anchor(1, lastAnchor[0], lastAnchor[1], lastAnchor[2]);
  } else {
    uex.verdant_wasm_camera_set_mode(wasm.verdant_wasm_camera_mode() >>> 0);
  }
  uex.verdant_wasm_camera_set_pose_intent(
    liveYaw || lastPose.yaw,
    lastPose.pitch,
    lastPose.distMm,
    lastPose.aspectMilli,
  );
  syncOrgansToWasm(uex);
}

async function ensureUorcSceneHost() {
  uorcWasm ??= await getUorcWasm();
  if (!uorcSceneActive) {
    uorcWasm.verdant_wasm_init(activeSeed);
    uorcWasm.verdant_wasm_set_lod_rank(wasm.verdant_wasm_active_lod_rank() >>> 0);
    syncCoreStateToUorc(uorcWasm);
    uorcWasm.verdant_wasm_scene_rebuild_buffers(0);
    uorcSceneActive = true;
  }
  return uorcWasm;
}

function drawScene(eye, lookTarget, lodSpec, simClockSec, rebuildDynamic = false, targetW = 0, targetH = 0, forceSync = true) {
  const host = activeSceneHost();
  if (!host) return { tris: 0, aspect: 1.6 };
  const glbAnimated = customGlbActive && typeof host.verdant_wasm_glb_has_animation === "function" && host.verdant_wasm_glb_has_animation() === 1;
  if (rebuildDynamic && (!customGlbActive || glbAnimated)) {
    host.verdant_wasm_scene_rebuild_buffers(simClockSec || 0);
  }
  const split = readSplitMeshViews(host);
  if (!canvas || !renderer) {
    return { tris: split?.tris ?? 0, aspect: 1.6 };
  }
  if (targetW > 0 && targetH > 0 && (canvas.width !== targetW || canvas.height !== targetH)) {
    canvas.width = targetW;
    canvas.height = targetH;
  }
  if (canvas.width <= 0 || canvas.height <= 0) {
    return { tris: split?.tris ?? 0, aspect: 1.6 };
  }
  const aspect = Math.max(0.2, canvas.width / canvas.height);

  if ((forceTileMode || renderer.backend === "tensor-2d") && host.memory && typeof host.verdant_wasm_render_tensor_rgba === "function") {
    const scaleDiv = canvas.width > 480 ? 2 : 1;
    const w = Math.max(tileEdgePx, Math.floor(canvas.width / scaleDiv / tileEdgePx) * tileEdgePx);
    const h = Math.max(tileEdgePx, Math.floor(canvas.height / scaleDiv / tileEdgePx) * tileEdgePx);
    const byteLen = host.verdant_wasm_render_tensor_rgba(
      w >>> 0,
      h >>> 0,
      eye[0],
      eye[1],
      eye[2],
      lookTarget[0],
      lookTarget[1],
      lookTarget[2],
    ) >>> 0;
    const ptr = host.verdant_wasm_fb_ptr() >>> 0;
    if (byteLen === w * h * 4 && ptr > 0) {
      renderer.blitTensorRgba(new Uint8Array(host.memory.buffer, ptr, byteLen), w, h);
    }
  } else if (split) {
    if (customGlbActive && typeof host.verdant_wasm_scene_tex_ptr === "function") {
      const te = host.verdant_wasm_scene_tex_epoch() >>> 0;
      const tp = host.verdant_wasm_scene_tex_ptr() >>> 0, tw = host.verdant_wasm_scene_tex_width() >>> 0, th = host.verdant_wasm_scene_tex_height() >>> 0;
      const prevEp = renderer._lastPbrEp || 0;
      if (tp && tw && th) {
        renderer.setTex?.(1, new Uint8Array(host.memory.buffer, tp, tw * th * 4), tw, th, te);
        if (te !== prevEp && typeof host.verdant_wasm_scene_pbr_tex_ptr === "function") {
          renderer._lastPbrEp = te;
          for (const [ch, slot] of [[1, 3], [2, 4], [3, 5]]) {
            const p = host.verdant_wasm_scene_pbr_tex_ptr(ch) >>> 0;
            const pw = host.verdant_wasm_scene_pbr_tex_width(ch) >>> 0;
            const ph = host.verdant_wasm_scene_pbr_tex_height(ch) >>> 0;
            renderer.setTex?.(slot, p && pw && ph ? new Uint8Array(host.memory.buffer, p, pw * ph * 4) : null, pw, ph, te);
          }
        }
      } else if (!renderer._browserPbrTexActive) {
        renderer.setTex?.(1, null, 0, 0, te);
      }
    } else {
      renderer._lastPbrEp = 0;
      renderer._browserPbrTexActive = false;
      renderer.setTex?.(1, null, 0, 0, 0);
    }
    renderer.uploadSplitBuffers(split.staticViews, split.dynViews, split.instViews);
    if (
      split.staticViews &&
      split.staticViews.idx.length > 24576 &&
      typeof renderer.updateStaticIndices === "function" &&
      typeof host.verdant_wasm_uhd_cull_meshlets === "function" &&
      typeof host.verdant_wasm_uhd_visible_idx_ptr === "function"
    ) {
      const sseTenths = customGlbActive ? Math.max(6, activeMeshletSseTenths) : Math.max(8, activeMeshletSseTenths);
      const ck = `${split.staticViews.epoch}:${Math.round(eye[0] * 10)},${Math.round(eye[1] * 10)},${Math.round(eye[2] * 10)}:${canvas.height}:${sseTenths}`;
      if (ck !== renderer._lastCullKey) {
        renderer._lastCullKey = ck;
        host.verdant_wasm_uhd_cull_meshlets(
          eye[0], eye[1], eye[2],
          lookTarget[0], lookTarget[1], lookTarget[2],
          aspect, canvas.height >>> 0, sseTenths >>> 0,
        );
        const vPtr = host.verdant_wasm_uhd_visible_idx_ptr() >>> 0;
        const vCnt = host.verdant_wasm_uhd_visible_idx_count() >>> 0;
        if (vPtr > 0 && vCnt > 0 && vCnt <= split.staticViews.idx.length) {
          renderer.updateStaticIndices(new Uint32Array(host.memory.buffer, vPtr, vCnt));
        }
      }
    }
    const fovRad = Math.PI / 3.4;
    const camDist = Math.hypot(eye[0] - lookTarget[0], eye[1] - lookTarget[1], eye[2] - lookTarget[2]) || 4;
    const nearPlane = customGlbActive ? Math.max(0.05, camDist * 0.004) : 0.25;
    const farPlane = customGlbActive ? Math.max(CSM_FAR_METERS, camDist * 8) : CSM_FAR_METERS;
    const proj = mat4Perspective(fovRad, aspect, nearPlane, farPlane);
    const view = mat4LookAt(eye, lookTarget, [0, 1, 0]);
    const effectiveLod = customGlbActive
      ? { ...(lodSpec || {}), rank: 3, sky: [0.06, 0.07, 0.09], fog: [0.06, 0.07, 0.09] }
      : lodSpec;
    renderer.render(mat4Mul(proj, view), eye, effectiveLod, simClockSec || 0, lookTarget, fovRad, forceSync);
  }

  return { tris: split?.tris ?? 0, aspect };
}

self.onmessage = async ({ data: msg }) => {
  if (!msg || typeof msg.kind !== "string") return;
  if (pendingCommands >= ringCapacity && msg.kind === "frame") {
    self.postMessage({ kind: "frame-dropped" });
    return;
  }
  pendingCommands++;
  try {
    wasm ??= await wasmPromise;

    if (msg.kind === "init") {
      canvas = msg.canvas || null;
      currentLod = msg.lod;
      autoScaleLodEnabled = Boolean(msg.autoScale);
      const w = Math.max(1, msg.width | 0);
      const h = Math.max(1, msg.height | 0);
      if (canvas) {
        canvas.width = w;
        canvas.height = h;
        if (msg.hdRenderer) {
          let wgRenderer = null;
          try {
            const wgMod = await import("./scene-webgpu-renderer.mjs");
            wgRenderer = await wgMod.createWebGpuRenderer(canvas);
          } catch {
            wgRenderer = null;
          }
          if (wgRenderer) {
            renderer = wgRenderer;
          } else {
            const rMod = await import("./scene-hd-renderer.mjs");
            renderer = await rMod.createBestRenderer(canvas);
          }
        } else {
          renderer = await createBestRenderer(canvas);
        }
      }
      const csmSplits = [0, 1, 2, 3].map((i) => wasm.verdant_wasm_csm_split_meters(0.25, CSM_FAR_METERS, i));
      renderer?.setCsmSplits?.(csmSplits);
      const aspectMilli = Math.max(200, Math.round((w / h) * 1000)) >>> 0;
      lastPose.aspectMilli = aspectMilli;
      const reqRank = (currentLod?.rank ?? 2) >>> 0;
      if ((wasm.verdant_wasm_active_lod_rank() >>> 0) !== reqRank) {
        wasm.verdant_wasm_set_lod_rank(reqRank);
      }
      wasm.verdant_wasm_camera_set_anchor(1, 2.2, 3.8, 9.2);
      wasm.verdant_wasm_camera_set_pose_intent(460, 260, 18500, aspectMilli);
      wasm.verdant_wasm_scene_rebuild_buffers(0);
      if (renderer?.backend === "tensor-2d") {
        await ensureUorcSceneHost();
      }
      const shInit = simHost();
      const cam = readSolvedCamera(shInit);
      const stats = drawScene(cam.eye, cam.lookTarget, currentLod, 0, false, w, h);
      const idx0 = cam.lookTarget[0] - cam.eye[0], idy0 = cam.lookTarget[1] - cam.eye[1], idz0 = cam.lookTarget[2] - cam.eye[2];
      const idl0 = Math.hypot(idx0, idy0, idz0) || 1;
      const initVis = wasm.verdant_wasm_bvh_visible_count(
        cam.eye[0], cam.eye[1], cam.eye[2],
        idx0 / idl0, idy0 / idl0, idz0 / idl0,
        stats.aspect,
      ) >>> 0;
      self.postMessage({
        kind: "ready",
        id: msg.id,
        backend: forceTileMode ? "tile-16x16" : (renderer?.backend ?? "headless"),
        isSoftware: Boolean(renderer?.isSoftware),
        maxTextureSize: renderer?.maxTextureSize || 2048,
        walkablePolys: wasm.verdant_wasm_navmesh_polygon_count() >>> 0,
        tensorKernels: wasm.verdant_wasm_unified_tensor_kernel_count() >>> 0,
        codedTiers: wasm.verdant_wasm_coded_tier_count() >>> 0,
        hdMaxLodRank: wasm.verdant_wasm_hd_max_lod_rank() >>> 0,
        csmSplits,
        tris: stats.tris,
        bvhVisible: initVis,
        camera: cam,
        universeCell: readUniverseCell(wasm),
        audioSpec: {
          quantumFrames: wasm.verdant_wasm_audio_quantum_frames() >>> 0,
          stereoChannels: wasm.verdant_wasm_audio_channels() >>> 0,
          modeNumers: [0, 1, 2, 3, 4].map((i) => wasm.verdant_wasm_modal_bar_numer(i) >>> 0),
          modeDenom: wasm.verdant_wasm_modal_bar_denom() >>> 0,
          rt60Ms: wasm.verdant_wasm_sabine_rt60_millis(1800, 96000) >>> 0,
        },
      });
      return;
    }

    if (msg.kind === "resize") {
      const w = Math.max(1, msg.width | 0);
      const h = Math.max(1, msg.height | 0);
      const sh = simHost();
      const cam = readSolvedCamera(sh);
      drawScene(cam.eye, cam.lookTarget, currentLod, 0, false, w, h);
      return;
    }

    if (msg.kind === "camera-set-mode") {
      if (!customGlbActive) await ensureUorcSceneHost();
      const sh = simHost();
      const wasAnchor = lastAnchorActive;
      lastAnchorActive = false;
      sh.verdant_wasm_camera_set_mode((msg.mode ?? 0) >>> 0);
      if (msg.mode === 0) {
        const aspectMilli = canvas && canvas.height > 0 ? Math.round((canvas.width / canvas.height) * 1000) >>> 0 : 1600;
        const nextYaw = wasAnchor ? -580 : (sh.verdant_wasm_camera_yaw_milli() | 0);
        lastPose = { yaw: nextYaw, pitch: 280, distMm: 6200, aspectMilli };
        sh.verdant_wasm_camera_set_pose_intent(nextYaw, 280, 6200, aspectMilli);
      }
      if (!customGlbActive) sh.verdant_wasm_scene_rebuild_buffers(msg.simClockSec || 0);
      const cam = readSolvedCamera(sh);
      const stats = drawScene(cam.eye, cam.lookTarget, currentLod, msg.simClockSec || 0, false);
      self.postMessage({ kind: "camera-updated", id: msg.id, camera: cam, tris: stats.tris });
      return;
    }

    if (msg.kind === "camera-focus-landmark") {
      if (msg.possessExplorer && !customGlbActive) await ensureUorcSceneHost();
      const sh = simHost();
      if (msg.lod) {
        currentLod = msg.lod;
        customGlbActive = false;
        const reqRank = (currentLod?.rank ?? 3) >>> 0;
        if ((sh.verdant_wasm_active_lod_rank() >>> 0) !== reqRank) {
          sh.verdant_wasm_set_lod_rank(reqRank);
        }
      }
      const aspectMilli = canvas && canvas.height > 0 ? Math.round((canvas.width / canvas.height) * 1000) >>> 0 : 1600;
      if (msg.possessExplorer) {
        lastAnchorActive = false;
        sh.verdant_wasm_camera_set_mode(0);
        lastPose = { yaw: -580, pitch: 280, distMm: Math.round((msg.dist || 6.2) * 1000) >>> 0, aspectMilli };
        sh.verdant_wasm_camera_set_pose_intent(-580, 280, lastPose.distMm, aspectMilli);
      } else {
        lastAnchor = [msg.target[0], msg.target[1], msg.target[2]];
        lastAnchorActive = true;
        sh.verdant_wasm_camera_set_anchor(1, lastAnchor[0], lastAnchor[1], lastAnchor[2]);
        lastPose = {
          yaw: msg.yawMilli ?? 480,
          pitch: msg.pitchMilli ?? 310,
          distMm: Math.round((msg.dist || 17.5) * 1000) >>> 0,
          aspectMilli,
        };
        sh.verdant_wasm_camera_set_pose_intent(lastPose.yaw, lastPose.pitch, lastPose.distMm, aspectMilli);
      }
      if (!customGlbActive) sh.verdant_wasm_scene_rebuild_buffers(msg.simClockSec || 0);
      const cam = readSolvedCamera(sh);
      const stats = drawScene(cam.eye, cam.lookTarget, currentLod, msg.simClockSec || 0, false, 0, 0, true);
      self.postMessage({ kind: "camera-updated", id: msg.id, camera: cam, tris: stats.tris });
      return;
    }

    if (msg.kind === "set-render-mode") {
      forceTileMode = msg.mode === "tile";
      if (forceTileMode) {
        await ensureUorcSceneHost();
      }
      if (msg.lod) currentLod = msg.lod;
      const sh = simHost();
      const cam = readSolvedCamera(sh);
      const stats = drawScene(cam.eye, cam.lookTarget, currentLod, msg.simClockSec || 0, false, 0, 0, true);
      self.postMessage({
        kind: "render-mode-set",
        id: msg.id,
        mode: forceTileMode ? "tile-16x16" : (renderer?.backend ?? "webgl2"),
        tris: stats.tris,
        camera: cam,
      });
      return;
    }

    if (msg.kind === "set-lod") {
      currentLod = msg.lod;
      customGlbActive = false;
      const sh = simHost();
      sh.verdant_wasm_set_lod_rank((currentLod?.rank ?? 2) >>> 0);
      const cam = readSolvedCamera(sh);
      drawScene(cam.eye, cam.lookTarget, currentLod, msg.simClockSec || 0, false);
      return;
    }

    if (msg.kind === "get-packed-buffers") {
      const { readZeroCopyMeshViews } = await getExtendedWorkerModule();
      const host = activeSceneHost();
      const sh = simHost();
      if (!customGlbActive) host.verdant_wasm_scene_rebuild_buffers(msg.simClockSec || 0);
      const views = readZeroCopyMeshViews(host);
      const cam = readSolvedCamera(sh);
      if (!views) {
        self.postMessage({ kind: "packed-buffers", id: msg.id, ok: false, camera: cam });
        return;
      }
      const pos = views.pos.slice();
      const norm = views.norm.slice();
      const col = views.col.slice();
      const idx = views.idx.slice();
      self.postMessage(
        { kind: "packed-buffers", id: msg.id, ok: true, pos, norm, col, idx, tris: views.tris, camera: cam },
        [pos.buffer, norm.buffer, col.buffer, idx.buffer],
      );
      return;
    }

    if (msg.kind === "frame") {
      const tStart = performance.now();
      const lod = msg.lod || currentLod;
      currentLod = lod;
      const targetW = msg.width > 0 ? msg.width | 0 : 0;
      const targetH = msg.height > 0 ? msg.height | 0 : 0;
      const dtMicros = Math.max(1000, Math.min(50000, Math.round((msg.dt || 0.016) * 1_000_000))) >>> 0;
      const aspectMilli = Math.max(200, Math.round(((msg.width || 1280) / Math.max(1, msg.height || 800)) * 1000)) >>> 0;
      const sh = simHost();

      if (msg.followMover) {
        const scale = 16.5 / 26.0;
        const mx = sh.verdant_wasm_mover_x() * scale;
        const mz = sh.verdant_wasm_mover_z() * scale;
        const my = sh.verdant_wasm_terrain_height(mx, mz, 1.0) + 0.8;
        sh.verdant_wasm_camera_set_anchor(1, mx, my, mz);
      }

      sh.verdant_wasm_camera_intent(
        Math.round((msg.deltaYaw || 0) * 1000) | 0,
        Math.round((msg.deltaPitch || 0) * 1000) | 0,
        Math.round((msg.deltaDist || 0) * 1000) | 0,
        aspectMilli,
        dtMicros,
      );

      let cam = readSolvedCamera(sh);
      if (msg.stepPhysics) {
        const dx = cam.lookTarget[0] - cam.eye[0];
        const dz = cam.lookTarget[2] - cam.eye[2];
        const lookAzimuth = Math.atan2(dx, dz);
        const fwdM = Math.round((msg.fwd || 0) * 1000) | 0;
        const strafeM = Math.round((msg.strafe || 0) * 1000) | 0;
        const yawM = Math.round(lookAzimuth * 1000) | 0;
        const speedM = Math.round((msg.speedScale || 1.0) * 1000) >>> 0;
        const jumpBit = msg.jump ? 1 : 0;
        sh.verdant_wasm_step_lawful(dtMicros, fwdM, strafeM, yawM, speedM, jumpBit);
        if (!uorcSceneActive && (fwdM || strafeM || jumpBit)) {
          if (walkJournal.length >= 256) walkJournal.shift();
          walkJournal.push({ dtMicros, fwdM, strafeM, yawM, speedM, jump: jumpBit });
        }
        cam = readSolvedCamera(sh);
      }

      sh.verdant_wasm_step_observer(
        (msg.gazedSlot ?? 0) >>> 0,
        (msg.dwellMs ?? 250) >>> 0,
        (msg.angSpeedMilli ?? 80) >>> 0,
        Math.round(Math.hypot(msg.fwd || 0, msg.strafe || 0) * 1000) >>> 0,
        Math.round(cam.yawRad * 1000) | 0,
      );

      const stats = drawScene(cam.eye, cam.lookTarget, lod, msg.simClockSec || 0, Boolean(msg.stepPhysics), targetW, targetH, false);
      const dx = cam.lookTarget[0] - cam.eye[0];
      const dy = cam.lookTarget[1] - cam.eye[1];
      const dz = cam.lookTarget[2] - cam.eye[2];
      const dl = Math.hypot(dx, dy, dz) || 1;
      const fx = dx / dl, fy = dy / dl, fz = dz / dl;
      const host = activeSceneHost();
      const bvhVisible = host.verdant_wasm_bvh_visible_count(
        cam.eye[0], cam.eye[1], cam.eye[2],
        fx, fy, fz,
        stats.aspect,
      ) >>> 0;
      const pan = sh.verdant_wasm_binaural_pan(
        -9.0, 2.0, -4.0,
        cam.eye[0], cam.eye[1], cam.eye[2],
        fx, fy, fz,
      );
      const gain = sh.verdant_wasm_binaural_gain();
      const binauralItdMicros = sh.verdant_wasm_binaural_itd_micros();
      const scale = 16.5 / 26.0;
      const mx = sh.verdant_wasm_mover_x() * scale;
      const mz = sh.verdant_wasm_mover_z() * scale;
      const my = sh.verdant_wasm_terrain_height(mx, mz, 1.0) + 0.5;
      const salience = [0, 1, 2, 3, 4, 5, 6].map((s) => sh.verdant_wasm_observer_salience_milli(s));
      const frameMicros = Math.max(50, Math.round((performance.now() - tStart) * 1000));
      emaFrameMicros = Math.round(emaFrameMicros * 0.85 + frameMicros * 0.15);
      const govScale = sh.verdant_wasm_governor_scale_permille(emaFrameMicros >>> 0) >>> 0;
      let scalePermille = govScale;
      let casPermille = 350;
      if (typeof uorcWasm?.verdant_wasm_workload_governor_step_aux === "function") {
        const vw = (canvas?.width || targetW || 1280) >>> 0;
        const vh = (canvas?.height || targetH || 720) >>> 0;
        const isSw = renderer?.isSoftware ? 1 : 0;
        const isWgpu = renderer?.backend === "webgpu" ? 1 : 0;
        const gLen = uorcWasm.verdant_wasm_workload_governor_step_aux(
          vw, vh, isSw, isWgpu, frameMicros >>> 0,
        ) >>> 0;
        if (gLen > 0) {
          try {
            const gPtr = uorcWasm.verdant_wasm_aux_ptr() >>> 0;
            const dec = JSON.parse(new TextDecoder().decode(new Uint8Array(uorcWasm.memory.buffer, gPtr, gLen)));
            lastWorkloadScaling = dec;
            scalePermille = Math.min(govScale, dec.drsScalePermille || govScale);
            casPermille = dec.casSharpnessPermille || 350;
            activeMeshletSseTenths = dec.meshletSseTenthsPx || 10;
            if (
              autoScaleLodEnabled &&
              !customGlbActive &&
              typeof dec.recommendedLodRank === "number" &&
              dec.recommendedLodRank !== (sh.verdant_wasm_active_lod_rank() >>> 0) &&
              uorcSceneActive
            ) {
              sh.verdant_wasm_set_lod_rank(dec.recommendedLodRank >>> 0);
            }
          } catch {
            // Keep prior decision on parse edge.
          }
        }
      } else if (typeof uorcWasm?.verdant_wasm_record_frame_time_us === "function") {
        const oracleDrs = uorcWasm.verdant_wasm_record_frame_time_us(frameMicros >>> 0) >>> 0;
        if (oracleDrs > 0) scalePermille = Math.min(govScale, oracleDrs);
      }
      renderer?.setRenderScale?.(scalePermille, casPermille);
      const upload = renderer?.getUploadTelemetry?.() || { staticBytes: 0, dynBytes: 0, instCount: activeInstancedCount };
      self.postMessage({
        kind: "telemetry",
        backend: forceTileMode ? "tile-16x16" : (renderer?.backend ?? "webgl2"),
        tris: stats.tris,
        bvhVisible,
        frameMicros: emaFrameMicros,
        scalePermille,
        casPermille,
        workloadScaling: lastWorkloadScaling,
        upload,
        eye: cam.eye,
        lookTarget: cam.lookTarget,
        camMode: cam.camMode,
        camObstructed: cam.obstructed,
        camClearanceMm: cam.clearanceMm,
        universeCell: readUniverseCell(sh),
        playerPos: [sh.verdant_wasm_player_x(), sh.verdant_wasm_player_y() - 1.55, sh.verdant_wasm_player_z()],
        moverPos: [mx, my, mz],
        pavilionContact: sh.verdant_wasm_pavilion_contact() === 1,
        binauralPan: pan,
        binauralGain: gain,
        binauralItdMicros,
        attendedSlot: sh.verdant_wasm_observer_attended_slot() >>> 0,
        foveatedLod: sh.verdant_wasm_observer_foveated_lod() >>> 0,
        gimbalYawMilli: sh.verdant_wasm_observer_gimbal_yaw_milli() | 0,
        spikeMask: sh.verdant_wasm_observer_spike_mask() >>> 0,
        salience,
      });
      return;
    }

    if (msg.kind === "configure-instanced") {
      const sh = await ensureUorcSceneHost();
      const count = Math.max(0, Math.min(65535, msg.count | 0)) >>> 0;
      activeInstancedCount = count;
      sh.verdant_wasm_configure_instanced_workload(count);
      const cam = readSolvedCamera(sh);
      const stats = drawScene(cam.eye, cam.lookTarget, currentLod, msg.simClockSec || 0, false);
      self.postMessage({
        kind: "instanced-configured",
        id: msg.id,
        ok: true,
        instCount: count,
        tris: stats.tris,
        upload: renderer?.getUploadTelemetry?.() || { staticBytes: 0, dynBytes: 0, instCount: count },
      });
      return;
    }

    if (msg.kind === "story-status" || msg.kind === "story-interact") {
      const { readStoryStatus } = await getExtendedWorkerModule();
      uorcWasm ??= await getUorcWasm();
      const sh = simHost();
      let advanced = false;
      if (msg.kind === "story-interact") {
        const px = msg.pos ? msg.pos[0] : sh.verdant_wasm_player_x();
        const py = msg.pos ? msg.pos[1] : sh.verdant_wasm_player_y() - 1.55;
        const pz = msg.pos ? msg.pos[2] : sh.verdant_wasm_player_z();
        advanced = uorcWasm.verdant_wasm_story_interact_at(px, py, pz) === 1;
      }
      const status = readStoryStatus(uorcWasm);
      self.postMessage({
        kind: "story-updated",
        id: msg.id,
        ok: Boolean(status),
        advanced,
        ...status,
      });
      return;
    }

    if (msg.kind === "export-glb") {
      const { readAuxBytes } = await getExtendedWorkerModule();
      uorcWasm ??= await getUorcWasm();
      const sh = simHost();
      uorcWasm.verdant_wasm_set_lod_rank(sh.verdant_wasm_active_lod_rank() >>> 0);
      const len = uorcWasm.verdant_wasm_glb_export_aux() >>> 0;
      if (len === 0) {
        self.postMessage({ kind: "glb-ready", id: msg.id, buffer: null });
        return;
      }
      const aux = readAuxBytes(uorcWasm);
      self.postMessage({ kind: "glb-ready", id: msg.id, buffer: aux.buffer }, [aux.buffer]);
      return;
    }

    if (msg.kind === "import-glb") {
      const ext = await getExtendedWorkerModule();
      const { readAuxBytes, stageIoBytes } = ext;
      const raw = msg.bytes instanceof Uint8Array ? msg.bytes : new Uint8Array(msg.bytes || []);
      uorcWasm ??= await getUorcWasm();
      if (!raw.byteLength || !stageIoBytes(uorcWasm, raw)) {
        self.postMessage({ kind: "glb-imported", id: msg.id, ok: false });
        return;
      }
      const tris = uorcWasm.verdant_wasm_glb_import_io() >>> 0;
      if (tris === 0) {
        self.postMessage({ kind: "glb-imported", id: msg.id, ok: false });
        return;
      }
      customGlbActive = true;
      const uorcStream = readAuxBytes(uorcWasm);
      const cam = readSolvedCamera(simHost());
      lastAnchor = [cam.lookTarget[0], cam.lookTarget[1], cam.lookTarget[2]];
      lastAnchorActive = true;
      const dx = cam.eye[0] - cam.lookTarget[0];
      const dy = cam.eye[1] - cam.lookTarget[1];
      const dz = cam.eye[2] - cam.lookTarget[2];
      const distM = Math.max(0.5, Math.hypot(dx, dy, dz));
      lastPose = {
        yaw: Math.round(Math.atan2(dx, dz) * 1000),
        pitch: Math.round(Math.asin(Math.max(-0.95, Math.min(0.95, dy / distM))) * 1000),
        distMm: Math.round(distM * 1000) >>> 0,
        aspectMilli: lastPose?.aspectMilli || 1600,
      };
      let browserPbrTex = null;
      if ((uorcWasm.verdant_wasm_scene_pbr_tex_width?.(0) >>> 0) === 0 && renderer) {
        browserPbrTex = await ext.decodeGlbPbrTexturesInBrowser(raw, renderer.maxTextureSize || 1024, renderer);
        renderer._browserPbrTexActive = Boolean(browserPbrTex?.albedo);
      } else if (renderer) {
        renderer._browserPbrTexActive = false;
      }
      drawScene(cam.eye, cam.lookTarget, currentLod || { sky: [0.18, 0.30, 0.38], fog: [0.30, 0.40, 0.44] }, 0, false, 0, 0, true);
      const pbrChannels = {
        albedo: Boolean(browserPbrTex?.albedo) || (uorcWasm.verdant_wasm_scene_pbr_tex_width?.(0) >>> 0) > 0,
        metallicRoughness: Boolean(browserPbrTex?.metallicRoughness) || (uorcWasm.verdant_wasm_scene_pbr_tex_width?.(1) >>> 0) > 0,
        normal: Boolean(browserPbrTex?.normal) || (uorcWasm.verdant_wasm_scene_pbr_tex_width?.(2) >>> 0) > 0,
        emissive: Boolean(browserPbrTex?.emissive) || (uorcWasm.verdant_wasm_scene_pbr_tex_width?.(3) >>> 0) > 0,
        hasAnimation: uorcWasm.verdant_wasm_glb_has_animation?.() === 1,
        atlasTileCount: uorcWasm.verdant_wasm_glb_atlas_tile_count?.() >>> 0,
      };
      let meshletTelemetry = null;
      if (typeof uorcWasm.verdant_wasm_uhd_cull_meshlets === "function") {
        const aspect = canvas && canvas.height > 0 ? canvas.width / canvas.height : 1.6;
        const mLen = uorcWasm.verdant_wasm_uhd_cull_meshlets(
          cam.eye[0], cam.eye[1], cam.eye[2],
          cam.lookTarget[0], cam.lookTarget[1], cam.lookTarget[2],
          aspect, (canvas?.height || 1080) >>> 0, 10,
        ) >>> 0;
        if (mLen) meshletTelemetry = JSON.parse(new TextDecoder().decode(readAuxBytes(uorcWasm)));
      }
      self.postMessage(
        {
          kind: "glb-imported",
          id: msg.id,
          ok: true,
          tris,
          verts: uorcWasm.verdant_wasm_scene_vert_count() >>> 0,
          camera: cam,
          pbrChannels,
          meshletTelemetry,
          uorcBytes: uorcStream,
        },
        [uorcStream.buffer],
      );
      return;
    }

    if (msg.kind === "reset-glb") {
      customGlbActive = false;
      if (uorcWasm) uorcWasm.verdant_wasm_clear_imported_glb();
      const sh = simHost();
      sh.verdant_wasm_scene_rebuild_buffers(0);
      const cam = readSolvedCamera(sh);
      drawScene(cam.eye, cam.lookTarget, currentLod, 0, false);
      self.postMessage({
        kind: "glb-reset",
        id: msg.id,
        ok: true,
        tris: Math.floor((sh.verdant_wasm_scene_idx_count() >>> 0) / 3),
      });
      return;
    }

    if (msg.kind === "encode-snapshot") {
      const { readIoBytes } = await getExtendedWorkerModule();
      uorcWasm ??= await getUorcWasm();
      const sh = simHost();
      const baseRank = Math.min(3, sh.verdant_wasm_active_lod_rank() >>> 0);
      uorcWasm.verdant_wasm_set_lod_rank(baseRank);
      const len = uorcWasm.verdant_wasm_snapshot_encode() >>> 0;
      if (len === 0 || len > snapshotMaxBytes) {
        self.postMessage({ kind: "snapshot-encoded", id: msg.id, ok: false });
        return;
      }
      const bytes = readIoBytes(uorcWasm);
      self.postMessage(
        { kind: "snapshot-encoded", id: msg.id, ok: true, bytes },
        [bytes.buffer],
      );
      return;
    }

    if (msg.kind === "restore-snapshot") {
      const { stageIoBytes } = await getExtendedWorkerModule();
      const raw = msg.bytes instanceof Uint8Array ? msg.bytes : new Uint8Array(msg.bytes || []);
      uorcWasm ??= await getUorcWasm();
      if (raw.byteLength === 0 || raw.byteLength > snapshotMaxBytes || !stageIoBytes(uorcWasm, raw)) {
        self.postMessage({ kind: "snapshot-restored", id: msg.id, ok: false });
        return;
      }
      const ok = uorcWasm.verdant_wasm_snapshot_decode_and_restore() === 1;
      if (ok) {
        customGlbActive = false;
        uorcSceneActive = true;
        walkJournal.length = 0;
        activeSeed = BigInt(uorcWasm.verdant_wasm_active_seed());
        syncOrgansToWasm(uorcWasm);
      }
      const lodRank = uorcWasm.verdant_wasm_active_lod_rank() >>> 0;
      if (ok) wasm.verdant_wasm_set_lod_rank(lodRank);
      const sh = simHost();
      const seed = Number(uorcWasm.verdant_wasm_active_seed());
      const cam = readSolvedCamera(sh);
      drawScene(cam.eye, cam.lookTarget, currentLod, 0, false);
      self.postMessage({
        kind: "snapshot-restored",
        id: msg.id,
        ok,
        lodRank,
        seed,
        byteLen: raw.byteLength,
      });
      return;
    }

    if (msg.kind === "stream-uorc" || msg.kind === "set-organ-grant") {
      await handleStreamOrGrantWorkerMessage(msg, {
        getUorcWasm: async () => { uorcWasm ??= await getUorcWasm(); return uorcWasm; },
        getCanonicalUorcStream,
        getUorcSceneActive: () => uorcSceneActive,
        setUorcSceneActive: (v) => { uorcSceneActive = Boolean(v); },
        setCustomGlbActive: (v) => { customGlbActive = Boolean(v); },
        clearWalkJournal: () => { walkJournal.length = 0; },
        syncCoreStateToUorc,
        syncOrgansToWasm,
        simHost,
        getCurrentLod: () => currentLod,
        setCurrentLod: (v) => { currentLod = v; },
        drawScene,
        organModes,
        slmKappaCache,
        postMessage: (p) => self.postMessage(p),
      });
      return;
    }

    if (await handleExtendedWorkerMessage(msg, {
      ensureUorcSceneHost, simHost, drawScene, syncOrgansToWasm, slmKappaCache,
      getWasm: () => wasm, getLoadedUorcWasm: () => uorcWasm,
      getUorcWasm: async () => { uorcWasm ??= await getUorcWasm(); return uorcWasm; },
      getCanvas: () => canvas, getRenderer: () => renderer, getEmaFrameMicros: () => emaFrameMicros,
      getOrganModes: () => organModes, getCurrentLod: () => currentLod, setCurrentLod: (v) => { currentLod = v; },
      getCustomGlbActive: () => customGlbActive, setCustomGlbActive: (v) => { customGlbActive = Boolean(v); },
      getUorcSceneActive: () => uorcSceneActive, setUorcSceneActive: (v) => { uorcSceneActive = Boolean(v); },
      getLastAnchor: () => lastAnchor, getLastAnchorActive: () => lastAnchorActive,
      setLastAnchor: (anchor) => { lastAnchor = anchor; lastAnchorActive = true; },
      getLastPose: () => lastPose, setLastPose: (pose) => { lastPose = pose; },
      setActiveSeed: (s) => { activeSeed = s; }, clearWalkJournal: () => { walkJournal.length = 0; },
      getSlmKappaHits: () => slmKappaHits, incSlmKappaHits: () => ++slmKappaHits,
      postMessage: (p, t) => self.postMessage(p, t),
    })) {
      return;
    }
  } catch (err) {
    if (typeof WebAssembly !== "undefined" && err instanceof WebAssembly.RuntimeError) {
      wasm = null; uorcWasm = null; uorcModPromise = null; wasmPromise = initCoreSceneWasm();
    }
    if (msg.kind === "frame") self.postMessage({ kind: "frame-dropped" });
    else if (msg.id) self.postMessage({ kind: "worker-error", id: msg.id, ok: false, error: String(err?.message || err) });
  } finally {
    pendingCommands = Math.max(0, pendingCommands - 1);
  }
};
