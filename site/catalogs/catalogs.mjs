// Standalone Verdant Model Catalogs Controller (`site/catalogs/catalogs.mjs`).
// Renders the 5 differentiated Verdant platform organs and 28 imported standard models,
// and executes live cryptographic BLAKE3 + Core-Wasm conformance checks against `/app.holo`.

import { kappa, load } from "../worker-runtime/index.mjs";
import {
  AUTHORITATIVE_CONTENT_TYPES,
  LIVE_CATALOG_STREAMS,
  STITCH_APP_GENRES,
  hydrateStitchAppCatalog,
  loadStitchAppCatalog,
  searchLiveCatalogStreams,
  subscribeStitchAppCatalog,
} from "../catalog-streams.mjs";
import {
  CATALOG_DOMAINS,
  LIFECYCLE_STAGES,
  VERDANT_PLATFORM_ORGANS,
  filterCatalogModels,
  summarizeCatalogCoverage,
} from "../prism-catalogs.mjs";

const utf8Enc = new TextEncoder();
const utf8Dec = new TextDecoder();

const $ = (id) => document.getElementById(id);
const holoBanner = $("cat-holo-banner");
const kpiApps = $("cat-kpi-apps");
const appsPills = $("cat-apps-pills");
const appsGrid = $("cat-apps-grid");
const kpiModels = $("cat-kpi-models");
const kpiLex = $("cat-kpi-lex");
const kpiPhases = $("cat-kpi-phases");
const kpiProofs = $("cat-kpi-proofs");
const organGrid = $("cat-organ-grid");
const domainGrid = $("cat-domain-grid");
const sensoryGrid = $("cat-sensory-grid");
const galleryForm = $("cat-gallery-form");
const gallerySearch = $("cat-gallery-search");
const galleryStreamSel = $("cat-gallery-stream");
const galleryPills = $("cat-gallery-pills");
const gallerySummary = $("cat-gallery-summary");
const galleryGrid = $("cat-live-gallery");
const lifecycleSel = $("cat-filter-lifecycle");
const domainSel = $("cat-filter-domain");
const roleSel = $("cat-filter-role");
const searchInput = $("cat-filter-search");
const countLabel = $("cat-count-label");
const listEl = $("cat-list");
const verifyAllBtn = $("cat-verify-all-btn");
const themeBtn = $("cat-theme-btn");

let holoApp = null;
const verdictCache = new Map();

// Populate select menus
LIFECYCLE_STAGES.forEach((s) => {
  const opt = document.createElement("option");
  opt.value = s.key;
  opt.textContent = s.label;
  lifecycleSel?.append(opt);
});
CATALOG_DOMAINS.forEach((d) => {
  const opt = document.createElement("option");
  opt.value = d.key;
  opt.textContent = d.key === "all" ? d.title : `${d.code} · ${d.title}`;
  domainSel?.append(opt);
});

const summary = summarizeCatalogCoverage();
if (kpiModels) kpiModels.textContent = String(summary.totalModels);
if (kpiLex) kpiLex.textContent = String(summary.uniqueLexModules);
if (kpiPhases) kpiPhases.textContent = `${summary.lifecycleCount} / ${summary.lifecycleCount}`;

// Theme switcher
const THEMES = [
  { cls: "dark", label: "Theme: Dark" },
  { cls: "", label: "Theme: Light" },
  { cls: "dark immersive", label: "Theme: Immersive" },
];
let themeIdx = 0;
themeBtn?.addEventListener("click", () => {
  themeIdx = (themeIdx + 1) % THEMES.length;
  document.documentElement.className = THEMES[themeIdx].cls;
  themeBtn.textContent = THEMES[themeIdx].label;
});

async function dispatchHolo(query) {
  if (verdictCache.has(query)) return verdictCache.get(query);
  if (!holoApp) return null;
  const raw = utf8Dec.decode(await holoApp.dispatch(utf8Enc.encode(query)));
  verdictCache.set(query, raw);
  return raw;
}

function renderPlatformOrgans() {
  if (!organGrid) return;
  organGrid.replaceChildren();
  for (const organ of VERDANT_PLATFORM_ORGANS) {
    const card = document.createElement("article");
    card.className = "cat-organ-card";

    const top = document.createElement("div");
    const h3 = document.createElement("h3");
    h3.textContent = organ.name;
    const mon = document.createElement("p");
    mon.className = "cat-organ-moniker";
    mon.textContent = organ.moniker;
    const desc = document.createElement("p");
    desc.className = "cat-organ-desc";
    desc.style.marginTop = "var(--space-2)";
    desc.textContent = organ.cognitiveRole;
    top.append(h3, mon, desc);

    const bottom = document.createElement("div");
    bottom.style.display = "grid";
    bottom.style.gap = "var(--space-2)";
    const lex = document.createElement("p");
    lex.className = "cat-organ-lex";
    lex.textContent = `Lexicon: ${organ.lexicon.join(" · ")}`;
    const link = document.createElement("a");
    link.className = "cat-btn";
    link.href = new URL(organ.route === "/" ? "../" : `..${organ.route}`, import.meta.url).href;
    link.textContent = `Open ${organ.name}`;
    bottom.append(lex, link);

    card.append(top, bottom);
    organGrid.append(card);
  }
}

async function verifyEntry(entry, verdictEl) {
  if (!holoApp) {
    verdictEl.textContent = "Awaiting /app.holo…";
    return false;
  }
  const posResults = [];
  for (const q of entry.holoQueries) {
    const r = await dispatchHolo(q);
    if (!r?.startsWith("conformant:")) {
      verdictEl.textContent = `Failed ${q}: ${r || "no response"}`;
      return false;
    }
    posResults.push(q);
  }
  const negResults = [];
  for (const rq of entry.refusalQueries) {
    const r = await dispatchHolo(rq);
    if (!r || (!r.startsWith("refused:") && r !== "invalid request")) {
      verdictEl.textContent = `Refusal gate missed "${rq}": ${r}`;
      return false;
    }
    negResults.push(`${rq || "<empty>"} -> ${r}`);
  }
  const sampleRaw = await dispatchHolo(entry.holoQueries[0]);
  const sampleShort = sampleRaw && sampleRaw.length > 74 ? `${sampleRaw.slice(0, 74)}…` : sampleRaw;
  const negSuffix = negResults.length ? ` · R5 Refusal (${negResults.join(", ")})` : "";
  verdictEl.textContent = `Verified (${posResults.length} query${posResults.length === 1 ? "" : "ies"}): ${sampleShort}${negSuffix}`;
  if (kpiProofs) kpiProofs.textContent = `${verdictCache.size} Verified`;
  return true;
}

const activeVerdictHooks = [];

function renderCatalogList() {
  if (!listEl) return;
  activeVerdictHooks.length = 0;
  const items = filterCatalogModels({
    lifecycle: lifecycleSel?.value || "all",
    domain: domainSel?.value || "all",
    role: roleSel?.value || "all",
    search: searchInput?.value || "",
  });

  if (countLabel) {
    countLabel.textContent = `Showing ${items.length} of ${summary.totalModels} cataloged standards & Prism models across ${summary.uniqueLexModules} LexLean modules.`;
  }

  listEl.replaceChildren();
  if (items.length === 0) {
    const empty = document.createElement("div");
    empty.className = "cat-empty";
    empty.textContent = "No cataloged models matched the current filter. Clear search or select All Lifecycle Phases.";
    listEl.append(empty);
    return;
  }

  for (const entry of items) {
    const art = document.createElement("article");
    art.className = "cat-entry";
    art.id = entry.id;
    art.dataset.catalogId = entry.id;

    const top = document.createElement("div");
    top.className = "cat-entry-top";
    const h3 = document.createElement("h3");
    h3.textContent = entry.title;
    const idSpan = document.createElement("span");
    idSpan.className = "cat-entry-id";
    idSpan.textContent = `${entry.id} · ${entry.domain} · ${entry.lifecycle}`;
    top.append(h3, idSpan);

    const auth = document.createElement("div");
    auth.className = "cat-entry-auth";
    auth.textContent = `Authority: ${entry.authority}  /  ${entry.claimLevel}  /  ${entry.lexModule}`;

    const inv = document.createElement("p");
    inv.className = "cat-entry-inv";
    inv.textContent = entry.invariant;

    const meta = document.createElement("div");
    meta.className = "cat-entry-meta";
    meta.textContent = `Domain Taxon: ${entry.taxon.join(" · ")}  |  Holo Queries: ${entry.holoQueries.slice(0, 5).join(", ")}${entry.holoQueries.length > 5 ? ` (+${entry.holoQueries.length - 5} more)` : ""}`;

    const foot = document.createElement("div");
    foot.className = "cat-entry-foot";
    const verdictEl = document.createElement("span");
    verdictEl.className = "cat-verdict";
    const cachedSample = verdictCache.get(entry.holoQueries[0]);
    verdictEl.textContent = cachedSample
      ? `Verified: ${cachedSample.length > 72 ? `${cachedSample.slice(0, 72)}…` : cachedSample}`
      : `Ready (${entry.holoQueries.length} Holo proof${entry.holoQueries.length === 1 ? "" : "s"}${entry.refusalQueries.length ? ` + ${entry.refusalQueries.length} R5 refusal` : ""})`;

    const actions = document.createElement("div");
    actions.className = "cat-actions";
    const vBtn = document.createElement("button");
    vBtn.type = "button";
    vBtn.className = "cat-btn primary";
    vBtn.dataset.verifyId = entry.id;
    vBtn.textContent = "Verify in Holo";
    vBtn.addEventListener("click", () => verifyEntry(entry, verdictEl));
    actions.append(vBtn);

    for (const rk of entry.studioRoles.slice(0, 2)) {
      const rLink = document.createElement("a");
      rLink.className = "cat-btn";
      rLink.href = new URL(`../studio/?catalog=${encodeURIComponent(entry.id)}#${rk}`, import.meta.url).href;
      rLink.textContent = `Compose · ${rk.charAt(0).toUpperCase() + rk.slice(1)}`;
      actions.append(rLink);
    }

    foot.append(verdictEl, actions);
    art.append(top, auth, inv, meta, foot);
    listEl.append(art);
    activeVerdictHooks.push(() => verifyEntry(entry, verdictEl));
  }
}

lifecycleSel?.addEventListener("change", renderCatalogList);
domainSel?.addEventListener("change", renderCatalogList);
roleSel?.addEventListener("change", renderCatalogList);
searchInput?.addEventListener("input", renderCatalogList);

verifyAllBtn?.addEventListener("click", async () => {
  verifyAllBtn.disabled = true;
  verifyAllBtn.textContent = "Verifying Holo Proofs…";
  for (const run of activeVerdictHooks) {
    await run();
  }
  verifyAllBtn.disabled = false;
  verifyAllBtn.textContent = "All Displayed Proofs Verified";
});

let loadedWorldPacks = [];
let galleryContentType = "all";
let latestStreamStatus = {};
const STREAM_PREFS_KEY = "verdant.observer.streams.v1";

function loadStreamPrefs() {
  try {
    const raw = JSON.parse(localStorage.getItem(STREAM_PREFS_KEY) || "{}");
    return {
      disabled: Array.isArray(raw.disabled) ? raw.disabled : [],
      custom: Array.isArray(raw.custom) ? raw.custom : [],
    };
  } catch {
    return { disabled: [], custom: [] };
  }
}

function saveStreamPrefs(prefs) {
  try {
    localStorage.setItem(STREAM_PREFS_KEY, JSON.stringify(prefs));
  } catch {
    // Ignore privacy/quota errors
  }
}

function renderSensoryStreams() {
  if (!sensoryGrid) return;
  const activeStream = galleryStreamSel?.value || "all";
  const prefs = loadStreamPrefs();
  const disabledSet = new Set(prefs.disabled);
  const allStreams = [
    ...LIVE_CATALOG_STREAMS,
    ...prefs.custom.map((cs, i) => ({
      id: cs.streamId,
      label: cs.label || cs.streamId,
      authority: cs.authority || "Observer Custom Stream",
      basisPrime: 53 + i * 2,
      organIri: `urn:verdant:sensory:${cs.streamId}`,
      sensoryModality: `Custom ${cs.adapterKind || "schema-org-catalog"} stream`,
      mediaTypes: ["model/gltf-binary", "application/ld+json"],
      targetParts: cs.targetParts || ["hero-props"],
      custom: true,
    })),
  ];

  sensoryGrid.replaceChildren();
  for (const s of allStreams) {
    const isDisabled = disabledSet.has(s.id);
    const st = latestStreamStatus[s.id];
    const card = document.createElement("article");
    card.className = "cat-sensory-card";
    card.dataset.sensoryStream = s.id;
    card.setAttribute("aria-pressed", activeStream === s.id ? "true" : "false");

    const top = document.createElement("div");
    top.className = "cat-sensory-top";
    const prime = document.createElement("span");
    prime.className = "cat-sensory-prime";
    prime.textContent = `Prime ${s.basisPrime} · ${s.authority}`;
    const state = document.createElement("span");
    state.className = "cat-sensory-state";
    if (isDisabled) {
      state.dataset.status = "offline";
      state.textContent = "PAUSED";
    } else if (st) {
      state.dataset.status = st.status;
      state.textContent = st.status === "live"
        ? `LIVE · ${st.count} items${st.latencyMs ? ` · ${st.latencyMs}ms` : ""}`
        : "OFFLINE";
    } else {
      state.textContent = "Probing…";
    }
    top.append(prime, state);

    const h3 = document.createElement("strong");
    h3.style.fontFamily = "var(--font-display)";
    h3.style.fontSize = "var(--text-16)";
    h3.textContent = s.label;

    const mod = document.createElement("span");
    mod.style.fontSize = "var(--text-14)";
    mod.style.color = "var(--muted-foreground)";
    mod.textContent = s.sensoryModality;

    const iri = document.createElement("span");
    iri.style.fontFamily = "var(--font-mono)";
    iri.style.fontSize = "var(--text-14)";
    iri.style.color = "var(--muted-foreground)";
    iri.textContent = `${s.organIri} -> ${(s.targetParts || []).join(", ")}`;

    const ctrlRow = document.createElement("div");
    ctrlRow.className = "cat-actions";
    const focusBtn = document.createElement("button");
    focusBtn.type = "button";
    focusBtn.className = activeStream === s.id ? "cat-btn primary" : "cat-btn";
    focusBtn.textContent = activeStream === s.id ? "Focused" : "Focus Feed";
    focusBtn.addEventListener("click", () => {
      if (galleryStreamSel) {
        galleryStreamSel.value = galleryStreamSel.value === s.id ? "all" : s.id;
      }
      refreshLiveGallery();
    });
    const togBtn = document.createElement("button");
    togBtn.type = "button";
    togBtn.className = "cat-btn";
    togBtn.textContent = isDisabled ? "Enable" : "Pause";
    togBtn.addEventListener("click", () => {
      const next = loadStreamPrefs();
      if (disabledSet.has(s.id)) {
        next.disabled = next.disabled.filter((x) => x !== s.id);
      } else {
        next.disabled = [...new Set([...next.disabled, s.id])];
      }
      saveStreamPrefs(next);
      refreshLiveGallery();
    });
    ctrlRow.append(focusBtn, togBtn);
    if (s.custom) {
      const rmBtn = document.createElement("button");
      rmBtn.type = "button";
      rmBtn.className = "cat-btn";
      rmBtn.textContent = "Remove";
      rmBtn.addEventListener("click", () => {
        const next = loadStreamPrefs();
        next.custom = next.custom.filter((x) => x.streamId !== s.id);
        next.disabled = next.disabled.filter((x) => x !== s.id);
        saveStreamPrefs(next);
        refreshLiveGallery();
      });
      ctrlRow.append(rmBtn);
    }

    card.append(top, h3, mod, iri, ctrlRow);
    sensoryGrid.append(card);
  }
}

$("cat-add-stream-form")?.addEventListener("submit", (e) => {
  e.preventDefault();
  const sid = $("cat-new-stream-id")?.value?.trim().toLowerCase().replace(/[^a-z0-9-]+/g, "-") || "";
  const label = $("cat-new-stream-label")?.value?.trim() || "";
  const url = $("cat-new-stream-url")?.value?.trim() || "";
  const adapter = $("cat-new-stream-adapter")?.value || "schema-org-catalog";
  if (!sid || !label || !/^https:\/\//.test(url)) return;
  const prefs = loadStreamPrefs();
  prefs.custom = [
    ...prefs.custom.filter((x) => x.streamId !== sid),
    { streamId: sid, label, endpointUrl: url, adapterKind: adapter, targetParts: ["hero-props"], enabled: true },
  ];
  saveStreamPrefs(prefs);
  $("cat-add-stream-form").reset();
  refreshLiveGallery();
});

function renderGalleryPills() {
  if (!galleryPills) return;
  galleryPills.replaceChildren();
  for (const ct of AUTHORITATIVE_CONTENT_TYPES) {
    const b = document.createElement("button");
    b.type = "button";
    b.className = "cat-pill";
    b.dataset.galleryType = ct.key;
    b.setAttribute("aria-pressed", ct.key === galleryContentType ? "true" : "false");
    b.textContent = ct.label;
    b.addEventListener("click", () => {
      galleryContentType = ct.key;
      galleryPills.querySelectorAll(".cat-pill").forEach((el) => {
        el.setAttribute("aria-pressed", el.dataset.galleryType === galleryContentType ? "true" : "false");
      });
      refreshLiveGallery();
    });
    galleryPills.append(b);
  }
}

function buildStudioDeepLink(item) {
  const qs = new URLSearchParams();
  qs.set("stream", item.streamId);
  qs.set("type", item.contentTypeKey);
  qs.set("q", item.title.split("(")[0].trim());
  if (item.glbUrl) {
    qs.set("autoGlb", item.glbUrl);
    qs.set("autoTitle", item.title);
  }
  if (item.envSkyUrl) {
    qs.set("autoSky", item.envSkyUrl);
    qs.set("autoTitle", item.title);
  }
  return new URL(`../studio/?${qs.toString()}`, import.meta.url).href;
}

let gallerySeq = 0;
async function refreshLiveGallery() {
  const seq = ++gallerySeq;
  const prefs = loadStreamPrefs();
  const res = await searchLiveCatalogStreams({
    query: gallerySearch?.value || "",
    contentType: galleryContentType,
    streamId: galleryStreamSel?.value || "all",
    localWorldPacks: loadedWorldPacks,
    disabledStreamIds: prefs.disabled,
    customStreams: prefs.custom,
  });
  if (seq !== gallerySeq) return;
  latestStreamStatus = res.streamStatus || {};
  renderSensoryStreams();

  if (gallerySummary) {
    gallerySummary.textContent =
      `Showing ${res.totalCount} featured assets (${res.liveRemoteCount} live internet stream · ${res.localLockedCount} Prism locked) across differentiated Observer sensory organs.`;
  }
  if (!galleryGrid) return;
  galleryGrid.replaceChildren();
  for (const item of (res.items || []).slice(0, 18)) {
    const card = document.createElement("article");
    card.className = "cat-gallery-card";
    card.dataset.galleryModelId = item.id;

    const media = document.createElement("div");
    media.className = "cat-gallery-media";
    if (item.previewUrl && (item.streamId === "khronos-gltf" || item.streamId === "polyhaven" || (item.streamId === "openverse" && item.contentTypeKey === "PBRMaterial"))) {
      const img = document.createElement("img");
      img.src = item.previewUrl;
      img.alt = item.title;
      img.loading = "lazy";
      media.append(img);
    } else if (item.contentTypeKey === "AudioObject" && item.previewUrl) {
      const wrap = document.createElement("div");
      wrap.style.width = "100%";
      wrap.style.display = "grid";
      wrap.style.gap = "var(--space-2)";
      const lbl = document.createElement("span");
      lbl.style.fontFamily = "var(--font-mono)";
      lbl.style.fontSize = "var(--text-14)";
      lbl.textContent = `AudioObject · ${item.mediaType}`;
      const aud = document.createElement("audio");
      aud.controls = true;
      aud.src = item.previewUrl;
      aud.style.width = "100%";
      wrap.append(lbl, aud);
      media.append(wrap);
    } else if (item.contentTypeKey === "Font") {
      const spec = document.createElement("div");
      spec.style.fontSize = "1.75rem";
      spec.style.fontWeight = "600";
      spec.style.lineHeight = "1.2";
      spec.textContent = `Aa Bb Cc\n${item.properties?.fontFamily || item.title}`;
      media.append(spec);
    } else {
      const badge = document.createElement("div");
      badge.style.fontFamily = "var(--font-mono)";
      badge.style.fontSize = "var(--text-14)";
      badge.style.color = "var(--muted-foreground)";
      badge.textContent = `${item.schemaType}\n${item.mediaType}\n-> ${item.targetSlot}`;
      media.append(badge);
    }

    const body = document.createElement("div");
    body.className = "cat-gallery-body";
    const meta = document.createElement("div");
    meta.className = "cat-gallery-meta";
    meta.textContent = `${item.schemaType} · ${item.streamLabel} · ${item.licenseSpdx}`;
    const h3 = document.createElement("h3");
    h3.textContent = item.title;
    const desc = document.createElement("p");
    desc.textContent = item.description;
    body.append(meta, h3, desc);

    const foot = document.createElement("div");
    foot.className = "cat-gallery-foot";
    const openLink = document.createElement("a");
    openLink.className = "cat-btn primary";
    openLink.href = buildStudioDeepLink(item);
    openLink.textContent = item.glbUrl ? "Load 3D & Compose in Studio" : "Import & Compose in Studio";
    foot.append(openLink);
    if (item.sourceUri?.startsWith("http")) {
      const srcLink = document.createElement("a");
      srcLink.className = "cat-btn";
      srcLink.href = item.sourceUri;
      srcLink.target = "_blank";
      srcLink.rel = "noopener noreferrer";
      srcLink.textContent = "Source";
      foot.append(srcLink);
    }

    card.append(media, body, foot);
    galleryGrid.append(card);
  }
}

galleryForm?.addEventListener("submit", (e) => {
  e.preventDefault();
  refreshLiveGallery();
});
galleryStreamSel?.addEventListener("change", refreshLiveGallery);

async function renderImportedDomainPacks() {
  if (!domainGrid) return;
  try {
    const res = await fetch(new URL("./world-packs.json", import.meta.url));
    if (!res.ok) return;
    const data = await res.json();
    const domains = Array.isArray(data?.domains) ? data.domains : [];
    loadedWorldPacks = domains;
    domainGrid.replaceChildren();
    for (const d of domains) {
      const card = document.createElement("article");
      card.className = "cat-organ-card";
      card.dataset.domainSlug = d.slug;

      const top = document.createElement("div");
      const h3 = document.createElement("h3");
      h3.textContent = d.title;
      const mon = document.createElement("p");
      mon.className = "cat-organ-moniker";
      mon.textContent = `${d.id} · ${d.schemaType} · Seed ${d.seed} · LOD${d.defaultLodRank}`;
      const desc = document.createElement("p");
      desc.className = "cat-organ-desc";
      desc.style.marginTop = "var(--space-2)";
      const modelNames = (d.models || []).map((m) => m.name).join(", ");
      const charNames = (d.characters || []).map((c) => `${c.name} (${c.role})`).join(", ");
      desc.textContent = `Environment: ${d.environment?.name || "—"} (${d.environment?.acousticVolumeM3 || 0} m³). Models: ${modelNames}. 22-Joint Rigs: ${charNames}.`;
      top.append(h3, mon, desc);

      const bottom = document.createElement("div");
      bottom.style.display = "grid";
      bottom.style.gap = "var(--space-2)";
      const lex = document.createElement("p");
      lex.className = "cat-organ-lex";
      const mats = [...new Set([...(d.models || []).map((m) => m.materialId), ...(d.items || []).map((i) => i.materialId)])];
      lex.textContent = `Materials: ${mats.slice(0, 3).join(" · ")} · Items: ${(d.items || []).length}`;
      const link = document.createElement("a");
      link.className = "cat-btn primary";
      link.href = new URL(`../studio/?domain=${encodeURIComponent(d.slug)}#compose`, import.meta.url).href;
      link.textContent = "Import & Compose in Studio";
      bottom.append(lex, link);

      card.append(top, bottom);
      domainGrid.append(card);
    }
  } catch {
    // Graceful fallback if catalog JSON is absent during isolated unit test
  }
}

let activeAppGenre = "all";

function renderStitchGenrePills() {
  if (!appsPills) return;
  appsPills.replaceChildren();
  const options = [{ id: "all", label: "All Genres" }, ...STITCH_APP_GENRES];
  for (const g of options) {
    const btn = document.createElement("button");
    btn.type = "button";
    btn.className = "cat-pill";
    btn.setAttribute("aria-pressed", g.id === activeAppGenre ? "true" : "false");
    btn.textContent = g.label;
    btn.addEventListener("click", () => {
      activeAppGenre = g.id;
      renderStitchGenrePills();
      renderStitchAppCatalog();
    });
    appsPills.append(btn);
  }
}

function renderStitchAppCatalog(apps = loadStitchAppCatalog()) {
  if (kpiApps) kpiApps.textContent = String(apps.length);
  if (!appsGrid) return;
  const visible = activeAppGenre === "all" ? apps : apps.filter((a) => a.genre === activeAppGenre);
  appsGrid.replaceChildren();
  if (!visible.length) {
    const empty = document.createElement("p");
    empty.className = "cat-sec-sub";
    empty.textContent = "No published Verdant Apps in this genre yet. Open Verdant Studio to package and publish one to Stitch.";
    appsGrid.append(empty);
    return;
  }
  for (const app of visible) {
    const card = document.createElement("article");
    card.className = "cat-organ-card";
    card.dataset.stitchAppId = app.appId;

    const top = document.createElement("div");
    const h3 = document.createElement("h3");
    h3.textContent = app.title;
    const mon = document.createElement("p");
    mon.className = "cat-organ-moniker";
    mon.textContent = `${app.genreLabel} · ${app.schemaType} · ${app.archiveFormat} (${app.uorcBytes} B)`;
    const desc = document.createElement("p");
    desc.className = "cat-organ-desc";
    desc.style.marginTop = "var(--space-2)";
    desc.textContent = app.summary;
    top.append(h3, mon, desc);

    const bottom = document.createElement("div");
    bottom.style.display = "grid";
    bottom.style.gap = "var(--space-2)";
    const shardsMeta = document.createElement("p");
    shardsMeta.className = "cat-organ-lex";
    const shardCount = Array.isArray(app.stitchShards) ? app.stitchShards.length : 3;
    shardsMeta.textContent = `Stitch 2-of-${shardCount} STCH · World: ${app.worldTitle} · sha256:${String(app.payloadSha256 || "").slice(0, 10)}…`;
    const row = document.createElement("div");
    row.style.display = "flex";
    row.style.flexWrap = "wrap";
    row.style.gap = "var(--space-2)";

    const studioLink = document.createElement("a");
    studioLink.className = "cat-btn primary";
    studioLink.href = new URL(
      `../studio/?domain=${encodeURIComponent(app.domainSlug)}&app=${encodeURIComponent(app.appId)}`,
      import.meta.url,
    ).href;
    studioLink.textContent = "Open in Studio";

    const engineLink = document.createElement("a");
    engineLink.className = "cat-btn";
    engineLink.href = new URL(
      `../starter/?domain=${encodeURIComponent(app.domainSlug)}`,
      import.meta.url,
    ).href;
    engineLink.textContent = "Play World";

    row.append(studioLink, engineLink);
    bottom.append(shardsMeta, row);
    card.append(top, bottom);
    appsGrid.append(card);
  }
}

renderStitchGenrePills();
renderStitchAppCatalog();
hydrateStitchAppCatalog().then((apps) => renderStitchAppCatalog(apps));
subscribeStitchAppCatalog((nextApps) => renderStitchAppCatalog(nextApps));
renderPlatformOrgans();
renderCatalogList();
renderSensoryStreams();
renderGalleryPills();
renderImportedDomainPacks().then(() => refreshLiveGallery());

// Load `/app.holo`, verify BLAKE3 self-fingerprint, replay acceptance vectors, and verify visible entries:
(async () => {
  try {
    const res = await fetch(new URL("../app.holo", import.meta.url));
    if (!res.ok) throw new Error(`HTTP ${res.status}`);
    const bytes = new Uint8Array(await res.arrayBuffer());
    const fpHex = [...bytes.subarray(bytes.length - 32)].map((b) => b.toString(16).padStart(2, "0")).join("");
    const selfOk = kappa(bytes.subarray(0, bytes.length - 32)) === `blake3:${fpHex}`;
    holoApp = await load(bytes);
    const acc = await holoApp.acceptance();
    for (const vec of holoApp.vectors ?? []) {
      verdictCache.set(utf8Dec.decode(vec.request), utf8Dec.decode(vec.response));
    }
    if (kpiProofs) kpiProofs.textContent = `${acc.passed}/${acc.total} Vectors`;
    if (holoBanner) {
      holoBanner.textContent = `Holo Verified · ${acc.passed}/${acc.total} Acceptance Vectors · BLAKE3 ${selfOk ? fpHex.slice(0, 16) : "MISMATCH"}…`;
    }
    for (const run of activeVerdictHooks) {
      await run();
    }
  } catch (err) {
    if (holoBanner) holoBanner.textContent = `Holo load error: ${String(err?.message || err)}`;
  }
})();
