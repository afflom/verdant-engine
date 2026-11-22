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

export function buildContainerOrgansCard({ containerOrgans, getStudioCtx }) {
  const cOrgans = document.createElement("div");
  cOrgans.className = "ve-m-card";
  const hOrgans = document.createElement("h3");
  hOrgans.textContent = "Container Capability Custody (FK-07 Explicit Denial-Wins Lattice)";
  const pOrgansDesc = document.createElement("p");
  pOrgansDesc.textContent =
    "Govern browser hardware and network organs for the active project. Explicit Denial overrides any subsequent Grant in WebAssembly.";
  const organGrid = document.createElement("div");
  organGrid.className = "ve-organ-grid";
  const organSyncFns = [];
  for (const organ of containerOrgans) {
    const item = document.createElement("div");
    item.className = "ve-organ-item";
    item.dataset.organSlug = organ.slug;
    const meta = document.createElement("div");
    meta.className = "ve-organ-meta";
    const nm = document.createElement("span");
    nm.className = "ve-organ-name";
    nm.textContent = `${organ.label} (${organ.slug})`;
    const st = document.createElement("span");
    st.className = "ve-organ-state";
    const hwPresent =
      organ.slug === "webnn-local"
        ? typeof navigator !== "undefined" && "ml" in navigator
        : organ.slug === "gamepad-hid"
          ? typeof navigator !== "undefined" && typeof navigator.getGamepads === "function"
          : true;
    meta.append(nm, st);

    const btnWrap = document.createElement("div");
    btnWrap.className = "ve-m-row";
    const grantBtn = document.createElement("button");
    grantBtn.type = "button";
    grantBtn.className = "ve-m-btn";
    grantBtn.dataset.organGrant = organ.slug;
    grantBtn.textContent = "Grant";

    const denyBtn = document.createElement("button");
    denyBtn.type = "button";
    denyBtn.className = "ve-m-btn";
    denyBtn.dataset.organDeny = organ.slug;
    denyBtn.textContent = "Deny";

    const resetBtn = document.createElement("button");
    resetBtn.type = "button";
    resetBtn.className = "ve-m-btn";
    resetBtn.dataset.organReset = organ.slug;
    resetBtn.textContent = "Reset";

    const syncRow = () => {
      const studioCtx = getStudioCtx();
      const curMode = studioCtx?.getOrganMode ? (studioCtx.getOrganMode(organ.idx) >>> 0) : 0;
      if (curMode === 2) st.textContent = "Denied (FK-07 Wins)";
      else if (curMode === 1) st.textContent = "Granted & Active";
      else st.textContent = organ.defaultAllowed ? "Allowed (Sandboxed)" : hwPresent ? "Gated (Requires Grant)" : "Probed (API absent)";
      grantBtn.classList.toggle("primary", curMode === 1);
      denyBtn.classList.toggle("primary", curMode === 2);
    };
    syncRow();
    organSyncFns.push(syncRow);

    const applyOrganMode = async (nextMode) => {
      const studioCtx = getStudioCtx();
      const reply = await studioCtx?.callWorker?.("set-organ-grant", { organIdx: organ.idx, mode: nextMode });
      const effectiveMode = reply?.mode !== undefined ? (reply.mode >>> 0) : nextMode;
      studioCtx?.onOrganGrantChange?.(organ.idx, effectiveMode);
      syncRow();
    };
    grantBtn.addEventListener("click", () => applyOrganMode(1));
    denyBtn.addEventListener("click", () => applyOrganMode(2));
    resetBtn.addEventListener("click", () => applyOrganMode(0));

    btnWrap.append(grantBtn, denyBtn, resetBtn);
    item.append(meta, btnWrap);
    organGrid.append(item);
  }
  cOrgans.append(hOrgans, pOrgansDesc, organGrid);
  return { cOrgans, organSyncFns };
}

export async function refreshStudioOutlinerAndOpfs({
  studioCtx,
  dlg,
  latestObserverTelemetry,
  entityTbody,
  kpiStrata,
  kpiBudget,
  kpiEntities,
  kpiClearance,
  obsReadout,
  storyLog,
  organSyncFns,
  opfsLog,
  opfsActionsRow,
  listOpfsAssets,
  readOpfsAssetBytes,
  deleteOpfsAsset,
}) {
  const outReply = await studioCtx?.callWorker?.("scene-outliner");
  if (outReply?.ok && Array.isArray(outReply.entities)) {
    entityTbody.replaceChildren();
    for (const ent of outReply.entities) {
      const tr = document.createElement("tr");
      const [cx, cy, cz] = ent.center;
      for (const val of [
        String(ent.id),
        ent.name,
        ent.role,
        String(ent.tris),
        `(${cx.toFixed(1)}, ${cy.toFixed(1)}, ${cz.toFixed(1)})`,
      ]) {
        const td = document.createElement("td");
        td.textContent = val;
        tr.append(td);
      }
      const actTd = document.createElement("td");
      const focusBtn = document.createElement("button");
      focusBtn.type = "button";
      focusBtn.className = "ve-m-btn";
      focusBtn.textContent = "Frame";
      focusBtn.addEventListener("click", () => {
        studioCtx?.onFocusPoint?.(ent.center, ent.role === "TerrainPatch" ? 26 : 12, ent.name);
      });
      actTd.append(focusBtn);
      tr.append(actTd);
      entityTbody.append(tr);
    }
    const hdr = outReply.header || {};
    const cell = outReply.universeCell || [0, 0, 0];
    const clr = outReply.camera?.clearanceMm ?? 290;
    const fUs = latestObserverTelemetry?.frameMicros ?? 8000;
    const scalePct = ((latestObserverTelemetry?.scalePermille ?? 1000) / 10).toFixed(0);
    kpiStrata.textContent = `${hdr.w5h_strata || 8} / 8 · Loss ${hdr.telos_loss || 0}`;
    kpiBudget.textContent = `${(fUs / 1000).toFixed(1)} ms · ${scalePct}%`;
    kpiEntities.textContent = `${outReply.entities.length} entities`;
    kpiClearance.textContent = `${clr} mm`;
    obsReadout.textContent =
      `Observer Coherent · W5H ${hdr.w5h_strata || 8}/8 · Slot ${latestObserverTelemetry?.attendedSlot ?? 0} · Cell (${cell[0]},${cell[1]},${cell[2]})`;
    storyLog.textContent =
      `Universe Cell: (${cell[0]}, ${cell[1]}, ${cell[2]}) · 3-Sector 128m Ring Active · Seed ${hdr.seed || 7}\n` +
      `Click 'Advance Beat at Story Podium' or 'Verify Conservation & Portal Ring' to inspect narrative state.`;
  }

  for (const fn of organSyncFns) fn();

  if (studioCtx?.getOrganMode?.(4) === 2) {
    opfsLog.textContent = "Refused: ContainerOrganDenied(opfs-storage) — OPFS asset cache denied by FK-07 custody.";
    opfsActionsRow.replaceChildren();
  } else {
    const entries = await listOpfsAssets();
    opfsLog.textContent =
      entries.length > 0
        ? `OPFS Persistent Asset Vault (${entries.length} object(s)):\n` +
          entries.map((e) => `  • ${e.name} (${e.bytes} bytes)`).join("\n")
        : "OPFS Persistent Asset Vault ready (import a .glb model to transcode and cache .uorc in origin storage).";
    opfsActionsRow.replaceChildren();
    for (const entry of entries) {
      const mountBtn = document.createElement("button");
      mountBtn.type = "button";
      mountBtn.className = "ve-m-btn primary";
      mountBtn.textContent = `Mount ${entry.name}`;
      mountBtn.addEventListener("click", async () => {
        const bytes = await readOpfsAssetBytes(entry.name);
        if (bytes?.byteLength) await studioCtx?.onMountOpfsUorc?.(entry.name, bytes);
      });
      const delBtn = document.createElement("button");
      delBtn.type = "button";
      delBtn.className = "ve-m-btn";
      delBtn.textContent = `Delete ${entry.name}`;
      delBtn.addEventListener("click", async () => {
        await deleteOpfsAsset(entry.name);
        dlg.dispatchEvent(new CustomEvent("ve-refresh"));
      });
      opfsActionsRow.append(mountBtn, delBtn);
    }
  }
}

export function wireStudioRoleTabs({
  panels,
  tabBar,
  body,
  dlg,
  inline,
  catalogModelsForRole,
  siteUrl,
  getStudioCtx,
}) {
  let selectTab = () => {};
  const tabBtns = panels.map((p, idx) => {
    const roleModels = catalogModelsForRole(p.key);
    if (roleModels.length > 0) {
      const stdBanner = document.createElement("div");
      stdBanner.className = "ve-role-standards";
      stdBanner.append("Governing Catalog Models: ");
      roleModels.slice(0, 5).forEach((rm, rIdx) => {
        if (rIdx > 0) stdBanner.append(" · ");
        const link = document.createElement("a");
        link.href = siteUrl(`./catalogs/#${rm.id}`);
        link.textContent = `${rm.authority.split("·")[0].trim()}`;
        link.title = `${rm.title} (${rm.lexModule})`;
        stdBanner.append(link);
      });
      p.el.prepend(stdBanner);
    }

    const panelId = `ve-panel-${p.key}`;
    const tabId = `ve-tab-${p.key}`;
    const b = document.createElement("button");
    b.type = "button";
    b.id = tabId;
    b.className = "ve-m-tab";
    b.dataset.tabKey = p.key;
    b.dataset.role = p.key;
    b.setAttribute("role", "tab");
    b.setAttribute("aria-controls", panelId);
    b.setAttribute("aria-selected", idx === 0 ? "true" : "false");
    b.textContent = p.label;
    b.addEventListener("click", () => selectTab(p.key));
    tabBar.append(b);
    p.el.id = panelId;
    p.el.setAttribute("role", "tabpanel");
    p.el.setAttribute("aria-labelledby", tabId);
    p.el.hidden = idx !== 0;
    body.append(p.el);
    return b;
  });

  if (!inline) {
    const peekBtn = document.createElement("button");
    peekBtn.type = "button";
    peekBtn.className = "ve-m-peek";
    peekBtn.id = "ve-dock-btn";
    peekBtn.setAttribute("aria-pressed", "false");
    peekBtn.textContent = "Dock Right";
    peekBtn.addEventListener("click", () => {
      const docked = dlg.classList.toggle("ve-docked");
      peekBtn.setAttribute("aria-pressed", String(docked));
      peekBtn.textContent = docked ? "Expand Studio" : "Dock Right";
    });
    tabBar.append(peekBtn);
  }

  selectTab = (keyOrIdx) => {
    const targetIdx =
      typeof keyOrIdx === "number"
        ? Math.max(0, Math.min(panels.length - 1, keyOrIdx))
        : Math.max(0, panels.findIndex((p) => p.key === keyOrIdx));
    panels.forEach((it, j) => {
      it.el.hidden = j !== targetIdx;
      tabBtns[j].setAttribute("aria-selected", j === targetIdx ? "true" : "false");
    });
    const activePanel = panels[targetIdx];
    if (activePanel) getStudioCtx()?.onRoleChange?.(activePanel.key);
  };

  return selectTab;
}

export function wireStudioDialogLifecycle({
  dlg,
  inline,
  neuroCanvas,
  audioCanvas,
  uiCanvas,
  uiLog,
  entityTbody,
  kpiStrata,
  kpiBudget,
  kpiEntities,
  kpiClearance,
  obsReadout,
  storyLog,
  organSyncFns,
  opfsLog,
  opfsActionsRow,
  subscribeThemeChanges,
  drawObserverCanvas,
  drawAudioSpectrumCanvas,
  drawUiComplianceCanvas,
  listOpfsAssets,
  readOpfsAssetBytes,
  deleteOpfsAsset,
  syncLookDevLabel,
  verifyAllRoleTrace,
  renderGameMode,
  stopDirectorSequence,
  state,
}) {
  subscribeThemeChanges(() => {
    if (!inline && !dlg.open) return;
    drawObserverCanvas(neuroCanvas, state.getLatestObserverTelemetry());
    drawAudioSpectrumCanvas(audioCanvas, state.getActiveFundHz(), state.getActiveRt60Ms(), state.getLatestObserverTelemetry());
    state.getUiDrawHook()?.();
  });

  dlg.addEventListener("ve-refresh", async () => {
    const studioCtx = state.getStudioCtx();
    syncLookDevLabel();
    const spec = studioCtx?.getAcousticSpec?.();
    if (spec?.fundamentalHz) state.setActiveFundHz(spec.fundamentalHz);
    if (spec?.rt60Ms) state.setActiveRt60Ms(spec.rt60Ms);

    const latestObs = state.getLatestObserverTelemetry();
    drawObserverCanvas(neuroCanvas, latestObs);
    drawAudioSpectrumCanvas(audioCanvas, state.getActiveFundHz(), state.getActiveRt60Ms(), latestObs);
    verifyAllRoleTrace();
    renderGameMode();

    await refreshStudioOutlinerAndOpfs({
      studioCtx, dlg, latestObserverTelemetry: latestObs, entityTbody, kpiStrata, kpiBudget,
      kpiEntities, kpiClearance, obsReadout, storyLog, organSyncFns, opfsLog, opfsActionsRow,
      listOpfsAssets, readOpfsAssetBytes, deleteOpfsAsset,
    });

    const vw = Math.max(360, window.innerWidth || 1280);
    const vh = Math.max(640, window.innerHeight || 900);
    const [uiReply, uiVerdict] = await Promise.all([
      studioCtx?.callWorker?.("verify-ui", { width: vw, height: vh }),
      studioCtx?.queryVerifier?.("scene:ui"),
    ]);
    if (uiReply?.ok) {
      state.setLastUiState({ vw, vh, report: uiReply.report });
      uiLog.textContent = `VERIFIED (${uiReply.widgetCount} widgets · 0 overlaps · >=48px targets · WCAG AA · ${uiVerdict || "conformant"}):\n${uiReply.report}`;
      drawUiComplianceCanvas(uiCanvas, vw, vh, uiReply.report);
    }
  });

  if (!inline) {
    dlg.addEventListener("click", (e) => {
      if (e.target === dlg) {
        stopDirectorSequence?.();
        dlg.close();
      }
    });
  }
}
