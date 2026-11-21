// Post-boot extended Verdant Worker handlers (`site/engine-worker-extended.mjs`),
// lazy-loaded on demand by `site/engine-worker-codec.mjs` so cold-boot `/` stays well below FR-03.

import { readAuxBytes, readSolvedCamera, stageIoBytes } from "./engine-worker-codec.mjs";

export async function handleExtendedWorkerMessage(msg, ctx) {
  const {
    ensureUorcSceneHost,
    getUorcWasm,
    getLoadedUorcWasm,
    simHost,
    getCanvas,
    getRenderer,
    getEmaFrameMicros,
    getOrganModes,
    getCurrentLod,
    drawScene,
    setCustomGlbActive,
    setLastAnchor,
    postMessage,
  } = ctx;

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
