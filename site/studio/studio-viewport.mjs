// Verdant Studio Viewport, Camera Rig, Progressive LOD & Render Loop Controller
// (`site/studio/studio-viewport.mjs`). Extracted from `studio-page.mjs` for sub-1125L JS modularity.

export function createStudioViewportController({
  canvas,
  simBtn,
  camBtn,
  statusEl,
  LODS,
  NODES,
  callWorker,
  chip,
  currentLodSpec,
  syncHud,
  state,
}) {
  function pxDims() {
    if (!canvas) return { w: 640, h: 360 };
    const dpr = Math.min(window.devicePixelRatio || 1, 4);
    return {
      w: Math.max(1, Math.floor((canvas.clientWidth || 640) * dpr)),
      h: Math.max(1, Math.floor((canvas.clientHeight || 360) * dpr)),
    };
  }

  async function syncFallbackBuffers() {
    const fbR = state.getFbR();
    const worker = state.getWorker();
    if (state.getOffOk() || !fbR || !worker) return;
    const r = await callWorker("get-packed-buffers", { simClockSec: state.getSimClockSec() });
    if (r?.ok) {
      fbR.uploadPackedBuffers(r.pos, r.norm, r.col, r.idx);
      state.setLTris(r.tris || state.getLTris());
      if (r.camera) {
        state.setCamera(r.camera.eye, r.camera.lookTarget, r.camera.clearanceMm);
      }
    }
  }

  function requestRender() {
    if (state.getRafId() === 0) {
      state.setPrevNow(performance.now());
      state.setRafId(requestAnimationFrame(renderFrame));
    }
  }

  function renderFrame(now) {
    state.setRafId(0);
    const dt = Math.min(0.05, Math.max(0.001, (now - state.getPrevNow()) * 0.001));
    state.setPrevNow(now);
    const simLive = state.getSimLive();
    const followMover = state.getFollowMover();
    const stepping = simLive || followMover;
    if (stepping) state.setSimClockSec(state.getSimClockSec() + dt);
    const { dYaw: rawYaw, dPit, dDst } = state.consumePointerDeltas();
    const dragging = state.getDragging();
    let dYaw = rawYaw;
    if (simLive && !dragging && state.getCamPreset() === "landmark" && !followMover) dYaw += dt * 0.14;

    const { w, h } = pxDims();
    const lod = currentLodSpec();
    const worker = state.getWorker();
    const fbR = state.getFbR();
    const fbM = state.getFbM();
    if (state.getOffOk() && worker && state.getWorkerFramesInFlight() < 2) {
      state.incWorkerFramesInFlight();
      worker.postMessage({
        kind: "frame",
        width: w,
        height: h,
        dt,
        stepPhysics: stepping,
        followMover,
        fwd: 0,
        strafe: 0,
        speedScale: 1,
        jump: false,
        deltaYaw: dYaw,
        deltaPitch: dPit,
        deltaDist: dDst,
        gazedSlot: 0,
        dwellMs: 400,
        angSpeedMilli: dragging ? 600 : 60,
        lod,
        simClockSec: state.getSimClockSec(),
      });
    } else if (fbR && fbM && canvas) {
      if (canvas.width !== w || canvas.height !== h) {
        canvas.width = w;
        canvas.height = h;
      }
      const sEye = state.getSEye();
      const sTgt = state.getSTgt();
      const proj = fbM.mat4Perspective(Math.PI / 3.4, Math.max(0.2, w / h), 0.25, 140.0);
      fbR.render(fbM.mat4Mul(proj, fbM.mat4LookAt(sEye, sTgt, [0, 1, 0])), sEye, lod, state.getSimClockSec(), sTgt, Math.PI / 3.4);
    }
    if (simLive || dragging || followMover) state.setRafId(requestAnimationFrame(renderFrame));
  }

  function setSimLive(next) {
    const val = Boolean(next);
    state.setSimLive(val);
    const exp = state.getExperienceDesign();
    if (simBtn) {
      simBtn.setAttribute("aria-pressed", String(val));
      simBtn.textContent = val
        ? (exp?.plainLanguage?.pausePreview || "Pause Preview")
        : (exp?.plainLanguage?.playPreview || "Play Preview");
    }
    requestRender();
  }

  async function cycleCameraMode(force = null) {
    const order = ["landmark", "third", "first"];
    const nextPreset = force || order[(order.indexOf(state.getCamPreset()) + 1) % 3];
    state.setCamPreset(nextPreset);
    state.setFollowMover(false);
    const simClockSec = state.getSimClockSec();
    const spec =
      nextPreset === "landmark"
        ? ["Camera: Orbit", "camera-focus-landmark", { target: [2.2, 3.8, 9.2], dist: 18.5, yawMilli: 460, pitchMilli: 260 }, "Orbit Rig · Guarded (>=290mm)"]
        : nextPreset === "third"
          ? ["Camera: 3rd IK", "camera-set-mode", { mode: 0, simClockSec }, "3rd-Person Character Rig · Foot IK"]
          : ["Camera: 1st Eye", "camera-set-mode", { mode: 1, simClockSec }, "1st-Person Head-Masked Eye Rig"];
    if (camBtn) camBtn.textContent = spec[0];
    document.querySelectorAll("[data-wiz-cam]").forEach((b) => {
      b.classList.toggle("primary", b.dataset.wizCam === nextPreset);
    });
    const r = await callWorker(spec[1], spec[2]);
    if (r?.camera) {
      state.setCamera(r.camera.eye, r.camera.lookTarget, r.camera.clearanceMm);
    }
    await syncFallbackBuffers();
    syncHud();
    chip(statusEl, spec[3]);
    requestRender();
  }

  async function streamLod(targetRank) {
    const clamped = Math.max(0, Math.min(LODS.length - 1, targetRank | 0));
    state.setLodIdx(clamped);
    const lod = currentLodSpec();
    const r = await callWorker("stream-uorc", { targetRank: clamped, lod, simClockSec: state.getSimClockSec() });
    if (r?.tris) state.setLTris(r.tris);
    await syncFallbackBuffers();
    syncHud();
    chip(statusEl, `Dynamic workload scaled (${state.getLTris().toLocaleString()} tris · Σ=${r?.codedChecksum ?? 320})`);
    requestRender();
    return r;
  }

  async function focusPoint(target, dist = 12, label = "Entity", yawMilli = 520, pitchMilli = 300) {
    state.setCamPreset("landmark");
    state.setFollowMover(false);
    if (camBtn) camBtn.textContent = "Camera: Orbit";
    const r = await callWorker("camera-focus-landmark", { target, dist, yawMilli, pitchMilli, simClockSec: state.getSimClockSec() });
    if (r?.camera) {
      state.setCamera(r.camera.eye, r.camera.lookTarget, r.camera.clearanceMm);
    }
    await syncFallbackBuffers();
    syncHud();
    chip(statusEl, `Framed ${label} · Guarded Rig (${state.getLClr()} mm clearance)`);
    requestRender();
  }

  async function focusPreset(idx) {
    const node = NODES[idx];
    if (!node) return;
    const follow = Boolean(node.followMover);
    state.setFollowMover(follow);
    if (follow || node.id === 3) setSimLive(true);
    state.setCamPreset(node.possessExplorer ? "third" : "landmark");
    if (camBtn) camBtn.textContent = node.possessExplorer ? "Camera: 3rd IK" : "Camera: Orbit";
    const r = await callWorker("camera-focus-landmark", {
      possessExplorer: Boolean(node.possessExplorer),
      target: node.target,
      dist: node.dist,
      yawMilli: node.yawMilli ?? 560,
      pitchMilli: node.pitchMilli ?? 300,
      lod: currentLodSpec(),
      simClockSec: state.getSimClockSec(),
    });
    if (r?.camera) {
      state.setCamera(r.camera.eye, r.camera.lookTarget, r.camera.clearanceMm);
    }
    if (r?.tris) state.setLTris(r.tris);
    await syncFallbackBuffers();
    syncHud();
    chip(statusEl, `Framed ${node.name}`);
    if (state.getSimLive() || follow || !state.getOffOk()) requestRender();
  }

  async function setWorldSeed(seed) {
    const r = await callWorker("set-seed", { seed });
    if (r?.ok) {
      state.setLTris(r.tris || state.getLTris());
      await syncFallbackBuffers();
      syncHud();
      chip(statusEl, `${state.getActiveProject()} · Seed ${r.seed} · ${LODS[state.getLodIdx()].label}`);
      requestRender();
    }
    return r;
  }

  async function toggleRenderBackend(force = null) {
    const nextMode = force || (state.getRenderMode() === "webgl2" ? "tile" : "webgl2");
    state.setRenderMode(nextMode);
    await callWorker("set-render-mode", { mode: nextMode, simClockSec: state.getSimClockSec() });
    chip(statusEl, nextMode === "tile" ? "Look-Dev: Wasm 16x16 Software Tile" : "Look-Dev: Hardware WebGL2 3-Pass PBR");
    requestRender();
  }

  return {
    pxDims,
    syncFallbackBuffers,
    requestRender,
    renderFrame,
    setSimLive,
    cycleCameraMode,
    streamLod,
    focusPoint,
    focusPreset,
    setWorldSeed,
    toggleRenderBackend,
  };
}
