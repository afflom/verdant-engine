// Post-boot extended Verdant Worker handlers (`site/engine-worker-extended.mjs`),
// lazy-loaded on demand by `site/engine-worker-codec.mjs` so cold-boot `/` stays well below FR-03.

import {
  parsePipeKv,
  readAuxBytes,
  readIoBytes,
  readSolvedCamera,
  readUniverseCell,
  stageIoBytes,
} from "./engine-worker-codec.mjs";

export async function handleExtendedWorkerMessage(msg, ctx) {
  const {
    ensureUorcSceneHost,
    getWasm,
    getUorcWasm,
    getLoadedUorcWasm,
    simHost,
    getCanvas,
    getRenderer,
    getEmaFrameMicros,
    getOrganModes,
    getCurrentLod,
    setCurrentLod,
    getCustomGlbActive,
    getUorcSceneActive,
    setUorcSceneActive,
    drawScene,
    setCustomGlbActive,
    getLastAnchor,
    getLastAnchorActive,
    setLastAnchor,
    getLastPose,
    setLastPose,
    setActiveSeed,
    clearWalkJournal,
    syncOrgansToWasm,
    postMessage,
  } = ctx;

  if (msg.kind === "set-seed") {
    const wasm = getWasm();
    const uorcWasm = getLoadedUorcWasm();
    const nextSeed = BigInt(Math.max(1, Number(msg.seed) || 7));
    setActiveSeed(nextSeed);
    clearWalkJournal();
    const rank = simHost().verdant_wasm_active_lod_rank() >>> 0;
    wasm.verdant_wasm_init(nextSeed);
    wasm.verdant_wasm_set_lod_rank(rank);
    if (uorcWasm) {
      uorcWasm.verdant_wasm_init(nextSeed);
      uorcWasm.verdant_wasm_set_lod_rank(rank);
      syncOrgansToWasm(uorcWasm);
    }
    setCustomGlbActive(false);
    const sh = simHost();
    const lastAnchor = getLastAnchor();
    if (getLastAnchorActive()) {
      sh.verdant_wasm_camera_set_anchor(1, lastAnchor[0], lastAnchor[1], lastAnchor[2]);
    } else {
      sh.verdant_wasm_camera_set_mode(wasm.verdant_wasm_camera_mode() >>> 0);
    }
    const lastPose = getLastPose();
    sh.verdant_wasm_camera_set_pose_intent(
      lastPose.yaw,
      lastPose.pitch,
      lastPose.distMm,
      lastPose.aspectMilli,
    );
    sh.verdant_wasm_scene_rebuild_buffers(0);
    const cam = readSolvedCamera(sh);
    const stats = drawScene(cam.eye, cam.lookTarget, getCurrentLod(), 0, false);
    postMessage({
      kind: "seed-updated",
      id: msg.id,
      ok: true,
      seed: Number(nextSeed),
      tris: stats.tris,
      camera: cam,
    });
    return true;
  }

  if (msg.kind === "load-domain-pack") {
    const domainIdx = Math.max(0, Math.min(3, msg.domainIdx | 0)) >>> 0;
    const uorcWasm = await ensureUorcSceneHost();
    const len = uorcWasm.verdant_wasm_load_domain_pack(domainIdx) >>> 0;
    if (len === 0) {
      postMessage({ kind: "domain-pack-loaded", id: msg.id, ok: false });
      return true;
    }
    const observer = JSON.parse(new TextDecoder().decode(readAuxBytes(uorcWasm)));
    setCustomGlbActive(false);
    setUorcSceneActive(true);
    clearWalkJournal();
    const seed = BigInt(uorcWasm.verdant_wasm_active_seed());
    setActiveSeed(seed);
    if (msg.lod) setCurrentLod(msg.lod);
    const sh = simHost();
    const cam = readSolvedCamera(sh);
    setLastAnchor([cam.lookTarget[0], cam.lookTarget[1], cam.lookTarget[2]]);
    const dx = cam.eye[0] - cam.lookTarget[0];
    const dy = cam.eye[1] - cam.lookTarget[1];
    const dz = cam.eye[2] - cam.lookTarget[2];
    const distM = Math.max(1.0, Math.hypot(dx, dy, dz));
    const prevPose = getLastPose();
    setLastPose({
      yaw: Math.round(Math.atan2(dx, dz) * 1000),
      pitch: Math.round(Math.asin(Math.max(-0.95, Math.min(0.95, dy / distM))) * 1000),
      distMm: Math.round(distM * 1000) >>> 0,
      aspectMilli: prevPose?.aspectMilli || 1600,
    });
    const stats = drawScene(cam.eye, cam.lookTarget, getCurrentLod(), msg.simClockSec || 0, false, 0, 0, true);
    postMessage({
      kind: "domain-pack-loaded",
      id: msg.id,
      ok: true,
      domainIdx,
      seed: Number(seed),
      tris: stats.tris,
      walkablePolys: sh.verdant_wasm_navmesh_polygon_count() >>> 0,
      camera: cam,
      observer,
    });
    return true;
  }

  if (
    msg.kind === "studio-load-blank" ||
    msg.kind === "studio-add-object" ||
    msg.kind === "studio-update-object" ||
    msg.kind === "studio-remove-object" ||
    msg.kind === "studio-clear-objects"
  ) {
    const uorcWasm = await ensureUorcSceneHost();
    let resId = 0;
    if (msg.kind === "studio-load-blank") {
      resId = uorcWasm.verdant_wasm_studio_load_blank_stage() >>> 0;
    } else if (msg.kind === "studio-add-object") {
      const p = msg.pos || [0, 0, 0];
      const s = msg.scale || [1.5, 1.5, 1.5];
      const c = msg.color || [0.82, 0.35, 0.18];
      resId = uorcWasm.verdant_wasm_studio_add_object(msg.primKind | 0, p[0], p[1], p[2], s[0], s[1], s[2], c[0], c[1], c[2]) >>> 0;
    } else if (msg.kind === "studio-update-object") {
      const p = msg.pos || [0, 0, 0];
      const s = msg.scale || [1.5, 1.5, 1.5];
      const c = msg.color || [0.82, 0.35, 0.18];
      resId = uorcWasm.verdant_wasm_studio_update_object(msg.objId >>> 0, p[0], p[1], p[2], msg.yawMilli | 0, s[0], s[1], s[2], c[0], c[1], c[2]) >>> 0;
    } else if (msg.kind === "studio-remove-object") {
      resId = uorcWasm.verdant_wasm_studio_remove_object(msg.objId >>> 0) >>> 0;
    } else {
      resId = uorcWasm.verdant_wasm_studio_clear_objects() >>> 0;
    }
    setCustomGlbActive(false);
    setUorcSceneActive(true);
    if (msg.lod) setCurrentLod(msg.lod);
    const sh = simHost();
    const cam = readSolvedCamera(sh);
    if (msg.kind === "studio-load-blank") {
      setLastAnchor([cam.lookTarget[0], cam.lookTarget[1], cam.lookTarget[2]]);
      const dx = cam.eye[0] - cam.lookTarget[0], dy = cam.eye[1] - cam.lookTarget[1], dz = cam.eye[2] - cam.lookTarget[2];
      const distM = Math.max(1.0, Math.hypot(dx, dy, dz));
      const prevPose = getLastPose();
      setLastPose({
        yaw: Math.round(Math.atan2(dx, dz) * 1000),
        pitch: Math.round(Math.asin(Math.max(-0.95, Math.min(0.95, dy / distM))) * 1000),
        distMm: Math.round(distM * 1000) >>> 0,
        aspectMilli: prevPose?.aspectMilli || 1600,
      });
    }
    const stats = drawScene(cam.eye, cam.lookTarget, getCurrentLod(), msg.simClockSec || 0, false, 0, 0, true);
    const oLen = uorcWasm.verdant_wasm_studio_objects_json_aux() >>> 0;
    const objects = oLen > 0 ? JSON.parse(new TextDecoder().decode(readAuxBytes(uorcWasm))) : [];
    postMessage({ kind: "studio-scene-updated", id: msg.id, ok: resId > 0 || msg.kind === "studio-clear-objects", objId: resId, tris: stats.tris, camera: cam, objects });
    return true;
  }

  if (msg.kind === "set-npc-directive") {
    const npcIdx = Math.max(0, msg.npcIdx | 0) >>> 0;
    const directiveIdx = Math.max(0, msg.directiveIdx | 0) >>> 0;
    const uorcWasm = await ensureUorcSceneHost();
    const len = uorcWasm.verdant_wasm_set_npc_directive(npcIdx, directiveIdx) >>> 0;
    if (len === 0) {
      postMessage({ kind: "npc-directive-updated", id: msg.id, ok: false });
      return true;
    }
    const [characterId, directiveLabel, wpStr] = new TextDecoder().decode(readAuxBytes(uorcWasm)).split("|");
    const sh = simHost();
    sh.verdant_wasm_scene_rebuild_buffers(msg.simClockSec || 0);
    const cam = readSolvedCamera(sh);
    const stats = drawScene(cam.eye, cam.lookTarget, getCurrentLod(), msg.simClockSec || 0, false);
    postMessage({
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
    return true;
  }

  if (msg.kind === "npc-dialogue") {
    const organModes = getOrganModes();
    if (organModes[0] === 2) {
      postMessage({ kind: "npc-dialogue-reply", id: msg.id, ok: false, organDenied: "wasm-core" });
      return true;
    }
    if (organModes[7] === 2) {
      postMessage({ kind: "npc-dialogue-reply", id: msg.id, ok: false, organDenied: "webcrypto-prism-id" });
      return true;
    }
    const npcIdx = Math.max(0, msg.npcIdx | 0) >>> 0;
    const utterance = String(msg.utterance || "").trim();
    if (!utterance) {
      postMessage({ kind: "npc-dialogue-reply", id: msg.id, ok: false });
      return true;
    }
    const uorcWasm = await ensureUorcSceneHost();
    stageIoBytes(uorcWasm, new TextEncoder().encode(utterance));
    const len = uorcWasm.verdant_wasm_dialogue_turn_io(npcIdx) >>> 0;
    if (len === 0) {
      postMessage({ kind: "npc-dialogue-reply", id: msg.id, ok: false });
      return true;
    }
    const turn = JSON.parse(new TextDecoder().decode(readAuxBytes(uorcWasm)));
    const sh = simHost();
    if (turn.mutatedDirectiveIndex != null && turn.mutatedDirectiveIndex >= 0) {
      sh.verdant_wasm_scene_rebuild_buffers(msg.simClockSec || 0);
      const cam = readSolvedCamera(sh);
      drawScene(cam.eye, cam.lookTarget, getCurrentLod(), msg.simClockSec || 0, false);
    }
    postMessage({ kind: "npc-dialogue-reply", id: msg.id, ok: true, turn });
    return true;
  }

  if (msg.kind === "observer-status") {
    const uorcWasm = await ensureUorcSceneHost();
    const len = uorcWasm.verdant_wasm_observer_status_aux() >>> 0;
    if (len === 0) {
      postMessage({ kind: "observer-status", id: msg.id, ok: false });
      return true;
    }
    const observer = JSON.parse(new TextDecoder().decode(readAuxBytes(uorcWasm)));
    postMessage({ kind: "observer-status", id: msg.id, ok: true, observer });
    return true;
  }

  if (msg.kind === "observe-comm") {
    const channelCode = Math.max(0, Math.min(4, msg.channelCode | 0)) >>> 0;
    const wire = `${msg.sender || "urn:verdant:organ:engine"}|${msg.recipient || "urn:verdant:organ:studio"}|${msg.message || ""}`;
    const uorcWasm = await ensureUorcSceneHost();
    stageIoBytes(uorcWasm, new TextEncoder().encode(wire));
    const len = uorcWasm.verdant_wasm_observe_comm_io(channelCode) >>> 0;
    const kappaHex = len > 0 ? new TextDecoder().decode(readAuxBytes(uorcWasm)) : "";
    postMessage({ kind: "comm-observed", id: msg.id, ok: len > 0, kappaHex });
    return true;
  }

  if (msg.kind === "import-project-model") {
    const uorcWasm = await ensureUorcSceneHost();
    const jsonBytes = new TextEncoder().encode(JSON.stringify(msg.request || {}));
    stageIoBytes(uorcWasm, jsonBytes);
    const len = uorcWasm.verdant_wasm_import_project_model_io() >>> 0;
    if (len === 0) {
      postMessage({ kind: "project-model-imported", id: msg.id, ok: false });
      return true;
    }
    const model = JSON.parse(new TextDecoder().decode(readAuxBytes(uorcWasm)));
    postMessage({ kind: "project-model-imported", id: msg.id, ok: true, model });
    return true;
  }

  if (msg.kind === "edit-project-model") {
    const uorcWasm = await ensureUorcSceneHost();
    const jsonBytes = new TextEncoder().encode(JSON.stringify(msg.request || {}));
    stageIoBytes(uorcWasm, jsonBytes);
    const len = uorcWasm.verdant_wasm_edit_project_model_io() >>> 0;
    if (len === 0) {
      postMessage({ kind: "project-model-edited", id: msg.id, ok: false });
      return true;
    }
    const model = JSON.parse(new TextDecoder().decode(readAuxBytes(uorcWasm)));
    postMessage({ kind: "project-model-edited", id: msg.id, ok: true, model });
    return true;
  }

  if (msg.kind === "revert-project-model") {
    const uorcWasm = await ensureUorcSceneHost();
    stageIoBytes(uorcWasm, new TextEncoder().encode(String(msg.modelId || "")));
    const targetVer = Math.max(1, msg.targetVersion | 0) >>> 0;
    const len = uorcWasm.verdant_wasm_revert_project_model_io(targetVer) >>> 0;
    if (len === 0) {
      postMessage({ kind: "project-model-reverted", id: msg.id, ok: false });
      return true;
    }
    const model = JSON.parse(new TextDecoder().decode(readAuxBytes(uorcWasm)));
    postMessage({ kind: "project-model-reverted", id: msg.id, ok: true, model });
    return true;
  }

  if (msg.kind === "remove-project-model") {
    const uorcWasm = await ensureUorcSceneHost();
    stageIoBytes(uorcWasm, new TextEncoder().encode(String(msg.modelId || "")));
    const len = uorcWasm.verdant_wasm_remove_project_model_io() >>> 0;
    const kappaHex = len > 0 ? new TextDecoder().decode(readAuxBytes(uorcWasm)) : "";
    postMessage({ kind: "project-model-removed", id: msg.id, ok: len > 0, kappaHex });
    return true;
  }

  if (msg.kind === "list-project-models") {
    const uorcWasm = await ensureUorcSceneHost();
    const len = uorcWasm.verdant_wasm_list_project_models_aux() >>> 0;
    if (len === 0) {
      postMessage({ kind: "project-models-listed", id: msg.id, ok: false, models: [] });
      return true;
    }
    const payload = JSON.parse(new TextDecoder().decode(readAuxBytes(uorcWasm)));
    postMessage({ kind: "project-models-listed", id: msg.id, ok: true, ...payload });
    return true;
  }

  if (msg.kind === "compose-glb-append") {
    const raw = msg.bytes instanceof Uint8Array ? msg.bytes : new Uint8Array(msg.bytes || []);
    const uorcWasm = await getUorcWasm();
    if (!raw.byteLength || !stageIoBytes(uorcWasm, raw)) {
      postMessage({ kind: "glb-composed", id: msg.id, ok: false });
      return true;
    }
    const tris = uorcWasm.verdant_wasm_glb_compose_append_io() >>> 0;
    if (!tris) {
      postMessage({ kind: "glb-composed", id: msg.id, ok: false });
      return true;
    }
    setCustomGlbActive(true);
    const cam = readSolvedCamera(simHost());
    setLastAnchor([...cam.lookTarget]);
    drawScene(cam.eye, cam.lookTarget, getCurrentLod(), 0, false, 0, 0, true);
    postMessage({ kind: "glb-composed", id: msg.id, ok: true, tris, camera: cam });
    return true;
  }

  if (msg.kind === "set-env-sky") {
    const renderer = getRenderer();
    renderer?.setTex?.(2, msg.bitmap || null);
    const sh = simHost();
    const cam = readSolvedCamera(sh);
    drawScene(cam.eye, cam.lookTarget, getCurrentLod(), 0, false, 0, 0, true);
    let cognition = null;
    const uorcWasm = getLoadedUorcWasm();
    if (uorcWasm && typeof uorcWasm.verdant_wasm_record_part_hydration_io === "function") {
      const wire = msg.bitmap
        ? `stage-environment|polyhaven|${msg.label || "Poly Haven HDRI Sky Dome"}`
        : "stage-environment|verdant-prism|Procedural Atmosphere";
      stageIoBytes(uorcWasm, new TextEncoder().encode(wire));
      const len = uorcWasm.verdant_wasm_record_part_hydration_io(msg.bitmap ? 4 : 1) >>> 0;
      if (len) cognition = JSON.parse(new TextDecoder().decode(readAuxBytes(uorcWasm)));
    }
    postMessage({ kind: "env-sky-set", id: msg.id, ok: true, active: Boolean(msg.bitmap), cognition });
    return true;
  }

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

  if (msg.kind === "vcs-commit-pack-holo" || msg.kind === "paged-workload-step" || msg.kind === "authority-exec") {
    const uorcWasm = await ensureUorcSceneHost();
    stageIoBytes(uorcWasm, new TextEncoder().encode(JSON.stringify(msg.request || {})));
    const fn =
      msg.kind === "vcs-commit-pack-holo"
        ? uorcWasm.verdant_wasm_vcs_commit_and_pack_holo_io
        : msg.kind === "authority-exec"
          ? uorcWasm.verdant_wasm_authority_exec_io
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

  if (msg.kind === "stitch-shard") {
    const uorcWasm = await getUorcWasm();
    let source = msg.bytes instanceof Uint8Array ? msg.bytes : null;
    if (!source || !source.byteLength) {
      uorcWasm.verdant_wasm_set_lod_rank(Math.min(3, simHost().verdant_wasm_active_lod_rank() >>> 0));
      const snapLen = uorcWasm.verdant_wasm_snapshot_encode() >>> 0;
      if (snapLen === 0) {
        postMessage({ kind: "stitch-sharded", id: msg.id, ok: false });
        return true;
      }
      source = readIoBytes(uorcWasm);
    }
    stageIoBytes(uorcWasm, source);
    const shards = [];
    for (let s = 0; s < 3; s++) {
      const len = uorcWasm.verdant_wasm_stitch_shard_io(s) >>> 0;
      if (len === 0) {
        postMessage({ kind: "stitch-sharded", id: msg.id, ok: false });
        return true;
      }
      shards.push(readAuxBytes(uorcWasm));
    }
    postMessage(
      { kind: "stitch-sharded", id: msg.id, ok: true, sourceBytes: source.byteLength, shards },
      shards.map((u) => u.buffer),
    );
    return true;
  }

  if (msg.kind === "stitch-recover") {
    const uorcWasm = await getUorcWasm();
    const a = msg.shardA instanceof Uint8Array ? msg.shardA : new Uint8Array(msg.shardA || []);
    const b = msg.shardB instanceof Uint8Array ? msg.shardB : new Uint8Array(msg.shardB || []);
    const combined = new Uint8Array(a.byteLength + b.byteLength);
    combined.set(a, 0);
    combined.set(b, a.byteLength);
    stageIoBytes(uorcWasm, combined);
    const recLen = uorcWasm.verdant_wasm_stitch_recover_io(a.byteLength >>> 0) >>> 0;
    if (recLen === 0) {
      postMessage({ kind: "stitch-recovered", id: msg.id, ok: false });
      return true;
    }
    const recovered = readAuxBytes(uorcWasm);
    postMessage({ kind: "stitch-recovered", id: msg.id, ok: true, recovered }, [recovered.buffer]);
    return true;
  }

  if (msg.kind === "scene-outliner") {
    const uorcWasm = await getUorcWasm();
    if (!getCustomGlbActive() && !getUorcSceneActive()) {
      uorcWasm.verdant_wasm_set_lod_rank(simHost().verdant_wasm_active_lod_rank() >>> 0);
    }
    const len = uorcWasm.verdant_wasm_scene_outliner_aux((msg.observedFrameMicros || 8200) >>> 0) >>> 0;
    if (len === 0) {
      postMessage({ kind: "scene-outliner", id: msg.id, ok: false });
      return true;
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
    postMessage({
      kind: "scene-outliner",
      id: msg.id,
      ok: true,
      header,
      entities,
      camera: readSolvedCamera(sh),
      universeCell: readUniverseCell(sh),
    });
    return true;
  }

  if (msg.kind === "sabine-rt60") {
    const wasm = getWasm();
    const vol = Math.max(10, Math.round(Number(msg.volumeM3) || 1800)) >>> 0;
    const absMilli = Math.max(100, Math.round(Number(msg.absorptionMilliM2) || 96000)) >>> 0;
    const rt60Ms = wasm.verdant_wasm_sabine_rt60_millis(vol, absMilli) >>> 0;
    postMessage({ kind: "sabine-rt60", id: msg.id, ok: true, volumeM3: vol, absorptionMilliM2: absMilli, rt60Ms });
    return true;
  }

  return false;
}

// Native browser GLB PBR texture decoder (`createImageBitmap` + `OffscreenCanvas`), avoiding
// shipping `image`/`moxcms`/`png`/`zune-jpeg` inside `verdant-uorc.wasm` on `wasm32`.
export async function decodeGlbPbrTexturesInBrowser(rawGlbBytes, maxEdgePx, renderer) {
  const out = { albedo: false, metallicRoughness: false, normal: false, emissive: false };
  if (!renderer?.setTex || typeof createImageBitmap !== "function" || typeof OffscreenCanvas !== "function") {
    return out;
  }
  if (!(rawGlbBytes instanceof Uint8Array) || rawGlbBytes.byteLength < 20) return out;
  const dv = new DataView(rawGlbBytes.buffer, rawGlbBytes.byteOffset, rawGlbBytes.byteLength);
  if (dv.getUint32(0, true) !== 0x46546c67) return out;
  const jsonLen = dv.getUint32(12, true);
  if (dv.getUint32(16, true) !== 0x4e4f534a || 20 + jsonLen > rawGlbBytes.byteLength) return out;
  let gltfJson;
  try {
    gltfJson = JSON.parse(new TextDecoder().decode(rawGlbBytes.subarray(20, 20 + jsonLen)));
  } catch {
    return out;
  }
  const binHeaderOff = 20 + jsonLen;
  if (binHeaderOff + 8 > rawGlbBytes.byteLength || dv.getUint32(binHeaderOff + 4, true) !== 0x004e4942) {
    return out;
  }
  const binLen = dv.getUint32(binHeaderOff, true);
  const binOff = binHeaderOff + 8;
  const binChunk = rawGlbBytes.subarray(binOff, Math.min(rawGlbBytes.byteLength, binOff + binLen));
  const textures = Array.isArray(gltfJson.textures) ? gltfJson.textures : [];
  const images = Array.isArray(gltfJson.images) ? gltfJson.images : [];
  const bufferViews = Array.isArray(gltfJson.bufferViews) ? gltfJson.bufferViews : [];
  const materials = Array.isArray(gltfJson.materials) ? gltfJson.materials : [];
  const imgIndexForTex = (texInfo) => {
    const ti = texInfo?.index;
    return typeof ti === "number" && typeof textures[ti]?.source === "number" ? textures[ti].source : null;
  };
  let albedoIdx = null, mrIdx = null, normIdx = null, emisIdx = null;
  for (const mat of materials) {
    if (albedoIdx == null) albedoIdx = imgIndexForTex(mat?.pbrMetallicRoughness?.baseColorTexture);
    if (mrIdx == null) mrIdx = imgIndexForTex(mat?.pbrMetallicRoughness?.metallicRoughnessTexture);
    if (normIdx == null) normIdx = imgIndexForTex(mat?.normalTexture);
    if (emisIdx == null) emisIdx = imgIndexForTex(mat?.emissiveTexture);
  }
  const cap = Math.max(64, Math.min(2048, Number(maxEdgePx) || 1024));
  const ep = ((Date.now() & 0x7fffffff) || 1) >>> 0;
  const decodeOne = async (imgIdx, slot, key) => {
    if (imgIdx == null || !images[imgIdx]) return;
    const bvIdx = images[imgIdx].bufferView;
    if (typeof bvIdx !== "number" || !bufferViews[bvIdx]) return;
    const bv = bufferViews[bvIdx];
    const start = bv.byteOffset || 0;
    const end = start + (bv.byteLength || 0);
    if (end <= start || end > binChunk.byteLength) return;
    try {
      const blob = new Blob([binChunk.subarray(start, end)], { type: images[imgIdx].mimeType || "image/png" });
      const bmp = await createImageBitmap(blob);
      const maxDim = Math.max(bmp.width, bmp.height, 1);
      const scale = maxDim > cap ? cap / maxDim : 1;
      const w = Math.max(1, Math.round(bmp.width * scale));
      const h = Math.max(1, Math.round(bmp.height * scale));
      const oc = new OffscreenCanvas(w, h);
      const ctx2d = oc.getContext("2d");
      if (ctx2d) {
        ctx2d.drawImage(bmp, 0, 0, w, h);
        const rgba = new Uint8Array(ctx2d.getImageData(0, 0, w, h).data.buffer);
        renderer.setTex(slot, rgba, w, h, ep);
        out[key] = true;
      }
      bmp.close?.();
    } catch {}
  };
  await Promise.all([
    decodeOne(albedoIdx, 1, "albedo"),
    decodeOne(mrIdx, 3, "metallicRoughness"),
    decodeOne(normIdx, 4, "normal"),
    decodeOne(emisIdx, 5, "emissive"),
  ]);
  return out;
}
