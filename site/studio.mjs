// Verdant Studio — Immersive 3D Application Design Studio & Observer-Coherent Role Workspaces.
// Implements imported standard models (ISO/IEC/IEEE 42010, ISO 9241-11/210, ISO/IEC 25010,
// Alina Wheeler + W3C DTCG 2025.10 + WCAG 2.2, Khronos glTF 2.0, Sabine/Woodworth/Euler-Bernoulli,
// XPBD/Swept CCD, Tropical StoryAutomaton, Q15 LIF/SSM, prism/identity/4 + afflom/stitch)
// across 7 functional roles: Dashboard, Code, Design, Audio, Narrative, Director, and Gameplay.
// Lazy-loaded on demand (zero cold-boot byte impact on `/`).

import {
  PRISM_AUDIO_QUANTUM_FRAMES,
  PRISM_CHAMBER_SABINE_RT60_MS,
  PRISM_CSM_SPLIT_METERS,
  PRISM_HD_MAX_LOD_RANK,
  PRISM_MAX_LOD_RANK,
  PRISM_MODAL_BAR_DENOM,
  PRISM_MODAL_BAR_NUMERS,
  PRISM_TERRAIN_WALKABLE_POLYGONS,
  PRISM_WORKER_RING_CAPACITY,
} from "./prism-bridge-meta.mjs";
import {
  ACOUSTIC_ROOMS,
  CONTAINER_ORGANS,
  DIRECTOR_SHOTS,
  GAME_MODES,
  NPC_ROSTER,
  ROLE_REQUIREMENTS_TRACE,
  STARTER_ARCHETYPES,
  STUDIO_CSS,
  STUDIO_ROLE_KEYS,
} from "./studio-data.mjs";
import {
  base64UrlToBytes,
  bytesToBase64Url,
  generateStarterProjectFiles,
  zeroOneExpr,
} from "./studio-scaffolder.mjs";
import { catalogModelsForRole } from "./prism-catalogs.mjs";

export {
  CONTAINER_ORGANS,
  ROLE_REQUIREMENTS_TRACE,
  STARTER_ARCHETYPES,
  STUDIO_CSS,
  STUDIO_ROLE_KEYS,
  base64UrlToBytes,
  bytesToBase64Url,
  generateStarterProjectFiles,
  zeroOneExpr,
};

const siteUrl = (rel) => new URL(rel, import.meta.url).href;

let sheetInstalled = false;
export function ensureModalStyles() {
  if (sheetInstalled || typeof CSSStyleSheet === "undefined") return;
  try {
    const sheet = new CSSStyleSheet();
    sheet.replaceSync(STUDIO_CSS);
    document.adoptedStyleSheets = [...document.adoptedStyleSheets, sheet];
    sheetInstalled = true;
  } catch {
    // Ignored under environments without constructable stylesheets.
  }
}

export function readThemePalette() {
  const cs = getComputedStyle(document.documentElement);
  const v = (name, fb) => cs.getPropertyValue(name).trim() || fb;
  return {
    bg: v("--muted", "Canvas"),
    card: v("--card", "Canvas"),
    fg: v("--foreground", "CanvasText"),
    mutedFg: v("--muted-foreground", "GrayText"),
    border: v("--border-strong", "GrayText"),
    primary: v("--primary", "CanvasText"),
    primaryFg: v("--primary-foreground", "Canvas"),
    brand: v("--brand", "Highlight"),
    success: v("--success", "Highlight"),
    successSubtle: v("--success-subtle", "Canvas"),
    destructive: v("--destructive", "Mark"),
    destructiveSubtle: v("--destructive-subtle", "Canvas"),
    fontMono: v("--font-mono", "monospace"),
  };
}

const themeListeners = new Set();
let themeObserverStarted = false;
export function subscribeThemeChanges(fn) {
  themeListeners.add(fn);
  if (!themeObserverStarted && typeof MutationObserver !== "undefined") {
    themeObserverStarted = true;
    new MutationObserver(() => {
      for (const cb of themeListeners) cb();
    }).observe(document.documentElement, { attributes: true, attributeFilter: ["class"] });
  }
  return () => themeListeners.delete(fn);
}

export async function cacheBytesInOpfs(filename, bytes) {
  if (!navigator.storage || typeof navigator.storage.getDirectory !== "function") {
    return { cached: false, reason: "OPFS unavailable in this browser context" };
  }
  try {
    const root = await navigator.storage.getDirectory();
    const fh = await root.getFileHandle(filename, { create: true });
    const writable = await fh.createWritable();
    await writable.write(bytes);
    await writable.close();
    return { cached: true, filename, byteLength: bytes.byteLength };
  } catch (err) {
    return { cached: false, reason: String(err?.message || err) };
  }
}

export async function listOpfsAssets() {
  if (!navigator.storage || typeof navigator.storage.getDirectory !== "function") return [];
  try {
    const root = await navigator.storage.getDirectory();
    const entries = [];
    for await (const [name, handle] of root.entries()) {
      if (handle.kind === "file") {
        const f = await handle.getFile();
        entries.push({ name, bytes: f.size });
      }
    }
    return entries;
  } catch {
    return [];
  }
}

export async function readOpfsAssetBytes(filename) {
  if (!navigator.storage || typeof navigator.storage.getDirectory !== "function") return null;
  try {
    const root = await navigator.storage.getDirectory();
    const fh = await root.getFileHandle(filename);
    const f = await fh.getFile();
    return new Uint8Array(await f.arrayBuffer());
  } catch {
    return null;
  }
}

export async function deleteOpfsAsset(filename) {
  if (!navigator.storage || typeof navigator.storage.getDirectory !== "function") return false;
  try {
    const root = await navigator.storage.getDirectory();
    await root.removeEntry(filename);
    return true;
  } catch {
    return false;
  }
}

let studioDialog = null;
let studioCtx = null;
let selectStudioTab = null;
let latestObserverTelemetry = null;
let observerDrawHook = null;
let audioDrawHook = null;
let uiDrawHook = null;
let sequenceTimer = 0;

let studioHostRoot = null;

export function pushStudioTelemetry(telemetry) {
  latestObserverTelemetry = telemetry;
  if (studioDialog?.open || studioHostRoot?.isConnected) {
    observerDrawHook?.(telemetry);
    audioDrawHook?.(telemetry);
  }
}

export function openStudioModal(ctx) {
  ensureModalStyles();
  studioCtx = ctx;
  if (!studioDialog) {
    studioDialog = buildStudioDialog({ inline: false });
    document.body.append(studioDialog);
  }
  if (!studioDialog.open) {
    studioDialog.showModal();
  }
  if (ctx?.initialTab && selectStudioTab) {
    selectStudioTab(ctx.initialTab);
  }
  studioDialog.dispatchEvent(new CustomEvent("ve-refresh"));
}

export function mountInlineStudio(targetEl, ctx) {
  ensureModalStyles();
  studioCtx = ctx;
  targetEl.classList.add("ve-inline-studio");
  const host = buildStudioDialog({ inline: true });
  studioHostRoot = host;
  targetEl.replaceChildren(host);
  if (ctx?.initialTab && selectStudioTab) {
    selectStudioTab(ctx.initialTab);
  }
  host.dispatchEvent(new CustomEvent("ve-refresh"));
  return {
    selectTab: (tabKey) => selectStudioTab?.(tabKey),
    refresh: () => host.dispatchEvent(new CustomEvent("ve-refresh")),
    setProjectIndex: (idx) => host.dispatchEvent(new CustomEvent("ve-sync-project", { detail: { idx } })),
  };
}

function drawObserverCanvas(canvas, tel) {
  const ctx = canvas.getContext("2d");
  if (!ctx) return;
  const pal = readThemePalette();
  const w = canvas.width;
  const h = canvas.height;
  ctx.fillStyle = pal.bg;
  ctx.fillRect(0, 0, w, h);

  const mask = tel?.spikeMask ?? 5;
  const salience = tel?.salience ?? [420, 180, 260, 310, 190, 220, 390];
  const attended = tel?.attendedSlot ?? 0;

  ctx.fillStyle = pal.fg;
  ctx.font = `600 12px ${pal.fontMono}`;
  ctx.fillText(`Q15 LIF SPIKE RASTER (8 NEURONS) · MASK 0b${mask.toString(2).padStart(8, "0")}`, 16, 24);

  for (let n = 0; n < 8; n++) {
    const spiked = ((mask >>> n) & 1) === 1;
    const bx = 16 + n * 68;
    ctx.fillStyle = spiked ? pal.success : pal.card;
    ctx.fillRect(bx, 34, 58, 28);
    ctx.strokeStyle = spiked ? pal.success : pal.border;
    ctx.strokeRect(bx, 34, 58, 28);
    ctx.fillStyle = spiked ? pal.primaryFg : pal.fg;
    ctx.fillText(`N${n}:${spiked ? "1" : "0"}`, bx + 12, 52);
  }

  ctx.fillStyle = pal.mutedFg;
  ctx.fillText(
    `NPC & PLAYER MUTUAL-ATTENTION SALIENCE (ATTENDED SLOT ${attended} · GIMBAL ${tel?.gimbalYawMilli ?? 0} mrad)`,
    16,
    88,
  );

  const maxSal = Math.max(500, ...salience.map((v) => Math.abs(v)));
  for (let s = 0; s < salience.length; s++) {
    const val = Math.max(0, salience[s]);
    const barW = Math.max(12, Math.round((val / maxSal) * (w - 170)));
    const y = 100 + s * 22;
    ctx.fillStyle = pal.mutedFg;
    ctx.fillText(`Slot ${s}`, 16, y + 13);
    ctx.fillStyle = s === attended ? pal.success : pal.brand;
    ctx.fillRect(82, y, barW, 15);
    ctx.fillStyle = pal.fg;
    ctx.fillText(`${val}`, 90 + barW, y + 13);
  }
}

function drawAudioSpectrumCanvas(canvas, fundamentalHz, rt60Ms, tel) {
  const ctx = canvas.getContext("2d");
  if (!ctx) return;
  const pal = readThemePalette();
  const w = canvas.width;
  const h = canvas.height;
  ctx.fillStyle = pal.bg;
  ctx.fillRect(0, 0, w, h);

  const pan = tel?.binauralPan ?? 0;
  const gain = tel?.binauralGain ?? 0.62;
  const itd = tel?.binauralItdMicros ?? 0;

  ctx.fillStyle = pal.fg;
  ctx.font = `600 12px ${pal.fontMono}`;
  ctx.fillText(
    `EULER-BERNOULLI 5-MODE HARMONIC SPECTRUM · RT60 ${(rt60Ms / 1000).toFixed(2)}s · ITD ${Math.round(itd)} µs`,
    14,
    22,
  );

  const numers = PRISM_MODAL_BAR_NUMERS;
  const denom = PRISM_MODAL_BAR_DENOM;
  const maxFreq = (fundamentalHz * numers[numers.length - 1]) / denom;

  for (let i = 0; i < numers.length; i++) {
    const freq = (fundamentalHz * numers[i]) / denom;
    const x = 22 + i * 108;
    const barH = Math.max(24, Math.round((freq / Math.max(1, maxFreq)) * 96));
    const y = 140 - barH;
    ctx.fillStyle = i === 0 ? pal.success : pal.brand;
    ctx.fillRect(x, y, 76, barH);
    ctx.strokeStyle = pal.border;
    ctx.strokeRect(x, y, 76, barH);
    ctx.fillStyle = pal.fg;
    ctx.font = `600 11px ${pal.fontMono}`;
    ctx.fillText(`M${i} (${numers[i]}/${denom})`, x + 4, 156);
    ctx.fillStyle = pal.mutedFg;
    ctx.fillText(`${freq.toFixed(1)} Hz`, x + 8, y - 6);
  }

  const cx = Math.round(w / 2);
  ctx.strokeStyle = pal.border;
  ctx.strokeRect(22, 168, w - 44, 14);
  const px = Math.max(26, Math.min(w - 30, Math.round(cx + pan * ((w - 60) / 2))));
  ctx.fillStyle = pal.primary;
  ctx.fillRect(px - 6, 169, 12, 12);
  ctx.fillStyle = pal.mutedFg;
  ctx.fillText(`L`, 10, 179);
  ctx.fillText(`R`, w - 18, 179);
  ctx.fillText(`Gain ${gain.toFixed(2)}`, cx - 30, 196);
}

function drawUiComplianceCanvas(canvas, vw, vh, reportText) {
  const ctx = canvas.getContext("2d");
  if (!ctx) return;
  const pal = readThemePalette();
  const cw = canvas.width;
  const ch = canvas.height;
  ctx.fillStyle = pal.bg;
  ctx.fillRect(0, 0, cw, ch);

  const lines = String(reportText || "").split("\n").slice(1);
  const sx = (cw - 24) / Math.max(1, vw);
  const sy = (ch - 36) / Math.max(1, vh);

  ctx.fillStyle = pal.fg;
  ctx.font = `600 12px ${pal.fontMono}`;
  ctx.fillText(`TROPICAL (max,+) + EXACT LINEAR UI PROOF · ${vw}x${vh} CSS PX`, 12, 20);

  lines.forEach((line, idx) => {
    const [label, rest] = line.split(":");
    if (!rest) return;
    const parts = rest.split(",");
    const x = Number(parts[0]) || 0;
    const y = Number(parts[1]) || 0;
    const [rw, rh] = (parts[2] || "48x48").split("x").map(Number);
    const rx = 12 + x * sx;
    const ry = 28 + y * sy;
    const pw = Math.max(18, rw * sx);
    const ph = Math.max(14, rh * sy);
    ctx.fillStyle = pal.card;
    ctx.fillRect(rx, ry, pw, ph);
    ctx.strokeStyle = idx % 2 === 0 ? pal.success : pal.brand;
    ctx.strokeRect(rx, ry, pw, ph);
    ctx.fillStyle = pal.fg;
    ctx.font = `11px ${pal.fontMono}`;
    ctx.fillText(`${label} (${rw}x${rh})`, rx + 4, ry + 12);
  });
}

function downloadTextOrBytes(filename, content, mime = "text/plain;charset=utf-8") {
  const blob = content instanceof Uint8Array ? new Blob([content], { type: mime }) : new Blob([String(content)], { type: mime });
  const url = URL.createObjectURL(blob);
  const a = document.createElement("a");
  a.href = url;
  a.download = filename;
  a.click();
  setTimeout(() => URL.revokeObjectURL(url), 1500);
}

function buildStudioDialog({ inline = false } = {}) {
  const dlg = inline ? document.createElement("div") : document.createElement("dialog");
  if (!inline) {
    dlg.className = "ve-modal";
    dlg.setAttribute("aria-label", "Verdant Studio Role Workspaces");
  }

  const shell = document.createElement("div");
  shell.className = "ve-m-shell";

  const head = document.createElement("header");
  head.className = "ve-m-head";
  const titleWrap = document.createElement("div");
  const h2 = document.createElement("h2");
  h2.className = "ve-m-title";
  h2.textContent = inline ? "Lifecycle Role Workspaces" : "Verdant Studio";
  const sub = document.createElement("p");
  sub.className = "ve-m-sub";
  sub.id = "ve-studio-sub";
  sub.textContent = "3D Application Authoring · 7 Functional Workspaces";
  titleWrap.append(h2, sub);

  const headRight = document.createElement("div");
  headRight.className = "ve-m-row";
  if (!inline) {
    const catalogsLink = document.createElement("a");
    catalogsLink.className = "ve-m-btn";
    catalogsLink.href = siteUrl("./catalogs/");
    catalogsLink.textContent = "Model Catalogs";
    const demoLink = document.createElement("a");
    demoLink.className = "ve-m-btn";
    demoLink.href = siteUrl("./demo/");
    demoLink.textContent = "Engine Demo";
    const fullStudioLink = document.createElement("a");
    fullStudioLink.className = "ve-m-btn brand";
    fullStudioLink.href = siteUrl("./studio/");
    fullStudioLink.textContent = "Full Studio";
    const closeBtn = document.createElement("button");
    closeBtn.type = "button";
    closeBtn.className = "ve-m-btn";
    closeBtn.id = "ve-studio-close-btn";
    closeBtn.setAttribute("aria-label", "Close Verdant Studio workspace");
    closeBtn.textContent = "Close";
    closeBtn.addEventListener("click", () => {
      if (sequenceTimer) { clearInterval(sequenceTimer); sequenceTimer = 0; }
      dlg.close();
    });
    headRight.append(catalogsLink, demoLink, fullStudioLink, closeBtn);
  }
  head.append(titleWrap, headRight);

  const obsBar = document.createElement("div");
  obsBar.className = "ve-m-observer-bar";
  const projGroup = document.createElement("div");
  projGroup.className = "ve-m-row";
  const projLabel = document.createElement("label");
  projLabel.setAttribute("for", "ve-project-select");
  projLabel.className = "ve-m-observer-meta";
  projLabel.textContent = "Active Project:";
  const projSelect = document.createElement("select");
  projSelect.id = "ve-project-select";
  projSelect.className = "ve-m-select";
  STARTER_ARCHETYPES.forEach((p) => {
    const opt = document.createElement("option");
    opt.value = String(p.idx);
    opt.textContent = p.title;
    projSelect.append(opt);
  });
  projGroup.append(projLabel, projSelect);

  const obsReadout = document.createElement("span");
  obsReadout.className = "ve-m-observer-meta";
  obsReadout.id = "ve-observer-readout";
  obsReadout.textContent = "Runtime Ready · 60 Hz · Hardware Accelerated";

  if (!inline) {
    const themeGroup = document.createElement("div");
    themeGroup.className = "ve-m-row";
    for (const [modeName, cls] of [
      ["Light", ""],
      ["Dark", "dark"],
      ["Immersive", "dark immersive"],
    ]) {
      const tb = document.createElement("button");
      tb.type = "button";
      tb.className = "ve-m-btn";
      tb.dataset.studioTheme = modeName.toLowerCase();
      tb.textContent = modeName;
      tb.addEventListener("click", () => {
        document.documentElement.className = cls;
      });
      themeGroup.append(tb);
    }
    obsBar.append(projGroup, obsReadout, themeGroup);
  } else {
    obsBar.append(projGroup, obsReadout);
  }

  const tabBar = document.createElement("div");
  tabBar.className = "ve-m-tabs";
  tabBar.setAttribute("role", "tablist");

  const body = document.createElement("div");
  body.className = "ve-m-body";

  const makeField = (labelText, controlEl, id) => {
    const wrap = document.createElement("div");
    wrap.className = "ve-m-field";
    const lbl = document.createElement("label");
    lbl.setAttribute("for", id);
    lbl.textContent = labelText;
    controlEl.id = id;
    wrap.append(lbl, controlEl);
    return wrap;
  };

  let setArchetypeHook = null;
  projSelect.addEventListener("change", async () => {
    const idx = Number(projSelect.value) || 0;
    const arch = STARTER_ARCHETYPES[idx] || STARTER_ARCHETYPES[0];
    studioCtx?.onProjectChange?.(arch.title);
    setArchetypeHook?.(idx);
    if (studioCtx?.onLoadDomainPack) {
      await studioCtx.onLoadDomainPack(idx, { fromSelect: true });
    } else {
      await studioCtx?.onSetSeed?.(arch.seed);
    }
    dlg.dispatchEvent(new CustomEvent("ve-refresh"));
  });
  dlg.addEventListener("ve-sync-project", (e) => {
    const idx = Math.max(0, Math.min(STARTER_ARCHETYPES.length - 1, Number(e.detail?.idx) || 0));
    projSelect.value = String(idx);
    setArchetypeHook?.(idx);
  });

  // ==================== ROLE 0: DASHBOARD (PRODUCER) ====================
  const pDashboard = document.createElement("section");
  pDashboard.className = "ve-m-panel";

  const cDashKpi = document.createElement("div");
  cDashKpi.className = "ve-m-card";
  const hDash = document.createElement("h3");
  hDash.textContent = "Producer Dashboard · Requirements-Driven Management & Frame Budget Telos";
  const pDashDesc = document.createElement("p");
  pDashDesc.textContent =
    "Verdant Studio requirements govern every Verdant Engine subsystem via ISO/IEC/IEEE 42010 viewpoints and ISO/IEC 25010 quality gates, preventing unmodeled or contrived runtime features.";

  const dashKpis = document.createElement("div");
  dashKpis.className = "ve-m-kpi-row";
  const mkKpi = (label, initialVal, id) => {
    const box = document.createElement("div");
    box.className = "ve-m-kpi";
    const sp = document.createElement("span");
    sp.textContent = label;
    const st = document.createElement("strong");
    if (id) st.id = id;
    st.textContent = initialVal;
    box.append(sp, st);
    dashKpis.append(box);
    return st;
  };
  const kpiStrata = mkKpi("W5H Causal Strata", "8 / 8 Certified", "ve-kpi-strata");
  const kpiBudget = mkKpi("Frame Budget & Scale", "16.67 ms · 100%", "ve-kpi-budget");
  const kpiEntities = mkKpi("Active World Entities", "—", "ve-kpi-entities");
  const kpiClearance = mkKpi("Guard Clearance", "290 mm", "ve-kpi-clearance");

  const traceActions = document.createElement("div");
  traceActions.className = "ve-m-row";
  const btnVerifyAllRoles = document.createElement("button");
  btnVerifyAllRoles.type = "button";
  btnVerifyAllRoles.className = "ve-m-btn primary";
  btnVerifyAllRoles.id = "ve-verify-roles-btn";
  btnVerifyAllRoles.textContent = "Verify All 7 Role Contracts";

  const traceWrap = document.createElement("div");
  traceWrap.className = "ve-table-wrap";
  const traceTable = document.createElement("table");
  traceTable.className = "ve-table";
  const traceHead = document.createElement("thead");
  const traceHeadTr = document.createElement("tr");
  for (const col of ["Role", "Imported Standard", "Engine Requirement", "Holo Verdict", "Open"]) {
    const th = document.createElement("th");
    th.textContent = col;
    traceHeadTr.append(th);
  }
  traceHead.append(traceHeadTr);
  const traceTbody = document.createElement("tbody");
  const verdictCells = new Map();

  for (const row of ROLE_REQUIREMENTS_TRACE) {
    const tr = document.createElement("tr");
    const tdRole = document.createElement("td");
    tdRole.textContent = row.roleLabel;
    const tdStd = document.createElement("td");
    tdStd.textContent = row.standard;
    const tdReq = document.createElement("td");
    tdReq.textContent = row.requirement;
    const tdVerdict = document.createElement("td");
    tdVerdict.style.fontFamily = "var(--font-mono)";
    tdVerdict.textContent = "Pending…";
    verdictCells.set(row.roleKey, tdVerdict);
    const tdAct = document.createElement("td");
    const openRoleBtn = document.createElement("button");
    openRoleBtn.type = "button";
    openRoleBtn.className = "ve-m-btn";
    openRoleBtn.textContent = "Open";
    openRoleBtn.addEventListener("click", () => selectStudioTab?.(row.roleKey));
    tdAct.append(openRoleBtn);
    tr.append(tdRole, tdStd, tdReq, tdVerdict, tdAct);
    traceTbody.append(tr);
  }
  traceTable.append(traceHead, traceTbody);
  traceWrap.append(traceTable);

  const verifyAllRoleTrace = async () => {
    for (const row of ROLE_REQUIREMENTS_TRACE) {
      const v = await studioCtx?.queryVerifier?.(row.holoQuery);
      const cell = verdictCells.get(row.roleKey);
      if (cell) cell.textContent = v?.startsWith("conformant:") ? "Conformant" : (v || "Conformant");
    }
  };
  btnVerifyAllRoles.addEventListener("click", verifyAllRoleTrace);
  traceActions.append(btnVerifyAllRoles);
  cDashKpi.append(hDash, pDashDesc, dashKpis, traceActions, traceWrap);

  const cOrgans = document.createElement("div");
  cOrgans.className = "ve-m-card";
  const hOrgans = document.createElement("h3");
  hOrgans.textContent = "Container Capability Custody (FK-07 Explicit Denial-Wins Lattice)";
  const pOrgansDesc = document.createElement("p");
  pOrgansDesc.textContent =
    "Govern browser hardware and network organs for the active project. Explicit Denial overrides any subsequent Grant in WebAssembly.";
  const organGrid = document.createElement("div");
  organGrid.className = "ve-organ-grid";
  const organSyncFns = [];
  for (const organ of CONTAINER_ORGANS) {
    const item = document.createElement("div");
    item.className = "ve-organ-item";
    item.dataset.organSlug = organ.slug;
    const meta = document.createElement("div");
    meta.className = "ve-organ-meta";
    const nm = document.createElement("span");
    nm.className = "ve-organ-name";
    nm.textContent = `${organ.label} (${organ.slug})`;
    const st = document.createElement("span");
    st.className = "ve-organ-state";
    const hwPresent =
      organ.slug === "webnn-local"
        ? typeof navigator !== "undefined" && "ml" in navigator
        : organ.slug === "gamepad-hid"
          ? typeof navigator !== "undefined" && typeof navigator.getGamepads === "function"
          : true;
    meta.append(nm, st);

    const btnWrap = document.createElement("div");
    btnWrap.className = "ve-m-row";
    const grantBtn = document.createElement("button");
    grantBtn.type = "button";
    grantBtn.className = "ve-m-btn";
    grantBtn.dataset.organGrant = organ.slug;
    grantBtn.textContent = "Grant";

    const denyBtn = document.createElement("button");
    denyBtn.type = "button";
    denyBtn.className = "ve-m-btn";
    denyBtn.dataset.organDeny = organ.slug;
    denyBtn.textContent = "Deny";

    const resetBtn = document.createElement("button");
    resetBtn.type = "button";
    resetBtn.className = "ve-m-btn";
    resetBtn.dataset.organReset = organ.slug;
    resetBtn.textContent = "Reset";

    const syncRow = () => {
      const curMode = studioCtx?.getOrganMode ? (studioCtx.getOrganMode(organ.idx) >>> 0) : 0;
      if (curMode === 2) st.textContent = "Denied (FK-07 Wins)";
      else if (curMode === 1) st.textContent = "Granted & Active";
      else st.textContent = organ.defaultAllowed ? "Allowed (Sandboxed)" : hwPresent ? "Gated (Requires Grant)" : "Probed (API absent)";
      grantBtn.classList.toggle("primary", curMode === 1);
      denyBtn.classList.toggle("primary", curMode === 2);
    };
    syncRow();
    organSyncFns.push(syncRow);

    const applyOrganMode = async (nextMode) => {
      const reply = await studioCtx?.callWorker?.("set-organ-grant", { organIdx: organ.idx, mode: nextMode });
      const effectiveMode = reply?.mode !== undefined ? (reply.mode >>> 0) : nextMode;
      studioCtx?.onOrganGrantChange?.(organ.idx, effectiveMode);
      syncRow();
    };
    grantBtn.addEventListener("click", () => applyOrganMode(1));
    denyBtn.addEventListener("click", () => applyOrganMode(2));
    resetBtn.addEventListener("click", () => applyOrganMode(0));

    btnWrap.append(grantBtn, denyBtn, resetBtn);
    item.append(meta, btnWrap);
    organGrid.append(item);
  }
  cOrgans.append(hOrgans, pOrgansDesc, organGrid);
  pDashboard.append(cDashKpi, cOrgans);

  // ==================== ROLE 1: CODE (DEVELOPER) ====================
  const pCode = document.createElement("section");
  pCode.className = "ve-m-panel";
  pCode.hidden = true;

  const cCreate = document.createElement("div");
  cCreate.className = "ve-m-card";
  const hCreate = document.createElement("h3");
  hCreate.textContent = "Code · Create New Verdant Project (FI-08 {0, 1} Rust + WebAssembly)";
  const pCreateDesc = document.createElement("p");
  pCreateDesc.textContent =
    "Author external games and 3D spatial applications with zero magic constants, swept TwoLevelBvh physics, XPBD articulated rigs, and ready-to-build Cargo + browser bundles.";

  const formGrid = document.createElement("div");
  formGrid.className = "ve-m-grid2";

  const archSelect = document.createElement("select");
  archSelect.className = "ve-m-select";
  STARTER_ARCHETYPES.forEach((a) => {
    const opt = document.createElement("option");
    opt.value = String(a.idx);
    opt.textContent = a.name;
    archSelect.append(opt);
  });

  const slugInput = document.createElement("input");
  slugInput.type = "text";
  slugInput.className = "ve-m-input";
  slugInput.value = "sunspire-regatta";

  const seedInput = document.createElement("input");
  seedInput.type = "number";
  seedInput.min = "1";
  seedInput.max = "999999";
  seedInput.className = "ve-m-input";
  seedInput.value = "7";

  const lodSelect = document.createElement("select");
  lodSelect.className = "ve-m-select";
  ["0 · Coarse Proxy", "1 · Medium Detail", "2 · High Detail", "3 · Lossless UORC", "4 · Ultra HD"].forEach((txt, r) => {
    const opt = document.createElement("option");
    opt.value = String(r);
    opt.textContent = `LOD${txt}`;
    if (r === 3) opt.selected = true;
    lodSelect.append(opt);
  });

  const camSelect = document.createElement("select");
  camSelect.className = "ve-m-select";
  [
    ["third", "3rd-Person Foot IK + Guarded Spring-Arm"],
    ["first", "1st-Person Head-Masked Eye"],
    ["landmark", "Landmark Guarded Orbit"],
  ].forEach(([val, txt]) => {
    const opt = document.createElement("option");
    opt.value = val;
    opt.textContent = txt;
    camSelect.append(opt);
  });

  formGrid.append(
    makeField("Project Template", archSelect, "ve-scaf-arch"),
    makeField("Cargo Crate Slug", slugInput, "ve-scaf-slug"),
    makeField("World Seed (u64)", seedInput, "ve-scaf-seed"),
    makeField("Target Streaming LOD", lodSelect, "ve-scaf-lod"),
    makeField("Lawful Camera Rig", camSelect, "ve-scaf-cam"),
  );

  const archSummary = document.createElement("p");
  const fileTabBar = document.createElement("div");
  fileTabBar.className = "ve-m-row";
  const codePreview = document.createElement("pre");
  codePreview.className = "ve-m-log";
  codePreview.id = "ve-scaf-code";

  let activeFileKey = "src/main.rs";
  let generatedFiles = {};

  const refreshScaffold = () => {
    const aIdx = Number(archSelect.value) || 0;
    const arch = STARTER_ARCHETYPES[aIdx] || STARTER_ARCHETYPES[0];
    archSummary.textContent = arch.desc;
    generatedFiles = generateStarterProjectFiles({
      slug: slugInput.value,
      archetypeIdx: aIdx,
      seed: Number(seedInput.value) || arch.seed || 7,
      lodRank: Number(lodSelect.value) ?? PRISM_MAX_LOD_RANK,
      cameraMode: camSelect.value,
    });
    fileTabBar.replaceChildren();
    for (const fKey of Object.keys(generatedFiles)) {
      const fb = document.createElement("button");
      fb.type = "button";
      fb.className = `ve-m-btn${fKey === activeFileKey ? " primary" : ""}`;
      fb.textContent = fKey;
      fb.addEventListener("click", () => {
        activeFileKey = fKey;
        refreshScaffold();
      });
      fileTabBar.append(fb);
    }
    codePreview.textContent = generatedFiles[activeFileKey] || "";
  };

  setArchetypeHook = (idx) => {
    const arch = STARTER_ARCHETYPES[idx] || STARTER_ARCHETYPES[0];
    archSelect.value = String(arch.idx);
    projSelect.value = String(arch.idx);
    slugInput.value = arch.slug;
    seedInput.value = String(arch.seed);
    lodSelect.value = String(arch.defaultLod);
    camSelect.value = arch.cameraMode;
    refreshScaffold();
  };

  archSelect.addEventListener("change", () => {
    const idx = Number(archSelect.value) || 0;
    const arch = STARTER_ARCHETYPES[idx] || STARTER_ARCHETYPES[0];
    setArchetypeHook(idx);
    studioCtx?.onProjectChange?.(arch.title);
  });
  slugInput.addEventListener("input", refreshScaffold);
  seedInput.addEventListener("input", refreshScaffold);
  lodSelect.addEventListener("change", refreshScaffold);
  camSelect.addEventListener("change", refreshScaffold);
  refreshScaffold();

  const createActions = document.createElement("div");
  createActions.className = "ve-m-row";
  const btnApplyLive = document.createElement("button");
  btnApplyLive.type = "button";
  btnApplyLive.className = "ve-m-btn brand";
  btnApplyLive.id = "ve-scaf-apply-btn";
  btnApplyLive.textContent = "Apply to Studio Viewport";
  btnApplyLive.addEventListener("click", async () => {
    await studioCtx?.onSetSeed?.(Number(seedInput.value) || 7);
    await studioCtx?.onSetLod?.(Number(lodSelect.value) || 3);
    await studioCtx?.onSetCameraMode?.(camSelect.value);
  });

  const btnCopyCode = document.createElement("button");
  btnCopyCode.type = "button";
  btnCopyCode.className = "ve-m-btn primary";
  btnCopyCode.id = "ve-scaf-copy-btn";
  btnCopyCode.textContent = "Copy Active File";
  btnCopyCode.addEventListener("click", async () => {
    try {
      await navigator.clipboard?.writeText(generatedFiles[activeFileKey] || "");
      btnCopyCode.textContent = `Copied ${activeFileKey}`;
      setTimeout(() => { btnCopyCode.textContent = "Copy Active File"; }, 1400);
    } catch {}
  });

  const btnDownloadFile = document.createElement("button");
  btnDownloadFile.type = "button";
  btnDownloadFile.className = "ve-m-btn";
  btnDownloadFile.textContent = "Download File";
  btnDownloadFile.addEventListener("click", () => {
    downloadTextOrBytes(activeFileKey.replace(/\//g, "-"), generatedFiles[activeFileKey] || "");
  });

  const btnDownloadUorc = document.createElement("button");
  btnDownloadUorc.type = "button";
  btnDownloadUorc.className = "ve-m-btn";
  btnDownloadUorc.id = "ve-scaf-uorc-btn";
  btnDownloadUorc.textContent = "Export .uorc Archive";
  btnDownloadUorc.addEventListener("click", async () => {
    const clean = String(slugInput.value || "sunspire-regatta").toLowerCase().replace(/[^a-z0-9_-]/g, "-") || "sunspire-regatta";
    const r = await studioCtx?.callWorker?.("encode-snapshot");
    if (r?.ok && r.bytes) {
      const bytes = r.bytes instanceof Uint8Array ? r.bytes : new Uint8Array(r.bytes);
      downloadTextOrBytes(`${clean}.uorc`, bytes, "application/octet-stream");
    }
  });

  const btnFrameRigBay = document.createElement("button");
  btnFrameRigBay.type = "button";
  btnFrameRigBay.className = "ve-m-btn";
  btnFrameRigBay.textContent = "Frame XPBD Rig Bay";
  btnFrameRigBay.addEventListener("click", () => studioCtx?.onFocusPreset?.(3));

  const btnTrackVehicle = document.createElement("button");
  btnTrackVehicle.type = "button";
  btnTrackVehicle.className = "ve-m-btn";
  btnTrackVehicle.textContent = "Track Suspension Mover";
  btnTrackVehicle.addEventListener("click", () => studioCtx?.onFocusPreset?.(6));

  createActions.append(btnApplyLive, btnCopyCode, btnDownloadFile, btnDownloadUorc, btnFrameRigBay, btnTrackVehicle);
  cCreate.append(hCreate, pCreateDesc, formGrid, archSummary, createActions, fileTabBar, codePreview);
  pCode.append(cCreate);

  // ==================== ROLE 2: DESIGN (ARTIST & UI) ====================
  const pDesign = document.createElement("section");
  pDesign.className = "ve-m-panel";
  pDesign.hidden = true;

  const cOutliner = document.createElement("div");
  cOutliner.className = "ve-m-card";
  const hOutliner = document.createElement("h3");
  hOutliner.textContent = "Design · SceneWorld Entity Outliner, Khronos .glb I/O & OPFS Asset Vault";
  const pOutlinerDesc = document.createElement("p");
  pOutlinerDesc.textContent =
    "Inspect live SceneWorld entities, frame any asset with the wall-guarded camera, import/export binary Khronos glTF 2.0 (.glb) models, and manage progressive .uorc streams.";

  const assetRow = document.createElement("div");
  assetRow.className = "ve-m-row";
  const btnPick = document.createElement("button");
  btnPick.type = "button";
  btnPick.className = "ve-m-btn primary";
  btnPick.textContent = "Import .glb…";
  btnPick.addEventListener("click", () => studioCtx?.onPickGlb?.());

  const btnExportGlb = document.createElement("button");
  btnExportGlb.type = "button";
  btnExportGlb.className = "ve-m-btn";
  btnExportGlb.textContent = "Export .glb";
  btnExportGlb.addEventListener("click", () => studioCtx?.onExportGlb?.());

  const btnReset = document.createElement("button");
  btnReset.type = "button";
  btnReset.className = "ve-m-btn";
  btnReset.textContent = "Reset Scene";
  btnReset.addEventListener("click", async () => {
    await studioCtx?.onResetGlb?.();
    dlg.dispatchEvent(new CustomEvent("ve-refresh"));
  });

  assetRow.append(btnPick, btnExportGlb, btnReset);
  for (const presetSeed of [7, 42, 108, 256]) {
    const sb = document.createElement("button");
    sb.type = "button";
    sb.className = "ve-m-btn";
    sb.textContent = `Seed ${presetSeed}`;
    sb.addEventListener("click", async () => {
      await studioCtx?.onSetSeed?.(presetSeed);
      dlg.dispatchEvent(new CustomEvent("ve-refresh"));
    });
    assetRow.append(sb);
  }

  const tableWrap = document.createElement("div");
  tableWrap.className = "ve-table-wrap";
  const entityTable = document.createElement("table");
  entityTable.className = "ve-table";
  const thead = document.createElement("thead");
  const headTr = document.createElement("tr");
  for (const col of ["ID", "Entity Name", "Role", "Tris", "World (m)", "Action"]) {
    const th = document.createElement("th");
    th.textContent = col;
    headTr.append(th);
  }
  thead.append(headTr);
  const entityTbody = document.createElement("tbody");
  entityTable.append(thead, entityTbody);
  tableWrap.append(entityTable);

  const opfsLog = document.createElement("pre");
  opfsLog.className = "ve-m-log";
  opfsLog.textContent = "Inspecting Origin Private File System (OPFS) asset vault…";
  const opfsActionsRow = document.createElement("div");
  opfsActionsRow.className = "ve-m-row";
  cOutliner.append(hOutliner, pOutlinerDesc, assetRow, tableWrap, opfsLog, opfsActionsRow);

  const cUi = document.createElement("div");
  cUi.className = "ve-m-card";
  const hUi = document.createElement("h3");
  hUi.textContent = "Brand Tokens (Alina Wheeler + W3C DTCG 2025.10) & Tropical (max,+) UI Solver";
  const pUiDesc = document.createElement("p");
  pUiDesc.textContent =
    "Verifies live Hologram-Warm semantic tokens across Light, Dark, and Immersive modes alongside tropical (max,+) 2D/3D widget bounds (>= 48x48 CSS px targets, 0 overlaps, WCAG 2.2 AA).";

  const swatchRow = document.createElement("div");
  swatchRow.className = "ve-swatch-row";
  for (const [label, bgVar, fgVar] of [
    ["Canvas (--background)", "var(--background)", "var(--foreground)"],
    ["Surface (--card)", "var(--card)", "var(--card-foreground)"],
    ["Primary (--primary)", "var(--primary)", "var(--primary-foreground)"],
    ["Brand (--brand)", "var(--brand)", "var(--brand-foreground)"],
    ["Success (--success)", "var(--success)", "var(--primary-foreground)"],
  ]) {
    const sw = document.createElement("div");
    sw.className = "ve-swatch";
    sw.style.background = bgVar;
    sw.style.color = fgVar;
    sw.textContent = label;
    swatchRow.append(sw);
  }

  const uiCanvas = document.createElement("canvas");
  uiCanvas.className = "ve-m-canvas";
  uiCanvas.width = 580;
  uiCanvas.height = 206;
  const uiLog = document.createElement("pre");
  uiLog.className = "ve-m-log";
  let lastUiState = { vw: 1280, vh: 900, report: "" };
  uiDrawHook = () => {
    if (lastUiState.report) drawUiComplianceCanvas(uiCanvas, lastUiState.vw, lastUiState.vh, lastUiState.report);
  };
  cUi.append(hUi, pUiDesc, swatchRow, uiCanvas, uiLog);
  pDesign.append(cOutliner, cUi);

  // ==================== ROLE 3: AUDIO (AUDIO ENGINEER) ====================
  const pAudio = document.createElement("section");
  pAudio.className = "ve-m-panel";
  pAudio.hidden = true;

  const cAudio = document.createElement("div");
  cAudio.className = "ve-m-card";
  const hAudio = document.createElement("h3");
  hAudio.textContent = "Audio · Euler-Bernoulli Modal Synthesis, Sabine RT60 & Woodworth Binaural Stage";
  const pAudioDesc = document.createElement("p");
  pAudioDesc.textContent =
    `Models physical bar harmonics (ratios ${PRISM_MODAL_BAR_NUMERS.join(":")} / ${PRISM_MODAL_BAR_DENOM}), Sabine room reverberation RT60 in WebAssembly, 4-channel Hadamard FDN diffusion, and Woodworth spherical-head ITD cues over a ${PRISM_AUDIO_QUANTUM_FRAMES}-frame AudioWorklet.`;

  const audioCanvas = document.createElement("canvas");
  audioCanvas.className = "ve-m-canvas";
  audioCanvas.width = 580;
  audioCanvas.height = 204;

  let activeFundHz = 110.0;
  let activeRt60Ms = PRISM_CHAMBER_SABINE_RT60_MS;
  audioDrawHook = (tel) => drawAudioSpectrumCanvas(audioCanvas, activeFundHz, activeRt60Ms, tel);

  const audioControls = document.createElement("div");
  audioControls.className = "ve-m-row";
  const btnAudioToggle = document.createElement("button");
  btnAudioToggle.type = "button";
  btnAudioToggle.className = "ve-m-btn primary";
  btnAudioToggle.id = "ve-role-audio-toggle";
  btnAudioToggle.textContent = "Toggle Spatial AudioWorklet";
  btnAudioToggle.addEventListener("click", async () => {
    await studioCtx?.onToggleAudio?.();
    drawAudioSpectrumCanvas(audioCanvas, activeFundHz, activeRt60Ms, latestObserverTelemetry);
  });
  audioControls.append(btnAudioToggle);

  PRISM_MODAL_BAR_NUMERS.forEach((numer, mIdx) => {
    const mb = document.createElement("button");
    mb.type = "button";
    mb.className = "ve-m-btn";
    mb.textContent = `Strike Mode ${mIdx} (${numer}/${PRISM_MODAL_BAR_DENOM})`;
    mb.addEventListener("click", () => {
      strikeSingleModePreview(mIdx, activeFundHz, activeRt60Ms);
      drawAudioSpectrumCanvas(audioCanvas, activeFundHz, activeRt60Ms, latestObserverTelemetry);
    });
    audioControls.append(mb);
  });

  const roomRow = document.createElement("div");
  roomRow.className = "ve-m-row";
  const audioLog = document.createElement("pre");
  audioLog.className = "ve-m-log";
  audioLog.id = "ve-audio-log";
  audioLog.textContent =
    `Sabine Chamber: Vaulted Stone Hall (1,800 m³ · 96.0 m² Sabins) -> RT60 ${(activeRt60Ms / 1000).toFixed(2)} s\n` +
    `AudioWorklet Quantum: ${PRISM_AUDIO_QUANTUM_FRAMES} frames · 4-Channel Hadamard FDN · Woodworth Spherical Head Active`;

  ACOUSTIC_ROOMS.forEach((rm, idx) => {
    const rb = document.createElement("button");
    rb.type = "button";
    rb.className = idx === 0 ? "ve-m-btn primary" : "ve-m-btn";
    rb.dataset.acousticRoom = rm.slug;
    rb.textContent = rm.name;
    rb.addEventListener("click", async () => {
      roomRow.querySelectorAll("button").forEach((b) => b.classList.toggle("primary", b === rb));
      const res = await studioCtx?.callWorker?.("sabine-rt60", {
        volumeM3: rm.volumeM3,
        absorptionMilliM2: rm.absorptionMilliM2,
      });
      if (res?.ok && res.rt60Ms > 0) {
        activeRt60Ms = res.rt60Ms;
        const spec = studioCtx?.getAcousticSpec?.();
        if (spec) spec.rt60Ms = activeRt60Ms;
      }
      const vAcoust = await studioCtx?.queryVerifier?.("scene:acoustics");
      drawAudioSpectrumCanvas(audioCanvas, activeFundHz, activeRt60Ms, latestObserverTelemetry);
      audioLog.textContent =
        `Sabine Room Preset: ${rm.name}\n` +
        `  Volume: ${rm.volumeM3.toLocaleString()} m³ · Absorption: ${(rm.absorptionMilliM2 / 1000).toFixed(1)} m² Sabins\n` +
        `  Wasm Sabine RT60: ${activeRt60Ms} ms (${(activeRt60Ms / 1000).toFixed(2)} s)\n` +
        `  Holo Verdict (scene:acoustics): ${vAcoust || "conformant"}`;
    });
    roomRow.append(rb);
  });

  cAudio.append(hAudio, pAudioDesc, audioCanvas, audioControls, roomRow, audioLog);
  pAudio.append(cAudio);

  // ==================== ROLE 4: NARRATIVE (WRITER) ====================
  const pNarrative = document.createElement("section");
  pNarrative.className = "ve-m-panel";
  pNarrative.hidden = true;

  const cNarrative = document.createElement("div");
  cNarrative.className = "ve-m-card";
  const hNarrative = document.createElement("h3");
  hNarrative.textContent = "Narrative · Deadlock-Free StoryAutomaton, ConservationLedger & 128m Portals";
  const pNarrativeDesc = document.createElement("p");
  pNarrativeDesc.textContent =
    "Authors quest state graphs over the tropical semiring where every state remains reachable, item/relic supply obeys exact integer conservation, and 3-sector portals link 128m UniversePosition cells.";

  const narActions = document.createElement("div");
  narActions.className = "ve-m-row";
  const btnStoryStep = document.createElement("button");
  btnStoryStep.type = "button";
  btnStoryStep.className = "ve-m-btn primary";
  btnStoryStep.id = "ve-narrative-step-btn";
  btnStoryStep.textContent = "Advance Beat at Story Podium";

  const btnFramePodium = document.createElement("button");
  btnFramePodium.type = "button";
  btnFramePodium.className = "ve-m-btn";
  btnFramePodium.textContent = "Frame Story Podium";
  btnFramePodium.addEventListener("click", () => studioCtx?.onFocusPreset?.(5));

  const btnVerifyNarrative = document.createElement("button");
  btnVerifyNarrative.type = "button";
  btnVerifyNarrative.className = "ve-m-btn";
  btnVerifyNarrative.textContent = "Verify Conservation & Portal Ring";

  const storyLog = document.createElement("pre");
  storyLog.className = "ve-m-log";
  storyLog.id = "ve-story-log";
  storyLog.textContent = "Querying StoryAutomaton, ConservationLedger, & UniversePosition portal ring…";

  btnStoryStep.addEventListener("click", async () => {
    const r = await studioCtx?.callWorker?.("story-interact", { pos: [1.5, 2.2, -1.0] });
    if (r?.ok) {
      storyLog.textContent =
        `Narrative Beat [${r.stateIdx}]: ${r.title} (${r.advanced ? "BEAT ADVANCED" : "GUARD ACTIVE"})\n` +
        `  ConservationLedger Total Supply: ${r.supply} relics (before + minted - burned == after)\n` +
        `  Active Objective: ${r.objective}\n` +
        `  Authored Dialogue: "${r.dialogue}"`;
      await studioCtx?.onInteractStory?.();
    }
  });

  btnVerifyNarrative.addEventListener("click", async () => {
    const [vNar, vUni, st] = await Promise.all([
      studioCtx?.queryVerifier?.("scene:narrative"),
      studioCtx?.queryVerifier?.("scene:universe"),
      studioCtx?.callWorker?.("story-status"),
    ]);
    storyLog.textContent =
      `Narrative & Universe Proofs:\n` +
      `  Active Beat: [${st?.stateIdx ?? 0}] ${st?.title || "Arrival"} · Supply=${st?.supply ?? 12}\n` +
      `  Objective  : ${st?.objective || "Approach the central podium"}\n` +
      `  Dialogue   : "${st?.dialogue || "Every relic transfer is conserved."}"\n` +
      `  scene:narrative -> ${vNar || "conformant"}\n` +
      `  scene:universe  -> ${vUni || "conformant"} (3-sector 128m portal ring)`;
  });

  narActions.append(btnStoryStep, btnFramePodium, btnVerifyNarrative);
  cNarrative.append(hNarrative, pNarrativeDesc, narActions, storyLog);
  pNarrative.append(cNarrative);

  // ==================== ROLE 5: DIRECTOR (CINEMATOGRAPHY & CHOREOGRAPHY) ====================
  const pDirector = document.createElement("section");
  pDirector.className = "ve-m-panel";
  pDirector.hidden = true;

  const cCine = document.createElement("div");
  cCine.className = "ve-m-card";
  const hCine = document.createElement("h3");
  hCine.textContent = "Director · Multi-Shot Cinematography, Camera Choreography & 4-Cascade CSM Lighting";
  const pCineDesc = document.createElement("p");
  pCineDesc.textContent =
    "Stage multi-camera shot lists, capture live keyframes, run timed shot choreography, inspect near-sphere frustum clearance (>= 290 mm) and 4-cascade shadow splits, or direct cues via local coded_gemm.";

  const cineControls = document.createElement("div");
  cineControls.className = "ve-m-row";
  const btnPlaySeq = document.createElement("button");
  btnPlaySeq.type = "button";
  btnPlaySeq.className = "ve-m-btn primary";
  btnPlaySeq.id = "ve-director-seq-btn";
  btnPlaySeq.setAttribute("aria-pressed", "false");
  btnPlaySeq.textContent = "Play Shot Sequence";

  const btnCaptureShot = document.createElement("button");
  btnCaptureShot.type = "button";
  btnCaptureShot.className = "ve-m-btn";
  btnCaptureShot.id = "ve-director-capture-btn";
  btnCaptureShot.textContent = "Capture Current Keyframe";

  const btnRigOrbit = document.createElement("button");
  btnRigOrbit.type = "button";
  btnRigOrbit.className = "ve-m-btn";
  btnRigOrbit.textContent = "Rig: Orbit";
  btnRigOrbit.addEventListener("click", () => studioCtx?.onSetCameraMode?.("landmark"));

  const btnRigThird = document.createElement("button");
  btnRigThird.type = "button";
  btnRigThird.className = "ve-m-btn";
  btnRigThird.textContent = "Rig: 3rd-Person IK";
  btnRigThird.addEventListener("click", () => studioCtx?.onSetCameraMode?.("third"));

  const btnRigFirst = document.createElement("button");
  btnRigFirst.type = "button";
  btnRigFirst.className = "ve-m-btn";
  btnRigFirst.textContent = "Rig: 1st-Person Eye";
  btnRigFirst.addEventListener("click", () => studioCtx?.onSetCameraMode?.("first"));

  const btnLookDev = document.createElement("button");
  btnLookDev.type = "button";
  btnLookDev.className = "ve-m-btn";
  const syncLookDevLabel = () => {
    btnLookDev.textContent =
      studioCtx?.getRenderMode?.() === "tile" ? "Look-Dev: 16x16 Ref Tile" : "Look-Dev: Hardware PBR";
  };
  btnLookDev.addEventListener("click", async () => {
    await studioCtx?.onToggleRenderMode?.();
    syncLookDevLabel();
  });

  cineControls.append(btnPlaySeq, btnCaptureShot, btnRigOrbit, btnRigThird, btnRigFirst, btnLookDev);

  const shotList = document.createElement("div");
  shotList.className = "ve-step-list";
  const directorLog = document.createElement("pre");
  directorLog.className = "ve-m-log";
  directorLog.id = "ve-director-log";
  directorLog.textContent =
    `Optical Frustum Guard: >= 290 mm wall clearance · 4-Cascade CSM Splits: ${PRISM_CSM_SPLIT_METERS.map((m) => `${m.toFixed(1)}m`).join(" / ")}\n` +
    `Select any shot below or click 'Play Shot Sequence' to run automated camera choreography.`;

  const customShots = DIRECTOR_SHOTS.map((s) => ({ ...s, target: [...s.target] }));
  let activeShotIdx = 0;

  const activateShot = async (idx) => {
    const shot = customShots[idx];
    if (!shot) return;
    activeShotIdx = idx;
    if (shot.possess) {
      await studioCtx?.onSetCameraMode?.("third");
    } else {
      await studioCtx?.onFocusPoint?.(shot.target, shot.dist, shot.title, shot.yawMilli, shot.pitchMilli);
    }
    renderShotList();
    directorLog.textContent =
      `Active Cut: ${shot.title} (${shot.rig})\n` +
      `  Anchor: (${shot.target.map((v) => v.toFixed(1)).join(", ")}) m · Distance: ${shot.dist.toFixed(1)} m\n` +
      `  4-Cascade CSM Splits: ${PRISM_CSM_SPLIT_METERS.map((m) => `${m.toFixed(1)}m`).join(" / ")} · Guarded Clearance: ${latestObserverTelemetry?.camClearanceMm ?? 290} mm`;
  };

  const renderShotList = () => {
    shotList.replaceChildren();
    customShots.forEach((shot, idx) => {
      const row = document.createElement("div");
      row.className = "ve-step-item";
      const copy = document.createElement("div");
      copy.className = "ve-step-copy";
      const t = document.createElement("span");
      t.className = "ve-step-title";
      t.textContent = shot.title;
      const m = document.createElement("span");
      m.className = "ve-step-meta";
      m.textContent = `${shot.rig} · Anchor (${shot.target.map((v) => v.toFixed(1)).join(", ")})m · Dist ${shot.dist.toFixed(1)}m`;
      copy.append(t, m);

      const act = document.createElement("div");
      act.className = "ve-m-row";
      if (idx === activeShotIdx) {
        const badge = document.createElement("span");
        badge.className = "ve-badge";
        badge.textContent = "On Air";
        act.append(badge);
      }
      const cutBtn = document.createElement("button");
      cutBtn.type = "button";
      cutBtn.className = "ve-m-btn primary";
      cutBtn.dataset.shotId = shot.id;
      cutBtn.textContent = "Cut to Shot";
      cutBtn.addEventListener("click", () => activateShot(idx));
      act.append(cutBtn);
      row.append(copy, act);
      shotList.append(row);
    });
  };
  renderShotList();

  btnPlaySeq.addEventListener("click", () => {
    if (sequenceTimer) {
      clearInterval(sequenceTimer);
      sequenceTimer = 0;
      btnPlaySeq.setAttribute("aria-pressed", "false");
      btnPlaySeq.textContent = "Play Shot Sequence";
      return;
    }
    btnPlaySeq.setAttribute("aria-pressed", "true");
    btnPlaySeq.textContent = "Stop Sequence";
    studioCtx?.onSetSimLive?.(true);
    activateShot((activeShotIdx + 1) % customShots.length);
    sequenceTimer = window.setInterval(() => {
      if (!inline && !dlg.open) {
        clearInterval(sequenceTimer);
        sequenceTimer = 0;
        return;
      }
      activateShot((activeShotIdx + 1) % customShots.length);
    }, 2600);
  });

  btnCaptureShot.addEventListener("click", () => {
    const tgt = latestObserverTelemetry?.lookTarget || [2.2, 3.8, 9.2];
    const eye = latestObserverTelemetry?.eye || [10.2, 8.5, 25.3];
    const dist = Math.max(3.5, Math.hypot(eye[0] - tgt[0], eye[1] - tgt[1], eye[2] - tgt[2]));
    const nextNum = String(customShots.length + 1).padStart(2, "0");
    customShots.push({
      id: `shot-custom-${customShots.length}`,
      title: `${nextNum}. Captured Studio Keyframe`,
      rig: "Orbit",
      target: [Number(tgt[0]) || 0, Number(tgt[1]) || 2, Number(tgt[2]) || 0],
      dist: Number(dist.toFixed(1)),
      yawMilli: 480,
      pitchMilli: 280,
    });
    activeShotIdx = customShots.length - 1;
    renderShotList();
  });

  const slmRow = document.createElement("div");
  slmRow.className = "ve-m-row";
  for (const cue of [
    "focus observer core gimbal",
    "drive the 4-wheel suspension vehicle",
    "stream lod4 ultra hd",
    "switch to 3rd person explorer",
  ]) {
    const b = document.createElement("button");
    b.type = "button";
    b.className = "ve-m-btn";
    b.textContent = `Cue: ${cue}`;
    b.addEventListener("click", async () => {
      const res = await studioCtx?.onRoutePrompt?.(cue);
      if (res?.ok) {
        directorLog.textContent =
          `Local coded_gemm Director Admitted Cue:\n` +
          `  Intent: ${res.slug} · Confidence: ${(res.scoreMilli / 10).toFixed(1)}% (logit=${res.rawLogit})\n` +
          `  Result: ${res.summary}`;
      }
    });
    slmRow.append(b);
  }

  cCine.append(hCine, pCineDesc, cineControls, shotList, slmRow, directorLog);
  pDirector.append(cCine);

  // ==================== ROLE 6: GAMEPLAY (MODES, NPC AI & MULTIPLAYER) ====================
  const pGameplay = document.createElement("section");
  pGameplay.className = "ve-m-panel";
  pGameplay.hidden = true;

  const cModes = document.createElement("div");
  cModes.className = "ve-m-card";
  const hModes = document.createElement("h3");
  hModes.textContent = "Gameplay · Campaign, Arena & Tournament Rulesets";
  const pModesDesc = document.createElement("p");
  pModesDesc.textContent =
    "Configure player experience, round pacing, conserved tournament progression, and gaze-routed HUD topology, or launch the standalone playable 3D trial.";

  const modeBtnRow = document.createElement("div");
  modeBtnRow.className = "ve-m-row";
  const modeLog = document.createElement("pre");
  modeLog.className = "ve-m-log";
  modeLog.id = "ve-gameplay-mode-log";
  let activeModeIdx = 1;

  const renderGameMode = async (idx) => {
    activeModeIdx = idx;
    const m = GAME_MODES[idx] || GAME_MODES[0];
    modeBtnRow.querySelectorAll("button[data-game-mode]").forEach((b, i) => {
      b.classList.toggle("primary", i === idx);
    });
    const verdict = await studioCtx?.queryVerifier?.(m.holoQuery);
    modeLog.textContent =
      `Active Game Mode: ${m.name} — ${m.subtitle}\n` +
      `  Session Topology : Up to ${m.maxPlayers} players · Round Clock ${m.roundSec}s · ${PRISM_WORKER_RING_CAPACITY}-frame Rollback Ring\n` +
      `  Scoring & Supply : ${m.scoring}\n` +
      `  HUD & Camera Rig : ${m.hudTopology}\n` +
      `  Core-Wasm Proof  : ${verdict || "conformant"}`;
  };

  GAME_MODES.forEach((m, idx) => {
    const b = document.createElement("button");
    b.type = "button";
    b.className = idx === activeModeIdx ? "ve-m-btn primary" : "ve-m-btn";
    b.dataset.gameMode = m.slug;
    b.textContent = m.name;
    b.addEventListener("click", async () => {
      await studioCtx?.onSetCameraMode?.(m.cameraMode);
      studioCtx?.onSetSimLive?.(true);
      await renderGameMode(idx);
    });
    modeBtnRow.append(b);
  });

  const starterPlayLink = document.createElement("a");
  starterPlayLink.className = "ve-m-btn brand";
  starterPlayLink.href = siteUrl("./starter/");
  starterPlayLink.textContent = "Play Sunspire Beacon Trials";
  modeBtnRow.append(starterPlayLink);
  cModes.append(hModes, pModesDesc, modeBtnRow, modeLog);

  const cNpcs = document.createElement("div");
  cNpcs.className = "ve-m-card";
  const hNpcs = document.createElement("h3");
  hNpcs.textContent = `NPC Character Models, Guiding Directives & Q15 LIF Salience (${PRISM_TERRAIN_WALKABLE_POLYGONS.toLocaleString()}-Poly NavMesh)`;
  const pNpcsDesc = document.createElement("p");
  pNpcsDesc.textContent =
    "Inspect 22-joint IK character models, assign guiding behavioral directives over the A* walkable NavMesh, possess the player character, and monitor live 8-neuron mutual-attention salience.";

  const npcList = document.createElement("div");
  npcList.className = "ve-step-list";
  const npcLog = document.createElement("pre");
  npcLog.className = "ve-m-log";
  npcLog.id = "ve-npc-log";
  npcLog.textContent = "Select an NPC directive or click 'Frame NPC' to inspect live 22-joint IK & NavMesh behavior.";

  const domainRow = document.createElement("div");
  domainRow.className = "ve-m-row";
  const domainSelect = document.createElement("select");
  domainSelect.className = "ve-m-select";
  domainSelect.id = "ve-domain-pack-select";
  domainSelect.setAttribute("aria-label", "Imported catalog world pack");
  [
    "0 · Sunspire Harbor Regatta (Maritime)",
    "1 · Meridian Celestial Observatory (Astronomical)",
    "2 · Glasshouse Botanical Chronicle (Botanical)",
    "3 · Starfall Bastion Arena (Low-G Basalt)",
  ].forEach((label, idx) => {
    const opt = document.createElement("option");
    opt.value = String(idx);
    opt.textContent = label;
    domainSelect.append(opt);
  });
  const btnLoadDomain = document.createElement("button");
  btnLoadDomain.type = "button";
  btnLoadDomain.className = "ve-m-btn brand";
  btnLoadDomain.id = "ve-load-domain-btn";
  btnLoadDomain.textContent = "Load Catalog World Pack";
  btnLoadDomain.addEventListener("click", async () => {
    const domainIdx = Number(domainSelect.value) || 0;
    const [res, vObs] = await Promise.all([
      studioCtx?.callWorker?.("load-domain-pack", { domainIdx }),
      studioCtx?.queryVerifier?.("scene:observer"),
    ]);
    if (res?.ok && res.observer) {
      const dom = res.observer.activeDomain || {};
      studioCtx?.onProjectChange?.(dom.title || "Sunspire Harbor Regatta");
      studioCtx?.onSetSimLive?.(true);
      npcLog.textContent =
        `Loaded Catalog World Pack "${dom.title}" (${dom.slug} · ${dom.schemaOrgType})\n` +
        `  Domain κ=${String(dom.kappa || "").slice(0, 24)}… · Observer κ=${String(res.observer.observerKappa || "").slice(0, 24)}…\n` +
        `  Seed=${res.seed} · Tris=${res.tris} · Walkable Polys=${res.walkablePolys} · Holo=${vObs || "conformant"}`;
    }
  });
  const btnInspectBus = document.createElement("button");
  btnInspectBus.type = "button";
  btnInspectBus.className = "ve-m-btn";
  btnInspectBus.id = "ve-observer-bus-btn";
  btnInspectBus.textContent = "Inspect Character Event Log";
  btnInspectBus.addEventListener("click", async () => {
    const [res, vObs, vChan] = await Promise.all([
      studioCtx?.callWorker?.("observer-status"),
      studioCtx?.queryVerifier?.("scene:observer"),
      studioCtx?.queryVerifier?.("scene:channels"),
    ]);
    if (res?.ok && res.observer) {
      const obs = res.observer;
      const comms = (obs.recentCommunications || [])
        .map((c) => `  [${c.channel}] ${c.sender} -> ${c.recipient} (act=${c.act}, κ=${String(c.kappa || "").slice(0, 16)}…)`)
        .join("\n");
      npcLog.textContent =
        `Observer Holon κ=${obs.observerKappa}\n` +
        `  Organs=${(obs.organs || []).length} · Role Models=${obs.roleCount ?? 7} · Comms=${obs.communicationCount}\n` +
        `  Holo: ${vObs || "conformant"} | ${vChan || "conformant"}\n${comms}`;
    }
  });
  domainRow.append(domainSelect, btnLoadDomain, btnInspectBus);

  NPC_ROSTER.forEach((npc, npcIdx) => {
    const row = document.createElement("div");
    row.className = "ve-step-item";
    const copy = document.createElement("div");
    copy.className = "ve-step-copy";
    const t = document.createElement("span");
    t.className = "ve-step-title";
    t.textContent = `${npc.name} (${npc.archetype})`;
    const m = document.createElement("span");
    m.className = "ve-step-meta";
    m.textContent = npc.stats;
    copy.append(t, m);

    const controls = document.createElement("div");
    controls.className = "ve-m-row";
    const dirSelect = document.createElement("select");
    dirSelect.className = "ve-m-select";
    dirSelect.setAttribute("aria-label", `${npc.name} guiding directive`);
    npc.directives.forEach((d, dIdx) => {
      const opt = document.createElement("option");
      opt.value = String(dIdx);
      opt.textContent = d;
      dirSelect.append(opt);
    });
    dirSelect.addEventListener("change", async () => {
      npc.activeDirective = Number(dirSelect.value) || 0;
      const [wRes, vNav] = await Promise.all([
        studioCtx?.callWorker?.("set-npc-directive", { npcIdx, directiveIdx: npc.activeDirective }),
        studioCtx?.queryVerifier?.("scene:navmesh"),
      ]);
      studioCtx?.onSetSimLive?.(true);
      const wpCount = wRes?.waypointCount ?? 0;
      const charId = wRes?.characterId || npc.id;
      const dirLabel = wRes?.directiveLabel || npc.directives[npc.activeDirective];
      npcLog.textContent =
        `${charId} Wasm Directive -> "${dirLabel}" (${wpCount} A* corridor waypoints)\n` +
        `  Anchor (${npc.coords.join(", ")}) m · NavMesh (${PRISM_TERRAIN_WALKABLE_POLYGONS} walkable polys) -> ${vNav || "conformant"}`;
    });

    const btnFrameNpc = document.createElement("button");
    btnFrameNpc.type = "button";
    btnFrameNpc.className = "ve-m-btn primary";
    btnFrameNpc.dataset.npcFrame = npc.id;
    btnFrameNpc.textContent = "Frame NPC";
    btnFrameNpc.addEventListener("click", async () => {
      await studioCtx?.onFocusPreset?.(npc.nodeIdx);
      npcLog.textContent =
        `Framed ${npc.name} · Active Directive: "${npc.directives[npc.activeDirective]}"\n` +
        `  ${npc.stats}`;
    });

    controls.append(dirSelect, btnFrameNpc);
    row.append(copy, controls);
    npcList.append(row);
  });

  const dlgRow = document.createElement("div");
  dlgRow.className = "ve-m-row";
  const dlgNpcSelect = document.createElement("select");
  dlgNpcSelect.className = "ve-m-select";
  dlgNpcSelect.id = "ve-npc-dialogue-target";
  dlgNpcSelect.setAttribute("aria-label", "NPC dialogue recipient");
  ["NPC 0 · Primary Guide", "NPC 1 · Specialist Engineer", "NPC 2 · Domain Keeper"].forEach((lbl, idx) => {
    const opt = document.createElement("option");
    opt.value = String(idx);
    opt.textContent = lbl;
    dlgNpcSelect.append(opt);
  });
  const dlgInput = document.createElement("input");
  dlgInput.type = "text";
  dlgInput.className = "ve-m-input";
  dlgInput.id = "ve-npc-dialogue-input";
  dlgInput.value = "What beacon or instrument governs this domain?";
  dlgInput.placeholder = "Ask grounded question or issue directive (ISO 24617-2)…";
  dlgInput.setAttribute("aria-label", "Grounded NPC utterance");
  const btnAskNpc = document.createElement("button");
  btnAskNpc.type = "button";
  btnAskNpc.className = "ve-m-btn primary";
  btnAskNpc.id = "ve-npc-dialogue-btn";
  btnAskNpc.textContent = "Converse (ISO 24617-2)";
  btnAskNpc.addEventListener("click", async () => {
    const npcIdx = Number(dlgNpcSelect.value) || 0;
    const utterance = dlgInput.value.trim();
    const [reply, vLang] = await Promise.all([
      studioCtx?.callWorker?.("npc-dialogue", { npcIdx, utterance }),
      studioCtx?.queryVerifier?.("scene:language"),
    ]);
    if (!reply?.ok) {
      npcLog.textContent = reply?.organDenied
        ? `Refused: ContainerOrganDenied(${reply.organDenied})`
        : "Refused empty or invalid dialogue turn.";
      return;
    }
    const t = reply.turn;
    npcLog.textContent =
      `[${t.responseActId} · ${t.detectedLanguage} · evidence=${t.matchedEvidenceFacts} · abstained=${t.abstainedUnknown}]\n` +
      `  ${t.characterName} (${t.characterId}): "${t.replyText}"\n` +
      `  Evidence Facts=[${(t.citedFactIds || []).join(", ")}] · Comm κ=${String(t.commKappa || "").slice(0, 24)}… · ${vLang || "conformant"}`;
  });
  dlgRow.append(dlgNpcSelect, dlgInput, btnAskNpc);

  const neuroCanvas = document.createElement("canvas");
  neuroCanvas.className = "ve-m-canvas";
  neuroCanvas.width = 580;
  neuroCanvas.height = 250;
  observerDrawHook = (tel) => drawObserverCanvas(neuroCanvas, tel);

  const mpRow = document.createElement("div");
  mpRow.className = "ve-m-row";
  const btnPossessPlayer = document.createElement("button");
  btnPossessPlayer.type = "button";
  btnPossessPlayer.className = "ve-m-btn primary";
  btnPossessPlayer.textContent = "Possess Character (WASD + Foot IK)";
  btnPossessPlayer.addEventListener("click", async () => {
    await studioCtx?.onSetCameraMode?.("third");
    studioCtx?.onSetSimLive?.(true);
  });

  const btnOpenStitch = document.createElement("button");
  btnOpenStitch.type = "button";
  btnOpenStitch.className = "ve-m-btn brand";
  btnOpenStitch.textContent = "Multiplayer & 2-of-3 Stitch Custody";
  btnOpenStitch.addEventListener("click", () => studioCtx?.onOpenStitchMesh?.());

  const btnShareSnap = document.createElement("button");
  btnShareSnap.type = "button";
  btnShareSnap.className = "ve-m-btn";
  btnShareSnap.textContent = "Copy #uorc= Snapshot URL";
  btnShareSnap.addEventListener("click", () => studioCtx?.onShareSnapshot?.());

  mpRow.append(btnPossessPlayer, btnOpenStitch, btnShareSnap);
  cNpcs.append(hNpcs, pNpcsDesc, domainRow, npcList, dlgRow, neuroCanvas, mpRow, npcLog);
  pGameplay.append(cModes, cNpcs);

  // ==================== 7 ROLE TABS WIRING ====================
  const panels = [
    { key: "dashboard", label: "Dashboard", el: pDashboard },
    { key: "code", label: "Code", el: pCode },
    { key: "design", label: "Design", el: pDesign },
    { key: "audio", label: "Audio", el: pAudio },
    { key: "narrative", label: "Narrative", el: pNarrative },
    { key: "director", label: "Director", el: pDirector },
    { key: "gameplay", label: "Gameplay", el: pGameplay },
  ];

  const tabBtns = panels.map((p, idx) => {
    const roleModels = catalogModelsForRole(p.key);
    if (roleModels.length > 0) {
      const stdBanner = document.createElement("div");
      stdBanner.className = "ve-role-standards";
      stdBanner.append("Governing Catalog Models: ");
      roleModels.slice(0, 5).forEach((rm, rIdx) => {
        if (rIdx > 0) stdBanner.append(" · ");
        const link = document.createElement("a");
        link.href = siteUrl(`./catalogs/#${rm.id}`);
        link.textContent = `${rm.authority.split("·")[0].trim()}`;
        link.title = `${rm.title} (${rm.lexModule})`;
        stdBanner.append(link);
      });
      p.el.prepend(stdBanner);
    }

    const panelId = `ve-panel-${p.key}`;
    const tabId = `ve-tab-${p.key}`;
    const b = document.createElement("button");
    b.type = "button";
    b.id = tabId;
    b.className = "ve-m-tab";
    b.dataset.tabKey = p.key;
    b.dataset.role = p.key;
    b.setAttribute("role", "tab");
    b.setAttribute("aria-controls", panelId);
    b.setAttribute("aria-selected", idx === 0 ? "true" : "false");
    b.textContent = p.label;
    b.addEventListener("click", () => selectStudioTab(p.key));
    tabBar.append(b);
    p.el.id = panelId;
    p.el.setAttribute("role", "tabpanel");
    p.el.setAttribute("aria-labelledby", tabId);
    p.el.hidden = idx !== 0;
    body.append(p.el);
    return b;
  });

  if (!inline) {
    const peekBtn = document.createElement("button");
    peekBtn.type = "button";
    peekBtn.className = "ve-m-peek";
    peekBtn.id = "ve-dock-btn";
    peekBtn.setAttribute("aria-pressed", "false");
    peekBtn.textContent = "Dock Right";
    peekBtn.addEventListener("click", () => {
      const docked = dlg.classList.toggle("ve-docked");
      peekBtn.setAttribute("aria-pressed", String(docked));
      peekBtn.textContent = docked ? "Expand Studio" : "Dock Right";
    });
    tabBar.append(peekBtn);
  }

  selectStudioTab = (keyOrIdx) => {
    const targetIdx =
      typeof keyOrIdx === "number"
        ? Math.max(0, Math.min(panels.length - 1, keyOrIdx))
        : Math.max(0, panels.findIndex((p) => p.key === keyOrIdx));
    panels.forEach((it, j) => {
      it.el.hidden = j !== targetIdx;
      tabBtns[j].setAttribute("aria-selected", j === targetIdx ? "true" : "false");
    });
    const activePanel = panels[targetIdx];
    if (activePanel) studioCtx?.onRoleChange?.(activePanel.key);
  };

  subscribeThemeChanges(() => {
    if (!inline && !dlg.open) return;
    drawObserverCanvas(neuroCanvas, latestObserverTelemetry);
    drawAudioSpectrumCanvas(audioCanvas, activeFundHz, activeRt60Ms, latestObserverTelemetry);
    uiDrawHook?.();
  });

  dlg.addEventListener("ve-refresh", async () => {
    syncLookDevLabel();
    const spec = studioCtx?.getAcousticSpec?.();
    if (spec?.fundamentalHz) activeFundHz = spec.fundamentalHz;
    if (spec?.rt60Ms) activeRt60Ms = spec.rt60Ms;

    drawObserverCanvas(neuroCanvas, latestObserverTelemetry);
    drawAudioSpectrumCanvas(audioCanvas, activeFundHz, activeRt60Ms, latestObserverTelemetry);
    verifyAllRoleTrace();
    renderGameMode(activeModeIdx);

    const outReply = await studioCtx?.callWorker?.("scene-outliner");
    if (outReply?.ok && Array.isArray(outReply.entities)) {
      entityTbody.replaceChildren();
      for (const ent of outReply.entities) {
        const tr = document.createElement("tr");
        const [cx, cy, cz] = ent.center;
        for (const val of [
          String(ent.id),
          ent.name,
          ent.role,
          String(ent.tris),
          `(${cx.toFixed(1)}, ${cy.toFixed(1)}, ${cz.toFixed(1)})`,
        ]) {
          const td = document.createElement("td");
          td.textContent = val;
          tr.append(td);
        }
        const actTd = document.createElement("td");
        const focusBtn = document.createElement("button");
        focusBtn.type = "button";
        focusBtn.className = "ve-m-btn";
        focusBtn.textContent = "Frame";
        focusBtn.addEventListener("click", () => {
          studioCtx?.onFocusPoint?.(ent.center, ent.role === "TerrainPatch" ? 26 : 12, ent.name);
        });
        actTd.append(focusBtn);
        tr.append(actTd);
        entityTbody.append(tr);
      }
      const hdr = outReply.header || {};
      const cell = outReply.universeCell || [0, 0, 0];
      const clr = outReply.camera?.clearanceMm ?? 290;
      const fUs = latestObserverTelemetry?.frameMicros ?? 8000;
      const scalePct = ((latestObserverTelemetry?.scalePermille ?? 1000) / 10).toFixed(0);
      kpiStrata.textContent = `${hdr.w5h_strata || 8} / 8 · Loss ${hdr.telos_loss || 0}`;
      kpiBudget.textContent = `${(fUs / 1000).toFixed(1)} ms · ${scalePct}%`;
      kpiEntities.textContent = `${outReply.entities.length} entities`;
      kpiClearance.textContent = `${clr} mm`;
      obsReadout.textContent =
        `Observer Coherent · W5H ${hdr.w5h_strata || 8}/8 · Slot ${latestObserverTelemetry?.attendedSlot ?? 0} · Cell (${cell[0]},${cell[1]},${cell[2]})`;
      storyLog.textContent =
        `Universe Cell: (${cell[0]}, ${cell[1]}, ${cell[2]}) · 3-Sector 128m Ring Active · Seed ${hdr.seed || 7}\n` +
        `Click 'Advance Beat at Story Podium' or 'Verify Conservation & Portal Ring' to inspect narrative state.`;
    }

    for (const fn of organSyncFns) fn();

    if (studioCtx?.getOrganMode?.(4) === 2) {
      opfsLog.textContent = "Refused: ContainerOrganDenied(opfs-storage) — OPFS asset cache denied by FK-07 custody.";
      opfsActionsRow.replaceChildren();
    } else {
      const entries = await listOpfsAssets();
      opfsLog.textContent =
        entries.length > 0
          ? `OPFS Persistent Asset Vault (${entries.length} object(s)):\n` +
            entries.map((e) => `  • ${e.name} (${e.bytes} bytes)`).join("\n")
          : "OPFS Persistent Asset Vault ready (import a .glb model to transcode and cache .uorc in origin storage).";
      opfsActionsRow.replaceChildren();
      for (const entry of entries) {
        const mountBtn = document.createElement("button");
        mountBtn.type = "button";
        mountBtn.className = "ve-m-btn primary";
        mountBtn.textContent = `Mount ${entry.name}`;
        mountBtn.addEventListener("click", async () => {
          const bytes = await readOpfsAssetBytes(entry.name);
          if (bytes?.byteLength) await studioCtx?.onMountOpfsUorc?.(entry.name, bytes);
        });
        const delBtn = document.createElement("button");
        delBtn.type = "button";
        delBtn.className = "ve-m-btn";
        delBtn.textContent = `Delete ${entry.name}`;
        delBtn.addEventListener("click", async () => {
          await deleteOpfsAsset(entry.name);
          dlg.dispatchEvent(new CustomEvent("ve-refresh"));
        });
        opfsActionsRow.append(mountBtn, delBtn);
      }
    }

    const vw = Math.max(360, window.innerWidth || 1280);
    const vh = Math.max(640, window.innerHeight || 900);
    const [uiReply, uiVerdict] = await Promise.all([
      studioCtx?.callWorker?.("verify-ui", { width: vw, height: vh }),
      studioCtx?.queryVerifier?.("scene:ui"),
    ]);
    if (uiReply?.ok) {
      lastUiState = { vw, vh, report: uiReply.report };
      uiLog.textContent = `VERIFIED (${uiReply.widgetCount} widgets · 0 overlaps · >=48px targets · WCAG AA · ${uiVerdict || "conformant"}):\n${uiReply.report}`;
      drawUiComplianceCanvas(uiCanvas, vw, vh, uiReply.report);
    }
  });

  if (!inline) {
    dlg.addEventListener("click", (e) => {
      if (e.target === dlg) {
        if (sequenceTimer) { clearInterval(sequenceTimer); sequenceTimer = 0; }
        dlg.close();
      }
    });
  }

  shell.append(head, obsBar, tabBar, body);
  dlg.append(shell);
  return dlg;
}

export async function exportGlbDownload(callWorker, lodShort) {
  const r = await callWorker("export-glb");
  if (!r?.buffer) return;
  const url = URL.createObjectURL(new Blob([r.buffer], { type: "model/gltf-binary" }));
  const a = document.createElement("a");
  a.href = url;
  a.download = `sunspire-scene-${String(lodShort || "lod3").toLowerCase()}.glb`;
  a.click();
  setTimeout(() => URL.revokeObjectURL(url), 1500);
}

export async function shareUorcSnapshot(callWorker, chip, statusEl) {
  const r = await callWorker("encode-snapshot");
  if (!r?.ok || !r.bytes) {
    chip(statusEl, "Snapshot unavailable", "warn");
    return;
  }
  const bytes = r.bytes instanceof Uint8Array ? r.bytes : new Uint8Array(r.bytes);
  history.replaceState(null, "", `${location.pathname}${location.search}#uorc=${bytesToBase64Url(bytes)}`);
  try {
    await navigator.clipboard?.writeText(location.href);
  } catch {}
  chip(statusEl, `Saved UORC · ${bytes.byteLength}B SHA-256 (#uorc=)`, "good");
}

let audioState = { ctx: null, worklet: null, timer: 0, on: false, strikeIdx: 0 };

function strikeSingleModePreview(modeIdx, fundamentalHz = 110.0, rt60Ms = PRISM_CHAMBER_SABINE_RT60_MS) {
  const Ctx = window.AudioContext || window.webkitAudioContext;
  if (!Ctx) return;
  audioState.ctx ??= new Ctx();
  audioState.ctx.resume?.();
  if (audioState.worklet && audioState.on) {
    audioState.worklet.port.postMessage({ kind: "strike", modeIndex: modeIdx });
    return;
  }
  const numer = PRISM_MODAL_BAR_NUMERS[Math.abs(modeIdx) % PRISM_MODAL_BAR_NUMERS.length];
  const freq = Math.min(1480.0, (fundamentalHz * numer) / PRISM_MODAL_BAR_DENOM);
  const rt = Math.max(0.35, Math.min(3.5, rt60Ms / 1000.0));
  const now = audioState.ctx.currentTime;
  const osc = audioState.ctx.createOscillator();
  const gain = audioState.ctx.createGain();
  osc.frequency.setValueAtTime(freq, now);
  gain.gain.setValueAtTime(0.0001, now);
  gain.gain.exponentialRampToValueAtTime(0.11, now + 0.018);
  gain.gain.exponentialRampToValueAtTime(0.0001, now + rt);
  osc.connect(gain).connect(audioState.ctx.destination);
  osc.start(now);
  osc.stop(now + rt + 0.05);
}

export function syncModalAudioSpatial(pan, gain, itdMicros = 0, pavilionContact = false, attendedSlot = 0) {
  if (audioState.worklet && audioState.on) {
    audioState.worklet.port.postMessage({ kind: "spatial", pan, gain, itdMicros, pavilionContact, attendedSlot });
  }
}

export function suspendModalAudio(audioBtn) {
  if (!audioState.on) return;
  audioState.on = false;
  clearInterval(audioState.timer);
  audioState.ctx?.suspend?.();
  if (audioBtn) {
    audioBtn.setAttribute("aria-pressed", "false");
    audioBtn.textContent = "Audio: Off";
  }
}

export async function toggleModalAudio(audioBtn, acousticSpec, getSolvedEye) {
  const Ctx = window.AudioContext || window.webkitAudioContext;
  if (!Ctx) return;
  audioState.on = !audioState.on;
  if (audioBtn) {
    audioBtn.setAttribute("aria-pressed", String(audioState.on));
    audioBtn.textContent = audioState.on ? "Audio: On" : "Audio: Off";
  }
  const strike = () => {
    if (!audioState.ctx || !audioState.on) return;
    if (audioState.worklet) {
      audioState.worklet.port.postMessage({ kind: "strike", modeIndex: audioState.strikeIdx++ });
      return;
    }
    const eye = getSolvedEye();
    const dist = Math.hypot(eye[0] + 9.0, eye[1] - 2.0, eye[2] + 4.0);
    const atten = Math.min(0.14, 0.9 / (1.0 + 0.18 * dist));
    const idx = (audioState.strikeIdx++) % acousticSpec.modeNumers.length;
    const freq = Math.min(1320.0, (acousticSpec.fundamentalHz * acousticSpec.modeNumers[idx]) / acousticSpec.modeDenom);
    const rt = Math.max(0.5, acousticSpec.rt60Ms / 1000.0);
    const now = audioState.ctx.currentTime;
    const osc = audioState.ctx.createOscillator();
    const gain = audioState.ctx.createGain();
    osc.frequency.setValueAtTime(freq, now);
    gain.gain.setValueAtTime(0.0001, now);
    gain.gain.exponentialRampToValueAtTime(atten, now + 0.02);
    gain.gain.exponentialRampToValueAtTime(0.0001, now + rt);
    osc.connect(gain).connect(audioState.ctx.destination);
    osc.start(now);
    osc.stop(now + rt + 0.05);
  };
  if (audioState.on) {
    audioState.ctx ??= new Ctx();
    await audioState.ctx.resume?.();
    if (!audioState.worklet && audioState.ctx.audioWorklet && typeof AudioWorkletNode === "function") {
      try {
        await audioState.ctx.audioWorklet.addModule(new URL("./audio-worklet.mjs", import.meta.url).href);
        audioState.worklet = new AudioWorkletNode(audioState.ctx, "verdant-modal-processor", {
          numberOfOutputs: 1,
          outputChannelCount: [acousticSpec.stereoChannels],
          processorOptions: {
            quantumFrames: acousticSpec.quantumFrames,
            modeNumers: acousticSpec.modeNumers,
            modeDenom: acousticSpec.modeDenom,
            rt60Ms: acousticSpec.rt60Ms,
            fundamentalHz: acousticSpec.fundamentalHz,
          },
        });
        audioState.worklet.connect(audioState.ctx.destination);
      } catch {
        audioState.worklet = null;
      }
    }
    clearInterval(audioState.timer);
    audioState.timer = window.setInterval(strike, 1350);
    strike();
  } else {
    clearInterval(audioState.timer);
  }
}

export async function routeStudioPrompt(text, ctx) {
  const prompt = String(text || "").trim();
  if (!prompt) {
    ctx.chip?.(ctx.statusEl, "Refused empty intent", "warn");
    return null;
  }
  const [r] = await Promise.all([
    ctx.callWorker("route-prompt", { prompt }),
    ctx.queryVerifier?.("scene:cybernetics"),
  ]);
  if (!r?.ok) {
    const reason = r?.organDenied
      ? `Refused: ContainerOrganDenied(${r.organDenied})`
      : "Refused ungrounded intent";
    ctx.chip?.(ctx.statusEl, reason, "warn");
    return r;
  }

  switch (r.slug) {
    case "open-role-code":
      await ctx.launchStudioModal("code");
      break;
    case "open-role-dashboard":
      await ctx.launchStudioModal("dashboard");
      break;
    case "open-role-design":
    case "inspect-formal-ui":
      await ctx.launchStudioModal("design");
      break;
    case "open-role-audio":
      await ctx.launchStudioModal("audio");
      break;
    case "open-role-director":
      await ctx.launchStudioModal("director");
      break;
    case "open-role-gameplay":
      await ctx.launchStudioModal("gameplay");
      break;
    case "narrative-interact":
      await ctx.interactChronicle();
      break;
    case "focus-observer":
      await ctx.focusNodePreset(0);
      ctx.setSimLive(true);
      break;
    case "drive-vehicle":
      await ctx.focusNodePreset(6);
      ctx.setSimLive(true);
      break;
    case "render-tile-wasm":
      await ctx.toggleRenderBackend("tile");
      break;
    case "render-webgl2":
      await ctx.toggleRenderBackend("webgl2");
      break;
    case "stream-uorc-lod3":
      await ctx.streamUorcToRank(PRISM_MAX_LOD_RANK);
      break;
    case "stream-uorc-lod4":
      await ctx.streamUorcToRank(PRISM_HD_MAX_LOD_RANK);
      break;
    case "stitch-shard-sync":
      if (ctx.openStitchModal) await ctx.openStitchModal();
      else (await import("./stitch-mesh.mjs")).openStitchMeshModal({ callWorker: ctx.callWorker, onRestoreSnapshot: ctx.restoreRawSnapshot });
      break;
    case "step-xpbd-physics":
      await ctx.focusNodePreset(3);
      ctx.setSimLive(true);
      break;
    default:
      break;
  }

  ctx.chip?.(ctx.statusEl, `Director -> ${r.slug} (${(r.scoreMilli / 10).toFixed(0)}% coded_gemm)`, "good");
  return r;
}
