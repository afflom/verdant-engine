const PRIMITIVE_DEFAULTS = [
{ scale: [1.6, 1.6, 1.6], color: [0.86, 0.34, 0.16] }, // Box (Terracotta)
{ scale: [2.2, 1.0, 2.2], color: [0.84, 0.82, 0.76] }, // Platform (Stone)
{ scale: [1.2, 2.2, 1.2], color: [2.84, 0.82, 0.76] }, // Pillar (Metallic)
{ scale: [2.0, 1.5, 2.6], color: [0.26, 0.3, 0.36] }, // Ramp (Basalt)
{ scale: [1.2, 1.2, 1.2], color: [3.24, 0.66, 0.86] }, // Beacon Orb (Emissive Cyan)
];
export function initStudioOutliner(ctx) {
const {
studioObjCountEl,
studioOutlinerEl,
objInspCard,
objInspTitle,
objPosX,
objPosY,
objPosZ,
objSclX,
objSclY,
objSclZ,
objYawDeg,
objMatSurface,
objSwatches,
studioClearObjsBtn,
objInspDelBtn,
statusEl,
callWorker,
chip,
currentLodSpec,
getSimClockSec,
getStudioObjects,
setStudioObjects,
getSelectedStudioObjId,
setSelectedStudioObjId,
setRightRailTab,
setUserSelectedDomain,
setActiveStageAsset,
setActiveProject,
setActiveSkyFog,
setTris,
setCameraState,
syncTemplatePills,
syncFallbackBuffers,
syncHud,
requestRender,
} = ctx;
function renderStudioOutliner() {
const studioObjects = getStudioObjects();
const selectedStudioObjId = getSelectedStudioObjId();
if (studioObjCountEl) {
studioObjCountEl.textContent = `${studioObjects.length} ${studioObjects.length === 1 ? "Object" : "Objects"}`;
}
if (!studioOutlinerEl) return;
studioOutlinerEl.replaceChildren();
if (!studioObjects.length) {
const empty = document.createElement("div");
empty.className = "proj-empty";
empty.innerHTML =
"Click <strong>+ Box</strong>, <strong>+ Platform</strong>, or <strong>+ Pillar</strong> to place 3D objects in your scene.";
studioOutlinerEl.append(empty);
if (objInspCard) objInspCard.hidden = true;
return;
}
for (const o of studioObjects) {
const row = document.createElement("div");
row.className = "vx-outliner-row";
row.dataset.studioObjId = String(o.id);
row.dataset.selected = o.id === selectedStudioObjId ? "true" : "false";
const sel = document.createElement("button");
sel.type = "button";
sel.className = "vx-outliner-sel";
sel.textContent = o.label || `Object #${o.id}`;
sel.addEventListener("click", () => selectStudioObject(o.id));
const del = document.createElement("button");
del.type = "button";
del.className = "vx-outliner-del";
del.setAttribute("aria-label", `Remove ${o.label}`);
del.textContent = "×";
del.addEventListener("click", () => removeStudioObject(o.id));
row.append(sel, del);
studioOutlinerEl.append(row);
}
}
function selectStudioObject(objId) {
setSelectedStudioObjId(objId);
renderStudioOutliner();
const obj = getStudioObjects().find((o) => o.id === objId);
if (!obj || !objInspCard) {
if (objInspCard) objInspCard.hidden = true;
return;
}
setRightRailTab("inspector");
objInspCard.hidden = false;
if (objInspTitle) objInspTitle.textContent = obj.label || `Object #${obj.id}`;
if (objPosX) objPosX.value = String(Number(obj.pos[0]).toFixed(2));
if (objPosY) objPosY.value = String(Number(obj.pos[1]).toFixed(2));
if (objPosZ) objPosZ.value = String(Number(obj.pos[2]).toFixed(2));
if (objSclX) objSclX.value = String(Number(obj.scale[0]).toFixed(2));
if (objSclY) objSclY.value = String(Number(obj.scale[1]).toFixed(2));
if (objSclZ) objSclZ.value = String(Number(obj.scale[2]).toFixed(2));
if (objYawDeg) objYawDeg.value = String(Math.round(((obj.yawMilli || 0) / 1000) * (180 / Math.PI)));
const matClass = Math.floor(obj.color[0] || 0);
if (objMatSurface) objMatSurface.value = String(matClass === 2 || matClass === 3 ? matClass : 0);
const baseR = (obj.color[0] || 0) - matClass;
const baseG = obj.color[1] || 0;
const baseB = obj.color[2] || 0;
objSwatches?.querySelectorAll("[data-swatch]").forEach((btn) => {
const [sr, sg, sb] = btn.dataset.swatch.split(",").map(Number);
const match = Math.abs(sr - baseR) + Math.abs(sg - baseG) + Math.abs(sb - baseB) < 0.12;
btn.setAttribute("aria-pressed", match ? "true" : "false");
});
}
async function loadBlankStudioStage() {
setUserSelectedDomain(true);
setActiveStageAsset(null);
setActiveProject("Blank Studio Canvas");
setActiveSkyFog([0.16, 0.17, 0.19], [0.22, 0.23, 0.26]);
syncTemplatePills("blank");
document.querySelectorAll("[data-wiz-domain-idx]").forEach((c) => c.setAttribute("aria-pressed", "false"));
const r = await callWorker("studio-load-blank", { lod: currentLodSpec(), simClockSec: getSimClockSec() });
if (r?.ok) {
if (r.tris) setTris(r.tris);
if (r.camera) setCameraState(r.camera);
if (Array.isArray(r.objects)) setStudioObjects(r.objects);
await syncFallbackBuffers();
}
renderStudioOutliner();
syncHud();
chip(statusEl, "Blank Studio Canvas ready — click + Box, + Platform, or + Pillar to start building.");
requestRender();
}
async function addStudioPrimitive(primKind) {
const idx = getStudioObjects().length;
const ring = Math.floor(idx / 6);
const angle = (idx % 6) * (Math.PI / 3);
const radius = idx === 0 ? 0 : 2.8 + ring * 2.4;
const px = Number((Math.cos(angle) * radius).toFixed(2));
const pz = Number((Math.sin(angle) * radius).toFixed(2));
const defaults = PRIMITIVE_DEFAULTS[primKind] || { scale: [1.5, 1.5, 1.5], color: [0.86, 0.34, 0.16] };
const r = await callWorker("studio-add-object", {
primKind,
pos: [px, 0, pz],
scale: defaults.scale,
color: defaults.color,
lod: currentLodSpec(),
simClockSec: getSimClockSec(),
});
if (r?.ok) {
if (r.tris) setTris(r.tris);
if (Array.isArray(r.objects)) setStudioObjects(r.objects);
await syncFallbackBuffers();
selectStudioObject(r.objId);
syncHud();
chip(statusEl, `Added object #${r.objId} to your scene (${getStudioObjects().length} total).`);
requestRender();
}
}
async function updateSelectedStudioObject(overrideRgb = null) {
const selectedStudioObjId = getSelectedStudioObjId();
if (!selectedStudioObjId) return;
const obj = getStudioObjects().find((o) => o.id === selectedStudioObjId);
if (!obj) return;
const matClass = Number(objMatSurface?.value || 0);
const activeSwatch = objSwatches?.querySelector('[data-swatch][aria-pressed="true"]');
const [sr, sg, sb] =
overrideRgb || (activeSwatch ? activeSwatch.dataset.swatch.split(",").map(Number) : [0.86, 0.34, 0.16]);
const pos = [Number(objPosX?.value) || 0, Number(objPosY?.value) || 0, Number(objPosZ?.value) || 0];
const scale = [
Math.max(0.25, Number(objSclX?.value) || 1),
Math.max(0.25, Number(objSclY?.value) || 1),
Math.max(0.25, Number(objSclZ?.value) || 1),
];
const yawMilli = Math.round(((Number(objYawDeg?.value) || 0) * Math.PI / 180) * 1000);
const color = [matClass + sr, sg, sb];
const r = await callWorker("studio-update-object", {
objId: selectedStudioObjId,
pos,
yawMilli,
scale,
color,
lod: currentLodSpec(),
simClockSec: getSimClockSec(),
});
if (r?.ok) {
if (r.tris) setTris(r.tris);
if (Array.isArray(r.objects)) setStudioObjects(r.objects);
await syncFallbackBuffers();
syncHud();
requestRender();
}
}
async function removeStudioObject(objId) {
const r = await callWorker("studio-remove-object", { objId, lod: currentLodSpec(), simClockSec: getSimClockSec() });
if (r?.ok) {
if (r.tris) setTris(r.tris);
const nextObjs = Array.isArray(r.objects) ? r.objects : getStudioObjects();
setStudioObjects(nextObjs);
if (getSelectedStudioObjId() === objId) setSelectedStudioObjId(nextObjs[0]?.id ?? null);
await syncFallbackBuffers();
renderStudioOutliner();
const nextSel = getSelectedStudioObjId();
if (nextSel) selectStudioObject(nextSel);
syncHud();
requestRender();
}
}
async function clearStudioObjects() {
const r = await callWorker("studio-clear-objects", { lod: currentLodSpec(), simClockSec: getSimClockSec() });
if (r?.ok) {
if (r.tris) setTris(r.tris);
setStudioObjects([]);
setSelectedStudioObjId(null);
await syncFallbackBuffers();
renderStudioOutliner();
syncHud();
requestRender();
}
}
document.querySelectorAll("[data-add-prim]").forEach((btn) => {
btn.addEventListener("click", () => addStudioPrimitive(Number(btn.dataset.addPrim)));
});
studioClearObjsBtn?.addEventListener("click", clearStudioObjects);
objInspDelBtn?.addEventListener("click", () => {
const sel = getSelectedStudioObjId();
if (sel) removeStudioObject(sel);
});
[objPosX, objPosY, objPosZ, objSclX, objSclY, objSclZ, objYawDeg, objMatSurface].forEach((el) => {
el?.addEventListener("input", () => updateSelectedStudioObject());
el?.addEventListener("change", () => updateSelectedStudioObject());
});
objSwatches?.querySelectorAll("[data-swatch]").forEach((btn) => {
btn.addEventListener("click", () => {
objSwatches.querySelectorAll("[data-swatch]").forEach((b) => b.setAttribute("aria-pressed", "false"));
btn.setAttribute("aria-pressed", "true");
updateSelectedStudioObject(btn.dataset.swatch.split(",").map(Number));
});
});
return {
renderStudioOutliner,
selectStudioObject,
loadBlankStudioStage,
addStudioPrimitive,
updateSelectedStudioObject,
removeStudioObject,
clearStudioObjects,
};
}
