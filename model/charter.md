# Verdant Engine Charter

Verdant Engine is a universal commercial 3D browser, WebAssembly, and native game engine mechanized end-to-end with Prism. Instead of relying on conditional browser GPU APIs, Verdant Engine hosts exact Laurent and tropical tensor contractions (`uor-matmul`), portable 16x16 software tile PBR rasterization, and progressive multi-LOD high-resolution streaming (`uorc`) alongside archetype ECS, surface-area-heuristic BVH culling, binary glTF 2.0 export, portable WGSL shader validation, symplectic physics, GJK collision manifolds, NavMesh pathfinding, spatial modal audio, and SHA-256 binary world snapshots.

## Progressive UORC Streaming Levels of Detail (`LOD0 -> LOD3`)

- Lod0CoarseProxy: Initial UORC coarse proxy frame streaming low-poly collision hulls, heightfield bounds, and ambient sky.
- Lod1MediumDetail: Progressive UORC medium-detail frame refining terrain subdivisions, rigid-body colliders, and directional shadow splits.
- Lod2HighDetail: Progressive UORC high-detail frame streaming skeletal skinning weights, specular IBL mip chains, and spatial audio emitters.
- Lod3LosslessSource: Terminal UORC lossless source frame delivering bit-exact full-resolution geometry, PBR materials, and SHA-256 chain verification.

## Portable Prism-Hosted Tensor Kernels (`uor-matmul`)

- StaticMeshTransformBatch: Exact Laurent 4x4 homogeneous world-view-projection vertex batch contraction via uor-matmul.
- LinearBlendSkinningBatch: Normalized four-joint skeletal skinning and morph target matrix contraction without intermediate rounding.
- TropicalBvhVisibilityCull: Tropical max-plus semiring bounding-volume frustum and occlusion distance culling across scene hierarchies.
- TileFramebufferRasterizer: Portable 16x16 software tile depth-buffered PBR rasterizer hosting deterministic rendering without conditional browser GPU APIs.
- ShadowCascadeProjector: Four-cascade directional shadow frustum projection and GGX roughness mip chain evaluation.
- ModalAudioMixMatrix: Multi-emitter modal bar overtone synthesis, Woodworth stereo panning, and Sabine RT60 reverberation contraction.
- KinematicSplineController: Arc-length parameterized spline trajectory evaluation, GJK convex contact impulse solve, and A-star navmesh steering.
- ZeroExtentContraction: Invalid tensor contraction with zero inner extent refused before execution.
- NarrowAccumulatorContraction: Invalid tensor contraction whose accumulator bit-width falls below exact Laurent overflow bounds.

## Architecture Standards, Stakeholders, Concerns, and Components

- Standards: ISO-25010-2023 (2023), ISO-27005-2022 (2022), ISO-27034-1-2011 (2011), ISO-42010-2022 (2022).
- Game developer: Builds 3D games and interactive worlds on verdant-core with deterministic ECS, rigid-body physics, spatial audio, and starter templates.
- Technical artist: Authors procedural terrain, skeletal rigs, PBR materials, portable WGSL shaders, and glTF 2.0 scene exports that validate cleanly.
- Engine systems engineer: Maintains exact uor-matmul tensor kernels, progressive UORC streaming frames, frame-budget job graphs, and zero-constant parametric simulation.
- Browser player: Explores responsive 60fps 3D scenes in the browser across desktop and mobile viewports with keyboard, pointer, and spatial audio controls.
- Security auditor: Verifies shader safety, UORC stream and snapshot integrity, strict Content-Security-Policy isolation, and Core-Wasm archive provenance.

### Architectural Concerns

- Frame-time determinism and 60fps budget: Simulation, culling, portable tensor rendering, and audio synthesis execute within bounded frame budgets across browser worker and viewport pipelines.
- Graphics and asset standards compliance: Exported binary glTF 2.0 scenes, portable WGSL shaders, and UORC archives pass authoritative Khronos gltf, W3C naga, and UOR validators.
- Tensor determinism and progressive UORC streaming: Every uor-matmul tensor contraction accumulates without intermediate rounding and UORC scene frames stream monotonically up to lossless source fidelity.
- Commercial developer onboarding: Developers and agents can adopt the engine through runnable examples, a browser starter template, Diataxis documentation, and llms.txt.
- Accessible and clutter-free studio viewport: The browser Engine Studio keeps the 3D viewport front-and-center while meeting WCAG 2.2 AA contrast and coarse touch-target standards.
- Sandbox and snapshot integrity: World snapshots and UORC streams carry SHA-256 digests, shaders validate before compilation, and the Wasm verifier runs in an isolated worker.

### System Components

- Verdant Core Engine: crates/verdant-core: standalone Rust and WebAssembly 3D game engine library providing uor-matmul tensor kernels, UORC streaming, ECS, BVH, glTF 2.0, WGSL, physics, spatial audio, and snapshots.
- Engine CLI Examples and Starter: Runnable examples (tensor_uorc_showcase, custom_game_arena, headless_benchmark) and templates/browser-3d-starter for custom 3D games.
- Verdant Engine Studio and Docs: Interactive 3D browser studio, progressive UORC LOD scrubber, glTF exporter, Diataxis documentation pages, Markdown twins, and llms.txt.
- Engine Verifier Guest: The Core-Wasm guest compiled from Engine.lex.tex that verifies progressive UORC streaming LODs and uor-matmul tensor kernels inside a Web Worker.
- Browser Viewport and WebGL2 Host: The visitor's universal browser canvas runtime presenting portable tensor framebuffers, depth-buffered WebGL2 3D draw calls, and pointer/keyboard input.
- Browser WebAudio Host: The browser audio graph rendering spatialized modal bar harmonics and room reverberation.
- GitHub Pages: Publishes the verified Engine Studio, documentation suite, and signed holo archive from CI.
- Agent Client: An automated coding agent reading llms.txt and Markdown documentation twins to build 3D applications on Verdant Engine.

## Design Views

- Streaming a 3D scene across progressive UORC LODs (Architecture::viewpointSystemsTensorStreaming).
- Components of Verdant Engine (Architecture::viewpointDeveloperOnboarding).
- Executing a verified browser frame (Architecture::viewpointSystemsPerformance).
- States of the engine verifier guest (Architecture::viewpointAuditorIntegrity).

## Features and Stakeholder Journeys

- A developer or player streams multi-LOD 3D scenes progressively via UORC from coarse collision proxies up to lossless SHA-256 chain-verified source resolution. (Tensor::streamingLods, Tensor::streamingLodsStrictlyMonotonic)
- Systems engineers execute exact Laurent and tropical uor-matmul tensor contractions and portable 16x16 software tile PBR rasterization without conditional browser GPU APIs. (Tensor::tensorKernels, Tensor::allSevenTensorKernelsValid)
- Game developers spawn entities, hierarchical transforms, and PBR materials into an archetype ECS indexed by a surface-area-heuristic BVH for raycasts and frustum culling. (verdant_core::ecs::SceneWorld, VE-03)
- Technical artists generate procedural terrain, skeletal rigs, morph targets, binary glTF 2.0 scenes, and portable WGSL shaders validated by Khronos gltf and W3C naga. (verdant_core::render::export_scene_glb, VE-04, VE-05)
- Symplectic Euler rigid bodies, player capsule controllers, and arc-length kinematic spline movers step deterministically over terrain heightfields. (verdant_core::physics::PhysicsWorld, VE-06)
- Spatial audio listeners render stereo modal bar harmonics and environmental soundscapes with inverse-distance attenuation and Sabine RT60 reverberation. (verdant_core::audio::AudioScene, VE-07)
- Systems engineers schedule acyclic worker job graphs within 60fps frame budgets and capture bit-exact IEEE-754 world snapshots with SHA-256 trailers. (verdant_core::runtime::WorldSnapshot, VE-08, VE-09)
- Developers, players, and coding agents explore the interactive 3D Engine Studio, browser starter template, Diataxis documentation, and llms.txt. (Engine::dispatch, VE-10)

## Product Quality Model (ISO/IEC 25010:2023)

- Functional suitability: Verdant Engine executes exact uor-matmul tensor contractions, streams progressive UORC scenes, and exports standards-valid 3D assets.
- Interaction capability: Developers, players, and coding agents operate the 3D studio and documentation cleanly across devices.
- Maintainability: Every engine kernel is purely parametric with zero unexplained magic constants.
- Performance efficiency: Simulation, portable tensor rendering, UORC streaming, and Wasm verification execute within bounded frame and byte budgets.
- Subcharacteristics: Functional suitability: Functional completeness. Functional suitability: Functional correctness. Interaction capability: Inclusivity. Interaction capability: Learnability. Maintainability: Modifiability. Performance efficiency: Time behaviour.
- AccessibleStudio: Every studio and documentation page meets WCAG 2.2 AA and scores 100 in mobile Lighthouse accessibility.
- ValidTensorKernels: Every declared uor-matmul and software tile rasterizer kernel satisfies accumulator overflow bounds and tile alignment.
- MonotonicUorcStreaming: Progressive UORC scene streaming advances strictly monotonically from coarse proxy to lossless source resolution.
- StakeholderJourneys: Every declared engine stakeholder has at least one end-to-end journey through the product.
- ZeroConstantsKernel: Shipped engine Rust sources and constructive domain modules contain zero unexplained magic constants.
- BoundedFrameAndVerifier: The Core-Wasm engine verifier bounds requests to at most 64 bytes before evaluation.
- AccessibilityScore: Lighthouse mobile accessibility score of the Engine Studio root page.
- InvalidTensorKernels: Count of declared engine tensor kernels failing accumulator bit-width or tile-alignment bounds.
- DisorderedStreamingLods: Count of UORC progressive LOD transitions violating strict resolution or work monotonicity.
- JourneysPerStakeholder: Minimum number of journeys declared for any engine stakeholder.
- MagicConstantViolations: Count of magic numeric literals detected by the permanent parametric ratchet.
- VerifierRequestCapBytes: Maximum byte length accepted by the Core-Wasm engine verifier.

## Application Security and Threat Model (ISO/IEC 27005 & ISO/IEC 27034)

- Verdant Engine security controls (0.1.0, 2026-11-15T00:00:00Z): Verdant Engine runs wholly client-side across universal browser viewports, WebAudio, and an isolated Core-Wasm Web Worker, validating portable WGSL shaders, binary glTF assets, UORC streams, and SHA-256 world snapshots.
- Security::assetVerdantCoreRuntime — Verdant Core runtime: The uor-matmul tensor, UORC streaming, ECS, physics, shader, glTF 2.0, and binary world-snapshot kernels in crates/verdant-core.
- Security::assetStudioAndDocsPages — Studio and documentation pages: The interactive 3D browser Engine Studio, Diataxis documentation pages, Markdown twins, and llms.txt.
- Security::assetVerifierGuestArchive — Engine verifier holo: The signed Core-Wasm guest archive verified inside the browser worker before dispatch.
- Security::assetPublishedPagesArtifact — Published Pages bundle: The static Engine Studio and holo bundle deployed by the GitHub Pages workflow.
- Security::threatInvalidShaderCompilation — Invalid or unsafe WGSL shader: A malformed or out-of-bounds portable shader source reaches compilation without front-end type and binding validation.
- Security::threatSnapshotTampering — Corrupted UORC stream or world snapshot: A truncated or modified UORC multi-LOD stream or binary world snapshot alters simulation state or entity transforms silently.
- Security::threatMalformedGlbExport — Non-conformant glTF 2.0 buffer layout: A degenerate mesh or misaligned buffer produces a binary glTF asset that downstream DCC tools reject or misread.
- Security::threatVerifierQueryExhaustion — Oversized verifier query: An unbounded byte payload sent to the Core-Wasm verifier exhausts guest memory or worker CPU budget.
- Security::threatStudioScriptInjection — Inline style or script injection: Unescaped markup or inline styles bypass Content-Security-Policy hashes in the browser Engine Studio.
- Security::threatHostSubstitution — Substituted static host: An untrusted server serves modified HTML in place of the repository's GitHub Pages deployment.
- Security::controlNagaShaderOracle — W3C naga WGSL validator: Every portable WGSL shader module is parsed and type-checked by naga::valid::Validator before acceptance. (cargo test -p repo-conformance engine_ve_05)
- Security::controlSnapshotSha256Trailer — Authenticated SHA-256 snapshot and UORC chain verifier: Every WorldSnapshot and progressive UORC frame verifies magic bytes, exact framing, and SHA-256 digests before decoding. (cargo test -p repo-conformance engine_ve_09)
- Security::controlKhronosGltfOracle — Khronos gltf 2.0 binary validator: Exported GLB payloads refuse empty geometry and pass gltf::Glb and gltf::import buffer validation. (cargo test -p repo-conformance engine_ve_04)
- Security::controlVerifierRequestCap — Bounded 64-byte Core-Wasm request cap: The Core-Wasm guest enforces requestMaximum := 64 and refuses unknown or empty queries deterministically. (prismpm --project app verify acceptance vectors)
- Security::controlStrictHashedCsp — Hash-locked Content-Security-Policy and worker verification: bootstrap/page.mjs hashes all styles and scripts into CSP with zero inline style mutations, and guest-worker.mjs verifies app.holo off the main thread. (cargo test -p repo-conformance frame_fr_01)
- Security::threatHostSubstitution rationale: A static browser bundle cannot prevent substitution of its own HTTP origin; the holo archive carries verifiable cryptographic provenance and the repository rebuilds deterministically from source.

## Compliance Obligations

- ISO/IEC/IEEE 42010:2022 — Architecture description: Stakeholders, concerns, components, edges, viewpoints, and views of Verdant Engine resolve cleanly. (Architecture::architectureDescriptionHolds)
- ISO/PAS 19450:2015 and OMG UML 2.5.1 — Design views: OPM scene-streaming and UML component, sequence, and state-machine views conform to the architecture model. (Design::viewOpmSceneStreaming, Design::viewUmlComponents, Design::viewUmlFrameSequence, Design::viewUmlVerifierStates)
- ISO/IEC 25010:2023 — Product quality: Characteristics, subcharacteristics, requirements, and thresholds are numbered and verified by the Lean kernel. (Quality::qualityModelHolds)
- ISO/IEC 27034-1:2011 and ISO/IEC 27005:2022 — Application security and risk: Every browser and engine runtime threat maps to an evidenced security control or explicit accepted risk. (Security::controlSet)
- Khronos glTF 2.0 and W3C WGSL — 3D asset and shader standards: Binary glTF 2.0 exports and portable WGSL shaders are validated mechanically by authoritative gltf and naga crates. (VE-04, VE-05)
- UORC-SHA256-CHAIN-v1 and IEEE 754-2019 — Progressive UORC streaming and exact tensor contractions: All four progressive UORC LOD frames verify SHA-256 chain integrity and all seven uor-matmul tensor kernels satisfy exact accumulator bounds. (Tensor::streamingLodsStrictlyMonotonic, Tensor::allSevenTensorKernelsValid)
- WCAG 2.2 AA and W3C DTCG 2025.10 — Accessible studio and design tokens: Design tokens and surface contrast ratios satisfy WCAG 2.2 SC 1.4.3 and SC 1.4.11 and pass mobile Lighthouse checks. (Brand::contrasts, Brand::tokens)
- ISO 9241-11:2018 and ISO 24495-1:2023 — Context of use and plain-language documentation: User groups, goals, tasks, Diataxis documentation, Markdown twins, and llms.txt pass mechanical content checks. (Users::goals)

## Core-Wasm Verifier Contract

- prismpm/text-application/1 — Verdant Engine (verdant-engine-guest, 0.1.0): Verifies Verdant Engine progressive UORC streaming LODs and exact uor-matmul tensor kernels in WebAssembly. Repository: https://github.com/afflom/verdant-engine, Homepage: https://afflom.github.io/verdant-engine/, Entry: VerdantEngine.Engine.dispatch, Contract: hologram:guest/core-wasm@1.
- View: Verdant Engine — Verdant Engine Studio and Verifier, Engine verification query, Verify, Engine verdict, Enter a UTF-8 engine query of at most 64 bytes., The engine verdict could not be displayed.

## Engine Runtime Subsystems

- LinearAlgebraAndFrustum: Vector, quaternion, matrix, ray, AABB, and six-plane Gribb-Hartmann frustum kernels derived without magic constants.
- ArchetypeEcsAndSahBvh: Generational entity handles, hierarchical world-transform propagation, and surface-area-heuristic BVH raycast and frustum culling.
- TensorAndUorcStreaming: Exact Laurent and tropical uor-matmul tensor contractions, portable software tile rasterization, and progressive UORC multi-LOD streaming.
- ProceduralGeometryPrimitives: Heightfield terrain grids, fluted cylindrical columns, deformable ribbon strips, and faceted convex prism meshes.
- PbrMaterialsAndPortableShaders: Metallic-roughness PBR material descriptors, shadow cascades, and W3C WGSL portable shader modules validated by naga.
- BinaryGltfSceneExporter: Aligned binary glTF 2.0 container serializer validated mechanically against the Khronos gltf reference crate.
- SymplecticPhysicsAndSplines: Symplectic Euler rigid bodies, Archimedes fluid buoyancy, player capsule ground clamping, and arc-length kinematic spline movers.
- SpatialModalAcoustics: Euler-Bernoulli modal bar overtone synthesis, Woodworth interaural stereo panning, and Sabine RT60 reverberation.
- JobGraphAndWorldSnapshots: Topological six-stage worker frame scheduler and deterministic IEEE-754 binary world snapshots with SHA-256 trailers.
- SkeletalAndMorphAnimation: Acyclic joint hierarchies, four-joint normalized linear-blend skinning weights, clamped keyframe interpolation, and morph target channels.
- PbrShadowAndIblRenderGraph: Four-cascade logarithmic-uniform shadow frustum splits, GGX specular roughness mip chains, and five-pass acyclic render barrier graphs.
- GjkConvexCollisionAndSolver: Bounded 3D Gilbert-Johnson-Keerthi Minkowski support intersections, Baumgarte position stabilization, and Coulomb friction cones.
- NavMeshCorridorPathfinding: Slope-filtered walkable terrain navmeshes, bounded admissible A-star polygon search, and portal corridor steering.
- BrowserWasmWorkerBridge: Dedicated WebAssembly simulation worker ring framing, 128-frame WebAudio worklet quanta, and bounded URL-safe VDNT snapshot envelopes.
- CliAssetPackerAndOracle: Command-line binary glTF 2.0, WGSL, and UORC stream validator, multi-LOD scene packer, and headless 60fps benchmark.

## Developer Relations, Documentation, SDK Artifacts, and Studio Controls

- DocsIndex: Commercial 3D browser and WebAssembly game engine documentation landing page organizing tutorials, how-to guides, explanations, and API reference.
- First3DSceneTutorial: Step-by-step tutorial creating a SceneWorld, spawning procedural meshes, stepping physics, and exporting binary glTF 2.0.
- CustomGameHowTo: Task-focused guide building a custom 3D browser game from templates/browser-3d-starter with capsule controllers and rigid bodies.
- AssetShaderPipelineHowTo: Technical artist guide packing progressive UORC scenes, binary glTF 2.0 exports, and portable WGSL shaders validated by Khronos gltf and naga.
- TensorAndUorcArchitectureExplanation: Architectural explanation of exact Laurent and tropical uor-matmul tensor kernels, portable tile rasterization, and progressive UORC scene streaming.
- VerdantCoreApiReference: Complete technical reference for verdant-core modules, progressive UORC framing, binary world snapshots, and Core-Wasm verifier queries.
- ExampleTensorUorcShowcase: Runnable example crates/verdant-core/examples/tensor_uorc_showcase.rs executing uor-matmul tensor contractions, progressive UORC streaming, and binary glTF 2.0 export.
- ExampleCustomGameArena: Runnable example crates/verdant-core/examples/custom_game_arena.rs stepping a custom physics arena with capsule controller and spatial audio.
- ExampleHeadlessBenchmark: Runnable example crates/verdant-core/examples/headless_benchmark.rs executing the six-stage worker job graph and SHA-256 snapshot round-trip.
- TemplateBrowser3DStarter: Standalone starter project templates/browser-3d-starter/Cargo.toml for building custom 3D browser and WebAssembly games on verdant-core.
- VerdantWasmRuntime: Dedicated WebAssembly runtime crate crates/verdant-wasm/Cargo.toml driving off-thread simulation, portable tensor rendering, spatial acoustics, and SHA-256 VDNT snapshots.
- VerdantCliPacker: Command-line asset validator, progressive UORC, GLB, and VDNT scene packer, and headless 60fps benchmark crate crates/verdant-cli/Cargo.toml.
- ve-lod-btn: Cycles progressive UORC streaming level of detail from coarse collision proxy LOD0 to lossless source LOD3.
- ve-sim-btn: Toggles between demand-driven stepped inspection and continuous 60fps orbital and spline mover simulation.
- ve-cam-btn: Switches between 3D Orbit camera framing and First-Person WASD capsule walk mode.
- ve-audio-btn: Toggles WebAudio Euler-Bernoulli modal bar harmonics and spatial room reverberation.
- ve-glb-btn: Serializes and downloads the active 3D scene as a standards-valid binary glTF 2.0 (.glb) asset.
- ve-snap-btn: Encodes an authenticated SHA-256 VDNT binary world snapshot and copies a shareable #vdnt= studio URL.
- ve-docs-link: Navigates directly from the Engine Studio bar to the Diataxis documentation suite.

## Brand Identity and Context of Use

- Brand authority: Alina Wheeler, Designing Brand Identity, 5th ed., Wiley, 2017; W3C Design Tokens Format Module 2025.10; WCAG 2.2 — UOR-Foundation/website at commit e82ffdedc5d85ebd0bde239d50d4651005045d31.
- Core brand values and mark: Open by default | Identity from content | No central authority | public/favicon.png.
- Canonical surface palette: fd-bg #0a0a0a, fd-panel #121212, fd-fg #fafafa, fd-dim #999999, fd-line #333333, fd-line-strong #616161, fd-accent #fafafa.
- Canonical typography: Space Grotesk, system-ui, sans-serif; JetBrains Mono, ui-monospace, monospace (tokens fd-head Space Grotesk, system-ui, sans-serif and fd-mono JetBrains Mono, ui-monospace, monospace).
- Canonical spectrum gradient: fd-spectrum 100deg, #fb5661 0, #fa792e 0.07, #f9cf24 0.14, #f6e41e 0.19, #25d05e 0.25, #20d0f3 0.31, #4f81f8 0.38, #ae6ff6 0.45, #f368c5 0.48, #fb5661 0.5, #fa792e 0.57, #f9cf24 0.64, #f6e41e 0.69, #25d05e 0.75, #20d0f3 0.81, #4f81f8 0.88, #ae6ff6 0.95, #f368c5 0.98, #fb5661 1.
- Context-of-use authority: ISO 9241-11:2018 context of use and usability; ISO 9241-210:2019 human-centred design — https://github.com/afflom/verdant-engine at the revision of this model.
- Usability effectiveness method: acceptance rows of the goal's tasks: each is a test that the task's feature completes.
- Usability efficiency method: tasks the goal takes, from its journey's steps.
- Usability satisfaction method: open: no instrument in the model measures it.
