import { fetchLiveKnowledgePassage } from "../catalog-streams.mjs";
export function initStudioObserverPanel(ctx) {
const {
cogHostEnvelopeEl,
cogPartsListEl,
cogStreamsListEl,
discStreamSelect,
cogLogEl,
cogClearSkyBtn,
cogAddStreamForm,
cogStreamIdInput,
cogStreamLabelInput,
cogStreamAdapterSel,
cogStreamUrlInput,
cogStreamPartSel,
statusEl,
callWorker,
chip,
saveStreamPrefs,
setLastCognition,
getLastDiscoveryItems,
runUnifiedDiscovery,
loadRemoteGlbIntoStage,
applyRemoteEnvSky,
} = ctx;
let cachedRefineAgenda = null;
let cachedLifecycleGate = null;
async function loadObserverRefineAgenda() {
if (!cachedRefineAgenda) {
try {
const res = await fetch("../generated/observer-refinement-agenda.json", { cache: "no-store" });
if (res.ok) cachedRefineAgenda = await res.json();
} catch {}
}
if (!cachedLifecycleGate) {
try {
const res = await fetch("../generated/lifecycle-gate-report.json", { cache: "no-store" });
if (res.ok) cachedLifecycleGate = await res.json();
} catch {}
}
return cachedRefineAgenda;
}
function renderObserverCognition(cog) {
if (!cog) return;
setLastCognition(cog);
saveStreamPrefs(cog);
if (cogHostEnvelopeEl && cog.hostEnvelope) {
const he = cog.hostEnvelope;
const gpuTag = he.isSoftwareGpu ? "Software WebGL2" : "Hardware GPU";
const drsPct = Math.round((he.recommendedRenderScalePermille || 1000) / 10);
const agLine = cachedRefineAgenda
? `\nSelf-Refinement: ${(cachedRefineAgenda.overallRefinementScorePermille / 10).toFixed(1)}% · ${cachedRefineAgenda.liveMetrics?.frontierLifted ?? 0} MIR-lifted · ${cachedRefineAgenda.proposals?.length ?? 0} proposals (v${cachedRefineAgenda.releaseVersion})`
: "";
const lgLine = cachedLifecycleGate
? `\nLifecycle Gate: ${cachedLifecycleGate.passedCount}/${cachedLifecycleGate.totalCount} ${cachedLifecycleGate.releaseQualified ? "QUALIFIED" : "OPEN"} · ${cachedLifecycleGate.liftedCoverage?.totalLiftedModules ?? 0} lifted lifecycle modules`
: "";
cogHostEnvelopeEl.textContent =
`Observer Precision: ${(cog.resolutionPermille / 10).toFixed(1)}% · ${cog.enabledStreamCount}/${cog.streamCount} Streams · ${cog.ingestedKnowledgeCount} Knowledge Passages\n` +
`UHD Tier: ${he.uhdTierLabel || "FHD 1080p"} (${he.viewportWidthPx || 1280}×${he.viewportHeightPx || 720}) · DRS ${drsPct}% + CAS · Aniso ×${he.recommendedAnisotropy || 1}\n` +
`Host: ${gpuTag} · TexCap ${he.recommendedTextureCapPx}px · Shadow ${he.recommendedShadowMapPx || 2048}px · Headroom ${(he.hostHeadroomPermille / 10).toFixed(0)}% · Max GLB ${Math.round(he.recommendedGlbMaxBytes / 1048576)}MB${agLine}${lgLine}`;
}
if (cogPartsListEl && Array.isArray(cog.parts)) {
cogPartsListEl.replaceChildren();
for (const p of cog.parts) {
const row = document.createElement("div");
row.className = "wiz-check";
row.dataset.observerPart = p.slot;
const left = document.createElement("span");
left.textContent = `${p.label} (${(p.connectedStreams || []).length} streams)`;
const right = document.createElement("strong");
right.dataset.state = p.precisionTier >= 3 ? "pass" : "pending";
right.textContent = p.precisionLabel;
row.append(left, right);
cogPartsListEl.append(row);
}
}
if (cogStreamsListEl && Array.isArray(cog.streams)) {
cogStreamsListEl.replaceChildren();
for (const s of cog.streams) {
const card = document.createElement("div");
card.className = "insp-rev-item";
card.dataset.observerStreamId = s.streamId;
const top = document.createElement("div");
top.className = "insp-rev-top";
const title = document.createElement("strong");
title.textContent = `p=${s.basisPrime} · ${s.label}`;
const btns = document.createElement("div");
btns.className = "insp-row";
const tog = document.createElement("button");
tog.type = "button";
tog.className = s.enabled ? "st-btn primary sm" : "st-btn sm";
tog.textContent = s.enabled ? "On" : "Off";
tog.addEventListener("click", async () => {
const r = await callWorker("toggle-stream", { streamId: s.streamId, enabled: !s.enabled });
if (r?.cognition) {
renderObserverCognition(r.cognition);
runUnifiedDiscovery();
}
});
btns.append(tog);
const rm = document.createElement("button");
rm.type = "button";
rm.className = "st-btn danger sm";
rm.setAttribute("aria-label", `Remove stream ${s.streamId}`);
rm.textContent = "×";
rm.addEventListener("click", async () => {
const r = await callWorker("remove-stream", { streamId: s.streamId });
if (r?.cognition) {
renderObserverCognition(r.cognition);
runUnifiedDiscovery();
if (cogLogEl) cogLogEl.textContent = `Removed sensory stream "${s.streamId}" from ObserverPrismHolon.`;
}
});
btns.append(rm);
top.append(title, btns);
const sub = document.createElement("div");
sub.style.color = "var(--muted-foreground)";
sub.textContent = `${s.organIri} -> ${(s.targetParts || []).join(", ")}`;
card.append(top, sub);
cogStreamsListEl.append(card);
}
}
if (discStreamSelect && Array.isArray(cog.streams)) {
const prevVal = discStreamSelect.value || "all";
const enabledCount = cog.streams.filter((s) => s.enabled).length;
discStreamSelect.replaceChildren();
const allOpt = document.createElement("option");
allOpt.value = "all";
allOpt.textContent = `All ${enabledCount} Active Streams`;
discStreamSelect.append(allOpt);
for (const s of cog.streams) {
if (!s.enabled) continue;
const o = document.createElement("option");
o.value = s.streamId;
o.textContent = s.label;
discStreamSelect.append(o);
}
discStreamSelect.value = [...discStreamSelect.options].some((o) => o.value === prevVal) ? prevVal : "all";
}
}
async function refreshObserverCognition(gpuInfo = null) {
await loadObserverRefineAgenda();
const r = gpuInfo
? await callWorker("update-host-envelope", gpuInfo)
: await callWorker("cognitive-resolution");
if (r?.ok && r.cognition) renderObserverCognition(r.cognition);
return r?.cognition || null;
}
async function autoRefineObserverFromStreams() {
if (cogLogEl) {
cogLogEl.textContent =
"Observer analyzing resolution gaps across 8 parts & querying connected internet streams within host envelope…";
}
chip(statusEl, "Observer self-refining graphics & natural-language cognition from live streams…");
const cog = await refreshObserverCognition();
const maxBytes = cog?.hostEnvelope?.recommendedGlbMaxBytes || 12582912;
const lastDiscoveryItems = getLastDiscoveryItems() || [];
const candidates = lastDiscoveryItems.filter((i) => i.glbUrl && (!i.sizeBytes || i.sizeBytes <= maxBytes));
const hero =
candidates.find((i) => /apollo|fox|soldier|damaged/i.test(i.title)) ||
candidates[0] || {
title: "Fox (Khronos Rigged)",
glbUrl: "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Assets/main/Models/Fox/glTF-Binary/Fox.glb",
properties: { wikipediaTopic: "Apollo_Lunar_Module" },
};
const skyItem = lastDiscoveryItems.find((i) => i.envSkyUrl);
const logLines = [];
if (hero?.glbUrl) {
const ok = await loadRemoteGlbIntoStage(hero.glbUrl, hero.title);
if (ok) logLines.push(`[R4 3D] Mounted "${hero.title}" within ${Math.round(maxBytes / 1048576)}MB host envelope`);
}
if (skyItem?.envSkyUrl) {
const ok = await applyRemoteEnvSky(skyItem.envSkyUrl, skyItem.title);
if (ok) logLines.push(`[R4 Sky] Hydrated Poly Haven panoramic dome "${skyItem.title}"`);
}
const topic =
hero?.properties?.wikipediaTopic ||
(/apollo/i.test(hero?.title || "")
? "Apollo_Lunar_Module"
: /fox/i.test(hero?.title || "")
? "Vulpes"
: /helmet|suit|crew/i.test(hero?.title || "")
? "Space_suit"
: "Physically_based_rendering");
try {
const passage = await fetchLiveKnowledgePassage(topic, 5500);
if (passage) {
const ing = await callWorker("ingest-knowledge", { request: passage });
if (ing?.ok) {
if (ing.cognition) renderObserverCognition(ing.cognition);
logLines.push(
`[R4 Language] Ingested live Wikipedia/Wikidata passage "${passage.topic}" into ISO 24617-2 engine (${String(ing.result?.kappaHex || "").slice(0, 10)}…)`,
);
}
}
} catch {
logLines.push("[Language] Offline sandbox — retaining locked Prism catalog facts.");
}
const finalCog = await refreshObserverCognition();
const pct = finalCog ? (finalCog.resolutionPermille / 10).toFixed(1) : "50.0";
if (cogLogEl) {
cogLogEl.textContent =
`Observer Self-Refinement Complete · Precision ${pct}%\n` +
(logLines.length ? logLines.join("\n") : "No external streams reachable — deterministic Prism geometry preserved.");
}
chip(statusEl, `Observer refined to ${pct}% precision from connected streams (within host envelope)`);
}
cogClearSkyBtn?.addEventListener("click", () => applyRemoteEnvSky(null, "Reset"));
cogAddStreamForm?.addEventListener("submit", async (e) => {
e.preventDefault();
const request = {
streamId: cogStreamIdInput?.value?.trim() || "",
label: cogStreamLabelInput?.value?.trim() || "",
authority: "Custom Asset Source",
adapterKind: cogStreamAdapterSel?.value || "schema-org-catalog",
endpointUrl: cogStreamUrlInput?.value?.trim() || "",
targetParts: [cogStreamPartSel?.value || "hero-props"],
};
const r = await callWorker("register-stream", { request });
if (r?.ok && r.cognition) {
renderObserverCognition(r.cognition);
if (cogLogEl) {
cogLogEl.textContent = `Connected custom source "${r.result?.streamId}" (${request.endpointUrl}).`;
}
cogAddStreamForm.reset();
runUnifiedDiscovery();
} else if (cogLogEl) {
cogLogEl.textContent = "Source rejected — please provide a valid https:// endpoint and unique slug.";
}
});
return {
renderObserverCognition,
refreshObserverCognition,
autoRefineObserverFromStreams,
};
}
