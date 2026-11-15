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
      <a href="${docsRoot}guides/progressive-uorc-streaming/">UORC streaming</a>
      <a href="${docsRoot}reference/engine-api/">API reference</a>
      <a href="${docsRoot}explanation/tensor-and-uorc-streaming/">Tensor architecture</a>
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
      caption: "Each section serves a distinct engineering goal, from your first 3D scene to the full API reference.",
      x: ["Study and design", "Hands-on code"],
      y: ["Practical steps", "Deep context"],
      cells: [
        { label: "Getting started", sub: "Build a 3D scene in five minutes", x: 1, y: 0, href: "./getting-started/" },
        { label: "Custom world guide", sub: "Terrain, WGSL shaders, and GLB export", x: 1, y: 1, href: "./guides/custom-game-world/" },
        { label: "API reference", sub: "Crates, types, budgets, and snapshots", x: 0, y: 0, href: "./reference/engine-api/" },
        { label: "Tensor architecture", sub: "Exact kernels and progressive UORC", x: 0, y: 1, href: "./explanation/tensor-and-uorc-streaming/" },
      ],
    },
    body: `
      <h2>Choose your path by role</h2>
      <p>Verdant Engine is a standalone 3D game engine written in Rust for native tools and WebAssembly browsers.</p>
      <p>Every subsystem in <code>verdant-core</code> speaks standard 3D concepts: scene worlds, bounding volume hierarchies, rigid bodies, spatial audio, WGSL shaders, glTF 2.0, and UORC streaming.</p>
      <div data-content="cast"></div>
      <h2>Documentation map</h2>
      <ul>
        <li><a href="./getting-started/">Build your first 3D world</a> walks through spawning entities, stepping physics, and saving a binary asset.</li>
        <li><a href="./guides/custom-game-world/">Author terrain, shaders, and spatial audio</a> covers heightfields, procedural meshes, shader checks, and 3D sound.</li>
        <li><a href="./guides/progressive-uorc-streaming/">Stream high-resolution scenes with UORC</a> shows how LOD0 through LOD3 frames stream in the browser studio.</li>
        <li><a href="./reference/engine-api/">Verdant Core API reference</a> lists modules, types, CLI commands, and 60fps frame budgets.</li>
        <li><a href="./explanation/tensor-and-uorc-streaming/">Portable tensor kernels and UORC streaming</a> explains exact Laurent contractions and cryptographic frame chains.</li>
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
      caption: "The starter template builds a scene world, steps 60Hz physics, verifies a UORC stream, and writes a binary GLB file.",
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
      <p>That command builds <code>verdant-core</code>, validates all four built-in WGSL shaders, steps GJK beacon triggers and NavMesh steering, and writes <code>custom-starter-world.glb</code>.</p>
      <h2>Spawn terrain and child beacons</h2>
      <p>Open <code>templates/browser-3d-starter/src/main.rs</code> to inspect how a scene world organizes 3D objects.</p>
      <p>You spawn a root terrain patch first, attach three fluted columns and skinned beacons as children, and build the bounding volume hierarchy.</p>
      <pre><code>let mut scene = SceneWorld::new();
let ground_id = scene.spawn("StarterTerrain", EntityRole::TerrainPatch, None, Transform::default(), bounds, (1.0, "Terrain"));
let beacon_id = scene.spawn("NorthBeacon", EntityRole::StaticArchitecture, Some(col_id), Transform::from_xyz(0.0, 4.6, 0.0), beacon_bounds, (1.0, "Beacon"));
let bvh = scene.build_bvh();</code></pre>
      <h2>Capture a world snapshot and UORC stream</h2>
      <p>Before exporting the scene, encode a world snapshot and a multi-LOD UORC stream with SHA-256 verification.</p>
      <pre><code>let snap_bytes = WorldSnapshot::encode(&amp;lod_scene)?;
let restored = WorldSnapshot::decode(&amp;snap_bytes)?;
let uorc_bytes = pack_progressive_uorc_stream(7)?;</code></pre>
      <h2>Play the starter arena in the browser</h2>
      <p>Serve <code>templates/browser-3d-starter</code> over HTTP and open <code>index.html</code> to pilot the Kinetic Probe with WASD keys and activate all three beacons.</p>
    `,
    prev: { href: "", label: "Verdant Engine documentation" },
    next: { href: "guides/custom-game-world/", label: "Author terrain, shaders, and spatial audio" },
  },
  {
    rel: "docs/guides/custom-game-world/index.html",
    title: "Author terrain, shaders, and spatial audio",
    summary: "Build 3D terrain meshes, check portable shaders, and mix 3D sound.",
    diataxis: "howToGuide",
    journey: "build",
    audience: "iris",
    figure: {
      kind: "graph",
      title: "Asset and sensory pipeline in verdant-core",
      caption: "Procedural geometry, PBR materials, WGSL shaders, and spatial audio converge into browser rendering and GLB export.",
      nodes: [
        { id: "geo", label: "Geometry", sub: "Terrain, columns, morphs", layer: 0 },
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
      <h2>Generate heightfield terrain and procedural meshes</h2>
      <p>Call <code>generate_terrain_mesh</code> with <code>TerrainReliefParams</code> to create an indexed triangle grid with analytical surface normals.</p>
      <p>For columns, faceted prisms, and ribbons, call <code>generate_fluted_entasis_column_mesh</code>, <code>generate_faceted_prism_mesh</code>, or <code>canonical_scene_mesh_library</code>.</p>
      <p>When deformable ribbons bend in the wind, blend vertex offsets with <code>MorphTarget</code>.</p>
      <h2>Validate portable WGSL shaders</h2>
      <p>Every shader passes through W3C <code>naga</code> front-end parsing and module validation before shipping in an asset bundle.</p>
      <pre><code>let shaders = validate_builtin_engine_shaders()?;
assert_eq!(shaders.len(), 4);</code></pre>
      <h2>Mix spatial audio with AudioWorklet synthesis</h2>
      <p>Create an <code>AcousticEnvironment</code> with a <code>ListenerPose</code> and evaluate <code>SpatialAudioSource</code> emitters in 3D space.</p>
      <p>Each call to <code>evaluate_binaural_cue</code> computes Woodworth interaural time difference, level difference, and Sabine RT60 decay for the 128-frame AudioWorklet.</p>
      <h2>Pack and verify assets with verdant-cli</h2>
      <p>Call <code>export_scene_to_glb(&amp;scene, &amp;meshes, &amp;materials)</code> in Rust, or run <code>cargo run -p verdant-cli -- pack-scene out.glb</code> from your shell.</p>
      <p>Run <code>verdant verify-uorc</code>, <code>verdant validate-glb</code>, <code>verdant validate-wgsl</code>, and <code>verdant snapshot-verify</code> in continuous integration.</p>
    `,
    prev: { href: "getting-started/", label: "Build your first 3D world" },
    next: { href: "guides/progressive-uorc-streaming/", label: "Stream high-resolution scenes with UORC" },
  },
  {
    rel: "docs/guides/progressive-uorc-streaming/index.html",
    title: "Stream high-resolution scenes with UORC",
    summary: "Stream multi-LOD 3D scenes smoothly up to lossless source detail.",
    diataxis: "howToGuide",
    journey: "discover",
    audience: "wren",
    figure: {
      kind: "flow",
      title: "Four progressive UORC streaming levels from LOD0 to LOD3",
      caption: "Scenes boot fast from coarse proxy bounds and refine monotonically up to SHA-256 verified lossless resolution.",
      steps: [
        { label: "LOD0 proxy", sub: "1/8 scale · bounds and NavMesh" },
        { label: "LOD1 medium", sub: "2/8 scale · terrain and shadows" },
        { label: "LOD2 high", sub: "5/8 scale · IBL and spline mover" },
        { label: "LOD3 lossless", sub: "8/8 scale · 1:1 UORC source" },
      ],
    },
    body: `
      <h2>Keep the 3D viewport fast on every device</h2>
      <p>Verdant Engine runs simulation, bounding volume hierarchy culling, and WebGL2 or software tile rendering inside an OffscreenCanvas worker.</p>
      <p>The studio bar keeps six clear controls beside the 3D stage: LOD scrubbing, simulation mode, camera mode, spatial audio, glTF export, and world snapshot sharing.</p>
      <h2>Scrub progressive LODs and inspect scene nodes</h2>
      <p>Select <strong>LOD</strong> in the studio bar to step from <code>LOD0</code> proxy bounds up to lossless <code>LOD3</code> detail.</p>
      <p>In Orbit mode, drag or scroll to frame any of the seven canonical engine node presets or follow the kinematic spline mover.</p>
      <p>Switch to Walk mode on desktop to traverse the terrain at eye level with <code>WASD</code> or arrow keys.</p>
      <h2>Pack and verify progressive UORC streams</h2>
      <p>Encode all four LOD tiers into one <code>UORC-SHA256-CHAIN-v1</code> stream and verify every frame within bounded memory.</p>
      <pre><code>let stream = pack_progressive_uorc_stream(7)?;
let summary = verify_progressive_uorc_stream(&amp;stream)?;
let lod3 = decode_uorc_stream_up_to_lod(&amp;stream, 3)?;</code></pre>
    `,
    prev: { href: "guides/custom-game-world/", label: "Author terrain, shaders, and spatial audio" },
    next: { href: "reference/engine-api/", label: "Verdant Core API reference" },
  },
  {
    rel: "docs/reference/engine-api/index.html",
    title: "Verdant Core API reference",
    summary: "Look up engine modules, core types, CLI commands, and 60fps budgets.",
    diataxis: "reference",
    journey: "scale",
    audience: "otto",
    figure: {
      kind: "bars",
      title: "60fps frame stage budgets in microseconds",
      caption: "A 60fps browser frame spans 16,666 microseconds; the six-stage job graph completes in 6,300 microseconds.",
      unit: "us",
      bound: { value: 16666, label: "60fps frame ceiling (16,666 us)" },
      rows: [
        { label: "ECS hierarchy", value: 900, note: "World matrix propagation" },
        { label: "Physics step", value: 1800, note: "Symplectic Euler + GJK contacts" },
        { label: "BVH culling", value: 1100, note: "Frustum planes + tropical bounds" },
        { label: "Spline steering", value: 700, note: "Arc-length mover + NavMesh A*" },
        { label: "Spatial audio", value: 600, note: "Modal harmonics + stereo mix" },
        { label: "Tile and draw", value: 1200, note: "Batch prep + tile rasterizer" },
      ],
    },
    body: `
      <h2>Core modules in crates/verdant-core</h2>
      <table>
        <caption>Simulation, tensor, and streaming modules</caption>
        <thead>
          <tr><th>Module</th><th>Primary types and functions</th><th>Purpose</th></tr>
        </thead>
        <tbody>
          <tr><td><code>math</code></td><td><code>Vec3</code>, <code>Quat</code>, <code>Mat4</code>, <code>Aabb</code>, <code>Frustum</code></td><td>3D linear algebra, rays, and frustum planes</td></tr>
          <tr><td><code>ecs</code></td><td><code>SceneWorld</code>, <code>EntityId</code>, <code>BoundingVolumeHierarchy</code></td><td>Entity hierarchy, raycasts, and frustum culling</td></tr>
          <tr><td><code>tensor</code></td><td><code>batch_transform_positions_exact</code>, <code>render_triangles_tile_framebuffer</code></td><td>Exact <code>uor-matmul</code> contractions and 16x16 tile PBR</td></tr>
          <tr><td><code>streaming</code></td><td><code>StandardSceneState</code>, <code>LodLevel</code>, <code>pack_progressive_uorc_stream</code></td><td>Progressive LOD synthesis and UORC chain streaming</td></tr>
          <tr><td><code>geometry</code></td><td><code>ProceduralMesh</code>, <code>MorphTarget</code>, <code>generate_terrain_mesh</code></td><td>Procedural terrain, columns, prisms, and ribbons</td></tr>
          <tr><td><code>animation</code></td><td><code>Skeleton</code>, <code>SkeletalClip</code>, <code>skin_mesh_linear_blend</code></td><td>Acyclic joint trees and four-joint LBS skinning</td></tr>
          <tr><td><code>render</code></td><td><code>PbrMaterial</code>, <code>RenderFrameGraph</code>, <code>compute_csm_splits</code></td><td>Five-pass PBR graph, CSM splits, IBL mips, and WGSL</td></tr>
        </tbody>
      </table>
      <h2>Physics, audio, and runtime modules</h2>
      <table>
        <caption>Dynamics, navigation, audio, and snapshot modules</caption>
        <thead>
          <tr><th>Module</th><th>Primary types and functions</th><th>Purpose</th></tr>
        </thead>
        <tbody>
          <tr><td><code>gltf_io</code></td><td><code>export_scene_to_glb</code>, <code>validate_glb_bytes</code></td><td>Binary Khronos glTF 2.0 export and verification</td></tr>
          <tr><td><code>collision</code></td><td><code>ConvexHullCollider</code>, <code>gjk_intersect_and_manifold</code></td><td>3D GJK manifolds, Baumgarte, and Coulomb cones</td></tr>
          <tr><td><code>physics</code></td><td><code>PhysicsWorld</code>, <code>RigidBody</code>, <code>KinematicSplineBody</code></td><td>60Hz rigid bodies, capsules, buoyancy, and splines</td></tr>
          <tr><td><code>navmesh</code></td><td><code>NavMesh</code>, <code>NavPolygon</code>, <code>steer_along_corridor</code></td><td>Slope-filtered navmeshes and admissible A-star paths</td></tr>
          <tr><td><code>audio</code></td><td><code>AcousticEnvironment</code>, <code>evaluate_binaural_cue</code></td><td>Woodworth binaural cues, Sabine RT60, and modal bars</td></tr>
          <tr><td><code>runtime</code></td><td><code>EngineConfig</code>, <code>WorkerJobGraph</code>, <code>WorldSnapshot</code></td><td>Topological job scheduling and SHA-256 world snapshots</td></tr>
        </tbody>
      </table>
      <h2>WebAssembly bridge and CLI commands</h2>
      <ul>
        <li><code>crates/verdant-wasm</code> compiles <code>verdant-core</code> to WebAssembly for OffscreenCanvas workers, 16x16 tile rendering, AudioWorklet synthesis, and <code>#vdnt=</code> URLs.</li>
        <li><code>cargo run -p verdant-cli -- pack-scene &lt;out.glb&gt;</code> packs a validated <code>.glb</code>, <code>.vdnt</code> world snapshot, and <code>.uorc</code> stream.</li>
        <li><code>cargo run -p verdant-cli -- verify-uorc &lt;file.uorc&gt;</code> verifies <code>UORC-SHA256-CHAIN-v1</code> multi-LOD archives.</li>
        <li><code>cargo run -p verdant-cli -- validate-glb &lt;file.glb&gt;</code> and <code>validate-wgsl &lt;file.wgsl&gt;</code> run Khronos and W3C validators.</li>
        <li><code>cargo run -p verdant-cli -- benchmark</code> profiles 120 headless simulation frames against the 60fps budget.</li>
      </ul>
    `,
    prev: { href: "guides/progressive-uorc-streaming/", label: "Stream high-resolution scenes with UORC" },
    next: { href: "explanation/tensor-and-uorc-streaming/", label: "Portable tensor kernels and UORC streaming" },
  },
  {
    rel: "docs/explanation/tensor-and-uorc-streaming/index.html",
    title: "Portable tensor kernels and UORC streaming",
    summary: "Learn how exact tensor contractions and UORC chains run in every browser.",
    diataxis: "explanation",
    journey: "evaluate",
    audience: "vera",
    figure: {
      kind: "bars",
      title: "Exact accumulator bit widths across engine tensor kernels",
      caption: "Every Prism-certified engine tensor kernel reserves at least 64 bits so inner products never suffer intermediate rounding.",
      unit: "bits",
      bound: { value: 64, label: "Minimum exact bound (64 bits)" },
      rows: [
        { label: "Static mesh batch", value: 96, note: "LinearExact · k = 4 homogeneous" },
        { label: "Linear skinning", value: 128, note: "LinearExact · 4-joint LBS blend" },
        { label: "Tropical BVH cull", value: 64, note: "TropicalMaxPlus · 6 frustum planes" },
        { label: "Tile rasterizer", value: 128, note: "LinearExact · 16x16 RGBA8 + depth" },
        { label: "Modal audio mix", value: 128, note: "ModularAudioRing · 8 emitters" },
      ],
    },
    body: `
      <h2>Why Verdant Engine hosts tensor compute in Prism</h2>
      <p>Conditional browser GPU compute extensions are absent on many mobile browsers, embedded WebViews, and privacy-hardened clients.</p>
      <p>Instead of branching on optional browser APIs, Verdant Engine models its seven core tensor kernels in Prism and executes them via <code>uor-matmul</code>.</p>
      <p>Every contraction decodes operands into an exact Laurent, integer, or tropical accumulator, sums without intermediate rounding, and encodes once.</p>
      <h2>Portable 16x16 software tile rasterization</h2>
      <p>When a browser provides WebGL2, the worker streams static and dynamic batches at 60fps inside an OffscreenCanvas context.</p>
      <p>When hardware contexts are unavailable, <code>render_triangles_tile_framebuffer</code> shades 16x16 RGBA8 tiles with depth testing in WebAssembly and blits directly to Canvas.</p>
      <h2>Monotonic UORC streaming and runtime integrity</h2>
      <p>Multi-LOD scene worlds compress through UORC into self-delimiting <code>UORC-SHA256-CHAIN-v1</code> archives achieving ten-to-one lossless compression across four progressive tiers.</p>
      <p>At runtime, WGSL shaders are checked by <code>naga</code>, glTF binaries are checked by <code>gltf</code>, and every world snapshot verifies a 32-byte SHA-256 digest.</p>
    `,
    prev: { href: "reference/engine-api/", label: "Verdant Core API reference" },
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
  "> Universal 3D browser and WebAssembly game engine in Rust (`crates/verdant-core`) mechanized with Prism, exact `uor-matmul` tensor contractions, progressive `UORC` multi-LOD streaming, archetype ECS, SAH BVH culling, procedural geometry, skeletal animation, PBR materials, portable W3C WGSL shader validation, Khronos glTF 2.0 binary export, 60Hz rigid-body and GJK physics, NavMesh pathfinding, Woodworth spatial audio, and SHA-256 world snapshots.",
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
