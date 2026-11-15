#!/usr/bin/env node
/**
 * Generates the clean source Diataxis HTML pages under site/docs/ and the base
 * site/llms.txt index before scripts/content/enrich.mjs enriches them with
 * SVG diagrams, persona guide cards, glossary term links, JSON-LD, and .md twins.
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
    background: hsl(var(--fd-bg) / 0.9);
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
    min-height: 2.25rem;
    padding: 0.25rem 0.7rem;
    border-radius: 999px;
    border: 1px solid hsl(var(--fd-line));
    background: hsl(var(--fd-panel));
    color: hsl(var(--fd-fg));
    text-decoration: none;
    font-weight: 500;
    transition: border-color 0.15s, background 0.15s;
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
  main a:hover {
    color: hsl(var(--fd-fg));
  }
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
  pre code {
    padding: 0;
    border: 0;
    background: transparent;
    color: inherit;
  }
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
  caption {
    text-align: left;
    font-weight: 700;
    padding: 0.5rem 0.25rem;
    color: hsl(var(--fd-dim));
  }
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
  .ve-diagram {
    margin: 1.75rem 0 2.25rem;
  }
  .ve-diagram svg {
    display: block;
    width: 100%;
    height: auto;
    max-height: 21rem;
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

const ARCH_STACK_DIAGRAM = `
<figure class="cs-figure ve-diagram" id="fig-arch-stack">
  <div class="cs-canvas">
    <svg viewBox="0 0 800 250" role="img" aria-labelledby="ve-stack-t ve-stack-d" xmlns="http://www.w3.org/2000/svg">
      <title id="ve-stack-t">Verdant Engine four-layer Prism and WebAssembly stack</title>
      <desc id="ve-stack-d">Four horizontal layers from constructive LexLean specs at the bottom through verdant-tensor semiring kernels and verdant-core subsystems up to the browser OffscreenCanvas and uorc_decompress worker.</desc>
      <rect x="16" y="14" width="768" height="46" rx="10" fill="hsl(164 60% 18%)" stroke="hsl(160 72% 58%)" stroke-width="1.6"/>
      <text x="34" y="35" fill="hsl(160 80% 88%)" font-size="13.5" font-weight="700">4 · Browser Studio &amp; Wasm Workers (site/ + crates/verdant-wasm)</text>
      <text x="34" y="51" fill="hsl(196 30% 84%)" font-size="11" font-family="JetBrains Mono, monospace">OffscreenCanvas WebGL2 + 16x16 Tile Blit · uorc_decompress Wasm · 128f AudioWorklet · #vdnt= URLs</text>

      <rect x="16" y="72" width="768" height="48" rx="10" fill="hsl(202 52% 17%)" stroke="hsl(192 86% 60%)" stroke-width="1.6"/>
      <text x="34" y="93" fill="hsl(192 90% 88%)" font-size="13.5" font-weight="700">3 · Universal 3D Engine Subsystems (crates/verdant-core + crates/verdant-cli)</text>
      <text x="34" y="110" fill="hsl(196 30% 84%)" font-size="11" font-family="JetBrains Mono, monospace">ECS + SAH BVH · Khronos .glb -&gt; .uorc · XPBD Joints &amp; 4-Wheel Vehicle · 64f Rollback · Naga WGSL</text>

      <rect x="16" y="132" width="768" height="48" rx="10" fill="hsl(252 40% 19%)" stroke="hsl(264 84% 72%)" stroke-width="1.6"/>
      <text x="34" y="153" fill="hsl(264 90% 90%)" font-size="13.5" font-weight="700">2 · Unified Semiring Tensor Substrate (crates/verdant-tensor + uor-matmul)</text>
      <text x="34" y="170" fill="hsl(196 30% 84%)" font-size="11" font-family="JetBrains Mono, monospace">Linear (+, *) · CodedMatrix (Grid, Packed, Runs) · Tropical (max, +)/(min, +) · Q15 Hadamard FDN</text>

      <rect x="16" y="192" width="768" height="44" rx="10" fill="hsl(32 48% 17%)" stroke="hsl(42 92% 62%)" stroke-width="1.6"/>
      <text x="34" y="212" fill="hsl(44 96% 88%)" font-size="13.5" font-weight="700">1 · Constructive Prism Model (app/src/Tensor.lex.tex + Subsystems.lex.tex)</text>
      <text x="34" y="228" fill="hsl(196 30% 84%)" font-size="11" font-family="JetBrains Mono, monospace">Zero Classical.choice · {0, 1} Parametric Ratchet · Compiled Mechanically to prism_kernel.rs + app.holo</text>
    </svg>
  </div>
  <figcaption><strong>Verdant Engine full-stack architecture.</strong> Constructive Prism theorems compile directly into the semiring tensor substrate, 3D subsystems, and WebAssembly workers.</figcaption>
  <details class="cs-data"><summary>Architecture layer table</summary>
    <table>
      <thead><tr><th>Layer</th><th>Crate or directory</th><th>Core responsibilities</th></tr></thead>
      <tbody>
        <tr><td>4 · Browser Studio</td><td><code>site/</code>, <code>crates/verdant-wasm</code></td><td>OffscreenCanvas rendering, native Wasm <code>uorc_decompress</code>, AudioWorklet, and <code>#vdnt=</code> snapshots</td></tr>
        <tr><td>3 · Engine Core</td><td><code>crates/verdant-core</code>, <code>crates/verdant-cli</code></td><td>ECS, BVH, <code>.glb</code> import and transcode, XPBD joints, vehicles, rollback, and WGSL checks</td></tr>
        <tr><td>2 · Tensor Substrate</td><td><code>crates/verdant-tensor</code></td><td>Nine semiring kernels and three zero-decode <code>CodedMatrix</code> tiers over <code>uor-matmul</code></td></tr>
        <tr><td>1 · Prism Model</td><td><code>app/src/*.lex.tex</code></td><td>Constructive Lean proofs compiled into <code>prism_kernel.rs</code> and <code>app.holo</code></td></tr>
      </tbody>
    </table>
  </details>
</figure>`;

const ZERO_DECODE_SVG = `
<figure class="cs-figure ve-diagram" id="fig-zero-decode">
  <div class="cs-canvas">
    <svg viewBox="0 0 680 218" role="img" aria-labelledby="ve-zd-t ve-zd-d" xmlns="http://www.w3.org/2000/svg">
      <title id="ve-zd-t">Zero-decode UORC and CodedMatrix browser streaming pipeline</title>
      <desc id="ve-zd-d">Shows 53 KB of raw four-LOD scene state compressed to 5.7 KB in scene-stream.uorc, decoded in WebAssembly by uorc_decompress, and contracted directly via Grid, Packed, and Runs coded_gemm.</desc>
      <rect x="14" y="24" width="144" height="170" rx="12" fill="hsl(208 30% 13%)" stroke="hsl(42 92% 62%)" stroke-width="1.6"/>
      <text x="86" y="52" text-anchor="middle" fill="hsl(44 96% 86%)" font-size="13" font-weight="700">Authoring &amp; GLB</text>
      <text x="86" y="76" text-anchor="middle" fill="hsl(196 32% 92%)" font-size="20" font-weight="700">53.2 KB</text>
      <text x="86" y="96" text-anchor="middle" fill="hsl(204 22% 76%)" font-size="11" font-family="JetBrains Mono, monospace">4 LODs raw VDNT</text>
      <rect x="30" y="114" width="112" height="60" rx="8" fill="hsl(42 90% 55% / 0.14)" stroke="hsl(42 90% 60% / 0.5)"/>
      <text x="86" y="138" text-anchor="middle" fill="hsl(44 95% 84%)" font-size="11" font-family="JetBrains Mono, monospace">transcode-glb</text>
      <text x="86" y="156" text-anchor="middle" fill="hsl(204 22% 78%)" font-size="10.5" font-family="JetBrains Mono, monospace">stateless V04_PAIR</text>

      <line x1="162" y1="109" x2="196" y2="109" stroke="hsl(168 65% 58%)" stroke-width="2.4"/>
      <polygon points="196,109 188,104 188,114" fill="hsl(168 65% 58%)"/>

      <rect x="200" y="24" width="152" height="170" rx="12" fill="hsl(166 44% 14%)" stroke="hsl(160 72% 58%)" stroke-width="1.6"/>
      <text x="276" y="52" text-anchor="middle" fill="hsl(160 84% 86%)" font-size="13" font-weight="700">scene-stream.uorc</text>
      <text x="276" y="76" text-anchor="middle" fill="hsl(156 88% 76%)" font-size="20" font-weight="700">5.7 KB</text>
      <text x="276" y="96" text-anchor="middle" fill="hsl(204 22% 78%)" font-size="11" font-family="JetBrains Mono, monospace">9.3x compression</text>
      <rect x="216" y="114" width="120" height="60" rx="8" fill="hsl(160 70% 50% / 0.15)" stroke="hsl(160 70% 58% / 0.5)"/>
      <text x="276" y="138" text-anchor="middle" fill="hsl(160 88% 86%)" font-size="11" font-family="JetBrains Mono, monospace">uorc_decompress</text>
      <text x="276" y="156" text-anchor="middle" fill="hsl(204 22% 78%)" font-size="10.5" font-family="JetBrains Mono, monospace">SHA-256 chain seal</text>

      <line x1="356" y1="109" x2="390" y2="109" stroke="hsl(192 86% 62%)" stroke-width="2.4"/>
      <polygon points="390,109 382,104 382,114" fill="hsl(192 86% 62%)"/>

      <rect x="394" y="24" width="272" height="170" rx="12" fill="hsl(226 38% 15%)" stroke="hsl(212 88% 68%)" stroke-width="1.6"/>
      <text x="530" y="50" text-anchor="middle" fill="hsl(206 92% 88%)" font-size="13" font-weight="700">Zero-Decode coded_gemm (0 B float scratch)</text>
      <rect x="410" y="64" width="240" height="34" rx="7" fill="hsl(192 80% 50% / 0.16)" stroke="hsl(192 80% 62%)"/>
      <text x="422" y="85" fill="hsl(192 92% 88%)" font-size="11.5" font-family="JetBrains Mono, monospace">Tier 0 · Grid&lt;16&gt; vertex geometry</text>
      <rect x="410" y="106" width="240" height="34" rx="7" fill="hsl(264 80% 64% / 0.16)" stroke="hsl(264 82% 72%)"/>
      <text x="422" y="127" fill="hsl(264 92% 90%)" font-size="11.5" font-family="JetBrains Mono, monospace">Tier 1 · Packed&lt;2&gt; 4-bit anim curves</text>
      <rect x="410" y="148" width="240" height="34" rx="7" fill="hsl(152 72% 48% / 0.16)" stroke="hsl(152 72% 58%)"/>
      <text x="422" y="169" fill="hsl(152 88% 86%)" font-size="11.5" font-family="JetBrains Mono, monospace">Tier 2 · Runs&lt;16&gt; sparse IBL probes</text>
    </svg>
  </div>
  <figcaption><strong>Native WebAssembly zero-decode streaming.</strong> Portable <code>.uorc</code> archives decompress inside the worker and contract directly via <code>uor_matmul::codec</code> without float buffer expansion.</figcaption>
  <details class="cs-data"><summary>Zero-decode codec tier table</summary>
    <table>
      <thead><tr><th>Stage or tier</th><th>Representation</th><th>Scratch memory</th></tr></thead>
      <tbody>
        <tr><td>Portable UORC stream</td><td>4 self-delimiting <code>UORC-SHA256-CHAIN-v1</code> frames (5.7 KB)</td><td>Stateless <code>uorc_decompress::run</code></td></tr>
        <tr><td>Tier 0 · Geometry</td><td><code>CodedMatrix</code> <code>Grid</code> 16-entry codebook</td><td>0 bytes intermediate float buffer</td></tr>
        <tr><td>Tier 1 · Animation</td><td><code>Packed&lt;Grid, 2&gt;</code> sub-byte 4-bit nibbles</td><td>0 bytes intermediate float buffer</td></tr>
        <tr><td>Tier 2 · IBL harmonics</td><td><code>Runs&lt;Grid, 16&gt;</code> run-length sparse stream</td><td>0 bytes intermediate float buffer</td></tr>
      </tbody>
    </table>
  </details>
</figure>`;

const ROLLBACK_TIMELINE_SVG = `
<figure class="cs-figure ve-diagram" id="fig-rollback">
  <div class="cs-canvas">
    <svg viewBox="0 0 680 204" role="img" aria-labelledby="ve-rb-t ve-rb-d" xmlns="http://www.w3.org/2000/svg">
      <title id="ve-rb-t">Deterministic 64-frame rollback and DeltaUorcSnapshot timeline</title>
      <desc id="ve-rb-d">Shows predicted frames advancing from t=60 to t=67, a remote input arriving for frame t=64, ring-buffer state restore at t=64, and deterministic resimulation back to t=67 with identical SHA-256 chain digests.</desc>
      <text x="24" y="32" fill="hsl(196 36% 94%)" font-size="13" font-weight="700">64-Frame Ring Buffer + SHA-256 Chained Input Journal</text>
      <line x1="24" y1="74" x2="650" y2="74" stroke="hsl(204 28% 44%)" stroke-width="2"/>
      <g font-family="JetBrains Mono, monospace" font-size="11">
        <rect x="34" y="54" width="76" height="40" rx="8" fill="hsl(208 28% 15%)" stroke="hsl(204 26% 42%)"/>
        <text x="72" y="78" text-anchor="middle" fill="hsl(204 22% 80%)">f = 62</text>

        <rect x="132" y="54" width="76" height="40" rx="8" fill="hsl(208 28% 15%)" stroke="hsl(204 26% 42%)"/>
        <text x="170" y="78" text-anchor="middle" fill="hsl(204 22% 80%)">f = 63</text>

        <rect x="230" y="48" width="96" height="52" rx="9" fill="hsl(38 72% 18%)" stroke="hsl(42 95% 62%)" stroke-width="2"/>
        <text x="278" y="70" text-anchor="middle" fill="hsl(44 96% 86%)" font-weight="700">f = 64*</text>
        <text x="278" y="88" text-anchor="middle" fill="hsl(44 90% 76%)" font-size="10">late input</text>

        <rect x="348" y="54" width="84" height="40" rx="8" fill="hsl(164 52% 16%)" stroke="hsl(160 72% 58%)"/>
        <text x="390" y="78" text-anchor="middle" fill="hsl(160 86% 86%)">resim 65</text>

        <rect x="454" y="54" width="84" height="40" rx="8" fill="hsl(164 52% 16%)" stroke="hsl(160 72% 58%)"/>
        <text x="496" y="78" text-anchor="middle" fill="hsl(160 86% 86%)">resim 66</text>

        <rect x="560" y="54" width="90" height="40" rx="8" fill="hsl(192 60% 17%)" stroke="hsl(192 88% 62%)" stroke-width="1.8"/>
        <text x="605" y="78" text-anchor="middle" fill="hsl(192 92% 88%)" font-weight="700">sync 67</text>
      </g>
      <path d="M605,100 C540,148 340,148 278,104" fill="none" stroke="hsl(42 95% 62%)" stroke-width="2" stroke-dasharray="6 4"/>
      <text x="442" y="156" text-anchor="middle" fill="hsl(44 96% 84%)" font-size="11.5" font-family="JetBrains Mono, monospace">1. Restore ring state at f=64  -&gt;  2. Replay 64..67  -&gt;  3. Emit XOR DeltaUorcSnapshot</text>
      <text x="24" y="184" fill="hsl(204 22% 76%)" font-size="11.5">Guarded by Lean predicates: rollbackFrameWithinWindow (max 64) &amp; inputJournalSequenceValid</text>
    </svg>
  </div>
  <figcaption><strong>Deterministic rollback and delta state sync.</strong> Late network commands rewind the bounded 64-frame physics ring and resimulate to bit-identical convergence.</figcaption>
  <details class="cs-data"><summary>Rollback and netcode bounds table</summary>
    <table>
      <thead><tr><th>Component</th><th>Bound or format</th><th>Verification predicate</th></tr></thead>
      <tbody>
        <tr><td><code>DeterministicInputJournal</code></td><td>Monotonic frame commands + 32B SHA-256 chain</td><td><code>inputJournalSequenceValid</code></td></tr>
        <tr><td><code>RollbackSession</code></td><td>64-frame <code>PhysicsWorld</code> history ring</td><td><code>rollbackFrameWithinWindow</code></td></tr>
        <tr><td><code>DeltaUorcSnapshot</code></td><td>XOR residual compressed via stateless <code>UORC</code></td><td><code>WorldSnapshot::decode</code> SHA-256 seal</td></tr>
      </tbody>
    </table>
  </details>
</figure>`;

const SEMIRING_MATRIX_SVG = `
<figure class="cs-figure ve-diagram" id="fig-semiring-grid">
  <div class="cs-canvas">
    <svg viewBox="0 0 800 244" role="img" aria-labelledby="ve-sr-t ve-sr-d" xmlns="http://www.w3.org/2000/svg">
      <title id="ve-sr-t">Unified verdant-tensor semiring execution domains</title>
      <desc id="ve-sr-d">Four semiring quadrants in verdant-tensor: Linear exact (+, *), Coded zero-decode streaming, Tropical (max, +) and (min, +), and Modular Q15 Hadamard FDN audio ring.</desc>
      <rect x="16" y="16" width="374" height="98" rx="10" fill="hsl(196 50% 15%)" stroke="hsl(192 88% 62%)" stroke-width="1.6"/>
      <text x="32" y="42" fill="hsl(192 92% 88%)" font-size="13.5" font-weight="700">Linear Exact Semiring (+, *) · i128 / Laurent</text>
      <text x="32" y="64" fill="hsl(196 30% 86%)" font-size="11" font-family="JetBrains Mono, monospace">· Hierarchical ECS &amp; skeleton world matrices</text>
      <text x="32" y="82" fill="hsl(196 30% 86%)" font-size="11" font-family="JetBrains Mono, monospace">· 4-joint LBS skinning &amp; Delassus J*M^-1*J^T</text>
      <text x="32" y="100" fill="hsl(196 30% 86%)" font-size="11" font-family="JetBrains Mono, monospace">· 16x16 software tile PBR + 4-cascade CSM</text>

      <rect x="410" y="16" width="374" height="98" rx="10" fill="hsl(262 40% 17%)" stroke="hsl(268 86% 74%)" stroke-width="1.6"/>
      <text x="426" y="42" fill="hsl(268 92% 90%)" font-size="13.5" font-weight="700">Coded Zero-Decode · uor_matmul::codec</text>
      <text x="426" y="64" fill="hsl(196 30% 86%)" font-size="11" font-family="JetBrains Mono, monospace">· Grid&lt;16&gt; quantized vertex batches</text>
      <text x="426" y="82" fill="hsl(196 30% 86%)" font-size="11" font-family="JetBrains Mono, monospace">· Packed&lt;Grid, 2&gt; nibble animation curves</text>
      <text x="426" y="100" fill="hsl(196 30% 86%)" font-size="11" font-family="JetBrains Mono, monospace">· Runs&lt;Grid, 16&gt; sparse GGX IBL harmonics</text>

      <rect x="16" y="128" width="374" height="98" rx="10" fill="hsl(36 48% 16%)" stroke="hsl(42 94% 62%)" stroke-width="1.6"/>
      <text x="32" y="154" fill="hsl(44 96% 88%)" font-size="13.5" font-weight="700">Tropical Semirings (max, +) &amp; (min, +)</text>
      <text x="32" y="176" fill="hsl(196 30% 86%)" font-size="11" font-family="JetBrains Mono, monospace">· Batch 6-plane Gribb-Hartmann frustum cull</text>
      <text x="32" y="194" fill="hsl(196 30% 86%)" font-size="11" font-family="JetBrains Mono, monospace">· GJK Minkowski directional support maps</text>
      <text x="32" y="212" fill="hsl(196 30% 86%)" font-size="11" font-family="JetBrains Mono, monospace">· NavMesh all-pairs shortest-path closure</text>

      <rect x="410" y="128" width="374" height="98" rx="10" fill="hsl(156 44% 15%)" stroke="hsl(154 74% 56%)" stroke-width="1.6"/>
      <text x="426" y="154" fill="hsl(154 88% 86%)" font-size="13.5" font-weight="700">Quantized Audio Ring · Q15 + Hadamard FDN</text>
      <text x="426" y="176" fill="hsl(196 30% 86%)" font-size="11" font-family="JetBrains Mono, monospace">· Toeplitz Euler-Bernoulli modal FIR</text>
      <text x="426" y="194" fill="hsl(196 30% 86%)" font-size="11" font-family="JetBrains Mono, monospace">· Woodworth ITD/ILD + BVH ray occlusion</text>
      <text x="426" y="212" fill="hsl(196 30% 86%)" font-size="11" font-family="JetBrains Mono, monospace">· 4-channel unitary Hadamard Sabine reverb</text>
    </svg>
  </div>
  <figcaption><strong>Four unified semiring execution domains in <code>verdant-tensor</code>.</strong> Every hot loop reduces to an exact, rounding-free contraction over <code>uor-matmul</code>.</figcaption>
  <details class="cs-data"><summary>Semiring kernel domain table</summary>
    <table>
      <thead><tr><th>Semiring domain</th><th>Algebra</th><th>Engine subsystems</th></tr></thead>
      <tbody>
        <tr><td><code>LinearExact</code></td><td><code>(+, *)</code> exact Laurent / <code>i128</code></td><td>Transforms, 4-joint LBS skinning, Delassus joints, 16x16 tile PBR</td></tr>
        <tr><td><code>CodedZeroDecode</code></td><td><code>coded_gemm</code> over <code>Grid</code>, <code>Packed</code>, <code>Runs</code></td><td>Zero-scratch compressed geometry, animation tracks, and IBL probes</td></tr>
        <tr><td><code>TropicalMaxPlus</code> / <code>MinPlus</code></td><td><code>(max, +)</code> and <code>(min, +)</code></td><td>Frustum culling, GJK hull support, and NavMesh shortest-path closure</td></tr>
        <tr><td><code>ModularAudioRing</code></td><td>Quantized <code>Q15</code> -> <code>i64</code></td><td>Modal FIR bars, Woodworth stereo pan, and 4x4 Hadamard FDN reverb</td></tr>
      </tbody>
    </table>
  </details>
</figure>`;

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
        <a href="${docsRoot}guides/custom-game-world/">Authoring &amp; GLB</a>
        <a href="${docsRoot}guides/progressive-uorc-streaming/">UORC streaming</a>
        <a href="${docsRoot}reference/engine-api/">API reference</a>
        <a href="${docsRoot}explanation/tensor-and-uorc-streaming/">Tensor core</a>
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
  </body>
</html>
`;
}

const PAGES = [
  {
    rel: "docs/index.html",
    title: "Verdant Engine documentation",
    summary: "Build fast 3D games and interactive worlds for the web and Rust.",
    journey: "discover",
    audience: "kai",
    figure: {
      kind: "quadrant",
      title: "Four ways to explore Verdant Engine documentation",
      caption: "Pick a hands-on tutorial, an authoring guide, the crate API reference, or the semiring tensor architecture.",
      x: ["Study and design", "Hands-on code"],
      y: ["Practical steps", "Deep context"],
      cells: [
        { label: "Getting started", sub: "Run a 3D world in five minutes", x: 1, y: 0, href: "./getting-started/" },
        { label: "Authoring & GLB", sub: "Import GLB, XPBD joints, and audio", x: 1, y: 1, href: "./guides/custom-game-world/" },
        { label: "API reference", sub: "Crates, rollback, and CLI commands", x: 0, y: 0, href: "./reference/engine-api/" },
        { label: "Tensor architecture", sub: "Semirings and zero-decode UORC", x: 0, y: 1, href: "./explanation/tensor-and-uorc-streaming/" },
      ],
    },
    body: `
      <h2>Choose your path by role</h2>
      <p>Verdant Engine is a universal 3D game engine written in Rust for native tools and WebAssembly browsers.</p>
      <p>Every pass runs on standard 3D constructs: scene worlds, bounding volume hierarchies, XPBD physics, spatial audio, WGSL shaders, glTF 2.0, and UORC streaming.</p>
      <div data-content="cast"></div>
      <h2>Full-stack engine architecture</h2>
      <p>Constructive Prism proofs generate the semiring kernel bounds that drive <code>verdant-tensor</code>, <code>verdant-core</code>, and <code>verdant-wasm</code>.</p>
      ${ARCH_STACK_DIAGRAM}
      <h2>Documentation map</h2>
      <ul>
        <li><a href="./getting-started/">Build your first 3D world</a> walks through spawning entities, stepping physics, and saving a binary asset.</li>
        <li><a href="./guides/custom-game-world/">Author scenes, physics joints, and spatial audio</a> covers <code>.glb</code> import, XPBD joints, vehicles, and occluded 3D sound.</li>
        <li><a href="./guides/progressive-uorc-streaming/">Stream zero-decode scenes with UORC</a> shows how <code>uorc_decompress</code> and <code>coded_gemm</code> stream LOD0 through LOD3 in WebAssembly.</li>
        <li><a href="./reference/engine-api/">Verdant Engine API reference</a> lists crates, types, deterministic rollback sessions, CLI commands, and 60fps budgets.</li>
        <li><a href="./explanation/tensor-and-uorc-streaming/">Unified semiring tensor core and UORC streaming</a> explains linear, tropical, coded, and audio-ring contractions.</li>
      </ul>
    `,
    next: { href: "getting-started/", label: "Build your first 3D world" },
  },
  {
    rel: "docs/getting-started/index.html",
    title: "Build your first 3D world",
    summary: "Create a Rust 3D scene, step rigid-body physics, and export a binary asset.",
    diataxis: "tutorial",
    journey: "learn",
    audience: "kai",
    figure: {
      kind: "flow",
      title: "Four steps from starter crate to live 3D scene",
      caption: "The starter project builds a scene world, constructs a BVH, steps 60Hz physics, and exports a validated GLB file.",
      steps: [
        { label: "Spawn entities", sub: "SceneWorld + PBR materials", icon: ICONS.cube },
        { label: "Build BVH", sub: "Raycasts + frustum culling", icon: ICONS.bvh },
        { label: "Step physics", sub: "60Hz joints + vehicles", icon: ICONS.physics },
        { label: "Stream UORC", sub: "Portable GLB + UORC pack", icon: ICONS.stream },
      ],
    },
    body: `
      <h2>Run the starter template</h2>
      <p>The repository ships a runnable project in <code>templates/browser-3d-starter</code> so you can launch a 3D game world right away.</p>
      <pre><code>cargo run --manifest-path templates/browser-3d-starter/Cargo.toml</code></pre>
      <p>That command builds <code>verdant-core</code>, validates all five built-in WGSL shaders, steps GJK triggers and NavMesh steering, and writes <code>custom-starter-world.glb</code>.</p>
      <h2>Spawn terrain and child beacons</h2>
      <p>Open <code>templates/browser-3d-starter/src/main.rs</code> to inspect how a scene world organizes parent and child 3D objects.</p>
      <p>You spawn a root terrain patch first, attach fluted columns and beacons as children, and build the bounding volume hierarchy.</p>
      <pre><code>let mut scene = SceneWorld::new();
let ground_id = scene.spawn("StarterTerrain", EntityRole::TerrainPatch, None, Transform::default(), bounds, (1.0, "Terrain"));
let beacon_id = scene.spawn("NorthBeacon", EntityRole::StaticArchitecture, Some(col_id), Transform::from_xyz(0.0, 4.6, 0.0), beacon_bounds, (1.0, "Beacon"));
let bvh = scene.build_bvh();</code></pre>
      <h2>Capture a world snapshot and portable UORC stream</h2>
      <p>Before exporting the scene, encode a world snapshot and a portable multi-LOD UORC stream with SHA-256 verification.</p>
      <pre><code>let snap_bytes = WorldSnapshot::encode(&amp;lod_scene)?;
let restored = WorldSnapshot::decode(&amp;snap_bytes)?;
let uorc_bytes = pack_portable_progressive_uorc_stream(7)?;</code></pre>
      <h2>Play the starter arena in the browser</h2>
      <p>Serve <code>templates/browser-3d-starter</code> over HTTP and open <code>index.html</code> to pilot the probe with WASD keys and activate all three beacons.</p>
    `,
    prev: { href: "", label: "Verdant Engine documentation" },
    next: { href: "guides/custom-game-world/", label: "Author scenes, physics joints, and spatial audio" },
  },
  {
    rel: "docs/guides/custom-game-world/index.html",
    title: "Author scenes, physics joints, and spatial audio",
    summary: "Import GLB scenes, solve articulated joints and vehicles, and mix occluded 3D sound.",
    diataxis: "howToGuide",
    journey: "build",
    audience: "iris",
    figure: {
      kind: "graph",
      title: "Authoring, physics, and sensory pipeline in verdant-core",
      caption: "External GLB assets, procedural meshes, XPBD joints, raycast vehicles, and occluded audio unite inside SceneWorld.",
      nodes: [
        { id: "glb", label: "GLB importer", sub: "Meshes, skins, clips, PBR", layer: 0 },
        { id: "mat", label: "WGSL shaders", sub: "5 validated Naga modules", layer: 0 },
        { id: "ecs", label: "SceneWorld + BVH", sub: "Exact hierarchy + raycasts", layer: 1 },
        { id: "phy", label: "XPBD + vehicles", sub: "Joints, capsule, 4-wheel rig", layer: 1 },
        { id: "out", label: "UORC + Studio", sub: "Progressive LOD0..LOD3", layer: 2 },
      ],
      edges: [
        { from: "glb", to: "ecs" },
        { from: "mat", to: "ecs" },
        { from: "ecs", to: "phy" },
        { from: "ecs", to: "out" },
        { from: "phy", to: "out" },
      ],
    },
    body: `
      <h2>Import external GLB scenes and transcode to UORC</h2>
      <p>Call <code>import_glb_to_scene</code> to load node hierarchies, indexed meshes, skin weights, skeletal clips, and PBR materials into a <code>SceneWorld</code>.</p>
      <p>Call <code>transcode_glb_to_progressive_uorc</code> or run <code>verdant transcode-glb authored.glb scene.uorc</code> to emit a four-LOD stream.</p>
      <pre><code>let imported = import_glb_to_scene(&amp;glb_bytes)?;
let uorc_stream = transcode_glb_to_progressive_uorc(&amp;glb_bytes, 7)?;</code></pre>
      <h2>Configure XPBD joints, swept capsules, and raycast vehicles</h2>
      <p>Attach <code>RigidJointConstraint</code> links with <code>Hinge</code>, <code>BallSocket</code>, <code>Slider</code>, or <code>SpringDamper</code> topology solved via Delassus matrices.</p>
      <p>Drive characters over stairs with <code>step_swept_against_bvh</code> and simulate four-wheel suspension rigs with <code>RaycastVehicle</code>.</p>
      <h2>Validate portable WGSL shaders</h2>
      <p>Every built-in PBR, terrain, fluid, sky, and skinned shadow shader passes W3C <code>naga</code> validation before asset packing.</p>
      <pre><code>let shaders = validate_builtin_engine_shaders()?;
assert_eq!(shaders.len(), 5);</code></pre>
      <h2>Mix occluded spatial audio and Hadamard reverb</h2>
      <p>Call <code>evaluate_binaural_cue_with_occlusion</code> to cast a line-of-sight ray against the BVH and attenuate obstructed sound sources.</p>
      <p>Call <code>synthesize_binaural_reverb_quantum_exact</code> to convolve Euler-Bernoulli modal bars and step the four-channel Hadamard FDN tank.</p>
    `,
    prev: { href: "getting-started/", label: "Build your first 3D world" },
    next: { href: "guides/progressive-uorc-streaming/", label: "Stream zero-decode scenes with UORC" },
  },
  {
    rel: "docs/guides/progressive-uorc-streaming/index.html",
    title: "Stream zero-decode scenes with UORC",
    summary: "Pack and read UORC scene streams step by step in the browser.",
    diataxis: "howToGuide",
    journey: "discover",
    audience: "wren",
    figure: {
      kind: "flow",
      title: "Four progressive UORC streaming levels from LOD0 to LOD3",
      caption: "Browser workers decode stateless UORC frames in WebAssembly and refine scenes monotonically to lossless source fidelity.",
      steps: [
        { label: "LOD0 proxy", sub: "1/8 scale · bounds + NavMesh", icon: ICONS.bvh },
        { label: "LOD1 medium", sub: "2/8 scale · terrain + shadows", icon: ICONS.cube },
        { label: "LOD2 high", sub: "5/8 scale · IBL + vehicle rig", icon: ICONS.physics },
        { label: "LOD3 lossless", sub: "8/8 scale · 5.7 KB UORC", icon: ICONS.stream },
      ],
    },
    body: `
      <h2>Decode UORC natively inside the browser worker</h2>
      <p>Verdant Engine compiles <code>uorc_decompress</code> directly to <code>wasm32-unknown-unknown</code> without shipping the five-megabyte search tables.</p>
      <p>Cold boot stays inside the 120 KiB mobile budget while <code>verdant-uorc.wasm</code> decodes <code>scene-stream.uorc</code> frames on demand.</p>
      ${ZERO_DECODE_SVG}
      <h2>Contract CodedMatrix tiers without float expansion</h2>
      <p>Each streamed tier pairs with <code>uor_matmul::codec</code> so <code>coded_gemm</code> contracts compressed weights without intermediate float buffers.</p>
      <p>Geometry uses <code>Grid</code> lookup tables, animation tracks use 4-bit <code>Packed</code> nibbles, and IBL probes use sparse <code>Runs</code>.</p>
      <h2>Scrub progressive LODs and inspect visual telemetry</h2>
      <p>Select <strong>LOD</strong> or <strong>Stream .uorc</strong> in the Studio rail to step from coarse proxy bounds to lossless <code>LOD3</code> detail.</p>
      <p>Select <strong>Visuals</strong> in the Studio rail to open the high-contrast pipeline, semiring kernel, and UORC compression inspector.</p>
      <pre><code>let stream = pack_portable_progressive_uorc_stream(7)?;
let lod3 = decode_portable_uorc_single_frame(&amp;stream, 3)?;
let coded = CodedLodFrameBundle::synthesize(7, 3)?;</code></pre>
    `,
    prev: { href: "guides/custom-game-world/", label: "Author scenes, physics joints, and spatial audio" },
    next: { href: "reference/engine-api/", label: "Verdant Engine API reference" },
  },
  {
    rel: "docs/reference/engine-api/index.html",
    title: "Verdant Engine API reference",
    summary: "Look up crates, modules, rollback types, CLI commands, and 60fps budgets.",
    diataxis: "reference",
    journey: "scale",
    audience: "otto",
    figure: {
      kind: "bars",
      title: "60fps worker frame stage budgets in microseconds",
      caption: "A 60fps frame spans 16,666 microseconds; the six-stage parallel worker DAG completes in 6,300 microseconds.",
      unit: "us",
      bound: { value: 16666, label: "60fps frame ceiling (16,666 us)" },
      rows: [
        { label: "ECS hierarchy", value: 900, note: "Topological uor-matmul propagation" },
        { label: "Physics + XPBD", value: 1800, note: "Delassus joints + 4-wheel vehicle" },
        { label: "BVH + cull", value: 1100, note: "SAH build + tropical frustum cull" },
        { label: "NavMesh min-plus", value: 700, note: "Tropical shortest paths + spline" },
        { label: "FDN audio ring", value: 600, note: "Modal FIR + occluded Woodworth" },
        { label: "Tile and draw", value: 1200, note: "WebGL2 or 16x16 software PBR" },
      ],
    },
    body: `
      <h2>Semiring and simulation modules</h2>
      <table>
        <caption>Crates and simulation modules across verdant-tensor and verdant-core</caption>
        <thead>
          <tr><th>Module</th><th>Primary types and functions</th><th>Purpose</th></tr>
        </thead>
        <tbody>
          <tr><td><code>verdant_tensor::linear</code></td><td><code>propagate_hierarchy_matrices_exact</code>, <code>delassus_effective_mass_matrix</code></td><td>Exact <code>(+, *)</code> transforms, 4-joint LBS skinning, and Delassus operators</td></tr>
          <tr><td><code>verdant_tensor::coded</code></td><td><code>contract_coded_geometry_grid</code>, <code>contract_packed_animation_tracks</code></td><td>Zero-decode <code>coded_gemm</code> over <code>Grid</code>, <code>Packed</code>, and <code>Runs</code></td></tr>
          <tr><td><code>verdant_tensor::tropical</code></td><td><code>tropical_frustum_distance_bounds</code>, <code>tropical_min_plus_all_pairs_closure</code></td><td>Tropical <code>(max, +)</code> culling/GJK and <code>(min, +)</code> NavMesh closure</td></tr>
          <tr><td><code>verdant_tensor::audio_ring</code></td><td><code>convolve_modal_fir_exact</code>, <code>step_sabine_fdn_reverb_ring</code></td><td>Quantized <code>Q15</code> FIR bars, stereo pan, and 4x4 Hadamard FDN</td></tr>
          <tr><td><code>ecs</code> &amp; <code>render</code></td><td><code>SceneWorld</code>, <code>BoundingVolumeHierarchy</code>, <code>RenderFrameGraph</code></td><td>Entity hierarchy, SAH raycasts, 5-pass CSM/IBL graph, and WGSL</td></tr>
          <tr><td><code>streaming</code></td><td><code>StandardSceneState</code>, <code>decode_portable_uorc_single_frame</code>, <code>CodedLodFrameBundle</code></td><td>Stateless Wasm UORC streaming and zero-decode coded bundles</td></tr>
        </tbody>
      </table>
      <h2>Authoring, physics, and deterministic rollback modules</h2>
      <table>
        <caption>GLB transcoding, articulated dynamics, audio occlusion, and netcode</caption>
        <thead>
          <tr><th>Module</th><th>Primary types and functions</th><th>Purpose</th></tr>
        </thead>
        <tbody>
          <tr><td><code>gltf_io</code></td><td><code>import_glb_to_scene</code>, <code>transcode_glb_to_progressive_uorc</code></td><td>Full Khronos GLB ingestion and progressive UORC transcoding</td></tr>
          <tr><td><code>physics</code></td><td><code>RigidJointConstraint</code>, <code>RaycastVehicle</code>, <code>CapsuleController</code></td><td>XPBD joints, 4-wheel suspension vehicles, and swept BVH capsules</td></tr>
          <tr><td><code>collision</code> &amp; <code>navmesh</code></td><td><code>gjk_intersect_and_manifold</code>, <code>NavMesh</code></td><td>Tropical GJK hulls, Coulomb friction, and A-star / min-plus paths</td></tr>
          <tr><td><code>audio</code></td><td><code>evaluate_binaural_cue_with_occlusion</code>, <code>synthesize_binaural_reverb_quantum_exact</code></td><td>BVH-occluded Woodworth binaural cues and Hadamard FDN reverb</td></tr>
          <tr><td><code>runtime</code></td><td><code>RollbackSession</code>, <code>DeterministicInputJournal</code>, <code>DeltaUorcSnapshot</code></td><td>64-frame rollback resimulation, SHA-256 input logs, and delta UORC</td></tr>
        </tbody>
      </table>
      <h2>Deterministic rollback and delta snapshot timeline</h2>
      <p>Networked lockstep and rollback sessions record quantized inputs in a SHA-256 chain and synchronize state deltas via stateless UORC.</p>
      ${ROLLBACK_TIMELINE_SVG}
      <h2>CLI subcommands in crates/verdant-cli</h2>
      <ul>
        <li><code>verdant import-glb &lt;file.glb&gt;</code> imports nodes, meshes, skeletons, keyframe clips, and PBR materials.</li>
        <li><code>verdant transcode-glb &lt;in.glb&gt; &lt;out.uorc&gt; [seed]</code> transcodes an authored GLB into a portable four-LOD UORC stream.</li>
        <li><code>verdant pack-portable-uorc &lt;out.uorc&gt; [seed]</code> and <code>verify-portable-uorc &lt;file.uorc&gt;</code> pack and verify stateless Wasm streams.</li>
        <li><code>verdant pack-scene &lt;out.glb&gt;</code>, <code>validate-glb</code>, <code>validate-wgsl</code>, <code>snapshot-verify</code>, and <code>benchmark</code> validate assets in CI.</li>
      </ul>
    `,
    prev: { href: "guides/progressive-uorc-streaming/", label: "Stream zero-decode scenes with UORC" },
    next: { href: "explanation/tensor-and-uorc-streaming/", label: "Unified semiring tensor core and UORC streaming" },
  },
  {
    rel: "docs/explanation/tensor-and-uorc-streaming/index.html",
    title: "Unified semiring tensor core and UORC streaming",
    summary: "See how the engine runs exact math and scene streams in any browser.",
    diataxis: "explanation",
    journey: "evaluate",
    audience: "vera",
    figure: {
      kind: "bars",
      title: "Exact accumulator bit widths across all nine unified tensor kernels",
      caption: "Every Prism-certified kernel in Tensor.lex.tex reserves 64 to 128 exact accumulator bits so contractions never round mid-sum.",
      unit: "bits",
      bound: { value: 64, label: "Minimum exact bound (64 bits)" },
      rows: [
        { label: "Linear (+,*) · transforms (k0,6)", value: 96, note: "96 bits" },
        { label: "Linear (+,*) · LBS & CSM (k1,k4)", value: 128, note: "128 bits" },
        { label: "Tropical (max,+) · cull/GJK (k2)", value: 64, note: "64 bits" },
        { label: "Linear (+,*) · tile PBR (k3)", value: 128, note: "128 bits" },
        { label: "Audio Q15 · FIR & FDN (k5)", value: 128, note: "128 bits" },
        { label: "Tropical (min,+) · NavMesh (k7)", value: 64, note: "64 bits" },
        { label: "Coded zero-decode · stream (k8)", value: 128, note: "128 bits" },
      ],
    },
    body: `
      <h2>Why Verdant Engine distills every pass to semiring tensor logic</h2>
      <p>Conditional browser GPU compute APIs are absent on many mobile browsers, embedded WebViews, and hardened desktop clients.</p>
      <p>Instead of maintaining divergent code paths, <code>Tensor.lex.tex</code> models nine unified semiring kernels executed by <code>crates/verdant-tensor</code> over <code>uor-matmul</code>.</p>
      ${SEMIRING_MATRIX_SVG}
      <h2>Zero-decode contraction with uor_matmul::codec</h2>
      <p>Decompressing assets into temporary float arrays wastes cache bandwidth and mobile memory during progressive scene streaming.</p>
      <p>By pairing <code>uorc_decompress</code> with <code>CodedMatrix</code> tiers, <code>coded_gemm</code> decodes each codebook entry into an exact integer accumulator and writes once.</p>
      <h2>Portable 16x16 software tile rasterization and runtime proofs</h2>
      <p>When WebGL2 is present, the OffscreenCanvas worker streams static and dynamic batches at 60fps off the main thread.</p>
      <p>When hardware contexts are absent, <code>render_triangles_tile_framebuffer</code> shades 16x16 RGBA8 tiles in WebAssembly with identical deterministic visibility.</p>
      <p>Every constant in shipped Rust and constructive LexLean derives from <code>{0, 1}</code>, enforced continuously by the <code>FI-08</code> zero-constants ratchet.</p>
    `,
    prev: { href: "reference/engine-api/", label: "Verdant Engine API reference" },
  },
];

rmSync(join(SITE, "docs"), { recursive: true, force: true });

for (const page of PAGES) {
  const outFile = join(SITE, page.rel);
  mkdirSync(dirname(outFile), { recursive: true });
  writeFileSync(outFile, layout(page));
}

const llmsLines = [
  "# Verdant Engine",
  "",
  "> Universal 3D browser and WebAssembly game engine in Rust (`crates/verdant-tensor`, `crates/verdant-core`, `crates/verdant-wasm`, `crates/verdant-cli`) mechanized with Prism, unified `uor-matmul` semiring contractions, native WebAssembly `uorc_decompress` + `uor_matmul::codec` zero-decode streaming, Khronos `.glb` scene import and `.glb -> .uorc` transcoding, XPBD articulated joints, 4-wheel raycast vehicles, BVH acoustic occlusion, and 64-frame deterministic rollback.",
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
console.log(`generate-docs: wrote ${PAGES.length} pages and site/llms.txt`);
