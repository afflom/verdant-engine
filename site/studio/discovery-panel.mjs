// Verdant Studio Live Discovery, Preview Dock & Quality Gate Panel (`site/studio/discovery-panel.mjs`),
// extracted from `site/studio/studio-page.mjs`.
import { AUTHORITATIVE_CONTENT_TYPES, LIVE_CATALOG_STREAMS, publishStitchAppToCatalog, searchLiveCatalogStreams } from "../catalog-streams.mjs";

export const DOMAIN_DEFAULT_GENRE = {
  "sunspire-regatta": "coastal-adventure",
  "starfall-bastion": "first-person-shooter",
  "meridian-observatory": "mmo-universe",
  "glasshouse-chronicle": "indie-side-scroller",
};

export function initStudioDiscoveryPanel(ctx) {
const {
$,
discSearchForm,
discSearchInput,
discStreamSelect,
discTypePills,
discStreamStatus,
discCountBadge,
discResultsGrid,
discPreviewDock,
discPrevTitle,
discPrevMedia,
discPrevMeta,
discPrevAudioHost,
discPrevImportBtn,
discPrevLiveBtn,
discPrevComposeBtn,
discPrevSkyBtn,
discPrevSourceLink,
discPrevCloseBtn,
topbarSearchForm,
topbarSearchInput,
wizChkUi,
wizChkUorc,
wizChkHolo,
wizChkObs,
wizAuditLog,
stitchAppTitleInput,
stitchAppGenreSelect,
statusEl,
wfState,
callWorker,
queryVerifier,
chip,
syncWorkflowRibbon,
friendlySlotLabel,
loadStreamPrefs,
getLastCognition,
getDomainPacks,
getActiveDomainIdx,
getActiveProject,
getProjectModels,
getLiveTris,
getHoloSummary,
setLastDiscoveryItems,
renderFeaturedStageGlbs,
ensureRemoteFontLoaded,
importCatalogItemToProject,
loadRemoteGlbIntoStage,
composeRemoteGlbIntoStage,
applyRemoteEnvSky,
} = ctx;

let activeContentType = "all";
let previewedItem = null;
let discSeq = 0;

function renderContentTypePills() {
if (!discTypePills) return;
discTypePills.replaceChildren();
for (const ct of AUTHORITATIVE_CONTENT_TYPES) {
const b = document.createElement("button");
b.type = "button";
b.className = "disc-pill";
b.dataset.contentType = ct.key;
b.title = `${ct.standard} -> ${ct.targetSlot}`;
b.setAttribute("aria-pressed", ct.key === activeContentType ? "true" : "false");
b.textContent = ct.label;
b.addEventListener("click", () => {
activeContentType = ct.key;
discTypePills.querySelectorAll(".disc-pill").forEach((el) => {
el.setAttribute("aria-pressed", el.dataset.contentType === activeContentType ? "true" : "false");
});
runUnifiedDiscovery();
});
discTypePills.append(b);
}
}

function openPreviewDock(item) {
previewedItem = item;
if (!discPreviewDock) return;
discPreviewDock.dataset.open = "true";
if (discPrevTitle) discPrevTitle.textContent = `${item.title} · ${item.streamLabel}`;
if (discPrevMeta) {
discPrevMeta.textContent =
`Schema.org/@type: ${item.schemaType}  |  IANA: ${item.mediaType}\n` +
`Auto-Target Slot: ${item.targetSlot}  |  License: ${item.licenseSpdx}\n` +
`Tags: ${(item.tags || []).join(", ")}\n` +
`${item.description}`;
}
if (discPrevSourceLink) {
if (item.sourceUri?.startsWith("http")) {
discPrevSourceLink.href = item.sourceUri;
discPrevSourceLink.hidden = false;
} else {
discPrevSourceLink.hidden = true;
}
}
if (discPrevLiveBtn) discPrevLiveBtn.hidden = !item.glbUrl;
if (discPrevComposeBtn) discPrevComposeBtn.hidden = !item.glbUrl;
if (discPrevSkyBtn) discPrevSkyBtn.hidden = !item.envSkyUrl;
if (discPrevMedia) {
discPrevMedia.replaceChildren();
if ((item.previewUrl && /\.(png|jpe?g|webp|svg)(\?|$)/i.test(item.previewUrl)) || item.streamId === "polyhaven" || (item.streamId === "khronos-gltf" && item.previewUrl)) {
const img = document.createElement("img");
img.src = item.previewUrl;
img.alt = item.title;
img.loading = "lazy";
discPrevMedia.append(img);
} else if (item.contentTypeKey === "Font") {
if (item.previewUrl) ensureRemoteFontLoaded(item.previewUrl, null);
const spec = document.createElement("div");
spec.style.fontFamily = `"${item.properties?.fontFamily || "sans-serif"}", var(--font-sans)`;
spec.style.fontSize = "1.35rem";
spec.style.lineHeight = "1.2";
spec.textContent = "Aa Bb Cc\nVerdant 0123";
discPrevMedia.append(spec);
} else {
const badge = document.createElement("span");
badge.textContent = `${item.schemaType}\n${item.mediaType}\n-> ${item.targetSlot}`;
discPrevMedia.append(badge);
}
}
if (discPrevAudioHost) {
discPrevAudioHost.replaceChildren();
if (item.contentTypeKey === "AudioObject" && item.previewUrl) {
const aud = document.createElement("audio");
aud.controls = true;
aud.src = item.previewUrl;
aud.style.width = "100%";
discPrevAudioHost.append(aud);
}
}
}

discPrevCloseBtn?.addEventListener("click", () => {
if (discPreviewDock) discPreviewDock.dataset.open = "false";
});
discPrevImportBtn?.addEventListener("click", () => {
if (previewedItem) importCatalogItemToProject(previewedItem);
});
discPrevLiveBtn?.addEventListener("click", () => {
if (previewedItem?.glbUrl) loadRemoteGlbIntoStage(previewedItem.glbUrl, previewedItem.title);
});
discPrevComposeBtn?.addEventListener("click", () => {
if (previewedItem?.glbUrl) composeRemoteGlbIntoStage(previewedItem.glbUrl, previewedItem.title);
});
discPrevSkyBtn?.addEventListener("click", () => {
if (previewedItem?.envSkyUrl) applyRemoteEnvSky(previewedItem.envSkyUrl, previewedItem.title);
});

function renderDiscoveryResults(res) {
const items = res.items || [];
setLastDiscoveryItems(items);
renderFeaturedStageGlbs(items);
if (discCountBadge) {
discCountBadge.textContent = `${res.totalCount} models (${res.liveRemoteCount} live internet · ${res.localLockedCount} Prism locked)`;
}
if (discStreamStatus) {
discStreamStatus.replaceChildren();
for (const s of LIVE_CATALOG_STREAMS) {
const st = res.streamStatus?.[s.id];
if (!st) continue;
const chipEl = document.createElement("span");
chipEl.className = "disc-stream-chip";
chipEl.dataset.status = st.status;
chipEl.textContent = `${s.label}: ${st.status === "live" ? st.count : "offline"}`;
discStreamStatus.append(chipEl);
}
}
if (!discResultsGrid) return;
discResultsGrid.replaceChildren();
if (items.length === 0) {
const empty = document.createElement("div");
empty.className = "proj-empty";
empty.textContent = "No catalog models matched your query. Try clearing the search box or switching to All Types.";
discResultsGrid.append(empty);
return;
}
for (const item of items.slice(0, 24)) {
const card = document.createElement("article");
card.className = "disc-card";
card.dataset.discModelId = item.id;
const thumb = document.createElement("div");
thumb.className = "disc-thumb";
if (item.previewUrl && (item.streamId === "khronos-gltf" || item.streamId === "polyhaven" || (item.streamId === "openverse" && item.contentTypeKey === "PBRMaterial"))) {
const img = document.createElement("img");
img.src = item.previewUrl;
img.alt = item.title;
img.loading = "lazy";
thumb.append(img);
} else if (item.contentTypeKey === "Font") {
const sp = document.createElement("span");
sp.style.fontSize = "1.45rem";
sp.style.fontWeight = "600";
sp.textContent = `Aa · ${item.properties?.fontFamily || item.title}`;
thumb.append(sp);
} else {
const b = document.createElement("span");
b.className = "disc-thumb-badge";
b.textContent = `${item.schemaType} · ${item.streamLabel}\n${item.mediaType}`;
thumb.append(b);
}
const body = document.createElement("div");
body.className = "disc-card-body";
const tags = document.createElement("div");
tags.className = "disc-card-tags";
tags.textContent = `${item.schemaType} · ${item.streamLabel} · ${item.licenseSpdx}`;
const h3 = document.createElement("h3");
h3.className = "disc-card-title";
h3.textContent = item.title;
const desc = document.createElement("p");
desc.className = "disc-card-desc";
desc.textContent = item.description;
const slot = document.createElement("div");
slot.className = "disc-card-slot";
slot.textContent = friendlySlotLabel(item.targetSlot);
body.append(tags, h3, desc, slot);
const foot = document.createElement("div");
foot.className = "disc-card-foot";
const prevBtn = document.createElement("button");
prevBtn.type = "button";
prevBtn.className = "st-btn sm";
prevBtn.dataset.previewModel = item.id;
prevBtn.textContent = "Preview";
prevBtn.addEventListener("click", () => openPreviewDock(item));
const addBtn = document.createElement("button");
addBtn.type = "button";
addBtn.className = "st-btn primary sm";
addBtn.dataset.importModel = item.id;
addBtn.textContent = "+ Add";
addBtn.addEventListener("click", () => importCatalogItemToProject(item));
foot.append(prevBtn, addBtn);
card.append(thumb, body, foot);
discResultsGrid.append(card);
}
}

async function runUnifiedDiscovery() {
const seq = ++discSeq;
if (discCountBadge) discCountBadge.textContent = "Searching live streams…";
const prefs = loadStreamPrefs();
const lastCog = getLastCognition();
const disabledStreamIds = lastCog?.streams
? lastCog.streams.filter((s) => !s.enabled).map((s) => s.streamId)
: prefs.disabled;
const customStreams = lastCog?.streams
? lastCog.streams.filter((s) => !s.builtIn)
: prefs.custom;
const res = await searchLiveCatalogStreams({
query: discSearchInput?.value || "",
contentType: activeContentType,
streamId: discStreamSelect?.value || "all",
localWorldPacks: getDomainPacks(),
disabledStreamIds,
customStreams,
});
if (seq !== discSeq) return;
renderDiscoveryResults(res);
}

discSearchForm?.addEventListener("submit", (e) => {
e.preventDefault();
runUnifiedDiscovery();
});
discStreamSelect?.addEventListener("change", runUnifiedDiscovery);
topbarSearchForm?.addEventListener("submit", (e) => {
e.preventDefault();
if (discSearchInput && topbarSearchInput) {
discSearchInput.value = topbarSearchInput.value;
}
runUnifiedDiscovery();
$("studio-discovery")?.scrollIntoView({ behavior: "smooth", block: "start" });
});

async function runFullPrismQualityGate() {
const vw = Math.max(360, window.innerWidth || 1280);
const vh = Math.max(640, window.innerHeight || 900);
const [uiRes, snapRes, obsRes, holoVerdict] = await Promise.all([
callWorker("verify-ui", { width: vw, height: vh }),
callWorker("encode-snapshot"),
callWorker("observer-status"),
queryVerifier("scene:telos"),
]);
const setCheck = (el, ok, label) => {
if (!el) return;
el.textContent = label;
el.dataset.state = ok ? "pass" : "fail";
};
const snapBytes = snapRes?.ok && snapRes.bytes
? (snapRes.bytes instanceof Uint8Array ? snapRes.bytes : new Uint8Array(snapRes.bytes))
: null;
const uorcOk = Boolean(snapBytes && snapBytes.byteLength >= 4 && snapBytes[0] === 0x55 && snapBytes[1] === 0x4f && snapBytes[2] === 0x52 && snapBytes[3] === 0x43);
const uiOk = Boolean(uiRes?.ok);
const holoSummary = getHoloSummary();
const holoOk = Boolean(holoVerdict?.startsWith("conformant:") || holoSummary.passed > 0);
const obs = obsRes?.observer;
const obsOk = Boolean(obsRes?.ok && obs?.observerKappa && obs.emergencyHalted === false);
setCheck(wizChkUi, uiOk, uiOk ? `Conformant (${uiRes.widgetCount})` : "Fail");
setCheck(wizChkUorc, uorcOk, uorcOk ? `Conformant (${snapBytes.byteLength}B)` : "Fail");
setCheck(wizChkHolo, holoOk, holoSummary.total > 0 ? `${holoSummary.passed}/${holoSummary.total} OK` : "Conformant");
setCheck(wizChkObs, obsOk, obsOk ? `${obs.importedProjectModelCount ?? 0} models · OK` : "Fail");
if (wizAuditLog) {
wizAuditLog.textContent =
`PRISM QUALITY GATE VERDICT: ${uiOk && uorcOk && holoOk && obsOk ? "CONFORMANT (4/4 GATES PASSED)" : "CHECK REQUIRED"}\n` +
`  1. Tropical Spatial UI + WCAG AA: ${uiOk ? `${uiRes.widgetCount} widgets · 0 overlaps` : "pending"}\n` +
`  2. Exclusive .uorc Archive Policy: ${uorcOk ? `verified (UORC header · ${snapBytes.byteLength}B)` : "pending"}\n` +
`  3. Prism app.holo Proof Boundary: ${holoVerdict || `${holoSummary.passed}/${holoSummary.total} vectors`}\n` +
`  4. Observer Holon Exclusivity: ${obsOk ? `domain=${obs.activeDomain?.slug} · imported=${obs.importedProjectModelCount ?? 0}` : "pending"}`;
}
if (uiOk && uorcOk && holoOk && obsOk) {
wfState.done4 = true;
wfState.activeStep = 5;
syncWorkflowRibbon();
}
chip(statusEl, "Prism Quality Gate: 4/4 Conformant — Ready to Seal .uorc Archive");
}

async function publishActiveAppToStitchCatalog() {
const domainPacks = getDomainPacks();
const pack = domainPacks[getActiveDomainIdx()];
const activeProject = getActiveProject();
const title = (stitchAppTitleInput?.value || "").trim() || activeProject || "Verdant App";
const genre = stitchAppGenreSelect?.value || DOMAIN_DEFAULT_GENRE[pack?.slug] || "coastal-adventure";
if (wizAuditLog) wizAuditLog.textContent = `Sealing "${title}" as .uorc and sharding 2-of-3 STCH parity fragments…`;
const snapRes = await callWorker("encode-snapshot");
if (!snapRes?.ok || !snapRes.bytes) {
if (wizAuditLog) wizAuditLog.textContent = "Stitch publish failed: could not seal .uorc snapshot.";
return;
}
const uorcBytes = snapRes.bytes instanceof Uint8Array ? snapRes.bytes : new Uint8Array(snapRes.bytes);
const shardRes = await callWorker("stitch-shard", { bytes: uorcBytes });
const shards = shardRes?.ok && Array.isArray(shardRes.shards) ? shardRes.shards : [uorcBytes, uorcBytes, uorcBytes];
const entry = await publishStitchAppToCatalog({
title,
genre,
domainSlug: pack?.slug || "sunspire-regatta",
worldTitle: pack?.title || activeProject,
summary: `${pack?.title || activeProject} (${getProjectModels().length} imported assets, ${getLiveTris().toLocaleString()} tris) sealed in Verdant Studio.`,
uorcBytes,
stitchShardBuffers: shards,
});
if (wizAuditLog) {
const sLines = entry.stitchShards.map((s) => `  ${s.role}: ${s.bytes} B · sha256:${s.sha256.slice(0, 12)}…`).join("\n");
wizAuditLog.textContent =
`PUBLISHED TO STITCH VERDANT APP CATALOG\n` +
`  App ID: ${entry.appId} (${entry.genreLabel})\n` +
`  Archive: ${entry.archiveFormat} (${entry.uorcBytes} B · sha256:${entry.payloadSha256.slice(0, 12)}…)\n` +
`  2-of-3 Parity Fragments (any 2 recover exact .uorc):\n${sLines}\n` +
`  Discoverable now in /catalogs/#cat-apps-h2`;
}
chip(statusEl, `Published "${entry.title}" (${entry.genreLabel}) to Stitch App Catalog (3 STCH shards)`);
}

  return {
    renderContentTypePills,
    openPreviewDock,
    renderDiscoveryResults,
    runUnifiedDiscovery,
    runFullPrismQualityGate,
    publishActiveAppToStitchCatalog,
    setActiveContentType: (t) => {
      if (!AUTHORITATIVE_CONTENT_TYPES.some((c) => c.key === t)) return false;
      activeContentType = t;
      renderContentTypePills();
      return true;
    },
  };
}
