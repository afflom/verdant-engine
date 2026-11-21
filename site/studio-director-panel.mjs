// Role 5 Director Panel (Multi-Shot Cinematography, Camera Choreography & 4-Cascade CSM Lighting)
// Extracted from `site/studio.mjs` to preserve shipped JS modularity (<= 1250 L).

import { PRISM_CSM_SPLIT_METERS } from "./prism-bridge-meta.mjs";
import { DIRECTOR_SHOTS } from "./studio-data.mjs";

export function buildDirectorPanel({ getStudioCtx, getLatestObserverTelemetry, getInline, getDialog }) {
  const pDirector = document.createElement("section");
  pDirector.className = "ve-m-panel";
  pDirector.hidden = true;

  const cCine = document.createElement("div");
  cCine.className = "ve-m-card";
  const hCine = document.createElement("h3");
  hCine.textContent = "Director · Multi-Shot Cinematography, Camera Choreography & 4-Cascade CSM Lighting";
  const pCineDesc = document.createElement("p");
  pCineDesc.textContent =
    "Stage multi-camera shot lists, capture live keyframes, run timed shot choreography, inspect near-sphere frustum clearance (>= 290 mm) and 4-cascade shadow splits, or direct cues via local coded_gemm.";

  const cineControls = document.createElement("div");
  cineControls.className = "ve-m-row";
  const btnPlaySeq = document.createElement("button");
  btnPlaySeq.type = "button";
  btnPlaySeq.className = "ve-m-btn primary";
  btnPlaySeq.id = "ve-director-seq-btn";
  btnPlaySeq.setAttribute("aria-pressed", "false");
  btnPlaySeq.textContent = "Play Shot Sequence";

  const btnCaptureShot = document.createElement("button");
  btnCaptureShot.type = "button";
  btnCaptureShot.className = "ve-m-btn";
  btnCaptureShot.id = "ve-director-capture-btn";
  btnCaptureShot.textContent = "Capture Current Keyframe";

  const btnRigOrbit = document.createElement("button");
  btnRigOrbit.type = "button";
  btnRigOrbit.className = "ve-m-btn";
  btnRigOrbit.textContent = "Rig: Orbit";
  btnRigOrbit.addEventListener("click", () => getStudioCtx()?.onSetCameraMode?.("landmark"));

  const btnRigThird = document.createElement("button");
  btnRigThird.type = "button";
  btnRigThird.className = "ve-m-btn";
  btnRigThird.textContent = "Rig: 3rd-Person IK";
  btnRigThird.addEventListener("click", () => getStudioCtx()?.onSetCameraMode?.("third"));

  const btnRigFirst = document.createElement("button");
  btnRigFirst.type = "button";
  btnRigFirst.className = "ve-m-btn";
  btnRigFirst.textContent = "Rig: 1st-Person Eye";
  btnRigFirst.addEventListener("click", () => getStudioCtx()?.onSetCameraMode?.("first"));

  const btnLookDev = document.createElement("button");
  btnLookDev.type = "button";
  btnLookDev.className = "ve-m-btn";
  const syncLookDevLabel = () => {
    btnLookDev.textContent =
      getStudioCtx()?.getRenderMode?.() === "tile" ? "Look-Dev: 16x16 Ref Tile" : "Look-Dev: Hardware PBR";
  };
  btnLookDev.addEventListener("click", async () => {
    await getStudioCtx()?.onToggleRenderMode?.();
    syncLookDevLabel();
  });

  cineControls.append(btnPlaySeq, btnCaptureShot, btnRigOrbit, btnRigThird, btnRigFirst, btnLookDev);

  const shotList = document.createElement("div");
  shotList.className = "ve-step-list";
  const directorLog = document.createElement("pre");
  directorLog.className = "ve-m-log";
  directorLog.id = "ve-director-log";
  directorLog.textContent =
    `Optical Frustum Guard: >= 290 mm wall clearance · 4-Cascade CSM Splits: ${PRISM_CSM_SPLIT_METERS.map((m) => `${m.toFixed(1)}m`).join(" / ")}\n` +
    `Select any shot below or click 'Play Shot Sequence' to run automated camera choreography.`;

  const customShots = DIRECTOR_SHOTS.map((s) => ({ ...s, target: [...s.target] }));
  let activeShotIdx = 0;
  let sequenceTimer = 0;

  const stopSequence = () => {
    if (sequenceTimer) {
      clearInterval(sequenceTimer);
      sequenceTimer = 0;
      btnPlaySeq.setAttribute("aria-pressed", "false");
      btnPlaySeq.textContent = "Play Shot Sequence";
    }
  };

  const activateShot = async (idx) => {
    const shot = customShots[idx];
    if (!shot) return;
    activeShotIdx = idx;
    const ctx = getStudioCtx();
    if (shot.possess) {
      await ctx?.onSetCameraMode?.("third");
    } else {
      await ctx?.onFocusPoint?.(shot.target, shot.dist, shot.title, shot.yawMilli, shot.pitchMilli);
    }
    renderShotList();
    const obs = getLatestObserverTelemetry();
    directorLog.textContent =
      `Active Cut: ${shot.title} (${shot.rig})\n` +
      `  Anchor: (${shot.target.map((v) => v.toFixed(1)).join(", ")}) m · Distance: ${shot.dist.toFixed(1)} m\n` +
      `  4-Cascade CSM Splits: ${PRISM_CSM_SPLIT_METERS.map((m) => `${m.toFixed(1)}m`).join(" / ")} · Guarded Clearance: ${obs?.camClearanceMm ?? 290} mm`;
  };

  const renderShotList = () => {
    shotList.replaceChildren();
    customShots.forEach((shot, idx) => {
      const row = document.createElement("div");
      row.className = "ve-step-item";
      const copy = document.createElement("div");
      copy.className = "ve-step-copy";
      const t = document.createElement("span");
      t.className = "ve-step-title";
      t.textContent = shot.title;
      const m = document.createElement("span");
      m.className = "ve-step-meta";
      m.textContent = `${shot.rig} · Anchor (${shot.target.map((v) => v.toFixed(1)).join(", ")})m · Dist ${shot.dist.toFixed(1)}m`;
      copy.append(t, m);

      const act = document.createElement("div");
      act.className = "ve-m-row";
      if (idx === activeShotIdx) {
        const badge = document.createElement("span");
        badge.className = "ve-badge";
        badge.textContent = "On Air";
        act.append(badge);
      }
      const cutBtn = document.createElement("button");
      cutBtn.type = "button";
      cutBtn.className = "ve-m-btn primary";
      cutBtn.dataset.shotId = shot.id;
      cutBtn.textContent = "Cut to Shot";
      cutBtn.addEventListener("click", () => activateShot(idx));
      act.append(cutBtn);
      row.append(copy, act);
      shotList.append(row);
    });
  };
  renderShotList();

  btnPlaySeq.addEventListener("click", () => {
    if (sequenceTimer) {
      stopSequence();
      return;
    }
    btnPlaySeq.setAttribute("aria-pressed", "true");
    btnPlaySeq.textContent = "Stop Sequence";
    getStudioCtx()?.onSetSimLive?.(true);
    activateShot((activeShotIdx + 1) % customShots.length);
    sequenceTimer = window.setInterval(() => {
      const dlg = getDialog();
      if (!getInline() && dlg && !dlg.open) {
        stopSequence();
        return;
      }
      activateShot((activeShotIdx + 1) % customShots.length);
    }, 2600);
  });

  btnCaptureShot.addEventListener("click", () => {
    const obs = getLatestObserverTelemetry();
    const tgt = obs?.lookTarget || [2.2, 3.8, 9.2];
    const eye = obs?.eye || [10.2, 8.5, 25.3];
    const dist = Math.max(3.5, Math.hypot(eye[0] - tgt[0], eye[1] - tgt[1], eye[2] - tgt[2]));
    const nextNum = String(customShots.length + 1).padStart(2, "0");
    customShots.push({
      id: `shot-custom-${customShots.length}`,
      title: `${nextNum}. Captured Studio Keyframe`,
      rig: "Orbit",
      target: [Number(tgt[0]) || 0, Number(tgt[1]) || 2, Number(tgt[2]) || 0],
      dist: Number(dist.toFixed(1)),
      yawMilli: 480,
      pitchMilli: 280,
    });
    activeShotIdx = customShots.length - 1;
    renderShotList();
  });

  const slmRow = document.createElement("div");
  slmRow.className = "ve-m-row";
  for (const cue of [
    "focus observer core gimbal",
    "drive the 4-wheel suspension vehicle",
    "stream lod4 ultra hd",
    "switch to 3rd person explorer",
  ]) {
    const b = document.createElement("button");
    b.type = "button";
    b.className = "ve-m-btn";
    b.textContent = `Cue: ${cue}`;
    b.addEventListener("click", async () => {
      const res = await getStudioCtx()?.onRoutePrompt?.(cue);
      if (res?.ok) {
        directorLog.textContent =
          `Local coded_gemm Director Admitted Cue:\n` +
          `  Intent: ${res.slug} · Confidence: ${(res.scoreMilli / 10).toFixed(1)}% (logit=${res.rawLogit})\n` +
          `  Result: ${res.summary}`;
      }
    });
    slmRow.append(b);
  }

  cCine.append(hCine, pCineDesc, cineControls, shotList, slmRow, directorLog);
  pDirector.append(cCine);

  return { pDirector, syncLookDevLabel, stopSequence };
}
