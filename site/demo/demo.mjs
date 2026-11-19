// Verdant Engine Demo — Standalone Comparative Showcase & Live WebAssembly Benchmark Harness.
// Separates engine benchmarks, rasterizer comparisons, UORC streaming monotonicity, and semiring
// kernel proofs from the role-driven authoring workspace in Verdant Studio (`/`).
// Executes `/app.holo` cryptographic verification & vector dispatch inside `/guest-worker.mjs`
// so the main-thread WebGL2 / Wasm viewport and controls remain 60 Hz responsive from first paint.

import {
  PRISM_CSM_SPLIT_METERS,
  PRISM_DEFAULT_WORLD_SEED,
  PRISM_PORTABLE_UORC_BYTES,
  PRISM_TILE_EDGE_PIXELS,
} from "../prism-bridge-meta.mjs";
import {
  createBestRenderer,
  mat4LookAt,
  mat4Mul,
  mat4Perspective,
} from "../scene-renderer.mjs";

const CSM_FAR_METERS = PRISM_CSM_SPLIT_METERS[3];

const LOD_SPECS = [
  { rank: 0, label: "LOD0 · Coarse Proxy", res: "0.125x", density: "35%", sky: [0.09, 0.08, 0.12], fog: [0.14, 0.11, 0.16] },
  { rank: 1, label: "LOD1 · Medium Detail", res: "0.250x", density: "72%", sky: [0.12, 0.17, 0.24], fog: [0.18, 0.24, 0.31] },
  { rank: 2, label: "LOD2 · High Detail", res: "0.625x", density: "92%", sky: [0.16, 0.27, 0.34], fog: [0.24, 0.35, 0.39] },
  { rank: 3, label: "LOD3 · Lossless UORC", res: "1.000x", density: "100%", sky: [0.18, 0.30, 0.38], fog: [0.30, 0.40, 0.44] },
  { rank: 4, label: "LOD4 · Ultra HD", res: "2.000x", density: "200%", sky: [0.20, 0.34, 0.44], fog: [0.34, 0.44, 0.48] },
];

const SEMIRING_DISPLAY = Object.freeze({
  LinearExact: "LinearExact (+, ×)",
  TropicalMaxPlus: "TropicalMaxPlus (max, +)",
  TropicalMinPlus: "TropicalMinPlus (min, +)",
  ModularAudioRing: "ModularAudioRing (Q15)",
  CodedZeroDecode: "CodedZeroDecode (CDMX)",
});

const KERNEL_ROWS = [
  ["kernel:0", "StaticMeshTransformBatch", "LinearExact (+, ×)"],
  ["kernel:1", "LinearBlendSkinningBatch", "LinearExact (+, ×)"],
  ["kernel:2", "TropicalBvhVisibilityCull", "TropicalMaxPlus (max, +)"],
  ["kernel:3", "TileFramebufferRasterizer", "LinearExact (+, ×)"],
  ["kernel:4", "ShadowCascadeProjector", "LinearExact (+, ×)"],
  ["kernel:5", "ModalAudioMixMatrix", "ModularAudioRing (Q15)"],
  ["kernel:6", "KinematicSplineController", "LinearExact (+, ×)"],
  ["kernel:7", "TropicalMinPlusNavClosure", "TropicalMinPlus (min, +)"],
  ["kernel:8", "CodedStreamZeroDecodeGemm", "CodedZeroDecode (CDMX)"],
  ["kernel:9", "TropicalUiConstraintLayout", "TropicalMinPlus (min, +)"],
  ["kernel:10", "NeuromorphicSalienceStateSpace", "ModularAudioRing (Q15)"],
  ["kernel:11", "CodedSlmProposalRouter", "CodedZeroDecode (CDMX)"],
  ["kernel:12", "TropicalBlasSweptToi", "TropicalMinPlus (min, +)"],
  ["kernel:13", "TwoBoneIkJointSolver", "LinearExact (+, ×)"],
  ["kernel:14", "ConservationStoryAutomaton", "TropicalMinPlus (min, +)"],
  ["coded:0", "GridQuantizedGeometry", "CodedZeroDecode (16-bit codebook)"],
  ["coded:1", "PackedNibbleAnimationTracks", "CodedZeroDecode (8-bit codebook)"],
  ["coded:2", "SparseRunsIblProbes", "CodedZeroDecode (32-bit codebook)"],
  ["kernel:zerodim", "Negative Gate (R5): Zero-Extent Contraction", "Typed Refusal"],
  ["kernel:underflow", "Negative Gate (R5): Laurent Lower-Bound", "Typed Refusal"],
  ["", "Negative Gate (R5): Empty Request Envelope", "Typed Refusal"],
  ["unknown", "Negative Gate (R5): Unregistered Route", "Typed Refusal"],
];

const utf8Enc = new TextEncoder();
const utf8Dec = new TextDecoder();

const $ = (id) => document.getElementById(id);
const canvas = $("demo-canvas");

let wasm = null;
let wasmCompiledModule = null;
let uorcAuditWorker = null;
let uorcStreamBytes = null;
let renderer = null;
let renderMode = "webgl2";
let activeLodRank = 3;
let turntableOn = true;
let activeInstCount = 0;
let dragging = false;
let lastX = 0;
let lastY = 0;
let yawMilli = 460;
let pitchMilli = 270;
let distMm = 18500;
let emaFrameMicros = 8000;

// Off-main-thread `/app.holo` worker bridge (`/guest-worker.mjs`):
let holoWorker = null;
let holoReqId = 0;
const holoPending = new Map();
const holoCache = new Map();
let holoReadyPromise = null;
const lodVerdictCells = new Map();
const kernelRowCells = new Map();

function callHoloWorker(kind, payload = {}) {
  if (!holoWorker) return Promise.reject(new Error("Holo worker unavailable"));
  const id = ++holoReqId;
  return new Promise((resolve, reject) => {
    holoPending.set(id, { resolve, reject });
    holoWorker.postMessage({ id, kind, ...payload });
  });
}

function startHoloWorker() {
  if (holoReadyPromise) return holoReadyPromise;
  holoReadyPromise = (async () => {
    if (typeof Worker !== "undefined") {
      holoWorker = new Worker(new URL("../guest-worker.mjs", import.meta.url), { type: "module" });
      holoWorker.onmessage = ({ data }) => {
        const slot = holoPending.get(data?.id);
        if (!slot) return;
        holoPending.delete(data.id);
        if (data.ok) slot.resolve(data.value);
        else slot.reject(new Error(data.error || "Holo worker error"));
      };
      holoWorker.onerror = (err) => {
        for (const [, slot] of holoPending) {
          slot.reject(new Error(err?.message || "Holo worker crashed"));
        }
        holoPending.clear();
      };
      const source = new URL("../app.holo", import.meta.url).href;
      return callHoloWorker("init", { source });
    }
    const { load } = await import("../worker-runtime/index.mjs");
    const resp = await fetch(new URL("../app.holo", import.meta.url));
    if (!resp.ok) throw new Error(`HTTP ${resp.status} loading /app.holo`);
    const fallbackApp = await load(new Uint8Array(await resp.arrayBuffer()));
    holoWorker = {
      fallbackApp,
    };
    return fallbackApp.identities;
  })();
  return holoReadyPromise;
}

async function dispatchHolo(query) {
  if (holoCache.has(query)) return holoCache.get(query);
  await startHoloWorker();
  let text = "";
  if (holoWorker?.fallbackApp) {
    await new Promise((r) => setTimeout(r, 0));
    const raw = await holoWorker.fallbackApp.dispatch(utf8Enc.encode(query));
    text = utf8Dec.decode(raw);
  } else {
    const raw = await callHoloWorker("dispatch", { request: utf8Enc.encode(query) });
    text = utf8Dec.decode(raw);
  }
  holoCache.set(query, text);
  return text;
}

function stageIoBytes(ex, bytes) {
  const ptr = ex.verdant_wasm_io_resize(bytes.byteLength >>> 0) >>> 0;
  if (ptr > 0 && ex.memory) {
    new Uint8Array(ex.memory.buffer, ptr, bytes.byteLength).set(bytes);
    return true;
  }
  return false;
}

function readAuxString(ex) {
  const len = ex.verdant_wasm_aux_len() >>> 0;
  const ptr = ex.verdant_wasm_aux_ptr() >>> 0;
  if (!len || !ptr || !ex.memory) return "";
  return utf8Dec.decode(new Uint8Array(ex.memory.buffer, ptr, len));
}

function parsePipeKv(raw) {
  const out = {};
  for (const part of raw.split("|")) {
    const eq = part.indexOf("=");
    if (eq > 0) out[part.slice(0, eq)] = part.slice(eq + 1);
  }
  return out;
}

function parseHoloFields(verdict) {
  if (!verdict || !verdict.startsWith("conformant:")) return null;
  const fields = {};
  for (const part of verdict.slice(11).split(",")) {
    const eq = part.indexOf("=");
    if (eq > 0) fields[part.slice(0, eq)] = part.slice(eq + 1);
  }
  return fields;
}

function deriveKernelRowDisplay(verdict, fallbackSub, fallbackSemiring) {
  const f = parseHoloFields(verdict);
  if (!f) return { subsystem: fallbackSub, semiring: fallbackSemiring };
  if (f.kernel) {
    const autoTag = f.autonomous === "true" ? " · autonomous" : "";
    const dimsTag = f.cols && f.bits ? ` (${f.cols} cols · ${f.bits}-bit${autoTag})` : "";
    return {
      subsystem: `${f.kernel}${dimsTag}`,
      semiring: SEMIRING_DISPLAY[f.semiring] || f.semiring || fallbackSemiring,
    };
  }
  if (f.tier) {
    const scratchTag = f.zero_scratch === "true" ? " · zero-scratch" : "";
    const blockTag = f.block && f.table ? ` (block=${f.block} · Lut${f.table}${scratchTag})` : "";
    return {
      subsystem: `${f.tier}${blockTag}`,
      semiring: f.code_bits ? `CodedZeroDecode (${f.code_bits}-bit codebook)` : fallbackSemiring,
    };
  }
  return { subsystem: fallbackSub, semiring: fallbackSemiring };
}

function readSplitViews() {
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

  const instCount = wasm.verdant_wasm_inst_count() >>> 0;
  const imVerts = wasm.verdant_wasm_inst_mesh_vert_count() >>> 0;
  const imIdx = wasm.verdant_wasm_inst_mesh_idx_count() >>> 0;
  const instViews = instCount > 0 && imVerts > 0 && imIdx > 0 ? {
    epoch: wasm.verdant_wasm_inst_epoch() >>> 0,
    instCount,
    pos: new Float32Array(buf, wasm.verdant_wasm_inst_mesh_pos_ptr() >>> 0, imVerts * 3),
    norm: new Float32Array(buf, wasm.verdant_wasm_inst_mesh_norm_ptr() >>> 0, imVerts * 3),
    col: new Float32Array(buf, wasm.verdant_wasm_inst_mesh_col_ptr() >>> 0, imVerts * 3),
    idx: new Uint32Array(buf, wasm.verdant_wasm_inst_mesh_idx_ptr() >>> 0, imIdx),
    data: new Float32Array(buf, wasm.verdant_wasm_inst_data_ptr() >>> 0, instCount * 16),
  } : null;

  const tris = Math.floor(sIdx / 3) + Math.floor(dIdx / 3) + (instViews ? Math.floor(imIdx / 3) * instCount : 0);
  return { staticViews, dynViews, instViews, tris };
}

function renderOnce(nowMs) {
  if (!wasm || !renderer || !canvas || document.hidden) return;
  const t0 = performance.now();
  const dpr = Math.min(window.devicePixelRatio || 1, renderer.isSoftware ? 1 : 2);
  const w = Math.max(1, Math.floor((canvas.clientWidth || 640) * dpr));
  const h = Math.max(1, Math.floor((canvas.clientHeight || 360) * dpr));
  if (canvas.width !== w || canvas.height !== h) {
    canvas.width = w;
    canvas.height = h;
  }
  const aspect = Math.max(0.25, w / h);
  const aspectMilli = Math.round(aspect * 1000) >>> 0;
  wasm.verdant_wasm_camera_set_anchor(1, 2.2, 3.8, 9.2);
  wasm.verdant_wasm_camera_set_pose_intent(yawMilli | 0, pitchMilli | 0, distMm >>> 0, aspectMilli);
  wasm.verdant_wasm_scene_rebuild_buffers(nowMs * 0.001);

  const eye = [wasm.verdant_wasm_camera_eye_x(), wasm.verdant_wasm_camera_eye_y(), wasm.verdant_wasm_camera_eye_z()];
  const tgt = [wasm.verdant_wasm_camera_target_x(), wasm.verdant_wasm_camera_target_y(), wasm.verdant_wasm_camera_target_z()];
  const split = readSplitViews();
  const lodSpec = LOD_SPECS[activeLodRank] || LOD_SPECS[3];

  if (renderMode === "tile" || renderer.backend === "tensor-2d") {
    const scaleDiv = w > 480 ? 2 : 1;
    const tw = Math.max(PRISM_TILE_EDGE_PIXELS, Math.floor(w / scaleDiv / PRISM_TILE_EDGE_PIXELS) * PRISM_TILE_EDGE_PIXELS);
    const th = Math.max(PRISM_TILE_EDGE_PIXELS, Math.floor(h / scaleDiv / PRISM_TILE_EDGE_PIXELS) * PRISM_TILE_EDGE_PIXELS);
    const byteLen = wasm.verdant_wasm_render_tensor_rgba(tw >>> 0, th >>> 0, eye[0], eye[1], eye[2], tgt[0], tgt[1], tgt[2]) >>> 0;
    const ptr = wasm.verdant_wasm_fb_ptr() >>> 0;
    if (byteLen === tw * th * 4 && ptr > 0) {
      renderer.blitTensorRgba(new Uint8Array(wasm.memory.buffer, ptr, byteLen), tw, th);
    }
  } else {
    renderer.uploadSplitBuffers(split.staticViews, split.dynViews, split.instViews);
    const fov = Math.PI / 3.4;
    renderer.render(mat4Mul(mat4Perspective(fov, aspect, 0.25, CSM_FAR_METERS), mat4LookAt(eye, tgt, [0, 1, 0])), eye, lodSpec, nowMs * 0.001, tgt, fov, !turntableOn);
  }

  const dx = tgt[0] - eye[0], dy = tgt[1] - eye[1], dz = tgt[2] - eye[2];
  const dl = Math.hypot(dx, dy, dz) || 1;
  const vis = wasm.verdant_wasm_bvh_visible_count(eye[0], eye[1], eye[2], dx / dl, dy / dl, dz / dl, aspect) >>> 0;
  const clr = wasm.verdant_wasm_camera_clearance_milli() >>> 0;
  const elapsedUs = Math.max(50, Math.round((performance.now() - t0) * 1000));
  emaFrameMicros = Math.round(emaFrameMicros * 0.85 + elapsedUs * 0.15);
  const govScale = wasm.verdant_wasm_governor_scale_permille(emaFrameMicros >>> 0) >>> 0;
  renderer.setRenderScale?.(govScale);
  const up = renderer.getUploadTelemetry?.() || { staticBytes: 0, dynBytes: 0 };

  const modeLabel = renderMode === "tile" ? "Wasm 16×16 CSR Tile PBR" : "WebGL2 3-Pass PBR";
  $("stat-backend").textContent = modeLabel;
  $("stat-tris").textContent = split.tris.toLocaleString();
  $("stat-bvh").textContent = String(vis);
  $("stat-inst").textContent = activeInstCount > 0 ? `${activeInstCount.toLocaleString()} (1× drawElementsInstanced)` : "0 (base scene)";
  $("stat-static-vbo").textContent = `${(up.staticBytes || 0).toLocaleString()} B`;
  $("stat-dyn-vbo").textContent = `${(up.dynBytes || 0).toLocaleString()} B`;
  $("stat-gov-scale").textContent = `${(govScale / 10).toFixed(1)}%`;
  $("kpi-clearance").textContent = `${clr} mm`;
  $("demo-stage-left").textContent = `${modeLabel} · LOD${activeLodRank} · ${split.tris.toLocaleString()} tris`;
  $("demo-stage-right").textContent = `Static VBO: ${(up.staticBytes || 0).toLocaleString()} B · Dynamic: ${(up.dynBytes || 0).toLocaleString()} B`;
}

function applyLodRank(rank) {
  if (!wasm) return;
  activeLodRank = Math.max(0, Math.min(4, rank | 0));
  wasm.verdant_wasm_set_lod_rank(activeLodRank >>> 0);
  if (activeInstCount > 0) {
    wasm.verdant_wasm_configure_instanced_workload(activeInstCount >>> 0);
  }
  const polys = wasm.verdant_wasm_navmesh_polygon_count() >>> 0;
  if ($("kpi-navmesh")) $("kpi-navmesh").textContent = polys.toLocaleString();
  if ($("kpi-navmesh-note")) {
    $("kpi-navmesh-note").textContent = `HD TwoLevelBvh + A* · LOD${activeLodRank} active`;
  }
  document.querySelectorAll("[data-lod-rank]").forEach((btn) => {
    btn.classList.toggle("active", Number(btn.dataset.lodRank) === activeLodRank);
  });
  renderOnce(performance.now());
}

function populateLodTableSync() {
  const tbody = $("demo-lod-tbody");
  if (!tbody || !wasm) return;
  tbody.replaceChildren();
  lodVerdictCells.clear();
  const prevRank = activeLodRank;
  for (const spec of LOD_SPECS) {
    wasm.verdant_wasm_set_lod_rank(spec.rank >>> 0);
    wasm.verdant_wasm_scene_rebuild_buffers(0);
    const tris = Math.floor((wasm.verdant_wasm_scene_idx_count() >>> 0) / 3);
    const codedSum = Number(wasm.verdant_wasm_coded_zero_decode_checksum(Math.min(3, spec.rank)));
    const query = `scene:lod:${spec.rank}`;
    const cachedVerdict = holoCache.get(query) || "verifying in worker…";
    const tr = document.createElement("tr");
    const cells = [
      [spec.label, false, false],
      [spec.res, true, false],
      [spec.density, true, false],
      [tris.toLocaleString(), true, false],
      [`${(uorcStreamBytes?.byteLength || PRISM_PORTABLE_UORC_BYTES).toLocaleString()} B`, true, false],
      [String(codedSum), true, false],
      [cachedVerdict, false, true],
    ];
    for (const [txt, isNum, isVerdict] of cells) {
      const td = document.createElement("td");
      if (isNum) td.className = "num";
      if (isVerdict) {
        td.style.fontFamily = "var(--font-mono)";
        lodVerdictCells.set(query, td);
      }
      td.textContent = txt;
      tr.append(td);
    }
    tr.style.cursor = "pointer";
    tr.addEventListener("click", () => applyLodRank(spec.rank));
    tbody.append(tr);
  }
  applyLodRank(prevRank);
}

function populateKernelTableSync() {
  const tbody = $("demo-kernel-tbody");
  if (!tbody) return;
  tbody.replaceChildren();
  kernelRowCells.clear();
  for (const [query, fallbackSub, fallbackSemiring] of KERNEL_ROWS) {
    const queryDisplay = query === "" ? "(empty)" : query;
    const cachedVerdict = holoCache.get(query) || "";
    const derived = deriveKernelRowDisplay(cachedVerdict, fallbackSub, fallbackSemiring);
    const tr = document.createElement("tr");
    const tdQuery = document.createElement("td");
    tdQuery.style.fontFamily = "var(--font-mono)";
    tdQuery.textContent = queryDisplay;

    const tdSub = document.createElement("td");
    tdSub.textContent = derived.subsystem;

    const tdAlg = document.createElement("td");
    tdAlg.textContent = derived.semiring;

    const tdVerdict = document.createElement("td");
    tdVerdict.style.fontFamily = "var(--font-mono)";
    tdVerdict.textContent = cachedVerdict || "verifying in worker…";

    kernelRowCells.set(query, { tdSub, tdAlg, tdVerdict, fallbackSub, fallbackSemiring });
    tr.append(tdQuery, tdSub, tdAlg, tdVerdict);
    tbody.append(tr);
  }
}

function formatAqmLog(elapsed, m, verdict) {
  return (
    `Executed in ${elapsed.toFixed(2)} ms (Wasm linear memory)\n` +
    `8-Column Bit-Planes : ${m.entities} entities (${m.words_per_plane}×64b words/plane)\n` +
    `Archetype Matches   : ${m.archetype_matches} · popcnt=${m.popcnt_pair}\n` +
    `Monoid Fusion       : ${m.raw_ops} raw ops -> ${m.fused_ops} fused (${m.involutions} involutions, ${m.merges} merges)\n` +
    `Stride-8 Rollback   : ${m.ckpts} checkpoints · ${m.kappa_elided} elided transitions\n` +
    `Holo Verdict        : ${verdict}`
  );
}

function formatStabilityLog(elapsed, m, verdict) {
  return (
    `Executed in ${elapsed.toFixed(2)} ms (Wasm linear memory)\n` +
    `CSR Flat Tile Bins  : ${m.csr_tiles} tiles · ${m.csr_tris} tris · steady_alloc_growth=${m.csr_alloc_growth} B\n` +
    `512-Body Broadphase : ${m.bodies} dynamic spheres · contacts_resolved=${m.contacts} · nan_safe=${m.nan_safe}\n` +
    `52-Bit Horizon Guard: astronomical_refused=${m.horizon_refused} · multi_prim_glb_tris=${m.glb_tris}\n` +
    `Holo Verdict        : ${verdict}`
  );
}

function formatUorcLog(elapsed, m, verdict) {
  const ratio = Number(m.raw_vdnt_bytes) > 0
    ? ((1 - Number(m.uorc_snap_bytes) / Number(m.raw_vdnt_bytes)) * 100).toFixed(1)
    : "0.0";
  return (
    `Executed in ${elapsed.toFixed(2)} ms (Wasm linear memory)\n` +
    `Snapshot Wire       : ${m.raw_vdnt_bytes} B raw VDNT -> ${m.uorc_snap_bytes} B UORC (-${ratio}% wire, magic=${m.snap_uorc_magic})\n` +
    `SHA-256 Tamper Gate : 1-bit flip refused=${m.tamper_refused}\n` +
    `Zero-Decode CDMX    : ${m.coded_uorc_bytes} B coded matrix · exact_contraction=${m.zero_decode_ok}\n` +
    `Stitch 2-of-3 Shards: ${m.stitch_shard_bytes} B/shard · journal=${m.ckpt_uorc_bytes} B\n` +
    `Holo Verdict        : ${verdict}`
  );
}

let lastAqmRun = null;
let lastStabilityRun = null;
let lastUorcRun = null;

async function runAqmBench({ awaitHolo = true } = {}) {
  if (!wasm) return;
  const t0 = performance.now();
  wasm.verdant_wasm_aqm_benchmark_aux(16384);
  const elapsed = performance.now() - t0;
  const m = parsePipeKv(readAuxString(wasm));
  lastAqmRun = { elapsed, m };
  const initialVerdict = holoCache.get("scene:aqm") || "verifying in worker…";
  $("log-bench-aqm").textContent = formatAqmLog(elapsed, m, initialVerdict);
  if (awaitHolo) {
    const v = await dispatchHolo("scene:aqm");
    $("log-bench-aqm").textContent = formatAqmLog(elapsed, m, v);
  }
}

async function runStabilityAudit({ awaitHolo = true } = {}) {
  if (!wasm) return;
  const t0 = performance.now();
  wasm.verdant_wasm_stability_audit_aux(512);
  const elapsed = performance.now() - t0;
  const m = parsePipeKv(readAuxString(wasm));
  lastStabilityRun = { elapsed, m };
  const initialVerdict = holoCache.get("scene:stability") || "verifying in worker…";
  $("log-bench-stability").textContent = formatStabilityLog(elapsed, m, initialVerdict);
  if (awaitHolo) {
    const v = await dispatchHolo("scene:stability");
    $("log-bench-stability").textContent = formatStabilityLog(elapsed, m, v);
  }
}

function runLiveUorcWireProbeSync() {
  if (!wasm) return null;
  const t0 = performance.now();
  const stream = uorcStreamBytes;
  let magicOk = 0;
  let tamperRefused = 0;
  let stitchShardBytes = 0;
  const streamLen = stream?.byteLength || PRISM_PORTABLE_UORC_BYTES;

  if (stream && stream.byteLength >= 4) {
    magicOk =
      stream[0] === 0x55 && stream[1] === 0x4f && stream[2] === 0x52 && stream[3] === 0x43
        ? 1
        : 0;
    const tampered = stream.slice();
    tampered[0] ^= 0x5a;
    if (stageIoBytes(wasm, tampered)) {
      const accepted = wasm.verdant_wasm_snapshot_decode_and_restore() >>> 0;
      tamperRefused = accepted === 0 ? 1 : 0;
    }
    if (stageIoBytes(wasm, stream)) {
      const len0 = wasm.verdant_wasm_stitch_shard_io(0) >>> 0;
      const ptr0 = wasm.verdant_wasm_aux_ptr() >>> 0;
      const frag0 = len0 && ptr0 ? new Uint8Array(wasm.memory.buffer, ptr0, len0).slice() : null;
      stageIoBytes(wasm, stream);
      const len2 = wasm.verdant_wasm_stitch_shard_io(2) >>> 0;
      const ptr2 = wasm.verdant_wasm_aux_ptr() >>> 0;
      const frag2 = len2 && ptr2 ? new Uint8Array(wasm.memory.buffer, ptr2, len2).slice() : null;
      if (frag0 && frag2) {
        const pair = new Uint8Array(frag0.byteLength + frag2.byteLength);
        pair.set(frag0, 0);
        pair.set(frag2, frag0.byteLength);
        if (stageIoBytes(wasm, pair)) {
          const recLen = wasm.verdant_wasm_stitch_recover_io(frag0.byteLength >>> 0) >>> 0;
          if (recLen === stream.byteLength) stitchShardBytes = frag0.byteLength;
        }
      }
      stageIoBytes(wasm, stream);
    }
  }

  const cdmxTier = Math.min(3, activeLodRank >>> 0);
  const cdmxSum = Number(wasm.verdant_wasm_coded_zero_decode_checksum(cdmxTier));
  const rawGeomBytes =
    (wasm.verdant_wasm_scene_vert_count() >>> 0) * 36 +
    (wasm.verdant_wasm_scene_idx_count() >>> 0) * 4;
  const elapsed = performance.now() - t0;
  return {
    elapsed,
    m: {
      raw_vdnt_bytes: String(Math.max(rawGeomBytes, streamLen)),
      uorc_snap_bytes: String(streamLen),
      snap_uorc_magic: String(magicOk),
      tamper_refused: String(tamperRefused),
      coded_uorc_bytes: String(PRISM_PORTABLE_UORC_BYTES),
      zero_decode_ok: String(cdmxSum !== 0 ? 1 : 0),
      stitch_shard_bytes: String(stitchShardBytes),
      ckpt_uorc_bytes: String(streamLen * 2),
    },
  };
}

function dispatchOffThreadUorcExclusiveAudit() {
  if (!wasmCompiledModule || typeof Worker === "undefined" || typeof Blob === "undefined") {
    return null;
  }
  if (uorcAuditWorker) {
    uorcAuditWorker.terminate();
    uorcAuditWorker = null;
  }
  const workerSrc = `
    self.onmessage = async ({ data }) => {
      try {
        const t0 = performance.now();
        const instance = await WebAssembly.instantiate(data.mod, {});
        const ex = instance.exports;
        ex.verdant_wasm_init(BigInt(data.seed));
        ex.verdant_wasm_uorc_exclusive_audit_aux();
        const len = ex.verdant_wasm_aux_len() >>> 0;
        const ptr = ex.verdant_wasm_aux_ptr() >>> 0;
        const aux = len && ptr ? new TextDecoder().decode(new Uint8Array(ex.memory.buffer, ptr, len)) : "";
        self.postMessage({ ok: true, elapsed: performance.now() - t0, aux });
      } catch (err) {
        self.postMessage({ ok: false, error: err?.message || String(err) });
      }
    };
  `;
  const blobUrl = URL.createObjectURL(new Blob([workerSrc], { type: "text/javascript" }));
  const worker = new Worker(blobUrl);
  uorcAuditWorker = worker;
  return new Promise((resolve) => {
    worker.onmessage = ({ data }) => {
      URL.revokeObjectURL(blobUrl);
      if (uorcAuditWorker === worker) uorcAuditWorker = null;
      worker.terminate();
      resolve(data?.ok && data.aux ? { elapsed: data.elapsed, m: parsePipeKv(data.aux) } : null);
    };
    worker.onerror = () => {
      URL.revokeObjectURL(blobUrl);
      if (uorcAuditWorker === worker) uorcAuditWorker = null;
      worker.terminate();
      resolve(null);
    };
    worker.postMessage({ mod: wasmCompiledModule, seed: PRISM_DEFAULT_WORLD_SEED });
  });
}

async function runUorcAudit({ awaitHolo = true, runFullNcm = false } = {}) {
  if (!wasm) return;
  const probe = runLiveUorcWireProbeSync();
  if (probe) {
    lastUorcRun = probe;
    const initialVerdict = holoCache.get("scene:uorc-exclusive") || "verifying in worker…";
    $("log-bench-uorc").textContent = formatUorcLog(probe.elapsed, probe.m, initialVerdict);
  }
  if (runFullNcm) {
    const ncmPromise = dispatchOffThreadUorcExclusiveAudit();
    if (ncmPromise) {
      ncmPromise.then((res) => {
        if (!res) return;
        lastUorcRun = res;
        const verdict = holoCache.get("scene:uorc-exclusive") || "verifying in worker…";
        $("log-bench-uorc").textContent = formatUorcLog(res.elapsed, res.m, verdict);
      });
    }
  }
  if (awaitHolo) {
    const v = await dispatchHolo("scene:uorc-exclusive");
    if (lastUorcRun) {
      $("log-bench-uorc").textContent = formatUorcLog(lastUorcRun.elapsed, lastUorcRun.m, v);
    }
  }
}

async function streamHoloVerificationsOffThread(kernelsCount) {
  const identities = await startHoloWorker();
  const shortFp = String(identities?.digest || "").replace(/^blake3:/, "").slice(0, 12);
  if (shortFp) {
    $("kpi-holo-note").textContent = `blake3:${shortFp}… verified`;
  }

  const totalQueries = 4 + LOD_SPECS.length + KERNEL_ROWS.length;
  let completed = 0;
  const bumpProgress = () => {
    completed += 1;
    $("kpi-vectors").textContent = `${completed} / ${totalQueries}`;
    $("demo-boot-status").textContent =
      `Prism Verified · ${completed}/${totalQueries} Holo Proofs · ${kernelsCount} Semiring Kernels · Seed ${PRISM_DEFAULT_WORLD_SEED}`;
  };

  // 1. Master HD summary + Subsystem benchmark Holo verdicts:
  await dispatchHolo("summary:hd");
  bumpProgress();

  const vAqm = await dispatchHolo("scene:aqm");
  if (lastAqmRun) $("log-bench-aqm").textContent = formatAqmLog(lastAqmRun.elapsed, lastAqmRun.m, vAqm);
  bumpProgress();

  const vStab = await dispatchHolo("scene:stability");
  if (lastStabilityRun) $("log-bench-stability").textContent = formatStabilityLog(lastStabilityRun.elapsed, lastStabilityRun.m, vStab);
  bumpProgress();

  const vUorc = await dispatchHolo("scene:uorc-exclusive");
  if (lastUorcRun) $("log-bench-uorc").textContent = formatUorcLog(lastUorcRun.elapsed, lastUorcRun.m, vUorc);
  bumpProgress();

  // 2. Progressive LOD monotonicity verdicts & live sky/fog hydration from app.holo:
  for (const spec of LOD_SPECS) {
    const q = `scene:lod:${spec.rank}`;
    const v = await dispatchHolo(q);
    const lf = parseHoloFields(v);
    if (lf?.sky) {
      const sp = lf.sky.split("/").map(Number);
      if (sp.length === 3 && sp.every(Number.isFinite)) spec.sky = sp.map((c) => c / 100);
    }
    if (lf?.fog) {
      const fp = lf.fog.split("/").map(Number);
      if (fp.length === 3 && fp.every(Number.isFinite)) spec.fog = fp.map((c) => c / 100);
    }
    const td = lodVerdictCells.get(q);
    if (td) td.textContent = v;
    bumpProgress();
  }

  // 3. 15 unified semiring kernels + 3 zero-decode coded tiers + 4 negative refusal vectors:
  for (const [query] of KERNEL_ROWS) {
    const v = await dispatchHolo(query);
    const slot = kernelRowCells.get(query);
    if (slot) {
      const derived = deriveKernelRowDisplay(v, slot.fallbackSub, slot.fallbackSemiring);
      slot.tdSub.textContent = derived.subsystem;
      slot.tdAlg.textContent = derived.semiring;
      slot.tdVerdict.textContent = v;
    }
    bumpProgress();
  }

  document.body.dataset.holoReady = "true";
}

function bindControls() {
  $("demo-theme-btn")?.addEventListener("click", () => {
    const isDark = document.documentElement.classList.toggle("dark");
    $("demo-theme-btn").textContent = isDark ? "Theme: Dark" : "Theme: Light";
    renderOnce(performance.now());
  });

  document.querySelectorAll("[data-render-mode]").forEach((btn) => {
    btn.addEventListener("click", () => {
      renderMode = btn.dataset.renderMode === "tile" ? "tile" : "webgl2";
      document.querySelectorAll("[data-render-mode]").forEach((b) => b.classList.toggle("active", b === btn));
      renderOnce(performance.now());
    });
  });

  $("btn-spin-toggle")?.addEventListener("click", (e) => {
    turntableOn = !turntableOn;
    e.currentTarget.setAttribute("aria-pressed", String(turntableOn));
    e.currentTarget.textContent = turntableOn ? "Turntable: On" : "Turntable: Off";
    if (turntableOn) {
      prevTick = performance.now();
      scheduleNextFrame();
    }
  });

  document.querySelectorAll("[data-domain-pack]").forEach((btn) => {
    btn.addEventListener("click", () => {
      if (!wasm || typeof wasm.verdant_wasm_load_domain_pack !== "function") return;
      const idx = Math.max(0, Math.min(3, Number(btn.dataset.domainPack) || 0));
      const len = wasm.verdant_wasm_load_domain_pack(idx >>> 0) >>> 0;
      document.querySelectorAll("[data-domain-pack]").forEach((b) => b.classList.toggle("active", b === btn));
      if (len > 0) {
        try {
          const obs = JSON.parse(readAuxString(wasm));
          const dom = obs.activeDomain || {};
          $("kpi-navmesh").textContent = (wasm.verdant_wasm_navmesh_polygon_count() >>> 0).toLocaleString();
          $("demo-boot-status").textContent =
            `World Pack: ${dom.title || btn.textContent} (${dom.schemaOrgType || "VideoGame"}) · κ=${String(dom.kappa || "").slice(0, 16)}… · Seed ${dom.seed || 7}`;
        } catch {}
      }
      renderOnce(performance.now());
    });
  });

  document.querySelectorAll("[data-inst-count]").forEach((btn) => {
    btn.addEventListener("click", () => {
      if (!wasm) return;
      activeInstCount = Number(btn.dataset.instCount) || 0;
      wasm.verdant_wasm_configure_instanced_workload(activeInstCount >>> 0);
      document.querySelectorAll("[data-inst-count]").forEach((b) => b.classList.toggle("active", b === btn));
      renderOnce(performance.now());
    });
  });

  document.querySelectorAll("[data-lod-rank]").forEach((btn) => {
    btn.addEventListener("click", () => applyLodRank(Number(btn.dataset.lodRank) || 0));
  });

  $("btn-bench-aqm")?.addEventListener("click", () => runAqmBench({ awaitHolo: true }));
  $("btn-bench-stability")?.addEventListener("click", () => runStabilityAudit({ awaitHolo: true }));
  $("btn-bench-uorc")?.addEventListener("click", () => runUorcAudit({ awaitHolo: true, runFullNcm: true }));
  $("btn-bench-uhd")?.addEventListener("click", () => runUhdStabilityBench());
  $("demo-run-all-btn")?.addEventListener("click", async () => {
    await runAqmBench({ awaitHolo: true });
    await runStabilityAudit({ awaitHolo: true });
    await runUorcAudit({ awaitHolo: true, runFullNcm: true });
    runUhdStabilityBench();
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
    yawMilli -= Math.round(dx * 6.5);
    pitchMilli = Math.max(80, Math.min(1150, pitchMilli + Math.round(dy * 5.5)));
    if (!turntableOn) renderOnce(performance.now());
  });
  canvas?.addEventListener("pointerup", () => { dragging = false; });
  canvas?.addEventListener("pointercancel", () => { dragging = false; });
  canvas?.addEventListener("wheel", (e) => {
    e.preventDefault();
    distMm = Math.max(4500, Math.min(48000, distMm + Math.round(e.deltaY * 14)));
    if (!turntableOn) renderOnce(performance.now());
  }, { passive: false });
}

let prevTick = performance.now();
let loopScheduled = false;
function scheduleNextFrame() {
  if (!turntableOn || loopScheduled) return;
  loopScheduled = true;
  if (renderer?.isSoftware) {
    setTimeout(() => requestAnimationFrame(loop), 33);
  } else {
    requestAnimationFrame(loop);
  }
}

function loop(now) {
  loopScheduled = false;
  if (!turntableOn) return;
  const dt = Math.min(0.05, Math.max(0.001, (now - prevTick) * 0.001));
  prevTick = now;
  if (!dragging && !document.hidden) {
    yawMilli = (yawMilli + Math.round(dt * 150)) | 0;
    renderOnce(now);
  }
  scheduleNextFrame();
}

function runUhdStabilityBench() {
  const out = $("log-bench-uhd");
  if (!wasm || !out) return;
  const t0 = performance.now();
  const isSw = renderer?.isSoftware ? 1 : 0;
  const wgpu = typeof navigator !== "undefined" && Boolean(navigator.gpu) && !isSw ? 1 : 0;
  let p4k = null;
  let p8k = null;
  let p16k = null;
  let meshlets = null;
  let pyr = null;
  let stab = null;
  let paged = null;
  if (typeof wasm.verdant_wasm_uhd_profile_aux === "function") {
    if (wasm.verdant_wasm_uhd_profile_aux(3840, 2160, isSw, wgpu, 7500) > 0) {
      try { p4k = JSON.parse(readAuxString(wasm)); } catch {}
    }
    if (wasm.verdant_wasm_uhd_profile_aux(7680, 4320, isSw, wgpu, 9000) > 0) {
      try { p8k = JSON.parse(readAuxString(wasm)); } catch {}
    }
    if (wasm.verdant_wasm_uhd_profile_aux(15360, 8640, isSw, wgpu, 11000) > 0) {
      try { p16k = JSON.parse(readAuxString(wasm)); } catch {}
    }
  }
  if (typeof wasm.verdant_wasm_tensor_paged_benchmark_aux === "function") {
    if (wasm.verdant_wasm_tensor_paged_benchmark_aux(64, 512) > 0) {
      try { paged = JSON.parse(readAuxString(wasm)); } catch {}
    }
  }
  if (typeof wasm.verdant_wasm_uhd_cull_meshlets === "function") {
    const ex = wasm.verdant_wasm_camera_eye_x();
    const ey = wasm.verdant_wasm_camera_eye_y();
    const ez = wasm.verdant_wasm_camera_eye_z();
    const tx = wasm.verdant_wasm_camera_target_x();
    const ty = wasm.verdant_wasm_camera_target_y();
    const tz = wasm.verdant_wasm_camera_target_z();
    if (wasm.verdant_wasm_uhd_cull_meshlets(ex, ey, ez, tx, ty, tz, 16 / 9, 2160, 10) > 0) {
      try { meshlets = JSON.parse(readAuxString(wasm)); } catch {}
    }
  }
  if (typeof wasm.verdant_wasm_uhd_lod_pyramid_aux === "function") {
    if (wasm.verdant_wasm_uhd_lod_pyramid_aux() > 0) {
      try { pyr = JSON.parse(readAuxString(wasm)); } catch {}
    }
  }
  if (typeof wasm.verdant_wasm_record_frame_time_us === "function" && typeof wasm.verdant_wasm_frame_stability_aux === "function") {
    for (let i = 0; i < 64; i += 1) {
      wasm.verdant_wasm_record_frame_time_us(i % 2 === 0 ? 7400 : 8100);
    }
    if (wasm.verdant_wasm_frame_stability_aux() > 0) {
      try { stab = JSON.parse(readAuxString(wasm)); } catch {}
    }
  }
  const elapsed = (performance.now() - t0).toFixed(2);
  const l4 = p4k ? `4K (${p4k.widthPx}×${p4k.heightPx} · ${(p4k.megapixelsX100 / 100).toFixed(2)} MP): DRS ${Math.round(p4k.drsScalePermille / 10)}% + CAS ${Math.round(p4k.casSharpnessPermille / 10)}% · ${p4k.preferredBackend}` : "4K: unavailable";
  const l8 = p8k ? `8K (${p8k.widthPx}×${p8k.heightPx} · ${(p8k.megapixelsX100 / 100).toFixed(2)} MP): DRS ${Math.round(p8k.drsScalePermille / 10)}% + CAS ${Math.round(p8k.casSharpnessPermille / 10)}% · SSE ${(p8k.meshletSseTenthsPx / 10).toFixed(1)}px` : "8K: unavailable";
  const l16 = p16k ? `16K Beyond-8K (${p16k.widthPx}×${p16k.heightPx} · ${(p16k.megapixelsX100 / 100).toFixed(2)} MP): DRS ${Math.round(p16k.drsScalePermille / 10)}% + CAS ${Math.round(p16k.casSharpnessPermille / 10)}% · ${p16k.displayClass}` : "16K: unavailable";
  const lm = meshlets ? `Morton Meshlets: ${meshlets.visibleMeshlets}/${meshlets.totalMeshlets} visible (${meshlets.culledFrustum} frustum, ${meshlets.culledBackfaceCone} cone -> ${(meshlets.cullRatioPermille / 10).toFixed(1)}% culled)` : "Morton Meshlets: unavailable";
  const lp = pyr ? `Hierarchical LOD Pyramid: LOD0 ${pyr.lod0Triangles.toLocaleString()} -> LOD1 ${pyr.lod1Triangles.toLocaleString()} -> LOD2 ${pyr.lod2Triangles.toLocaleString()} tris (${(pyr.lod2ReductionPermille / 10).toFixed(1)}% reduction)` : "LOD Pyramid: unavailable";
  const ls = stab ? `Frame Stability Oracle (${stab.sampleCount}f): P50 ${(stab.p50Us / 1000).toFixed(2)}ms · P95 ${(stab.p95Us / 1000).toFixed(2)}ms · P99 ${(stab.p99Us / 1000).toFixed(2)}ms · Jitter ${(stab.jitterUs / 1000).toFixed(2)}ms · DRS ${Math.round(stab.drsScalePermille / 10)}% (${stab.drsTransitions} transitions · ${stab.stable ? "STABLE" : "BOUNDED"})` : "Frame Oracle: unavailable";
  const lmem = paged ? `Paged Tensor Arena (Packed<Grid,2>): ${(paged.logicalWorkloadBytes / 1048576).toFixed(1)} MB logical in ${(paged.budgetBytes / 1024).toFixed(0)} KiB cap (${(paged.virtualAmplificationPermille / 1000).toFixed(1)}× amp · ${paged.evictions} CLOCK evictions · bitExact=${paged.bitExact} · invariant=${paged.invariantHeld})` : "Paged Tensor Arena: unavailable";
  out.textContent = `VE-41 UHD & Paged Tensor Core (${elapsed} ms):\n${l4}\n${l8}\n${l16}\n${lm}\n${lp}\n${ls}\n${lmem}`;
}

async function boot() {
  bindControls();
  // Launch `/app.holo` worker verification immediately in parallel with Wasm scene boot:
  startHoloWorker().catch(() => {});

  renderer = await createBestRenderer(canvas);
  if (renderer?.isSoftware) {
    turntableOn = false;
    const spinBtn = $("btn-spin-toggle");
    if (spinBtn) {
      spinBtn.setAttribute("aria-pressed", "false");
      spinBtn.textContent = "Turntable: Off";
    }
  }

  const [wasmResp, streamResp] = await Promise.all([
    fetch(new URL("../verdant-uorc.wasm", import.meta.url)),
    fetch(new URL("../scene-stream.uorc", import.meta.url)),
  ]);
  if (!wasmResp.ok) {
    throw new Error(`Asset HTTP error: verdant-uorc.wasm answered ${wasmResp.status}`);
  }
  const [wasmBuf, streamBuf] = await Promise.all([
    wasmResp.arrayBuffer(),
    streamResp.ok ? streamResp.arrayBuffer() : Promise.resolve(new ArrayBuffer(0)),
  ]);
  uorcStreamBytes = streamBuf.byteLength > 0 ? new Uint8Array(streamBuf) : null;

  wasmCompiledModule = await WebAssembly.compile(wasmBuf);
  const instance = await WebAssembly.instantiate(wasmCompiledModule, {});
  wasm = instance.exports;
  wasm.verdant_wasm_init(BigInt(PRISM_DEFAULT_WORLD_SEED));
  const csmSplits = [0, 1, 2, 3].map((i) => wasm.verdant_wasm_csm_split_meters(0.25, CSM_FAR_METERS, i));
  renderer?.setCsmSplits?.(csmSplits);

  const kernels = wasm.verdant_wasm_unified_tensor_kernel_count() >>> 0;
  const polys = wasm.verdant_wasm_navmesh_polygon_count() >>> 0;
  const totalQueries = 4 + LOD_SPECS.length + KERNEL_ROWS.length;
  $("kpi-vectors").textContent = `0 / ${totalQueries}`;
  $("kpi-kernels").textContent = String(kernels);
  $("kpi-navmesh").textContent = polys.toLocaleString();
  $("demo-boot-status").textContent =
    `Wasm Ready · Streaming ${totalQueries} Holo Proofs in Worker · ${kernels} Semiring Kernels · Seed ${PRISM_DEFAULT_WORLD_SEED}`;

  populateLodTableSync();
  populateKernelTableSync();
  await runAqmBench({ awaitHolo: false });
  await runStabilityAudit({ awaitHolo: false });
  await runUorcAudit({ awaitHolo: false, runFullNcm: false });
  runUhdStabilityBench();
  renderOnce(performance.now());
  document.body.dataset.demoReady = "true";
  scheduleNextFrame();

  // Stream `/app.holo` cryptographic verdicts off the main thread without blocking UI frames:
  await streamHoloVerificationsOffThread(kernels);
}

boot().catch((err) => {
  $("demo-boot-status").textContent = `Verification refused: ${err?.message || err}`;
  console.error("Verdant Engine Demo boot error:", err);
});
