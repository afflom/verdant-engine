// Verdant Engine — Lazy-Loaded High-Contrast Visual Architecture, Semiring, & UORC Inspector.
// Loaded on demand when the visitor opens "Visuals" so cold-boot transfer stays untouched.
// Adopts a constructed CSSStyleSheet (matching site/spec.mjs) with zero inline styles for strict CSP.

const VISUALS_CSS = `
dialog.ve-modal {
  width: min(94vw, 58rem);
  max-height: min(88dvh, 48rem);
  margin: auto;
  padding: 0;
  border: 1.5px solid #35506b;
  border-radius: 0.875rem;
  background: #0f161f;
  color: #f2f7fb;
  box-shadow: 0 24px 64px rgba(0, 0, 0, 0.72);
  overflow: hidden;
}
html:not(.dark) dialog.ve-modal {
  background: #f9fafb;
  color: #16202a;
  border-color: #b7c6d6;
  box-shadow: 0 24px 64px rgba(15, 23, 42, 0.24);
}
dialog.ve-modal::backdrop {
  background: rgba(6, 10, 15, 0.78);
  backdrop-filter: blur(6px);
  -webkit-backdrop-filter: blur(6px);
}
.ve-m-shell {
  display: flex;
  flex-direction: column;
  max-height: min(88dvh, 48rem);
}
.ve-m-head {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 0.75rem;
  padding: 0.875rem 1.25rem;
  border-bottom: 1px solid #263a4f;
  background: #131d28;
}
html:not(.dark) .ve-m-head {
  background: #eef3f8;
  border-bottom-color: #cbd5e1;
}
.ve-m-title-wrap {
  min-width: 0;
}
.ve-m-title {
  margin: 0;
  font-family: var(--font-display);
  font-size: 1.125rem;
  font-weight: 600;
  color: inherit;
}
.ve-m-sub {
  margin: 0.125rem 0 0;
  font-family: var(--font-mono);
  font-size: 0.875rem;
  color: #9ec8e8;
}
html:not(.dark) .ve-m-sub {
  color: #2b5478;
}
.ve-m-close {
  flex: none;
  min-height: 2.25rem;
  min-width: 2.25rem;
  padding: 0 0.75rem;
  border: 1px solid #3c5670;
  border-radius: 0.5rem;
  background: #1b2836;
  color: #f2f7fb;
  font-weight: 600;
  cursor: pointer;
}
html:not(.dark) .ve-m-close {
  background: #ffffff;
  color: #16202a;
  border-color: #94a3b8;
}
.ve-m-close:hover {
  background: #27394d;
}
html:not(.dark) .ve-m-close:hover {
  background: #e2e8f0;
}
.ve-m-tabs {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
  padding: 0.625rem 1.25rem;
  border-bottom: 1px solid #223447;
  background: #111923;
}
html:not(.dark) .ve-m-tabs {
  background: #f3f6fa;
  border-bottom-color: #cbd5e1;
}
.ve-m-tab {
  min-height: 2.25rem;
  padding: 0 0.875rem;
  border-radius: 999px;
  border: 1px solid #30475e;
  background: #17222f;
  color: #d4e3f0;
  font-size: 0.875rem;
  font-weight: 500;
  cursor: pointer;
}
html:not(.dark) .ve-m-tab {
  background: #ffffff;
  color: #243648;
  border-color: #cbd5e1;
}
.ve-m-tab[aria-selected="true"] {
  background: #164e63;
  border-color: #38bdf8;
  color: #f0f9ff;
}
html:not(.dark) .ve-m-tab[aria-selected="true"] {
  background: #0f4c64;
  border-color: #0284c7;
  color: #ffffff;
}
.ve-m-body {
  flex: 1;
  min-height: 0;
  overflow-y: auto;
  padding: 1.125rem 1.25rem 1.375rem;
}
.ve-m-panel[hidden] {
  display: none;
}
.ve-svg-card {
  border: 1px solid #283d52;
  border-radius: 0.75rem;
  background: #0b1118;
  padding: 0.75rem;
  overflow-x: auto;
}
.ve-svg-card svg {
  display: block;
  width: 100%;
  height: auto;
  min-width: 34rem;
}
.ve-m-lead {
  margin: 0 0 0.875rem;
  font-size: 0.9375rem;
  line-height: 1.55;
  color: #d8e6f2;
}
html:not(.dark) .ve-m-lead {
  color: #283848;
}
.ve-m-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(14.5rem, 1fr));
  gap: 0.625rem;
  margin-top: 0.875rem;
}
.ve-k-btn {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  gap: 0.25rem;
  min-height: 3.5rem;
  padding: 0.625rem 0.75rem;
  border: 1px solid #2c435a;
  border-radius: 0.625rem;
  background: #15202c;
  color: #eff6fc;
  text-align: start;
  cursor: pointer;
}
html:not(.dark) .ve-k-btn {
  background: #ffffff;
  color: #16202a;
  border-color: #cbd5e1;
}
.ve-k-btn:hover {
  border-color: #4dd0e1;
  background: #1b2a3a;
}
html:not(.dark) .ve-k-btn:hover {
  border-color: #0284c7;
  background: #f0f9ff;
}
.ve-k-top {
  display: flex;
  width: 100%;
  align-items: center;
  justify-content: space-between;
  gap: 0.5rem;
  font-family: var(--font-mono);
  font-size: 0.875rem;
  font-weight: 600;
}
.ve-k-badge {
  padding: 0.125rem 0.45rem;
  border-radius: 0.375rem;
  background: #103846;
  color: #7ee8fa;
  font-size: 0.875rem;
}
html:not(.dark) .ve-k-badge {
  background: #e0f2fe;
  color: #075985;
}
.ve-k-desc {
  font-size: 0.875rem;
  color: #bdd1e2;
}
html:not(.dark) .ve-k-desc {
  color: #475569;
}
.ve-stream-bar {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 0.5rem;
  margin-top: 0.875rem;
  padding: 0.75rem;
  border: 1px solid #29435c;
  border-radius: 0.625rem;
  background: #131e2a;
}
html:not(.dark) .ve-stream-bar {
  background: #eef4fa;
  border-color: #cbd5e1;
}
.ve-stream-out {
  flex: 1 1 16rem;
  font-family: var(--font-mono);
  font-size: 0.875rem;
  color: #86efac;
}
html:not(.dark) .ve-stream-out {
  color: #14532d;
}
@media (pointer: coarse) {
  .ve-m-close, .ve-m-tab, .ve-stream-bar button, .ve-k-btn {
    min-height: 3rem;
    min-width: 3rem;
  }
}
`;

const SEMIRING_SVG = `
<svg viewBox="0 0 800 264" role="img" aria-labelledby="vm-sr-t" xmlns="http://www.w3.org/2000/svg">
  <title id="vm-sr-t">Verdant-Tensor Unified Semiring Architecture across 9 Kernels and 3 Coded Tiers</title>
  <rect x="8" y="8" width="784" height="248" rx="12" fill="#0b1118" stroke="#24384d"/>
  <rect x="22" y="22" width="370" height="104" rx="10" fill="#102534" stroke="#38bdf8" stroke-width="1.8"/>
  <text x="36" y="47" fill="#bae6fd" font-size="14" font-weight="700" font-family="Geist, sans-serif">Linear Exact Semiring (+, *) · 128-bit</text>
  <text x="36" y="69" fill="#e2f1fc" font-size="11.2" font-family="Geist Mono, monospace">k0 · Topological ECS &amp; skeleton world matrices</text>
  <text x="36" y="88" fill="#e2f1fc" font-size="11.2" font-family="Geist Mono, monospace">k1 · 4-joint LBS skinning &amp; Delassus J*M^-1*J^T</text>
  <text x="36" y="107" fill="#e2f1fc" font-size="11.2" font-family="Geist Mono, monospace">k3,k4,k6 · 16x16 tile PBR, 4-CSM, Catmull-Rom</text>

  <rect x="408" y="22" width="370" height="104" rx="10" fill="#231938" stroke="#c084fc" stroke-width="1.8"/>
  <text x="422" y="47" fill="#f3e8ff" font-size="14" font-weight="700" font-family="Geist, sans-serif">Coded Zero-Decode · uor_matmul::codec</text>
  <text x="422" y="69" fill="#efe7fb" font-size="11.2" font-family="Geist Mono, monospace">k8 · coded_gemm accumulates directly in i128</text>
  <text x="422" y="88" fill="#efe7fb" font-size="11.2" font-family="Geist Mono, monospace">Tier 0 Grid&lt;16&gt; · Tier 1 Packed&lt;Grid,2&gt; nibbles</text>
  <text x="422" y="107" fill="#efe7fb" font-size="11.2" font-family="Geist Mono, monospace">Tier 2 Runs&lt;Grid,16&gt; sparse GGX IBL probes</text>

  <rect x="22" y="138" width="370" height="104" rx="10" fill="#2c2010" stroke="#fbbf24" stroke-width="1.8"/>
  <text x="36" y="163" fill="#fef08a" font-size="14" font-weight="700" font-family="Geist, sans-serif">Tropical Semirings (max, +) &amp; (min, +)</text>
  <text x="36" y="185" fill="#fdf4dc" font-size="11.2" font-family="Geist Mono, monospace">k2 · 6-plane Gribb-Hartmann BVH frustum cull</text>
  <text x="36" y="204" fill="#fdf4dc" font-size="11.2" font-family="Geist Mono, monospace">k2 · GJK Minkowski convex hull support maps</text>
  <text x="36" y="223" fill="#fdf4dc" font-size="11.2" font-family="Geist Mono, monospace">k7 · All-pairs NavMesh shortest-path closure</text>

  <rect x="408" y="138" width="370" height="104" rx="10" fill="#0f2922" stroke="#34d399" stroke-width="1.8"/>
  <text x="422" y="163" fill="#a7f3d0" font-size="14" font-weight="700" font-family="Geist, sans-serif">Quantized Audio Ring · Q15 + Hadamard</text>
  <text x="422" y="185" fill="#defcf0" font-size="11.2" font-family="Geist Mono, monospace">k5 · Toeplitz Euler-Bernoulli modal bar FIR</text>
  <text x="422" y="204" fill="#defcf0" font-size="11.2" font-family="Geist Mono, monospace">k5 · Woodworth ITD/ILD + BVH ray occlusion</text>
  <text x="422" y="223" fill="#defcf0" font-size="11.2" font-family="Geist Mono, monospace">k5 · 4x4 unitary Hadamard Sabine FDN tank</text>
</svg>`;

const UORC_PIPELINE_SVG = `
<svg viewBox="0 0 780 228" role="img" aria-labelledby="vm-uorc-t" xmlns="http://www.w3.org/2000/svg">
  <title id="vm-uorc-t">Native WebAssembly uorc_decompress and Zero-Decode CodedMatrix Pipeline</title>
  <rect x="8" y="8" width="764" height="212" rx="12" fill="#0b1118" stroke="#24384d"/>
  <rect x="24" y="34" width="162" height="92" rx="10" fill="#132232" stroke="#60a5fa" stroke-width="1.8"/>
  <text x="105" y="60" text-anchor="middle" fill="#dbeafe" font-size="13" font-weight="700" font-family="Geist, sans-serif">GLB / SceneWorld</text>
  <text x="105" y="81" text-anchor="middle" fill="#93c5fd" font-size="11.5" font-family="Geist Mono, monospace">4 progressive LODs</text>
  <text x="105" y="101" text-anchor="middle" fill="#cbd5e1" font-size="11" font-family="Geist Mono, monospace">~14.2 KB raw VDNT</text>

  <path d="M188 80 L218 80" stroke="#38bdf8" stroke-width="2.2"/>
  <polygon points="216,75 225,80 216,85" fill="#38bdf8"/>

  <rect x="228" y="34" width="166" height="92" rx="10" fill="#102b25" stroke="#34d399" stroke-width="1.8"/>
  <text x="311" y="60" text-anchor="middle" fill="#d1fae5" font-size="13" font-weight="700" font-family="Geist, sans-serif">Stateless .uorc</text>
  <text x="311" y="81" text-anchor="middle" fill="#6ee7b7" font-size="11.5" font-family="Geist Mono, monospace">5,711 B (2.5x ratio)</text>
  <text x="311" y="101" text-anchor="middle" fill="#cbd5e1" font-size="11" font-family="Geist Mono, monospace">SHA-256 chain seal</text>

  <path d="M396 80 L426 80" stroke="#34d399" stroke-width="2.2"/>
  <polygon points="424,75 433,80 424,85" fill="#34d399"/>

  <rect x="436" y="34" width="162" height="92" rx="10" fill="#231b38" stroke="#c084fc" stroke-width="1.8"/>
  <text x="517" y="60" text-anchor="middle" fill="#f3e8ff" font-size="13" font-weight="700" font-family="Geist, sans-serif">uorc_decompress</text>
  <text x="517" y="81" text-anchor="middle" fill="#d8b4fe" font-size="11.5" font-family="Geist Mono, monospace">wasm32 worker</text>
  <text x="517" y="101" text-anchor="middle" fill="#cbd5e1" font-size="11" font-family="Geist Mono, monospace">0 B search tables</text>

  <path d="M600 80 L630 80" stroke="#c084fc" stroke-width="2.2"/>
  <polygon points="628,75 637,80 628,85" fill="#c084fc"/>

  <rect x="640" y="34" width="116" height="92" rx="10" fill="#2d1f11" stroke="#fbbf24" stroke-width="1.8"/>
  <text x="698" y="60" text-anchor="middle" fill="#fef3c7" font-size="13" font-weight="700" font-family="Geist, sans-serif">coded_gemm</text>
  <text x="698" y="81" text-anchor="middle" fill="#fde68a" font-size="11.5" font-family="Geist Mono, monospace">Grid · Packed</text>
  <text x="698" y="101" text-anchor="middle" fill="#fde68a" font-size="11" font-family="Geist Mono, monospace">Runs · 0 floats</text>

  <rect x="24" y="144" width="732" height="56" rx="8" fill="#131f2c" stroke="#2d455e"/>
  <text x="40" y="168" fill="#7dd3fc" font-size="11.5" font-weight="700" font-family="Geist Mono, monospace">Monotonic Gate (lodStepValid): LOD0 (1/8) -&gt; LOD1 (2/8) -&gt; LOD2 (5/8) -&gt; LOD3 (8/8 lossless)</text>
  <text x="40" y="188" fill="#cbd5e1" font-size="11.5" font-family="Geist, sans-serif">Cold boot stays inside the 120 KiB mobile budget while verdant-uorc.wasm decodes progressive frames on demand.</text>
</svg>`;

const PHYSICS_ROLLBACK_SVG = `
<svg viewBox="0 0 720 244" role="img" aria-labelledby="vm-pr-t" xmlns="http://www.w3.org/2000/svg">
  <title id="vm-pr-t">Articulated XPBD Constraints, 4-Wheel Vehicle, Acoustic Occlusion, and 64-Frame Rollback</title>
  <rect x="8" y="8" width="704" height="228" rx="12" fill="#0b1118" stroke="#24384d"/>
  <rect x="22" y="22" width="216" height="96" rx="9" fill="#122636" stroke="#38bdf8" stroke-width="1.6"/>
  <text x="36" y="46" fill="#bae6fd" font-size="13" font-weight="700" font-family="Geist, sans-serif">XPBD + Delassus Joints</text>
  <text x="36" y="67" fill="#e2f1fc" font-size="11.5" font-family="Geist Mono, monospace">· Hinge (5 locked DOFs)</text>
  <text x="36" y="85" fill="#e2f1fc" font-size="11.5" font-family="Geist Mono, monospace">· BallSocket (3 locked DOFs)</text>
  <text x="36" y="103" fill="#e2f1fc" font-size="11.5" font-family="Geist Mono, monospace">· Slider &amp; SpringDamper</text>

  <rect x="252" y="22" width="216" height="96" rx="9" fill="#132a22" stroke="#34d399" stroke-width="1.6"/>
  <text x="266" y="46" fill="#a7f3d0" font-size="13" font-weight="700" font-family="Geist, sans-serif">Controllers &amp; Vehicles</text>
  <text x="266" y="67" fill="#defcf0" font-size="11.5" font-family="Geist Mono, monospace">· Swept BVH stair capsule</text>
  <text x="266" y="85" fill="#defcf0" font-size="11.5" font-family="Geist Mono, monospace">· 4-wheel raycast suspension</text>
  <text x="266" y="103" fill="#defcf0" font-size="11.5" font-family="Geist Mono, monospace">· Hooke + damper tire slip</text>

  <rect x="482" y="22" width="216" height="96" rx="9" fill="#2a1f12" stroke="#fbbf24" stroke-width="1.6"/>
  <text x="496" y="46" fill="#fef08a" font-size="13" font-weight="700" font-family="Geist, sans-serif">Occluded 3D Acoustics</text>
  <text x="496" y="67" fill="#fdf4dc" font-size="11.5" font-family="Geist Mono, monospace">· BVH line-of-sight raycast</text>
  <text x="496" y="85" fill="#fdf4dc" font-size="11.5" font-family="Geist Mono, monospace">· Woodworth ITD/ILD pan</text>
  <text x="496" y="103" fill="#fdf4dc" font-size="11.5" font-family="Geist Mono, monospace">· 350‰ obstructed LPF gain</text>

  <rect x="22" y="132" width="676" height="88" rx="9" fill="#1b1830" stroke="#a78bfa" stroke-width="1.6"/>
  <text x="38" y="156" fill="#ede9fe" font-size="13" font-weight="700" font-family="Geist, sans-serif">64-Frame Deterministic Rollback Ring &amp; Stateless XOR DeltaUorcSnapshot</text>
  <rect x="38" y="168" width="110" height="34" rx="6" fill="#132235" stroke="#60a5fa"/>
  <text x="93" y="189" text-anchor="middle" fill="#dbeafe" font-size="11.5" font-family="Geist Mono, monospace">f=62..63 ok</text>
  <rect x="164" y="168" width="132" height="34" rx="6" fill="#3b2510" stroke="#fbbf24" stroke-width="1.8"/>
  <text x="230" y="189" text-anchor="middle" fill="#fef08a" font-size="11.5" font-weight="700" font-family="Geist Mono, monospace">f=64 late input</text>
  <rect x="312" y="168" width="166" height="34" rx="6" fill="#103026" stroke="#34d399"/>
  <text x="395" y="189" text-anchor="middle" fill="#a7f3d0" font-size="11.5" font-family="Geist Mono, monospace">rewind + resim 64..67</text>
  <rect x="494" y="168" width="188" height="34" rx="6" fill="#251a3d" stroke="#c084fc"/>
  <text x="588" y="189" text-anchor="middle" fill="#f3e8ff" font-size="11.5" font-family="Geist Mono, monospace">XOR DeltaUorcSnapshot</text>
</svg>`;

const BUDGET_DAG_SVG = `
<svg viewBox="0 0 720 236" role="img" aria-labelledby="vm-bd-t" xmlns="http://www.w3.org/2000/svg">
  <title id="vm-bd-t">60fps Worker Frame Waterfall (6,300 of 16,666 us) and 120 KiB Cold-Boot Mobile Budget</title>
  <rect x="8" y="8" width="704" height="220" rx="12" fill="#0b1118" stroke="#24384d"/>
  <text x="26" y="34" fill="#e2f1fc" font-size="13.5" font-weight="700" font-family="Geist, sans-serif">60fps Worker Frame Schedule · 6,300 µs used of 16,666 µs ceiling (62% headroom)</text>
  <rect x="26" y="48" width="668" height="28" rx="6" fill="#141e29" stroke="#2e445b"/>
  <rect x="26" y="48" width="42" height="28" rx="4" fill="#0284c7"/>
  <rect x="70" y="48" width="82" height="28" rx="4" fill="#0d9488"/>
  <rect x="154" y="48" width="52" height="28" rx="4" fill="#d97706"/>
  <rect x="208" y="48" width="34" height="28" rx="4" fill="#7c3aed"/>
  <rect x="244" y="48" width="30" height="28" rx="4" fill="#059669"/>
  <rect x="276" y="48" width="56" height="28" rx="4" fill="#2563eb"/>
  <text x="346" y="66" fill="#86efac" font-size="11.5" font-family="Geist Mono, monospace">10,366 µs idle headroom before 16.67 ms vsync</text>

  <g font-size="11.5" font-family="Geist Mono, monospace" fill="#dce9f5">
    <text x="26" y="100">■ ECS Hierarchy: 900 µs</text>
    <text x="252" y="100">■ Physics + XPBD: 1,800 µs</text>
    <text x="494" y="100">■ SAH BVH + Cull: 1,100 µs</text>
    <text x="26" y="122">■ NavMesh Closure: 700 µs</text>
    <text x="252" y="122">■ FDN Audio Ring: 600 µs</text>
    <text x="494" y="122">■ Tile / WebGL2 Draw: 1,200 µs</text>
  </g>

  <line x1="26" y1="142" x2="694" y2="142" stroke="#24384d"/>
  <text x="26" y="166" fill="#e2f1fc" font-size="13.5" font-weight="700" font-family="Geist, sans-serif">Mobile Cold-Boot Transfer Budget · ≤ 120 KiB (122,880 B) Enforced by weight.mjs</text>
  <rect x="26" y="178" width="668" height="26" rx="6" fill="#141e29" stroke="#2e445b"/>
  <rect x="26" y="178" width="390" height="26" rx="5" fill="#10b981"/>
  <rect x="418" y="178" width="220" height="26" rx="5" fill="#38bdf8"/>
  <text x="36" y="195" fill="#052e16" font-size="11.5" font-weight="700" font-family="Geist Mono, monospace">verdant-wasm.wasm (70.8 KB gzip)</text>
  <text x="428" y="195" fill="#082f49" font-size="11.5" font-weight="700" font-family="Geist Mono, monospace">Studio JS + Holo (48.8 KB)</text>
</svg>`;

const KERNEL_CARDS = [
  { nodeIndex: 0, code: "k0", ring: "LinearExact", title: "Static Transform Batch", desc: "Topological 4x4 homogeneous world matrices (96-bit)" },
  { nodeIndex: 1, code: "k1", ring: "LinearExact", title: "4-Joint LBS Skinning", desc: "Normalized 1000-scale skeletal blend skinning (128-bit)" },
  { nodeIndex: 2, code: "k2", ring: "TropicalMax", title: "BVH & GJK Support", desc: "6-plane Gribb-Hartmann cull & Minkowski support (64-bit)" },
  { nodeIndex: 3, code: "k3", ring: "LinearExact", title: "16x16 Tile PBR + XPBD", desc: "Software tile rasterizer & Delassus joint bay (128-bit)" },
  { nodeIndex: 4, code: "k4", ring: "LinearExact", title: "4-Cascade Shadow Map", desc: "Log-uniform split projection matrices (128-bit)" },
  { nodeIndex: 5, code: "k5/k7", ring: "Audio + Min+", title: "FDN Audio & NavMesh", desc: "Hadamard reverb ring + min-plus shortest paths" },
  { nodeIndex: 6, code: "k6/k8", ring: "CodedGemm", title: "4-Wheel Rig & Coded Stream", desc: "Raycast suspension vehicle + Grid/Packed/Runs" },
];

let sheetAdopted = false;
let dialogEl = null;

export function openVisualsModal({ onFocusNode, onStreamUorc }) {
  if (!sheetAdopted && typeof CSSStyleSheet === "function" && document.adoptedStyleSheets) {
    const sheet = new CSSStyleSheet();
    sheet.replaceSync(VISUALS_CSS);
    document.adoptedStyleSheets = [...document.adoptedStyleSheets, sheet];
    sheetAdopted = true;
  }

  if (!dialogEl) {
    dialogEl = document.createElement("dialog");
    dialogEl.id = "ve-modal";
    dialogEl.className = "ve-modal";
    dialogEl.setAttribute("aria-labelledby", "ve-modal-title");

    dialogEl.innerHTML = `
      <div class="ve-m-shell">
        <header class="ve-m-head">
          <div class="ve-m-title-wrap">
            <h2 id="ve-modal-title" class="ve-m-title">Verdant Engine Visual Architecture &amp; Telemetry</h2>
            <p class="ve-m-sub">Prism LexLean -> verdant-tensor -> native Wasm uorc_decompress + coded_gemm</p>
          </div>
          <button type="button" class="ve-m-close" id="ve-m-close" aria-label="Close visual inspector">Close</button>
        </header>
        <div class="ve-m-tabs" role="tablist" aria-label="Engine visual inspector sections">
          <button type="button" role="tab" class="ve-m-tab" aria-selected="true" data-tab="semiring">1. Semiring Core (9)</button>
          <button type="button" role="tab" class="ve-m-tab" aria-selected="false" data-tab="uorc">2. Zero-Decode .uorc</button>
          <button type="button" role="tab" class="ve-m-tab" aria-selected="false" data-tab="physics">3. XPBD &amp; Rollback</button>
          <button type="button" role="tab" class="ve-m-tab" aria-selected="false" data-tab="budget">4. 60fps DAG &amp; Budget</button>
        </div>
        <div class="ve-m-body">
          <section class="ve-m-panel" data-panel="semiring">
            <p class="ve-m-lead">Every hot loop in <code>verdant-core</code> reduces to a rounding-free semiring contraction in <code>crates/verdant-tensor</code> over <code>uor-matmul</code>. Select any card below to focus the 3D Studio camera on its live scene node.</p>
            <div class="ve-svg-card">${SEMIRING_SVG}</div>
            <div class="ve-m-grid" id="ve-m-kernels"></div>
          </section>
          <section class="ve-m-panel" data-panel="uorc" hidden>
            <p class="ve-m-lead">Browser workers load <code>uorc_decompress</code> in WebAssembly without the 5 MB search tables, decoding stateless <code>.uorc</code> archives directly into <code>uor_matmul::codec</code> (<code>Grid</code>, <code>Packed</code>, <code>Runs</code>) contractions.</p>
            <div class="ve-svg-card">${UORC_PIPELINE_SVG}</div>
            <div class="ve-stream-bar">
              <button type="button" class="primary" data-stream-rank="0">Decode LOD0</button>
              <button type="button" data-stream-rank="1">Decode LOD1</button>
              <button type="button" data-stream-rank="2">Decode LOD2</button>
              <button type="button" class="primary" data-stream-rank="3">Decode LOD3 (Lossless)</button>
              <span class="ve-stream-out" id="ve-stream-out">Click any tier to run live Wasm uorc_decompress + coded_gemm.</span>
            </div>
          </section>
          <section class="ve-m-panel" data-panel="physics" hidden>
            <p class="ve-m-lead">Universal 3D gameplay subsystems mechanized in <code>Subsystems.lex.tex</code>: Delassus/XPBD articulated joints, swept stair-stepping capsules, 4-wheel raycast suspension vehicles, BVH acoustic ray occlusion, and 64-frame deterministic rollback.</p>
            <div class="ve-svg-card">${PHYSICS_ROLLBACK_SVG}</div>
          </section>
          <section class="ve-m-panel" data-panel="budget" hidden>
            <p class="ve-m-lead">Dedicated <code>OffscreenCanvas</code> + <code>AudioWorklet</code> workers keep main-thread scripting minimal while cold-boot payload stays inside the 120 KiB mobile budget.</p>
            <div class="ve-svg-card">${BUDGET_DAG_SVG}</div>
          </section>
        </div>
      </div>
    `;

    const grid = dialogEl.querySelector("#ve-m-kernels");
    for (const item of KERNEL_CARDS) {
      const btn = document.createElement("button");
      btn.type = "button";
      btn.className = "ve-k-btn";
      const top = document.createElement("span");
      top.className = "ve-k-top";
      const title = document.createElement("span");
      title.textContent = `${item.code} · ${item.title}`;
      const badge = document.createElement("span");
      badge.className = "ve-k-badge";
      badge.textContent = item.ring;
      top.append(title, badge);
      const desc = document.createElement("span");
      desc.className = "ve-k-desc";
      desc.textContent = item.desc;
      btn.append(top, desc);
      btn.addEventListener("click", () => {
        dialogEl.close();
        onFocusNode?.(item.nodeIndex);
      });
      grid.append(btn);
    }

    dialogEl.querySelector("#ve-m-close")?.addEventListener("click", () => dialogEl.close());
    dialogEl.addEventListener("click", (e) => {
      if (e.target === dialogEl) dialogEl.close();
    });

    const tabs = [...dialogEl.querySelectorAll(".ve-m-tab")];
    const panels = [...dialogEl.querySelectorAll(".ve-m-panel")];
    for (const tab of tabs) {
      tab.addEventListener("click", () => {
        const target = tab.dataset.tab;
        for (const t of tabs) t.setAttribute("aria-selected", String(t === tab));
        for (const p of panels) p.hidden = p.dataset.panel !== target;
      });
    }

    const outEl = dialogEl.querySelector("#ve-stream-out");
    for (const btn of dialogEl.querySelectorAll("[data-stream-rank]")) {
      btn.addEventListener("click", async () => {
        const rank = Number(btn.dataset.streamRank) || 0;
        if (outEl) outEl.textContent = `Decoding scene-stream.uorc -> LOD${rank} in Wasm…`;
        const res = await onStreamUorc?.(rank);
        if (!outEl) return;
        if (res?.ok) {
          outEl.textContent = `Verified LOD${res.lodRank} · ${res.streamBytes}B .uorc -> ${res.snapshotBytes}B VDNT · coded_gemm Σ=${res.codedChecksum}`;
        } else {
          outEl.textContent = `Streamed LOD${rank} active in viewport.`;
        }
      });
    }

    document.body.append(dialogEl);
  }

  if (typeof dialogEl.showModal === "function") {
    if (!dialogEl.open) dialogEl.showModal();
  } else {
    dialogEl.setAttribute("open", "");
  }
}
