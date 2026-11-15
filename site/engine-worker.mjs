// Verdant Engine — Dedicated WebAssembly Simulation, Tensor Compute, & OffscreenCanvas Worker.
// Runs crates/verdant-wasm (wasm32-unknown-unknown) off the main thread alongside universal
// WebGL2/WebGL1 or Prism uor-matmul 16x16 software tile rendering, SAH BVH frustum culling,
// Woodworth binaural cues, binary glTF 2.0 (.glb) scene packing, and SHA-256 VDNT snapshots.

import {
  PRISM_DEFAULT_WORLD_SEED,
  PRISM_SNAPSHOT_URL_MAX_BYTES,
  PRISM_TILE_EDGE_PIXELS,
  PRISM_WORKER_RING_CAPACITY,
} from "./prism-bridge-meta.mjs";
import {
  buildDynamicActors,
  buildGlbArrayBuffer,
  buildStaticWorld,
  createBestRenderer,
  mat4LookAt,
  mat4Mul,
  mat4Perspective,
} from "./scene-renderer.mjs";

const wasmUrl = new URL("./verdant-wasm.wasm", import.meta.url);

async function instantiateVerdantWasm() {
  const response = await fetch(wasmUrl);
  if (!response.ok) throw new Error(`verdant-wasm HTTP ${response.status}`);
  if (typeof WebAssembly.instantiateStreaming === "function") {
    try {
      const { instance } = await WebAssembly.instantiateStreaming(response, {});
      return instance.exports;
    } catch {
      const fallbackResp = await fetch(wasmUrl);
      const buf = await fallbackResp.arrayBuffer();
      const { instance } = await WebAssembly.instantiate(buf, {});
      return instance.exports;
    }
  }
  const buf = await response.arrayBuffer();
  const { instance } = await WebAssembly.instantiate(buf, {});
  return instance.exports;
}

let ringCapacity = PRISM_WORKER_RING_CAPACITY;
let snapshotMaxBytes = PRISM_SNAPSHOT_URL_MAX_BYTES;
let tileEdgePx = PRISM_TILE_EDGE_PIXELS;

const wasmPromise = instantiateVerdantWasm().then((ex) => {
  const seed =
    typeof ex.verdant_wasm_default_seed === "function"
      ? ex.verdant_wasm_default_seed()
      : BigInt(PRISM_DEFAULT_WORLD_SEED);
  ex.verdant_wasm_init(seed);
  ringCapacity = ex.verdant_wasm_worker_ring_capacity() >>> 0;
  snapshotMaxBytes = ex.verdant_wasm_snapshot_max_bytes() >>> 0;
  tileEdgePx = ex.verdant_wasm_tile_edge_pixels() >>> 0;
  return ex;
});

let wasm = null;
let canvas = null;
let renderer = null;
let terrainGrid = 24;
let staticMesh = null;
let lastDynMesh = null;
let currentLod = null;
let pendingCommands = 0;

function readMover() {
  if (!wasm) return null;
  return {
    x: wasm.verdant_wasm_mover_x(),
    z: wasm.verdant_wasm_mover_z(),
    yaw: wasm.verdant_wasm_mover_yaw(),
  };
}

function drawScene(eye, lookTarget, lodSpec, simClockSec) {
  if (!canvas || !renderer || !staticMesh || canvas.width <= 0 || canvas.height <= 0) return null;
  const wasmMover = (lodSpec?.rank ?? 3) >= 2 ? readMover() : null;
  lastDynMesh = buildDynamicActors(lodSpec, simClockSec, wasmMover);
  const aspect = Math.max(0.2, canvas.width / canvas.height);

  if (renderer.backend === "tensor-2d" && wasm && wasm.memory) {
    const w = Math.max(tileEdgePx, Math.floor(canvas.width / tileEdgePx) * tileEdgePx);
    const h = Math.max(tileEdgePx, Math.floor(canvas.height / tileEdgePx) * tileEdgePx);
    const byteLen = wasm.verdant_wasm_render_tensor_rgba(
      w >>> 0,
      h >>> 0,
      eye[0],
      eye[1],
      eye[2],
      lookTarget[0],
      lookTarget[1],
      lookTarget[2],
    ) >>> 0;
    const ptr = wasm.verdant_wasm_fb_ptr() >>> 0;
    if (byteLen === w * h * 4 && ptr > 0) {
      const slice = new Uint8Array(wasm.memory.buffer, ptr, byteLen);
      renderer.blitTensorRgba(slice, w, h);
    } else {
      renderer.render(null, eye, lodSpec, simClockSec);
    }
  } else {
    renderer.uploadDynamic(lastDynMesh.mb);
    const proj = mat4Perspective(Math.PI / 3.4, aspect, 0.25, 140.0);
    const view = mat4LookAt(eye, lookTarget, [0, 1, 0]);
    renderer.render(mat4Mul(proj, view), eye, lodSpec, simClockSec);
  }

  return {
    tris: Math.round((staticMesh.idx.length + lastDynMesh.mb.idx.length) / 3),
    aspect,
  };
}

self.onmessage = async ({ data: msg }) => {
  if (!msg || typeof msg.kind !== "string") return;
  if (pendingCommands >= ringCapacity && msg.kind === "frame") {
    return;
  }
  pendingCommands++;
  try {
    wasm ??= await wasmPromise;

    if (msg.kind === "init") {
      canvas = msg.canvas || null;
      currentLod = msg.lod;
      if (canvas) {
        canvas.width = Math.max(1, msg.width | 0);
        canvas.height = Math.max(1, msg.height | 0);
        renderer = await createBestRenderer(canvas);
      }
      terrainGrid = renderer?.isSoftware ? 18 : 36;
      wasm.verdant_wasm_set_lod_rank((currentLod?.rank ?? 3) >>> 0);
      staticMesh = buildStaticWorld(currentLod, terrainGrid);
      renderer?.uploadStatic(staticMesh);
      const stats = drawScene(msg.eye, msg.lookTarget, currentLod, 0);
      self.postMessage({
        kind: "ready",
        id: msg.id,
        backend: renderer?.backend ?? "headless",
        isSoftware: Boolean(renderer?.isSoftware),
        walkablePolys: wasm.verdant_wasm_navmesh_polygon_count() >>> 0,
        tensorKernels: wasm.verdant_wasm_tensor_kernel_count() >>> 0,
        tris: stats?.tris ?? Math.round((staticMesh?.idx.length ?? 0) / 3),
      });
      return;
    }

    if (msg.kind === "resize") {
      if (canvas) {
        canvas.width = Math.max(1, msg.width | 0);
        canvas.height = Math.max(1, msg.height | 0);
      }
      return;
    }

    if (msg.kind === "set-lod") {
      currentLod = msg.lod;
      wasm.verdant_wasm_set_lod_rank((currentLod?.rank ?? 3) >>> 0);
      staticMesh = buildStaticWorld(currentLod, terrainGrid);
      renderer?.uploadStatic(staticMesh);
      if (msg.eye && msg.lookTarget) {
        drawScene(msg.eye, msg.lookTarget, currentLod, msg.simClockSec || 0);
      }
      return;
    }

    if (msg.kind === "frame") {
      const lod = msg.lod || currentLod;
      currentLod = lod;
      if (canvas && msg.width > 0 && msg.height > 0) {
        if (canvas.width !== msg.width || canvas.height !== msg.height) {
          canvas.width = msg.width;
          canvas.height = msg.height;
        }
      }
      if (msg.stepPhysics) {
        wasm.verdant_wasm_step(
          Math.max(1000, Math.min(50000, Math.round(msg.dt * 1_000_000))) >>> 0,
          Math.round((msg.fwd || 0) * 1000) | 0,
          Math.round((msg.strafe || 0) * 1000) | 0,
          Math.round((msg.walkYaw || 0) * 1000) | 0,
        );
      }
      const stats = drawScene(msg.eye, msg.lookTarget, lod, msg.simClockSec || 0);
      const dx = msg.lookTarget[0] - msg.eye[0];
      const dy = msg.lookTarget[1] - msg.eye[1];
      const dz = msg.lookTarget[2] - msg.eye[2];
      const dl = Math.hypot(dx, dy, dz) || 1;
      const fx = dx / dl, fy = dy / dl, fz = dz / dl;
      const bvhVisible = wasm.verdant_wasm_bvh_visible_count(
        msg.eye[0], msg.eye[1], msg.eye[2],
        fx, fy, fz,
        stats?.aspect ?? 1.6,
      ) >>> 0;
      const pan = wasm.verdant_wasm_binaural_pan(
        -9.0, 2.0, -4.0,
        msg.eye[0], msg.eye[1], msg.eye[2],
        fx, fy, fz,
      );
      const gain = wasm.verdant_wasm_binaural_gain();
      self.postMessage({
        kind: "telemetry",
        tris: stats?.tris ?? 0,
        bvhVisible,
        moverPos: lastDynMesh?.moverPos ?? [16.5, 0.5, 0],
        pavilionContact: wasm.verdant_wasm_pavilion_contact() === 1,
        binauralPan: pan,
        binauralGain: gain,
      });
      return;
    }

    if (msg.kind === "export-glb") {
      const sMesh = staticMesh || buildStaticWorld(currentLod, terrainGrid);
      const dMesh = lastDynMesh || buildDynamicActors(currentLod, 0, readMover());
      const buffer = buildGlbArrayBuffer(sMesh, dMesh.mb);
      self.postMessage({ kind: "glb-ready", id: msg.id, buffer }, [buffer]);
      return;
    }

    if (msg.kind === "encode-snapshot") {
      const len = wasm.verdant_wasm_snapshot_encode() >>> 0;
      if (len === 0 || len > snapshotMaxBytes) {
        self.postMessage({ kind: "snapshot-encoded", id: msg.id, ok: false });
        return;
      }
      const bytes = new Uint8Array(len);
      for (let i = 0; i < len; i++) {
        bytes[i] = wasm.verdant_wasm_io_get_byte(i) & 0xff;
      }
      self.postMessage(
        { kind: "snapshot-encoded", id: msg.id, ok: true, bytes },
        [bytes.buffer],
      );
      return;
    }

    if (msg.kind === "restore-snapshot") {
      const raw = msg.bytes instanceof Uint8Array ? msg.bytes : new Uint8Array(msg.bytes || []);
      if (raw.byteLength === 0 || raw.byteLength > snapshotMaxBytes) {
        self.postMessage({ kind: "snapshot-restored", id: msg.id, ok: false });
        return;
      }
      wasm.verdant_wasm_io_clear();
      for (let i = 0; i < raw.byteLength; i++) {
        wasm.verdant_wasm_io_push_byte(raw[i]);
      }
      const ok = wasm.verdant_wasm_snapshot_decode_and_restore() === 1;
      const lodRank = wasm.verdant_wasm_active_lod_rank() >>> 0;
      const seed = Number(wasm.verdant_wasm_active_seed());
      self.postMessage({
        kind: "snapshot-restored",
        id: msg.id,
        ok,
        lodRank,
        seed,
        byteLen: raw.byteLength,
      });
    }
  } catch (err) {
    self.postMessage({
      kind: "worker-error",
      id: msg.id,
      error: String(err?.message ?? err),
    });
  } finally {
    pendingCommands = Math.max(0, pendingCommands - 1);
  }
};
