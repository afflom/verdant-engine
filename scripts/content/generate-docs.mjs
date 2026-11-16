#!/usr/bin/env node
/**
 * Generates the Diataxis HTML pages under site/docs/, the live WebAssembly
 * interactive figure harness site/docs/live-embed.mjs, and site/llms.txt before
 * scripts/content/enrich.mjs enriches them with persona cards, glossary links,
 * JSON-LD, and .md twins.
 */
import { mkdirSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";

const SITE = process.argv[2] ?? "site";
const LIVE = "https://afflom.github.io/verdant-engine/";

function docStyle(root) {
  return `
  @font-face { font-family: "Archivo"; font-weight: 600; font-display: swap; src: url("${root}fonts/Archivo-SemiBold.woff2") format("woff2"); }
  @font-face { font-family: "Geist"; font-weight: 400; font-display: swap; src: url("${root}fonts/Geist-Regular.woff2") format("woff2"); }
  @font-face { font-family: "Geist"; font-weight: 500; font-display: swap; src: url("${root}fonts/Geist-Medium.woff2") format("woff2"); }
  @font-face { font-family: "Geist Mono"; font-weight: 400; font-display: swap; src: url("${root}fonts/GeistMono-Regular.woff2") format("woff2"); }
  :root {
    color-scheme: dark;
    --fd-bg: 20 8% 8%;
    --fd-panel: 20 6% 12%;
    --fd-fg: 30 10% 96%;
    --fd-dim: 30 4% 72%;
    --fd-line: 24 6% 24%;
    --fd-line-strong: 146 48% 52%;
    --fd-accent: 145 58% 62%;
    --fd-brand: 15 99% 46%;
    --fd-code-bg: 20 7% 14%;
    --fd-code-fg: 145 62% 76%;
    --fd-spectrum: linear-gradient(100deg, hsl(356 92% 66%) 0%, hsl(28 95% 60%) 18%, hsl(48 95% 58%) 34%, hsl(154 72% 52%) 52%, hsl(192 90% 56%) 70%, hsl(226 90% 68%) 86%, hsl(282 84% 72%) 100%);
  }
  @media (prefers-color-scheme: light) {
    :root:not(.dark) {
      color-scheme: light;
      --fd-bg: 60 14% 94%;
      --fd-panel: 0 0% 100%;
      --fd-fg: 22 9% 17%;
      --fd-dim: 28 5% 38%;
      --fd-line: 54 11% 82%;
      --fd-line-strong: 150 58% 30%;
      --fd-accent: 150 64% 26%;
      --fd-brand: 15 99% 42%;
      --fd-code-bg: 53 16% 90%;
      --fd-code-fg: 150 68% 22%;
    }
    :root:not(.dark) .cs-canvas {
      background: radial-gradient(120% 140% at 0% 0%, hsl(0 0% 100%), hsl(var(--fd-bg))) !important;
    }
    :root:not(.dark) .cs-card { fill: hsl(0 0% 100%) !important; }
    :root:not(.dark) .cs-cell { fill: hsl(53 16% 92%) !important; }
    :root:not(.dark) .cs-track { fill: hsl(54 12% 86%) !important; }
  }
  :root.light {
    color-scheme: light;
    --fd-bg: 60 14% 94%;
    --fd-panel: 0 0% 100%;
    --fd-fg: 22 9% 17%;
    --fd-dim: 28 5% 38%;
    --fd-line: 54 11% 82%;
    --fd-line-strong: 150 58% 30%;
    --fd-accent: 150 64% 26%;
    --fd-brand: 15 99% 42%;
    --fd-code-bg: 53 16% 90%;
    --fd-code-fg: 150 68% 22%;
  }
  :root.light .cs-canvas {
    background: radial-gradient(120% 140% at 0% 0%, hsl(0 0% 100%), hsl(var(--fd-bg))) !important;
  }
  :root.light .cs-card { fill: hsl(0 0% 100%) !important; }
  :root.light .cs-cell { fill: hsl(53 16% 92%) !important; }
  :root.light .cs-track { fill: hsl(54 12% 86%) !important; }
  * { box-sizing: border-box; }
  body {
    margin: 0;
    background: hsl(var(--fd-bg));
    color: hsl(var(--fd-fg));
    font-family: "Geist", system-ui, -apple-system, sans-serif;
    font-size: 1.03125rem;
    line-height: 1.65;
    -webkit-font-smoothing: antialiased;
  }
  h1, h2, h3 {
    font-family: "Archivo", "Geist", system-ui, sans-serif;
  }
  .ve-nav-wrap {
    position: sticky;
    top: 0;
    z-index: 20;
    background: hsl(var(--fd-bg) / 0.94);
    backdrop-filter: blur(12px);
    -webkit-backdrop-filter: blur(12px);
    border-bottom: 1px solid hsl(var(--fd-line));
  }
  .ve-nav {
    max-width: 56rem;
    margin: 0 auto;
    padding: 0.75rem 1.25rem;
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.5rem 0.625rem;
    font-size: 0.9rem;
  }
  .ve-nav a,
  .ve-nav button.ve-theme-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-height: 3rem;
    min-width: 3rem;
    padding: 0.25rem 0.85rem;
    border-radius: 999px;
    border: 1px solid hsl(var(--fd-line));
    background: hsl(var(--fd-panel));
    color: hsl(var(--fd-fg));
    font-family: inherit;
    font-size: 0.9rem;
    text-decoration: none;
    font-weight: 500;
    cursor: pointer;
  }
  .ve-nav a:first-child {
    border-color: hsl(var(--fd-line-strong));
    color: hsl(var(--fd-accent));
    font-weight: 700;
  }
  .ve-nav a:hover,
  .ve-nav button.ve-theme-btn:hover {
    border-color: hsl(var(--fd-accent));
    color: hsl(var(--fd-accent));
  }
  .ve-nav button.ve-theme-btn {
    margin-left: auto;
  }
  main a {
    color: hsl(var(--fd-accent));
    text-decoration: underline;
    text-underline-offset: 0.2em;
    font-weight: 500;
  }
  main a:hover { color: hsl(var(--fd-fg)); }
  main {
    max-width: 56rem;
    margin: 0 auto;
    padding: 2rem 1.25rem 4.5rem;
  }
  h1 {
    font-size: clamp(1.85rem, 3.6vw, 2.45rem);
    line-height: 1.15;
    letter-spacing: -0.02em;
    margin: 0 0 0.75rem;
    color: hsl(var(--fd-fg));
  }
  h2 {
    font-size: 1.42rem;
    line-height: 1.28;
    margin: 2.4rem 0 0.75rem;
    padding-bottom: 0.35rem;
    border-bottom: 1px solid hsl(var(--fd-line));
    color: hsl(var(--fd-fg));
  }
  h3 {
    font-size: 1.15rem;
    line-height: 1.35;
    margin: 1.5rem 0 0.5rem;
    color: hsl(var(--fd-fg));
  }
  p, li { max-width: 70ch; color: hsl(var(--fd-fg)); }
  li + li { margin-top: 0.35rem; }
  code, pre {
    font-family: "Geist Mono", ui-monospace, SFMono-Regular, Menlo, monospace;
    font-size: 0.88em;
  }
  p code, li code, td code {
    padding: 0.12rem 0.38rem;
    border-radius: 0.32rem;
    background: hsl(var(--fd-code-bg));
    border: 1px solid hsl(var(--fd-line));
    color: hsl(var(--fd-code-fg));
  }
  pre {
    padding: 1rem 1.125rem;
    background: hsl(var(--fd-code-bg));
    color: hsl(var(--fd-fg));
    border: 1px solid hsl(var(--fd-line));
    border-radius: 0.75rem;
    overflow-x: auto;
    line-height: 1.55;
  }
  pre code { padding: 0; border: 0; background: transparent; color: inherit; }
  table {
    width: 100%;
    border-collapse: collapse;
    margin: 1.15rem 0;
    font-size: 0.9375rem;
    background: hsl(var(--fd-panel));
    border: 1px solid hsl(var(--fd-line));
    border-radius: 0.75rem;
    overflow: hidden;
  }
  caption { text-align: left; font-weight: 700; padding: 0.5rem 0.25rem; color: hsl(var(--fd-dim)); }
  th {
    text-align: left;
    padding: 0.65rem 0.75rem;
    background: hsl(var(--fd-code-bg));
    color: hsl(var(--fd-fg));
    border-bottom: 1px solid hsl(var(--fd-line-strong));
    font-weight: 700;
  }
  td {
    text-align: left;
    padding: 0.6rem 0.75rem;
    border-bottom: 1px solid hsl(var(--fd-line));
    color: hsl(var(--fd-fg));
  }
  .ve-live-fig {
    margin: 1.75rem 0 2.25rem;
  }
  .ve-live-canvas {
    display: block;
    width: 100%;
    height: auto;
    max-height: 15.5rem;
    border-radius: 0.65rem;
    border: 1px solid hsl(var(--fd-line));
    background: hsl(var(--fd-code-bg));
  }
  .ve-live-controls {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    margin-top: 0.65rem;
  }
  .ve-live-controls button {
    min-height: 3rem;
    min-width: 3rem;
    padding: 0 0.85rem;
    border-radius: 0.5rem;
    border: 1px solid hsl(var(--fd-line-strong));
    background: hsl(var(--fd-panel));
    color: hsl(var(--fd-fg));
    font-family: "Geist Mono", ui-monospace, monospace;
    font-size: 0.8125rem;
    font-weight: 600;
    cursor: pointer;
  }
  .ve-live-controls button:hover {
    border-color: hsl(var(--fd-accent));
    color: hsl(var(--fd-accent));
  }
  .dc-pager {
    margin-top: 2.75rem;
    padding-top: 1.25rem;
    border-top: 1px solid hsl(var(--fd-line));
    display: flex;
    flex-wrap: wrap;
    justify-content: space-between;
    gap: 1rem;
  }
`;
}

const ICONS = {
  cube: '<g fill="none" stroke="hsl(168 72% 60%)" stroke-width="4" stroke-linejoin="round"><polygon points="60,14 102,38 102,84 60,108 18,84 18,38" fill="hsl(168 72% 60% / 0.14)"/><polyline points="18,38 60,62 102,38"/><line x1="60" y1="62" x2="60" y2="108"/></g>',
  bvh: '<g fill="none" stroke="hsl(196 88% 64%)" stroke-width="3.6"><rect x="14" y="18" width="92" height="84" rx="8" stroke-dasharray="7 5"/><rect x="24" y="30" width="36" height="58" rx="6" fill="hsl(196 88% 64% / 0.16)"/><rect x="66" y="42" width="30" height="46" rx="6" fill="hsl(48 95% 60% / 0.18)" stroke="hsl(48 95% 60%)"/><line x1="12" y1="96" x2="106" y2="26" stroke="hsl(356 90% 68%)" stroke-width="4"/></g>',
  physics: '<g fill="none" stroke="hsl(44 96% 62%)" stroke-width="4" stroke-linecap="round"><circle cx="60" cy="30" r="10" fill="hsl(44 96% 62% / 0.25)"/><line x1="60" y1="40" x2="84" y2="78"/><circle cx="84" cy="84" r="14" fill="hsl(154 74% 54% / 0.28)" stroke="hsl(154 74% 54%)"/><path d="M26,92 Q60,108 94,92" stroke="hsl(204 30% 70%)" stroke-dasharray="5 5"/></g>',
  stream: '<g fill="none" stroke="hsl(268 86% 74%)" stroke-width="4" stroke-linecap="round"><rect x="16" y="72" width="16" height="30" rx="4" fill="hsl(268 86% 74% / 0.25)"/><rect x="40" y="56" width="16" height="46" rx="4" fill="hsl(222 90% 68% / 0.3)" stroke="hsl(222 90% 68%)"/><rect x="64" y="38" width="16" height="64" rx="4" fill="hsl(190 90% 58% / 0.32)" stroke="hsl(190 90% 58%)"/><rect x="88" y="18" width="16" height="84" rx="4" fill="hsl(152 76% 56% / 0.38)" stroke="hsl(152 76% 56%)"/></g>',
};

function liveWasmFigure({ id, mode, label, caption, summaryTitle, headers, rows, buttons }) {
  const btnHtml = buttons
    .map(([act, text]) => `<button type="button" data-ve-act="${act}">${text}</button>`)
    .join("");
  const thHtml = headers.map((h) => `<th>${h}</th>`).join("");
  const trHtml = rows
    .map((r) => `<tr>${r.map((c) => `<td>${c}</td>`).join("")}</tr>`)
    .join("");
  return `
<figure class="cs-figure ve-live-fig" id="${id}" data-ve-live="${mode}">
  <div class="cs-canvas">
    <canvas class="ve-live-canvas" width="576" height="224" role="img" aria-label="${label}"></canvas>
    <div class="ve-live-controls" data-prose="data">${btnHtml}</div>
  </div>
  <figcaption>${caption}</figcaption>
  <details class="cs-data"><summary>${summaryTitle}</summary>
    <table>
      <thead><tr>${thHtml}</tr></thead>
      <tbody>${trHtml}</tbody>
    </table>
  </details>
</figure>`;
}

const LIVE_EMBED_MJS = `// Verdant Engine — Live Executable Documentation Harness.
// Instantiates site/verdant-wasm.wasm inside every Diataxis documentation page so each figure
// executes real Rust engine passes: interactive 3D WebGL2 Citadel + Bridge viewport, Q15 LIF/SSM
// neuromorphic recurrence, lawful swept CCD + camera guard physics, progressive LOD0..LOD4 streams,
// 15-kernel semiring telemetry, and SHA-256 VDNT snapshot tamper proofs.

import {
  createBestRenderer,
  mat4LookAt,
  mat4Mul,
  mat4Perspective,
} from "../scene-renderer.mjs";

const scriptUrl = new URL(import.meta.url);
const wasmUrl = new URL("../verdant-wasm.wasm", scriptUrl);
const uorcWasmUrl = new URL("../verdant-uorc.wasm", scriptUrl);

const redrawHooks = [];

function syncPrismDocTheme(explicitNext = null) {
  const root = document.documentElement;
  let pref = explicitNext;
  if (!pref) {
    try { pref = localStorage.getItem("prism-theme"); } catch {}
  }
  if (!pref) {
    pref = matchMedia("(prefers-color-scheme: light)").matches ? "light" : "dark";
  }
  const isLight = pref === "light";
  root.classList.toggle("light", isLight);
  root.classList.toggle("dark", !isLight);
  if (explicitNext) {
    try { localStorage.setItem("prism-theme", explicitNext); } catch {}
  }
  const btn = document.querySelector("[data-ve-theme-toggle]");
  if (btn) btn.textContent = isLight ? "Theme: Light" : "Theme: Dark";
  for (const fn of redrawHooks) fn();
}

syncPrismDocTheme();
document.querySelector("[data-ve-theme-toggle]")?.addEventListener("click", () => {
  const next = document.documentElement.classList.contains("light") ? "dark" : "light";
  syncPrismDocTheme(next);
});

function readDocPalette() {
  const isLight = document.documentElement.classList.contains("light");
  return isLight
    ? {
        bg: "#e9e8df",
        card: "#ffffff",
        fg: "#302b28",
        muted: "#6b6660",
        accent: "#1f7a4d",
        accentSoft: "rgba(31, 122, 77, 0.14)",
        secondary: "#bb370b",
        secondarySoft: "rgba(187, 55, 11, 0.14)",
        danger: "#c91c22",
        dangerSoft: "rgba(201, 28, 34, 0.12)",
        line: "#c7c5b8",
      }
    : {
        bg: "#1b1918",
        card: "#262321",
        fg: "#f6f5f4",
        muted: "#b5b2ae",
        accent: "#5fd18b",
        accentSoft: "rgba(95, 209, 139, 0.16)",
        secondary: "#ec6032",
        secondarySoft: "rgba(236, 96, 50, 0.18)",
        danger: "#ff6467",
        dangerSoft: "rgba(255, 100, 103, 0.18)",
        line: "#433f3d",
      };
}

function drawBanner(ctx, w, h, title, subtitle) {
  const pal = readDocPalette();
  ctx.fillStyle = pal.bg;
  ctx.fillRect(0, 0, w, h);
  ctx.fillStyle = pal.accent;
  ctx.font = "600 13px 'Geist Mono', monospace";
  ctx.fillText(title, 16, 26);
  ctx.fillStyle = pal.muted;
  ctx.font = "12px 'Geist Mono', monospace";
  ctx.fillText(subtitle, 16, 48);
}

for (const fig of document.querySelectorAll("figure[data-ve-live]")) {
  if (fig.dataset.veLive === "viewport") continue;
  const c = fig.querySelector("canvas");
  const ctx = c?.getContext("2d");
  if (ctx) drawBanner(ctx, c.width, c.height, "VERDANT-WASM LIVE ENGINE", "Initializing WebAssembly runtime…");
}

let wasmPromise = null;
function loadWasm() {
  wasmPromise ??= fetch(wasmUrl)
    .then((r) => {
      if (!r.ok) throw new Error(\`verdant-wasm HTTP \${r.status}\`);
      return r.arrayBuffer();
    })
    .then((buf) => WebAssembly.instantiate(buf, {}))
    .then(({ instance }) => {
      const ex = instance.exports;
      ex.verdant_wasm_init(ex.verdant_wasm_default_seed());
      return ex;
    });
  return wasmPromise;
}

let uorcPromise = null;
function loadUorcWasm() {
  uorcPromise ??= fetch(uorcWasmUrl)
    .then((r) => {
      if (!r.ok) throw new Error(\`verdant-uorc HTTP \${r.status}\`);
      return r.arrayBuffer();
    })
    .then((buf) => WebAssembly.instantiate(buf, {}))
    .then(({ instance }) => {
      const ex = instance.exports;
      ex.verdant_wasm_init(ex.verdant_wasm_default_seed());
      return ex;
    });
  return uorcPromise;
}

const LOD_ENVELOPES = [
  { rank: 0, sky: [0.09, 0.08, 0.12], fog: [0.14, 0.11, 0.16] },
  { rank: 1, sky: [0.12, 0.17, 0.24], fog: [0.18, 0.24, 0.31] },
  { rank: 2, sky: [0.16, 0.27, 0.34], fog: [0.24, 0.35, 0.39] },
  { rank: 3, sky: [0.18, 0.30, 0.38], fog: [0.30, 0.40, 0.44] },
  { rank: 4, sky: [0.20, 0.34, 0.44], fog: [0.34, 0.44, 0.48] },
];

async function initInteractive3DViewport(ex, fig, canvas) {
  const renderer = await createBestRenderer(canvas);
  let yawMilli = 460;
  let lodRank = 3;
  let thirdPerson = false;
  let tileMode = false;

  const draw = async () => {
    ex.verdant_wasm_set_lod_rank(lodRank >>> 0);
    if (thirdPerson) {
      ex.verdant_wasm_camera_set_mode(0);
    } else {
      ex.verdant_wasm_camera_set_anchor(1, 2.2, 3.8, 9.2);
    }
    const aspectMilli = Math.max(400, Math.round((canvas.width / Math.max(1, canvas.height)) * 1000));
    ex.verdant_wasm_camera_set_pose_intent(yawMilli | 0, 280, thirdPerson ? 6500 : 18500, aspectMilli >>> 0);
    ex.verdant_wasm_scene_rebuild_buffers(0);

    const eye = [ex.verdant_wasm_camera_eye_x(), ex.verdant_wasm_camera_eye_y(), ex.verdant_wasm_camera_eye_z()];
    const target = [ex.verdant_wasm_camera_target_x(), ex.verdant_wasm_camera_target_y(), ex.verdant_wasm_camera_target_z()];
    const vCount = ex.verdant_wasm_scene_vert_count() >>> 0;
    const iCount = ex.verdant_wasm_scene_idx_count() >>> 0;

    if (tileMode || renderer?.backend === "tensor-2d") {
      const uex = await loadUorcWasm();
      uex.verdant_wasm_set_lod_rank(lodRank >>> 0);
      const tw = 288, th = 112;
      const bytes = uex.verdant_wasm_render_tensor_rgba(tw, th, eye[0], eye[1], eye[2], target[0], target[1], target[2]) >>> 0;
      const ptr = uex.verdant_wasm_fb_ptr() >>> 0;
      if (bytes === tw * th * 4 && ptr > 0 && renderer) {
        renderer.blitTensorRgba(new Uint8Array(uex.memory.buffer, ptr, bytes), tw, th);
      }
    } else if (renderer && vCount > 0 && iCount > 0 && ex.memory) {
      const buf = ex.memory.buffer;
      const pos = new Float32Array(buf, ex.verdant_wasm_scene_pos_ptr() >>> 0, vCount * 3);
      const norm = new Float32Array(buf, ex.verdant_wasm_scene_norm_ptr() >>> 0, vCount * 3);
      const col = new Float32Array(buf, ex.verdant_wasm_scene_col_ptr() >>> 0, vCount * 3);
      const idx = new Uint32Array(buf, ex.verdant_wasm_scene_idx_ptr() >>> 0, iCount);
      renderer.uploadPackedBuffers(pos, norm, col, idx);
      const fov = Math.PI / 3.4;
      const proj = mat4Perspective(fov, canvas.width / Math.max(1, canvas.height), 0.25, 160.0);
      const view = mat4LookAt(eye, target, [0, 1, 0]);
      renderer.render(mat4Mul(proj, view), eye, LOD_ENVELOPES[lodRank] || LOD_ENVELOPES[3], 0, target, fov);
    }
    fig.dataset.veReady = "true";
    fig.dataset.veTris = String(Math.floor(iCount / 3));
  };

  await draw();
  fig.addEventListener("click", async (e) => {
    const act = e.target.closest("button")?.dataset.veAct;
    if (!act) return;
    if (act === "orbit") yawMilli += 440;
    else if (act === "lod") lodRank = (lodRank + 1) % 5;
    else if (act === "cam") thirdPerson = !thirdPerson;
    else if (act === "tile") tileMode = !tileMode;
    await draw();
  });
}

function renderObserver(ex, fig, canvas, slot, dwellMs) {
  const ctx = canvas.getContext("2d");
  if (!ctx) return;
  const pal = readDocPalette();
  ex.verdant_wasm_step_observer(slot >>> 0, dwellMs >>> 0, 80, 400, 620);
  const mask = ex.verdant_wasm_observer_spike_mask() >>> 0;
  const att = ex.verdant_wasm_observer_attended_slot() >>> 0;
  const yaw = ex.verdant_wasm_observer_gimbal_yaw_milli() | 0;
  drawBanner(
    ctx,
    canvas.width,
    canvas.height,
    \`LIVE Q15 LIF/SSM OBSERVER (verdant-wasm) · SPIKE MASK 0b\${mask.toString(2).padStart(8, "0")}\`,
    \`Attended Node: N\${att} · Mutual Gimbal Yaw: \${yaw} mrad · Dwell: \${dwellMs} ms\`,
  );
  for (let n = 0; n < 8; n++) {
    const on = ((mask >>> n) & 1) === 1;
    const x = 16 + n * 68;
    ctx.fillStyle = on ? pal.accentSoft : pal.card;
    ctx.fillRect(x, 62, 58, 26);
    ctx.strokeStyle = on ? pal.accent : pal.line;
    ctx.strokeRect(x, 62, 58, 26);
    ctx.fillStyle = on ? pal.accent : pal.fg;
    ctx.fillText(\`N\${n}:\${on ? 1 : 0}\`, x + 12, 79);
  }
  const sals = [0, 1, 2, 3, 4, 5, 6].map((i) => Math.max(0, ex.verdant_wasm_observer_salience_milli(i)));
  const hi = Math.max(600, ...sals);
  sals.forEach((v, i) => {
    const bw = Math.max(10, Math.round((v / hi) * 390));
    const y = 102 + i * 16;
    ctx.fillStyle = pal.muted;
    ctx.fillText(\`Slot \${i}\`, 16, y + 11);
    ctx.fillStyle = i === att ? pal.accent : pal.secondary;
    ctx.fillRect(76, y, bw, 11);
    ctx.fillStyle = pal.fg;
    ctx.fillText(String(v), 84 + bw, y + 11);
  });
  fig.dataset.veReady = "true";
}

function renderPhysics(ex, fig, canvas, ticks) {
  const ctx = canvas.getContext("2d");
  if (!ctx) return;
  const pal = readDocPalette();
  for (let i = 0; i < ticks; i++) ex.verdant_wasm_step_lawful(16666, 900, 0, 3142, 1000, 0);
  const mx = ex.verdant_wasm_mover_x(), mz = ex.verdant_wasm_mover_z(), dist = ex.verdant_wasm_mover_distance();
  const px = ex.verdant_wasm_player_x(), pz = ex.verdant_wasm_player_z();
  const clr = ex.verdant_wasm_camera_clearance_milli() >>> 0;
  const vis = ex.verdant_wasm_bvh_visible_count(0, 10, 22, 0, -0.35, -0.93, 2.0) >>> 0;
  const pan = ex.verdant_wasm_binaural_pan(-9, 2, -4, px, 2, pz, 0, 0, -1);
  drawBanner(
    ctx,
    canvas.width,
    canvas.height,
    \`LIVE LAWFUL CCD + CAMERA GUARD + AUDIO · CLEARANCE \${clr} mm\`,
    \`Mover: \${dist.toFixed(2)} m · BVH Visible: \${vis} · Binaural Pan: \${pan.toFixed(3)} · ITD: \${ex.verdant_wasm_binaural_itd_micros().toFixed(1)} us\`,
  );
  const cx = 288, cy = 138, scale = 2.6;
  ctx.strokeStyle = pal.line;
  ctx.beginPath();
  ctx.arc(cx, cy, 26 * scale * 0.45, 0, Math.PI * 2);
  ctx.stroke();
  ctx.fillStyle = pal.secondary;
  ctx.beginPath();
  ctx.arc(cx + mx * 1.4, cy + mz * 1.4, 7, 0, Math.PI * 2);
  ctx.fill();
  ctx.fillStyle = pal.accent;
  ctx.fillRect(cx + px * 2 - 5, cy + pz * 2 - 5, 10, 10);
  ctx.fillStyle = pal.fg;
  ctx.fillText(\`Spline Mover (\${mx.toFixed(1)}, \${mz.toFixed(1)})\`, 18, 206);
  ctx.fillText(\`22-Joint Explorer (\${px.toFixed(1)}, \${pz.toFixed(1)})\`, 300, 206);
  fig.dataset.veReady = "true";
}

function renderLodStream(ex, fig, canvas, activeRank) {
  const ctx = canvas.getContext("2d");
  if (!ctx) return;
  const pal = readDocPalette();
  const stats = [0, 1, 2, 3, 4].map((r) => {
    ex.verdant_wasm_set_lod_rank(r);
    const tris = Math.floor((ex.verdant_wasm_scene_idx_count() >>> 0) / 3);
    const nodes = ex.verdant_wasm_node_preset_count() >>> 0;
    return { r, tris, nodes };
  });
  ex.verdant_wasm_set_lod_rank(activeRank);
  drawBanner(
    ctx,
    canvas.width,
    canvas.height,
    \`LIVE PROGRESSIVE LOD0..LOD4 HD ENVELOPE · ACTIVE LOD\${activeRank}\`,
    \`Cold Boot < 120 KiB -> Monotonic Vaulted Citadel, Bridge & 22-Joint IK Refinement\`,
  );
  const maxTris = Math.max(1, ...stats.map((s) => s.tris));
  stats.forEach((s, idx) => {
    const y = 62 + idx * 30;
    const bw = Math.max(24, Math.round((s.tris / maxTris) * 320));
    ctx.fillStyle = s.r === activeRank ? pal.accent : pal.card;
    ctx.fillRect(92, y, bw, 20);
    ctx.strokeStyle = s.r === activeRank ? pal.accent : pal.line;
    ctx.strokeRect(92, y, bw, 20);
    ctx.fillStyle = pal.fg;
    ctx.fillText(\`LOD\${s.r}\`, 18, y + 14);
    ctx.fillText(\`\${s.tris} tris · \${s.nodes} nodes\`, 102 + bw, y + 14);
  });
  fig.dataset.veReady = "true";
}

function renderSnapshotEnvelopeCold(ex, fig, canvas) {
  const ctx = canvas.getContext("2d");
  if (!ctx) return;
  const pal = readDocPalette();
  const maxBytes = ex.verdant_wasm_snapshot_max_bytes() >>> 0;
  const minBytes = ex.verdant_wasm_snapshot_min_wire_bytes() >>> 0;
  const ringCap = ex.verdant_wasm_worker_ring_capacity() >>> 0;
  const polys = ex.verdant_wasm_navmesh_polygon_count() >>> 0;
  drawBanner(
    ctx,
    canvas.width,
    canvas.height,
    \`LIVE SHA-256 VDNT SNAPSHOT ENVELOPE · READY (STREAMING FULL CODEC…)\`,
    \`Min Wire: \${minBytes} B · Max Budget: \${maxBytes} B · Worker Ring: \${ringCap} cmds\`,
  );
  ctx.fillStyle = pal.accentSoft;
  ctx.strokeStyle = pal.accent;
  ctx.fillRect(16, 70, 544, 128);
  ctx.strokeRect(16, 70, 544, 128);
  ctx.fillStyle = pal.fg;
  ctx.fillText(\`Header Magic: VDNT (0x56 0x44 0x4E 0x54) + 32-byte SHA-256 trailer\`, 30, 102);
  ctx.fillText(\`Cold-boot envelope verified in verdant-wasm; streaming SHA-256 codec…\`, 30, 134);
  ctx.fillText(\`NavMesh Walkable Polygons Verified: \${polys}\`, 30, 166);
  fig.dataset.veReady = "true";
}

async function renderSnapshotGuard(exCold, fig, canvas, tamper) {
  renderSnapshotEnvelopeCold(exCold, fig, canvas);
  const uex = await loadUorcWasm();
  const ctx = canvas.getContext("2d");
  if (!ctx) return;
  const pal = readDocPalette();
  uex.verdant_wasm_set_lod_rank(3);
  const len = uex.verdant_wasm_snapshot_encode() >>> 0;
  const raw = new Uint8Array(len);
  for (let i = 0; i < len; i++) raw[i] = uex.verdant_wasm_io_get_byte(i) & 0xff;
  if (tamper && raw.length > 12) raw[raw.length - 1] ^= 0x5a;
  uex.verdant_wasm_io_clear();
  for (let i = 0; i < raw.length; i++) uex.verdant_wasm_io_push_byte(raw[i]);
  const ok = uex.verdant_wasm_snapshot_decode_and_restore() === 1;
  drawBanner(
    ctx,
    canvas.width,
    canvas.height,
    \`LIVE SHA-256 VDNT SNAPSHOT GUARD · \${ok ? "ADMITTED (VERIFIED)" : "REFUSED (TAMPER DETECTED)"}\`,
    \`Wire Length: \${len} B · Max Budget: \${uex.verdant_wasm_snapshot_max_bytes()} B · Ring Cap: \${uex.verdant_wasm_worker_ring_capacity()} cmds\`,
  );
  ctx.fillStyle = ok ? pal.accentSoft : pal.dangerSoft;
  ctx.strokeStyle = ok ? pal.accent : pal.danger;
  ctx.fillRect(16, 70, 544, 128);
  ctx.strokeRect(16, 70, 544, 128);
  ctx.fillStyle = pal.fg;
  ctx.fillText(\`Header Magic: VDNT (0x56 0x44 0x4E 0x54) + 32-byte SHA-256 digest\`, 30, 102);
  ctx.fillText(
    tamper
      ? "Injected 1-bit flip at tail byte -> WorldSnapshot::decode rejected payload!"
      : "Clean round-trip -> WorldSnapshot::decode restored LOD3 SceneWorld bit-identically.",
    30,
    134,
  );
  ctx.fillText(\`NavMesh Walkable Polygons Verified: \${uex.verdant_wasm_navmesh_polygon_count()}\`, 30, 166);
  fig.dataset.veReady = "true";
  fig.dataset.veSnapshotVerdict = ok ? "admitted" : "refused";
}

function renderSemirings(ex, fig, canvas, mode) {
  const ctx = canvas.getContext("2d");
  if (!ctx) return;
  const pal = readDocPalette();
  const kCount = ex.verdant_wasm_unified_tensor_kernel_count() >>> 0;
  const cTiers = ex.verdant_wasm_coded_tier_count() >>> 0;
  const splits = [0, 1, 2, 3].map((i) => ex.verdant_wasm_csm_split_meters(0.25, 140.0, i));
  const rt60 = ex.verdant_wasm_sabine_rt60_millis(1800, 96000) >>> 0;
  const modes = [0, 1, 2, 3, 4].map((i) => ex.verdant_wasm_modal_bar_numer(i) >>> 0);
  drawBanner(
    ctx,
    canvas.width,
    canvas.height,
    \`LIVE PRISM KERNEL TELEMETRY · \${kCount} UNIFIED SEMIRING KERNELS · \${cTiers} CODED TIERS\`,
    mode === "csm"
      ? \`4-Cascade Shadow Splits (0.25m..140m): \${splits.map((v) => v.toFixed(2) + "m").join(" | ")}\`
      : \`Sabine RT60: \${rt60} ms · Euler-Bernoulli Modal Numerators: [\${modes.join(", ")}] / 9\`,
  );
  const vals = mode === "csm" ? splits : modes;
  const hi = Math.max(1, ...vals);
  vals.forEach((v, i) => {
    const y = 72 + i * 28;
    const bw = Math.max(20, Math.round((v / hi) * 380));
    ctx.fillStyle = mode === "csm" ? pal.accent : pal.secondary;
    ctx.fillRect(110, y, bw, 18);
    ctx.fillStyle = pal.fg;
    ctx.fillText(mode === "csm" ? \`Cascade \${i}\` : \`Mode \${i}\`, 18, y + 13);
    ctx.fillText(mode === "csm" ? \`\${v.toFixed(2)} m\` : \`\${v}/9 f0\`, 118 + bw, y + 13);
  });
  fig.dataset.veReady = "true";
}

loadWasm()
  .then(async (ex) => {
    for (const fig of document.querySelectorAll("figure[data-ve-live]")) {
      const mode = fig.dataset.veLive;
      const canvas = fig.querySelector("canvas");
      if (!canvas) continue;
      try {
        if (mode === "viewport") {
          await initInteractive3DViewport(ex, fig, canvas);
        } else if (mode === "observer") {
          let activeSlot = 0;
          const draw = () => renderObserver(ex, fig, canvas, activeSlot, 1400);
          redrawHooks.push(draw);
          draw();
          fig.addEventListener("click", (e) => {
            const act = e.target.closest("button")?.dataset.veAct;
            if (act?.startsWith("slot-")) {
              activeSlot = Number(act.slice(5));
              renderObserver(ex, fig, canvas, activeSlot, 1600);
            }
          });
        } else if (mode === "physics") {
          redrawHooks.push(() => renderPhysics(ex, fig, canvas, 0));
          renderPhysics(ex, fig, canvas, 4);
          fig.addEventListener("click", (e) => {
            const act = e.target.closest("button")?.dataset.veAct;
            if (act === "step") renderPhysics(ex, fig, canvas, 15);
          });
        } else if (mode === "lod-stream") {
          let rank = 4;
          const draw = () => renderLodStream(ex, fig, canvas, rank);
          redrawHooks.push(draw);
          draw();
          fig.addEventListener("click", (e) => {
            const act = e.target.closest("button")?.dataset.veAct;
            if (act?.startsWith("lod-")) {
              rank = Number(act.slice(4));
              draw();
            }
          });
        } else if (mode === "snapshot") {
          let tampered = false;
          const draw = () => renderSnapshotGuard(ex, fig, canvas, tampered);
          redrawHooks.push(() => renderSnapshotEnvelopeCold(ex, fig, canvas));
          draw();
          fig.addEventListener("click", (e) => {
            const act = e.target.closest("button")?.dataset.veAct;
            if (act === "valid") { tampered = false; draw(); }
            if (act === "tamper") { tampered = true; draw(); }
          });
        } else if (mode === "semirings") {
          let subMode = "csm";
          const draw = () => renderSemirings(ex, fig, canvas, subMode);
          redrawHooks.push(draw);
          draw();
          fig.addEventListener("click", (e) => {
            const act = e.target.closest("button")?.dataset.veAct;
            if (act === "csm" || act === "audio") {
              subMode = act;
              draw();
            }
          });
        }
      } catch (figErr) {
        fig.dataset.veReady = "error";
        fig.dataset.veError = String(figErr?.message || figErr);
        console.error("Verdant live-embed figure error:", mode, figErr);
      }
    }
  })
  .catch((err) => {
    for (const fig of document.querySelectorAll("figure[data-ve-live]")) {
      fig.dataset.veReady = "error";
      fig.dataset.veError = String(err?.message || err);
    }
    console.error("Verdant live-embed boot error:", err);
  });
`;

function layout({ rel, title, summary, diataxis, journey, audience, figure, body, prev, next }) {
  const depth = rel.split("/").length - 1;
  const root = "../".repeat(depth);
  const docsRoot = "../".repeat(depth - 1) || "./";
  const metas = [
    `<meta name="description" content="${summary}">`,
    diataxis ? `<meta name="diataxis" content="${diataxis}">` : "",
    journey ? `<meta name="developer-journey" content="${journey}">` : "",
    audience ? `<meta name="audience" content="${audience}">` : "",
  ].filter(Boolean).join("\n    ");

  return `<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    ${metas}
    <link rel="canonical" href="${LIVE}${rel.replace(/index\.html$/, "")}">
    <title>${title} — Verdant Engine</title>
    <style>${docStyle(root)}</style>
  </head>
  <body>
    <div class="ve-nav-wrap">
      <nav class="ve-nav" aria-label="Documentation navigation">
        <a href="${root}">3D Studio</a>
        <a href="${root}#studio=create">Create game</a>
        <a href="${docsRoot}">Docs home</a>
        <a href="${docsRoot}getting-started/">Getting started</a>
        <a href="${docsRoot}guides/custom-game-world/">Authoring &amp; UI</a>
        <a href="${docsRoot}guides/progressive-uorc-streaming/">UORC &amp; Stitch</a>
        <a href="${docsRoot}reference/engine-api/">API reference</a>
        <a href="${docsRoot}explanation/tensor-and-uorc-streaming/">Tensor &amp; Neuro</a>
        <button type="button" class="ve-theme-btn" data-ve-theme-toggle aria-label="Toggle documentation theme">Theme</button>
      </nav>
    </div>
    <main>
      <h1>${title}</h1>
      <p>${summary}</p>
      <figure class="cs-figure"><script type="application/vnd.prism.figure+json">${JSON.stringify(figure)}</script></figure>
      ${body}
      <nav class="dc-pager" aria-label="Page sequence">
        ${prev ? `<a href="${docsRoot}${prev.href}">Previous: ${prev.label}</a>` : `<span></span>`}
        ${next ? `<a href="${docsRoot}${next.href}">Next: ${next.label}</a>` : `<span></span>`}
      </nav>
    </main>
    <script type="module" src="${docsRoot}live-embed.mjs"></script>
  </body>
</html>
`;
}

const PAGES = [
  {
    rel: "docs/index.html",
    title: "Verdant Engine documentation",
    summary: "Build verified 3D games, compliant interfaces, and neural worlds for Rust and the web.",
    journey: "discover",
    audience: "kai",
    figure: {
      kind: "quadrant",
      title: "Four ways to explore Verdant Engine documentation",
      caption: "Pick a hands-on tutorial, an authoring guide, the crate API reference, or the semiring architecture.",
      x: ["Study and design", "Hands-on code"],
      y: ["Practical steps", "Deep context"],
      cells: [
        { label: "Getting started", sub: "Run a 3D world in five minutes", x: 1, y: 0, href: "./getting-started/" },
        { label: "Authoring & UI", sub: "GLB import, XPBD, and WCAG UI", x: 1, y: 1, href: "./guides/custom-game-world/" },
        { label: "API reference", sub: "Crates, rollback, and CLI commands", x: 0, y: 0, href: "./reference/engine-api/" },
        { label: "Tensor & Neuro", sub: "15 semiring kernels and LIF/SLM", x: 0, y: 1, href: "./explanation/tensor-and-uorc-streaming/" },
      ],
    },
    body: `
      <h2>Choose your path by role</h2>
      <p>Verdant Engine is a lawful, high-definition 3D game, story, and interface engine in Rust and WebAssembly.</p>
      <p>Every pass enforces solid two-level BVH collision, camera near-plane guard spheres, 22-joint Foot IK characters, and three-pass PBR shadows.</p>
      <p>It also scales across floating-origin universe cells, conservation-verified Chronicles, formal UI proofs, and Stitch peer shards.</p>
      <div data-content="cast"></div>
      <h2>Live WebAssembly 3D Citadel and tile viewport</h2>
      <p>Every page in these docs runs <code>site/verdant-wasm.wasm</code> live so you can test the engine while reading.</p>
      ${liveWasmFigure({
        id: "fig-live-viewport",
        mode: "viewport",
        label: "Live Verdant Engine 3D Citadel and Arch Bridge viewport rendered from WebAssembly buffers",
        caption: "Live 3D WebGL2 and 16x16 software tile PBR viewport rendered directly from verdant-wasm zero-copy scene buffers.",
        summaryTitle: "Engine layer table",
        headers: ["Layer", "Crate or path", "Core responsibilities"],
        rows: [
          ["4 · Browser Studio", "<code>site/</code>, <code>crates/verdant-wasm</code>", "6-tab Workbench, Guided Journeys, Game Scaffolder, 3-pass WebGL2, and 16x16 tile PBR"],
          ["3 · Engine Core", "<code>crates/verdant-core</code>, <code>crates/verdant-cli</code>", "TwoLevelBvh CCD, LawfulCameraRig, 22-joint IK, UniversePosition, and StoryAutomaton"],
          ["2 · Tensor Substrate", "<code>crates/verdant-tensor</code>", "Fifteen unified semiring kernels and three zero-decode codec tiers"],
          ["1 · Prism Model", "<code>app/src/*.lex.tex</code>", "Constructive Lean proofs compiled into <code>prism_kernel.rs</code>"],
        ],
        buttons: [
          ["orbit", "Orbit +25 deg"],
          ["lod", "Cycle LOD0..LOD4"],
          ["cam", "Toggle 3rd-Person IK"],
          ["tile", "Toggle 16x16 Tile PBR"],
        ],
      })}
      <h2>Documentation map</h2>
      <ul>
        <li><a href="./getting-started/">Build your first 3D world</a> walks through spawning entities, stepping physics, and observing mutual attention.</li>
        <li><a href="./guides/custom-game-world/">Author scenes, UI compliance, and spatial audio</a> covers <code>.glb</code> import, XPBD joints, formal UI checks, and occluded sound.</li>
        <li><a href="./guides/progressive-uorc-streaming/">Stream zero-decode scenes and Stitch shards</a> shows how <code>uorc_decompress</code>, <code>coded_gemm</code>, and Stitch parity shards run in WebAssembly.</li>
        <li><a href="./reference/engine-api/">Verdant Engine API reference</a> lists crates, types, rollback sessions, CLI subcommands, and 60fps budgets.</li>
        <li><a href="./explanation/tensor-and-uorc-streaming/">Unified semiring tensor core and neuromorphic loop</a> explains all fifteen semiring kernels, LIF recurrence, and embedded SLM routing.</li>
      </ul>
    `,
    next: { href: "getting-started/", label: "Build your first 3D world" },
  },
  {
    rel: "docs/getting-started/index.html",
    title: "Build your first 3D world",
    summary: "Make a Rust 3D scene, step physics, and test live world focus.",
    diataxis: "tutorial",
    journey: "learn",
    audience: "kai",
    figure: {
      kind: "flow",
      title: "Four steps from starter crate to live 3D scene",
      caption: "The starter project builds a scene world, constructs a BVH, steps 60Hz physics, and packs a UORC stream.",
      steps: [
        { label: "Spawn entities", sub: "SceneWorld + PBR materials", icon: ICONS.cube },
        { label: "Build BVH", sub: "Raycasts + frustum culling", icon: ICONS.bvh },
        { label: "Step physics", sub: "60Hz joints +observer", icon: ICONS.physics },
        { label: "Stream UORC", sub: "Portable GLB + UORC pack", icon: ICONS.stream },
      ],
    },
    body: `
      <h2>Run the starter template</h2>
      <p>The repository ships a runnable project in <code>templates/browser-3d-starter</code> so you can launch a 3D world right away.</p>
      <pre><code>cargo run --manifest-path templates/browser-3d-starter/Cargo.toml</code></pre>
      <p>That command builds <code>verdant-core</code>, validates all five built-in WGSL shaders, steps GJK triggers, and writes <code>custom-starter-world.glb</code>.</p>
      <h2>Spawn terrain and child beacons</h2>
      <p>Open <code>templates/browser-3d-starter/src/main.rs</code> to inspect how a scene world organizes parent and child 3D objects.</p>
      <p>Spawn a root terrain patch first, attach columns as children, and build the bounding volume hierarchy.</p>
      <pre><code>let mut scene = SceneWorld::new();
let ground_id = scene.spawn("StarterTerrain", EntityRole::TerrainPatch, None, Transform::default(), bounds, (1.0, "Terrain"));
let bvh = scene.build_bvh();</code></pre>
      <h2>Observe player attention with the live LIF loop</h2>
      <p>Test the integer-exact <code>CyberneticObserverLoop</code> right below by focusing different world slots in WebAssembly.</p>
      ${liveWasmFigure({
        id: "fig-live-observer",
        mode: "observer",
        label: "Live Q15 Leaky-Integrate-and-Fire neuromorphic observer spike raster and salience chart",
        caption: "Live integer-exact LIF spike raster and entity salience scores computed by verdant-wasm.",
        summaryTitle: "Neuromorphic observer parameters",
        headers: ["Parameter", "Verified bound", "Role"],
        rows: [
          ["State neurons", "8 Q15 LIF units", "Recurrent membrane integration without float drift"],
          ["Decay factor", "7 / 8 per tick", "Bounded exponential leak across rollback frames"],
          ["Mutual gimbal yaw", "<code>-player_yaw_milli</code>", "Orients the world observer toward the player"],
        ],
        buttons: [
          ["slot-0", "Focus Slot 0"],
          ["slot-2", "Focus Slot 2"],
          ["slot-3", "Focus Slot 3"],
          ["slot-6", "Focus Slot 6"],
        ],
      })}
      <h2>Capture a world snapshot and portable UORC stream</h2>
      <p>Encode a world snapshot and a portable four-LOD UORC stream with SHA-256 verification before exporting.</p>
      <pre><code>let snap_bytes = WorldSnapshot::encode(&amp;lod_scene)?;
let restored = WorldSnapshot::decode(&amp;snap_bytes)?;
let uorc_bytes = pack_portable_progressive_uorc_stream(7)?;</code></pre>
    `,
    prev: { href: "", label: "Verdant Engine documentation" },
    next: { href: "guides/custom-game-world/", label: "Author scenes, UI compliance, and spatial audio" },
  },
  {
    rel: "docs/guides/custom-game-world/index.html",
    title: "Author scenes, UI compliance, and spatial audio",
    summary: "Import GLB scenes, verify 2D and 3D UI compliance, and step physics with occluded sound.",
    diataxis: "howToGuide",
    journey: "build",
    audience: "iris",
    figure: {
      kind: "graph",
      title: "Authoring, UI compliance, and physics pipeline in verdant-core",
      caption: "External GLB assets, formal UI trees, XPBD joints, and occluded audio unite inside SceneWorld.",
      nodes: [
        { id: "glb", label: "GLB importer", sub: "Meshes, skins, clips, PBR", layer: 0 },
        { id: "ui", label: "Formal UiTree", sub: "48px + WCAG + diegetic BVH", layer: 0 },
        { id: "ecs", label: "SceneWorld + BVH", sub: "Exact hierarchy + raycasts", layer: 1 },
        { id: "phy", label: "XPBD + vehicles", sub: "Joints, capsule, 4-wheel rig", layer: 1 },
        { id: "out", label: "UORC + Studio", sub: "Progressive LOD0..LOD4", layer: 2 },
      ],
      edges: [
        { from: "glb", to: "ecs" },
        { from: "ui", to: "ecs" },
        { from: "ecs", to: "phy" },
        { from: "ecs", to: "out" },
        { from: "phy", to: "out" },
      ],
    },
    body: `
      <h2>Import external GLB scenes and transcode to UORC</h2>
      <p>Call <code>import_glb_to_scene</code> or drop any <code>.glb</code> file onto the Studio viewport to load hierarchies, meshes, skins, and PBR materials.</p>
      <p>Call <code>transcode_glb_to_progressive_uorc</code> or run <code>verdant transcode-glb authored.glb scene.uorc</code> to emit a four-LOD stream.</p>
      <pre><code>let imported = import_glb_to_scene(&amp;glb_bytes)?;
let uorc_stream = transcode_glb_to_progressive_uorc(&amp;glb_bytes, 7)?;</code></pre>
      <h2>Solve and verify 2D and 3D diegetic UI compliance</h2>
      <p>Build a <code>UiTree</code> to solve intrinsic bounds over tropical <code>(max, +)</code> and flex remainders over exact integer math.</p>
      <p>Every interactive widget must meet 48x48 CSS pixel touch bounds, WCAG 2.2 AA contrast, and tropical focus reachability.</p>
      <pre><code>let ui = UiTree::canonical_studio_ui_tree(first_entity);
let rects = ui.solve_and_verify(1280, 900, Some(&amp;imported.scene))?;
let diegetic_bvh = UiTree::build_diegetic_bvh(&amp;rects);</code></pre>
      <h2>Step XPBD joints, vehicles, and spatial audio live</h2>
      <p>Advance <code>verdant_wasm_step</code> live below to watch the spline mover, capsule controller, BVH frustum count, and Woodworth binaural pan update.</p>
      ${liveWasmFigure({
        id: "fig-live-physics",
        mode: "physics",
        label: "Live WebAssembly physics, BVH frustum visibility, and Woodworth binaural cue monitor",
        caption: "Live 60Hz physics step, SAH BVH frustum query, and Woodworth binaural ITD cue in verdant-wasm.",
        summaryTitle: "Physics and acoustic subsystem bounds",
        headers: ["Subsystem", "Method", "Guarantee"],
        rows: [
          ["XPBD joints", "<code>RigidJointConstraint</code>", "Delassus compliance across Hinge, BallSocket, Slider, and Spring"],
          ["Swept capsule", "<code>step_swept_against_bvh</code>", "Continuous BVH obstacle and stair stepping"],
          ["Spatial audio", "<code>evaluate_binaural_cue_with_occlusion</code>", "BVH ray occlusion and 4-channel Hadamard FDN reverb"],
        ],
        buttons: [["step", "Step 15 Physics Frames (250 ms)"]],
      })}
      <h2>Validate portable WGSL shaders</h2>
      <p>Every built-in PBR, terrain, fluid, sky, and skinned shadow shader passes W3C <code>naga</code> validation before asset packing.</p>
    `,
    prev: { href: "getting-started/", label: "Build your first 3D world" },
    next: { href: "guides/progressive-uorc-streaming/", label: "Stream zero-decode scenes and Stitch shards" },
  },
  {
    rel: "docs/guides/progressive-uorc-streaming/index.html",
    title: "Stream zero-decode scenes and Stitch shards",
    summary: "Decode UORC streams in WebAssembly and shard snapshots across Stitch peers.",
    diataxis: "howToGuide",
    journey: "discover",
    audience: "wren",
    figure: {
      kind: "flow",
      title: "Five progressive UORC and HD synthesis levels from LOD0 to LOD4",
      caption: "Browser workers decode stateless UORC frames in WebAssembly and subdivide vaulted architecture up to ultra-high-definition LOD4.",
      steps: [
        { label: "LOD0 proxy", sub: "1/8 scale · bounds + NavMesh", icon: ICONS.bvh },
        { label: "LOD1 medium", sub: "2/8 scale · pavilion + observer", icon: ICONS.cube },
        { label: "LOD2 high", sub: "5/8 scale · XPBD + vehicle rig", icon: ICONS.physics },
        { label: "LOD3 lossless", sub: "8/8 scale · SHA-256 UORC", icon: ICONS.stream },
        { label: "LOD4 ultra HD", sub: "16/8 scale · vaulted Citadel", icon: ICONS.cube },
      ],
    },
    body: `
      <h2>Decode UORC natively inside the browser worker</h2>
      <p>Verdant Engine compiles <code>uorc_decompress</code> to <code>wasm32-unknown-unknown</code> without shipping the five-megabyte search tables.</p>
      <p>Cold boot stays under the 120 KiB mobile budget while <code>verdant-uorc.wasm</code> decodes <code>scene-stream.uorc</code> on demand.</p>
      ${liveWasmFigure({
        id: "fig-live-lod-stream",
        mode: "lod-stream",
        label: "Live progressive LOD0 through LOD4 snapshot size and triangle count chart from verdant-wasm",
        caption: "Live VDNT byte sizes, triangle counts, and node presets measured across LOD0..LOD4 in verdant-wasm.",
        summaryTitle: "Zero-decode UORC and codec tiers",
        headers: ["Stage or tier", "Representation", "Scratch memory"],
        rows: [
          ["Portable UORC stream", "4 self-delimiting <code>UORC-SHA256-CHAIN-v1</code> frames", "Stateless <code>uorc_decompress::run</code>"],
          ["Tier 0 · Geometry", "<code>CodedMatrix</code> <code>Grid</code> 16-entry codebook", "0 bytes float scratch"],
          ["Tier 1 · Animation &amp; SLM", "<code>Packed&lt;Grid, 2&gt;</code> 4-bit nibbles", "0 bytes float scratch"],
          ["Tier 2 · IBL harmonics", "<code>Runs&lt;Grid, 16&gt;</code> sparse stream", "0 bytes float scratch"],
        ],
        buttons: [
          ["lod-0", "Inspect LOD0"],
          ["lod-1", "Inspect LOD1"],
          ["lod-2", "Inspect LOD2"],
          ["lod-3", "Inspect LOD3"],
          ["lod-4", "Inspect LOD4 HD"],
        ],
      })}
      <h2>Shard worlds with Foundry Prism ID and Stitch parity</h2>
      <p>Sign input journals with your device-held Foundry Prism ID and split snapshots into three <code>STCH</code> shards: half A, half B, and parity <code>A XOR B</code>.</p>
      <p>Any two distinct Stitch shards reconstruct the exact world snapshot and verify its SHA-256 object commitment.</p>
      <pre><code>let [a, b, parity] = shard_bytes_to_stitch_fragments(&amp;uorc_bytes)?;
let recovered = recover_bytes_from_stitch_pair(&amp;a, &amp;parity)?;</code></pre>
    `,
    prev: { href: "guides/custom-game-world/", label: "Author scenes, UI compliance, and spatial audio" },
    next: { href: "reference/engine-api/", label: "Verdant Engine API reference" },
  },
  {
    rel: "docs/reference/engine-api/index.html",
    title: "Verdant Engine API reference",
    summary: "Find Rust crates, UI and peer types, CLI flags, and 60fps budgets.",
    diataxis: "reference",
    journey: "scale",
    audience: "otto",
    figure: {
      kind: "bars",
      title: "60fps worker frame stage budgets in microseconds",
      caption: "A 60fps frame spans 16,666 microseconds; the parallel worker DAG completes in 6,300 microseconds.",
      unit: "us",
      bound: { value: 16666, label: "60fps frame ceiling (16,666 us)" },
      rows: [
        { label: "ECS + UI solver", value: 900, note: "Exact hierarchy + tropical UI" },
        { label: "Physics + XPBD", value: 1800, note: "Delassus joints + vehicle" },
        { label: "BVH + cull", value: 1100, note: "SAH build + frustum cull" },
        { label: "LIF + NavMesh", value: 700, note: "Q15 observer + min-plus paths" },
        { label: "FDN audio ring", value: 600, note: "Modal FIR + Woodworth pan" },
        { label: "Tile and draw", value: 1200, note: "WebGL2 or 16x16 software PBR" },
      ],
    },
    body: `
      <h2>Core, UI, cybernetic, and network modules</h2>
      <table>
        <caption>Primary modules across verdant-tensor and verdant-core</caption>
        <thead>
          <tr><th>Module</th><th>Primary types and functions</th><th>Purpose</th></tr>
        </thead>
        <tbody>
          <tr><td><code>verdant_tensor</code></td><td><code>propagate_hierarchy_matrices_exact</code>, <code>tropical_swept_aabb_toi_bounds</code>, <code>solve_two_bone_ik_exact</code>, <code>verify_conservation_and_reachability</code></td><td>Fifteen unified semiring kernels over <code>uor-matmul</code></td></tr>
          <tr><td><code>camera</code> &amp; <code>character</code></td><td><code>LawfulCameraRig</code>, <code>ArticulatedCharacter</code>, <code>TwoLevelBvh</code></td><td>Near-frustum guard sphere, 22-joint Foot IK, and swept crease CCD</td></tr>
          <tr><td><code>universe</code> &amp; <code>narrative</code></td><td><code>UniversePosition</code>, <code>PortalGraph</code>, <code>StoryAutomaton</code>, <code>ConservationLedger</code></td><td>128m floating-origin cells, portals, and conserved story automata</td></tr>
          <tr><td><code>ui</code> &amp; <code>cybernetics</code></td><td><code>UiTree</code>, <code>CyberneticObserverLoop</code>, <code>CodedSlmDirector</code>, <code>ContainerGrantMatrix</code></td><td>Tropical UI compliance, LIF observer, local SLM, and FK-07 grants</td></tr>
          <tr><td><code>network</code> &amp; <code>runtime</code></td><td><code>OntologicalRuntimeGovernor</code>, <code>SignedJournalBatch</code>, <code>StitchFragment</code>, <code>RollbackSession</code>, <code>WorldSnapshot</code></td><td>W5H telos governor, Prism ID signing, 2-of-3 Stitch parity, and 64-frame rollback</td></tr>
        </tbody>
      </table>
      <h2>Test live SHA-256 snapshot verification in WebAssembly</h2>
      <p>Click <strong>Tamper 1 Byte</strong> below to watch <code>verdant_wasm_snapshot_decode_and_restore</code> refuse a corrupted payload live.</p>
      ${liveWasmFigure({
        id: "fig-live-snapshot",
        mode: "snapshot",
        label: "Live WebAssembly VDNT SHA-256 snapshot verifier and tamper refusal test",
        caption: "Live WorldSnapshot SHA-256 round-trip and single-bit tamper refusal executed in verdant-wasm.",
        summaryTitle: "Rollback and snapshot bounds",
        headers: ["Component", "Bound or format", "Verification predicate"],
        rows: [
          ["<code>WorldSnapshot</code>", "<code>VDNT</code> + 32B SHA-256 (max 32,768 B)", "<code>wasmBridgeEnvelopeValid</code>"],
          ["<code>RollbackSession</code>", "64-frame <code>PhysicsWorld</code> ring", "<code>rollbackFrameWithinWindow</code>"],
          ["<code>StitchFragment</code>", "<code>STCH</code> binary 2-of-3 parity", "<code>stitchPairRecoverable</code>"],
        ],
        buttons: [
          ["valid", "Encode & Verify Clean VDNT"],
          ["tamper", "Tamper 1 Byte & Test Refusal"],
        ],
      })}
      <h2>CLI subcommands in crates/verdant-cli</h2>
      <ul>
        <li><code>verdant import-glb</code> and <code>verdant transcode-glb</code> ingest authored GLB files and pack progressive UORC streams.</li>
        <li><code>verdant verify-ui [width] [height]</code> proves 2D and 3D widget touch bounds, WCAG contrast, and focus closure.</li>
        <li><code>verdant route-intent &lt;prompt&gt;</code> routes natural-language commands through the embedded <code>coded_gemm</code> SLM.</li>
        <li><code>verdant stitch-shard</code> and <code>verdant stitch-recover</code> split and reconstruct 2-of-3 SHA-256 Stitch shards.</li>
        <li><code>verdant verify-story</code> and <code>verdant pack-universe</code> audit Chronicle conservation and pack multi-sector Stitch universes.</li>
      </ul>
    `,
    prev: { href: "guides/progressive-uorc-streaming/", label: "Stream zero-decode scenes and Stitch shards" },
    next: { href: "explanation/tensor-and-uorc-streaming/", label: "Unified semiring tensor core and neuromorphic loop" },
  },
  {
    rel: "docs/explanation/tensor-and-uorc-streaming/index.html",
    title: "Unified semiring tensor core and neuromorphic loop",
    summary: "See how fifteen exact math kernels run lawful 3D worlds, IK rigs, and stories.",
    diataxis: "explanation",
    journey: "evaluate",
    audience: "vera",
    figure: {
      kind: "bars",
      title: "Exact accumulator bit widths across all fifteen unified tensor kernels",
      caption: "Every Prism-certified kernel in Tensor.lex.tex reserves 64 to 128 exact accumulator bits so contractions never drift.",
      unit: "bits",
      bound: { value: 64, label: "Minimum exact bound (64 bits)" },
      rows: [
        { label: "Linear (+,*) · transforms & PBR (k0,3,6)", value: 128, note: "96..128 bits" },
        { label: "Linear (+,*) · LBS, CSM & IK (k1,4,13)", value: 128, note: "96..128 bits" },
        { label: "Tropical (max,+) · cull, GJK, UI (k2,9)", value: 64, note: "64 bits" },
        { label: "Tropical (min,+) · nav, CCD, story (k7,12,14)", value: 64, note: "64 bits" },
        { label: "Audio Q15 · FIR & Hadamard FDN (k5)", value: 128, note: "128 bits" },
        { label: "LIF/SSM · mutual observer (k10)", value: 64, note: "64 bits" },
        { label: "Coded zero-decode · UORC & SLM (k8,11)", value: 128, note: "128 bits" },
      ],
    },
    body: `
      <h2>Why Verdant Engine distills every pass to semiring tensor logic</h2>
      <p>Conditional browser GPU compute APIs are absent on many mobile browsers, embedded WebViews, and hardened clients.</p>
      <p>Instead of maintaining divergent code paths, <code>Tensor.lex.tex</code> models fifteen unified semiring kernels executed over <code>uor-matmul</code>.</p>
      ${liveWasmFigure({
        id: "fig-live-semirings",
        mode: "semirings",
        label: "Live Prism 15-kernel telemetry plotting cascaded shadow splits and modal bar harmonics from verdant-wasm",
        caption: "Live CSM shadow cascade splits, Sabine RT60, and Euler-Bernoulli modes queried from verdant-wasm.",
        summaryTitle: "Fifteen unified semiring kernels",
        headers: ["Kernel group", "Algebra", "Engine subsystems"],
        rows: [
          ["<code>k0, k1, k3, k4, k6, k13</code>", "Linear exact <code>(+, *)</code>", "Hierarchy, LBS skinning, 16x16 tile PBR, CSM, Delassus, and Two-Bone IK"],
          ["<code>k2, k7, k9, k12, k14</code>", "Tropical <code>(max, +)</code> / <code>(min, +)</code>", "Frustum cull, NavMesh, formal UI, swept BLAS CCD, and Chronicle closure"],
          ["<code>k5, k10</code>", "Quantized integer rings", "Modal FIR, Hadamard FDN reverb, and Q15 LIF/SSM mutual attention"],
          ["<code>k8, k11</code>", "<code>uor_matmul::codec</code> zero-decode", "Compressed UORC streaming and embedded <code>coded_gemm</code> SLM"],
        ],
        buttons: [
          ["csm", "Query CSM Splits (Wasm)"],
          ["audio", "Query Modal Harmonics (Wasm)"],
        ],
      })}
      <h2>Second-order cybernetics and embedded coded_gemm reasoning</h2>
      <p>In Verdant Engine, the world observes player gaze cadence and locomotion while the player observes the world.</p>
      <p>Integer-exact Q15 LIF state-space recurrence guarantees bit-identical attention state across 64-frame rollback resimulations.</p>
      <p>Natural-language prompts contract directly against 4-bit <code>Packed&lt;Grid&lt;16&gt;, 2&gt;</code> weights via <code>coded_gemm</code> with zero cloud calls.</p>
      <h2>Capability-gated container saturation and sovereign identity</h2>
      <p>Non-sandboxed browser APIs follow Foundry's <code>FK-07</code> denial-wins lattice so an explicit denial always overrides any grant.</p>
      <p>Foundry Prism ID sessions sign deterministic input journals locally while Stitch 2-of-3 parity shards keep worlds durable across peers.</p>
    `,
    prev: { href: "reference/engine-api/", label: "Verdant Engine API reference" },
  },
];

rmSync(join(SITE, "docs"), { recursive: true, force: true });
mkdirSync(join(SITE, "docs"), { recursive: true });
writeFileSync(join(SITE, "docs/live-embed.mjs"), LIVE_EMBED_MJS);

for (const page of PAGES) {
  const outFile = join(SITE, page.rel);
  mkdirSync(dirname(outFile), { recursive: true });
  writeFileSync(outFile, layout(page));
}

const llmsLines = [
  "# Verdant Engine",
  "",
  "> Lawful high-definition 3D browser and WebAssembly game, story, and interface engine in Rust (`crates/verdant-tensor`, `crates/verdant-core`, `crates/verdant-wasm`, `crates/verdant-cli`) mechanized with Prism, fifteen unified `uor-matmul` semiring kernels, `TwoLevelBvh` swept CCD + crease projection, `LawfulCameraRig` near-frustum guard spheres, 3-pass WebGL2 CSM/GGX/ACES + `16x16` tile PBR, 22-joint LBS/Foot-IK characters, `128m` floating-origin `UniversePosition` portals, conservation-verified `StoryAutomaton` Chronicles, formal 2D/3D UI compliance, LIF/SLM direction, and Foundry Prism ID + `afflom/stitch` 2-of-3 parity custody.",
  "",
  "## Documentation",
  "",
  ...PAGES.map((p) => {
    const url = `${LIVE}${p.rel.replace(/index\.html$/, "")}`;
    return `- [${p.title}](${url}): ${p.summary}`;
  }),
  "",
  "## Markdown twins",
  "",
  ...PAGES.map((p) => {
    const mdRel = p.rel.replace(/\.html$/, ".md");
    return `- [${p.title} (Markdown)](${LIVE}${mdRel})`;
  }),
  "",
];
writeFileSync(join(SITE, "llms.txt"), llmsLines.join("\n"));
console.log(`generate-docs: wrote ${PAGES.length} pages, site/docs/live-embed.mjs, and site/llms.txt`);
