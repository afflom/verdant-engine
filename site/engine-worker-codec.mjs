// Verdant Worker Wasm I/O Buffer & Zero-Copy Scene Mesh View Readers
// (`site/engine-worker-codec.mjs`), extracted from `site/engine-worker.mjs`.

export function stageIoBytes(ex, bytes) {
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

export function readIoBytes(ex) {
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

export function readAuxBytes(ex) {
  const len = ex.verdant_wasm_aux_len() >>> 0;
  const ptr = ex.verdant_wasm_aux_ptr() >>> 0;
  if (len === 0 || ptr === 0 || !ex.memory) return new Uint8Array(0);
  return new Uint8Array(ex.memory.buffer, ptr, len).slice();
}

export function parsePipeKv(raw) {
  const kv = {};
  for (const part of raw.split("|")) {
    const eq = part.indexOf("=");
    if (eq > 0) kv[part.slice(0, eq)] = part.slice(eq + 1);
  }
  return kv;
}

export function readSolvedCamera(host) {
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

export function readUniverseCell(host) {
  return [
    Number(host.verdant_wasm_universe_cell_x()),
    Number(host.verdant_wasm_universe_cell_y()),
    Number(host.verdant_wasm_universe_cell_z()),
  ];
}

export function readStoryStatus(uex) {
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

export function readZeroCopyMeshViews(host) {
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

export function readSplitMeshViews(host) {
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

export async function handleExtendedWorkerMessage(msg, ctx) {
  const { ensureUorcSceneHost, simHost, getCanvas, getRenderer, getEmaFrameMicros, postMessage } = ctx;

  if (msg.kind === "update-host-envelope") {
    const uorcWasm = await ensureUorcSceneHost();
    const canvas = getCanvas();
    const renderer = getRenderer();
    const emaFrameMicros = getEmaFrameMicros();
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
    postMessage({ kind: "cognitive-resolution", id: msg.id, ok: Boolean(cognition), cognition });
    return true;
  }

  if (msg.kind === "uhd-profile") {
    const uorcWasm = await ensureUorcSceneHost();
    const renderer = getRenderer();
    const emaFrameMicros = getEmaFrameMicros();
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
    const cam = readSolvedCamera(simHost());
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
    postMessage({
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
    return true;
  }

  if (msg.kind === "tensor-paged-benchmark") {
    const uorcWasm = await ensureUorcSceneHost();
    const pages = Math.max(8, Math.min(256, (msg.logicalPages || 64) | 0)) >>> 0;
    const budgetKib = Math.max(256, (msg.budgetKib || 512) | 0) >>> 0;
    const bLen = typeof uorcWasm.verdant_wasm_tensor_paged_benchmark_aux === "function"
      ? (uorcWasm.verdant_wasm_tensor_paged_benchmark_aux(pages, budgetKib) >>> 0)
      : 0;
    const benchmark = bLen ? JSON.parse(new TextDecoder().decode(readAuxBytes(uorcWasm))) : null;
    postMessage({ kind: "tensor-paged-benchmark-ready", id: msg.id, ok: Boolean(benchmark), benchmark });
    return true;
  }

  if (msg.kind === "cognitive-resolution") {
    const uorcWasm = await ensureUorcSceneHost();
    const len = uorcWasm.verdant_wasm_cognitive_resolution_aux() >>> 0;
    const cognition = len ? JSON.parse(new TextDecoder().decode(readAuxBytes(uorcWasm))) : null;
    postMessage({ kind: "cognitive-resolution", id: msg.id, ok: Boolean(cognition), cognition });
    return true;
  }

  if (msg.kind === "vcs-commit-pack-holo" || msg.kind === "paged-workload-step") {
    const uorcWasm = await ensureUorcSceneHost();
    stageIoBytes(uorcWasm, new TextEncoder().encode(JSON.stringify(msg.request || {})));
    const fn =
      msg.kind === "vcs-commit-pack-holo"
        ? uorcWasm.verdant_wasm_vcs_commit_and_pack_holo_io
        : uorcWasm.verdant_wasm_paged_workload_step_io;
    const len = typeof fn === "function" ? fn() >>> 0 : 0;
    const result = len ? JSON.parse(new TextDecoder().decode(readAuxBytes(uorcWasm))) : { ok: false, error: "export unavailable" };
    postMessage({ kind: `${msg.kind}-done`, id: msg.id, ok: Boolean(result?.ok), result });
    return true;
  }

  if (msg.kind === "register-stream" || msg.kind === "remove-stream" || msg.kind === "toggle-stream" || msg.kind === "ingest-knowledge") {
    const uorcWasm = await ensureUorcSceneHost();
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
    postMessage({ kind: `${msg.kind}-done`, id: msg.id, ok: len > 0, result, cognition });
    return true;
  }

  return false;
}
