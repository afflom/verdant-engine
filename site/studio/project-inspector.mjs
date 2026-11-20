export function initStudioProjectInspector(ctx) {
const {
projectTreeEl,
projModelCountEl,
inspEmptyState,
inspEditor,
inspMetaEl,
inspTitleEl,
inspTagsEl,
inspPropsEl,
inspNoteEl,
inspSaveBtn,
inspApplyBtn,
inspRemoveBtn,
inspRevisionsEl,
statusEl,
callWorker,
chip,
syncHud,
setRightRailTab,
getProjectModels,
setProjectModels,
getSelectedProjectModelId,
setSelectedProjectModelId,
wfState,
syncWorkflowRibbon,
applyProjectModelLive,
} = ctx;
function renderProjectTree() {
if (!projectTreeEl) return;
const projectModels = getProjectModels();
const selectedProjectModelId = getSelectedProjectModelId();
if (projModelCountEl) projModelCountEl.textContent = `${projectModels.length} Imported`;
syncHud();
if (projectModels.length === 0) {
projectTreeEl.innerHTML =
`<div class="proj-empty">No external catalog models imported yet. Search live streams in the center Discovery panel and click <strong>+ Add</strong> to admit, edit, and version models inside Verdant Engine.</div>`;
return;
}
const bySlot = new Map();
for (const m of projectModels) {
if (!bySlot.has(m.targetSlot)) bySlot.set(m.targetSlot, []);
bySlot.get(m.targetSlot).push(m);
}
projectTreeEl.replaceChildren();
for (const [slot, list] of bySlot.entries()) {
const group = document.createElement("div");
group.className = "proj-slot-group";
const head = document.createElement("div");
head.className = "proj-slot-heading";
head.textContent = `${slot} (${list.length})`;
group.append(head);
for (const m of list) {
const row = document.createElement("div");
row.className = "proj-model-row";
row.dataset.projectModelId = m.modelId;
row.dataset.selected = m.modelId === selectedProjectModelId ? "true" : "false";
const pick = document.createElement("button");
pick.type = "button";
pick.className = "proj-model-pick";
const nm = document.createElement("strong");
nm.textContent = m.title;
const meta = document.createElement("small");
meta.textContent = `v${m.activeVersion} · ${m.schemaType} · ${m.currentKappaHex.slice(0, 8)}`;
pick.append(nm, meta);
pick.addEventListener("click", () => selectProjectModel(m.modelId));
const rm = document.createElement("button");
rm.type = "button";
rm.className = "st-btn danger sm";
rm.setAttribute("aria-label", `Remove ${m.title}`);
rm.textContent = "×";
rm.addEventListener("click", () => removeProjectModel(m.modelId));
row.append(pick, rm);
group.append(row);
}
projectTreeEl.append(group);
}
}
function renderInspector() {
const model = getProjectModels().find((m) => m.modelId === getSelectedProjectModelId()) || null;
if (!model) {
if (inspEmptyState) inspEmptyState.hidden = false;
if (inspEditor) inspEditor.hidden = true;
return;
}
if (inspEmptyState) inspEmptyState.hidden = true;
if (inspEditor) inspEditor.hidden = false;
if (inspMetaEl) {
inspMetaEl.textContent =
`ID: ${model.modelId}\n` +
`Type: Schema.org/${model.schemaType} · ${model.mediaType}\n` +
`Auto-Slot: ${model.targetSlot} · Stream: ${model.sourceStream}\n` +
`License: ${model.licenseSpdx} · Active: v${model.activeVersion} (${model.currentKappaHex.slice(0, 14)}…)`;
}
if (inspTitleEl) inspTitleEl.value = model.title;
if (inspTagsEl) inspTagsEl.value = (model.tags || []).join(", ");
if (inspPropsEl) {
try {
inspPropsEl.value = JSON.stringify(JSON.parse(model.propertiesJson || "{}"), null, 2);
} catch {
inspPropsEl.value = model.propertiesJson || "{}";
}
}
if (inspNoteEl) inspNoteEl.value = "";
if (inspSaveBtn) inspSaveBtn.textContent = `Save Version (v${model.activeVersion + 1})`;
if (inspRevisionsEl) {
inspRevisionsEl.replaceChildren();
const revs = [...(model.revisions || [])].reverse();
for (const rev of revs) {
const card = document.createElement("div");
card.className = "insp-rev-item";
card.dataset.revVersion = String(rev.version);
const top = document.createElement("div");
top.className = "insp-rev-top";
const badge = document.createElement("strong");
badge.textContent = `v${rev.version} · ${rev.kappaHex.slice(0, 12)}…`;
top.append(badge);
if (rev.version !== model.activeVersion) {
const restoreBtn = document.createElement("button");
restoreBtn.type = "button";
restoreBtn.className = "st-btn sm";
restoreBtn.dataset.revertVersion = String(rev.version);
restoreBtn.textContent = `Restore v${rev.version}`;
restoreBtn.addEventListener("click", () => revertProjectModel(model.modelId, rev.version));
top.append(restoreBtn);
} else {
const cur = document.createElement("span");
cur.style.color = "var(--success)";
cur.textContent = "Active";
top.append(cur);
}
const note = document.createElement("span");
note.style.color = "var(--muted-foreground)";
note.textContent = rev.commitNote || rev.title;
card.append(top, note);
inspRevisionsEl.append(card);
}
}
}
function selectProjectModel(modelId) {
setSelectedProjectModelId(modelId);
setRightRailTab("inspector");
renderProjectTree();
renderInspector();
}
async function refreshProjectModelsFromWasm() {
const r = await callWorker("list-project-models");
if (r?.ok && Array.isArray(r.models)) {
setProjectModels(r.models);
const curSel = getSelectedProjectModelId();
if (curSel && !r.models.some((m) => m.modelId === curSel)) {
setSelectedProjectModelId(r.models[0]?.modelId || null);
}
renderProjectTree();
renderInspector();
}
}
async function importCatalogItemToProject(item) {
if (!item) return null;
const request = {
modelId: item.id,
title: item.title,
schemaType: item.schemaType,
mediaType: item.mediaType,
targetSlot: item.targetSlot,
sourceStream: item.streamId,
sourceUri: item.sourceUri || `urn:verdant:catalog:${item.id}`,
licenseSpdx: item.licenseSpdx || "Apache-2.0",
tags: item.tags || [],
propertiesJson: JSON.stringify(item.properties || {}),
commitNote: `Imported from ${item.streamLabel} (${item.schemaType} -> ${item.targetSlot})`,
};
const r = await callWorker("import-project-model", { request });
if (!r?.ok || !r.model) {
chip(statusEl, `Refused model import (${item.title}) — Schema.org / DMN gate`);
return null;
}
await refreshProjectModelsFromWasm();
selectProjectModel(r.model.modelId);
wfState.done2 = true;
wfState.activeStep = 3;
syncWorkflowRibbon();
await applyProjectModelLive(r.model);
chip(
statusEl,
`Admitted "${r.model.title}" v${r.model.activeVersion} -> ${r.model.targetSlot} (kappa ${r.model.currentKappaHex.slice(0, 10)}…)`,
);
return r.model;
}
async function saveEditedProjectModel() {
const model = getProjectModels().find((m) => m.modelId === getSelectedProjectModelId());
if (!model) return;
const newTitle = (inspTitleEl?.value || "").trim() || model.title;
const newTags = (inspTagsEl?.value || "")
.split(",")
.map((s) => s.trim())
.filter(Boolean);
let normalizedProps = (inspPropsEl?.value || "{}").trim();
try {
normalizedProps = JSON.stringify(JSON.parse(normalizedProps));
} catch {
chip(statusEl, "Invalid JSON in properties editor — fix JSON syntax before committing version");
return;
}
const commitNote = (inspNoteEl?.value || "").trim() || `Edited properties for v${model.activeVersion + 1}`;
const r = await callWorker("edit-project-model", {
request: {
modelId: model.modelId,
title: newTitle,
tags: newTags,
propertiesJson: normalizedProps,
commitNote,
},
});
if (!r?.ok || !r.model) {
chip(statusEl, "Failed to commit edited model version in Wasm");
return;
}
await refreshProjectModelsFromWasm();
selectProjectModel(r.model.modelId);
wfState.done3 = true;
wfState.activeStep = 4;
syncWorkflowRibbon();
chip(statusEl, `Committed "${r.model.title}" v${r.model.activeVersion} (kappa ${r.model.currentKappaHex.slice(0, 10)}…)`);
}
async function revertProjectModel(modelId, targetVersion) {
const r = await callWorker("revert-project-model", { modelId, targetVersion });
if (!r?.ok || !r.model) {
chip(statusEl, `Failed to restore v${targetVersion}`);
return;
}
await refreshProjectModelsFromWasm();
selectProjectModel(r.model.modelId);
wfState.done2 = true;
wfState.activeStep = 3;
syncWorkflowRibbon();
await applyProjectModelLive(r.model);
chip(statusEl, `Restored v${targetVersion} as new audited revision v${r.model.activeVersion}`);
}
async function removeProjectModel(modelId) {
const r = await callWorker("remove-project-model", { modelId });
if (!r?.ok) {
chip(statusEl, `Could not remove ${modelId}`);
return;
}
await refreshProjectModelsFromWasm();
chip(statusEl, `Removed model ${modelId} from project (PROV-O logged)`);
}
inspSaveBtn?.addEventListener("click", saveEditedProjectModel);
inspApplyBtn?.addEventListener("click", () => {
const m = getProjectModels().find((x) => x.modelId === getSelectedProjectModelId());
if (m) applyProjectModelLive(m);
});
inspRemoveBtn?.addEventListener("click", () => {
const sel = getSelectedProjectModelId();
if (sel) removeProjectModel(sel);
});
return {
renderProjectTree,
renderInspector,
selectProjectModel,
refreshProjectModelsFromWasm,
importCatalogItemToProject,
saveEditedProjectModel,
revertProjectModel,
removeProjectModel,
};
}
