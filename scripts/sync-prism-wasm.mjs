#!/usr/bin/env node
// Builds crates/verdant-wasm for wasm32-unknown-unknown (both cold-boot runtime and
// portable uorc_decompress streaming module), packs site/scene-stream.uorc, executes the
// compiled Prism Wasm exports under Node WebAssembly, and emits site/prism-bridge-meta.mjs
// mechanically so AudioWorklet and Worker modules consume verified Prism kernel values.

import { execFileSync } from "node:child_process";
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const WASM_TARGET_OUT = path.join(
  ROOT,
  "target",
  "wasm32-unknown-unknown",
  "release",
  "verdant_wasm.wasm",
);
const SITE_WASM = path.join(ROOT, "site", "verdant-wasm.wasm");
const SITE_UORC_WASM = path.join(ROOT, "site", "verdant-uorc.wasm");
const SITE_UORC_STREAM = path.join(ROOT, "site", "scene-stream.uorc");
const SITE_META = path.join(ROOT, "site", "prism-bridge-meta.mjs");

const checkOnly = process.argv.includes("--check");

if (!checkOnly) {
  // 1. Build lean cold-boot WebAssembly engine runtime (< 76 KiB gzipped):
  execFileSync(
    "cargo",
    ["build", "-p", "verdant-wasm", "--target", "wasm32-unknown-unknown", "--release", "--locked"],
    { cwd: ROOT, stdio: "inherit" },
  );
  fs.copyFileSync(WASM_TARGET_OUT, SITE_WASM);

  // 2. Build portable uorc_decompress WebAssembly streaming decoder:
  execFileSync(
    "cargo",
    [
      "build",
      "-p",
      "verdant-wasm",
      "--target",
      "wasm32-unknown-unknown",
      "--release",
      "--features",
      "uorc-decoder",
      "--locked",
    ],
    { cwd: ROOT, stdio: "inherit" },
  );
  fs.copyFileSync(WASM_TARGET_OUT, SITE_UORC_WASM);

  // 3. Pack canonical LOD0..LOD3 portable .uorc stream if missing or --repack passed:
  if (!fs.existsSync(SITE_UORC_STREAM) || process.argv.includes("--repack")) {
    execFileSync(
      "cargo",
      [
        "run",
        "-p",
        "verdant-cli",
        "--release",
        "--locked",
        "--",
        "pack-portable-uorc",
        SITE_UORC_STREAM,
      ],
      { cwd: ROOT, stdio: "inherit" },
    );
  }
}

if (!fs.existsSync(SITE_WASM) || !fs.existsSync(SITE_UORC_WASM) || !fs.existsSync(SITE_UORC_STREAM)) {
  console.error(
    "sync-prism-wasm: missing site/verdant-wasm.wasm, site/verdant-uorc.wasm, or site/scene-stream.uorc",
  );
  process.exit(1);
}

const wasmBytes = fs.readFileSync(SITE_WASM);
const uorcWasmBytes = fs.readFileSync(SITE_UORC_WASM);
const uorcStreamBytes = fs.readFileSync(SITE_UORC_STREAM);

for (const [label, buf] of [
  ["verdant-wasm.wasm", wasmBytes],
  ["verdant-uorc.wasm", uorcWasmBytes],
]) {
  if (
    buf.length < 8 ||
    buf[0] !== 0x00 ||
    buf[1] !== 0x61 ||
    buf[2] !== 0x73 ||
    buf[3] !== 0x6d
  ) {
    console.error(`sync-prism-wasm: invalid WebAssembly magic header in ${label}`);
    process.exit(1);
  }
}

const { instance } = await WebAssembly.instantiate(wasmBytes, {});
const ex = instance.exports;

const defaultSeed = ex.verdant_wasm_default_seed();
const initOk = ex.verdant_wasm_init(defaultSeed);
if (initOk !== 1) {
  console.error(`sync-prism-wasm: verdant_wasm_init(${defaultSeed}) failed`);
  process.exit(1);
}

const quantumFrames = ex.verdant_wasm_audio_quantum_frames() >>> 0;
const stereoChannels = ex.verdant_wasm_audio_channels() >>> 0;
const modalBarDenom = ex.verdant_wasm_modal_bar_denom() >>> 0;
const modalBarNumers = [0, 1, 2, 3, 4].map((i) => ex.verdant_wasm_modal_bar_numer(i) >>> 0);
const chamberRt60Ms = ex.verdant_wasm_sabine_rt60_millis(1800, 96000) >>> 0;
const walkablePolygons = ex.verdant_wasm_navmesh_polygon_count() >>> 0;
const nodePresetCount = ex.verdant_wasm_node_preset_count() >>> 0;
const tensorKernelCount = ex.verdant_wasm_tensor_kernel_count() >>> 0;
const unifiedKernelCount = ex.verdant_wasm_unified_tensor_kernel_count() >>> 0;
const codedTierCount = ex.verdant_wasm_coded_tier_count() >>> 0;
const maxLodRank = ex.verdant_wasm_max_lod_rank() >>> 0;
const hdMaxLodRank = ex.verdant_wasm_hd_max_lod_rank() >>> 0;
const tileEdgePixels = ex.verdant_wasm_tile_edge_pixels() >>> 0;
const csmSplits = [0, 1, 2, 3].map((i) =>
  Number(ex.verdant_wasm_csm_split_meters(0.25, 140.0, i).toFixed(4)),
);
const ringCapacity = ex.verdant_wasm_worker_ring_capacity() >>> 0;
const snapshotMaxBytes = ex.verdant_wasm_snapshot_max_bytes() >>> 0;
const snapshotMinWireBytes = ex.verdant_wasm_snapshot_min_wire_bytes() >>> 0;

// Verify native WebAssembly uorc_decompress + uor_matmul::codec zero-decode streaming:
const { instance: uorcInstance } = await WebAssembly.instantiate(uorcWasmBytes, {});
const uex = uorcInstance.exports;
if (uex.verdant_wasm_init(defaultSeed) !== 1) {
  console.error("sync-prism-wasm: verdant-uorc.wasm init failed");
  process.exit(1);
}
const codedZeroDecodeChecksum = Number(uex.verdant_wasm_coded_zero_decode_checksum(maxLodRank));
uex.verdant_wasm_io_clear();
for (let i = 0; i < uorcStreamBytes.length; i++) {
  uex.verdant_wasm_io_push_byte(uorcStreamBytes[i]);
}
const decodeOk = uex.verdant_wasm_uorc_decode_and_restore(maxLodRank);
if (decodeOk !== 1 || (uex.verdant_wasm_active_lod_rank() >>> 0) !== maxLodRank) {
  console.error("sync-prism-wasm: WebAssembly uorc_decompress progressive LOD3 verification failed");
  process.exit(1);
}

const expectedMeta = `// Generated mechanically by scripts/sync-prism-wasm.mjs from Prism-verified verdant-wasm.
// Do not edit by hand.

export const PRISM_DEFAULT_WORLD_SEED = ${Number(defaultSeed)};
export const PRISM_MAX_LOD_RANK = ${maxLodRank};
export const PRISM_HD_MAX_LOD_RANK = ${hdMaxLodRank};
export const PRISM_SNAPSHOT_MIN_WIRE_BYTES = ${snapshotMinWireBytes};
export const PRISM_AUDIO_QUANTUM_FRAMES = ${quantumFrames};
export const PRISM_AUDIO_STEREO_CHANNELS = ${stereoChannels};
export const PRISM_MODAL_BAR_DENOM = ${modalBarDenom};
export const PRISM_MODAL_BAR_NUMERS = Object.freeze(${JSON.stringify(modalBarNumers)});
export const PRISM_CHAMBER_SABINE_RT60_MS = ${chamberRt60Ms};
export const PRISM_TERRAIN_WALKABLE_POLYGONS = ${walkablePolygons};
export const PRISM_NODE_PRESET_COUNT = ${nodePresetCount};
export const PRISM_TENSOR_KERNEL_COUNT = ${tensorKernelCount};
export const PRISM_UNIFIED_KERNEL_COUNT = ${unifiedKernelCount};
export const PRISM_CODED_TIER_COUNT = ${codedTierCount};
export const PRISM_CODED_ZERO_DECODE_CHECKSUM = ${codedZeroDecodeChecksum};
export const PRISM_PORTABLE_UORC_BYTES = ${uorcStreamBytes.length};
export const PRISM_TILE_EDGE_PIXELS = ${tileEdgePixels};
export const PRISM_CSM_SPLIT_METERS = Object.freeze(${JSON.stringify(csmSplits)});
export const PRISM_WORKER_RING_CAPACITY = ${ringCapacity};
export const PRISM_SNAPSHOT_URL_MAX_BYTES = ${snapshotMaxBytes};
`;

if (checkOnly) {
  if (!fs.existsSync(SITE_META)) {
    console.error("sync-prism-wasm --check: missing site/prism-bridge-meta.mjs");
    process.exit(1);
  }
  const currentMeta = fs.readFileSync(SITE_META, "utf8");
  if (currentMeta !== expectedMeta) {
    console.error("sync-prism-wasm --check: site/prism-bridge-meta.mjs is out of sync");
    process.exit(1);
  }
  console.log(
    `sync-prism-wasm --check: verified (cold=${wasmBytes.length}B, uorc_wasm=${uorcWasmBytes.length}B, stream=${uorcStreamBytes.length}B, kernels=${unifiedKernelCount}, coded_tiers=${codedTierCount})`,
  );
} else {
  fs.writeFileSync(SITE_META, expectedMeta, "utf8");
  console.log(
    `sync-prism-wasm: wrote site/verdant-wasm.wasm (${wasmBytes.length}B), site/verdant-uorc.wasm (${uorcWasmBytes.length}B), site/scene-stream.uorc (${uorcStreamBytes.length}B), and site/prism-bridge-meta.mjs`,
  );
}
