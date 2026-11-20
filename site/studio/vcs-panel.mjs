import { cacheBytesInOpfs } from "../studio.mjs";
const VCS_STORAGE_KEY = "verdant_studio_vcs_history_v1";
const PRISM_ID_KEY = "verdant_zero_account_prism_id_v1";
function bytesToBase64Url(u8) {
let bin = "";
for (let i = 0; i < u8.length; i++) bin += String.fromCharCode(u8[i]);
return btoa(bin).replace(/\+/g, "-").replace(/\//g, "_").replace(/=+$/, "");
}
function hexToBytes(hex) {
return Uint8Array.from({ length: hex.length / 2 }, (_, i) => parseInt(hex.slice(i * 2, i * 2 + 2), 16));
}
function bytesToHex(u8) {
return Array.from(u8, (b) => b.toString(16).padStart(2, "0")).join("");
}
async function gzipHoloCapsuleIfSmaller(u8) {
if (typeof CompressionStream !== "function" || u8.length < 24) return null;
const td = new TextDecoder(), te = new TextEncoder();
if (td.decode(u8.subarray(0, 16)) !== "VDNT_HOLO_REL_V1") return null;
const mLen = new DataView(u8.buffer, u8.byteOffset, u8.byteLength).getUint32(16, true);
const bodyStart = 20 + mLen;
if (!mLen || bodyStart >= u8.length) return null;
const manifest = JSON.parse(td.decode(u8.subarray(20, bodyStart)));
if (manifest.compression) return null;
const rawBody = u8.subarray(bodyStart);
const gz = new Uint8Array(
await new Response(new Blob([rawBody]).stream().pipeThrough(new CompressionStream("gzip"))).arrayBuffer(),
);
if (gz.length >= rawBody.length) return null;
manifest.compression = "gzip";
const mBytes = te.encode(JSON.stringify(manifest));
const out = new Uint8Array(20 + mBytes.length + gz.length);
out.set(u8.subarray(0, 16), 0);
new DataView(out.buffer).setUint32(16, mBytes.length, true);
out.set(mBytes, 20);
out.set(gz, 20 + mBytes.length);
const sha = bytesToHex(new Uint8Array(await crypto.subtle.digest("SHA-256", out)));
return { bytes: out, sha };
}
export function ensureLocalStudioPrismId() {
try {
const existing = JSON.parse(localStorage.getItem(PRISM_ID_KEY) || "null");
if (existing?.prismId) return existing.prismId;
} catch {}
const rnd = crypto.getRandomValues(new Uint8Array(8));
const hex = Array.from(rnd, (b) => b.toString(16).padStart(2, "0")).join("");
const prismId = `prism:local-${hex}`;
try {
localStorage.setItem(PRISM_ID_KEY, JSON.stringify({ prismId, secretHex: hex.repeat(4) }));
} catch {}
return prismId;
}
export function initStudioVcsPanel(ctx) {
const {
$,
canvas,
statusEl,
scaleBadgeEl,
stitchAppTitleInput,
callWorker,
chip,
getActiveProject,
setActiveProject,
getActiveDomainSlug,
getStudioObjects,
setStudioObjects,
getLodIdx,
getSimClockSec,
currentLodSpec,
clearStudioObjects,
syncFallbackBuffers,
renderStudioOutliner,
syncHud,
requestRender,
} = ctx;
let vcsHistory = [];
try {
vcsHistory = JSON.parse(localStorage.getItem(VCS_STORAGE_KEY) || "[]");
} catch {
vcsHistory = [];
}
if (!Array.isArray(vcsHistory)) vcsHistory = [];
let lastPackedHolo = null;
const vcsMsgEl = $("studio-vcs-msg");
const vcsHistEl = $("studio-vcs-history");
const checkoutVcsCommit = async (commitKappa) => {
const entry = vcsHistory.find((e) => e.commitKappa === commitKappa);
if (!entry) throw new Error(`Unknown VerdantVcs commit ${commitKappa}`);
if (entry.sceneJson) {
const parsed = JSON.parse(entry.sceneJson);
if (parsed.project) setActiveProject(parsed.project);
if (Array.isArray(parsed.objects)) {
await clearStudioObjects();
for (const obj of parsed.objects) {
await callWorker("studio-add-object", {
primKind: obj.kind ?? 0,
pos: obj.pos || [0, 0, 0],
scale: obj.scale || [1.5, 1.5, 1.5],
color: obj.color || [0.86, 0.34, 0.16],
lod: currentLodSpec(),
simClockSec: getSimClockSec(),
});
}
const refreshed = await callWorker("studio-list-objects");
if (Array.isArray(refreshed?.objects)) setStudioObjects(refreshed.objects);
await syncFallbackBuffers();
renderStudioOutliner();
syncHud();
requestRender();
}
}
chip(statusEl, `Checked out VerdantVcs e${entry.epoch} (${entry.commitKappa.slice(0, 10)}…) · "${entry.message}"`);
return entry;
};
const buildShareablePinUrl = (packed) => {
const u = new URL(`../frame/?pin=${encodeURIComponent(packed.stitchDeepLinkUri)}`, location.href);
if (packed.holoHex) {
u.hash = `holo=${bytesToBase64Url(hexToBytes(packed.holoHex))}`;
}
return u.href;
};
const renderVcsHistory = () => {
if (!vcsHistEl) return;
vcsHistEl.replaceChildren();
if (!vcsHistory.length) {
const empty = document.createElement("div");
empty.className = "ide-pane-sub";
empty.textContent = `Local Prism ID: ${ensureLocalStudioPrismId()} · 0 commits yet.`;
vcsHistEl.append(empty);
return;
}
for (const entry of vcsHistory.slice(0, 6)) {
const row = document.createElement("div");
row.className = "vx-outliner-row";
const label = document.createElement("span");
label.style.fontFamily = "var(--font-mono)";
label.style.fontSize = "var(--text-14)";
label.textContent = `e${entry.epoch} · ${entry.commitKappa.slice(0, 10)}… · ${entry.message}`;
const chkBtn = document.createElement("button");
chkBtn.type = "button";
chkBtn.className = "ide-btn";
chkBtn.textContent = "Checkout";
chkBtn.dataset.vcsCheckout = entry.commitKappa;
chkBtn.addEventListener("click", () => {
checkoutVcsCommit(entry.commitKappa).catch((err) => chip(statusEl, `Checkout error: ${err}`));
});
row.append(label, chkBtn);
vcsHistEl.append(row);
}
};
const commitAndPackHolo = async (overrideMsg) => {
const authorPrismId = ensureLocalStudioPrismId();
const parentCommitKappa = vcsHistory[0]?.commitKappa || "";
const epoch = (vcsHistory[0]?.epoch || 0) + 1;
const message = String(overrideMsg || vcsMsgEl?.value || "Studio scene revision").trim();
const appTitle = String(stitchAppTitleInput?.value || getActiveProject() || "Verdant Studio App").trim();
const sceneJson = JSON.stringify({
project: appTitle,
domain: getActiveDomainSlug(),
objects: getStudioObjects(),
lodRank: getLodIdx(),
});
const res = await callWorker("vcs-commit-pack-holo", {
request: { appTitle, authorPrismId, message, parentCommitKappa, epoch, version: "2.4.0", sceneJson },
});
const out = res?.result;
if (!res?.ok || !out?.ok) throw new Error(out?.error || "VerdantVcs commit failed");
if (out.holoHex) {
try {
const gzCap = await gzipHoloCapsuleIfSmaller(hexToBytes(out.holoHex));
if (gzCap) {
out.holoHex = bytesToHex(gzCap.bytes);
out.capsuleByteLen = gzCap.bytes.byteLength;
out.capsuleSha256Hex = gzCap.sha;
out.stitchDeepLinkUri = `stitch://${gzCap.sha}/studio/index.html`;
}
} catch {}
}
lastPackedHolo = out;
if (out.holoHex) {
try {
await cacheBytesInOpfs(`verdant-app-${out.capsuleSha256Hex}.holo`, hexToBytes(out.holoHex));
} catch {}
}
vcsHistory.unshift({
epoch: out.epoch,
commitKappa: out.commitKappa,
parentCommitKappa: out.parentCommitKappa || null,
capsuleSha256Hex: out.capsuleSha256Hex,
stitchDeepLinkUri: out.stitchDeepLinkUri,
message: out.message,
sceneJson,
holoHex: out.holoHex || "",
});
vcsHistory = vcsHistory.slice(0, 12);
try {
localStorage.setItem(VCS_STORAGE_KEY, JSON.stringify(vcsHistory));
} catch {}
renderVcsHistory();
chip(
statusEl,
`VerdantVcs e${out.epoch} (${out.commitKappa.slice(0, 10)}…) · .holo ${out.capsuleByteLen} B · ${out.shardCount} shards`,
);
return out;
};
const stepPagedUhdWorkload = async (workloadReq = {}) => {
const res = await callWorker("paged-workload-step", { request: workloadReq });
const out = res?.result;
if (out?.ok && scaleBadgeEl) {
const tier = out.decision?.displayClass || "UHD";
const pages = out.arena?.residentPages ?? 0;
const evicts = out.arena?.evictions ?? 0;
scaleBadgeEl.dataset.contract = out.contractMet ? "met" : "adapting";
scaleBadgeEl.textContent = `${tier} · ${pages}p (${evicts} evict) · DRS ${Math.round((out.decision?.drsScalePermille || 1000) / 10)}%`;
}
return out;
};
$("studio-vcs-commit-btn")?.addEventListener("click", async () => {
try {
await commitAndPackHolo();
} catch (err) {
chip(statusEl, `VerdantVcs error: ${err}`);
}
});
$("studio-export-holo-btn")?.addEventListener("click", async () => {
try {
const packed = lastPackedHolo || (await commitAndPackHolo("Portable .holo app export"));
const bytes = hexToBytes(packed.holoHex || "");
const slug = String(stitchAppTitleInput?.value || "verdant-app")
.toLowerCase()
.replace(/[^a-z0-9]+/g, "-");
const url = URL.createObjectURL(new Blob([bytes], { type: "application/octet-stream" }));
const a = document.createElement("a");
a.href = url;
a.download = `${slug}.holo`;
a.click();
setTimeout(() => URL.revokeObjectURL(url), 1000);
chip(
statusEl,
`Exported portable ${slug}.holo (${bytes.byteLength} B · SHA-256 ${packed.capsuleSha256Hex.slice(0, 12)}…)`,
);
} catch (err) {
chip(statusEl, `Export .holo error: ${err}`);
}
});
$("studio-copy-pin-btn")?.addEventListener("click", async () => {
try {
const packed = lastPackedHolo || (await commitAndPackHolo("Deep-link pin share"));
const frameUrl = buildShareablePinUrl(packed);
try {
await navigator.clipboard.writeText(frameUrl);
} catch {}
chip(statusEl, `Copied cross-browser Blank Frame pin: ${packed.stitchDeepLinkUri}`);
} catch (err) {
chip(statusEl, `Pin share error: ${err}`);
}
});
renderVcsHistory();
void stepPagedUhdWorkload({
viewportW: Math.round((canvas?.clientWidth || 1920) * (window.devicePixelRatio || 1)),
viewportH: Math.round((canvas?.clientHeight || 1080) * (window.devicePixelRatio || 1)),
frameUs: 8000,
budgetMb: 8,
gltfPages: 16,
codedAiPages: 16,
pageKb: 64,
}).catch(() => {});
return {
commitAndPackHolo,
checkoutVcsCommit,
buildShareablePinUrl,
stepPagedUhdWorkload,
getVcsHistory: () => vcsHistory.slice(),
getLocalPrismId: ensureLocalStudioPrismId,
};
}
