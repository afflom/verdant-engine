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

function stageIoBytes(ex, bytes) {
  if (typeof ex.verdant_wasm_io_resize === "function") {
    const ptr = ex.verdant_wasm_io_resize(bytes.byteLength >>> 0) >>> 0;
    if (ptr > 0 && ex.memory) {
      new Uint8Array(ex.memory.buffer, ptr, bytes.byteLength).set(bytes);
      return true;
    }
  }
  ex.verdant_wasm_io_clear();
  for (let i = 0; i < bytes.byteLength; i++) ex.verdant_wasm_io_push_byte(bytes[i]);
  return (ex.verdant_wasm_io_len() >>> 0) === bytes.byteLength;
}

function readIoBytes(ex) {
  const len = ex.verdant_wasm_io_len() >>> 0;
  if (len === 0) return new Uint8Array(0);
  if (typeof ex.verdant_wasm_io_ptr === "function" && ex.memory) {
    const ptr = ex.verdant_wasm_io_ptr() >>> 0;
    if (ptr > 0) return new Uint8Array(ex.memory.buffer, ptr, len).slice();
  }
  const out = new Uint8Array(len);
  for (let i = 0; i < len; i++) out[i] = ex.verdant_wasm_io_get_byte(i) & 0xff;
  return out;
}

function readAuxBytes(ex) {
  const len = ex.verdant_wasm_aux_len() >>> 0;
  const ptr = ex.verdant_wasm_aux_ptr() >>> 0;
  if (len === 0 || ptr === 0 || !ex.memory) return new Uint8Array(0);
  return new Uint8Array(ex.memory.buffer, ptr, len).slice();
}

function parsePipeKv(raw) {
  const kv = {};
  for (const part of raw.split("|")) {
    const eq = part.indexOf("=");
    if (eq > 0) kv[part.slice(0, eq)] = part.slice(eq + 1);
  }
  return kv;
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

function readSolvedCamera(host) {
  return {
    eye: [
      host.verdant_wasm_camera_eye_x(),
      host.verdant_wasm_camera_eye_y(),
      host.verdant_wasm_camera_eye_z(),
    ],
    lookTarget: [
      host.verdant_wasm_camera_target_x(),
      host.verdant_wasm_camera_target_y(),
      host.verdant_wasm_camera_target_z(),
    ],
    yawRad: (host.verdant_wasm_camera_yaw_milli() | 0) / 1000,
    camMode: host.verdant_wasm_camera_mode() >>> 0,
    obstructed: host.verdant_wasm_camera_obstructed() === 1,
    clearanceMm: host.verdant_wasm_camera_clearance_milli() >>> 0,
  };
}

function readUniverseCell(host) {
  return [
    Number(host.verdant_wasm_universe_cell_x()),
    Number(host.verdant_wasm_universe_cell_y()),
    Number(host.verdant_wasm_universe_cell_z()),
  ];
}

function readStoryStatus(uex) {
  const len = uex.verdant_wasm_story_status_aux() >>> 0;
  if (len === 0) return null;
  const decoded = new TextDecoder().decode(readAuxBytes(uex));
  const [idxStr, title, supplyStr, objective, ...dialogueParts] = decoded.split("|");
  return {
    stateIdx: Number(idxStr) || 0,
    title: title || "",
    supply: Number(supplyStr) || 0,
    objective: objective || "",
    dialogue: dialogueParts.join("|"),
  };
}

function readZeroCopyMeshViews(host) {
  if (!host || !host.memory) return null;
  const vCount = host.verdant_wasm_scene_vert_count() >>> 0;
  const iCount = host.verdant_wasm_scene_idx_count() >>> 0;
  const pPtr = host.verdant_wasm_scene_pos_ptr() >>> 0;
  const nPtr = host.verdant_wasm_scene_norm_ptr() >>> 0;
  const cPtr = host.verdant_wasm_scene_col_ptr() >>> 0;
  const iPtr = host.verdant_wasm_scene_idx_ptr() >>> 0;
  if (!vCount || !iCount || !pPtr || !nPtr || !cPtr || !iPtr) return null;
  const buf = host.memory.buffer;
  return {
    pos: new Float32Array(buf, pPtr, vCount * 3),
    norm: new Float32Array(buf, nPtr, vCount * 3),
    col: new Float32Array(buf, cPtr, vCount * 3),
    idx: new Uint32Array(buf, iPtr, iCount),
    tris: Math.floor(iCount / 3),
  };
}

function readSplitMeshViews(host) {
  if (!host || !host.memory) return null;
  const buf = host.memory.buffer;
  const sEpoch = host.verdant_wasm_static_epoch() >>> 0;
  const sVerts = host.verdant_wasm_static_vert_count() >>> 0;
  const sIdx = host.verdant_wasm_static_idx_count() >>> 0;
  const pPtr = host.verdant_wasm_scene_pos_ptr() >>> 0;
  const nPtr = host.verdant_wasm_scene_norm_ptr() >>> 0;
  const cPtr = host.verdant_wasm_scene_col_ptr() >>> 0;
  const iPtr = host.verdant_wasm_scene_idx_ptr() >>> 0;

  const uvPtr = typeof host.verdant_wasm_scene_uv_ptr === "function" ? (host.verdant_wasm_scene_uv_ptr() >>> 0) : 0;
  const staticViews = sVerts > 0 && sIdx > 0 && pPtr && nPtr && cPtr && iPtr ? {
    epoch: sEpoch,
    pos: new Float32Array(buf, pPtr, sVerts * 3),
    norm: new Float32Array(buf, nPtr, sVerts * 3),
    col: new Float32Array(buf, cPtr, sVerts * 3),
    uv: uvPtr ? new Float32Array(buf, uvPtr, sVerts * 2) : null,
    idx: new Uint32Array(buf, iPtr, sIdx),
  } : null;

  const dVerts = host.verdant_wasm_dyn_vert_count() >>> 0;
  const dIdx = host.verdant_wasm_dyn_idx_count() >>> 0;
  const dpPtr = host.verdant_wasm_dyn_pos_ptr() >>> 0;
  const dnPtr = host.verdant_wasm_dyn_norm_ptr() >>> 0;
  const dcPtr = host.verdant_wasm_dyn_col_ptr() >>> 0;
  const diPtr = host.verdant_wasm_dyn_idx_ptr() >>> 0;
  const dynViews = dVerts > 0 && dIdx > 0 && dpPtr && dnPtr && dcPtr && diPtr ? {
    pos: new Float32Array(buf, dpPtr, dVerts * 3),
    norm: new Float32Array(buf, dnPtr, dVerts * 3),
    col: new Float32Array(buf, dcPtr, dVerts * 3),
    idx: new Uint32Array(buf, diPtr, dIdx),
  } : null;

  let instViews = null, imIdx = 0, instCount = 0;
  if (typeof host.verdant_wasm_inst_count === "function") {
    instCount = host.verdant_wasm_inst_count() >>> 0;
    const imVerts = host.verdant_wasm_inst_mesh_vert_count() >>> 0;
    imIdx = host.verdant_wasm_inst_mesh_idx_count() >>> 0;
    const ipPtr = host.verdant_wasm_inst_mesh_pos_ptr() >>> 0;
    const inPtr = host.verdant_wasm_inst_mesh_norm_ptr() >>> 0;
    const icPtr = host.verdant_wasm_inst_mesh_col_ptr() >>> 0;
    const iiPtr = host.verdant_wasm_inst_mesh_idx_ptr() >>> 0;
    const idPtr = host.verdant_wasm_inst_data_ptr() >>> 0;
    if (instCount > 0 && imVerts > 0 && imIdx > 0 && ipPtr && idPtr) {
      instViews = {
        epoch: host.verdant_wasm_inst_epoch() >>> 0,
        instCount,
        pos: new Float32Array(buf, ipPtr, imVerts * 3),
        norm: new Float32Array(buf, inPtr, imVerts * 3),
        col: new Float32Array(buf, icPtr, imVerts * 3),
        idx: new Uint32Array(buf, iiPtr, imIdx),
        data: new Float32Array(buf, idPtr, instCount * 16),
      };
    }
  }

  const tris = Math.floor(sIdx / 3) + Math.floor(dIdx / 3) + (instViews ? Math.floor(imIdx / 3) * instCount : 0);
  return { staticViews, dynViews, instViews, tris };
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
      renderer.setTex?.(1, tp && tw && th ? new Uint8Array(host.memory.buffer, tp, tw * th * 4) : null, tw, th, te);
      if (te !== prevEp && typeof host.verdant_wasm_scene_pbr_tex_ptr === "function") {
        renderer._lastPbrEp = te;
        for (const [ch, slot] of [[1, 3], [2, 4], [3, 5]]) {
          const p = host.verdant_wasm_scene_pbr_tex_ptr(ch) >>> 0;
          const pw = host.verdant_wasm_scene_pbr_tex_width(ch) >>> 0;
          const ph = host.verdant_wasm_scene_pbr_tex_height(ch) >>> 0;
          renderer.setTex?.(slot, p && pw && ph ? new Uint8Array(host.memory.buffer, p, pw * ph * 4) : null, pw, ph, te);
        }
      }
    } else {
      renderer._lastPbrEp = 0;
      renderer.setTex?.(1, null, 0, 0, 0);
    }
    renderer.uploadSplitBuffers(split.staticViews, split.dynViews, split.instViews);
    if (
      customGlbActive &&
      split.staticViews &&
      split.staticViews.idx.length > 24576 &&
      typeof renderer.updateStaticIndices === "function" &&
      typeof host.verdant_wasm_uhd_cull_meshlets === "function" &&
      typeof host.verdant_wasm_uhd_visible_idx_ptr === "function"
    ) {
      const ck = `${split.staticViews.epoch}:${Math.round(eye[0] * 10)},${Math.round(eye[1] * 10)},${Math.round(eye[2] * 10)}:${canvas.height}`;
      if (ck !== renderer._lastCullKey) {
        renderer._lastCullKey = ck;
        host.verdant_wasm_uhd_cull_meshlets(
          eye[0], eye[1], eye[2],
          lookTarget[0], lookTarget[1], lookTarget[2],
          aspect, canvas.height >>> 0, 6,
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
      const w = Math.max(1, msg.width | 0);
      const h = Math.max(1, msg.height | 0);
      if (canvas) {
        canvas.width = w;
        canvas.height = h;
        const rMod = msg.hdRenderer ? await import("./scene-hd-renderer.mjs") : null;
        renderer = rMod ? await rMod.createBestRenderer(canvas) : await createBestRenderer(canvas);
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
      const oracleDrs = typeof uorcWasm?.verdant_wasm_record_frame_time_us === "function"
        ? (uorcWasm.verdant_wasm_record_frame_time_us(frameMicros >>> 0) >>> 0)
        : 0;
      const scalePermille = oracleDrs > 0 ? Math.min(govScale, oracleDrs) : govScale;
      renderer?.setRenderScale?.(scalePermille);
      const upload = renderer?.getUploadTelemetry?.() || { staticBytes: 0, dynBytes: 0, instCount: activeInstancedCount };
      self.postMessage({
        kind: "telemetry",
        tris: stats.tris,
        bvhVisible,
        frameMicros: emaFrameMicros,
        scalePermille,
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
      drawScene(cam.eye, cam.lookTarget, currentLod || { sky: [0.18, 0.30, 0.38], fog: [0.30, 0.40, 0.44] }, 0, false, 0, 0, true);
        const pbrChannels = {
        albedo: (uorcWasm.verdant_wasm_scene_pbr_tex_width?.(0) >>> 0) > 0,
        metallicRoughness: (uorcWasm.verdant_wasm_scene_pbr_tex_width?.(1) >>> 0) > 0,
        normal: (uorcWasm.verdant_wasm_scene_pbr_tex_width?.(2) >>> 0) > 0,
        emissive: (uorcWasm.verdant_wasm_scene_pbr_tex_width?.(3) >>> 0) > 0,
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

    if (msg.kind === "stream-uorc") {
      const targetRank = Math.max(0, Math.min(4, msg.targetRank | 0)) >>> 0;
      uorcWasm ??= await getUorcWasm();
      const streamBytes =
        msg.customStream instanceof Uint8Array && msg.customStream.byteLength > 0
          ? msg.customStream
          : await getCanonicalUorcStream();
      stageIoBytes(uorcWasm, streamBytes);
      const ok = uorcWasm.verdant_wasm_uorc_decode_and_restore(targetRank) === 1;
      if (!ok) {
        self.postMessage({ kind: "uorc-streamed", id: msg.id, ok: false });
        return;
      }
      const vdntBytes = readIoBytes(uorcWasm);
      if (!uorcSceneActive) {
        syncCoreStateToUorc(uorcWasm);
      } else {
        syncOrgansToWasm(uorcWasm);
      }
      walkJournal.length = 0;
      uorcSceneActive = true;
      customGlbActive = false;
      const sh = simHost();
      const codedChecksum = Number(uorcWasm.verdant_wasm_coded_zero_decode_checksum(Math.min(3, targetRank)));
      currentLod = msg.lod || currentLod;
      const cam = readSolvedCamera(sh);
      const stats = drawScene(cam.eye, cam.lookTarget, currentLod, msg.simClockSec || 0, false);
      self.postMessage({
        kind: "uorc-streamed",
        id: msg.id,
        ok: true,
        lodRank: sh.verdant_wasm_active_lod_rank() >>> 0,
        tris: stats.tris,
        streamBytes: streamBytes.byteLength,
        vdntBytes: vdntBytes.byteLength,
        codedChecksum,
      });
      return;
    }

    if (msg.kind === "route-prompt") {
      if (organModes[0] === 2) {
        self.postMessage({ kind: "prompt-routed", id: msg.id, ok: false, organDenied: "wasm-core" });
        return;
      }
      const promptKey = String(msg.prompt || "").trim().toLowerCase();
      const cached = slmKappaCache.get(promptKey);
      if (cached) {
        slmKappaHits++;
        self.postMessage({ ...cached, id: msg.id, elidedHit: true, slmKappaHits });
        return;
      }
      uorcWasm ??= await getUorcWasm();
      syncOrgansToWasm(uorcWasm);
      const raw = new TextEncoder().encode(String(msg.prompt || ""));
      stageIoBytes(uorcWasm, raw);
      const auxLen = uorcWasm.verdant_wasm_route_prompt_io() >>> 0;
      if (auxLen === 0) {
        const organDenied = organModes[8] === 2 ? "broadcast-channel" : null;
        self.postMessage({ kind: "prompt-routed", id: msg.id, ok: false, organDenied });
        return;
      }
      const decoded = new TextDecoder().decode(readAuxBytes(uorcWasm));
      const [slug, scoreStr, logitStr, ...rest] = decoded.split("|");
      const reply = {
        kind: "prompt-routed",
        ok: true,
        slug,
        scoreMilli: Number(scoreStr) || 0,
        rawLogit: Number(logitStr) || 0,
        summary: rest.join("|"),
      };
      if (slmKappaCache.size >= 64) slmKappaCache.clear();
      slmKappaCache.set(promptKey, reply);
      self.postMessage({ ...reply, id: msg.id, elidedHit: false, slmKappaHits });
      return;
    }

    if (msg.kind === "aqm-benchmark") {
      uorcWasm ??= await getUorcWasm();
      const t0 = performance.now();
      const len = uorcWasm.verdant_wasm_aqm_benchmark_aux((msg.entityCount || 16384) >>> 0) >>> 0;
      const elapsedMs = performance.now() - t0;
      if (len === 0) {
        self.postMessage({ kind: "aqm-benchmark", id: msg.id, ok: false });
        return;
      }
      const kvMap = parsePipeKv(new TextDecoder().decode(readAuxBytes(uorcWasm)));
      self.postMessage({
        kind: "aqm-benchmark",
        id: msg.id,
        ok: true,
        elapsedMs,
        metrics: kvMap,
        slmKappaHits,
      });
      return;
    }

    if (msg.kind === "stability-audit") {
      uorcWasm ??= await getUorcWasm();
      const t0 = performance.now();
      const len = uorcWasm.verdant_wasm_stability_audit_aux((msg.bodyCount || 512) >>> 0) >>> 0;
      const elapsedMs = performance.now() - t0;
      if (len === 0) {
        self.postMessage({ kind: "stability-audit", id: msg.id, ok: false });
        return;
      }
      const kvMap = parsePipeKv(new TextDecoder().decode(readAuxBytes(uorcWasm)));
      self.postMessage({ kind: "stability-audit", id: msg.id, ok: true, elapsedMs, metrics: kvMap });
      return;
    }

    if (msg.kind === "uorc-exclusive-audit") {
      uorcWasm ??= await getUorcWasm();
      const t0 = performance.now();
      const len = uorcWasm.verdant_wasm_uorc_exclusive_audit_aux() >>> 0;
      const elapsedMs = performance.now() - t0;
      if (len === 0) {
        self.postMessage({ kind: "uorc-exclusive-audit", id: msg.id, ok: false });
        return;
      }
      const kvMap = parsePipeKv(new TextDecoder().decode(readAuxBytes(uorcWasm)));
      self.postMessage({ kind: "uorc-exclusive-audit", id: msg.id, ok: true, elapsedMs, metrics: kvMap });
      return;
    }

    if (msg.kind === "verify-ui") {
      uorcWasm ??= await getUorcWasm();
      const count = uorcWasm.verdant_wasm_verify_ui((msg.width || 1280) >>> 0, (msg.height || 900) >>> 0) >>> 0;
      const report = new TextDecoder().decode(readAuxBytes(uorcWasm));
      self.postMessage({
        kind: "ui-verified",
        id: msg.id,
        ok: count > 0,
        widgetCount: count,
        report,
      });
      return;
    }

    if (msg.kind === "set-organ-grant") {
      uorcWasm ??= await getUorcWasm();
      const idx = Math.max(0, Math.min(10, msg.organIdx >>> 0));
      const reqMode = msg.mode >>> 0;
      if (reqMode === 0) {
        organModes[idx] = 0;
      } else if (reqMode === 2) {
        organModes[idx] = 2;
      } else if (reqMode === 1 && organModes[idx] !== 2) {
        organModes[idx] = 1;
      }
      slmKappaCache.clear();
      const allowed = syncOrgansToWasm(uorcWasm, idx);
      self.postMessage({
        kind: "organ-grant-updated",
        id: msg.id,
        ok: true,
        organIdx: idx,
        mode: organModes[idx],
        allowed,
      });
      return;
    }

    if (msg.kind === "stitch-shard") {
      uorcWasm ??= await getUorcWasm();
      let source = msg.bytes instanceof Uint8Array ? msg.bytes : null;
      if (!source || !source.byteLength) {
        uorcWasm.verdant_wasm_set_lod_rank(Math.min(3, simHost().verdant_wasm_active_lod_rank() >>> 0));
        const snapLen = uorcWasm.verdant_wasm_snapshot_encode() >>> 0;
        if (snapLen === 0) {
          self.postMessage({ kind: "stitch-sharded", id: msg.id, ok: false });
          return;
        }
        source = readIoBytes(uorcWasm);
      }
      stageIoBytes(uorcWasm, source);
      const shards = [];
      for (let s = 0; s < 3; s++) {
        const len = uorcWasm.verdant_wasm_stitch_shard_io(s) >>> 0;
        if (len === 0) {
          self.postMessage({ kind: "stitch-sharded", id: msg.id, ok: false });
          return;
        }
        shards.push(readAuxBytes(uorcWasm));
      }
      self.postMessage(
        {
          kind: "stitch-sharded",
          id: msg.id,
          ok: true,
          sourceBytes: source.byteLength,
          shards,
        },
        shards.map((u) => u.buffer),
      );
      return;
    }

    if (msg.kind === "stitch-recover") {
      uorcWasm ??= await getUorcWasm();
      const a = msg.shardA instanceof Uint8Array ? msg.shardA : new Uint8Array(msg.shardA || []);
      const b = msg.shardB instanceof Uint8Array ? msg.shardB : new Uint8Array(msg.shardB || []);
      const combined = new Uint8Array(a.byteLength + b.byteLength);
      combined.set(a, 0);
      combined.set(b, a.byteLength);
      stageIoBytes(uorcWasm, combined);
      const recLen = uorcWasm.verdant_wasm_stitch_recover_io(a.byteLength >>> 0) >>> 0;
      if (recLen === 0) {
        self.postMessage({ kind: "stitch-recovered", id: msg.id, ok: false });
        return;
      }
      const recovered = readAuxBytes(uorcWasm);
      self.postMessage(
        { kind: "stitch-recovered", id: msg.id, ok: true, recovered },
        [recovered.buffer],
      );
      return;
    }

    if (msg.kind === "scene-outliner") {
      uorcWasm ??= await getUorcWasm();
      if (!customGlbActive && !uorcSceneActive) {
        uorcWasm.verdant_wasm_set_lod_rank(simHost().verdant_wasm_active_lod_rank() >>> 0);
      }
      const len = uorcWasm.verdant_wasm_scene_outliner_aux((msg.observedFrameMicros || 8200) >>> 0) >>> 0;
      if (len === 0) {
        self.postMessage({ kind: "scene-outliner", id: msg.id, ok: false });
        return;
      }
      const lines = new TextDecoder().decode(readAuxBytes(uorcWasm)).split("\n");
      const header = parsePipeKv(lines[0] || "");
      const entities = lines.slice(1).filter(Boolean).map((line) => {
        const [id, name, role, tris, coords, minLodScale] = line.split("|");
        const [cx, cy, cz] = (coords || "0,0,0").split(",").map(Number);
        return {
          id: Number(id) || 0,
          name: name || "Entity",
          role: role || "StaticArchitecture",
          tris: Number(tris) || 0,
          center: [cx || 0, cy || 0, cz || 0],
          minLodScale: Number(minLodScale) || 0,
        };
      });
      const sh = simHost();
      self.postMessage({
        kind: "scene-outliner",
        id: msg.id,
        ok: true,
        header,
        entities,
        camera: readSolvedCamera(sh),
        universeCell: readUniverseCell(sh),
      });
      return;
    }

    if (msg.kind === "set-seed") {
      const nextSeed = BigInt(Math.max(1, Number(msg.seed) || 7));
      activeSeed = nextSeed;
      walkJournal.length = 0;
      const rank = simHost().verdant_wasm_active_lod_rank() >>> 0;
      wasm.verdant_wasm_init(nextSeed);
      wasm.verdant_wasm_set_lod_rank(rank);
      if (uorcWasm) {
        uorcWasm.verdant_wasm_init(nextSeed);
        uorcWasm.verdant_wasm_set_lod_rank(rank);
        syncOrgansToWasm(uorcWasm);
      }
      customGlbActive = false;
      const sh = simHost();
      if (lastAnchorActive) {
        sh.verdant_wasm_camera_set_anchor(1, lastAnchor[0], lastAnchor[1], lastAnchor[2]);
      } else {
        sh.verdant_wasm_camera_set_mode(wasm.verdant_wasm_camera_mode() >>> 0);
      }
      sh.verdant_wasm_camera_set_pose_intent(
        lastPose.yaw,
        lastPose.pitch,
        lastPose.distMm,
        lastPose.aspectMilli,
      );
      sh.verdant_wasm_scene_rebuild_buffers(0);
      const cam = readSolvedCamera(sh);
      const stats = drawScene(cam.eye, cam.lookTarget, currentLod, 0, false);
      self.postMessage({
        kind: "seed-updated",
        id: msg.id,
        ok: true,
        seed: Number(nextSeed),
        tris: stats.tris,
        camera: cam,
      });
      return;
    }

    if (msg.kind === "sabine-rt60") {
      const vol = Math.max(10, Math.round(Number(msg.volumeM3) || 1800)) >>> 0;
      const absMilli = Math.max(100, Math.round(Number(msg.absorptionMilliM2) || 96000)) >>> 0;
      const rt60Ms = wasm.verdant_wasm_sabine_rt60_millis(vol, absMilli) >>> 0;
      self.postMessage({
        kind: "sabine-rt60",
        id: msg.id,
        ok: true,
        volumeM3: vol,
        absorptionMilliM2: absMilli,
        rt60Ms,
      });
      return;
    }

    if (msg.kind === "load-domain-pack") {
      const domainIdx = Math.max(0, Math.min(3, msg.domainIdx | 0)) >>> 0;
      await ensureUorcSceneHost();
      const len = uorcWasm.verdant_wasm_load_domain_pack(domainIdx) >>> 0;
      if (len === 0) {
        self.postMessage({ kind: "domain-pack-loaded", id: msg.id, ok: false });
        return;
      }
      const observer = JSON.parse(new TextDecoder().decode(readAuxBytes(uorcWasm)));
      customGlbActive = false;
      uorcSceneActive = true;
      walkJournal.length = 0;
      activeSeed = BigInt(uorcWasm.verdant_wasm_active_seed());
      if (msg.lod) currentLod = msg.lod;
      const sh = simHost();
      const cam = readSolvedCamera(sh);
      lastAnchor = [cam.lookTarget[0], cam.lookTarget[1], cam.lookTarget[2]];
      lastAnchorActive = true;
      const dx = cam.eye[0] - cam.lookTarget[0];
      const dy = cam.eye[1] - cam.lookTarget[1];
      const dz = cam.eye[2] - cam.lookTarget[2];
      const distM = Math.max(1.0, Math.hypot(dx, dy, dz));
      lastPose = {
        yaw: Math.round(Math.atan2(dx, dz) * 1000),
        pitch: Math.round(Math.asin(Math.max(-0.95, Math.min(0.95, dy / distM))) * 1000),
        distMm: Math.round(distM * 1000) >>> 0,
        aspectMilli: lastPose?.aspectMilli || 1600,
      };
      const stats = drawScene(cam.eye, cam.lookTarget, currentLod, msg.simClockSec || 0, false, 0, 0, true);
      self.postMessage({
        kind: "domain-pack-loaded",
        id: msg.id,
        ok: true,
        domainIdx,
        seed: Number(activeSeed),
        tris: stats.tris,
        walkablePolys: sh.verdant_wasm_navmesh_polygon_count() >>> 0,
        camera: cam,
        observer,
      });
      return;
    }

    if (msg.kind === "set-npc-directive") {
      const npcIdx = Math.max(0, msg.npcIdx | 0) >>> 0;
      const directiveIdx = Math.max(0, msg.directiveIdx | 0) >>> 0;
      await ensureUorcSceneHost();
      const len = uorcWasm.verdant_wasm_set_npc_directive(npcIdx, directiveIdx) >>> 0;
      if (len === 0) {
        self.postMessage({ kind: "npc-directive-updated", id: msg.id, ok: false });
        return;
      }
      const [characterId, directiveLabel, wpStr] = new TextDecoder().decode(readAuxBytes(uorcWasm)).split("|");
      const sh = simHost();
      sh.verdant_wasm_scene_rebuild_buffers(msg.simClockSec || 0);
      const cam = readSolvedCamera(sh);
      const stats = drawScene(cam.eye, cam.lookTarget, currentLod, msg.simClockSec || 0, false);
      self.postMessage({
        kind: "npc-directive-updated",
        id: msg.id,
        ok: true,
        npcIdx,
        directiveIdx,
        characterId: characterId || "",
        directiveLabel: directiveLabel || "",
        waypointCount: Number(wpStr) || 0,
        tris: stats.tris,
      });
      return;
    }

    if (msg.kind === "npc-dialogue") {
      if (organModes[0] === 2) {
        self.postMessage({ kind: "npc-dialogue-reply", id: msg.id, ok: false, organDenied: "wasm-core" });
        return;
      }
      if (organModes[7] === 2) {
        self.postMessage({ kind: "npc-dialogue-reply", id: msg.id, ok: false, organDenied: "webcrypto-prism-id" });
        return;
      }
      const npcIdx = Math.max(0, msg.npcIdx | 0) >>> 0;
      const utterance = String(msg.utterance || "").trim();
      if (!utterance) {
        self.postMessage({ kind: "npc-dialogue-reply", id: msg.id, ok: false });
        return;
      }
      await ensureUorcSceneHost();
      stageIoBytes(uorcWasm, new TextEncoder().encode(utterance));
      const len = uorcWasm.verdant_wasm_dialogue_turn_io(npcIdx) >>> 0;
      if (len === 0) {
        self.postMessage({ kind: "npc-dialogue-reply", id: msg.id, ok: false });
        return;
      }
      const turn = JSON.parse(new TextDecoder().decode(readAuxBytes(uorcWasm)));
      const sh = simHost();
      if (turn.mutatedDirectiveIndex != null && turn.mutatedDirectiveIndex >= 0) {
        sh.verdant_wasm_scene_rebuild_buffers(msg.simClockSec || 0);
        const cam = readSolvedCamera(sh);
        drawScene(cam.eye, cam.lookTarget, currentLod, msg.simClockSec || 0, false);
      }
      self.postMessage({ kind: "npc-dialogue-reply", id: msg.id, ok: true, turn });
      return;
    }

    if (msg.kind === "observer-status") {
      await ensureUorcSceneHost();
      const len = uorcWasm.verdant_wasm_observer_status_aux() >>> 0;
      if (len === 0) {
        self.postMessage({ kind: "observer-status", id: msg.id, ok: false });
        return;
      }
      const observer = JSON.parse(new TextDecoder().decode(readAuxBytes(uorcWasm)));
      self.postMessage({ kind: "observer-status", id: msg.id, ok: true, observer });
      return;
    }

    if (msg.kind === "observe-comm") {
      const channelCode = Math.max(0, Math.min(4, msg.channelCode | 0)) >>> 0;
      const wire = `${msg.sender || "urn:verdant:organ:engine"}|${msg.recipient || "urn:verdant:organ:studio"}|${msg.message || ""}`;
      await ensureUorcSceneHost();
      stageIoBytes(uorcWasm, new TextEncoder().encode(wire));
      const len = uorcWasm.verdant_wasm_observe_comm_io(channelCode) >>> 0;
      const kappaHex = len > 0 ? new TextDecoder().decode(readAuxBytes(uorcWasm)) : "";
      self.postMessage({ kind: "comm-observed", id: msg.id, ok: len > 0, kappaHex });
      return;
    }

    if (msg.kind === "import-project-model") {
      await ensureUorcSceneHost();
      const jsonBytes = new TextEncoder().encode(JSON.stringify(msg.request || {}));
      stageIoBytes(uorcWasm, jsonBytes);
      const len = uorcWasm.verdant_wasm_import_project_model_io() >>> 0;
      if (len === 0) {
        self.postMessage({ kind: "project-model-imported", id: msg.id, ok: false });
        return;
      }
      const model = JSON.parse(new TextDecoder().decode(readAuxBytes(uorcWasm)));
      self.postMessage({ kind: "project-model-imported", id: msg.id, ok: true, model });
      return;
    }

    if (msg.kind === "edit-project-model") {
      await ensureUorcSceneHost();
      const jsonBytes = new TextEncoder().encode(JSON.stringify(msg.request || {}));
      stageIoBytes(uorcWasm, jsonBytes);
      const len = uorcWasm.verdant_wasm_edit_project_model_io() >>> 0;
      if (len === 0) {
        self.postMessage({ kind: "project-model-edited", id: msg.id, ok: false });
        return;
      }
      const model = JSON.parse(new TextDecoder().decode(readAuxBytes(uorcWasm)));
      self.postMessage({ kind: "project-model-edited", id: msg.id, ok: true, model });
      return;
    }

    if (msg.kind === "revert-project-model") {
      await ensureUorcSceneHost();
      stageIoBytes(uorcWasm, new TextEncoder().encode(String(msg.modelId || "")));
      const targetVer = Math.max(1, msg.targetVersion | 0) >>> 0;
      const len = uorcWasm.verdant_wasm_revert_project_model_io(targetVer) >>> 0;
      if (len === 0) {
        self.postMessage({ kind: "project-model-reverted", id: msg.id, ok: false });
        return;
      }
      const model = JSON.parse(new TextDecoder().decode(readAuxBytes(uorcWasm)));
      self.postMessage({ kind: "project-model-reverted", id: msg.id, ok: true, model });
      return;
    }

    if (msg.kind === "remove-project-model") {
      await ensureUorcSceneHost();
      stageIoBytes(uorcWasm, new TextEncoder().encode(String(msg.modelId || "")));
      const len = uorcWasm.verdant_wasm_remove_project_model_io() >>> 0;
      const kappaHex = len > 0 ? new TextDecoder().decode(readAuxBytes(uorcWasm)) : "";
      self.postMessage({ kind: "project-model-removed", id: msg.id, ok: len > 0, kappaHex });
      return;
    }

    if (msg.kind === "list-project-models") {
      await ensureUorcSceneHost();
      const len = uorcWasm.verdant_wasm_list_project_models_aux() >>> 0;
      if (len === 0) {
        self.postMessage({ kind: "project-models-listed", id: msg.id, ok: false, models: [] });
        return;
      }
      const payload = JSON.parse(new TextDecoder().decode(readAuxBytes(uorcWasm)));
      self.postMessage({ kind: "project-models-listed", id: msg.id, ok: true, ...payload });
      return;
    }

    if (msg.kind === "compose-glb-append") {
      const raw = msg.bytes instanceof Uint8Array ? msg.bytes : new Uint8Array(msg.bytes || []);
      uorcWasm ??= await getUorcWasm();
      if (!raw.byteLength || !stageIoBytes(uorcWasm, raw)) {
        self.postMessage({ kind: "glb-composed", id: msg.id, ok: false });
        return;
      }
      const tris = uorcWasm.verdant_wasm_glb_compose_append_io() >>> 0;
      if (!tris) {
        self.postMessage({ kind: "glb-composed", id: msg.id, ok: false });
        return;
      }
      customGlbActive = true;
      const cam = readSolvedCamera(simHost());
      lastAnchor = [...cam.lookTarget]; lastAnchorActive = true;
      drawScene(cam.eye, cam.lookTarget, currentLod, 0, false, 0, 0, true);
      self.postMessage({ kind: "glb-composed", id: msg.id, ok: true, tris, camera: cam });
      return;
    }

    if (msg.kind === "set-env-sky") {
      renderer?.setTex?.(2, msg.bitmap || null);
      const sh = simHost();
      const cam = readSolvedCamera(sh);
      drawScene(cam.eye, cam.lookTarget, currentLod, 0, false, 0, 0, true);
      let cognition = null;
      if (uorcWasm && typeof uorcWasm.verdant_wasm_record_part_hydration_io === "function") {
        const wire = msg.bitmap
          ? `stage-environment|polyhaven|${msg.label || "Poly Haven HDRI Sky Dome"}`
          : "stage-environment|verdant-prism|Procedural Atmosphere";
        stageIoBytes(uorcWasm, new TextEncoder().encode(wire));
        const len = uorcWasm.verdant_wasm_record_part_hydration_io(msg.bitmap ? 4 : 1) >>> 0;
        if (len) cognition = JSON.parse(new TextDecoder().decode(readAuxBytes(uorcWasm)));
      }
      self.postMessage({ kind: "env-sky-set", id: msg.id, ok: true, active: Boolean(msg.bitmap), cognition });
      return;
    }

    if (msg.kind === "update-host-envelope") {
      await ensureUorcSceneHost();
      const vw = (msg.viewportWidth || canvas?.width || 1920) >>> 0;
      const vh = (msg.viewportHeight || canvas?.height || 1080) >>> 0;
      const len = typeof uorcWasm.verdant_wasm_update_host_envelope_uhd === "function"
        ? (uorcWasm.verdant_wasm_update_host_envelope_uhd(
            (msg.maxTex || renderer?.maxTextureSize || 2048) >>> 0,
            renderer?.isSoftware ? 1 : 0,
            (msg.threads || 4) >>> 0,
            (msg.frameMicros || emaFrameMicros || 8000) >>> 0,
            vw,
            vh,
          ) >>> 0)
        : (uorcWasm.verdant_wasm_update_host_envelope(
            (msg.maxTex || renderer?.maxTextureSize || 2048) >>> 0,
            renderer?.isSoftware ? 1 : 0,
            (msg.threads || 4) >>> 0,
            (msg.frameMicros || emaFrameMicros || 8000) >>> 0,
          ) >>> 0);
      const cognition = len ? JSON.parse(new TextDecoder().decode(readAuxBytes(uorcWasm))) : null;
      if (cognition?.hostEnvelope?.recommendedRenderScalePermille) {
        renderer?.setRenderScale?.(cognition.hostEnvelope.recommendedRenderScalePermille);
      }
      self.postMessage({ kind: "cognitive-resolution", id: msg.id, ok: Boolean(cognition), cognition });
      return;
    }

    if (msg.kind === "uhd-profile") {
      await ensureUorcSceneHost();
      let webgpuHardware = null;
      if (typeof navigator !== "undefined" && navigator.gpu && !renderer?.isSoftware) {
        try {
          const wgpu = await import("./scene-webgpu-renderer.mjs");
          webgpuHardware = await wgpu.probeWebGpuHardware();
        } catch {}
      }
      const webgpuAvail = Boolean(webgpuHardware);
      const pLen = typeof uorcWasm.verdant_wasm_uhd_profile_aux === "function"
        ? (uorcWasm.verdant_wasm_uhd_profile_aux(
            (msg.widthPx || 3840) >>> 0,
            (msg.heightPx || 2160) >>> 0,
            renderer?.isSoftware ? 1 : 0,
            webgpuAvail ? 1 : 0,
            (msg.frameMicros || emaFrameMicros || 7500) >>> 0,
          ) >>> 0)
        : 0;
      const profile = pLen ? JSON.parse(new TextDecoder().decode(readAuxBytes(uorcWasm))) : null;
      const sh = simHost();
      const cam = readSolvedCamera(sh);
      let meshlets = null;
      if (typeof uorcWasm.verdant_wasm_uhd_cull_meshlets === "function") {
        const aspect = (msg.widthPx || 3840) / Math.max(1, msg.heightPx || 2160);
        const mLen = uorcWasm.verdant_wasm_uhd_cull_meshlets(
          cam.eye[0], cam.eye[1], cam.eye[2],
          cam.lookTarget[0], cam.lookTarget[1], cam.lookTarget[2],
          aspect, (msg.heightPx || 2160) >>> 0, (profile?.meshletSseTenthsPx || 10) >>> 0,
        ) >>> 0;
        if (mLen) meshlets = JSON.parse(new TextDecoder().decode(readAuxBytes(uorcWasm)));
      }
      let stability = null;
      if (typeof uorcWasm.verdant_wasm_frame_stability_aux === "function") {
        const sLen = uorcWasm.verdant_wasm_frame_stability_aux() >>> 0;
        if (sLen) stability = JSON.parse(new TextDecoder().decode(readAuxBytes(uorcWasm)));
      }
      let lodPyramid = null;
      if (msg.includeLodPyramid && typeof uorcWasm.verdant_wasm_uhd_lod_pyramid_aux === "function") {
        const lLen = uorcWasm.verdant_wasm_uhd_lod_pyramid_aux() >>> 0;
        if (lLen) lodPyramid = JSON.parse(new TextDecoder().decode(readAuxBytes(uorcWasm)));
      }
      self.postMessage({
        kind: "uhd-profile-ready",
        id: msg.id,
        ok: Boolean(profile),
        profile,
        meshlets,
        stability,
        lodPyramid,
        webgpuAvail,
        webgpuLimits: webgpuHardware
          ? { maxTextureDimension2D: webgpuHardware.maxTextureDimension2D, maxStorageBufferBindingSize: webgpuHardware.maxStorageBufferBindingSize }
          : null,
      });
      return;
    }

    if (msg.kind === "cognitive-resolution") {
      await ensureUorcSceneHost();
      const len = uorcWasm.verdant_wasm_cognitive_resolution_aux() >>> 0;
      const cognition = len ? JSON.parse(new TextDecoder().decode(readAuxBytes(uorcWasm))) : null;
      self.postMessage({ kind: "cognitive-resolution", id: msg.id, ok: Boolean(cognition), cognition });
      return;
    }

    if (msg.kind === "register-stream" || msg.kind === "remove-stream" || msg.kind === "toggle-stream" || msg.kind === "ingest-knowledge") {
      await ensureUorcSceneHost();
      const payloadStr = msg.kind === "remove-stream" || msg.kind === "toggle-stream"
        ? String(msg.streamId || "")
        : JSON.stringify(msg.request || {});
      stageIoBytes(uorcWasm, new TextEncoder().encode(payloadStr));
      const len = (
        msg.kind === "register-stream" ? uorcWasm.verdant_wasm_register_stream_io()
        : msg.kind === "remove-stream" ? uorcWasm.verdant_wasm_remove_stream_io()
        : msg.kind === "toggle-stream" ? uorcWasm.verdant_wasm_toggle_stream_io(msg.enabled ? 1 : 0)
        : uorcWasm.verdant_wasm_ingest_knowledge_io()
      ) >>> 0;
      const text = len ? new TextDecoder().decode(readAuxBytes(uorcWasm)) : "";
      let result = text;
      if (len && msg.kind !== "remove-stream") { try { result = JSON.parse(text); } catch {} }
      const cLen = uorcWasm.verdant_wasm_cognitive_resolution_aux() >>> 0;
      const cognition = cLen ? JSON.parse(new TextDecoder().decode(readAuxBytes(uorcWasm))) : null;
      self.postMessage({ kind: `${msg.kind}-done`, id: msg.id, ok: len > 0, result, cognition });
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
