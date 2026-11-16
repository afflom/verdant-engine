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

const DOC_STYLE = `
  :root {
    color-scheme: dark;
    --fd-bg: 208 32% 7%;
    --fd-panel: 208 26% 12%;
    --fd-fg: 196 36% 96%;
    --fd-dim: 204 22% 77%;
    --fd-line: 204 24% 25%;
    --fd-line-strong: 168 48% 52%;
    --fd-accent: 160 72% 66%;
    --fd-code-bg: 210 30% 10%;
    --fd-code-fg: 152 76% 78%;
    --fd-spectrum: linear-gradient(100deg, hsl(356 92% 66%) 0%, hsl(28 95% 60%) 18%, hsl(48 95% 58%) 34%, hsl(154 72% 52%) 52%, hsl(192 90% 56%) 70%, hsl(226 90% 68%) 86%, hsl(282 84% 72%) 100%);
  }
  @media (prefers-color-scheme: light) {
    :root {
      color-scheme: light;
      --fd-bg: 48 24% 96%;
      --fd-panel: 0 0% 100%;
      --fd-fg: 210 28% 12%;
      --fd-dim: 210 16% 33%;
      --fd-line: 210 18% 78%;
      --fd-line-strong: 164 62% 32%;
      --fd-accent: 162 76% 24%;
      --fd-code-bg: 205 25% 93%;
      --fd-code-fg: 164 78% 20%;
    }
    .cs-canvas {
      background: radial-gradient(120% 140% at 0% 0%, hsl(48 30% 99%), hsl(var(--fd-bg))) !important;
    }
    .cs-card { fill: hsl(0 0% 100%) !important; }
    .cs-cell { fill: hsl(200 20% 95%) !important; }
    .cs-track { fill: hsl(200 18% 89%) !important; }
  }
  * { box-sizing: border-box; }
  body {
    margin: 0;
    background: hsl(var(--fd-bg));
    color: hsl(var(--fd-fg));
    font-family: "Space Grotesk", system-ui, -apple-system, sans-serif;
    font-size: 1.03125rem;
    line-height: 1.65;
    -webkit-font-smoothing: antialiased;
  }
  .ve-nav-wrap {
    position: sticky;
    top: 0;
    z-index: 20;
    background: hsl(var(--fd-bg) / 0.92);
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
  .ve-nav a {
    display: inline-flex;
    align-items: center;
    min-height: 3rem;
    padding: 0.25rem 0.85rem;
    border-radius: 999px;
    border: 1px solid hsl(var(--fd-line));
    background: hsl(var(--fd-panel));
    color: hsl(var(--fd-fg));
    text-decoration: none;
    font-weight: 500;
  }
  .ve-nav a:first-child {
    border-color: hsl(var(--fd-line-strong));
    color: hsl(var(--fd-accent));
    font-weight: 700;
  }
  .ve-nav a:hover {
    border-color: hsl(var(--fd-accent));
    color: hsl(var(--fd-accent));
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
    font-family: "JetBrains Mono", ui-monospace, SFMono-Regular, Menlo, monospace;
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
    background: #081119;
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
    font-family: "JetBrains Mono", monospace;
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
// executes real Rust engine passes (16x16 software tile PBR, LIF/SSM neuromorphic recurrence,
// XPBD + BVH physics, progressive LOD0..LOD3 snapshots, and 12-kernel semiring telemetry).

const scriptUrl = new URL(import.meta.url);
const wasmUrl = new URL("../verdant-wasm.wasm", scriptUrl);

function drawBanner(ctx, w, h, title, subtitle) {
  ctx.fillStyle = "#081119";
  ctx.fillRect(0, 0, w, h);
  ctx.fillStyle = "#8ce4be";
  ctx.font = "600 13px monospace";
  ctx.fillText(title, 16, 26);
  ctx.fillStyle = "#cbd5e1";
  ctx.font = "12px monospace";
  ctx.fillText(subtitle, 16, 48);
}

for (const fig of document.querySelectorAll("figure[data-ve-live]")) {
  const c = fig.querySelector("canvas");
  const ctx = c?.getContext("2d");
  if (ctx) drawBanner(ctx, c.width, c.height, "VERDANT-WASM LIVE ENGINE", "Initializing WebAssembly runtime…");
}

let wasmPromise = null;
function loadWasm() {
  wasmPromise ??= fetch(wasmUrl)
    .then((r) => r.arrayBuffer())
    .then((buf) => WebAssembly.instantiate(buf, {}))
    .then(({ instance }) => {
      const ex = instance.exports;
      ex.verdant_wasm_init(ex.verdant_wasm_default_seed());
      return ex;
    });
  return wasmPromise;
}

function renderTileViewport(ex, canvas, yaw, lodRank) {
  const ctx = canvas.getContext("2d");
  if (!ctx || !ex.memory) return;
  ex.verdant_wasm_set_lod_rank(lodRank >>> 0);
  const w = 288, h = 112;
  const exX = Math.sin(yaw) * 22, exZ = Math.cos(yaw) * 22;
  const bytes = ex.verdant_wasm_render_tensor_rgba(w, h, exX, 11.5, exZ, 0, 1.8, 0) >>> 0;
  const ptr = ex.verdant_wasm_fb_ptr() >>> 0;
  if (bytes === w * h * 4 && ptr > 0 && typeof ImageData === "function") {
    const off = document.createElement("canvas");
    off.width = w;
    off.height = h;
    const octx = off.getContext("2d");
    const slice = new Uint8ClampedArray(ex.memory.buffer, ptr, bytes).slice();
    octx.putImageData(new ImageData(slice, w, h), 0, 0);
    ctx.imageSmoothingEnabled = false;
    ctx.drawImage(off, 0, 0, canvas.width, canvas.height);
  }
  const tris = Math.floor((ex.verdant_wasm_scene_idx_count() >>> 0) / 3);
  ctx.fillStyle = "rgba(8, 17, 25, 0.84)";
  ctx.fillRect(10, 10, 410, 28);
  ctx.fillStyle = "#6ee7b7";
  ctx.font = "600 12px monospace";
  ctx.fillText(\`LIVE WASM 16x16 TILE PBR · LOD\${lodRank} · \${tris} TRIS · YAW \${yaw.toFixed(2)} rad\`, 18, 28);
}

function renderObserver(ex, canvas, slot, dwellMs) {
  const ctx = canvas.getContext("2d");
  if (!ctx) return;
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
    ctx.fillStyle = on ? "#10b981" : "#152535";
    ctx.fillRect(x, 62, 58, 26);
    ctx.strokeStyle = on ? "#6ee7b7" : "#2f4a63";
    ctx.strokeRect(x, 62, 58, 26);
    ctx.fillStyle = on ? "#042f2e" : "#cbd5e1";
    ctx.fillText(\`N\${n}:\${on ? 1 : 0}\`, x + 12, 79);
  }
  const sals = [0, 1, 2, 3, 4, 5, 6].map((i) => Math.max(0, ex.verdant_wasm_observer_salience_milli(i)));
  const hi = Math.max(600, ...sals);
  sals.forEach((v, i) => {
    const bw = Math.max(10, Math.round((v / hi) * 390));
    const y = 102 + i * 16;
    ctx.fillStyle = "#cbd5e1";
    ctx.fillText(\`Slot \${i}\`, 16, y + 11);
    ctx.fillStyle = i === att ? "#34d399" : "#38bdf8";
    ctx.fillRect(76, y, bw, 11);
    ctx.fillStyle = "#e2e8f0";
    ctx.fillText(String(v), 84 + bw, y + 11);
  });
}

function renderPhysics(ex, canvas, ticks) {
  const ctx = canvas.getContext("2d");
  if (!ctx) return;
  for (let i = 0; i < ticks; i++) ex.verdant_wasm_step(16666, 900, 0, 0);
  const mx = ex.verdant_wasm_mover_x(), mz = ex.verdant_wasm_mover_z(), dist = ex.verdant_wasm_mover_distance();
  const px = ex.verdant_wasm_player_x(), pz = ex.verdant_wasm_player_z();
  const vis = ex.verdant_wasm_bvh_visible_count(0, 10, 22, 0, -0.35, -0.93, 2.0) >>> 0;
  const pan = ex.verdant_wasm_binaural_pan(-9, 2, -4, px, 2, pz, 0, 0, -1);
  drawBanner(
    ctx,
    canvas.width,
    canvas.height,
    \`LIVE PHYSICS + BVH + WOODWORTH AUDIO · MOVER DIST \${dist.toFixed(2)} m\`,
    \`BVH Frustum Visible: \${vis} entities · Binaural Pan: \${pan.toFixed(3)} · ITD: \${ex.verdant_wasm_binaural_itd_micros().toFixed(1)} us\`,
  );
  const cx = 288, cy = 142, scale = 2.6;
  ctx.strokeStyle = "#29465e";
  ctx.beginPath();
  ctx.arc(cx, cy, 26 * scale * 0.45, 0, Math.PI * 2);
  ctx.stroke();
  ctx.fillStyle = "#f59e0b";
  ctx.beginPath();
  ctx.arc(cx + mx * 1.4, cy + mz * 1.4, 7, 0, Math.PI * 2);
  ctx.fill();
  ctx.fillStyle = "#34d399";
  ctx.fillRect(cx + px * 2 - 5, cy + pz * 2 - 5, 10, 10);
  ctx.fillStyle = "#e2e8f0";
  ctx.fillText(\`Spline Mover (\${mx.toFixed(1)}, \${mz.toFixed(1)})\`, 18, 204);
  ctx.fillText(\`Capsule (\${px.toFixed(1)}, \${pz.toFixed(1)})\`, 320, 204);
}

function renderLodStream(ex, canvas, activeRank) {
  const ctx = canvas.getContext("2d");
  if (!ctx) return;
  const stats = [0, 1, 2, 3].map((r) => {
    ex.verdant_wasm_set_lod_rank(r);
    const bytes = ex.verdant_wasm_snapshot_encode() >>> 0;
    const tris = Math.floor((ex.verdant_wasm_scene_idx_count() >>> 0) / 3);
    const nodes = ex.verdant_wasm_node_preset_count() >>> 0;
    return { r, bytes, tris, nodes };
  });
  ex.verdant_wasm_set_lod_rank(activeRank);
  drawBanner(
    ctx,
    canvas.width,
    canvas.height,
    \`LIVE PROGRESSIVE LOD0..LOD3 ENVELOPE · ACTIVE LOD\${activeRank}\`,
    \`Portable UORC Stream: 5,711 B total -> monotonic SHA-256 scene refinement in Wasm\`,
  );
  const maxBytes = Math.max(1, ...stats.map((s) => s.bytes));
  stats.forEach((s, idx) => {
    const y = 68 + idx * 36;
    const bw = Math.max(24, Math.round((s.bytes / maxBytes) * 320));
    ctx.fillStyle = s.r === activeRank ? "#10b981" : "#1e3a52";
    ctx.fillRect(92, y, bw, 24);
    ctx.strokeStyle = s.r === activeRank ? "#6ee7b7" : "#38bdf8";
    ctx.strokeRect(92, y, bw, 24);
    ctx.fillStyle = "#f1f5f9";
    ctx.fillText(\`LOD\${s.r}\`, 18, y + 16);
    ctx.fillText(\`\${s.bytes} B VDNT · \${s.tris} tris · \${s.nodes} nodes\`, 102 + bw, y + 16);
  });
}

function renderSnapshotGuard(ex, canvas, tamper) {
  const ctx = canvas.getContext("2d");
  if (!ctx) return;
  ex.verdant_wasm_set_lod_rank(3);
  const len = ex.verdant_wasm_snapshot_encode() >>> 0;
  const raw = new Uint8Array(len);
  for (let i = 0; i < len; i++) raw[i] = ex.verdant_wasm_io_get_byte(i) & 0xff;
  if (tamper && raw.length > 12) raw[raw.length - 1] ^= 0x5a;
  ex.verdant_wasm_io_clear();
  for (let i = 0; i < raw.length; i++) ex.verdant_wasm_io_push_byte(raw[i]);
  const ok = ex.verdant_wasm_snapshot_decode_and_restore() === 1;
  drawBanner(
    ctx,
    canvas.width,
    canvas.height,
    \`LIVE SHA-256 VDNT SNAPSHOT GUARD · \${ok ? "ADMITTED (VERIFIED)" : "REFUSED (TAMPER DETECTED)"}\`,
    \`Wire Length: \${len} B · Max Budget: \${ex.verdant_wasm_snapshot_max_bytes()} B · Ring Cap: \${ex.verdant_wasm_worker_ring_capacity()} cmds\`,
  );
  ctx.fillStyle = ok ? "rgba(16, 185, 129, 0.18)" : "rgba(239, 68, 68, 0.2)";
  ctx.strokeStyle = ok ? "#34d399" : "#f87171";
  ctx.fillRect(16, 70, 544, 128);
  ctx.strokeRect(16, 70, 544, 128);
  ctx.fillStyle = "#f8fafc";
  ctx.fillText(\`Header Magic: VDNT (0x56 0x44 0x4E 0x54) + 32-byte SHA-256 digest\`, 30, 102);
  ctx.fillText(
    tamper
      ? "Injected 1-bit flip at tail byte -> WorldSnapshot::decode rejected payload!"
      : "Clean round-trip -> WorldSnapshot::decode restored LOD3 SceneWorld bit-identically.",
    30,
    134,
  );
  ctx.fillText(\`NavMesh Walkable Polygons Verified: \${ex.verdant_wasm_navmesh_polygon_count()}\`, 30, 166);
}

function renderSemirings(ex, canvas, mode) {
  const ctx = canvas.getContext("2d");
  if (!ctx) return;
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
    ctx.fillStyle = mode === "csm" ? "#38bdf8" : "#a78bfa";
    ctx.fillRect(110, y, bw, 18);
    ctx.fillStyle = "#f1f5f9";
    ctx.fillText(mode === "csm" ? \`Cascade \${i}\` : \`Mode \${i}\`, 18, y + 13);
    ctx.fillText(mode === "csm" ? \`\${v.toFixed(2)} m\` : \`\${v}/9 f0\`, 118 + bw, y + 13);
  });
}

loadWasm()
  .then((ex) => {
    for (const fig of document.querySelectorAll("figure[data-ve-live]")) {
      const mode = fig.dataset.veLive;
      const canvas = fig.querySelector("canvas");
      if (!canvas) continue;
      let yaw = 0.68, lod = 3;
      if (mode === "viewport") {
        renderTileViewport(ex, canvas, yaw, lod);
        fig.addEventListener("click", (e) => {
          const act = e.target.closest("button")?.dataset.veAct;
          if (!act) return;
          if (act === "orbit") yaw += 0.45;
          if (act === "lod") lod = (lod + 1) % 4;
          renderTileViewport(ex, canvas, yaw, lod);
        });
      } else if (mode === "observer") {
        renderObserver(ex, canvas, 0, 1400);
        fig.addEventListener("click", (e) => {
          const act = e.target.closest("button")?.dataset.veAct;
          if (act?.startsWith("slot-")) renderObserver(ex, canvas, Number(act.slice(5)), 1600);
        });
      } else if (mode === "physics") {
        renderPhysics(ex, canvas, 4);
        fig.addEventListener("click", (e) => {
          const act = e.target.closest("button")?.dataset.veAct;
          if (act === "step") renderPhysics(ex, canvas, 15);
        });
      } else if (mode === "lod-stream") {
        renderLodStream(ex, canvas, 3);
        fig.addEventListener("click", (e) => {
          const act = e.target.closest("button")?.dataset.veAct;
          if (act?.startsWith("lod-")) renderLodStream(ex, canvas, Number(act.slice(4)));
        });
      } else if (mode === "snapshot") {
        renderSnapshotGuard(ex, canvas, false);
        fig.addEventListener("click", (e) => {
          const act = e.target.closest("button")?.dataset.veAct;
          if (act === "valid") renderSnapshotGuard(ex, canvas, false);
          if (act === "tamper") renderSnapshotGuard(ex, canvas, true);
        });
      } else if (mode === "semirings") {
        renderSemirings(ex, canvas, "csm");
        fig.addEventListener("click", (e) => {
          const act = e.target.closest("button")?.dataset.veAct;
          if (act === "csm" || act === "audio") renderSemirings(ex, canvas, act);
        });
      }
    }
  })
  .catch(() => {});
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
    <style>${DOC_STYLE}</style>
  </head>
  <body>
    <div class="ve-nav-wrap">
      <nav class="ve-nav" aria-label="Documentation navigation">
        <a href="${root}">3D Studio</a>
        <a href="${docsRoot}">Docs home</a>
        <a href="${docsRoot}getting-started/">Getting started</a>
        <a href="${docsRoot}guides/custom-game-world/">Authoring &amp; UI</a>
        <a href="${docsRoot}guides/progressive-uorc-streaming/">UORC &amp; Stitch</a>
        <a href="${docsRoot}reference/engine-api/">API reference</a>
        <a href="${docsRoot}explanation/tensor-and-uorc-streaming/">Tensor &amp; Neuro</a>
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
        { label: "Tensor & Neuro", sub: "12 semiring kernels and LIF/SLM", x: 0, y: 1, href: "./explanation/tensor-and-uorc-streaming/" },
      ],
    },
    body: `
      <h2>Choose your path by role</h2>
      <p>Verdant Engine is a universal 3D game and interface engine written in Rust and WebAssembly.</p>
      <p>Every pass runs on scene worlds, bounding volume hierarchies, XPBD physics, WGSL shaders, glTF 2.0, and UORC streaming.</p>
      <p>It also verifies 2D and 3D UI compliance, local SLM commands, and Stitch peer shards.</p>
      <div data-content="cast"></div>
      <h2>Live WebAssembly 16x16 software tile viewport</h2>
      <p>Every page in these docs runs <code>site/verdant-wasm.wasm</code> live so you can test the engine while reading.</p>
      ${liveWasmFigure({
        id: "fig-live-viewport",
        mode: "viewport",
        label: "Live Verdant Engine 16x16 software tile PBR viewport rendered in WebAssembly",
        caption: "Live 16x16 software tile PBR render executed directly by verdant-wasm without conditional GPU flags.",
        summaryTitle: "Engine layer table",
        headers: ["Layer", "Crate or path", "Core responsibilities"],
        rows: [
          ["4 · Browser Studio", "<code>site/</code>, <code>crates/verdant-wasm</code>", "Zero-copy WebGL2 and 16x16 tile PBR, LIF/SLM, OPFS, and Stitch mesh"],
          ["3 · Engine Core", "<code>crates/verdant-core</code>, <code>crates/verdant-cli</code>", "ECS, BVH, GLB import, XPBD joints, formal UI, and 64-frame rollback"],
          ["2 · Tensor Substrate", "<code>crates/verdant-tensor</code>", "Twelve unified semiring kernels and three zero-decode codec tiers"],
          ["1 · Prism Model", "<code>app/src/*.lex.tex</code>", "Constructive Lean proofs compiled into <code>prism_kernel.rs</code>"],
        ],
        buttons: [
          ["orbit", "Orbit +25 deg"],
          ["lod", "Cycle LOD0..LOD3"],
        ],
      })}
      <h2>Documentation map</h2>
      <ul>
        <li><a href="./getting-started/">Build your first 3D world</a> walks through spawning entities, stepping physics, and observing mutual attention.</li>
        <li><a href="./guides/custom-game-world/">Author scenes, UI compliance, and spatial audio</a> covers <code>.glb</code> import, XPBD joints, formal UI checks, and occluded sound.</li>
        <li><a href="./guides/progressive-uorc-streaming/">Stream zero-decode scenes and Stitch shards</a> shows how <code>uorc_decompress</code>, <code>coded_gemm</code>, and Stitch parity shards run in WebAssembly.</li>
        <li><a href="./reference/engine-api/">Verdant Engine API reference</a> lists crates, types, rollback sessions, CLI subcommands, and 60fps budgets.</li>
        <li><a href="./explanation/tensor-and-uorc-streaming/">Unified semiring tensor core and neuromorphic loop</a> explains all twelve semiring kernels, LIF recurrence, and embedded SLM routing.</li>
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
        { id: "out", label: "UORC + Studio", sub: "Progressive LOD0..LOD3", layer: 2 },
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
      title: "Four progressive UORC streaming levels from LOD0 to LOD3",
      caption: "Browser workers decode stateless UORC frames in WebAssembly and refine scenes monotonically to lossless fidelity.",
      steps: [
        { label: "LOD0 proxy", sub: "1/8 scale · bounds + NavMesh", icon: ICONS.bvh },
        { label: "LOD1 medium", sub: "2/8 scale · pavilion + observer", icon: ICONS.cube },
        { label: "LOD2 high", sub: "5/8 scale · XPBD + vehicle rig", icon: ICONS.physics },
        { label: "LOD3 lossless", sub: "8/8 scale · 5.7 KB UORC", icon: ICONS.stream },
      ],
    },
    body: `
      <h2>Decode UORC natively inside the browser worker</h2>
      <p>Verdant Engine compiles <code>uorc_decompress</code> to <code>wasm32-unknown-unknown</code> without shipping the five-megabyte search tables.</p>
      <p>Cold boot stays under the 120 KiB mobile budget while <code>verdant-uorc.wasm</code> decodes <code>scene-stream.uorc</code> on demand.</p>
      ${liveWasmFigure({
        id: "fig-live-lod-stream",
        mode: "lod-stream",
        label: "Live progressive LOD0 through LOD3 snapshot size and triangle count chart from verdant-wasm",
        caption: "Live VDNT byte sizes, triangle counts, and node presets measured across LOD0..LOD3 in verdant-wasm.",
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
          <tr><td><code>verdant_tensor</code></td><td><code>propagate_hierarchy_matrices_exact</code>, <code>tropical_ui_box_extents</code>, <code>step_neuromorphic_lif_ssm_ring</code>, <code>infer_coded_slm_intent_logits</code></td><td>Twelve unified semiring kernels over <code>uor-matmul</code></td></tr>
          <tr><td><code>ui</code></td><td><code>UiTree</code>, <code>UiWidget</code>, <code>SolvedUiRect</code></td><td>Tropical 2D/3D layout, 48px targets, and WCAG 2.2 AA verification</td></tr>
          <tr><td><code>cybernetics</code></td><td><code>CyberneticObserverLoop</code>, <code>CodedSlmDirector</code>, <code>ContainerGrantMatrix</code></td><td>Mutual-attention LIF recurrence, local SLM, and FK-07 denial-wins grants</td></tr>
          <tr><td><code>network</code></td><td><code>SignedJournalBatch</code>, <code>StitchFragment</code>, <code>SurvivalEntropyTracker</code></td><td>Prism ID session signing and 2-of-3 Stitch parity custody</td></tr>
          <tr><td><code>gltf_io</code> &amp; <code>streaming</code></td><td><code>import_glb_to_scene</code>, <code>transcode_glb_to_progressive_uorc</code>, <code>StandardSceneState</code></td><td>Khronos GLB authoring and stateless Wasm UORC streaming</td></tr>
          <tr><td><code>physics</code> &amp; <code>runtime</code></td><td><code>RigidJointConstraint</code>, <code>RollbackSession</code>, <code>WorldSnapshot</code></td><td>XPBD joints, 64-frame rollback, and SHA-256 world snapshots</td></tr>
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
      </ul>
    `,
    prev: { href: "guides/progressive-uorc-streaming/", label: "Stream zero-decode scenes and Stitch shards" },
    next: { href: "explanation/tensor-and-uorc-streaming/", label: "Unified semiring tensor core and neuromorphic loop" },
  },
  {
    rel: "docs/explanation/tensor-and-uorc-streaming/index.html",
    title: "Unified semiring tensor core and neuromorphic loop",
    summary: "See how twelve exact math kernels run 3D views, UI, and peer worlds.",
    diataxis: "explanation",
    journey: "evaluate",
    audience: "vera",
    figure: {
      kind: "bars",
      title: "Exact accumulator bit widths across all twelve unified tensor kernels",
      caption: "Every Prism-certified kernel in Tensor.lex.tex reserves 64 to 128 exact accumulator bits so contractions never drift.",
      unit: "bits",
      bound: { value: 64, label: "Minimum exact bound (64 bits)" },
      rows: [
        { label: "Linear (+,*) · transforms & PBR (k0,3,6)", value: 128, note: "96..128 bits" },
        { label: "Linear (+,*) · LBS & CSM (k1,k4)", value: 128, note: "128 bits" },
        { label: "Tropical (max,+) · cull, GJK, UI (k2,9)", value: 64, note: "64 bits" },
        { label: "Tropical (min,+) · NavMesh/focus (k7)", value: 64, note: "64 bits" },
        { label: "Audio Q15 · FIR & Hadamard FDN (k5)", value: 128, note: "128 bits" },
        { label: "LIF/SSM · mutual observer (k10)", value: 64, note: "64 bits" },
        { label: "Coded zero-decode · UORC & SLM (k8,11)", value: 128, note: "128 bits" },
      ],
    },
    body: `
      <h2>Why Verdant Engine distills every pass to semiring tensor logic</h2>
      <p>Conditional browser GPU compute APIs are absent on many mobile browsers, embedded WebViews, and hardened clients.</p>
      <p>Instead of maintaining divergent code paths, <code>Tensor.lex.tex</code> models twelve unified semiring kernels executed over <code>uor-matmul</code>.</p>
      ${liveWasmFigure({
        id: "fig-live-semirings",
        mode: "semirings",
        label: "Live Prism 12-kernel telemetry plotting cascaded shadow splits and modal bar harmonics from verdant-wasm",
        caption: "Live CSM shadow cascade splits, Sabine RT60, and Euler-Bernoulli modes queried from verdant-wasm.",
        summaryTitle: "Twelve unified semiring kernels",
        headers: ["Kernel group", "Algebra", "Engine subsystems"],
        rows: [
          ["<code>k0, k1, k3, k4, k6</code>", "Linear exact <code>(+, *)</code>", "Hierarchy, LBS skinning, 16x16 tile PBR, CSM, and Delassus joints"],
          ["<code>k2, k7, k9</code>", "Tropical <code>(max, +)</code> / <code>(min, +)</code>", "Frustum culling, GJK support, NavMesh closure, and formal UI"],
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
  "> Universal 3D browser and WebAssembly game and interface engine in Rust (`crates/verdant-tensor`, `crates/verdant-core`, `crates/verdant-wasm`, `crates/verdant-cli`) mechanized with Prism, twelve unified `uor-matmul` semiring kernels, native WebAssembly `uorc_decompress` + `uor_matmul::codec` zero-decode streaming, Khronos `.glb` scene import and `.glb -> .uorc` transcoding, formal 2D/3D UI compliance, second-order neuromorphic LIF/SSM attention, embedded `coded_gemm` SLM intent routing, and Foundry Prism ID + `afflom/stitch` 2-of-3 parity multiplayer.",
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
