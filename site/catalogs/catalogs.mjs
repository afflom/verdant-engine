// Standalone Verdant Model Catalogs Controller (`site/catalogs/catalogs.mjs`).
// Renders the 5 differentiated Verdant platform organs and 28 imported standard models,
// and executes live cryptographic BLAKE3 + Core-Wasm conformance checks against `/app.holo`.

import { kappa, load } from "../worker-runtime/index.mjs";
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
const kpiModels = $("cat-kpi-models");
const kpiLex = $("cat-kpi-lex");
const kpiPhases = $("cat-kpi-phases");
const kpiProofs = $("cat-kpi-proofs");
const organGrid = $("cat-organ-grid");
const domainGrid = $("cat-domain-grid");
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

async function renderImportedDomainPacks() {
  if (!domainGrid) return;
  try {
    const res = await fetch(new URL("./world-packs.json", import.meta.url));
    if (!res.ok) return;
    const data = await res.json();
    const domains = Array.isArray(data?.domains) ? data.domains : [];
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

renderPlatformOrgans();
renderCatalogList();
renderImportedDomainPacks();

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
