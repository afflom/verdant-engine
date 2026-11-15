#!/usr/bin/env node
// Builds crates/verdant-wasm for wasm32-unknown-unknown, syncs site/verdant-wasm.wasm,
// executes the compiled Prism Wasm exports under Node WebAssembly, and emits
// site/prism-bridge-meta.mjs mechanically so AudioWorklet and Worker modules consume
// verified Prism kernel values with zero hand-authored literals.

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
const SITE_META = path.join(ROOT, "site", "prism-bridge-meta.mjs");

const checkOnly = process.argv.includes("--check");

if (!checkOnly) {
  execFileSync(
    "cargo",
    ["build", "-p", "verdant-wasm", "--target", "wasm32-unknown-unknown", "--release", "--locked"],
    { cwd: ROOT, stdio: "inherit" },
  );
}

const sourceWasmPath = fs.existsSync(WASM_TARGET_OUT) ? WASM_TARGET_OUT : SITE_WASM;
if (!fs.existsSync(sourceWasmPath)) {
  console.error(`sync-prism-wasm: missing ${sourceWasmPath}`);
  process.exit(1);
}

const wasmBytes = fs.readFileSync(sourceWasmPath);
if (
  wasmBytes.length < 8 ||
  wasmBytes[0] !== 0x00 ||
  wasmBytes[1] !== 0x61 ||
  wasmBytes[2] !== 0x73 ||
  wasmBytes[3] !== 0x6d
) {
  console.error("sync-prism-wasm: invalid WebAssembly magic header");
  process.exit(1);
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
const chimeDenom = ex.verdant_wasm_chime_mode_denom() >>> 0;
const chimeNumers = [0, 1, 2, 3, 4].map((i) => ex.verdant_wasm_chime_mode_numer(i) >>> 0);
const rotundaRt60Ms = ex.verdant_wasm_sabine_rt60_millis(1800, 96000) >>> 0;
const walkablePolygons = ex.verdant_wasm_navmesh_polygon_count() >>> 0;
const blueprintCount = ex.verdant_wasm_blueprint_count() >>> 0;
const csmSplits = [0, 1, 2, 3].map((i) =>
  Number(ex.verdant_wasm_csm_split_meters(0.25, 140.0, i).toFixed(4)),
);
const ringCapacity = ex.verdant_wasm_worker_ring_capacity() >>> 0;
const snapshotMaxBytes = ex.verdant_wasm_snapshot_max_bytes() >>> 0;
const maxEpochRank = ex.verdant_wasm_max_epoch_rank() >>> 0;
const snapshotMinWireBytes = ex.verdant_wasm_snapshot_min_wire_bytes() >>> 0;

const expectedMeta = `// Generated mechanically by scripts/sync-prism-wasm.mjs from Prism-verified verdant-wasm.
// Do not edit by hand.

export const PRISM_DEFAULT_WORLD_SEED = ${Number(defaultSeed)};
export const PRISM_MAX_EPOCH_RANK = ${maxEpochRank};
export const PRISM_SNAPSHOT_MIN_WIRE_BYTES = ${snapshotMinWireBytes};
export const PRISM_AUDIO_QUANTUM_FRAMES = ${quantumFrames};
export const PRISM_AUDIO_STEREO_CHANNELS = ${stereoChannels};
export const PRISM_CHIME_MODE_DENOM = ${chimeDenom};
export const PRISM_CHIME_MODE_NUMERS = Object.freeze(${JSON.stringify(chimeNumers)});
export const PRISM_ROTUNDA_SABINE_RT60_MS = ${rotundaRt60Ms};
export const PRISM_SANCTUARY_WALKABLE_POLYGONS = ${walkablePolygons};
export const PRISM_BUILDER_BLUEPRINT_COUNT = ${blueprintCount};
export const PRISM_CSM_SPLIT_METERS = Object.freeze(${JSON.stringify(csmSplits)});
export const PRISM_WORKER_RING_CAPACITY = ${ringCapacity};
export const PRISM_SNAPSHOT_URL_MAX_BYTES = ${snapshotMaxBytes};
`;

if (checkOnly) {
  if (!fs.existsSync(SITE_WASM) || !fs.existsSync(SITE_META)) {
    console.error("sync-prism-wasm --check: missing site/verdant-wasm.wasm or site/prism-bridge-meta.mjs");
    process.exit(1);
  }
  const currentMeta = fs.readFileSync(SITE_META, "utf8");
  if (currentMeta !== expectedMeta) {
    console.error("sync-prism-wasm --check: site/prism-bridge-meta.mjs is out of sync");
    process.exit(1);
  }
  console.log(
    `sync-prism-wasm --check: verified (${wasmBytes.length} bytes, ${walkablePolygons} walkable polys, RT60=${rotundaRt60Ms}ms)`,
  );
} else {
  fs.writeFileSync(SITE_WASM, wasmBytes);
  fs.writeFileSync(SITE_META, expectedMeta, "utf8");
  console.log(
    `sync-prism-wasm: wrote site/verdant-wasm.wasm (${wasmBytes.length} bytes) and site/prism-bridge-meta.mjs`,
  );
}
