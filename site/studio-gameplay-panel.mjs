// Verdant Studio Gameplay, NPC AI, Q15 LIF Observer & Stitch Multiplayer Panel
// (`site/studio-gameplay-panel.mjs`), extracted from `site/studio.mjs`.

import {
  PRISM_TERRAIN_WALKABLE_POLYGONS,
  PRISM_WORKER_RING_CAPACITY,
} from "./prism-bridge-meta.mjs";
import { GAME_MODES, NPC_ROSTER } from "./studio-data.mjs";
import { drawObserverCanvas } from "./studio-media.mjs";

export function buildGameplayPanel({ getStudioCtx, siteUrl, setObserverDrawHook }) {
  const pGameplay = document.createElement("section");
  pGameplay.className = "ve-m-panel";
  pGameplay.hidden = true;

  const cModes = document.createElement("div");
  cModes.className = "ve-m-card";
  const hModes = document.createElement("h3");
  hModes.textContent = "Gameplay · Campaign, Arena & Tournament Rulesets";
  const pModesDesc = document.createElement("p");
  pModesDesc.textContent =
    "Configure player experience, round pacing, conserved tournament progression, and gaze-routed HUD topology, or launch the standalone playable 3D trial.";

  const modeBtnRow = document.createElement("div");
  modeBtnRow.className = "ve-m-row";
  const modeLog = document.createElement("pre");
  modeLog.className = "ve-m-log";
  modeLog.id = "ve-gameplay-mode-log";
  let activeModeIdx = 1;

  const renderGameMode = async (idx = activeModeIdx) => {
    activeModeIdx = idx;
    const studioCtx = getStudioCtx();
    const m = GAME_MODES[idx] || GAME_MODES[0];
    modeBtnRow.querySelectorAll("button[data-game-mode]").forEach((b, i) => {
      b.classList.toggle("primary", i === idx);
    });
    const verdict = await studioCtx?.queryVerifier?.(m.holoQuery);
    modeLog.textContent =
      `Active Game Mode: ${m.name} — ${m.subtitle}\n` +
      `  Session Topology : Up to ${m.maxPlayers} players · Round Clock ${m.roundSec}s · ${PRISM_WORKER_RING_CAPACITY}-frame Rollback Ring\n` +
      `  Scoring & Supply : ${m.scoring}\n` +
      `  HUD & Camera Rig : ${m.hudTopology}\n` +
      `  Core-Wasm Proof  : ${verdict || "conformant"}`;
  };

  GAME_MODES.forEach((m, idx) => {
    const b = document.createElement("button");
    b.type = "button";
    b.className = idx === activeModeIdx ? "ve-m-btn primary" : "ve-m-btn";
    b.dataset.gameMode = m.slug;
    b.textContent = m.name;
    b.addEventListener("click", async () => {
      const studioCtx = getStudioCtx();
      await studioCtx?.onSetCameraMode?.(m.cameraMode);
      studioCtx?.onSetSimLive?.(true);
      await renderGameMode(idx);
    });
    modeBtnRow.append(b);
  });

  const starterPlayLink = document.createElement("a");
  starterPlayLink.className = "ve-m-btn brand";
  starterPlayLink.href = siteUrl("./starter/");
  starterPlayLink.textContent = "Play Sunspire Beacon Trials";
  modeBtnRow.append(starterPlayLink);
  cModes.append(hModes, pModesDesc, modeBtnRow, modeLog);

  const cNpcs = document.createElement("div");
  cNpcs.className = "ve-m-card";
  const hNpcs = document.createElement("h3");
  hNpcs.textContent = `NPC Character Models, Guiding Directives & Q15 LIF Salience (${PRISM_TERRAIN_WALKABLE_POLYGONS.toLocaleString()}-Poly NavMesh)`;
  const pNpcsDesc = document.createElement("p");
  pNpcsDesc.textContent =
    "Inspect 22-joint IK character models, assign guiding behavioral directives over the A* walkable NavMesh, possess the player character, and monitor live 8-neuron mutual-attention salience.";

  const npcList = document.createElement("div");
  npcList.className = "ve-step-list";
  const npcLog = document.createElement("pre");
  npcLog.className = "ve-m-log";
  npcLog.id = "ve-npc-log";
  npcLog.textContent = "Select an NPC directive or click 'Frame NPC' to inspect live 22-joint IK & NavMesh behavior.";

  const domainRow = document.createElement("div");
  domainRow.className = "ve-m-row";
  const domainSelect = document.createElement("select");
  domainSelect.className = "ve-m-select";
  domainSelect.id = "ve-domain-pack-select";
  domainSelect.setAttribute("aria-label", "Imported catalog world pack");
  [
    "0 · Sunspire Harbor Regatta (Maritime)",
    "1 · Meridian Celestial Observatory (Astronomical)",
    "2 · Glasshouse Botanical Chronicle (Botanical)",
    "3 · Starfall Bastion Arena (Low-G Basalt)",
  ].forEach((label, idx) => {
    const opt = document.createElement("option");
    opt.value = String(idx);
    opt.textContent = label;
    domainSelect.append(opt);
  });
  const btnLoadDomain = document.createElement("button");
  btnLoadDomain.type = "button";
  btnLoadDomain.className = "ve-m-btn brand";
  btnLoadDomain.id = "ve-load-domain-btn";
  btnLoadDomain.textContent = "Load Catalog World Pack";
  btnLoadDomain.addEventListener("click", async () => {
    const studioCtx = getStudioCtx();
    const domainIdx = Number(domainSelect.value) || 0;
    const [res, vObs] = await Promise.all([
      studioCtx?.callWorker?.("load-domain-pack", { domainIdx }),
      studioCtx?.queryVerifier?.("scene:observer"),
    ]);
    if (res?.ok && res.observer) {
      const dom = res.observer.activeDomain || {};
      studioCtx?.onProjectChange?.(dom.title || "Sunspire Harbor Regatta");
      studioCtx?.onSetSimLive?.(true);
      npcLog.textContent =
        `Loaded Catalog World Pack "${dom.title}" (${dom.slug} · ${dom.schemaOrgType})\n` +
        `  Domain κ=${String(dom.kappa || "").slice(0, 24)}… · Observer κ=${String(res.observer.observerKappa || "").slice(0, 24)}…\n` +
        `  Seed=${res.seed} · Tris=${res.tris} · Walkable Polys=${res.walkablePolys} · Holo=${vObs || "conformant"}`;
    }
  });
  const btnInspectBus = document.createElement("button");
  btnInspectBus.type = "button";
  btnInspectBus.className = "ve-m-btn";
  btnInspectBus.id = "ve-observer-bus-btn";
  btnInspectBus.textContent = "Inspect Character Event Log";
  btnInspectBus.addEventListener("click", async () => {
    const studioCtx = getStudioCtx();
    const [res, vObs, vChan] = await Promise.all([
      studioCtx?.callWorker?.("observer-status"),
      studioCtx?.queryVerifier?.("scene:observer"),
      studioCtx?.queryVerifier?.("scene:channels"),
    ]);
    if (res?.ok && res.observer) {
      const obs = res.observer;
      const comms = (obs.recentCommunications || [])
        .map((c) => `  [${c.channel}] ${c.sender} -> ${c.recipient} (act=${c.act}, κ=${String(c.kappa || "").slice(0, 16)}…)`)
        .join("\n");
      npcLog.textContent =
        `Observer Holon κ=${obs.observerKappa}\n` +
        `  Organs=${(obs.organs || []).length} · Role Models=${obs.roleCount ?? 7} · Comms=${obs.communicationCount}\n` +
        `  Holo: ${vObs || "conformant"} | ${vChan || "conformant"}\n${comms}`;
    }
  });
  domainRow.append(domainSelect, btnLoadDomain, btnInspectBus);

  NPC_ROSTER.forEach((npc, npcIdx) => {
    const row = document.createElement("div");
    row.className = "ve-step-item";
    const copy = document.createElement("div");
    copy.className = "ve-step-copy";
    const t = document.createElement("span");
    t.className = "ve-step-title";
    t.textContent = `${npc.name} (${npc.archetype})`;
    const m = document.createElement("span");
    m.className = "ve-step-meta";
    m.textContent = npc.stats;
    copy.append(t, m);

    const controls = document.createElement("div");
    controls.className = "ve-m-row";
    const dirSelect = document.createElement("select");
    dirSelect.className = "ve-m-select";
    dirSelect.setAttribute("aria-label", `${npc.name} guiding directive`);
    npc.directives.forEach((d, dIdx) => {
      const opt = document.createElement("option");
      opt.value = String(dIdx);
      opt.textContent = d;
      dirSelect.append(opt);
    });
    dirSelect.addEventListener("change", async () => {
      const studioCtx = getStudioCtx();
      npc.activeDirective = Number(dirSelect.value) || 0;
      const [wRes, vNav] = await Promise.all([
        studioCtx?.callWorker?.("set-npc-directive", { npcIdx, directiveIdx: npc.activeDirective }),
        studioCtx?.queryVerifier?.("scene:navmesh"),
      ]);
      studioCtx?.onSetSimLive?.(true);
      const wpCount = wRes?.waypointCount ?? 0;
      const charId = wRes?.characterId || npc.id;
      const dirLabel = wRes?.directiveLabel || npc.directives[npc.activeDirective];
      npcLog.textContent =
        `${charId} Wasm Directive -> "${dirLabel}" (${wpCount} A* corridor waypoints)\n` +
        `  Anchor (${npc.coords.join(", ")}) m · NavMesh (${PRISM_TERRAIN_WALKABLE_POLYGONS} walkable polys) -> ${vNav || "conformant"}`;
    });

    const btnFrameNpc = document.createElement("button");
    btnFrameNpc.type = "button";
    btnFrameNpc.className = "ve-m-btn primary";
    btnFrameNpc.dataset.npcFrame = npc.id;
    btnFrameNpc.textContent = "Frame NPC";
    btnFrameNpc.addEventListener("click", async () => {
      await getStudioCtx()?.onFocusPreset?.(npc.nodeIdx);
      npcLog.textContent =
        `Framed ${npc.name} · Active Directive: "${npc.directives[npc.activeDirective]}"\n` +
        `  ${npc.stats}`;
    });

    controls.append(dirSelect, btnFrameNpc);
    row.append(copy, controls);
    npcList.append(row);
  });

  const dlgRow = document.createElement("div");
  dlgRow.className = "ve-m-row";
  const dlgNpcSelect = document.createElement("select");
  dlgNpcSelect.className = "ve-m-select";
  dlgNpcSelect.id = "ve-npc-dialogue-target";
  dlgNpcSelect.setAttribute("aria-label", "NPC dialogue recipient");
  ["NPC 0 · Primary Guide", "NPC 1 · Specialist Engineer", "NPC 2 · Domain Keeper"].forEach((lbl, idx) => {
    const opt = document.createElement("option");
    opt.value = String(idx);
    opt.textContent = lbl;
    dlgNpcSelect.append(opt);
  });
  const dlgInput = document.createElement("input");
  dlgInput.type = "text";
  dlgInput.className = "ve-m-input";
  dlgInput.id = "ve-npc-dialogue-input";
  dlgInput.value = "What beacon or instrument governs this domain?";
  dlgInput.placeholder = "Ask grounded question or issue directive (ISO 24617-2)…";
  dlgInput.setAttribute("aria-label", "Grounded NPC utterance");
  const btnAskNpc = document.createElement("button");
  btnAskNpc.type = "button";
  btnAskNpc.className = "ve-m-btn primary";
  btnAskNpc.id = "ve-npc-dialogue-btn";
  btnAskNpc.textContent = "Converse (ISO 24617-2)";
  btnAskNpc.addEventListener("click", async () => {
    const studioCtx = getStudioCtx();
    const npcIdx = Number(dlgNpcSelect.value) || 0;
    const utterance = dlgInput.value.trim();
    const [reply, vLang] = await Promise.all([
      studioCtx?.callWorker?.("npc-dialogue", { npcIdx, utterance }),
      studioCtx?.queryVerifier?.("scene:language"),
    ]);
    if (!reply?.ok) {
      npcLog.textContent = reply?.organDenied
        ? `Refused: ContainerOrganDenied(${reply.organDenied})`
        : "Refused empty or invalid dialogue turn.";
      return;
    }
    const t = reply.turn;
    npcLog.textContent =
      `[${t.responseActId} · ${t.detectedLanguage} · evidence=${t.matchedEvidenceFacts} · abstained=${t.abstainedUnknown}]\n` +
      `  ${t.characterName} (${t.characterId}): "${t.replyText}"\n` +
      `  Evidence Facts=[${(t.citedFactIds || []).join(", ")}] · Comm κ=${String(t.commKappa || "").slice(0, 24)}… · ${vLang || "conformant"}`;
  });
  dlgRow.append(dlgNpcSelect, dlgInput, btnAskNpc);

  const neuroCanvas = document.createElement("canvas");
  neuroCanvas.className = "ve-m-canvas";
  neuroCanvas.width = 580;
  neuroCanvas.height = 250;
  setObserverDrawHook((tel) => drawObserverCanvas(neuroCanvas, tel));

  const mpRow = document.createElement("div");
  mpRow.className = "ve-m-row";
  const btnPossessPlayer = document.createElement("button");
  btnPossessPlayer.type = "button";
  btnPossessPlayer.className = "ve-m-btn primary";
  btnPossessPlayer.textContent = "Possess Character (WASD + Foot IK)";
  btnPossessPlayer.addEventListener("click", async () => {
    const studioCtx = getStudioCtx();
    await studioCtx?.onSetCameraMode?.("third");
    studioCtx?.onSetSimLive?.(true);
  });

  const btnOpenStitch = document.createElement("button");
  btnOpenStitch.type = "button";
  btnOpenStitch.className = "ve-m-btn brand";
  btnOpenStitch.textContent = "Multiplayer & 2-of-3 Stitch Custody";
  btnOpenStitch.addEventListener("click", () => getStudioCtx()?.onOpenStitchMesh?.());

  const btnShareSnap = document.createElement("button");
  btnShareSnap.type = "button";
  btnShareSnap.className = "ve-m-btn";
  btnShareSnap.textContent = "Copy #uorc= Snapshot URL";
  btnShareSnap.addEventListener("click", () => getStudioCtx()?.onShareSnapshot?.());

  mpRow.append(btnPossessPlayer, btnOpenStitch, btnShareSnap);
  cNpcs.append(hNpcs, pNpcsDesc, domainRow, npcList, dlgRow, neuroCanvas, mpRow, npcLog);
  pGameplay.append(cModes, cNpcs);

  return { pGameplay, neuroCanvas, renderGameMode };
}
