// Verdant Studio Workspace Boot, Offscreen/Fallback Renderer Init, Deep-Link Hydration & Panel Wiring
// Extracted from `site/studio/studio-page.mjs` to keep shipped JS modules modular (<= 1250 L).

import { hydrateStitchAppCatalog } from "../catalog-streams.mjs";
import { initStudioVcsPanel } from "./vcs-panel.mjs";
import { initStudioAuthorityPanel } from "./authority-panel.mjs";

export async function bootStudioWorkspace(ctx) {
  const {
    $,
    LODS,
    STUDIO_ROLE_KEYS,
    ACOUST,
    canvas,
    worker,
    inlineHost,
    glbFileIn,
    statusEl,
    scaleBadgeEl,
    audioBtn,
    catalogBanner,
    catalogBannerText,
    stitchAppTitleInput,
    stitchAppGenreSelect,
    discStreamSelect,
    discSearchInput,
    stageModeDomainBtn,
    wfState,
    pxDims,
    callWorker,
    queryVerifier,
    currentLodSpec,
    syncFallbackBuffers,
    loadStreamPrefs,
    refreshObserverCognition,
    mountInlineStudio,
    exportGlbDownload,
    shareUorcSnapshot,
    toggleRenderBackend,
    streamLod,
    cycleCameraMode,
    setSimLive,
    toggleGuardedAudio,
    focusPoint,
    focusPreset,
    setWorldSeed,
    loadCatalogDomainPack,
    loadBlankStudioStage,
    loadRemoteGlbIntoStage,
    applyRemoteEnvSky,
    openStitchGuarded,
    executePrompt,
    isOrganAllowed,
    refuseOrgan,
    suspendModalAudio,
    setActiveContentType,
    runUnifiedDiscovery,
    refreshProjectModelsFromWasm,
    syncWorkflowRibbon,
    setRightRailTab,
    base64UrlToBytes,
    hydrateExperienceDesignSystem,
    clearStudioObjects,
    renderStudioOutliner,
    syncHud,
    chip,
    requestRender,
    state,
  } = ctx;

  const { w, h } = pxDims();
  let hostMaxTex = 2048;
  if (worker && canvas && typeof canvas.transferControlToOffscreen === "function") {
    try {
      const oc = canvas.transferControlToOffscreen();
      const r = await callWorker(
        "init",
        { canvas: oc, width: w, height: h, lod: currentLodSpec(), hdRenderer: true, autoScale: true },
        [oc],
      );
      if (r?.kind === "ready") {
        state.setOffOk(true);
        state.setActiveRenderBackend(r.backend || "webgl2");
        if (r.tris) state.setLTris(r.tris);
        hostMaxTex = r.maxTextureSize || 2048;
        if (r.audioSpec) Object.assign(ACOUST, r.audioSpec);
      }
    } catch {
      state.setOffOk(false);
    }
  }
  if (!state.getOffOk() && canvas) {
    const fbM = await import("../scene-hd-renderer.mjs");
    const fbR = await fbM.createBestRenderer(canvas);
    state.setFallbackRenderer(fbM, fbR);
    hostMaxTex = fbR?.maxTextureSize || 2048;
    if (worker) {
      const r = await callWorker("init", { canvas: null, width: w, height: h, lod: currentLodSpec(), hdRenderer: true });
      if (r?.tris) state.setLTris(r.tris);
      await syncFallbackBuffers();
    }
  }

  // Hydrate persisted stream prefs into ObserverPrismHolon and report real host GPU envelope:
  const savedPrefs = loadStreamPrefs();
  for (const cs of savedPrefs.custom) {
    if (cs?.streamId && cs?.endpointUrl) {
      await callWorker("register-stream", {
        request: {
          streamId: cs.streamId,
          label: cs.label || cs.streamId,
          authority: cs.authority || "Observer Custom Stream",
          adapterKind: cs.adapterKind || "schema-org-catalog",
          endpointUrl: cs.endpointUrl,
          targetParts: cs.targetParts || ["hero-props"],
        },
      });
    }
  }
  for (const disId of savedPrefs.disabled) {
    await callWorker("toggle-stream", { streamId: disId, enabled: false });
  }
  await refreshObserverCognition({
    maxTex: hostMaxTex,
    threads: navigator.hardwareConcurrency || 4,
    frameMicros: 8000,
    viewportWidth: Math.round((window.screen?.width || 1920) * (window.devicePixelRatio || 1)),
    viewportHeight: Math.round((window.screen?.height || 1080) * (window.devicePixelRatio || 1)),
  });

  if (inlineHost) {
    const handle = mountInlineStudio(inlineHost, {
      initialTab: state.getActiveRoleKey(),
      callWorker,
      queryVerifier,
      onPickGlb: () => glbFileIn?.click(),
      onResetGlb: async () => {
        const r = await callWorker("reset-glb");
        if (r?.tris) state.setLTris(r.tris);
        await syncFallbackBuffers();
        syncHud();
        requestRender();
      },
      onExportGlb: () => exportGlbDownload(callWorker, LODS[state.getLodIdx()].short),
      onShareSnapshot: () => shareUorcSnapshot(callWorker, chip, statusEl),
      onToggleRenderMode: toggleRenderBackend,
      onSetLod: streamLod,
      onSetCameraMode: cycleCameraMode,
      onSetSimLive: setSimLive,
      onToggleAudio: toggleGuardedAudio,
      onFocusPoint: focusPoint,
      onFocusPreset: focusPreset,
      onSetSeed: setWorldSeed,
      onLoadDomainPack: loadCatalogDomainPack,
      onOpenStitchMesh: openStitchGuarded,
      onRoutePrompt: executePrompt,
      onInteractStory: async () => callWorker("story-interact"),
      onMountOpfsUorc: async (name, bytes) => {
        if (!isOrganAllowed(4, true)) {
          refuseOrgan(4);
          return false;
        }
        const r = await callWorker("stream-uorc", {
          targetRank: 3,
          customStream: bytes,
          lod: currentLodSpec(),
          simClockSec: state.getSimClockSec(),
        });
        if (r?.ok) {
          if (r.tris) state.setLTris(r.tris);
          await syncFallbackBuffers();
          syncHud();
          chip(statusEl, `Mounted OPFS ${name} · ${state.getLTris()} tris`);
          requestRender();
          return true;
        }
        return false;
      },
      onRoleChange: (rk) => {
        state.setActiveRoleKey(rk);
        syncHud();
      },
      onProjectChange: (projTitle) => {
        if (projTitle) {
          state.setActiveProject(projTitle);
          syncHud();
        }
      },
      getActiveProject: () => state.getActiveProject(),
      getAcousticSpec: () => ACOUST,
      isOrganAllowed,
      getOrganMode: (idx) => state.getOrganMode(idx),
      onOrganGrantChange: (idx, nextMode) => {
        state.setOrganMode(idx, nextMode);
        if (idx === 3 && nextMode === 2) suspendModalAudio(audioBtn);
      },
      onRefuseOrgan: refuseOrgan,
      getRenderMode: () => state.getRenderMode(),
      getLodRank: () => state.getLodIdx(),
    });
    state.setStudioHandle(handle);
  }

  // Deep-links from /catalogs/: ?domain=<slug>&catalog=<STD-ID>&stream=<id>&q=<term>&type=<ct>&autoGlb=<url>&autoTitle=<title>
  const params = new URLSearchParams(location.search);
  const reqDomain = params.get("domain");
  const reqCatalog = params.get("catalog");
  const reqStream = params.get("stream");
  const reqQuery = params.get("q");
  const reqType = params.get("type");
  const reqAutoGlb = params.get("autoGlb");
  const reqAutoSky = params.get("autoSky");
  const reqAutoTitle = params.get("autoTitle") || "Imported Stream Model";
  if (reqCatalog && catalogBanner && catalogBannerText) {
    catalogBanner.dataset.active = "true";
    catalogBannerText.textContent = `Imported Governing Catalog Standard: ${reqCatalog} — live-verified against Prism app.holo`;
  }
  const reqAppId = params.get("app");
  if (reqAppId) {
    const allApps = await hydrateStitchAppCatalog();
    const foundApp = allApps.find((a) => a.appId === reqAppId);
    if (foundApp) {
      if (stitchAppTitleInput) stitchAppTitleInput.value = foundApp.title;
      if (stitchAppGenreSelect) stitchAppGenreSelect.value = foundApp.genre;
      if (catalogBanner && catalogBannerText) {
        catalogBanner.dataset.active = "true";
        catalogBannerText.textContent = `Loaded Stitch App: ${foundApp.title} (${foundApp.genreLabel}) · .uorc ${foundApp.uorcBytes} B · 2-of-3 STCH verified`;
      }
    }
  }
  if (reqStream && discStreamSelect) discStreamSelect.value = reqStream;
  if (reqQuery && discSearchInput) discSearchInput.value = reqQuery;
  if (reqType) setActiveContentType(reqType);
  if (reqStream || reqQuery || reqType) {
    runUnifiedDiscovery();
  }
  const domainPacks = state.getDomainPacks();
  if (reqDomain && domainPacks.length > 0) {
    const initialDomainIdx = Math.max(
      0,
      domainPacks.findIndex((d) => d.slug === reqDomain || d.id === reqDomain),
    );
    await loadCatalogDomainPack(initialDomainIdx);
  } else if (!state.getUserSelectedDomain()) {
    await loadBlankStudioStage();
  }
  if (reqAutoGlb && /^https:\/\//.test(reqAutoGlb)) {
    wfState.activeStageAsset = reqAutoTitle;
    stageModeDomainBtn?.setAttribute("aria-pressed", "false");
    await loadRemoteGlbIntoStage(reqAutoGlb, reqAutoTitle);
  }
  if (reqAutoSky && /^https:\/\//.test(reqAutoSky)) {
    await applyRemoteEnvSky(reqAutoSky, reqAutoTitle);
  }
  await refreshProjectModelsFromWasm();
  syncWorkflowRibbon();

  if (location.hash === "#compose") {
    setRightRailTab("compose");
  } else if (STUDIO_ROLE_KEYS.includes(location.hash.slice(1))) {
    setRightRailTab("roles");
  }

  const snapMatch = /^#uorc=(.+)$/.exec(location.hash);
  if (snapMatch) {
    const bytes = base64UrlToBytes(snapMatch[1].trim());
    if (bytes?.byteLength) {
      const r = await callWorker("restore-snapshot", { bytes });
      if (r?.ok) await streamLod(r.lodRank ?? 3);
    }
  }

  await hydrateExperienceDesignSystem();
  syncHud();

  // Zero-account in-browser VerdantVcs, portable .holo export, ?pin=stitch:// deep-link sharing (VE-49)
  // live PagedTensorArena dynamic UHD/AI workload governor bridge (VE-50),
  // and GitHub-free Sovereign Signing-Request & Admin Quorum governance (VE-70 / VE-71):
  const vcsApi = initStudioVcsPanel({
    $,
    canvas,
    statusEl,
    scaleBadgeEl,
    stitchAppTitleInput,
    callWorker,
    chip,
    getActiveProject: () => state.getActiveProject(),
    setActiveProject: (v) => state.setActiveProject(v),
    getActiveDomainSlug: () => state.getDomainPacks()[state.getActiveDomainIdx()]?.slug || "sunspire-regatta",
    getStudioObjects: () => state.getStudioObjects(),
    setStudioObjects: (v) => state.setStudioObjects(v),
    getLodIdx: () => state.getLodIdx(),
    getSimClockSec: () => state.getSimClockSec(),
    currentLodSpec,
    clearStudioObjects,
    syncFallbackBuffers,
    renderStudioOutliner,
    syncHud,
    requestRender,
  });

  const authorityApi = initStudioAuthorityPanel({
    $,
    statusEl,
    chip,
    callWorker,
    getLocalPrismId: vcsApi.getLocalPrismId,
    getLatestCommitKappa: () => vcsApi.getVcsHistory()[0]?.commitKappa || null,
  });

  window.__VERDANT_STUDIO_API__ = {
    ...vcsApi,
    authority: authorityApi,
  };

  chip(
    statusEl,
    state.getExperienceDesign()?.plainLanguage?.readyBanner ||
      `Ready · ${state.getActiveProject()} — pick a world, browse 3D assets below, or click Play Preview.`,
  );
  requestRender();
}
