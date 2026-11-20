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
