// Verdant Platform — Authoritative Prism Model-Centric Runtime Taxonomy & Imported Standards Catalogs.
// Every Verdant organ is a differentiated cognitive expert governed by imported authoritative standards
// and verified mechanically against `site/app.holo` + `app/src/*.lex.tex` (Rule R1, R2, R4, R5).

import {
  PRISM_AUDIO_QUANTUM_FRAMES,
  PRISM_CHAMBER_SABINE_RT60_MS,
  PRISM_CODED_TIER_COUNT,
  PRISM_CSM_SPLIT_METERS,
  PRISM_HD_MAX_LOD_RANK,
  PRISM_MODAL_BAR_DENOM,
  PRISM_MODAL_BAR_NUMERS,
  PRISM_TERRAIN_WALKABLE_POLYGONS,
  PRISM_UNIFIED_KERNEL_COUNT,
  PRISM_WORKER_RING_CAPACITY,
} from "./prism-bridge-meta.mjs";

export const VERDANT_PLATFORM_IDENTITY = Object.freeze({
  title: "Verdant",
  thesis: "Prism Model-Centric Runtime — a universal model runtime powered by UOR Prism",
  principle:
    "Verdant never hand-invents ad-hoc specifications: the Cybernetic Observer imports authoritative mathematical, engineering, design, narrative, reasoning, and privacy standards as verified Prism models across the entire product lifecycle.",
});

export const VERDANT_PLATFORM_ORGANS = Object.freeze([
  {
    slug: "engine",
    name: "Verdant Engine",
    moniker: "Runtime Kernel & Spatial Stage",
    route: "/",
    accentToken: "var(--primary)",
    cognitiveRole: "Executes verified semiring contractions, progressive UORC streams, swept BLAS physics, and W5H cybernetic observation at 60 Hz.",
    lexicon: ["Stratum", "Semiring Contraction", "UORC Fidelity Tier", "Guarded Frustum", "Holo Verdict"],
    governingAuthorities: ["uor-matmul", "UORC-SHA256-CHAIN-v1", "IEEE 754-2019", "W3C WGSL / WebGL2"],
    holoQuery: "summary:hd",
  },
  {
    slug: "studio",
    name: "Verdant Studio",
    moniker: "7-Role Application Design Atelier",
    route: "/studio/",
    accentToken: "var(--brand)",
    cognitiveRole: "Authors external games and spatial applications across 7 functional roles driven strictly by imported standards and requirements.",
    lexicon: ["Dashboard", "Code", "Design", "Audio", "Narrative", "Director", "Gameplay"],
    governingAuthorities: ["ISO/IEC/IEEE 42010:2022", "ISO 9241-210:2019", "W3C DTCG 2025.10", "Khronos glTF 2.0"],
    holoQuery: "scene:studio",
  },
  {
    slug: "catalogs",
    name: "Verdant Catalogs",
    moniker: "Universal Registry of Imported Models",
    route: "/catalogs/",
    accentToken: "var(--success)",
    cognitiveRole: "Indexes and live-verifies imported standards, frameworks, characters, languages, reasoning harnesses, narratives, and policies.",
    lexicon: ["Imported Authority", "Built Claim", "LexLean Proof", "Refusal Vector", "Lifecycle Binding"],
    governingAuthorities: ["W3C PROV-O", "ISO/IEC 25010:2023", "ISO/IEC 27005:2022", "WCAG 2.2 AA"],
    holoQuery: "scene:telos",
  },
  {
    slug: "demo",
    name: "Verdant Engine Demo",
    moniker: "Comparative Benchmark Observatory",
    route: "/demo/",
    accentToken: "var(--warning)",
    cognitiveRole: "Measures hardware 3-pass PBR against pure-Wasm 16x16 tile rasterization, instancing, AQM bit-planes, and 15+3+4 semiring proofs.",
    lexicon: ["3-Pass CSM/GGX", "16x16 CSR Tile", "AQM Popcnt", "Zero-Decode CDMX", "Epoch 0 B/f"],
    governingAuthorities: ["ISO/IEC/IEEE 15939", "uor-matmul::codec", "IEEE 754-2019"],
    holoQuery: "scene:perf",
  },
  {
    slug: "docs",
    name: "Verdant Reference",
    moniker: "Diataxis Specification & Live Guides",
    route: "/docs/",
    accentToken: "var(--foreground)",
    cognitiveRole: "Teaches tutorials, task guides, API reference, and mathematical explanations in ISO 24495-1 plain language with live Wasm figures.",
    lexicon: ["Tutorial", "How-To Guide", "API Reference", "Explanation", "Schema.org TechArticle"],
    governingAuthorities: ["ISO 24495-1:2023", "Diataxis", "Schema.org TechArticle"],
    holoQuery: "summary",
  },
]);

export const LIFECYCLE_STAGES = Object.freeze([
  { key: "all", label: "All Lifecycle Phases", order: 0 },
  { key: "conceptualize", label: "01 · Conceptualization", order: 1 },
  { key: "architect", label: "02 · Architecture & Design", order: 2 },
  { key: "modeling", label: "03 · Tensor & World Modeling", order: 3 },
  { key: "development", label: "04 · Development & Harnesses", order: 4 },
  { key: "validation", label: "05 · Validation & Quality", order: 5 },
  { key: "policy", label: "06 · Policy, Privacy & Security", order: 6 },
  { key: "release", label: "07 · Release & Streaming", order: 7 },
  { key: "instances", label: "08 · User Instances & Custody", order: 8 },
]);

export const CATALOG_DOMAINS = Object.freeze([
  { key: "all", code: "ALL", title: "All Catalog Domains" },
  { key: "CAT-ARCH", code: "ARCH", title: "Systems Architecture & Views" },
  { key: "CAT-DESIGN", code: "DSGN", title: "Brand, Design Tokens & Ergonomics" },
  { key: "CAT-LANG", code: "LANG", title: "Formal Languages & Taxonomies" },
  { key: "CAT-TENSOR", code: "TNSR", title: "Semirings, Codecs & UORC Streaming" },
  { key: "CAT-WORLD", code: "WRLD", title: "3D Graphics, Physics, Characters & Audio" },
  { key: "CAT-NARRATIVE", code: "NARR", title: "Narrative, Choreography & Portals" },
  { key: "CAT-REASONING", code: "RSNG", title: "Cybernetic Observer & Local Reasoning" },
  { key: "CAT-POLICY", code: "PLCY", title: "Compliance, Privacy, Quality & Custody" },
]);

export const PRISM_MODEL_CATALOGS = Object.freeze([
  // --- CAT-ARCH: Systems Architecture & Views ---
  {
    id: "STD-ISO-42010",
    title: "Architecture Description & Viewpoint Governance",
    authority: "ISO/IEC/IEEE 42010:2022",
    claimLevel: "Imported Authority -> Built Viewpoints",
    domain: "CAT-ARCH",
    lifecycle: "conceptualize",
    lexModule: "app/src/Architecture.lex.tex",
    holoQueries: ["scene:telos", "scene:studio"],
    refusalQueries: [],
    studioRoles: ["dashboard", "code"],
    taxon: ["Architecture Description", "Concern", "Viewpoint", "Model Kind"],
    invariant:
      "Every Verdant Engine capability traces to an explicit architectural concern and viewpoint before implementation, eliminating unmodeled or ad-hoc features.",
  },
  {
    id: "STD-ISO-19450-OPM",
    title: "Object-Process Methodology (OPM) Bimodal Modeling",
    authority: "ISO/PAS 19450:2015",
    claimLevel: "Imported Standard -> Built Process Graph",
    domain: "CAT-ARCH",
    lifecycle: "architect",
    lexModule: "app/src/Design.lex.tex",
    holoQueries: ["scene:runtime", "scene:pipeline"],
    refusalQueries: [],
    studioRoles: ["dashboard", "code", "design"],
    taxon: ["Object", "Process", "State-Specified Link", "In-Zooming"],
    invariant:
      "Models stateful objects and transforming processes in a single bimodal ontology from asset ingestion through frame presentation.",
  },
  {
    id: "STD-OMG-UML-251",
    title: "Unified Structural, Statechart & Lifeline Views",
    authority: "OMG UML 2.5.1",
    claimLevel: "Imported Standard -> Built Contracts",
    domain: "CAT-ARCH",
    lifecycle: "architect",
    lexModule: "app/src/Design.lex.tex",
    holoQueries: ["scene:wasmbridge", "scene:rendergraph"],
    refusalQueries: [],
    studioRoles: ["code", "director"],
    taxon: ["Component Port", "Connector", "State Machine", "Interaction Lifeline"],
    invariant:
      "Specifies strict host/guest WebAssembly boundary ports, 3-pass render-graph lifelines, and deterministic session state transitions.",
  },

  // --- CAT-DESIGN: Brand, Design Tokens & Ergonomics ---
  {
    id: "STD-WHEELER-BRAND",
    title: "Cohesive Brand Identity & Cognitive Differentiation",
    authority: "Alina Wheeler · Designing Brand Identity (5th ed.)",
    claimLevel: "Imported Framework -> Built Identity",
    domain: "CAT-DESIGN",
    lifecycle: "conceptualize",
    lexModule: "app/src/Brand.lex.tex",
    holoQueries: ["scene:ui", "scene:studio"],
    refusalQueries: [],
    studioRoles: ["design", "dashboard"],
    taxon: ["Brand Ideals", "Differentiation", "Coherence", "Touchpoint Discipline"],
    invariant:
      "Differentiates Verdant Engine, Verdant Studio, Verdant Catalogs, and Verdant Demo with distinct domain missions while preserving unified Hologram-Warm identity.",
  },
  {
    id: "STD-W3C-DTCG",
    title: "Design Tokens Format Module (30 Semantic Tokens)",
    authority: "W3C DTCG 2025.10",
    claimLevel: "Imported Specification -> Built Token Graph",
    domain: "CAT-DESIGN",
    lifecycle: "architect",
    lexModule: "app/src/Brand.lex.tex",
    holoQueries: ["scene:ui"],
    refusalQueries: [],
    studioRoles: ["design", "code"],
    taxon: ["--background", "--card", "--primary", "--brand", "Light / Dark / Immersive"],
    invariant:
      "Binds all surfaces, 2D canvases, and live embeds to W3C DTCG semantic CSS custom properties with zero raw #hex literals.",
  },
  {
    id: "STD-W3C-WCAG22",
    title: "Visual Contrast & Pointer Target Accessibility",
    authority: "W3C WCAG 2.2 Level AA (SC 1.4.3, 1.4.11, 2.5.8)",
    claimLevel: "Imported Standard -> Proved Contrast Pairs",
    domain: "CAT-DESIGN",
    lifecycle: "validation",
    lexModule: "app/src/Brand.lex.tex",
    holoQueries: ["scene:ui"],
    refusalQueries: [],
    studioRoles: ["design", "gameplay"],
    taxon: [">= 4.5:1 Body Contrast", ">= 3.0:1 Non-Text UI", ">= 48x48 CSS px Coarse Target"],
    invariant:
      "Proves luminance contrast across 12 foreground/ground token pairs in Light, Dark, and Immersive modes and enforces 48 px coarse-pointer hit bounds.",
  },
  {
    id: "STD-M3-WINDOW-CLASSES",
    title: "Responsive Window Size Classes & Stage Fit",
    authority: "Material Design 3 Window Size Classes",
    claimLevel: "Imported Layout System -> Verified Gate",
    domain: "CAT-DESIGN",
    lifecycle: "validation",
    lexModule: "app/src/DevRel.lex.tex",
    holoQueries: ["scene:ui"],
    refusalQueries: [],
    studioRoles: ["design", "dashboard"],
    taxon: ["Compact 360", "Medium 600", "Expanded 840", "Large 1200", "Extra-Large 1600", "200% Zoom"],
    invariant:
      "Enforces zero horizontal overflow and exact viewport stage containment across all six Material 3 window classes and 200% text zoom.",
  },
  {
    id: "STD-TROPICAL-UI-SOLVER",
    title: "Tropical (max, +) Non-Overlapping Spatial UI Solver",
    authority: "Idempotent Semiring Layout · Wasm verify_ui",
    claimLevel: "Built Mathematical Solver",
    domain: "CAT-DESIGN",
    lifecycle: "modeling",
    lexModule: "app/src/Subsystems.lex.tex",
    holoQueries: ["kernel:8", "scene:ui"],
    refusalQueries: [],
    studioRoles: ["design", "gameplay"],
    taxon: ["UiTree", "Tropical (max, +)", "Diegetic 3D Anchor", "Zero Pairwise Overlap"],
    invariant:
      "Solves screen-space and 3D diegetic widget rectangles over tropical (max, +) with guaranteed non-overlap and minimum 48 px interactive targets.",
  },

  // --- CAT-LANG: Formal Languages & Taxonomies ---
  {
    id: "STD-LEXLEAN-HOLO",
    title: "Constructive LexLean Kernel & Self-Verifying Holo Archive",
    authority: "Lean 4 v4.32.1 · UOR Prism Core-Wasm",
    claimLevel: "Built Formal Authority (Zero Classical.choice)",
    domain: "CAT-LANG",
    lifecycle: "modeling",
    lexModule: "app/src/Engine.lex.tex",
    holoQueries: ["summary", "summary:hd"],
    refusalQueries: ["unknown"],
    studioRoles: ["code", "dashboard"],
    taxon: ["Constructive Kernel", "BLAKE3 Self-Fingerprint", "Acceptance Replay", "R5 Typed Refusal"],
    invariant:
      "Compiles declarative .lex.tex specifications into a BLAKE3-sealed Core-Wasm archive (`app.holo`) replayed locally in every browser session.",
  },
  {
    id: "STD-ISO-9241-HCD",
    title: "Human-Centred Lifecycle & Role Task Ergonomics",
    authority: "ISO 9241-11:2018 & ISO 9241-210:2019",
    claimLevel: "Imported Standard -> 7 Role Workspaces",
    domain: "CAT-LANG",
    lifecycle: "conceptualize",
    lexModule: "app/src/Users.lex.tex",
    holoQueries: ["scene:studio"],
    refusalQueries: [],
    studioRoles: ["dashboard", "code", "design", "audio", "narrative", "director", "gameplay"],
    taxon: ["Effectiveness", "Efficiency", "Satisfaction", "Context of Use"],
    invariant:
      "Organizes Verdant Studio strictly around 7 functional lifecycle roles and 23 concrete authoring tasks without internal jargon.",
  },
  {
    id: "STD-ISO-24495-DIATAXIS",
    title: "Plain Language, Diataxis Quadrants & Schema.org Metadata",
    authority: "ISO 24495-1:2023 · Diataxis · Schema.org TechArticle",
    claimLevel: "Imported Communication Standards -> Verified Docs",
    domain: "CAT-LANG",
    lifecycle: "release",
    lexModule: "app/src/DevRel.lex.tex",
    holoQueries: ["summary"],
    refusalQueries: [],
    studioRoles: ["dashboard", "narrative"],
    taxon: ["Tutorial", "How-To", "Reference", "Explanation", "Markdown Twin", "llms.txt"],
    invariant:
      "Measures readability grade, sentence length, glossary grounding, live figure parity, and JSON-LD TechArticle metadata across every documentation page.",
  },

  // --- CAT-TENSOR: Semirings, Codecs & UORC Streaming ---
  {
    id: "STD-UOR-MATMUL-15",
    title: `Unified ${PRISM_UNIFIED_KERNEL_COUNT}-Kernel Semiring Contraction Suite`,
    authority: "uor-matmul · Exact Laurent & Idempotent Semirings",
    claimLevel: "Built & Kernel-Proved",
    domain: "CAT-TENSOR",
    lifecycle: "modeling",
    lexModule: "app/src/Tensor.lex.tex",
    holoQueries: [
      "kernel:0", "kernel:1", "kernel:2", "kernel:3", "kernel:4",
      "kernel:5", "kernel:6", "kernel:7", "kernel:8", "kernel:9",
      "kernel:10", "kernel:11", "kernel:12", "kernel:13", "kernel:14",
    ],
    refusalQueries: ["kernel:zerodim", "kernel:underflow"],
    studioRoles: ["code", "audio", "director", "gameplay"],
    taxon: ["LinearExact", "TropicalMaxPlus", "TropicalMinPlus", "ModularRing", "CodedZeroDecode"],
    invariant:
      `Unifies transforms, skinning, BVH culling, tile rasterization, A* pathing, UI layout, audio FDN, LIF spikes, SLM routing, and Stitch parity over ${PRISM_UNIFIED_KERNEL_COUNT} overflow-safe semiring kernels while refusing zero-extent or narrow accumulators.`,
  },
  {
    id: "STD-CODED-ZERO-DECODE",
    title: `Zero-Decode Coded Contraction Tiers (${PRISM_CODED_TIER_COUNT} Codecs)`,
    authority: "uor-matmul::codec (Grid, Packed, Runs)",
    claimLevel: "Built & Checksum-Proved",
    domain: "CAT-TENSOR",
    lifecycle: "modeling",
    lexModule: "app/src/Tensor.lex.tex",
    holoQueries: ["coded:0", "coded:1", "coded:2", "scene:uorc-exclusive"],
    refusalQueries: [],
    studioRoles: ["code", "director"],
    taxon: ["Grid Codec", "Packed Bit-Stream", "Run-Length Semiring", "coded_gemm"],
    invariant:
      "Executes matrix contractions directly over compressed UORC and SLM weight streams without materializing dense intermediate float buffers.",
  },
  {
    id: "STD-UORC-PROGRESSIVE",
    title: `Monotone Progressive UORC Streaming (LOD0..LOD${PRISM_HD_MAX_LOD_RANK})`,
    authority: "UORC-SHA256-CHAIN-v1 · Content-Addressed Geometry",
    claimLevel: "Built & Monotonicity-Proved",
    domain: "CAT-TENSOR",
    lifecycle: "release",
    lexModule: "app/src/Tensor.lex.tex",
    holoQueries: ["lod:0", "lod:1", "lod:2", "lod:3", "lod:4", "scene:uorc"],
    refusalQueries: [],
    studioRoles: ["design", "code", "director"],
    taxon: ["LOD0 Proxy", "LOD2 High", "LOD3 Lossless", "LOD4 Ultra HD", "SHA-256 Chain"],
    invariant:
      "Streams 3D geometry strictly monotonically from coarse collision proxy through lossless source and HD architectural synthesis with 1-bit tamper refusal.",
  },
  {
    id: "STD-IEEE754-FI08",
    title: "Overflow-Safe Accumulators & FI-08 {0, 1} Parametric Discipline",
    authority: "IEEE 754-2019 · Prism Fidelity Ratchet FI-08",
    claimLevel: "Imported Arithmetic -> Enforced Source Ratchet",
    domain: "CAT-TENSOR",
    lifecycle: "development",
    lexModule: "app/src/Fidelity.lex.tex",
    holoQueries: ["scene:stability"],
    refusalQueries: ["kernel:underflow"],
    studioRoles: ["code"],
    taxon: ["Exact Accumulator Width", "NaN Sanitization", "Zero Magic Literals ({0, 1})"],
    invariant:
      "Rejects narrow accumulators in Lean and mechanically rejects any numeric literal outside {0, 1} across shipped Rust crates and generated Studio scaffolds.",
  },

  // --- CAT-WORLD: 3D Graphics, Physics, Characters & Audio ---
  {
    id: "STD-KHRONOS-GLTF2",
    title: "Binary 3D Scene Interchange & OPFS Transcoding",
    authority: "Khronos glTF 2.0 Specification (.glb)",
    claimLevel: "Imported Industry Standard -> Built I/O",
    domain: "CAT-WORLD",
    lifecycle: "development",
    lexModule: "app/src/Subsystems.lex.tex",
    holoQueries: ["scene:pipeline"],
    refusalQueries: [],
    studioRoles: ["design", "code"],
    taxon: ["GLB Binary Chunk", "PBR Metallic-Roughness", "OPFS Vault", "UORC Transcode"],
    invariant:
      "Imports and exports standard binary Khronos .glb assets directly in WebAssembly with optional Origin Private File System (.uorc) persistence.",
  },
  {
    id: "STD-WGSL-PBR-TILE",
    title: "Dual-Backend 3-Pass PBR & 16x16 Wasm Tile Rasterizer",
    authority: "W3C WGSL (naga) · WebGL2 · 4-Cascade CSM · ACES",
    claimLevel: "Imported Graphics Standards -> Dual Rasterizer",
    domain: "CAT-WORLD",
    lifecycle: "modeling",
    lexModule: "app/src/Subsystems.lex.tex",
    holoQueries: ["scene:rendergraph", "kernel:2", "kernel:3"],
    refusalQueries: [],
    studioRoles: ["design", "director"],
    taxon: [`CSM Splits (${PRISM_CSM_SPLIT_METERS.map((v) => v.toFixed(1)).join("/")}m)`, "GGX Microfacet", "ACES Filmic", "16x16 CSR Tile"],
    invariant:
      "Pairs hardware 3-pass light-space texel-snapped CSM + GGX + ACES rendering with a bit-deterministic 16x16 WebAssembly software tile fallback.",
  },
  {
    id: "STD-SWEPT-BLAS-XPBD",
    title: "Two-Level SAH BVH, Swept Triangle BLAS CCD & XPBD Joints",
    authority: "Continuous Collision Detection & Extended Position-Based Dynamics",
    claimLevel: "Built Physical Lawfulness",
    domain: "CAT-WORLD",
    lifecycle: "modeling",
    lexModule: "app/src/Subsystems.lex.tex",
    holoQueries: ["scene:collision", "scene:joints", "scene:lawful"],
    refusalQueries: [],
    studioRoles: ["code", "gameplay"],
    taxon: ["TwoLevelBvh", "Swept Capsule CCD", "Crease Projection", "XPBD Distance/Hinge"],
    invariant:
      "Prevents tunneling across concave masonry arches and stairs via exact swept capsule-triangle intersection and articulated XPBD compliance constraints.",
  },
  {
    id: "STD-CHARACTER-IK-NPC",
    title: `22-Joint Foot IK Characters, NPC Directives & ${PRISM_TERRAIN_WALKABLE_POLYGONS.toLocaleString()}-Poly NavMesh`,
    authority: "Linear Blend Skinning · Analytic Two-Bone IK · A* BDI Directives",
    claimLevel: "Built Character & Agent Model",
    domain: "CAT-WORLD",
    lifecycle: "modeling",
    lexModule: "app/src/Subsystems.lex.tex",
    holoQueries: ["scene:character", "scene:animation", "scene:navmesh", "kernel:7"],
    refusalQueries: [],
    studioRoles: ["gameplay", "narrative", "director"],
    taxon: ["22-Joint Rig", "Analytic Two-Bone Foot IK", "NPC Archetype", "Guiding Directive", "Tropical A*"],
    invariant:
      "Grounds player and NPC characters on sloped masonry with analytic two-bone Foot IK, tropical (min, +) NavMesh routing, and explicit behavioral directives.",
  },
  {
    id: "STD-SABINE-MODAL-AUDIO",
    title: "Euler-Bernoulli Modal Synthesis, Sabine RT60 & Woodworth ITD",
    authority: "Physical Acoustics · Sabine (1900) · Woodworth Spherical Head",
    claimLevel: "Imported Acoustic Laws -> Built Synth",
    domain: "CAT-WORLD",
    lifecycle: "modeling",
    lexModule: "app/src/Subsystems.lex.tex",
    holoQueries: ["scene:acoustics", "kernel:9"],
    refusalQueries: [],
    studioRoles: ["audio", "director"],
    taxon: [
      `Modes ${PRISM_MODAL_BAR_NUMERS.join(":")}/${PRISM_MODAL_BAR_DENOM}`,
      `Default RT60 ${PRISM_CHAMBER_SABINE_RT60_MS}ms`,
      `${PRISM_AUDIO_QUANTUM_FRAMES}-Frame Quantum`,
      "4-Ch Hadamard FDN",
    ],
    invariant:
      "Synthesizes 5-mode physical bar harmonics diffused through a 4-channel Hadamard FDN with Sabine room RT60 and Woodworth interaural time-difference panning.",
  },

  // --- CAT-NARRATIVE: Narrative, Choreography & Portals ---
  {
    id: "STD-TROPICAL-NARRATIVE",
    title: "Deadlock-Free Tropical StoryAutomaton & ConservationLedger",
    authority: "Finite-State Reachability & Double-Entry Conservation",
    claimLevel: "Built & Deadlock-Proved",
    domain: "CAT-NARRATIVE",
    lifecycle: "modeling",
    lexModule: "app/src/Features.lex.tex",
    holoQueries: ["scene:narrative", "kernel:13"],
    refusalQueries: [],
    studioRoles: ["narrative", "gameplay"],
    taxon: ["StoryAutomaton", "ConservationLedger", "Zero Dead-End States", "Conserved Relic Supply"],
    invariant:
      "Proves every authored narrative beat retains a reachable path over tropical (min, +) and enforces exact integer conservation (`before + minted - burned == after`).",
  },
  {
    id: "STD-CAMERA-CHOREOGRAPHY",
    title: "Lawful Near-Sphere Camera Guard & Multi-Shot Choreography",
    authority: "Optical Frustum Clearance (>= 290 mm) & Keyframe Sequencing",
    claimLevel: "Built Cinematography Contract",
    domain: "CAT-NARRATIVE",
    lifecycle: "modeling",
    lexModule: "app/src/Features.lex.tex",
    holoQueries: ["scene:studio", "scene:node:0", "scene:node:1"],
    refusalQueries: [],
    studioRoles: ["director", "design"],
    taxon: ["Near-Sphere Guard (>= 290mm)", "Orbit / 3rd IK / 1st Eye", "Shot Keyframe", "Head-Mask Culling"],
    invariant:
      "Guarantees camera rigs never clip into walls or character geometry while supporting multi-shot keyframe choreography and automated sequence cuts.",
  },
  {
    id: "STD-UNIVERSE-PORTALS",
    title: "128m Floating-Origin UniversePosition & 3-Sector Portal Ring",
    authority: "Cell-Relative Spatial Partitioning (52-bit Safe Horizon)",
    claimLevel: "Built Spatial Continuity Model",
    domain: "CAT-NARRATIVE",
    lifecycle: "modeling",
    lexModule: "app/src/Features.lex.tex",
    holoQueries: ["scene:universe", "scene:stability"],
    refusalQueries: [],
    studioRoles: ["narrative", "code"],
    taxon: ["128m Cell Lattice", "UniversePosition", "Seamless Portal Ring", "Precision Horizon Guard"],
    invariant:
      "Eliminates IEEE-754 far-from-origin vertex jitter via integer cell coordinates + local sub-cell offsets while refusing out-of-horizon coordinates.",
  },

  // --- CAT-REASONING: Cybernetic Observer & Local Reasoning ---
  {
    id: "STD-CYBERNETIC-OBSERVER",
    title: "Second-Order Cybernetic Observer & W5H Frame Governor",
    authority: "Second-Order Cybernetics (von Foerster) · W5H Causal Lattice",
    claimLevel: "Built Closed-Loop Governor",
    domain: "CAT-REASONING",
    lifecycle: "modeling",
    lexModule: "app/src/Features.lex.tex",
    holoQueries: ["scene:telos", "scene:cybernetics", "scene:aqm"],
    refusalQueries: [],
    studioRoles: ["dashboard", "gameplay", "director"],
    taxon: ["W5H 8/8 Strata", "OntologicalRuntimeGovernor", "16.67ms Frame Telos", "AQM Bit-Plane ECS"],
    invariant:
      "Couples observer, scene, roles, and players in a closed 16.67 ms feedback loop that adapts render scale monotonically without breaking physical invariants.",
  },
  {
    id: "STD-Q15-LIF-SALIENCE",
    title: "Q15 Leaky Integrate-and-Fire (LIF) & SSM Mutual Attention",
    authority: "Deterministic Fixed-Point Neuromorphic Recurrence",
    claimLevel: "Built Neuromorphic Kernel",
    domain: "CAT-REASONING",
    lifecycle: "modeling",
    lexModule: "app/src/Subsystems.lex.tex",
    holoQueries: ["kernel:10", "scene:cybernetics"],
    refusalQueries: [],
    studioRoles: ["gameplay", "director"],
    taxon: ["Q15 Fixed-Point", "8-Neuron Spike Mask", "Gaze Dwell Salience", "Mutual Attention"],
    invariant:
      "Computes deterministic integer spike rasters and NPC/camera mutual-attention salience in WebAssembly without floating-point drift.",
  },
  {
    id: "STD-CODED-SLM-DIRECTOR",
    title: "CodedSlmDirector Local Zero-Decode Intent Harness",
    authority: "uor-matmul CodedZeroDecode (`kernel:11`)",
    claimLevel: "Built Local Reasoning Router",
    domain: "CAT-REASONING",
    lifecycle: "development",
    lexModule: "app/src/Features.lex.tex",
    holoQueries: ["kernel:11", "scene:studio"],
    refusalQueries: [""],
    studioRoles: ["director", "code"],
    taxon: ["Zero-Decode Intent", "Local Wasm Execution", "Zero Cloud API Keys", "Empty-Prompt Refusal"],
    invariant:
      "Routes natural-language scene and workspace intents locally inside `verdant-uorc.wasm` via coded matrix contraction, refusing empty or ungrounded inputs.",
  },

  // --- CAT-POLICY: Compliance, Privacy, Quality & Custody ---
  {
    id: "STD-ISO-25010-15939",
    title: "Product Quality Model & Measurement Information Harness",
    authority: "ISO/IEC 25010:2023 & ISO/IEC/IEEE 15939",
    claimLevel: "Imported Quality Standards -> Plumb-Line Gates",
    domain: "CAT-POLICY",
    lifecycle: "validation",
    lexModule: "app/src/Quality.lex.tex",
    holoQueries: ["scene:perf", "scene:stability"],
    refusalQueries: [],
    studioRoles: ["dashboard", "code"],
    taxon: ["Performance Efficiency", "Reliability", "Maintainability", "120 KB Cold-Boot Gate"],
    invariant:
      "Binds cold-boot wire weight (<= 120 KB), 0 B/frame static VBO steady state, and planted-defect verification gates C1–C10 to measurable indicators.",
  },
  {
    id: "STD-ISO-27005-PROVO",
    title: "Security Risk Controls & W3C PROV-O Lineage Discipline",
    authority: "ISO/IEC 27005:2022 · ISO/IEC 27034-1 · W3C PROV-O",
    claimLevel: "Imported Governance -> Enforced Provenance",
    domain: "CAT-POLICY",
    lifecycle: "policy",
    lexModule: "app/src/Security.lex.tex",
    holoQueries: ["summary", "scene:wasmbridge"],
    refusalQueries: ["unknown"],
    studioRoles: ["dashboard", "code"],
    taxon: ["PROV-O Entity/Activity/Agent", "CSP Hash Pinning", "Path-Traversal Refusal", "Supply-Chain Lock"],
    invariant:
      "Separates imported facts from built proofs (Rule R2), pins SHA-256 CSP style hashes, and isolates untrusted assets inside linear-memory Wasm sandboxes.",
  },
  {
    id: "STD-FK07-PRIVACY-ORGANS",
    title: "FK-07 11-Organ Explicit Denial-Wins Capability & Privacy Lattice",
    authority: "Foundry Container Custody Policy FK-07",
    claimLevel: "Built & Monotonicity-Proved",
    domain: "CAT-POLICY",
    lifecycle: "policy",
    lexModule: "app/src/Compliance.lex.tex",
    holoQueries: ["scene:runtime"],
    refusalQueries: [],
    studioRoles: ["dashboard", "audio", "gameplay"],
    taxon: ["11 Browser Organs", "Explicit Denial Wins", "ContainerOrganDenied", "Zero Silent Escalation"],
    invariant:
      "Governs AudioWorklet, OPFS, Gamepad, Orientation, WebRTC, and WebNN capabilities so an explicit user Denial cannot be overridden by subsequent Grants.",
  },
  {
    id: "STD-PRISM-ID-STITCH",
    title: `Sovereign User Identity, ${PRISM_WORKER_RING_CAPACITY}-Frame Rollback & 2-of-3 Stitch Custody`,
    authority: "prism/identity/4 (WebCrypto P-256) · afflom/stitch",
    claimLevel: "Built Sovereign User Instance Protocol",
    domain: "CAT-POLICY",
    lifecycle: "instances",
    lexModule: "app/src/Features.lex.tex",
    holoQueries: ["scene:stitch", "scene:rollback", "kernel:14"],
    refusalQueries: [],
    studioRoles: ["gameplay", "dashboard"],
    taxon: ["Zero-Signin ECDSA P-256", "Non-Extractable Key", "2-of-3 XOR Parity Shards", "1-Bit Tamper Refusal"],
    invariant:
      "Authenticates multiplayer user instances with non-extractable browser keys (zero central accounts) and recovers world snapshots from any 2 of 3 signed Stitch shards.",
  },
  {
    id: "STD-SCHEMA-ORG-COUPLING",
    title: "Schema.org v29.4 Entity, Place, Character, Item & CommunicateAction Coupling",
    authority: "Schema.org v29.4 (CC BY-SA 3.0) · SHA-256 Oracle",
    claimLevel: "Imported Authority -> Oracle-Verified JSON-LD",
    domain: "CAT-LANG",
    lifecycle: "modeling",
    lexModule: "app/src/Observer.lex.tex",
    holoQueries: ["scene:observer", "scene:channels"],
    refusalQueries: ["unknown"],
    studioRoles: ["narrative", "design", "code", "dashboard"],
    taxon: ["VideoGame", "Place", "Person", "CommunicateAction", "@context JSON-LD"],
    invariant:
      "Couples worlds, places, characters, conserved items, and communications against the SHA-256-locked Schema.org vocabulary with typed refusal on unregistered classes or properties.",
  },
  {
    id: "STD-W3C-SOSA-PROVO",
    title: "W3C SOSA Sensory Observations & W3C PROV-O Communication Provenance",
    authority: "W3C SOSA/SSN (TR/vocab-ssn-20171019) · W3C PROV-O",
    claimLevel: "Imported Standard -> Built Turtle Shapes",
    domain: "CAT-REASONING",
    lifecycle: "validation",
    lexModule: "app/src/Observer.lex.tex",
    holoQueries: ["scene:observer", "scene:channels"],
    refusalQueries: [],
    studioRoles: ["director", "narrative", "dashboard"],
    taxon: ["sosa:Observation", "sosa:madeBySensor", "prov:wasGeneratedBy", "prov:wasAttributedTo"],
    invariant:
      "Validates every Observer sensory sample and inter-agent communication against W3C SOSA and W3C PROV-O Turtle shape constraints with distinct SHA-256 kappa digests.",
  },
  {
    id: "STD-KHRONOS-HD-PBR-SKIN",
    title: "Khronos glTF 2.0 Ratified HD PBR Extensions, 22-Joint Skinning & Naga WebGL2",
    authority: "Khronos glTF 2.0 · KHR_materials_* · W3C WGSL -> GLSL ES 3.00",
    claimLevel: "Imported Standard -> gltf + naga Oracle Verified",
    domain: "CAT-WORLD",
    lifecycle: "release",
    lexModule: "app/src/Subsystems.lex.tex",
    holoQueries: ["summary:hd", "scene:character", "kernel:1"],
    refusalQueries: ["kernel:zerodim"],
    studioRoles: ["design", "code", "director"],
    taxon: ["KHR_materials_ior", "KHR_materials_clearcoat", "KHR_materials_transmission", "22-Joint JOINTS_0/WEIGHTS_0"],
    invariant:
      "Emits binary .glb scenes with 5 ratified Khronos HD PBR extensions and 22-joint inverseBindMatrices verified by gltf::Glb, and transpiles WGSL to #version 300 es via naga.",
  },
  {
    id: "STD-ISO-24617-DIALOGUE",
    title: "ISO 24617-2 (DiAML) Grounded Multilingual NPC Dialogue & Unknown Abstention",
    authority: "ISO 24617-2:2012 · VerdantLanguageEngine (uor-matmul)",
    claimLevel: "Imported Standard -> Native Tensor Language Runtime",
    domain: "CAT-REASONING",
    lifecycle: "modeling",
    lexModule: "app/src/Observer.lex.tex",
    holoQueries: ["scene:language", "kernel:11"],
    refusalQueries: ["kernel:underflow"],
    studioRoles: ["narrative", "gameplay", "code"],
    taxon: ["8 DiAML Acts", "BCP-47 en/es/fr/ja", "PROV-O Passage Grounding", "Unknown Abstention"],
    invariant:
      "Classifies dialogue acts exclusively inside Verdant's uor-matmul runtime, cites exact cataloged knowledge fact IDs on grounded answers, and abstains with Unknown when evidence == 0.",
  },
  {
    id: "STD-OMG-DMN-ADMISSION",
    title: "OMG DMN 1.5 Arbitrary Prism Model Extrapolation & Runtime Exclusivity",
    authority: "OMG DMN 1.5 · Verdant Sole Model Runtime Policy",
    claimLevel: "Imported Standard -> Built Admission Gate",
    domain: "CAT-TENSOR",
    lifecycle: "development",
    lexModule: "app/src/Observer.lex.tex",
    holoQueries: ["scene:observer", "scene:stakeholders"],
    refusalQueries: ["kernel:zerodim"],
    studioRoles: ["code", "dashboard"],
    taxon: ["HitPolicy::Unique", "HitPolicy::First", "HitPolicy::Priority", "Zero External Runtimes"],
    invariant:
      "Extrapolates arbitrary Prism models over Verdant's 5 semirings and DMN 1.5 hit policies while mechanically prohibiting any non-Verdant ML, LLM, or 3D runtime.",
  },
  {
    id: "STD-OBSERVER-HOLON",
    title: "Second-Order Observer Holon, 7 Role Models, 4 World Packs & 5-Channel Bus",
    authority: "UOR Prism Observer.lex.tex · Basis Prime Product 2310",
    claimLevel: "Built Constructive Proof (observerHolonAndStakeholdersVerified)",
    domain: "CAT-ARCH",
    lifecycle: "instances",
    lexModule: "app/src/Observer.lex.tex",
    holoQueries: ["scene:observer", "scene:stakeholders", "scene:language", "scene:channels"],
    refusalQueries: ["unknown"],
    studioRoles: ["dashboard", "code", "design", "audio", "narrative", "director", "gameplay"],
    taxon: ["5 Organ Basis Primes", "7 Role Workspaces", "4 Catalog World Packs", "5 Observed Channels"],
    invariant:
      "Unifies the Observer, its 5 platform organs, 7 workspace role models, 4 SHA-256-locked catalog world packs, and all 5 player/agent/organ communication channels as verified Prism models.",
  },
]);

export function parseConformantKeyValues(rawVerdict) {
  if (typeof rawVerdict !== "string" || !rawVerdict.startsWith("conformant:")) return null;
  const out = {};
  for (const seg of rawVerdict.slice("conformant:".length).split(",")) {
    const eq = seg.indexOf("=");
    if (eq > 0) out[seg.slice(0, eq)] = seg.slice(eq + 1);
  }
  return out;
}

export function filterCatalogModels({
  lifecycle = "all",
  domain = "all",
  role = "all",
  search = "",
} = {}) {
  const needle = String(search || "").trim().toLowerCase();
  return PRISM_MODEL_CATALOGS.filter((entry) => {
    if (lifecycle !== "all" && entry.lifecycle !== lifecycle) return false;
    if (domain !== "all" && entry.domain !== domain) return false;
    if (role !== "all" && !entry.studioRoles.includes(role)) return false;
    if (!needle) return true;
    const haystack = [
      entry.id,
      entry.title,
      entry.authority,
      entry.domain,
      entry.lifecycle,
      entry.lexModule,
      entry.invariant,
      ...entry.taxon,
      ...entry.holoQueries,
      ...entry.studioRoles,
    ]
      .join(" ")
      .toLowerCase();
    return haystack.includes(needle);
  });
}

export function catalogModelsForRole(roleKey) {
  return PRISM_MODEL_CATALOGS.filter((entry) => entry.studioRoles.includes(roleKey));
}

export function summarizeCatalogCoverage() {
  const uniquePositiveQueries = new Set();
  const uniqueRefusalQueries = new Set();
  const uniqueLexModules = new Set();
  for (const item of PRISM_MODEL_CATALOGS) {
    uniqueLexModules.add(item.lexModule);
    for (const q of item.holoQueries) uniquePositiveQueries.add(q);
    for (const rq of item.refusalQueries) uniqueRefusalQueries.add(rq);
  }
  return {
    totalModels: PRISM_MODEL_CATALOGS.length,
    domainCount: CATALOG_DOMAINS.length - 1,
    lifecycleCount: LIFECYCLE_STAGES.length - 1,
    organCount: VERDANT_PLATFORM_ORGANS.length,
    uniqueLexModules: uniqueLexModules.size,
    coveredPositiveQueries: uniquePositiveQueries.size,
    coveredRefusalQueries: uniqueRefusalQueries.size,
  };
}
