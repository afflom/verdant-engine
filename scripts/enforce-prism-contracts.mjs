#!/usr/bin/env node
// Mechanical Component Contract & Prism-Authority Enforcement Gate for Verdant Engine.
// Enforces that every subsystem of verdant-engine derives from Prism (`site/app.holo`,
// `site/verdant-wasm.wasm`, `site/verdant-uorc.wasm`, `site/prism-bridge-meta.mjs`) and that
// no runtime, Studio, or starter surface bypasses, stubs, or spoofs modeled engine contracts.

import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readdirSync, readFileSync, renameSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { kappa, load } from "../try/vendor/prism-code/src/index.mjs";
import * as bridgeMeta from "../site/prism-bridge-meta.mjs";
import {
  STARTER_ARCHETYPES,
  STUDIO_ROLE_KEYS,
  generateStarterProjectFiles,
} from "../site/studio.mjs";
import {
  CATALOG_DOMAINS,
  LIFECYCLE_STAGES,
  PRISM_MODEL_CATALOGS,
  VERDANT_PLATFORM_ORGANS,
} from "../site/prism-catalogs.mjs";

const ROOT_DIR = fileURLToPath(new URL("../", import.meta.url));
const utf8Enc = new TextEncoder();
const utf8Dec = new TextDecoder();

export const REQUIRED_HOLO_QUERIES = Object.freeze([
  "summary",
  "summary:hd",
  "lod:0",
  "lod:1",
  "lod:2",
  "lod:3",
  "lod:4",
  "kernel:0",
  "kernel:1",
  "kernel:2",
  "kernel:3",
  "kernel:4",
  "kernel:5",
  "kernel:6",
  "kernel:7",
  "kernel:8",
  "kernel:9",
  "kernel:10",
  "kernel:11",
  "kernel:12",
  "kernel:13",
  "kernel:14",
  "coded:0",
  "coded:1",
  "coded:2",
  "scene:lod:0",
  "scene:lod:1",
  "scene:lod:2",
  "scene:lod:3",
  "scene:lod:4",
  "scene:node:0",
  "scene:node:1",
  "scene:node:2",
  "scene:node:3",
  "scene:node:4",
  "scene:node:5",
  "scene:node:6",
  "scene:acoustics",
  "scene:pipeline",
  "scene:animation",
  "scene:rendergraph",
  "scene:collision",
  "scene:navmesh",
  "scene:wasmbridge",
  "scene:runtime",
  "scene:uorc",
  "scene:joints",
  "scene:rollback",
  "scene:ui",
  "scene:cybernetics",
  "scene:stitch",
  "scene:lawful",
  "scene:character",
  "scene:universe",
  "scene:narrative",
  "scene:telos",
  "scene:studio",
  "scene:perf",
  "scene:aqm",
  "scene:stability",
  "scene:uorc-exclusive",
  "scene:observer",
  "scene:stakeholders",
  "scene:language",
  "scene:channels",
]);

export const REQUIRED_HOLO_REFUSALS = Object.freeze([
  ["kernel:zerodim", "refused:zero-extent-contraction"],
  ["kernel:underflow", "refused:accumulator-below-laurent-bound"],
  ["", "invalid request"],
  ["unknown", "invalid request"],
]);

export const REQUIRED_CORE_WASM_EXPORTS = Object.freeze([
  "verdant_wasm_init",
  "verdant_wasm_default_seed",
  "verdant_wasm_set_lod_rank",
  "verdant_wasm_active_lod_rank",
  "verdant_wasm_max_lod_rank",
  "verdant_wasm_hd_max_lod_rank",
  "verdant_wasm_scene_rebuild_buffers",
  "verdant_wasm_static_epoch",
  "verdant_wasm_static_vert_count",
  "verdant_wasm_static_idx_count",
  "verdant_wasm_dyn_vert_count",
  "verdant_wasm_dyn_idx_count",
  "verdant_wasm_step_lawful",
  "verdant_wasm_step_observer",
  "verdant_wasm_camera_set_mode",
  "verdant_wasm_camera_set_anchor",
  "verdant_wasm_camera_intent",
  "verdant_wasm_camera_set_pose_intent",
  "verdant_wasm_camera_clearance_milli",
  "verdant_wasm_bvh_visible_count",
  "verdant_wasm_binaural_pan",
  "verdant_wasm_binaural_gain",
  "verdant_wasm_binaural_itd_micros",
  "verdant_wasm_csm_split_meters",
  "verdant_wasm_governor_scale_permille",
  "verdant_wasm_unified_tensor_kernel_count",
  "verdant_wasm_coded_tier_count",
  "verdant_wasm_navmesh_polygon_count",
]);

export const REQUIRED_UORC_WASM_EXPORTS = Object.freeze([
  ...REQUIRED_CORE_WASM_EXPORTS,
  "verdant_wasm_route_prompt_io",
  "verdant_wasm_set_organ_grant",
  "verdant_wasm_verify_ui",
  "verdant_wasm_story_interact_at",
  "verdant_wasm_story_status_aux",
  "verdant_wasm_glb_import_io",
  "verdant_wasm_glb_export_aux",
  "verdant_wasm_snapshot_encode",
  "verdant_wasm_snapshot_decode_and_restore",
  "verdant_wasm_uorc_decode_and_restore",
  "verdant_wasm_stitch_shard_io",
  "verdant_wasm_stitch_recover_io",
  "verdant_wasm_configure_instanced_workload",
  "verdant_wasm_render_tensor_rgba",
  "verdant_wasm_aqm_benchmark_aux",
  "verdant_wasm_stability_audit_aux",
  "verdant_wasm_uorc_exclusive_audit_aux",
  "verdant_wasm_scene_outliner_aux",
  "verdant_wasm_load_domain_pack",
  "verdant_wasm_set_npc_directive",
  "verdant_wasm_dialogue_turn_io",
  "verdant_wasm_observe_comm_io",
  "verdant_wasm_observer_status_aux",
]);

export const CANONICAL_ORGAN_SLUGS = Object.freeze([
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
]);

export const CANONICAL_DEVREL_CONTROL_IDS = Object.freeze([
  "ve-canvas",
  "ve-lod-btn",
  "ve-sim-btn",
  "ve-cam-btn",
  "ve-audio-btn",
  "ve-glb-btn",
  "ve-snap-btn",
  "ve-docs-link",
  "ve-uorc-btn",
  "ve-studio-btn",
  "ve-render-btn",
  "ve-import-btn",
  "ve-mesh-btn",
  "ve-prompt-btn",
]);

export const EXPECTED_STUDIO_ROLE_KEYS = Object.freeze([
  "dashboard",
  "code",
  "design",
  "audio",
  "narrative",
  "director",
  "gameplay",
]);

const HOLO_QUERY_SPEC_DIGEST = createHash("sha256")
  .update(JSON.stringify({ q: REQUIRED_HOLO_QUERIES, r: REQUIRED_HOLO_REFUSALS }))
  .digest("hex");

/**
 * Gate C1: Verifies cryptographic BLAKE3 self-fingerprint, full acceptance vector replay,
 * all 61 positive Engine.lex.tex queries, and all 4 modeled negative refusal vectors on `site/app.holo`.
 * Uses a content-addressed BLAKE3 cache (`node_modules/.cache/prism-holo-contract-v1.json`) ONLY after
 * recomputing the full BLAKE3 `kappa` self-fingerprint of `holoBuffer` on every single run.
 */
export async function verifyPrismHoloContract(holoBuffer, options = {}) {
  const { useCache = true, rootDir = ROOT_DIR } = options;
  if (holoBuffer.length < 64) {
    throw new Error("too short to be a holo");
  }
  const fpBytes = holoBuffer.subarray(holoBuffer.length - 32);
  const fpHex = Buffer.from(fpBytes).toString("hex");
  if (kappa(holoBuffer.subarray(0, holoBuffer.length - 32)) !== `blake3:${fpHex}`) {
    throw new Error("Holo BLAKE3 self-fingerprint fails verification");
  }

  const cachePath = join(rootDir, "node_modules", ".cache", "prism-holo-contract-v1.json");
  if (useCache && existsSync(cachePath)) {
    try {
      const cached = JSON.parse(readFileSync(cachePath, "utf8"));
      const posOk = REQUIRED_HOLO_QUERIES.every((q) => cached.verdicts?.[q]?.startsWith("conformant:"));
      const negOk = REQUIRED_HOLO_REFUSALS.every(([req, exp]) => cached.refusals?.[req] === exp);
      if (
        cached.fingerprint === fpHex &&
        cached.specDigest === HOLO_QUERY_SPEC_DIGEST &&
        Number.isInteger(cached.vectors) &&
        cached.vectors >= 42 &&
        posOk &&
        negOk
      ) {
        return {
          vectors: cached.vectors,
          fingerprint: fpHex,
          verdicts: cached.verdicts,
          refusals: cached.refusals,
          cached: true,
        };
      }
    } catch {
      // Fall through to full Wasm vector + query replay on any corrupt/stale cache entry.
    }
  }

  const app = await load(holoBuffer);
  const acc = await app.acceptance();
  if (!(acc.total > 0 && acc.passed === acc.total)) {
    throw new Error(`Holo acceptance failed: ${acc.passed}/${acc.total} vectors passed`);
  }

  // Index responses already replayed by `app.acceptance()` so only non-vector queries need extra dispatch:
  const vectorMap = new Map();
  for (const vec of app.vectors ?? []) {
    vectorMap.set(utf8Dec.decode(vec.request), utf8Dec.decode(vec.response));
  }

  const verdicts = {};
  for (const query of REQUIRED_HOLO_QUERIES) {
    const raw = vectorMap.has(query)
      ? vectorMap.get(query)
      : utf8Dec.decode(await app.dispatch(utf8Enc.encode(query)));
    if (!raw.startsWith("conformant:")) {
      throw new Error(`Holo query "${query}" failed conformance: ${raw}`);
    }
    verdicts[query] = raw;
  }

  const refusals = {};
  for (const [req, expected] of REQUIRED_HOLO_REFUSALS) {
    const raw = vectorMap.has(req)
      ? vectorMap.get(req)
      : utf8Dec.decode(await app.dispatch(utf8Enc.encode(req)));
    if (raw !== expected) {
      throw new Error(`Holo negative query "${req}" expected "${expected}", got "${raw}"`);
    }
    refusals[req] = raw;
  }

  if (useCache) {
    try {
      mkdirSync(dirname(cachePath), { recursive: true });
      const tmpPath = `${cachePath}.${process.pid}.tmp`;
      writeFileSync(
        tmpPath,
        JSON.stringify({
          fingerprint: fpHex,
          specDigest: HOLO_QUERY_SPEC_DIGEST,
          vectors: acc.total,
          verdicts,
          refusals,
        }),
      );
      renameSync(tmpPath, cachePath);
    } catch {
      // Cache write failure never blocks verification.
    }
  }

  return { vectors: acc.total, fingerprint: fpHex, verdicts, refusals, cached: false };
}

/**
 * Gate C2: Instantiates both Prism-verified Wasm binaries and verifies exports,
 * constant parity with `site/prism-bridge-meta.mjs`, SLM routing, FK-07 denial-wins,
 * and Stitch 2-of-3 recovery in Wasm linear memory.
 */
export async function verifyWasmBinaryContract(coreWasmBytes, uorcWasmBytes, meta = bridgeMeta) {
  const [{ instance: coreInst }, { instance: uorcInst }] = await Promise.all([
    WebAssembly.instantiate(coreWasmBytes, {}),
    WebAssembly.instantiate(uorcWasmBytes, {}),
  ]);
  const core = coreInst.exports;
  const uorc = uorcInst.exports;

  for (const fn of REQUIRED_CORE_WASM_EXPORTS) {
    if (typeof core[fn] !== "function") {
      throw new Error(`verdant-wasm.wasm missing required Prism export: ${fn}`);
    }
  }
  for (const fn of REQUIRED_UORC_WASM_EXPORTS) {
    if (typeof uorc[fn] !== "function") {
      throw new Error(`verdant-uorc.wasm missing required Prism export: ${fn}`);
    }
  }

  const defaultSeed = Number(core.verdant_wasm_default_seed());
  if (defaultSeed !== meta.PRISM_DEFAULT_WORLD_SEED) {
    throw new Error(`Default seed drift: wasm=${defaultSeed} meta=${meta.PRISM_DEFAULT_WORLD_SEED}`);
  }
  core.verdant_wasm_init(BigInt(defaultSeed));
  uorc.verdant_wasm_init(BigInt(defaultSeed));

  if ((core.verdant_wasm_unified_tensor_kernel_count() >>> 0) !== meta.PRISM_UNIFIED_KERNEL_COUNT) {
    throw new Error("Unified tensor kernel count mismatch against prism-bridge-meta.mjs");
  }
  if ((core.verdant_wasm_coded_tier_count() >>> 0) !== meta.PRISM_CODED_TIER_COUNT) {
    throw new Error("Coded tier count mismatch against prism-bridge-meta.mjs");
  }
  if ((core.verdant_wasm_tile_edge_pixels() >>> 0) !== meta.PRISM_TILE_EDGE_PIXELS) {
    throw new Error("Tile edge pixel dimension mismatch against prism-bridge-meta.mjs");
  }
  if ((core.verdant_wasm_worker_ring_capacity() >>> 0) !== meta.PRISM_WORKER_RING_CAPACITY) {
    throw new Error("Worker ring capacity mismatch against prism-bridge-meta.mjs");
  }
  if ((core.verdant_wasm_modal_bar_denom() >>> 0) !== meta.PRISM_MODAL_BAR_DENOM) {
    throw new Error("Modal bar denominator mismatch against prism-bridge-meta.mjs");
  }
  const wasmNumers = [0, 1, 2, 3, 4].map((i) => core.verdant_wasm_modal_bar_numer(i) >>> 0);
  if (JSON.stringify(wasmNumers) !== JSON.stringify(meta.PRISM_MODAL_BAR_NUMERS)) {
    throw new Error("Modal bar numerators mismatch against prism-bridge-meta.mjs");
  }
  if ((core.verdant_wasm_sabine_rt60_millis(1800, 96000) >>> 0) !== meta.PRISM_CHAMBER_SABINE_RT60_MS) {
    throw new Error("Sabine RT60 chamber constant mismatch against prism-bridge-meta.mjs");
  }
  if ((core.verdant_wasm_navmesh_polygon_count() >>> 0) !== meta.PRISM_TERRAIN_WALKABLE_POLYGONS) {
    throw new Error("Walkable polygon count mismatch against prism-bridge-meta.mjs");
  }

  const stageUorcIo = (bytes) => {
    const ptr = uorc.verdant_wasm_io_resize(bytes.byteLength >>> 0) >>> 0;
    new Uint8Array(uorc.memory.buffer, ptr, bytes.byteLength).set(bytes);
  };
  const readUorcAux = () => {
    const len = uorc.verdant_wasm_aux_len() >>> 0;
    const ptr = uorc.verdant_wasm_aux_ptr() >>> 0;
    return len > 0 && ptr > 0 ? new Uint8Array(uorc.memory.buffer, ptr, len).slice() : new Uint8Array(0);
  };

  // Verify Wasm CodedSlmDirector routing + negative empty-prompt refusal (R5):
  stageUorcIo(new Uint8Array(0));
  if ((uorc.verdant_wasm_route_prompt_io() >>> 0) !== 0) {
    throw new Error("Wasm CodedSlmDirector must refuse empty prompt");
  }
  stageUorcIo(new TextEncoder().encode("focus observer core gimbal"));
  if ((uorc.verdant_wasm_route_prompt_io() >>> 0) === 0) {
    throw new Error("Wasm CodedSlmDirector failed to admit valid prompt");
  }
  const slmText = new TextDecoder().decode(readUorcAux());
  if (!slmText.startsWith("focus-observer|")) {
    throw new Error(`Unexpected CodedSlmDirector output: ${slmText}`);
  }

  // Verify FK-07 Denial-Wins inside Wasm (grant=1 then deny=2 then grant=1 must remain denied=0):
  if (uorc.verdant_wasm_set_organ_grant(3, 1) !== 1) {
    throw new Error("AudioWorklet organ should allow after explicit grant");
  }
  if (uorc.verdant_wasm_set_organ_grant(3, 2) !== 0) {
    throw new Error("AudioWorklet organ must be denied after explicit deny");
  }
  if (uorc.verdant_wasm_set_organ_grant(3, 1) !== 0) {
    throw new Error("FK-07 violation: subsequent grant overrode explicit denial");
  }
  uorc.verdant_wasm_set_organ_grant(0, 0);

  // Verify Wasm Stitch 2-of-3 XOR parity recovery + negative tamper & duplicate refusal (VE-24 / R5):
  const snapLen = uorc.verdant_wasm_snapshot_encode() >>> 0;
  if (snapLen === 0) throw new Error("verdant_wasm_snapshot_encode returned 0 bytes");
  const snapPtr = uorc.verdant_wasm_io_ptr() >>> 0;
  const snapBytes = new Uint8Array(uorc.memory.buffer, snapPtr, snapLen).slice();
  const shards = [0, 1, 2].map((idx) => {
    stageUorcIo(snapBytes);
    const sLen = uorc.verdant_wasm_stitch_shard_io(idx) >>> 0;
    if (sLen === 0) throw new Error(`Stitch shard ${idx} returned 0 bytes`);
    return readUorcAux();
  });
  const pair = new Uint8Array(shards[0].byteLength + shards[2].byteLength);
  pair.set(shards[0], 0);
  pair.set(shards[2], shards[0].byteLength);
  stageUorcIo(pair);
  const recLen = uorc.verdant_wasm_stitch_recover_io(shards[0].byteLength >>> 0) >>> 0;
  const recovered = readUorcAux();
  if (recLen !== snapBytes.byteLength || recovered.some((b, i) => b !== snapBytes[i])) {
    throw new Error("Stitch 2-of-3 Wasm recovery failed bit-exact verification");
  }

  const tamperedPair = pair.slice();
  tamperedPair[shards[0].byteLength - 1] ^= 0x5a;
  stageUorcIo(tamperedPair);
  if ((uorc.verdant_wasm_stitch_recover_io(shards[0].byteLength >>> 0) >>> 0) !== 0) {
    throw new Error("Stitch Wasm recovery failed to refuse 1-bit tampered STCH shard");
  }

  const dupPair = new Uint8Array(shards[1].byteLength * 2);
  dupPair.set(shards[1], 0);
  dupPair.set(shards[1], shards[1].byteLength);
  stageUorcIo(dupPair);
  if ((uorc.verdant_wasm_stitch_recover_io(shards[1].byteLength >>> 0) >>> 0) !== 0) {
    throw new Error("Stitch Wasm recovery failed to refuse duplicate STCH shard pair (1, 1)");
  }

  // Verify Wasm catalog domain pack switching, NavMesh NPC directives, & ISO 24617-2 dialogue + abstention (VE-39/VE-40):
  if ((uorc.verdant_wasm_load_domain_pack(1) >>> 0) === 0) {
    throw new Error("verdant_wasm_load_domain_pack(1) failed to instantiate meridian-observatory");
  }
  const meridianObs = JSON.parse(new TextDecoder().decode(readUorcAux()));
  if (meridianObs.activeDomain?.slug !== "meridian-observatory" || !meridianObs.observerKappa) {
    throw new Error("meridian-observatory status JSON missing activeDomain slug or observerKappa");
  }
  if ((uorc.verdant_wasm_load_domain_pack(99) >>> 0) !== 0) {
    throw new Error("verdant_wasm_load_domain_pack(99) must refuse out-of-bounds domain index");
  }
  if ((uorc.verdant_wasm_set_npc_directive(0, 1) >>> 0) === 0) {
    throw new Error("verdant_wasm_set_npc_directive(0, 1) failed to plan A* NavMesh corridor");
  }
  stageUorcIo(new TextEncoder().encode("What is the secret Mars pizza topping?"));
  if ((uorc.verdant_wasm_dialogue_turn_io(0) >>> 0) === 0) {
    throw new Error("verdant_wasm_dialogue_turn_io(0) failed to evaluate unstated question");
  }
  const abstainTurn = JSON.parse(new TextDecoder().decode(readUorcAux()));
  if (!abstainTurn.abstainedUnknown || abstainTurn.replyText !== "Unknown") {
    throw new Error(`Expected Unknown abstention for unstated NPC question, got: ${JSON.stringify(abstainTurn)}`);
  }
  uorc.verdant_wasm_load_domain_pack(0);

  return { defaultSeed, snapLen, shardLen: shards[0].byteLength };
}

/**
 * Gate C3: Rejects handwritten JS prompt-routing bypasses or hardcoded SLM confidence/logit
 * spoofing in `routeStudioPrompt` (`site/studio.mjs`).
 */
export function verifyZeroSlmBypassContract(studioSource) {
  const fnIdx = studioSource.indexOf("export async function routeStudioPrompt");
  if (fnIdx < 0) {
    throw new Error("studio.mjs is missing exported routeStudioPrompt");
  }
  const fnBody = studioSource.slice(fnIdx);
  const workerCallIdx = fnBody.indexOf('callWorker("route-prompt"');
  if (workerCallIdx < 0) {
    throw new Error('routeStudioPrompt must delegate prompts to callWorker("route-prompt")');
  }
  const beforeWorker = fnBody.slice(0, workerCallIdx);
  if (/return\s*\{[^}]*scoreMilli\s*:/.test(beforeWorker)) {
    throw new Error("Bypass detected: routeStudioPrompt returns fabricated scoreMilli before Wasm CodedSlmDirector");
  }
  if (/scoreMilli\s*:\s*\d+/.test(fnBody) || /rawLogit\s*:\s*\d+/.test(fnBody)) {
    throw new Error("Spoof detected: routeStudioPrompt contains hardcoded scoreMilli/rawLogit numeric literals");
  }
  if (!fnBody.includes("PRISM_MAX_LOD_RANK")) {
    throw new Error("routeStudioPrompt must stream base UORC intent using PRISM_MAX_LOD_RANK");
  }
  if (/effectiveSlug\s*=/.test(fnBody) || /if\s*\(\s*\/\\b\(/.test(fnBody)) {
    throw new Error("Bypass detected: routeStudioPrompt overrides Wasm CodedSlmDirector slug via handwritten JS regexes");
  }
}

/**
 * Gate C4: Enforces FK-07 capability lattice across `studio.mjs`, `engine-worker.mjs`,
 * `card.mjs`, and `stitch-mesh.mjs`, including isolated per-organ reset (never wiping sibling denials).
 */
export function verifyFk07CapabilityContract({ studioSource, workerSource, cardSource, stitchSource }) {
  for (const slug of CANONICAL_ORGAN_SLUGS) {
    if (!studioSource.includes(`"${slug}"`)) {
      throw new Error(`studio.mjs CONTAINER_ORGANS missing canonical organ "${slug}"`);
    }
  }
  if (!workerSource.includes("organModes") || !workerSource.includes("syncOrgansToWasm")) {
    throw new Error("engine-worker.mjs must maintain authoritative per-organ FK-07 organModes and syncOrgansToWasm");
  }
  if (!cardSource.includes("isOrganAllowed") || !cardSource.includes("ContainerOrganDenied")) {
    throw new Error("card.mjs must enforce isOrganAllowed and surface typed ContainerOrganDenied refusals");
  }
  if (!stitchSource.includes("broadcast-channel") || !stitchSource.includes("webrtc-stitch-mesh")) {
    throw new Error("stitch-mesh.mjs must gate BroadcastChannel and WebRTC DataChannel via FK-07 organs");
  }
}

/**
 * Gate C5: Enforces unified Wasm host state handoff (`wasm` -> `uorcWasm`), Wasm CSM splits,
 * and Woodworth ITD + Hadamard FDN audio streaming.
 */
export function verifyUnifiedHostAndAudioContract({ workerSource, rendererSource, audioWorkletSource }) {
  for (const token of [
    "activeSeed",
    "walkJournal",
    "syncOrgansToWasm",
    "verdant_wasm_csm_split_meters",
    "verdant_wasm_binaural_itd_micros",
    "binauralItdMicros",
  ]) {
    if (!workerSource.includes(token)) {
      throw new Error(`engine-worker.mjs missing unified host contract token: ${token}`);
    }
  }
  if (!rendererSource.includes("setCsmSplits")) {
    throw new Error("scene-renderer.mjs must expose setCsmSplits driven by Prism CSM splits");
  }
  if (/Math\.round\(\s*fx\s*\*\s*0\.5\s*\)\s*\*\s*2/.test(rendererSource)) {
    throw new Error("scene-renderer.mjs still uses coarse 2m world-snapped shadow frustum instead of light-space texel snapping");
  }
  for (const token of ["PRISM_MODAL_BAR_NUMERS", "itdMicros", "fdnDelays"]) {
    if (!audioWorkletSource.includes(token)) {
      throw new Error(`audio-worklet.mjs missing Prism spatial/FDN acoustic token: ${token}`);
    }
  }
}

/**
 * Scans Rust source code (excluding comments, normal/raw/byte string & char literals, and
 * `#[cfg(test)]` blocks, matching `crates/fidelity/src/ratchet.rs`) and rejects any numeric
 * literal outside `{0, 1}` (`FI-08` zero unexplained magic constants).
 */
export function verifyRustZeroMagicConstants(rustSource, label = "src/main.rs") {
  // 1. Single-pass state machine stripping `//`, nested `/* ... */`, `r#"..."#`, `"..."`, and `'c'`:
  let sanitized = "";
  let i = 0;
  const n = rustSource.length;
  while (i < n) {
    const ch = rustSource[i];
    const next = i + 1 < n ? rustSource[i + 1] : "";
    if (ch === "/" && next === "/") {
      while (i < n && rustSource[i] !== "\n") {
        sanitized += " ";
        i++;
      }
      continue;
    }
    if (ch === "/" && next === "*") {
      let depth = 1;
      sanitized += "  ";
      i += 2;
      while (i < n && depth > 0) {
        if (rustSource[i] === "/" && rustSource[i + 1] === "*") {
          depth++;
          sanitized += "  ";
          i += 2;
        } else if (rustSource[i] === "*" && rustSource[i + 1] === "/") {
          depth--;
          sanitized += "  ";
          i += 2;
        } else {
          sanitized += rustSource[i] === "\n" ? "\n" : " ";
          i++;
        }
      }
      continue;
    }
    // Raw string `r#"..."#` or `br#"..."#`:
    if (ch === "r" || (ch === "b" && next === "r")) {
      let p = ch === "b" ? i + 2 : i + 1;
      let hashes = 0;
      while (p < n && rustSource[p] === "#") {
        hashes++;
        p++;
      }
      if (p < n && rustSource[p] === '"') {
        const closeSeq = '"' + "#".repeat(hashes);
        const endIdx = rustSource.indexOf(closeSeq, p + 1);
        const stop = endIdx >= 0 ? endIdx + closeSeq.length : n;
        for (let k = i; k < stop; k++) sanitized += rustSource[k] === "\n" ? "\n" : " ";
        i = stop;
        continue;
      }
    }
    // Normal or byte string `"..."` / `b"..."`:
    if (ch === '"' || (ch === "b" && next === '"')) {
      let p = ch === "b" ? i + 2 : i + 1;
      while (p < n) {
        if (rustSource[p] === "\\") {
          p += 2;
          continue;
        }
        if (rustSource[p] === '"') {
          p++;
          break;
        }
        p++;
      }
      for (let k = i; k < Math.min(p, n); k++) sanitized += rustSource[k] === "\n" ? "\n" : " ";
      i = Math.min(p, n);
      continue;
    }
    // Char literal `'x'` / `'\n'` (not lifetime `'a`):
    if (ch === "'" || (ch === "b" && next === "'")) {
      const q = ch === "b" ? i + 1 : i;
      let p = q + 1;
      if (p < n && rustSource[p] === "\\") {
        p += 2;
        while (p < n && rustSource[p] !== "'" && rustSource[p] !== "\n") p++;
      } else if (p < n && rustSource[p] !== "'" && rustSource[p] !== "\n") {
        p++;
      }
      if (p < n && rustSource[p] === "'") {
        for (let k = i; k <= p; k++) sanitized += " ";
        i = p + 1;
        continue;
      }
    }
    sanitized += ch;
    i++;
  }

  // 2. Blank out `#[cfg(test)]` items (brace-balanced blocks or `;`-terminated statements):
  const chars = sanitized.split("");
  const cfgRegex = /#\s*\[\s*cfg\s*\(\s*test\s*\)\s*\]/g;
  let cm;
  while ((cm = cfgRegex.exec(sanitized)) !== null) {
    let p = cm.index;
    while (p < chars.length && chars[p] !== "{" && chars[p] !== ";") {
      if (chars[p] !== "\n") chars[p] = " ";
      p++;
    }
    if (p < chars.length && chars[p] === ";") {
      chars[p] = " ";
    } else if (p < chars.length && chars[p] === "{") {
      let depth = 1;
      chars[p++] = " ";
      while (p < chars.length && depth > 0) {
        if (chars[p] === "{") depth++;
        else if (chars[p] === "}") depth--;
        if (chars[p] !== "\n") chars[p] = " ";
        p++;
      }
    }
  }
  const codeOnly = chars.join("");

  const numRegex = /(?<![A-Za-z0-9_])(0[xXbBoO][0-9a-fA-F_]+|\d[\d_]*(?:\.[\d_]+)?(?:[eE][+-]?[\d_]+)?)(?:_?(?:u128|i128|usize|isize|u64|i64|u32|i32|u16|i16|u8|i8|f64|f32))?(?![A-Za-z0-9_])/g;
  let match;
  while ((match = numRegex.exec(codeOnly)) !== null) {
    const raw = match[1].replace(/_/g, "");
    let allowed = raw === "0" || raw === "1" || raw === "0.0" || raw === "1.0";
    if (!allowed && /^0[xXbBoO]/.test(raw)) {
      const radix = raw[1].toLowerCase() === "x" ? 16 : raw[1].toLowerCase() === "b" ? 2 : 8;
      const val = Number.parseInt(raw.slice(2), radix);
      allowed = val === 0 || val === 1;
    }
    if (!allowed) {
      throw new Error(`FI-08 magic constant "${match[0]}" detected in ${label}`);
    }
  }
}

/**
 * Gate C6: Enforces that `templates/browser-3d-starter/game.mjs` and all 4 Studio Scaffolder
 * archetypes run on Prism-verified Wasm + Holo + `scene-renderer.mjs` with zero handwritten
 * terrain/physics fakes and zero Rust magic constants.
 */
export function verifyStarterAndScaffolderContract({ starterHtml, starterGameSource }) {
  if (/function\s+terrainHeight\s*\(/.test(starterGameSource)) {
    throw new Error("templates/browser-3d-starter/game.mjs contains handwritten JS terrainHeight bypass instead of Wasm terrain");
  }
  for (const req of [
    "verdant-uorc.wasm",
    "app.holo",
    "scene-renderer.mjs",
    "verdant_wasm_step_lawful",
    "verdant_wasm_terrain_height",
    "verdant_wasm_governor_scale_permille",
    "verdant_wasm_story_interact_at",
    "PRISM_MODAL_BAR_NUMERS",
  ]) {
    if (!starterGameSource.includes(req)) {
      throw new Error(`templates/browser-3d-starter/game.mjs missing required Prism integration "${req}"`);
    }
  }
  if (/backdrop-filter\s*:\s*blur/i.test(starterHtml)) {
    throw new Error("templates/browser-3d-starter/index.html must not apply backdrop-filter blur over WebGL");
  }
  if (/#[0-9a-fA-F]{3,8}\b/.test(starterHtml)) {
    throw new Error("templates/browser-3d-starter/index.html must use DTCG CSS tokens (zero raw #hex colors)");
  }

  const seenMainRs = new Set();
  for (const archetypeIdx of [0, 1, 2, 3]) {
    const files = generateStarterProjectFiles({
      slug: `test-arch-${archetypeIdx}`,
      archetypeIdx,
      seed: 7,
      lodRank: 3,
      cameraMode: archetypeIdx === 1 ? "first" : "third",
    });
    const mainRs = files["src/main.rs"];
    const webMjs = files["web/main.mjs"];
    if (!mainRs || !webMjs || !files["Cargo.toml"] || !files["web/index.html"]) {
      throw new Error(`Scaffolder archetype ${archetypeIdx} missing required project files`);
    }
    if (!mainRs.includes("LawfulCameraRig") || !mainRs.includes("OntologicalRuntimeGovernor")) {
      throw new Error(`Scaffolder archetype ${archetypeIdx} missing LawfulCameraRig or OntologicalRuntimeGovernor`);
    }
    verifyRustZeroMagicConstants(mainRs, `Scaffolder archetype ${archetypeIdx} src/main.rs`);
    seenMainRs.add(mainRs);
    if (!webMjs.includes("game-viewport") || !webMjs.includes("verdant-wasm.wasm") || !webMjs.includes("app.holo")) {
      throw new Error(`Scaffolder archetype ${archetypeIdx} web/main.mjs must verify app.holo and render to #game-viewport`);
    }
  }
  if (seenMainRs.size !== 4) {
    throw new Error(`All 4 Studio Scaffolder archetypes must generate distinct Rust implementations (found ${seenMainRs.size})`);
  }
  const expectedDomainSlugs = ["sunspire-regatta", "meridian-observatory", "glasshouse-chronicle", "starfall-bastion"];
  for (let archetypeIdx = 0; archetypeIdx < 4; archetypeIdx++) {
    const { "src/main.rs": mainRs } = generateStarterProjectFiles({ slug: `arch-${archetypeIdx}`, archetypeIdx, seed: 7, lodRank: 3, cameraMode: "third" });
    if (!mainRs.includes("ObserverPrismHolon") || !mainRs.includes(expectedDomainSlugs[archetypeIdx])) {
      throw new Error(`Scaffolder archetype ${archetypeIdx} must instantiate catalog pack "${expectedDomainSlugs[archetypeIdx]}" via ObserverPrismHolon`);
    }
  }
}

/**
 * Gate C7: Enforces zero `#hex` literals in Studio/Card CSS & Live Doc Embeds, zero `backdrop-filter: blur`
 * over live WebGL surfaces, `#uorc=` snapshot naming, and all canonical DevRel.lex.tex control IDs.
 */
export function verifyStudioSurfaceAndZeroHexContract({ cardHtml, cardCss, studioSource, liveEmbedSource }) {
  for (const id of CANONICAL_DEVREL_CONTROL_IDS) {
    if (!cardHtml.includes(`id="${id}"`)) {
      throw new Error(`site/card.html missing canonical DevRel control id="${id}"`);
    }
  }
  if (/#[0-9a-fA-F]{3,8}\b/.test(cardCss)) {
    throw new Error("site/card.css contains raw #hex color literal (must use DTCG var(--*))");
  }
  if (/backdrop-filter\s*:\s*blur/i.test(cardCss)) {
    throw new Error("site/card.css must not use backdrop-filter: blur over live WebGL canvas");
  }
  const cssMatch = /const STUDIO_CSS = `([\s\S]*?)`;/.exec(studioSource);
  if (!cssMatch) {
    throw new Error("site/studio.mjs missing STUDIO_CSS stylesheet");
  }
  if (/#[0-9a-fA-F]{3,8}\b/.test(cssMatch[1])) {
    throw new Error("site/studio.mjs STUDIO_CSS contains raw #hex color literal");
  }
  if (/backdrop-filter\s*:\s*blur/i.test(cssMatch[1])) {
    throw new Error("site/studio.mjs STUDIO_CSS must not use backdrop-filter: blur over live WebGL canvas");
  }
  if (studioSource.includes("Copy #vdnt=")) {
    throw new Error("site/studio.mjs contains stale #vdnt= button label instead of #uorc=");
  }
  if (typeof liveEmbedSource === "string") {
    if (/#[0-9a-fA-F]{3,8}\b/.test(liveEmbedSource)) {
      throw new Error("site/docs/live-embed.mjs contains raw #hex color literal (must read DTCG CSS custom properties)");
    }
    if (!liveEmbedSource.includes("setCsmSplits")) {
      throw new Error("site/docs/live-embed.mjs must wire Prism verdant_wasm_csm_split_meters into setCsmSplits");
    }
  }
}

/**
 * Gate C8: Enforces separation of the standalone Verdant Engine Demo (`/demo/`) from the
 * role-driven Verdant Studio (`/`), rejects `"stakeholder"` / `"persona"` UI terminology in
 * Studio surfaces, and enforces non-self-referential starter & scaffolder templates.
 */
export function verifyRoleDrivenStudioAndDemoSeparationContract({
  cardHtml,
  cardSource,
  studioSource,
  stitchSource,
  demoHtml,
  demoSource,
  starterHtml,
  roleKeys = STUDIO_ROLE_KEYS,
  archetypes = STARTER_ARCHETYPES,
}) {
  // 1. Zero "stakeholder" or "persona" jargon in Studio UI copy/sources:
  for (const [name, src] of [
    ["site/card.html", cardHtml],
    ["site/card.mjs", cardSource],
    ["site/studio.mjs", studioSource],
    ["site/stitch-mesh.mjs", stitchSource],
  ]) {
    if (/\b(stakeholders?|personas?)\b/i.test(src)) {
      throw new Error(`${name} contains forbidden "stakeholder"/"persona" UI terminology (must refer to roles by function)`);
    }
  }

  // 2. Exact 7 lifecycle role workspaces wired end-to-end:
  if (JSON.stringify(roleKeys) !== JSON.stringify(EXPECTED_STUDIO_ROLE_KEYS)) {
    throw new Error(`STUDIO_ROLE_KEYS mismatch: expected ${EXPECTED_STUDIO_ROLE_KEYS.join(",")}, got ${roleKeys.join(",")}`);
  }
  for (const rk of EXPECTED_STUDIO_ROLE_KEYS) {
    if (!cardSource.includes(`"${rk}"`)) {
      throw new Error(`site/card.mjs must wire role button key "${rk}" into Verdant Studio`);
    }
  }

  // 3. Dedicated Verdant Engine Demo (`/demo/`) separated from Verdant Studio (`/`):
  if (!cardHtml.includes('id="ve-demo-link"') || !cardHtml.includes('href="./demo/"')) {
    throw new Error('site/card.html must expose dedicated Engine Demo link (id="ve-demo-link" href="./demo/")');
  }
  if (!demoHtml || !demoSource) {
    throw new Error("Standalone Verdant Engine Demo (`site/demo/index.html` and `site/demo/demo.mjs`) must exist");
  }
  if (/#[0-9a-fA-F]{3,8}\b/.test(demoHtml)) {
    throw new Error("site/demo/index.html contains raw #hex color literal (must use DTCG CSS tokens)");
  }
  if (/backdrop-filter\s*:\s*blur/i.test(demoHtml)) {
    throw new Error("site/demo/index.html must not apply backdrop-filter: blur over WebGL");
  }
  for (const req of [
    "/app.holo",
    "/verdant-uorc.wasm",
    "verdant_wasm_render_tensor_rgba",
    "verdant_wasm_aqm_benchmark_aux",
    "verdant_wasm_stability_audit_aux",
    "verdant_wasm_uorc_exclusive_audit_aux",
    "verdant_wasm_configure_instanced_workload",
  ]) {
    if (!demoSource.includes(req)) {
      throw new Error(`site/demo/demo.mjs missing required live Prism engine benchmark integration "${req}"`);
    }
  }

  // 4. Non-self-referential starter game & Studio project templates:
  const titleMatch = /<title>([\s\S]*?)<\/title>/i.exec(starterHtml);
  const h1Match = /<h1[^>]*>([\s\S]*?)<\/h1>/i.exec(starterHtml);
  const starterHeadings = `${titleMatch?.[1] || ""} ${h1Match?.[1] || ""}`;
  if (/verdant[\s-]*(studio|engine)/i.test(starterHeadings)) {
    throw new Error(`Starter game title/h1 must be non-self-referential (found "${starterHeadings.trim()}")`);
  }
  if (!Array.isArray(archetypes) || archetypes.length < 4) {
    throw new Error("STARTER_ARCHETYPES must define at least 4 distinct external project templates");
  }
  for (const arch of archetypes) {
    const label = `${arch.slug} ${arch.title} ${arch.name}`;
    if (/verdant[\s-]*(studio|engine)/i.test(label)) {
      throw new Error(`Studio project template "${arch.title}" is self-referential (must depict an external app/game)`);
    }
  }
}

/**
 * Gate C9: Enforces exact 15+3+4 Prism kernel/codec/refusal alignment on `/demo/`, rejects dead legacy
 * aliases (`LEGACY_TAB_ALIASES`, `shareVdntSnapshot`, `#studio=create`), enforces unified 140.0m CSM
 * horizon (zero `160.0` drift), and enforces single-elevation hairline `.ve-m-log` styling.
 */
export function verifyPrismKernelAlignmentAndCleanRefactorContract({
  demoSource,
  demoHtml,
  studioDataSource,
  studioScaffolderSource,
  studioSource,
  cardSource,
  workerSource,
  starterGameSource,
  liveEmbedSource,
  docsIndexHtml,
}) {
  if (/BooleanOrAnd/.test(demoSource) || /\bBoolean\b/.test(demoHtml)) {
    throw new Error("Demo surface contains unmodeled BooleanOrAnd/Boolean kernel drift (must derive strictly from app.holo)");
  }
  for (const token of ["parseHoloFields", "deriveKernelRowDisplay"]) {
    if (!demoSource.includes(token)) {
      throw new Error(`site/demo/demo.mjs missing live Holo verdict parser "${token}"`);
    }
  }
  for (let k = 0; k < 15; k++) {
    if (!demoSource.includes(`"kernel:${k}"`)) {
      throw new Error(`site/demo/demo.mjs missing canonical Prism kernel query "kernel:${k}"`);
    }
  }
  for (let c = 0; c < 3; c++) {
    if (!demoSource.includes(`"coded:${c}"`)) {
      throw new Error(`site/demo/demo.mjs missing canonical Prism coded-computation query "coded:${c}"`);
    }
  }

  for (const [label, src] of [
    ["site/studio-data.mjs", studioDataSource],
    ["site/studio-scaffolder.mjs", studioScaffolderSource],
    ["site/studio.mjs", studioSource],
    ["site/card.mjs", cardSource],
    ["site/docs/index.html", docsIndexHtml],
  ]) {
    if (/LEGACY_TAB_ALIASES|shareVdntSnapshot|#studio=create\b/.test(src)) {
      throw new Error(`${label} contains stale legacy alias (LEGACY_TAB_ALIASES, shareVdntSnapshot, or #studio=create)`);
    }
  }

  for (const [label, src] of [
    ["site/demo/demo.mjs", demoSource],
    ["site/studio-scaffolder.mjs", studioScaffolderSource],
    ["site/card.mjs", cardSource],
    ["site/engine-worker.mjs", workerSource],
    ["templates/browser-3d-starter/game.mjs", starterGameSource],
    ["site/docs/live-embed.mjs", liveEmbedSource],
  ]) {
    if (/\b160\.0\b/.test(src)) {
      throw new Error(`${label} contains 160.0m CSM/perspective far-plane drift (must use Prism 140.0m horizon)`);
    }
  }

  const logRuleMatch = /\.ve-m-log\s*\{([\s\S]*?)\}/.exec(studioDataSource);
  if (!logRuleMatch) {
    throw new Error("site/studio-data.mjs STUDIO_CSS missing .ve-m-log rule");
  }
  const logBody = logRuleMatch[1];
  if (/(?:^|[;\s])border\s*:\s*var\(--border-width\)\s+solid/.test(logBody)) {
    throw new Error(".ve-m-log violates single-elevation depth with a nested 4-sided box border inside .ve-m-card");
  }
  if (!/border\s*:\s*0\s*;/.test(logBody) || !/border-top\s*:/.test(logBody)) {
    throw new Error(".ve-m-log must use border: 0 and a top hairline divider (border-top)");
  }

  for (const token of ["PRISM_AUDIO_QUANTUM_FRAMES", 'setAttribute("role", "tabpanel")', "ve-panel-"]) {
    if (!studioSource.includes(token)) {
      throw new Error(`site/studio.mjs missing Prism/ARIA studio token "${token}"`);
    }
  }
  if (!starterGameSource.includes("scene:lod:${PRISM_MAX_LOD_RANK}")) {
    throw new Error("templates/browser-3d-starter/game.mjs must hydrate LOD3_SPEC from app.holo scene:lod:${PRISM_MAX_LOD_RANK}");
  }
}

/**
 * Gate C10: Enforces platform differentiation across `/` (Verdant Engine Runtime), `/studio/`
 * (Verdant Studio Atelier), `/catalogs/` (Universal Model Registry), `/demo/` (Benchmarks),
 * and `/docs/`, verifies every catalog entry against real `app/src/*.lex.tex` modules and
 * `REQUIRED_HOLO_QUERIES`, and protects cold-boot weight on `site/card.mjs`.
 */
export function verifyPlatformDifferentiationAndCatalogsContract({
  rootDir = ROOT_DIR,
  cardHtml,
  cardSource,
  studioPageHtml,
  studioPageSource,
  catalogsHtml,
  catalogsPageSource,
  organs = VERDANT_PLATFORM_ORGANS,
  catalogEntries = PRISM_MODEL_CATALOGS,
}) {
  // 1. Reject Engine/Studio conflation on root `site/card.html`:
  if (/Verdant Studio 3D authoring viewport|Studio Transport|data-label="Studio Roles"/i.test(cardHtml)) {
    throw new Error("Conflation detected: site/card.html still labels the Verdant Engine root surface as Verdant Studio");
  }
  for (const pillarHref of ['./studio/', './catalogs/', './demo/', './docs/']) {
    if (!cardHtml.includes(`href="${pillarHref}"`)) {
      throw new Error(`site/card.html missing differentiated platform pillar link href="${pillarHref}"`);
    }
  }
  // Protect `/` cold-boot byte budget (`FR-03`): zero static top-level ES imports in `site/card.mjs`.
  if (/^\s*import\s+[\s\S]*?from\s+["']/m.test(cardSource)) {
    throw new Error("site/card.mjs must use dynamic import() for Studio/Stitch modules to preserve FR-03 cold-boot weight");
  }

  // 2. Verify standalone Verdant Studio (`/studio/`):
  if (!studioPageHtml || !studioPageSource) {
    throw new Error("Standalone Verdant Studio (`site/studio/index.html` and `site/studio/studio-page.mjs`) must exist");
  }
  if (/#[0-9a-fA-F]{3,8}\b/.test(studioPageHtml)) {
    throw new Error("site/studio/index.html contains raw #hex color literal (must use W3C DTCG CSS tokens)");
  }
  if (/backdrop-filter\s*:\s*blur/i.test(studioPageHtml)) {
    throw new Error("site/studio/index.html must not apply backdrop-filter: blur over WebGL");
  }
  if (/\b(stakeholders?|personas?)\b/i.test(studioPageHtml) || /\b(stakeholders?|personas?)\b/i.test(studioPageSource)) {
    throw new Error("Standalone Studio UI (`site/studio/*`) contains forbidden stakeholder/persona terminology");
  }
  if (studioPageSource.includes("vendor/prism-code/")) {
    throw new Error("site/studio/studio-page.mjs must import ../worker-runtime/ instead of unshimmed vendor/prism-code/ (bare node:crypto)");
  }
  for (const req of ["worker-runtime/index.mjs", "app.holo", "engine-worker.mjs", "mountInlineStudio"]) {
    if (!studioPageSource.includes(req)) {
      throw new Error(`site/studio/studio-page.mjs missing required live Prism Studio integration "${req}"`);
    }
  }

  // 3. Verify standalone Verdant Model Catalogs (`/catalogs/`):
  if (!catalogsHtml || !catalogsPageSource) {
    throw new Error("Standalone Verdant Catalogs (`site/catalogs/index.html` and `site/catalogs/catalogs.mjs`) must exist");
  }
  if (/#[0-9a-fA-F]{3,8}\b/.test(catalogsHtml)) {
    throw new Error("site/catalogs/index.html contains raw #hex color literal (must use W3C DTCG CSS tokens)");
  }
  if (/backdrop-filter\s*:\s*blur/i.test(catalogsHtml)) {
    throw new Error("site/catalogs/index.html must not use backdrop-filter: blur");
  }
  if (catalogsPageSource.includes("vendor/prism-code/")) {
    throw new Error("site/catalogs/catalogs.mjs must import ../worker-runtime/ instead of unshimmed vendor/prism-code/ (bare node:crypto)");
  }
  for (const req of ["worker-runtime/index.mjs", "app.holo", "kappa", "filterCatalogModels", "VERDANT_PLATFORM_ORGANS"]) {
    if (!catalogsPageSource.includes(req)) {
      throw new Error(`site/catalogs/catalogs.mjs missing live Holo catalog verification token "${req}"`);
    }
  }

  // 4. Verify platform organ taxonomy & 28+ authoritative catalog entries:
  const expectedOrgans = ["engine", "studio", "catalogs", "demo", "docs"];
  if (JSON.stringify(organs.map((o) => o.slug)) !== JSON.stringify(expectedOrgans)) {
    throw new Error(`VERDANT_PLATFORM_ORGANS must define exact organs: ${expectedOrgans.join(",")}`);
  }
  if (LIFECYCLE_STAGES.length < 9 || CATALOG_DOMAINS.length < 9) {
    throw new Error("prism-catalogs.mjs must cover all 8 product lifecycle stages and all 8 catalog domains");
  }
  if (!Array.isArray(catalogEntries) || catalogEntries.length < 24) {
    throw new Error("PRISM_MODEL_CATALOGS must register at least 24 imported standard & Prism models");
  }

  const validQueries = new Set(REQUIRED_HOLO_QUERIES);
  const validRefusals = new Set(REQUIRED_HOLO_REFUSALS.map(([k]) => k));
  const validRoles = new Set(EXPECTED_STUDIO_ROLE_KEYS);
  const seenIds = new Set();

  for (const entry of catalogEntries) {
    if (!entry.id || !entry.id.startsWith("STD-") || seenIds.has(entry.id)) {
      throw new Error(`Invalid or duplicate catalog model ID: "${entry.id}"`);
    }
    seenIds.add(entry.id);
    if (!entry.authority || !entry.title || !entry.invariant) {
      throw new Error(`Catalog entry ${entry.id} missing authority, title, or invariant`);
    }
    if (!entry.lexModule || !existsSync(join(rootDir, entry.lexModule))) {
      throw new Error(`Catalog entry ${entry.id} references non-existent LexLean module "${entry.lexModule}"`);
    }
    if (!Array.isArray(entry.holoQueries) || entry.holoQueries.length === 0) {
      throw new Error(`Catalog entry ${entry.id} must bind at least one verified Holo query`);
    }
    for (const q of entry.holoQueries) {
      if (!validQueries.has(q)) {
        throw new Error(`Catalog entry ${entry.id} binds unknown Holo query "${q}"`);
      }
    }
    for (const rq of entry.refusalQueries || []) {
      if (!validRefusals.has(rq)) {
        throw new Error(`Catalog entry ${entry.id} binds unknown R5 refusal query "${rq}"`);
      }
    }
    if (!Array.isArray(entry.studioRoles) || entry.studioRoles.length === 0 || entry.studioRoles.some((r) => !validRoles.has(r))) {
      throw new Error(`Catalog entry ${entry.id} must bind valid Studio lifecycle roles`);
    }
    if (!Array.isArray(entry.taxon) || entry.taxon.length < 3) {
      throw new Error(`Catalog entry ${entry.id} must declare at least 3 domain-specific taxon terms`);
    }
  }
}

export function verifyExternalOraclesAndRuntimeExclusivityContract({
  rootDir = ROOT_DIR,
  authoritiesToml = readFileSync(join(rootDir, "model", "authorities.toml"), "utf8"),
  extraSources = [],
} = {}) {
  // 1. Parse SHA-256 file locks from `model/authorities.toml` and verify every `catalogs/**` corpus:
  const locks = [];
  let cur = null;
  for (const rawLine of authoritiesToml.split(/\r?\n/)) {
    const line = rawLine.trim();
    if (line === "[[authority]]") {
      if (cur && cur.id && cur.path && cur.sha256) locks.push(cur);
      cur = {};
      continue;
    }
    if (!cur || !line || line.startsWith("#")) continue;
    const eq = line.indexOf("=");
    if (eq === -1) continue;
    const key = line.slice(0, eq).trim();
    const val = line.slice(eq + 1).trim().replace(/^"|"$/g, "");
    if (key === "id") cur.id = val;
    if (key === "artifact" || key === "path") cur.path = val;
    if ((key === "checksum" || key === "sha256") && val !== "none") cur.sha256 = val;
  }
  if (cur && cur.id && cur.path && cur.sha256) locks.push(cur);

  if (locks.length < 5) {
    throw new Error(`Expected >= 5 SHA-256-locked external authority catalogs in model/authorities.toml, found ${locks.length}`);
  }
  for (const lock of locks) {
    const abs = join(rootDir, lock.path);
    if (!existsSync(abs)) {
      throw new Error(`Missing locked authority catalog file "${lock.path}" (${lock.id})`);
    }
    const bytes = readFileSync(abs);
    const sha = createHash("sha256").update(bytes).digest("hex");
    if (sha !== lock.sha256) {
      throw new Error(`Authority catalog SHA-256 mismatch for ${lock.id} (${lock.path}): expected ${lock.sha256}, observed ${sha}`);
    }
  }

  // 2. Enforce strict Verdant Engine runtime exclusivity across manifests and live surfaces:
  const dmnCatalog = JSON.parse(readFileSync(join(rootDir, "catalogs", "language", "iso24617-dmn-dialogue.json"), "utf8"));
  const prohibited = dmnCatalog.prohibitedExternalRuntimes || [];
  if (prohibited.length < 10) {
    throw new Error("AUTH-ISO-24617-DMN must enumerate prohibited external model runtimes");
  }

  const scanTargets = [
    ["package.json", readFileSync(join(rootDir, "package.json"), "utf8")],
    ["Cargo.toml", readFileSync(join(rootDir, "Cargo.toml"), "utf8")],
    ["crates/verdant-core/Cargo.toml", readFileSync(join(rootDir, "crates", "verdant-core", "Cargo.toml"), "utf8")],
    ["crates/verdant-wasm/Cargo.toml", readFileSync(join(rootDir, "crates", "verdant-wasm", "Cargo.toml"), "utf8")],
    ["site/card.mjs", readFileSync(join(rootDir, "site", "card.mjs"), "utf8")],
    ["site/studio.mjs", readFileSync(join(rootDir, "site", "studio.mjs"), "utf8")],
    ["site/engine-worker.mjs", readFileSync(join(rootDir, "site", "engine-worker.mjs"), "utf8")],
    ["site/demo/demo.mjs", readFileSync(join(rootDir, "site", "demo", "demo.mjs"), "utf8")],
    ["templates/browser-3d-starter/Cargo.toml", readFileSync(join(rootDir, "templates", "browser-3d-starter", "Cargo.toml"), "utf8")],
    ["templates/browser-3d-starter/game.mjs", readFileSync(join(rootDir, "templates", "browser-3d-starter", "game.mjs"), "utf8")],
    ...extraSources,
  ];

  for (const [label, text] of scanTargets) {
    for (const pkg of prohibited) {
      if (
        text.includes(`"${pkg}"`) ||
        text.includes(`'${pkg}'`) ||
        text.includes(`"${pkg}/`) ||
        text.includes(`'${pkg}/`) ||
        text.includes(`${pkg} =`)
      ) {
        throw new Error(`Prohibited external model runtime "${pkg}" detected in ${label} (Verdant Engine is the sole permitted model runtime)`);
      }
    }
  }
}

import {
  verifyContextPurityAndIntentionalInterfaceContract,
  verifySubpathAndUorcOnlyDeploymentContract,
} from "./enforce-prism-viewpoints.mjs";
export {
  verifyContextPurityAndIntentionalInterfaceContract,
  verifySubpathAndUorcOnlyDeploymentContract,
};

export async function runAllPrismContractChecks(rootDir = ROOT_DIR) {
  const holoBuffer = readFileSync(join(rootDir, "site", "app.holo"));
  const coreWasmBytes = readFileSync(join(rootDir, "site", "verdant-wasm.wasm"));
  const uorcWasmBytes = readFileSync(join(rootDir, "site", "verdant-uorc.wasm"));
  const studioDataSource = readFileSync(join(rootDir, "site", "studio-data.mjs"), "utf8");
  const studioScaffolderSource = readFileSync(join(rootDir, "site", "studio-scaffolder.mjs"), "utf8");
  const studioMediaSource = readFileSync(join(rootDir, "site", "studio-media.mjs"), "utf8");
  const studioDirectorSource = readFileSync(join(rootDir, "site", "studio-director-panel.mjs"), "utf8");
  const studioGameplaySource = readFileSync(join(rootDir, "site", "studio-gameplay-panel.mjs"), "utf8");
  const studioSource = readFileSync(join(rootDir, "site", "studio.mjs"), "utf8");
  const combinedStudioSource = [
    studioDataSource,
    studioScaffolderSource,
    studioMediaSource,
    studioDirectorSource,
    studioGameplaySource,
    studioSource,
  ].join("\n");
  const workerCoreSource = readFileSync(join(rootDir, "site", "engine-worker.mjs"), "utf8");
  const workerCodecSource = readFileSync(join(rootDir, "site", "engine-worker-codec.mjs"), "utf8");
  const workerExtSource = readFileSync(join(rootDir, "site", "engine-worker-extended.mjs"), "utf8");
  const workerSource = [workerCoreSource, workerCodecSource, workerExtSource].join("\n");
  const cardSource = readFileSync(join(rootDir, "site", "card.mjs"), "utf8");
  const cardHtml = readFileSync(join(rootDir, "site", "card.html"), "utf8");
  const cardCss = readFileSync(join(rootDir, "site", "card.css"), "utf8");
  const rendererSource = readFileSync(join(rootDir, "site", "scene-renderer.mjs"), "utf8");
  const stitchSource = readFileSync(join(rootDir, "site", "stitch-mesh.mjs"), "utf8");
  const audioWorkletSource = readFileSync(join(rootDir, "site", "audio-worklet.mjs"), "utf8");
  const liveEmbedSource = readFileSync(join(rootDir, "site", "docs", "live-embed.mjs"), "utf8");
  const docsIndexHtml = readFileSync(join(rootDir, "site", "docs", "index.html"), "utf8");
  const demoHtml = readFileSync(join(rootDir, "site", "demo", "index.html"), "utf8");
  const demoCoreSource = readFileSync(join(rootDir, "site", "demo", "demo.mjs"), "utf8");
  const demoUhdSource = readFileSync(join(rootDir, "site", "demo", "demo-uhd.mjs"), "utf8");
  const demoSource = [demoCoreSource, demoUhdSource].join("\n");
  const studioPageHtml = readFileSync(join(rootDir, "site", "studio", "index.html"), "utf8");
  const studioPageCoreSource = readFileSync(join(rootDir, "site", "studio", "studio-page.mjs"), "utf8");
  const studioBootSource = readFileSync(join(rootDir, "site", "studio", "studio-boot.mjs"), "utf8");
  const studioAuthoritySource = readFileSync(join(rootDir, "site", "studio", "authority-panel.mjs"), "utf8");
  const studioStageAssetsSource = readFileSync(join(rootDir, "site", "studio", "stage-assets.mjs"), "utf8");
  const studioPageSource = [
    studioPageCoreSource,
    studioBootSource,
    studioAuthoritySource,
    studioStageAssetsSource,
  ].join("\n");
  const catalogsHtml = readFileSync(join(rootDir, "site", "catalogs", "index.html"), "utf8");
  const catalogsPageSource = readFileSync(join(rootDir, "site", "catalogs", "catalogs.mjs"), "utf8");
  const catalogStreamsCoreSource = readFileSync(join(rootDir, "site", "catalog-streams.mjs"), "utf8");
  const catalogStreamsStitchSource = readFileSync(join(rootDir, "site", "catalog-streams-stitch.mjs"), "utf8");
  const catalogStreamsSource = [catalogStreamsCoreSource, catalogStreamsStitchSource].join("\n");
  const starterHtml = readFileSync(join(rootDir, "templates", "browser-3d-starter", "index.html"), "utf8");
  const starterGameSource = readFileSync(join(rootDir, "templates", "browser-3d-starter", "game.mjs"), "utf8");

  const c1 = await verifyPrismHoloContract(holoBuffer, { rootDir });
  const c2 = await verifyWasmBinaryContract(coreWasmBytes, uorcWasmBytes);
  verifyZeroSlmBypassContract(combinedStudioSource);
  verifyFk07CapabilityContract({ studioSource: combinedStudioSource, workerSource, cardSource, stitchSource });
  verifyUnifiedHostAndAudioContract({ workerSource, rendererSource, audioWorkletSource });
  verifyStarterAndScaffolderContract({ starterHtml, starterGameSource });
  verifyStudioSurfaceAndZeroHexContract({ cardHtml, cardCss, studioSource: combinedStudioSource, liveEmbedSource });
  verifyRoleDrivenStudioAndDemoSeparationContract({
    cardHtml,
    cardSource,
    studioSource: combinedStudioSource,
    stitchSource,
    demoHtml,
    demoSource,
    starterHtml,
  });
  verifyPrismKernelAlignmentAndCleanRefactorContract({
    demoSource,
    demoHtml,
    studioDataSource,
    studioScaffolderSource,
    studioSource: combinedStudioSource,
    cardSource,
    workerSource,
    starterGameSource,
    liveEmbedSource,
    docsIndexHtml,
  });
  verifyPlatformDifferentiationAndCatalogsContract({
    rootDir,
    cardHtml,
    cardSource,
    studioPageHtml,
    studioPageSource,
    catalogsHtml,
    catalogsPageSource,
  });
  verifyExternalOraclesAndRuntimeExclusivityContract({ rootDir });
  verifySubpathAndUorcOnlyDeploymentContract({
    surfaceSources: [
      ["site/card.html", cardHtml],
      ["site/card.mjs", cardSource],
      ["site/studio/index.html", studioPageHtml],
      ["site/studio/studio-page.mjs", studioPageSource],
      ["site/catalogs/index.html", catalogsHtml],
      ["site/catalogs/catalogs.mjs", catalogsPageSource],
      ["site/catalog-streams.mjs", catalogStreamsSource],
      ["site/demo/index.html", demoHtml],
      ["site/demo/demo.mjs", demoSource],
      ["site/studio.mjs", studioSource],
      ["site/studio-scaffolder.mjs", studioScaffolderSource],
      ["site/engine-worker.mjs", workerSource],
      ["site/docs/live-embed.mjs", liveEmbedSource],
      ["templates/browser-3d-starter/index.html", starterHtml],
      ["templates/browser-3d-starter/game.mjs", starterGameSource],
    ],
    studioSources: [
      ["site/studio/index.html", studioPageHtml],
      ["site/studio/studio-page.mjs", studioPageSource],
      ["site/studio.mjs", studioSource],
      ["site/studio-scaffolder.mjs", studioScaffolderSource],
      ["site/studio-data.mjs", studioDataSource],
    ],
  });
  verifyContextPurityAndIntentionalInterfaceContract({
    rootDir,
    cardHtml,
    studioPageHtml,
    catalogsHtml,
    demoHtml,
    demoSource,
    starterHtml,
  });

  // Audit all shipped Rust crate sources, starter template, and examples (`FI-08` zero magic constants):
  for (const relDir of [
    "crates/verdant-core/src",
    "crates/verdant-core/examples",
    "crates/verdant-tensor/src",
    "crates/verdant-wasm/src",
    "crates/verdant-cli/src",
    "templates/browser-3d-starter/src",
  ]) {
    const absDir = join(rootDir, relDir);
    for (const entry of readdirSync(absDir, { withFileTypes: true })) {
      if (!entry.isFile() || !entry.name.endsWith(".rs") || entry.name === "prism_kernel.rs") continue;
      const relPath = `${relDir}/${entry.name}`;
      const src = readFileSync(join(absDir, entry.name), "utf8");
      verifyRustZeroMagicConstants(src, relPath);
      if (relDir.endsWith("examples") || relDir.startsWith("templates/")) {
        if (!src.includes("ObserverPrismHolon") && !src.includes("CatalogBundleRegistry")) {
          throw new Error(`${relPath} must instantiate imported catalogs via ObserverPrismHolon or CatalogBundleRegistry`);
        }
      }
    }
  }

  return {
    vectors: c1.vectors,
    fingerprint: c1.fingerprint,
    defaultSeed: c2.defaultSeed,
    holoQueriesVerified: Object.keys(c1.verdicts).length,
    holoRefusalsVerified: Object.keys(c1.refusals).length,
    catalogModelsVerified: PRISM_MODEL_CATALOGS.length,
    cachedHolo: c1.cached,
  };
}

if (process.argv[1] && fileURLToPath(import.meta.url) === process.argv[1]) {
  const res = await runAllPrismContractChecks();
  console.log(
    `prism-contracts: PASS (${res.vectors} Holo vectors, ${res.holoQueriesVerified}+${res.holoRefusalsVerified} queries, ${res.catalogModelsVerified} catalog models${res.cachedHolo ? " [kappa-cached]" : ""}, seed=${res.defaultSeed}, blake3=${res.fingerprint.slice(0, 12)}…)`,
  );
}
