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
    --fd-bg: #151312;
    --fd-fg: #f4f2ee;
    --fd-muted: #b5ada4;
    --fd-card: #1e1b19;
    --fd-border: rgba(244, 242, 238, 0.14);
    --fd-accent: #6ec49c;
  }
  @media (prefers-color-scheme: light) {
    :root {
      --fd-bg: #f5f4ef;
      --fd-fg: #1f1c1a;
      --fd-muted: #5b534c;
      --fd-card: #ffffff;
      --fd-border: rgba(31, 28, 26, 0.14);
      --fd-accent: #1d6f4e;
    }
  }
  * { box-sizing: border-box; }
  body {
    margin: 0;
    background: var(--fd-bg);
    color: var(--fd-fg);
    font-family: system-ui, -apple-system, sans-serif;
    font-size: 1rem;
    line-height: 1.6;
  }
  .ve-nav {
    max-width: 52rem;
    margin: 0 auto;
    padding: 1rem 1.25rem;
    display: flex;
    flex-wrap: wrap;
    gap: 0.75rem 1.25rem;
    border-bottom: 1px solid var(--fd-border);
    font-size: 0.9375rem;
  }
  .ve-nav a, main a {
    color: var(--fd-accent);
    text-decoration: underline;
    text-underline-offset: 0.18em;
  }
  main {
    max-width: 52rem;
    margin: 0 auto;
    padding: 1.75rem 1.25rem 4rem;
  }
  h1 { font-size: 2rem; line-height: 1.2; margin: 0 0 0.75rem; }
  h2 { font-size: 1.375rem; line-height: 1.3; margin: 2.25rem 0 0.75rem; }
  h3 { font-size: 1.125rem; line-height: 1.35; margin: 1.5rem 0 0.5rem; }
  p, li { max-width: 68ch; }
  code, pre { font-family: ui-monospace, SFMono-Regular, Menlo, monospace; font-size: 0.9em; }
  pre {
    padding: 0.875rem 1rem;
    background: var(--fd-card);
    border: 1px solid var(--fd-border);
    border-radius: 0.5rem;
    overflow-x: auto;
  }
  table {
    width: 100%;
    border-collapse: collapse;
    margin: 1rem 0;
    font-size: 0.9375rem;
  }
  th, td {
    text-align: left;
    padding: 0.5rem 0.625rem;
    border-bottom: 1px solid var(--fd-border);
  }
  .dc-pager {
    margin-top: 2.5rem;
    padding-top: 1rem;
    border-top: 1px solid var(--fd-border);
    display: flex;
    flex-wrap: wrap;
    justify-content: space-between;
    gap: 1rem;
  }
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
    <nav class="ve-nav" aria-label="Documentation navigation">
      <a href="${root}">3D Studio</a>
      <a href="${docsRoot}">Docs home</a>
      <a href="${docsRoot}getting-started/">Getting started</a>
      <a href="${docsRoot}guides/custom-game-world/">Custom world guide</a>
      <a href="${docsRoot}reference/engine-api/">API reference</a>
      <a href="${docsRoot}explanation/cosmology-and-builders/">World expansion</a>
      <a href="${docsRoot}case-studies/verdant-resonance/">Verdant Resonance</a>
    </nav>
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
      title: "Four ways to read Verdant Engine documentation",
      caption: "Each Diátaxis section serves a distinct engineering goal, from your first runnable 3D scene to full API reference.",
      x: ["Study and design", "Hands-on code"],
      y: ["Practical steps", "Deep context"],
      cells: [
        { label: "Getting started", sub: "Build a 3D scene in five minutes", x: 1, y: 0, href: "./getting-started/" },
        { label: "Custom world guide", sub: "Terrain, WGSL shaders, and GLB export", x: 1, y: 1, href: "./guides/custom-game-world/" },
        { label: "API reference", sub: "Crates, types, budgets, and snapshots", x: 0, y: 0, href: "./reference/engine-api/" },
        { label: "World expansion", sub: "Cosmic epochs and Heyman statics", x: 0, y: 1, href: "./explanation/cosmology-and-builders/" },
      ],
    },
    body: `
      <h2>Choose your path by role</h2>
      <p>Verdant Engine is a standalone 3D game engine written in Rust for native tools and WebAssembly browsers.</p>
      <p>Every subsystem in <code>verdant-core</code> uses standard game engine concepts: scenes, spatial trees, rigid bodies, 3D audio, shaders, and binary glTF 2.0 assets.</p>
      <div data-content="cast"></div>
      <h2>Documentation map</h2>
      <ul>
        <li><a href="./getting-started/">Build your first 3D world</a> walks through spawning entities, stepping physics, and saving a binary asset.</li>
        <li><a href="./guides/custom-game-world/">Author terrain, shaders, and spatial audio</a> covers heightfields, flora, shader checks, and soundscapes.</li>
        <li><a href="./reference/engine-api/">Verdant Core API reference</a> lists modules, types, error variants, and 60fps frame budgets.</li>
        <li><a href="./explanation/cosmology-and-builders/">World expansion and architectural statics</a> explains cosmic epochs, Builder guilds, and Heyman equilibrium.</li>
        <li><a href="./case-studies/verdant-resonance/">Powering Verdant Resonance</a> shows how the browser studio renders seven structures and an autonomous rover at 60fps.</li>
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
      title: "Four steps from starter template to exported 3D world",
      caption: "The starter template constructs a scene world, steps 60Hz physics, captures a world snapshot, and writes a binary GLB file.",
      steps: [
        { label: "Spawn entities", sub: "SceneWorld and PBR materials" },
        { label: "Build BVH", sub: "Fast raycasts and frustum culling" },
        { label: "Step physics", sub: "60Hz symplectic Euler integration" },
        { label: "Export GLB", sub: "Validated Khronos glTF 2.0 binary" },
      ],
    },
    body: `
      <h2>Run the starter template</h2>
      <p>The repository ships a playable project in <code>templates/browser-3d-starter</code> so you can run a 3D game world right away.</p>
      <pre><code>cargo run --manifest-path templates/browser-3d-starter/Cargo.toml</code></pre>
      <p>That command builds <code>verdant-core</code>, validates all four built-in WGSL shaders, steps GJK relic triggers and NavMesh steering, and writes <code>custom-starter-world.glb</code>.</p>
      <h2>Spawn terrain and child beacons</h2>
      <p>Open <code>templates/browser-3d-starter/src/main.rs</code> to inspect how a scene world organizes 3D objects.</p>
      <p>You spawn a root terrain mesh first, attach three fluted columns and skinned quartz beacons as children, and build the bounding volume hierarchy.</p>
      <pre><code>let mut scene = SceneWorld::new();
let ground_id = scene.spawn("StarterTerrain", EntityRole::TerrainFeature, None, Transform::default(), bounds, (1.0, "Terrain"));
let beacon_id = scene.spawn("NorthRelicBeacon", EntityRole::EngineeredStructure, Some(col_id), Transform::from_xyz(0.0, 4.6, 0.0), beacon_bounds, (1.0, "Beacon"));
let bvh = scene.build_bvh();</code></pre>
      <h2>Capture a deterministic snapshot</h2>
      <p>Before exporting the scene, encode a world snapshot so worker threads or URL share links can restore the exact state with SHA-256 verification.</p>
      <pre><code>let snap_bytes = WorldSnapshot::encode(&amp;causal_world)?;
let restored = WorldSnapshot::decode(&amp;snap_bytes)?;</code></pre>
      <h2>Play Sanctuary Relic Resonator in the browser</h2>
      <p>Serve <code>templates/browser-3d-starter</code> over HTTP and open <code>index.html</code> to pilot the Resonator Orb with WASD keys and awaken all three beacons.</p>
    `,
    prev: { href: "", label: "Verdant Engine documentation" },
    next: { href: "guides/custom-game-world/", label: "Author terrain, shaders, and spatial audio" },
  },
  {
    rel: "docs/guides/custom-game-world/index.html",
    title: "Author terrain, shaders, and spatial audio",
    summary: "Build 3D terrain meshes, check GPU shaders, and mix 3D sound.",
    diataxis: "howToGuide",
    journey: "build",
    audience: "iris",
    figure: {
      kind: "graph",
      title: "Asset and sensory pipeline in verdant-core",
      caption: "Procedural geometry, PBR materials, WGSL shaders, and spatial audio converge into browser rendering and GLB export.",
      nodes: [
        { id: "geo", label: "Geometry", sub: "Terrain, L-systems, morphs", layer: 0 },
        { id: "mat", label: "PBR materials", sub: "Metallic-roughness + WGSL", layer: 0 },
        { id: "ecs", label: "SceneWorld", sub: "Transform hierarchy + BVH", layer: 1 },
        { id: "aud", label: "AudioScene", sub: "Inverse-distance + RT60", layer: 1 },
        { id: "out", label: "Browser and GLB", sub: "WebGL2, WebAudio, glTF 2.0", layer: 2 },
      ],
      edges: [
        { from: "geo", to: "ecs" },
        { from: "mat", to: "ecs" },
        { from: "ecs", to: "out" },
        { from: "aud", to: "out" },
      ],
    },
    body: `
      <h2>Generate heightfield terrain and botanical meshes</h2>
      <p>Call <code>generate_terrain_mesh</code> with <code>TerrainReliefParams</code> to create an indexed triangle grid with analytical surface normals.</p>
      <p>For columns, quartz outcrops, and botanical petals, call <code>generate_fluted_entasis_column_mesh</code>, <code>generate_hexagonal_quartz_mesh</code>, or <code>canonical_scene_mesh_library</code>.</p>
      <p>When foliage bends in the wind, blend vertex offsets with <code>MorphTarget</code>.</p>
      <h2>Validate custom WGSL shaders before GPU upload</h2>
      <p>Every shader passes through W3C <code>naga</code> front-end parsing and module validation before reaching a browser WebGPU or WebGL2 context.</p>
      <pre><code>let shaders = validate_builtin_engine_shaders()?;
assert_eq!(shaders.len(), 4);</code></pre>
      <h2>Mix spatial audio with AudioWorklet synthesis</h2>
      <p>Create an <code>AcousticEnvironment</code> with a <code>ListenerPose</code> and evaluate <code>SpatialAudioSource</code> emitters in 3D space.</p>
      <p>Each call to <code>evaluate_binaural_cue</code> computes Woodworth interaural time difference, level difference, and Sabine RT60 decay for the 128-frame AudioWorklet.</p>
      <h2>Pack and verify assets with verdant-cli</h2>
      <p>Call <code>export_scene_to_glb(&amp;scene, &amp;meshes, &amp;materials)</code> in Rust, or run <code>cargo run -p verdant-cli -- export-sanctuary out.glb</code> from your shell.</p>
      <p>Use <code>verdant validate-glb</code>, <code>verdant validate-wgsl</code>, and <code>verdant snapshot-verify</code> in continuous integration to reject broken assets early.</p>
    `,
    prev: { href: "getting-started/", label: "Build your first 3D world" },
    next: { href: "reference/engine-api/", label: "Verdant Core API reference" },
  },
  {
    rel: "docs/reference/engine-api/index.html",
    title: "Verdant Core API reference",
    summary: "Look up engine modules, core types, error codes, and 60fps frame budgets.",
    diataxis: "reference",
    journey: "scale",
    audience: "otto",
    figure: {
      kind: "bars",
      title: "60fps frame stage budgets in microseconds",
      caption: "A 60fps browser frame spans 16,667 microseconds; the default five-stage job graph completes in 7,900 microseconds.",
      unit: "us",
      bound: { value: 16667, label: "60fps frame ceiling (16,667 us)" },
      rows: [
        { label: "Physics step", value: 1800, note: "Symplectic Euler + heightfield contacts" },
        { label: "Spline rover", value: 600, note: "Autonomous guidance along sanctuary loop" },
        { label: "BVH culling", value: 1100, note: "Surface-area-heuristic tree + frustum planes" },
        { label: "GPU draw prep", value: 3200, note: "Static and dynamic batch submission" },
        { label: "Spatial audio", value: 1200, note: "Modal harmonics + stereo panning" },
      ],
    },
    body: `
      <h2>Crate modules in verdant-core</h2>
      <table>
        <caption>Shipped modules in crates/verdant-core/src</caption>
        <thead>
          <tr><th>Module</th><th>Primary types and functions</th><th>Purpose</th></tr>
        </thead>
        <tbody>
          <tr><td><code>math</code></td><td><code>Vec3</code>, <code>Quat</code>, <code>Mat4</code>, <code>Aabb</code>, <code>Ray</code>, <code>Frustum</code></td><td>3D linear algebra, rays, and frustum planes</td></tr>
          <tr><td><code>ecs</code></td><td><code>SceneWorld</code>, <code>EntityId</code>, <code>Transform</code>, <code>BoundingVolumeHierarchy</code></td><td>Entity hierarchy, world transforms, raycasts, and frustum culling</td></tr>
          <tr><td><code>genesis</code></td><td><code>CausalWorldState</code>, <code>CosmicEpoch</code>, <code>BuilderDiscipline</code>, <code>ArchitecturalBlueprint</code></td><td>Monotonic world expansion and Builder statics</td></tr>
          <tr><td><code>geometry</code></td><td><code>ProceduralMesh</code>, <code>MorphTarget</code>, <code>generate_terrain_mesh</code></td><td>Procedural terrain, fluted columns, crystals, and botanical geometry</td></tr>
          <tr><td><code>animation</code></td><td><code>Skeleton</code>, <code>SkeletalClip</code>, <code>SkinWeights4</code>, <code>skin_mesh_linear_blend</code></td><td>Acyclic joint trees, four-joint LBS skinning, and keyframe sampling</td></tr>
          <tr><td><code>render</code></td><td><code>PbrMaterial</code>, <code>RenderFrameGraph</code>, <code>compute_csm_splits</code>, <code>IblEnvironmentProbe</code></td><td>Five-pass PBR frame graph, four-cascade CSM splits, IBL mips, and WGSL checks</td></tr>
          <tr><td><code>gltf_io</code></td><td><code>export_scene_to_glb</code>, <code>validate_glb_bytes</code></td><td>Binary Khronos glTF 2.0 export and verification</td></tr>
          <tr><td><code>collision</code></td><td><code>ConvexHullCollider</code>, <code>gjk_intersect_and_manifold</code>, <code>solve_contact_impulse</code></td><td>3D GJK convex manifolds, Baumgarte stabilization, and Coulomb friction cones</td></tr>
          <tr><td><code>physics</code></td><td><code>PhysicsWorld</code>, <code>RigidBody</code>, <code>CapsuleController</code>, <code>KinematicSplineRover</code></td><td>60Hz rigid bodies, player capsules, lagoon buoyancy, and spline rovers</td></tr>
          <tr><td><code>navmesh</code></td><td><code>NavMesh</code>, <code>NavPolygon</code>, <code>steer_along_corridor</code></td><td>Slope-filtered walkable navmeshes and admissible A-star corridor steering</td></tr>
          <tr><td><code>audio</code></td><td><code>AcousticEnvironment</code>, <code>ListenerPose</code>, <code>SpatialAudioSource</code>, <code>evaluate_binaural_cue</code></td><td>Binaural spatial audio, RT60, and Euler-Bernoulli chime synthesis</td></tr>
          <tr><td><code>runtime</code></td><td><code>EngineConfig</code>, <code>FrameBudgetProfiler</code>, <code>WorkerJobGraph</code>, <code>WorldSnapshot</code></td><td>Topological job scheduling and SHA-256 world snapshots</td></tr>
        </tbody>
      </table>
      <h2>WebAssembly bridge and CLI packer</h2>
      <ul>
        <li><code>crates/verdant-wasm</code> compiles <code>verdant-core</code> to a compact WebAssembly module for OffscreenCanvas WebGPU and WebGL2 workers, AudioWorklet synthesis, and <code>#vdnt=</code> snapshot URLs.</li>
        <li><code>cargo run -p verdant-cli -- validate-glb &lt;file.glb&gt;</code> checks binary Khronos glTF 2.0 containers and buffer bounds.</li>
        <li><code>cargo run -p verdant-cli -- validate-wgsl &lt;file.wgsl&gt;</code> validates W3C WGSL shaders and entry points via <code>naga</code>.</li>
        <li><code>cargo run -p verdant-cli -- export-sanctuary &lt;out.glb&gt;</code> packs a causal world GLB asset and its companion <code>.vdnt</code> snapshot.</li>
        <li><code>cargo run -p verdant-cli -- snapshot-verify &lt;file.vdnt&gt;</code> verifies SHA-256 trailers and URL bridge byte bounds.</li>
      </ul>
      <h2>Runnable workspace examples</h2>
      <ul>
        <li><code>cargo run -p verdant-core --example cosmic_sanctuary</code> synthesizes the full sanctuary at <code>t_obs</code> and validates shaders, GLB bytes, physics, and audio.</li>
        <li><code>cargo run -p verdant-core --example custom_game_arena</code> builds a non-sanctuary 3D sci-fi arena to demonstrate general-purpose game usage.</li>
        <li><code>cargo run -p verdant-core --example headless_benchmark</code> profiles 120 frames against the 16.67ms frame budget.</li>
      </ul>
    `,
    prev: { href: "guides/custom-game-world/", label: "Author terrain, shaders, and spatial audio" },
    next: { href: "explanation/cosmology-and-builders/", label: "World expansion and architectural statics" },
  },
  {
    rel: "docs/explanation/cosmology-and-builders/index.html",
    title: "World expansion and architectural statics",
    summary: "Learn why natural biomes and constructed monuments emerge in strict order.",
    diataxis: "explanation",
    journey: "evaluate",
    audience: "vera",
    figure: {
      kind: "flow",
      title: "Four monotonic epochs from initial seed to present observation",
      caption: "Natural formations precede agency; constructed stone, bronze, and kinetic artifacts appear only at present observation t_obs.",
      steps: [
        { label: "Cosmogenic", sub: "a = 0.125 · solar flux and seed field" },
        { label: "Lithic-Hydrologic", sub: "a = 0.25 · basin terrain, lagoon, quartz" },
        { label: "Biotic-Ecological", sub: "a = 0.625 · soil and L-system flora" },
        { label: "Agentic-Architectural", sub: "a = 1.00 · five Builder guilds + rover" },
      ],
    },
    body: `
      <h2>Why world synthesis tracks universal expansion</h2>
      <p>When a player enters a rich 3D world, rocks, water, trees, pathways, monuments, and autonomous machines already coexist in one scene.</p>
      <p>Verdant Engine models that scene as a monotonic expansion timeline from initial seed <code>t_0</code> to present observation <code>t_obs</code>.</p>
      <p>Terrain and water condense in the Lithic-Hydrologic epoch, and phyllotactic flora colonizes the basin in the Biotic-Ecological epoch.</p>
      <h2>How the five Builder guilds differentiate</h2>
      <p>Arches, colonnades, sundials, tuned chimes, piers, paved paths, and wheeled rovers do not arise from unguided erosion.</p>
      <p>At scale factor <code>a = 1.0</code>, five specialized Builder disciplines emerge to optimize shelter, solar timekeeping, modal acoustics, safe water crossing, and perimeter stewardship.</p>
      <ul>
        <li><strong>Lithic Masonry</strong> raises the Harmonic Rotunda, the Quartz Torsion Chamber, and the Geodesic Travertine Path Network.</li>
        <li><strong>Celestial Gnomonics</strong> aligns the Equatorial Meridian Sundial with solar elevation.</li>
        <li><strong>Modal Acoustics</strong> suspends golden-ratio bronze tubes on the Modal Chime Pergola.</li>
        <li><strong>Hydrological Engineering</strong> bridges the lagoon with the Stone Pier Causeway.</li>
        <li><strong>Kinetic Automation</strong> deploys the six-wheeled Autonomous Perimeter Sentinel Rover along the perimeter spline.</li>
      </ul>
      <h2>Heyman equilibrium and runtime integrity</h2>
      <p>Every blueprint must satisfy Heyman equilibrium (<code>e / t &lt;= 1 / 6</code>) and a minimum two-to-one safety factor before placement.</p>
      <p>At runtime, WGSL shaders are checked by <code>naga</code>, GLB exports are checked by <code>gltf</code>, and every world snapshot verifies a 32-byte SHA-256 trailer.</p>
    `,
    prev: { href: "reference/engine-api/", label: "Verdant Core API reference" },
    next: { href: "case-studies/verdant-resonance/", label: "Powering Verdant Resonance" },
  },
  {
    rel: "docs/case-studies/verdant-resonance/index.html",
    title: "Powering Verdant Resonance in the browser",
    summary: "See how Verdant Engine runs a 60fps 3D sanctuary with clutter-free controls.",
    diataxis: "explanation",
    journey: "discover",
    audience: "wren",
    figure: {
      kind: "bars",
      title: "Safety factors of the seven sanctuary blueprints at t_obs",
      caption: "Every constructed monument and kinetic rover exceeds the 2.0 minimum structural safety factor.",
      unit: "x",
      bound: { value: 2, label: "Minimum safety factor (2.0x)" },
      rows: [
        { label: "Harmonic Rotunda", value: 3, note: "12 Doric columns + Cardan astrolabe" },
        { label: "Meridian Sundial", value: 3, note: "Equatorial bronze gnomon" },
        { label: "Modal Chime Pergola", value: 4, note: "Six tuned acoustic tubes" },
        { label: "Quartz Torsion Chamber", value: 4, note: "Octagonal basalt + quartz obelisk" },
        { label: "Lagoon Pier Causeway", value: 3, note: "Five masonry piers over water" },
        { label: "Travertine Paths", value: 4, note: "16 geodesic perimeter slabs" },
        { label: "Sentinel Rover", value: 3, note: "Autonomous spline-following rover" },
      ],
    },
    body: `
      <h2>Keeping the 3D viewport front and center</h2>
      <p>Early iterations of Verdant Resonance suffered from cluttered menu controls that distracted players from exploring the 3D sanctuary.</p>
      <p>Extracting Verdant Engine moved simulation, culling, and WebGPU or WebGL2 rendering into an OffscreenCanvas worker backed by <code>verdant-wasm</code>.</p>
      <p>The studio leaves clean actions beside the 3D stage: epoch scrubbing, run mode, camera mode, spatial audio, GLB export, and <code>#vdnt=</code> snapshot sharing.</p>
      <h2>Exploring the sanctuary across devices</h2>
      <p>In Orbit mode, drag or scroll to inspect the rotating Cardan astrolabe rings inside the Harmonic Rotunda or follow the Perimeter Sentinel Rover.</p>
      <p>Switch to Walk mode on desktop to traverse the amphitheater terrain at eye level with <code>WASD</code> or arrow keys.</p>
      <p>Enable Spatial Audio to hear Euler-Bernoulli chime harmonics attenuate smoothly with your 3D distance from the pergola.</p>
      <h2>Verified telemetry in a Web Worker</h2>
      <p>Every epoch change and blueprint selection queries the signed WebAssembly verifier inside a background worker without blocking 60fps rendering.</p>
    `,
    prev: { href: "explanation/cosmology-and-builders/", label: "World expansion and architectural statics" },
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
  "> Commercial 3D browser and WebAssembly game engine in Rust (`crates/verdant-core`) with archetype ECS, SAH BVH culling, procedural terrain and L-system flora, PBR materials, W3C WGSL shader validation, Khronos glTF 2.0 binary export, 60Hz symplectic Euler physics, spatial audio, SHA-256 world snapshots, and verified cosmological Builder statics.",
  "",
  "## Documentation",
  "",
  ...PAGES.map((p) => {
    const url = `${LIVE}${p.rel.replace(/index\.html$/, "")}`;
    const mdUrl = `${LIVE}${p.rel}`;
    void mdUrl;
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
