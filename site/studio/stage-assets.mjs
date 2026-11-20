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
