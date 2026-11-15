# Verdant Engine Charter

Verdant Engine is a multi-purpose commercial 3D browser and WebAssembly game engine extracted from Verdant Resonance in the Unreal Engine tradition. It runs 60fps 3D worlds in the browser with procedural terrain, PBR materials, binary glTF 2.0 export, WGSL shaders, symplectic rigid-body physics, spatial audio, deterministic snapshots, and verified causal expansion and structural equilibrium.

## Cosmological Expansion Epochs (`t_0 -> t_obs`)

- Cosmogenic: Initial expansion seed establishing solar irradiance, metric curvature, and elementary elemental abundances.
- LithicHydrologic: Tectonic amphitheater relief, silicate mineral crystallization, and mirror lagoon basin condensation.
- BioticEcological: Phyllotactic L-system flora colonization, organic soil strata, and natural wind-water modal resonance.
- AgenticArchitectural: Present observation t_obs where differentiated Observer-Builder guilds construct sanctuary structures and autonomous kinetic mechanisms.

## Differentiated Builder Guild Blueprints

- HarmonicRotunda: Travertine colonnade rotunda housing a nested Cardan astrolabe gimbal aligned to the sanctuary meridian.
- MeridianSundial: Equatorial bronze-and-basalt gnomon calibrating solar elevation and cosmic scale-factor timekeeping.
- ModalChimePergola: Timber-and-stone acoustic pergola suspending Euler-Bernoulli cylindrical chimes tuned to integer modal ratios.
- QuartzTorsionChamber: Corbelled silicate vault focusing piezoelectric crystal lattice harmonics across the amphitheater floor.
- LagoonPierCauseway: Ashlar stone pier causeway spanning the mirror lagoon margin without disturbing shallow-water wave dispersion.
- GeodesicTravertinePaths: Least-slope travertine walkway network minimizing grade work between the rotunda, lagoon pier, and pergola.
- PerimeterSentinelRover: Autonomous spline-guided kinetic sentinel rover continuously orbiting the garden perimeter to survey ecological and structural telemetry.
- PrematureMonument: Invalid monument placed before Builder differentiation.
- TopplingArch: Invalid arch whose thrust line exits Heyman middle third.

## Architecture Standards, Stakeholders, Concerns, and Components

- Standards: ISO-25010-2023 (2023), ISO-27005-2022 (2022), ISO-27034-1-2011 (2011), ISO-42010-2022 (2022).
- Game developer: Builds 3D games and interactive worlds on verdant-core with deterministic ECS, rigid-body physics, spatial audio, and starter templates.
- Technical artist: Authors procedural terrain, L-system flora, PBR materials, WGSL shaders, and glTF 2.0 scene exports that validate cleanly.
- Engine systems engineer: Maintains frame-budget job graphs, deterministic binary world snapshots, and zero-constant parametric simulation kernels.
- Browser player: Explores responsive 60fps 3D scenes in the browser across desktop and mobile viewports with keyboard, pointer, and spatial audio controls.
- Security auditor: Verifies shader safety, snapshot integrity, strict Content-Security-Policy isolation, and Core-Wasm archive provenance.

### Architectural Concerns

- Frame-time determinism and 60fps budget: Simulation, culling, rendering, and audio synthesis execute within bounded frame budgets across browser worker and GPU pipelines.
- Graphics and asset standards compliance: Exported binary glTF 2.0 scenes and WGSL shaders pass authoritative Khronos gltf and W3C naga validators.
- Causal expansion and structural equilibrium: Every natural formation and Builder-constructed structure emerges monotonically at its valid cosmic epoch in static equilibrium.
- Commercial developer onboarding: Developers and agents can adopt the engine through runnable examples, a browser starter template, Diataxis documentation, and llms.txt.
- Accessible and clutter-free studio viewport: The browser Engine Studio keeps the 3D viewport front-and-center while meeting WCAG 2.2 AA contrast and coarse touch-target standards.
- Sandbox and snapshot integrity: World snapshots carry SHA-256 trailers, shaders validate before GPU compilation, and the Wasm verifier runs in an isolated worker.

### System Components

- Verdant Core Engine: crates/verdant-core: standalone Rust and WebAssembly 3D game engine library providing ECS, BVH, procedural meshes, glTF 2.0, WGSL, physics, spatial audio, and snapshots.
- Engine CLI Examples and Starter: Runnable examples (cosmic_sanctuary, custom_game_arena, headless_benchmark) and templates/browser-3d-starter for custom 3D games.
- Verdant Engine Studio and Docs: Interactive WebGL2 3D browser studio, cosmic expansion scrubber, glTF exporter, Diataxis documentation pages, Markdown twins, and llms.txt.
- Engine Verifier Guest: The Core-Wasm guest compiled from Engine.lex.tex that verifies expansion epochs and Builder blueprint equilibrium inside a Web Worker.
- Browser WebGPU and WebGL2 Host: The visitor's browser graphics runtime executing compiled shaders, depth-buffered 3D draw calls, and pointer/keyboard input.
- Browser WebAudio Host: The browser audio graph rendering spatialized modal chime harmonics and lagoon water acoustics.
- GitHub Pages: Publishes the verified Engine Studio, documentation suite, and signed holo archive from CI.
- Agent Client: An automated coding agent reading llms.txt and Markdown documentation twins to build 3D applications on Verdant Engine.

## Design Views

- Synthesizing a world across expansion epochs (Architecture::viewpointSystemsCausalFidelity).
- Components of Verdant Engine (Architecture::viewpointDeveloperOnboarding).
- Executing a verified browser frame (Architecture::viewpointSystemsPerformance).
- States of the engine verifier guest (Architecture::viewpointAuditorIntegrity).

## Features and Stakeholder Journeys

- A developer or player scrubs cosmic expansion from initial seed to present observation t_obs, watching terrain, water, flora, and Builder artifacts emerge causally. (Genesis::epochs, Genesis::epochsStrictlyMonotonic)
- Five Builder guilds synthesize seven non-natural sanctuary structures and an autonomous perimeter sentinel rover verified for Heyman middle-third equilibrium. (Genesis::blueprints, Genesis::allSevenBlueprintsEquilibrium)
- Game developers spawn entities, hierarchical transforms, and PBR materials into an archetype ECS indexed by a surface-area-heuristic BVH for raycasts and frustum culling. (verdant_core::ecs::SceneWorld, VE-03)
- Technical artists generate procedural terrain, L-system flora, morph targets, binary glTF 2.0 scenes, and WGSL shaders validated by Khronos gltf and W3C naga. (verdant_core::assets::export_scene_glb, VE-04, VE-05)
- Symplectic Euler rigid bodies, player capsule controllers, and autonomous spline-following rovers step deterministically over terrain heightfields. (verdant_core::physics::PhysicsWorld, VE-06)
- Spatial audio listeners render stereo modal chime harmonics and water soundscapes with inverse-distance attenuation and Sabine RT60 reverberation. (verdant_core::audio::AudioScene, VE-07)
- Systems engineers schedule acyclic worker job graphs within 60fps frame budgets and capture bit-exact IEEE-754 world snapshots with SHA-256 trailers. (verdant_core::runtime::WorldSnapshot, VE-08, VE-09)
- Developers, players, and coding agents explore the interactive 3D WebGL2 Engine Studio, browser starter template, Diataxis documentation, and llms.txt. (Engine::dispatch, VE-10)

## Product Quality Model (ISO/IEC 25010:2023)

- Functional suitability: Verdant Engine synthesizes causally ordered worlds, statically balanced structures, and standards-valid 3D assets.
- Interaction capability: Developers, players, and coding agents operate the 3D studio and documentation cleanly across devices.
- Maintainability: Every engine kernel is purely parametric with zero unexplained magic constants.
- Performance efficiency: Simulation, rendering, and Wasm verification execute within bounded frame and byte budgets.
- Subcharacteristics: Functional suitability: Functional completeness. Functional suitability: Functional correctness. Interaction capability: Inclusivity. Interaction capability: Learnability. Maintainability: Modifiability. Performance efficiency: Time behaviour.
- AccessibleStudio: Every studio and documentation page meets WCAG 2.2 AA and scores 100 in mobile Lighthouse accessibility.
- EquilibriumBlueprints: Every constructed sanctuary structure and autonomous rover satisfies Heyman middle-third statics and safety factor bounds.
- MonotonicExpansion: World expansion progresses strictly monotonically from Cosmogenic seed to AgenticArchitectural observation.
- StakeholderJourneys: Every declared engine stakeholder has at least one end-to-end journey through the product.
- ZeroConstantsKernel: Shipped engine Rust sources and constructive domain modules contain zero unexplained magic constants.
- BoundedFrameAndVerifier: The Core-Wasm engine verifier bounds requests to at most 64 bytes before evaluation.
- AccessibilityScore: Lighthouse mobile accessibility score of the Engine Studio root page.
- UnstableBlueprints: Count of declared Builder blueprints failing static equilibrium or epoch gating.
- DisorderedEpochs: Count of cosmological expansion epoch transitions violating strict scale monotonicity.
- JourneysPerStakeholder: Minimum number of journeys declared for any engine stakeholder.
- MagicConstantViolations: Count of magic numeric literals detected by the permanent parametric ratchet.
- VerifierRequestCapBytes: Maximum byte length accepted by the Core-Wasm engine verifier.

## Application Security and Threat Model (ISO/IEC 27005 & ISO/IEC 27034)

- Verdant Engine security controls (0.1.0, 2026-11-14T00:00:00Z): Verdant Engine runs wholly client-side in WebGPU/WebGL2, WebAudio, and an isolated Core-Wasm Web Worker, validating shaders, binary glTF assets, and SHA-256 world snapshots.
- Security::assetVerdantCoreRuntime — Verdant Core runtime: The ECS, physics, shader, glTF 2.0, and binary world-snapshot kernels in crates/verdant-core.
- Security::assetStudioAndDocsPages — Studio and documentation pages: The interactive 3D browser Engine Studio, Diataxis documentation pages, Markdown twins, and llms.txt.
- Security::assetVerifierGuestArchive — Engine verifier holo: The signed Core-Wasm guest archive verified inside the browser worker before dispatch.
- Security::assetPublishedPagesArtifact — Published Pages bundle: The static Engine Studio and holo bundle deployed by the GitHub Pages workflow.
- Security::threatInvalidShaderCompilation — Invalid or unsafe WGSL shader: A malformed or out-of-bounds shader source reaches GPU compilation without front-end type and binding validation.
- Security::threatSnapshotTampering — Corrupted or tampered world snapshot: A truncated or modified binary world snapshot alters simulation state or entity transforms silently.
- Security::threatMalformedGlbExport — Non-conformant glTF 2.0 buffer layout: A degenerate mesh or misaligned buffer produces a binary glTF asset that downstream DCC tools reject or misread.
- Security::threatVerifierQueryExhaustion — Oversized verifier query: An unbounded byte payload sent to the Core-Wasm verifier exhausts guest memory or worker CPU budget.
- Security::threatStudioScriptInjection — Inline style or script injection: Unescaped markup or inline styles bypass Content-Security-Policy hashes in the browser Engine Studio.
- Security::threatHostSubstitution — Substituted static host: An untrusted server serves modified HTML in place of the repository's GitHub Pages deployment.
- Security::controlNagaShaderOracle — W3C naga WGSL validator: Every WGSL shader module is parsed and type-checked by naga::valid::Validator before acceptance. (cargo test -p repo-conformance engine_ve_05)
- Security::controlSnapshotSha256Trailer — Authenticated SHA-256 snapshot trailer: Every WorldSnapshot verifies magic bytes, exact payload length, and a 32-byte SHA-256 digest before decoding. (cargo test -p repo-conformance engine_ve_09)
- Security::controlKhronosGltfOracle — Khronos gltf 2.0 binary validator: Exported GLB payloads refuse empty geometry and pass gltf::Glb and gltf::import buffer validation. (cargo test -p repo-conformance engine_ve_04)
- Security::controlVerifierRequestCap — Bounded 64-byte Core-Wasm request cap: The Core-Wasm guest enforces requestMaximum := 64 and refuses unknown or empty queries deterministically. (prismpm --project app verify acceptance vectors)
- Security::controlStrictHashedCsp — Hash-locked Content-Security-Policy and worker verification: bootstrap/page.mjs hashes all styles and scripts into CSP with zero inline style mutations, and guest-worker.mjs verifies app.holo off the main thread. (cargo test -p repo-conformance frame_fr_01)
- Security::threatHostSubstitution rationale: A static browser bundle cannot prevent substitution of its own HTTP origin; the holo archive carries verifiable cryptographic provenance and the repository rebuilds deterministically from source.

## Compliance Obligations

- ISO/IEC/IEEE 42010:2022 — Architecture description: Stakeholders, concerns, components, edges, viewpoints, and views of Verdant Engine resolve cleanly. (Architecture::architectureDescriptionHolds)
- ISO/PAS 19450:2015 and OMG UML 2.5.1 — Design views: OPM world-genesis and UML component, sequence, and state-machine views conform to the architecture model. (Design::viewOpmWorldGenesis, Design::viewUmlComponents, Design::viewUmlFrameSequence, Design::viewUmlVerifierStates)
- ISO/IEC 25010:2023 — Product quality: Characteristics, subcharacteristics, requirements, and thresholds are numbered and verified by the Lean kernel. (Quality::qualityModelHolds)
- ISO/IEC 27034-1:2011 and ISO/IEC 27005:2022 — Application security and risk: Every browser and engine runtime threat maps to an evidenced security control or explicit accepted risk. (Security::controlSet)
- Khronos glTF 2.0 and W3C WGSL — 3D asset and shader standards: Binary glTF 2.0 exports and WGSL shaders are validated mechanically by authoritative gltf and naga crates. (VE-04, VE-05)
- Heyman Limit Analysis of Masonry Structures — Architectural statics: All seven constructed structures and the autonomous rover satisfy middle-third eccentricity and safety-factor bounds at t_obs. (Genesis::allSevenBlueprintsEquilibrium)
- WCAG 2.2 AA and W3C DTCG 2025.10 — Accessible studio and design tokens: Design tokens and surface contrast ratios satisfy WCAG 2.2 SC 1.4.3 and SC 1.4.11 and pass mobile Lighthouse checks. (Brand::contrasts, Brand::tokens)
- ISO 9241-11:2018 and ISO 24495-1:2023 — Context of use and plain-language documentation: User groups, goals, tasks, Diataxis documentation, Markdown twins, and llms.txt pass mechanical content checks. (Users::goals)

## Core-Wasm Verifier Contract

- prismpm/text-application/1 — Verdant Engine (verdant-engine-guest, 0.1.0): Verifies Verdant Engine expansion epochs and Builder blueprint static equilibrium in WebAssembly. Repository: https://github.com/afflom/verdant-engine, Homepage: https://afflom.github.io/verdant-engine/, Entry: VerdantEngine.Engine.dispatch, Contract: hologram:guest/core-wasm@1.
- View: Verdant Engine — Verdant Engine Studio and Verifier, Engine verification query, Verify, Engine verdict, Enter a UTF-8 engine query of at most 64 bytes., The engine verdict could not be displayed.

## Engine Runtime Subsystems

- LinearAlgebraAndFrustum: Vector, quaternion, matrix, ray, AABB, and six-plane Gribb-Hartmann frustum kernels derived without magic constants.
- ArchetypeEcsAndSahBvh: Generational entity handles, hierarchical world-transform propagation, and surface-area-heuristic BVH raycast and frustum culling.
- CausalWorldGenesis: Deterministic cosmic expansion from t_0 seed to t_obs observation synthesizing amphitheater terrain, flora, and seven Builder blueprints.
- ProceduralGeometryAndLSystems: Heightfield terrain grids, Vitruvian entasis colonnades, Cardan astrolabe gimbals, and phyllotactic L-system botanical meshes.
- PbrMaterialsAndWgslShaders: Metallic-roughness PBR material descriptors, shadow cascades, and W3C WGSL shaders validated by naga.
- BinaryGltfSceneExporter: Aligned binary glTF 2.0 container serializer validated mechanically against the Khronos gltf reference crate.
- SymplecticPhysicsAndRover: Symplectic Euler rigid bodies, Archimedes lagoon buoyancy, player capsule ground clamping, and autonomous spline rover patrol.
- SpatialModalAcoustics: Euler-Bernoulli chime overtone synthesis, Woodworth interaural stereo panning, and Sabine RT60 reverberation.
- JobGraphAndWorldSnapshots: Topological six-stage worker frame scheduler and deterministic IEEE-754 binary world snapshots with SHA-256 trailers.
- SkeletalAndMorphAnimation: Acyclic joint hierarchies, four-joint normalized linear-blend skinning weights, clamped keyframe interpolation, and morph target channels.
- PbrShadowAndIblRenderGraph: Four-cascade logarithmic-uniform shadow frustum splits, GGX specular roughness mip chains, and five-pass acyclic GPU barrier graphs.
- GjkConvexCollisionAndSolver: Bounded 3D Gilbert-Johnson-Keerthi Minkowski support intersections, Baumgarte position stabilization, and Coulomb friction cones.
- NavMeshCorridorPathfinding: Slope-filtered walkable terrain navmeshes, bounded admissible A-star polygon search, and portal corridor steering.
- BrowserWasmWorkerBridge: Dedicated WebAssembly simulation worker ring framing, 128-frame WebAudio worklet quanta, and bounded URL-safe VDNT snapshot envelopes.

## Developer Relations, Documentation, SDK Artifacts, and Studio Controls

- DocsIndex: Commercial 3D browser and WebAssembly game engine documentation landing page organizing tutorials, how-to guides, explanations, and API reference.
- First3DSceneTutorial: Step-by-step tutorial creating a SceneWorld, spawning procedural meshes, stepping physics, and exporting binary glTF 2.0.
- CustomGameHowTo: Task-focused guide building a custom 3D browser game from templates/browser-3d-starter with capsule controllers and rigid bodies.
- AssetShaderPipelineHowTo: Technical artist guide validating binary glTF 2.0 exports and W3C WGSL shaders against Khronos gltf and naga.
- CausalWorldArchitectureExplanation: Architectural explanation of causal cosmic expansion epochs, differentiated Builder guilds, and Heyman middle-third static equilibrium.
- VerdantCoreApiReference: Complete technical reference for verdant-core modules, binary world snapshot framing, and Core-Wasm verifier queries.
- ExampleCosmicSanctuary: Runnable example crates/verdant-core/examples/cosmic_sanctuary.rs synthesizing the t_obs sanctuary and exporting binary glTF 2.0.
- ExampleCustomGameArena: Runnable example crates/verdant-core/examples/custom_game_arena.rs stepping a custom physics arena with capsule controller and spatial audio.
- ExampleHeadlessBenchmark: Runnable example crates/verdant-core/examples/headless_benchmark.rs executing the six-stage worker job graph and SHA-256 snapshot round-trip.
- TemplateBrowser3DStarter: Standalone starter project templates/browser-3d-starter/Cargo.toml for building custom 3D browser and WebAssembly games on verdant-core.
- VerdantWasmRuntime: Dedicated WebAssembly runtime crate crates/verdant-wasm/Cargo.toml driving off-thread simulation, frustum culling, spatial acoustics, and SHA-256 VDNT snapshots.
- ve-epoch-btn: Cycles cosmic expansion epoch from initial Cosmogenic seed to present observation t_obs.
- ve-sim-btn: Toggles between demand-driven stepped inspection and continuous 60fps orbital and rover simulation.
- ve-cam-btn: Switches between 3D Orbit camera framing and First-Person WASD capsule walk mode.
- ve-audio-btn: Toggles WebAudio Euler-Bernoulli modal chime harmonics and lagoon water acoustics.
- ve-glb-btn: Serializes and downloads the active 3D scene as a standards-valid binary glTF 2.0 (.glb) asset.
- ve-snap-btn: Encodes an authenticated SHA-256 VDNT binary world snapshot and copies a shareable #vdnt= studio URL.
- ve-docs-link: Navigates directly from the Engine Studio bar to the Diataxis documentation suite.
