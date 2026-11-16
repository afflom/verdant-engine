// Verdant Engine — Live Developer Lifecycle, Formal UI Solver, Cybernetic LIF/SLM, & Container
// Saturation Studio. Lazy-loaded on demand (zero cold-boot byte impact). Zero inline styles (CSP).
// Every panel executes real WebAssembly functions inside site/engine-worker.mjs.

const STUDIO_CSS = `
dialog.ve-modal {
  width: min(95vw, 62rem);
  max-height: min(90dvh, 52rem);
  margin: auto;
  padding: 0;
  border: 1.5px solid #2e5948;
  border-radius: 0.875rem;
  background: #0d151c;
  color: #f2f8fc;
  box-shadow: 0 24px 64px rgba(0, 0, 0, 0.76);
  overflow: hidden;
}
html:not(.dark) dialog.ve-modal {
  background: #f8fafc;
  color: #111c26;
  border-color: #9eb6c9;
  box-shadow: 0 24px 64px rgba(15, 23, 42, 0.22);
}
dialog.ve-modal::backdrop {
  background: rgba(5, 10, 14, 0.8);
  backdrop-filter: blur(6px);
  -webkit-backdrop-filter: blur(6px);
}
.ve-m-shell {
  display: flex;
  flex-direction: column;
  max-height: min(90dvh, 52rem);
}
.ve-m-head {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 0.75rem;
  padding: 0.875rem 1.25rem;
  border-bottom: 1px solid #22384a;
  background: #111c26;
}
html:not(.dark) .ve-m-head {
  background: #eaf1f7;
  border-bottom-color: #cbd5e1;
}
.ve-m-title {
  margin: 0;
  font-family: var(--font-display);
  font-size: 1.125rem;
  font-weight: 600;
}
.ve-m-sub {
  margin: 0.125rem 0 0;
  font-family: var(--font-mono);
  font-size: 0.8125rem;
  color: #8ce4be;
}
html:not(.dark) .ve-m-sub {
  color: #0e6346;
}
.ve-m-tabs {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
  padding: 0.625rem 1.25rem;
  border-bottom: 1px solid #1e3142;
  background: #0f1821;
}
html:not(.dark) .ve-m-tabs {
  background: #f1f5f9;
  border-bottom-color: #cbd5e1;
}
.ve-m-btn,
.ve-m-tab {
  min-height: 3rem;
  min-width: 3rem;
  padding: 0 0.875rem;
  border-radius: 0.5rem;
  border: 1px solid #2f4b63;
  background: #162432;
  color: #eef6fc;
  font-size: 0.875rem;
  font-weight: 600;
  cursor: pointer;
}
html:not(.dark) .ve-m-btn,
html:not(.dark) .ve-m-tab {
  background: #ffffff;
  color: #152536;
  border-color: #94a3b8;
}
.ve-m-tab[aria-selected="true"],
.ve-m-btn.primary {
  background: #115e46;
  border-color: #34d399;
  color: #ecfdf5;
}
html:not(.dark) .ve-m-tab[aria-selected="true"],
html:not(.dark) .ve-m-btn.primary {
  background: #0f7656;
  border-color: #047857;
  color: #ffffff;
}
.ve-m-body {
  padding: 1rem 1.25rem 1.25rem;
  overflow-y: auto;
  display: grid;
  gap: 1rem;
}
.ve-m-panel[hidden] {
  display: none;
}
.ve-m-card {
  border: 1px solid #243b4f;
  border-radius: 0.75rem;
  background: #121e29;
  padding: 1rem;
  display: grid;
  gap: 0.75rem;
}
html:not(.dark) .ve-m-card {
  background: #ffffff;
  border-color: #cbd5e1;
}
.ve-m-card h3 {
  margin: 0;
  font-size: 1rem;
  font-weight: 600;
}
.ve-m-card p {
  margin: 0;
  font-size: 0.9375rem;
  line-height: 1.5;
  color: #d6e4f0;
}
html:not(.dark) .ve-m-card p {
  color: #263849;
}
.ve-m-row {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
  align-items: center;
}
.ve-m-canvas {
  width: 100%;
  height: auto;
  border-radius: 0.5rem;
  border: 1px solid #294359;
  background: #091017;
  display: block;
}
.ve-m-log {
  margin: 0;
  padding: 0.75rem;
  border-radius: 0.5rem;
  border: 1px solid #253d52;
  background: #091017;
  color: #d8f3e5;
  font-family: var(--font-mono);
  font-size: 0.8125rem;
  line-height: 1.45;
  white-space: pre-wrap;
  overflow-x: auto;
}
html:not(.dark) .ve-m-log {
  background: #f1f5f9;
  color: #0f291e;
  border-color: #cbd5e1;
}
.ve-organ-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(15rem, 1fr));
  gap: 0.625rem;
}
.ve-organ-item {
  border: 1px solid #284257;
  border-radius: 0.625rem;
  padding: 0.625rem 0.75rem;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 0.5rem;
  background: #0d1720;
}
html:not(.dark) .ve-organ-item {
  background: #f8fafc;
  border-color: #cbd5e1;
}
.ve-organ-meta {
  display: grid;
  gap: 0.125rem;
}
.ve-organ-name {
  font-family: var(--font-mono);
  font-size: 0.8125rem;
  font-weight: 600;
}
.ve-organ-state {
  font-size: 0.75rem;
  color: #8ce4be;
}
html:not(.dark) .ve-organ-state {
  color: #0f6848;
}
`;

let sheetInstalled = false;
export function ensureModalStyles() {
  if (sheetInstalled || typeof CSSStyleSheet === "undefined") return;
  try {
    const sheet = new CSSStyleSheet();
    sheet.replaceSync(STUDIO_CSS);
    document.adoptedStyleSheets = [...document.adoptedStyleSheets, sheet];
    sheetInstalled = true;
  } catch {
    // Fallback ignored under strict CSP if adoptedStyleSheets unavailable.
  }
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

const CONTAINER_ORGANS = [
  { idx: 0, slug: "wasm-core", label: "WebAssembly Sandbox", defaultAllowed: true },
  { idx: 1, slug: "offscreen-canvas", label: "OffscreenCanvas Viewport", defaultAllowed: true },
  { idx: 2, slug: "service-worker", label: "ServiceWorker Offline Cache", defaultAllowed: true },
  { idx: 3, slug: "audio-worklet", label: "AudioWorklet 128f Synth", defaultAllowed: false },
  { idx: 4, slug: "opfs-storage", label: "OPFS Persistent Asset Cache", defaultAllowed: false },
  { idx: 5, slug: "gamepad-hid", label: "Gamepad HID Controller", defaultAllowed: false },
  { idx: 6, slug: "device-orientation", label: "DeviceOrientation Pose", defaultAllowed: false },
  { idx: 7, slug: "webcrypto-prism-id", label: "WebCrypto Prism Identity", defaultAllowed: true },
  { idx: 8, slug: "broadcast-channel", label: "BroadcastChannel Multi-Tab", defaultAllowed: true },
  { idx: 9, slug: "webrtc-stitch-mesh", label: "WebRTC Stitch Peer Mesh", defaultAllowed: false },
  { idx: 10, slug: "webnn-local", label: "WebNN Local Tensor Probe", defaultAllowed: false },
];

let studioDialog = null;
let latestObserverTelemetry = null;
let observerDrawHook = null;

export function pushStudioTelemetry(telemetry) {
  latestObserverTelemetry = telemetry;
  if (studioDialog?.open && typeof observerDrawHook === "function") {
    observerDrawHook(telemetry);
  }
}

export function openStudioModal({
  callWorker,
  onPickGlb,
  onResetGlb,
  onToggleRenderMode,
  onRoutePrompt,
  getRenderMode,
}) {
  ensureModalStyles();
  if (!studioDialog) {
    studioDialog = buildStudioDialog({
      callWorker,
      onPickGlb,
      onResetGlb,
      onToggleRenderMode,
      onRoutePrompt,
      getRenderMode,
    });
    document.body.append(studioDialog);
  }
  if (!studioDialog.open) {
    studioDialog.showModal();
    studioDialog.dispatchEvent(new CustomEvent("ve-refresh"));
  }
}

function drawObserverCanvas(canvas, tel) {
  const ctx = canvas.getContext("2d");
  if (!ctx) return;
  const w = canvas.width;
  const h = canvas.height;
  ctx.fillStyle = "#081018";
  ctx.fillRect(0, 0, w, h);

  const mask = tel?.spikeMask ?? 5;
  const salience = tel?.salience ?? [420, 180, 260, 310, 190, 220, 390];
  const attended = tel?.attendedSlot ?? 0;

  ctx.fillStyle = "#9ec8e8";
  ctx.font = "600 12px monospace";
  ctx.fillText(`Q15 LIF SPIKE RASTER (8 NEURONS) · MASK 0b${mask.toString(2).padStart(8, "0")}`, 16, 24);

  for (let n = 0; n < 8; n++) {
    const spiked = ((mask >>> n) & 1) === 1;
    const bx = 16 + n * 68;
    ctx.fillStyle = spiked ? "#10b981" : "#182838";
    ctx.fillRect(bx, 34, 58, 28);
    ctx.strokeStyle = spiked ? "#6ee7b7" : "#2f4a63";
    ctx.strokeRect(bx, 34, 58, 28);
    ctx.fillStyle = spiked ? "#042f2e" : "#cbd5e1";
    ctx.fillText(`N${n}:${spiked ? "1" : "0"}`, bx + 12, 52);
  }

  ctx.fillStyle = "#9ec8e8";
  ctx.fillText(
    `MUTUAL-ATTENTION SALIENCE (ATTENDED NODE ${attended} · GIMBAL YAW ${tel?.gimbalYawMilli ?? 0} mrad)`,
    16,
    88,
  );

  const maxSal = Math.max(500, ...salience.map((v) => Math.abs(v)));
  for (let s = 0; s < salience.length; s++) {
    const val = Math.max(0, salience[s]);
    const barW = Math.max(12, Math.round((val / maxSal) * (w - 170)));
    const y = 100 + s * 22;
    ctx.fillStyle = "#cbd5e1";
    ctx.fillText(`Node ${s}`, 16, y + 13);
    ctx.fillStyle = s === attended ? "#34d399" : "#38bdf8";
    ctx.fillRect(82, y, barW, 15);
    ctx.fillStyle = "#e2e8f0";
    ctx.fillText(`${val}`, 90 + barW, y + 13);
  }
}

function drawUiComplianceCanvas(canvas, vw, vh, reportText) {
  const ctx = canvas.getContext("2d");
  if (!ctx) return;
  const cw = canvas.width;
  const ch = canvas.height;
  ctx.fillStyle = "#081018";
  ctx.fillRect(0, 0, cw, ch);

  const lines = String(reportText || "").split("\n").slice(1);
  const sx = (cw - 24) / Math.max(1, vw);
  const sy = (ch - 36) / Math.max(1, vh);

  ctx.fillStyle = "#8ce4be";
  ctx.font = "600 12px monospace";
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
    ctx.strokeStyle = idx % 2 === 0 ? "#34d399" : "#38bdf8";
    ctx.fillStyle = idx % 2 === 0 ? "rgba(16, 185, 129, 0.16)" : "rgba(56, 189, 248, 0.14)";
    ctx.fillRect(rx, ry, pw, ph);
    ctx.strokeRect(rx, ry, pw, ph);
    ctx.fillStyle = "#f1f5f9";
    ctx.font = "11px monospace";
    ctx.fillText(`${label} (${rw}x${rh})`, rx + 4, ry + 12);
  });
}

function buildStudioDialog({
  callWorker,
  onPickGlb,
  onResetGlb,
  onToggleRenderMode,
  onRoutePrompt,
  getRenderMode,
}) {
  const dlg = document.createElement("dialog");
  dlg.className = "ve-modal";
  dlg.setAttribute("aria-label", "Verdant Engine Live Developer Studio");

  const shell = document.createElement("div");
  shell.className = "ve-m-shell";

  const head = document.createElement("header");
  head.className = "ve-m-head";
  const titleWrap = document.createElement("div");
  const h2 = document.createElement("h2");
  h2.className = "ve-m-title";
  h2.textContent = "Verdant Engine Studio · 12 Semiring Kernels in WebAssembly";
  const sub = document.createElement("p");
  sub.className = "ve-m-sub";
  sub.textContent = "100% dogfooded on verdant-wasm · GLB -> UORC · Formal UI · LIF/SLM · Container Grants";
  titleWrap.append(h2, sub);

  const closeBtn = document.createElement("button");
  closeBtn.type = "button";
  closeBtn.className = "ve-m-btn";
  closeBtn.textContent = "Close";
  closeBtn.addEventListener("click", () => dlg.close());
  head.append(titleWrap, closeBtn);

  const tabBar = document.createElement("div");
  tabBar.className = "ve-m-tabs";
  tabBar.setAttribute("role", "tablist");

  const body = document.createElement("div");
  body.className = "ve-m-body";

  // Panel 0: Authoring & Render Pipeline (.glb -> .uorc + OPFS + WebGL2/Tile)
  const pAuthor = document.createElement("section");
  pAuthor.className = "ve-m-panel";
  const cAuthor = document.createElement("div");
  cAuthor.className = "ve-m-card";
  const hAuthor = document.createElement("h3");
  hAuthor.textContent = "Khronos .glb Import, Progressive .uorc Streaming, & Dual Rasterizer";
  const pAuthorDesc = document.createElement("p");
  pAuthorDesc.textContent =
    "Import any binary glTF 2.0 (.glb) model directly into Rust SceneWorld inside WebAssembly, cache snapshots in the Origin Private File System (OPFS), and toggle live between WebGL2 hardware draw calls and Prism's 16x16 uor-matmul software tile PBR rasterizer.";
  const authorRow = document.createElement("div");
  authorRow.className = "ve-m-row";
  const btnPick = document.createElement("button");
  btnPick.type = "button";
  btnPick.className = "ve-m-btn primary";
  btnPick.textContent = "Import .glb File…";
  btnPick.addEventListener("click", () => onPickGlb?.());

  const btnRenderSwitch = document.createElement("button");
  btnRenderSwitch.type = "button";
  btnRenderSwitch.className = "ve-m-btn";
  const syncRenderLabel = () => {
    btnRenderSwitch.textContent =
      getRenderMode?.() === "tile" ? "Active: Wasm 16x16 Tile PBR" : "Active: WebGL2 Hardware";
  };
  syncRenderLabel();
  btnRenderSwitch.addEventListener("click", async () => {
    await onToggleRenderMode?.();
    syncRenderLabel();
  });

  const btnReset = document.createElement("button");
  btnReset.type = "button";
  btnReset.className = "ve-m-btn";
  btnReset.textContent = "Restore Proving Ground";
  btnReset.addEventListener("click", () => onResetGlb?.());
  authorRow.append(btnPick, btnRenderSwitch, btnReset);

  const opfsLog = document.createElement("pre");
  opfsLog.className = "ve-m-log";
  opfsLog.textContent = "Inspecting Origin Private File System (OPFS) cache…";
  cAuthor.append(hAuthor, pAuthorDesc, authorRow, opfsLog);
  pAuthor.append(cAuthor);

  // Panel 1: Neuromorphic Observer & Embedded coded_gemm SLM
  const pNeuro = document.createElement("section");
  pNeuro.className = "ve-m-panel";
  pNeuro.hidden = true;
  const cNeuro = document.createElement("div");
  cNeuro.className = "ve-m-card";
  const hNeuro = document.createElement("h3");
  hNeuro.textContent = "Second-Order Cybernetic LIF/SSM Loop & Zero-Decode coded_gemm SLM";
  const pNeuroDesc = document.createElement("p");
  pNeuroDesc.textContent =
    "Integer-exact Q15 Leaky-Integrate-and-Fire recurrence couples player gaze/motion with world salience (zero IEEE-754 drift), paired with an embedded Packed<Grid<16>, 2> language head.";
  const neuroCanvas = document.createElement("canvas");
  neuroCanvas.className = "ve-m-canvas";
  neuroCanvas.width = 580;
  neuroCanvas.height = 264;
  observerDrawHook = (tel) => drawObserverCanvas(neuroCanvas, tel);

  const slmRow = document.createElement("div");
  slmRow.className = "ve-m-row";
  const slmLog = document.createElement("pre");
  slmLog.className = "ve-m-log";
  slmLog.textContent = "Click a sample intent or submit any prompt in the Studio command bar.";
  for (const samplePrompt of [
    "focus observer core gimbal",
    "drive the 4-wheel suspension vehicle",
    "switch to software 16x16 tile raster",
    "shard world into stitch parity fragments",
  ]) {
    const b = document.createElement("button");
    b.type = "button";
    b.className = "ve-m-btn";
    b.textContent = samplePrompt;
    b.addEventListener("click", async () => {
      const res = await onRoutePrompt?.(samplePrompt);
      if (res?.ok) {
        slmLog.textContent = `coded_gemm Proposal Admitted:\n  intent: ${res.slug}\n  confidence: ${(res.scoreMilli / 10).toFixed(1)}% (raw_logit=${res.rawLogit})\n  action: ${res.summary}`;
      }
    });
    slmRow.append(b);
  }
  cNeuro.append(hNeuro, pNeuroDesc, neuroCanvas, slmRow, slmLog);
  pNeuro.append(cNeuro);

  // Panel 2: Formal UI & WCAG Compliance Proof
  const pUi = document.createElement("section");
  pUi.className = "ve-m-panel";
  pUi.hidden = true;
  const cUi = document.createElement("div");
  cUi.className = "ve-m-card";
  const hUi = document.createElement("h3");
  hUi.textContent = "Formal 2D/3D UI Semiring Solver & WCAG 2.2 Compliance (kernel9)";
  const pUiDesc = document.createElement("p");
  pUiDesc.textContent =
    "Solves intrinsic widget bounds over tropical (max, +), distributes flex remainders over exact (+, *), and proves >= 48x48 CSS px touch targets, WCAG contrast, focus reachability, and diegetic BVH bounds.";
  const uiCanvas = document.createElement("canvas");
  uiCanvas.className = "ve-m-canvas";
  uiCanvas.width = 580;
  uiCanvas.height = 240;
  const uiLog = document.createElement("pre");
  uiLog.className = "ve-m-log";
  cUi.append(hUi, pUiDesc, uiCanvas, uiLog);
  pUi.append(cUi);

  // Panel 3: Browser Container API Saturation (Denial-Wins Lattice)
  const pOrgans = document.createElement("section");
  pOrgans.className = "ve-m-panel";
  pOrgans.hidden = true;
  const cOrgans = document.createElement("div");
  cOrgans.className = "ve-m-card";
  const hOrgans = document.createElement("h3");
  hOrgans.textContent = "Browser Container API Saturation · Foundry FK-07 Denial-Wins Lattice";
  const pOrgansDesc = document.createElement("p");
  pOrgansDesc.textContent =
    "Explicit visitor capability grants govern non-sandboxed W3C container organs. Under FK-07, explicit Denial overrides any prior or future Grant until reset.";
  const organGrid = document.createElement("div");
  organGrid.className = "ve-organ-grid";

  for (const organ of CONTAINER_ORGANS) {
    const item = document.createElement("div");
    item.className = "ve-organ-item";
    const meta = document.createElement("div");
    meta.className = "ve-organ-meta";
    const nm = document.createElement("span");
    nm.className = "ve-organ-name";
    nm.textContent = organ.label;
    const st = document.createElement("span");
    st.className = "ve-organ-state";
    const hwPresent =
      organ.slug === "webnn-local"
        ? typeof navigator !== "undefined" && "ml" in navigator
        : organ.slug === "gamepad-hid"
          ? typeof navigator !== "undefined" && typeof navigator.getGamepads === "function"
          : true;
    st.textContent = organ.defaultAllowed
      ? "Allowed (Sandboxed)"
      : hwPresent
        ? "Gated (Requires Grant)"
        : "Probed (API absent)";
    meta.append(nm, st);

    const toggleBtn = document.createElement("button");
    toggleBtn.type = "button";
    toggleBtn.className = "ve-m-btn";
    let mode = organ.defaultAllowed ? 1 : 0;
    toggleBtn.textContent = organ.defaultAllowed ? "Deny" : "Grant";
    toggleBtn.addEventListener("click", async () => {
      mode = mode === 1 ? 2 : 1;
      const reply = await callWorker?.("set-organ-grant", { organIdx: organ.idx, mode });
      const allowed = Boolean(reply?.allowed);
      st.textContent = mode === 2 ? "Denied (FK-07 Wins)" : allowed ? "Granted & Active" : "Gated";
      toggleBtn.textContent = mode === 2 ? "Reset" : "Deny";
      if (mode === 2) mode = -1;
      else if (mode === -1) mode = 0;
    });
    item.append(meta, toggleBtn);
    organGrid.append(item);
  }
  cOrgans.append(hOrgans, pOrgansDesc, organGrid);
  pOrgans.append(cOrgans);

  const panels = [
    { label: "GLB & Raster", el: pAuthor },
    { label: "Neuro & SLM", el: pNeuro },
    { label: "Formal UI Proof", el: pUi },
    { label: "Container APIs", el: pOrgans },
  ];
  const tabBtns = panels.map((p, idx) => {
    const b = document.createElement("button");
    b.type = "button";
    b.className = "ve-m-tab";
    b.setAttribute("role", "tab");
    b.setAttribute("aria-selected", idx === 0 ? "true" : "false");
    b.textContent = p.label;
    b.addEventListener("click", () => {
      panels.forEach((it, j) => {
        it.el.hidden = j !== idx;
        tabBtns[j].setAttribute("aria-selected", j === idx ? "true" : "false");
      });
    });
    tabBar.append(b);
    body.append(p.el);
    return b;
  });

  dlg.addEventListener("ve-refresh", async () => {
    syncRenderLabel();
    drawObserverCanvas(neuroCanvas, latestObserverTelemetry);
    const entries = await listOpfsAssets();
    opfsLog.textContent =
      entries.length > 0
        ? `OPFS Persistent World Cache (${entries.length} object(s)):\n` +
          entries.map((e) => `  • ${e.name} (${e.bytes} bytes)`).join("\n")
        : "OPFS Persistent World Cache ready (import a .glb or stream .uorc to populate).";

    const vw = Math.max(360, window.innerWidth || 1280);
    const vh = Math.max(640, window.innerHeight || 900);
    const uiReply = await callWorker?.("verify-ui", { width: vw, height: vh });
    if (uiReply?.ok) {
      uiLog.textContent = `VERIFIED (${uiReply.widgetCount} widgets · 0 overlaps · >=48px targets · WCAG AA):\n${uiReply.report}`;
      drawUiComplianceCanvas(uiCanvas, vw, vh, uiReply.report);
    }
  });

  dlg.addEventListener("click", (e) => {
    if (e.target === dlg) dlg.close();
  });

  shell.append(head, tabBar, body);
  dlg.append(shell);
  return dlg;
}
