// Verdant Studio Live External Stream & Project Model Stage Applier
// (`site/studio/stage-assets.mjs`), extracted from `site/studio/studio-page.mjs`.

export function initStudioStageAssets(ctx) {
  const {
    statusEl,
    hudProjectEl,
    hudMetricsEl,
    loadedFontUrls,
    ACOUST,
    callWorker,
    chip,
    getLastCognition,
    getSimLive,
    setSimLive,
    getTris,
    setTris,
    setCameraState,
    setLastRt60Ms,
    getDomainPacks,
    syncFallbackBuffers,
    syncHud,
    requestRender,
    refreshObserverCognition,
    renderObserverCognition,
    loadCatalogDomainPack,
  } = ctx;

  async function loadRemoteGlbIntoStage(glbUrl, label) {
    if (!glbUrl) return false;
    chip(statusEl, `Streaming live .glb (${label}) -> Verdant Wasm HD texture + UORC transcoder…`);
    try {
      const res = await fetch(glbUrl, { signal: AbortSignal.timeout(14000) });
      if (!res.ok) throw new Error(`HTTP ${res.status}`);
      const bytes = new Uint8Array(await res.arrayBuffer());
      const maxBytes = getLastCognition()?.hostEnvelope?.recommendedGlbMaxBytes || 25165824;
      if (bytes.byteLength > maxBytes) {
        chip(statusEl, `Host budget guard: "${label}" (${Math.round(bytes.byteLength / 1048576)}MB) exceeds host cap (${Math.round(maxBytes / 1048576)}MB)`);
        return false;
      }
      const r = await callWorker("import-glb", { bytes }, [bytes.buffer]);
      if (r?.ok) {
        const tris = r.tris || getTris();
        setTris(tris);
        if (r.camera) setCameraState(r.camera);
        await syncFallbackBuffers();
        const ch = r.pbrChannels || {};
        const activeMaps = [
          ch.albedo ? (ch.atlasTileCount > 1 ? `${ch.atlasTileCount}-Tile Atlas` : "Albedo") : null,
          ch.metallicRoughness ? "MR" : null,
          ch.normal ? "Normal" : null,
          ch.emissive ? "Emissive" : null,
        ].filter(Boolean);
        const mapLabel = activeMaps.length ? activeMaps.join("+") : "Vertex PBR";
        const animLabel = ch.hasAnimation ? " · Live LBS Rig" : "";
        const mt = r.meshletTelemetry;
        const cullLabel = mt?.totalMeshlets
          ? ` · ${mt.visibleMeshlets}/${mt.totalMeshlets} meshlets (${(mt.cullRatioPermille / 10).toFixed(0)}% culled)`
          : "";
        if (ch.hasAnimation && !getSimLive()) setSimLive(true);
        if (hudProjectEl) hudProjectEl.textContent = `HD Stream Stage · ${label}`;
        if (hudMetricsEl) hudMetricsEl.textContent = `glTF 2.0 (${mapLabel}${animLabel}) · ${tris.toLocaleString()} tris`;
        chip(statusEl, `Mounted live Khronos .glb "${label}" (${tris.toLocaleString()} tris · ${mapLabel}${animLabel}${cullLabel})`);
        refreshObserverCognition();
        requestRender();
        return true;
      }
      chip(statusEl, `Stage transcoder declined ${label} (unsupported extension or size)`);
      return false;
    } catch (err) {
      chip(statusEl, `Live .glb stream offline (${label}): ${err?.message || "network unavailable"}`);
      return false;
    }
  }

  async function composeRemoteGlbIntoStage(glbUrl, label) {
    if (!glbUrl) return false;
    chip(statusEl, `Composing "${label}" beside active stage geometry…`);
    try {
      const res = await fetch(glbUrl, { signal: AbortSignal.timeout(14000) });
      if (!res.ok) throw new Error(`HTTP ${res.status}`);
      const bytes = new Uint8Array(await res.arrayBuffer());
      const r = await callWorker("compose-glb-append", { bytes }, [bytes.buffer]);
      if (r?.ok) {
        const tris = r.tris || getTris();
        setTris(tris);
        if (r.camera) setCameraState(r.camera);
        if (hudMetricsEl) hudMetricsEl.textContent = `Composed Multi-Asset Stage · ${tris.toLocaleString()} tris`;
        chip(statusEl, `Composed "${label}" onto live stage (${tris.toLocaleString()} total tris)`);
        refreshObserverCognition();
        requestRender();
        return true;
      }
      return false;
    } catch (err) {
      chip(statusEl, `Compose stream offline (${label}): ${err?.message || "network unavailable"}`);
      return false;
    }
  }

  async function applyRemoteEnvSky(skyUrl, label) {
    if (!skyUrl) {
      await callWorker("set-env-sky", { bitmap: null });
      chip(statusEl, "Reset WebGL2 sky dome to procedural atmosphere");
      requestRender();
      return true;
    }
    chip(statusEl, `Streaming Poly Haven panoramic sky dome (${label}) -> WebGL2…`);
    try {
      const res = await fetch(skyUrl, { signal: AbortSignal.timeout(12000) });
      if (!res.ok) throw new Error(`HTTP ${res.status}`);
      const blob = await res.blob();
      const bitmap = await createImageBitmap(blob);
      const r = await callWorker("set-env-sky", { bitmap, label }, [bitmap]);
      if (r?.cognition) renderObserverCognition(r.cognition);
      chip(statusEl, `Applied live Poly Haven HDRI sky dome "${label}"`);
      requestRender();
      return true;
    } catch (err) {
      chip(statusEl, `Sky dome stream offline (${label}): ${err?.message || "network unavailable"}`);
      return false;
    }
  }

  function ensureRemoteFontLoaded(stylesheetUrl, fontFamily) {
    if (stylesheetUrl && !loadedFontUrls.has(stylesheetUrl)) {
      const link = document.createElement("link");
      link.rel = "stylesheet";
      link.href = stylesheetUrl;
      document.head.append(link);
      loadedFontUrls.add(stylesheetUrl);
    }
    if (fontFamily) {
      document.documentElement.style.setProperty("--font-display", `"${fontFamily}", var(--font-sans)`);
    }
  }

  async function applyProjectModelLive(model) {
    if (!model) return;
    let props = {};
    try {
      props = JSON.parse(model.propertiesJson || "{}");
    } catch {
      props = {};
    }
    if (props.glbStreamUrl) {
      await loadRemoteGlbIntoStage(props.glbStreamUrl, model.title);
      return;
    }
    if (props.envSkyUrl) {
      await applyRemoteEnvSky(props.envSkyUrl, model.title);
      return;
    }
    if (props.domainSlug) {
      const idx = getDomainPacks().findIndex((d) => d.slug === props.domainSlug);
      if (idx >= 0) {
        await loadCatalogDomainPack(idx, { userInitiated: true });
        return;
      }
    }
    if (model.targetSlot === "design:theme" && typeof props.themeClass === "string") {
      document.documentElement.className = props.themeClass;
      chip(statusEl, `Applied live W3C DTCG theme "${model.title}" (v${model.activeVersion})`);
      return;
    }
    if (model.targetSlot === "design:typography" && (props.stylesheetUrl || props.fontFamily)) {
      ensureRemoteFontLoaded(props.stylesheetUrl, props.fontFamily);
      chip(statusEl, `Applied live typography "${props.fontFamily || model.title}" (v${model.activeVersion})`);
      return;
    }
    if (model.targetSlot === "audio:soundscape" && props.volumeM3 && props.absorptionMilliM2) {
      const ac = await callWorker("sabine-rt60", {
        volumeM3: Number(props.volumeM3),
        absorptionMilliM2: Number(props.absorptionMilliM2),
      });
      if (ac?.ok) {
        setLastRt60Ms(ac.rt60Ms);
        ACOUST.rt60Ms = ac.rt60Ms;
        syncHud();
        chip(statusEl, `Applied Sabine acoustic room "${model.title}" -> RT60 ${ac.rt60Ms}ms`);
      }
      return;
    }
    if (model.targetSlot === "narrative:ontology" && props.groundedFact) {
      await callWorker("observe-comm", {
        channelCode: 0,
        sender: "urn:verdant:organ:catalogs",
        recipient: "urn:verdant:organ:studio",
        message: `ontology:${model.modelId}:${props.groundedFact.slice(0, 80)}`,
      });
      chip(statusEl, `Bound ontology "${model.title}" into Observer SOSA/PROV-O graph (v${model.activeVersion})`);
      return;
    }
    chip(statusEl, `Active model "${model.title}" bound to ${model.targetSlot} (v${model.activeVersion} · kappa ${model.currentKappaHex.slice(0, 10)}…)`);
  }

  return {
    loadRemoteGlbIntoStage,
    composeRemoteGlbIntoStage,
    applyRemoteEnvSky,
    ensureRemoteFontLoaded,
    applyProjectModelLive,
  };
}

export function initStudioDomainWizard(ctx) {
  const {
    manDomainEl,
    wizNpcSelect,
    wizDirectivesRow,
    wizComposeLog,
    wizPromptChips,
    wizDlgLog,
    wizDomainLog,
    wizDomainGrid,
    stageModeDomainBtn,
    stitchAppTitleInput,
    stitchAppGenreSelect,
    statusEl,
    wfState,
    ACOUST,
    STARTER_ARCHETYPES,
    DOMAIN_DEFAULT_GENRE,
    callWorker,
    chip,
    currentLodSpec,
    syncTemplatePills,
    syncFallbackBuffers,
    syncHud,
    requestRender,
    getDomainPacks,
    getActiveDomainIdx,
    setActiveDomainIdx,
    getActiveProject,
    setActiveProject,
    setUserSelectedDomain,
    setActiveSkyFog,
    getTris,
    setTris,
    setCameraState,
    getLastRt60Ms,
    setLastRt60Ms,
    getSimClockSec,
    getExperienceDesign,
    getStudioHandle,
  } = ctx;

  let domainLoadSeq = 0;

  function setWizardStep(step) {
    const activeStep = Math.max(1, Math.min(4, step | 0));
    document.querySelectorAll("[data-wiz-step]").forEach((btn) => {
      btn.setAttribute("aria-selected", Number(btn.dataset.wizStep) === activeStep ? "true" : "false");
    });
    document.querySelectorAll("[data-wiz-panel]").forEach((panel) => {
      panel.hidden = Number(panel.dataset.wizPanel) !== activeStep;
    });
  }

  async function runNpcDialogue(utterance) {
    const clean = String(utterance || "").trim();
    if (!clean) return;
    const npcIdx = Number(wizNpcSelect?.value) || 0;
    const r = await callWorker("npc-dialogue", { npcIdx, utterance: clean, simClockSec: getSimClockSec() });
    if (!r?.ok || !r.turn) {
      if (wizDlgLog) wizDlgLog.textContent = `Dialogue unavailable (${r?.organDenied || "wasm busy"}).`;
      return;
    }
    const t = r.turn;
    const stateLabel = t.abstainedUnknown ? "ABSTAINED (Unknown — Zero Hallucination)" : `GROUNDED (${t.matchedEvidenceFacts} catalog fact(s))`;
    if (wizDlgLog) {
      wizDlgLog.textContent =
        `ISO 24617-2 Act: ${t.act} · ${stateLabel} · Mutated: ${Boolean(t.worldMutated)}\n` +
        `${t.characterName || t.characterId}: "${t.reply}"\n` +
        `SOSA/PROV-O kappa: ${t.envelopeKappaHex || "verified"}`;
    }
    chip(statusEl, `ISO 24617-2 · ${t.act} · ${stateLabel}`);
  }

  function renderNpcDirectivesAndPrompts(npcIdx) {
    const pack = getDomainPacks()[getActiveDomainIdx()];
    const ch = pack?.characters?.[npcIdx] || pack?.characters?.[0];
    if (!ch) return;

    if (wizDirectivesRow) {
      wizDirectivesRow.replaceChildren();
      (ch.directives || []).forEach((dirLabel, dirIdx) => {
        const btn = document.createElement("button");
        btn.type = "button";
        btn.className = "st-btn sm";
        btn.textContent = dirLabel;
        btn.addEventListener("click", async () => {
          const r = await callWorker("set-npc-directive", { npcIdx, directiveIdx: dirIdx, simClockSec: getSimClockSec() });
          if (r?.ok) {
            setTris(r.tris || getTris());
            await syncFallbackBuffers();
            syncHud();
            if (wizComposeLog) {
              wizComposeLog.textContent = `NPC Directive -> ${ch.name}: "${r.directiveLabel}" · ${r.waypointCount} NavMesh waypoints`;
            }
            requestRender();
          }
        });
        wizDirectivesRow.append(btn);
      });
    }

    if (wizPromptChips) {
      wizPromptChips.replaceChildren();
      (ch.knowledgeFacts || []).slice(0, 2).forEach((fact, idx) => {
        const words = fact.split(/\s+/).slice(0, 5).join(" ");
        const b = document.createElement("button");
        b.type = "button";
        b.className = "st-btn sm";
        b.textContent = `Fact ${idx + 1}: ${words}…`;
        b.addEventListener("click", () => runNpcDialogue(fact));
        wizPromptChips.append(b);
      });
    }
  }

  function renderDomainControlsForActivePack() {
    const idx = getActiveDomainIdx();
    const pack = getDomainPacks()[idx];
    if (!pack) return;
    if (manDomainEl) manDomainEl.textContent = pack.id;
    document.querySelectorAll("[data-wiz-domain-idx]").forEach((card) => {
      card.setAttribute("aria-pressed", Number(card.dataset.wizDomainIdx) === idx ? "true" : "false");
    });
    if (wizNpcSelect) {
      wizNpcSelect.replaceChildren();
      (pack.characters || []).forEach((ch, i) => {
        const opt = document.createElement("option");
        opt.value = String(i);
        opt.textContent = `${ch.name} · ${ch.role} (${ch.rigId})`;
        wizNpcSelect.append(opt);
      });
    }
    renderNpcDirectivesAndPrompts(0);
  }

  async function loadCatalogDomainPack(domainIdx, { fromSelect = false, userInitiated = false } = {}) {
    if (userInitiated || fromSelect) setUserSelectedDomain(true);
    wfState.activeStageAsset = null;
    stageModeDomainBtn?.setAttribute("aria-pressed", "true");
    document.querySelectorAll("[data-stage-glb-id]").forEach((b) => b.setAttribute("aria-pressed", "false"));
    const seq = ++domainLoadSeq;
    const clamped = Math.max(0, Math.min(3, domainIdx | 0));
    setActiveDomainIdx(clamped);
    const pack = getDomainPacks()[clamped];
    const arch = STARTER_ARCHETYPES[clamped] || STARTER_ARCHETYPES[0];
    setActiveProject(pack?.title || arch.title);
    if (pack?.slug) syncTemplatePills(pack.slug);

    if (pack?.environment?.skyRgbPercent && pack?.environment?.fogRgbPercent) {
      setActiveSkyFog(
        pack.environment.skyRgbPercent.map((v) => v / 100),
        pack.environment.fogRgbPercent.map((v) => v / 100),
      );
    }
    if (pack) {
      if (stitchAppTitleInput && !stitchAppTitleInput.value) stitchAppTitleInput.placeholder = pack.title;
      if (stitchAppGenreSelect && DOMAIN_DEFAULT_GENRE[pack.slug]) {
        stitchAppGenreSelect.value = DOMAIN_DEFAULT_GENRE[pack.slug];
      }
    }
    renderDomainControlsForActivePack();

    const r = await callWorker("load-domain-pack", { domainIdx: clamped, lod: currentLodSpec(), simClockSec: getSimClockSec() });
    if (seq !== domainLoadSeq) return r;
    if (r?.ok) {
      setTris(r.tris || getTris());
      if (r.camera) setCameraState(r.camera);
      if (pack?.environment?.acousticVolumeM3 && pack?.environment?.acousticAbsorptionMilliM2) {
        const ac = await callWorker("sabine-rt60", {
          volumeM3: pack.environment.acousticVolumeM3,
          absorptionMilliM2: pack.environment.acousticAbsorptionMilliM2,
        });
        if (ac?.ok) {
          setLastRt60Ms(ac.rt60Ms);
          ACOUST.rt60Ms = ac.rt60Ms;
        }
      }
      await syncFallbackBuffers();
    }

    renderDomainControlsForActivePack();
    const studioHandle = getStudioHandle();
    if (!fromSelect) studioHandle?.setProjectIndex?.(clamped);
    studioHandle?.refresh?.();
    syncHud();

    const liveTris = getTris();
    if (wizDomainLog && pack) {
      const models = (pack.models || []).map((m) => `${m.name} (${m.materialId})`).join(", ");
      wizDomainLog.textContent =
        `Loaded Catalog Pack: ${pack.id} · ${pack.title} (Seed ${r?.seed ?? pack.seed})\n` +
        `Environment: ${pack.environment?.name} · ${liveTris.toLocaleString()} tris · RT60 ${getLastRt60Ms()}ms\n` +
        `HD Models: ${models}`;
    }
    chip(statusEl, `Loaded ${getActiveProject()} · ${liveTris.toLocaleString()} tris`);
    requestRender();
    return r;
  }

  function renderDomainPackCards() {
    if (!wizDomainGrid) return;
    wizDomainGrid.replaceChildren();
    const exp = getExperienceDesign();
    const activeIdx = getActiveDomainIdx();
    getDomainPacks().forEach((pack, idx) => {
      const btn = document.createElement("button");
      btn.type = "button";
      btn.className = "proj-domain-btn";
      btn.dataset.wizDomainIdx = String(idx);
      btn.dataset.wizDomainSlug = pack.slug;
      btn.setAttribute("aria-pressed", idx === activeIdx ? "true" : "false");
      const worldCopy = exp?.plainLanguage?.worldCards?.[pack.slug];
      if (worldCopy?.badge) {
        const badgeEl = document.createElement("span");
        badgeEl.className = "proj-domain-badge";
        badgeEl.textContent = worldCopy.badge;
        btn.append(badgeEl);
      }
      const titleEl = document.createElement("strong");
      titleEl.textContent = pack.title;
      const subEl = document.createElement("small");
      subEl.textContent = worldCopy?.blurb || `${pack.environment?.name || "3D World"}`;
      btn.append(titleEl, subEl);
      btn.addEventListener("click", () => loadCatalogDomainPack(idx, { userInitiated: true }));
      wizDomainGrid.append(btn);
    });
  }

  return {
    setWizardStep,
    renderDomainControlsForActivePack,
    renderNpcDirectivesAndPrompts,
    runNpcDialogue,
    loadCatalogDomainPack,
    renderDomainPackCards,
  };
}
