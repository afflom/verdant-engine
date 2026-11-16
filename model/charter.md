# Verdant Engine Charter

Verdant Engine is a universal commercial 3D browser, WebAssembly, and native game engine mechanized end-to-end with Prism. Instead of relying on conditional browser GPU APIs, Verdant Engine hosts exact Laurent and tropical tensor contractions (`uor-matmul`), portable 16x16 software tile PBR rasterization, and progressive multi-LOD high-resolution streaming (`uorc`) alongside archetype ECS, surface-area-heuristic BVH culling, binary glTF 2.0 export, portable WGSL shader validation, symplectic physics, GJK collision manifolds, NavMesh pathfinding, spatial modal audio, and SHA-256 binary world snapshots.

## Progressive UORC and High-Definition Streaming Levels of Detail (`LOD0 -> LOD4`)

- Lod0CoarseProxy: Initial UORC coarse proxy frame streaming low-poly collision hulls, heightfield bounds, and ambient sky.
- Lod1MediumDetail: Progressive UORC medium-detail frame refining terrain subdivisions, rigid-body colliders, and directional shadow splits.
- Lod2HighDetail: Progressive UORC high-detail frame streaming skeletal skinning weights, specular IBL mip chains, and spatial audio emitters.
- Lod3LosslessSource: Terminal UORC lossless source frame delivering bit-exact full-resolution geometry, PBR materials, and SHA-256 chain verification.
- Lod4UltraHighDef: Ultra-high-definition synthesis tier subdividing vaulted Citadel masonry, parabolic arch bridges, and multi-octave cliff terrain past lossless UORC frames.

## Portable Prism-Hosted Tensor Kernels (`uor-matmul`)

- StaticMeshTransformBatch: Exact Laurent 4x4 homogeneous world-view-projection and scene hierarchy matrix contraction via uor-matmul.
- LinearBlendSkinningBatch: Normalized four-joint skeletal skinning and morph target matrix contraction without intermediate rounding.
- TropicalBvhVisibilityCull: Tropical max-plus semiring bounding-volume frustum and GJK directional support contraction across scene hierarchies.
- TileFramebufferRasterizer: Portable 16x16 software tile depth-buffered PBR rasterizer hosting deterministic rendering without conditional browser GPU APIs.
- ShadowCascadeProjector: Four-cascade directional shadow frustum projection, Delassus constraint effective mass, and GGX roughness mip evaluation.
- ModalAudioMixMatrix: Multi-emitter modal FIR convolution, Woodworth stereo panning, and Sabine/Eyring feedback delay network contraction.
- KinematicSplineController: Arc-length parameterized spline trajectory evaluation, GJK convex contact impulse solve, and A-star navmesh steering.
- TropicalMinPlusNavClosure: Tropical min-plus semiring matrix squaring for exact all-pairs and corridor shortest-path closure over walkable navmeshes.
- CodedStreamZeroDecodeGemm: Zero-decode uor_matmul::codec contraction over Grid, Packed, and Runs UORC frames without intermediate float expansion.
- TropicalUiConstraintLayout: Tropical min-plus and exact linear box-tree constraint contraction solving responsive 2D panels, focus reachability, and diegetic 3D anchors.
- NeuromorphicSalienceStateSpace: Exact Q15 leaky-integrate-and-fire state-space recurrence coupling player gaze, locomotion cadence, and world mutual-attention salience.
- CodedSlmProposalRouter: Zero-decode Packed<Grid<16>, 2> and Runs<Grid<16>, 16> embedded language and intent reasoning contraction routing verified world proposals.
- TropicalBlasSweptToi: Tropical min-plus and max-plus triangle BLAS pruning and continuous swept-sphere time-of-impact bound contraction.
- TwoBoneIkJointSolver: Exact linear cosine-law and pole-plane matrix contraction solving analytical two-bone foot and hand inverse kinematics.
- ConservationStoryAutomaton: Exact integer resource conservation balance and tropical min-plus reachability closure verifying deadlock-free narrative and app graphs.
- GridQuantizedGeometry: Fixed-width 16-entry Grid lookup CodedMatrix contracting quantized UORC mesh positions and normals directly into world space.
- PackedNibbleAnimationTracks: Sub-byte 4-bit Packed<Grid<16>, 2> CodedMatrix streaming two animation curve samples per byte into exact pose accumulators.
- SparseRunsIblProbes: Variable-length Runs<Grid<16>, 16> CodedMatrix contracting sparse environment irradiance and shadow cascade residuals.
- InvalidZeroCodeTier: Invalid coded matrix tier with zero code width and narrow accumulator refused before execution.
- ZeroExtentContraction: Invalid tensor contraction with zero inner extent refused before execution.
- NarrowAccumulatorContraction: Invalid tensor contraction whose accumulator bit-width falls below exact Laurent overflow bounds.

## Architecture Standards, Stakeholders, Concerns, and Components

- Standards: ISO-25010-2023 (2023), ISO-27005-2022 (2022), ISO-27034-1-2011 (2011), ISO-42010-2022 (2022).
- Game developer: Builds 3D games and interactive worlds on verdant-core with deterministic ECS, rigid-body physics, spatial audio, and starter templates.
- Technical artist: Authors procedural terrain, skeletal rigs, PBR materials, portable WGSL shaders, and glTF 2.0 scene exports that validate cleanly.
- Engine systems engineer: Maintains exact uor-matmul tensor kernels, progressive UORC streaming frames, frame-budget job graphs, and zero-constant parametric simulation.
- Browser player: Explores responsive 60fps 3D scenes in the browser across desktop and mobile viewports with keyboard, pointer, and spatial audio controls.
- Security auditor: Verifies shader safety, UORC stream and snapshot integrity, strict Content-Security-Policy isolation, and Core-Wasm archive provenance.
- UI and compliance architect: Models responsive 2D panels, diegetic 3D world-space interfaces, focus graphs, and WCAG 2.2 and ISO 9241 compliance invariants directly in Prism.
- Multiplayer world citizen: Inhabits persistent peer-to-peer 3D universes authenticated by zero-signin WebCrypto Prism IDs and healed by Stitch erasure fragments.

### Architectural Concerns

- Frame-time determinism and 60fps budget: Simulation, culling, portable tensor rendering, and audio synthesis execute within bounded frame budgets across browser worker and viewport pipelines.
- Graphics and asset standards compliance: Exported binary glTF 2.0 scenes, portable WGSL shaders, and UORC archives pass authoritative Khronos gltf, W3C naga, and UOR validators.
- Tensor determinism and progressive UORC streaming: Every uor-matmul tensor contraction accumulates without intermediate rounding and UORC scene frames stream monotonically up to lossless source fidelity.
- Commercial developer onboarding: Developers and agents can adopt the engine through runnable examples, a browser starter template, Diataxis documentation, and llms.txt.
- Accessible and clutter-free studio viewport: The browser Engine Studio keeps the 3D viewport front-and-center while meeting WCAG 2.2 AA contrast and coarse touch-target standards.
- Sandbox and snapshot integrity: World snapshots and UORC streams carry SHA-256 digests, shaders validate before compilation, and the Wasm verifier runs in an isolated worker.
- Formal UI layout and accessibility compliance: Screen-space and diegetic 3D interfaces solve via tropical and linear semiring constraints while enforcing coarse touch-target and WCAG contrast bounds.
- Second-order cybernetic mutual attention: The world observes player gaze, cadence, and focus via exact integer state-space recurrence and embedded coded_gemm reasoning without cloud APIs.
- Sovereign identity and Stitch peer persistence: Input journals carry non-extractable Prism ID signatures and world snapshots recover from any two of three SHA-256 Stitch parity fragments.
- Unbreakable spatial, optical, and narrative laws: Two-level triangle BLAS swept collision, near-sphere camera guards, floating-origin sector coordinates, and tropical conservation story automatons prevent clipping and deadlocks.

### System Components

- Verdant Core Engine: crates/verdant-core: standalone Rust and WebAssembly 3D game engine library providing uor-matmul tensor kernels, UORC streaming, ECS, BVH, glTF 2.0, WGSL, physics, spatial audio, and snapshots.
- Engine CLI Examples and Starter: Runnable examples (tensor_uorc_showcase, custom_game_arena, headless_benchmark) and templates/browser-3d-starter for custom 3D games.
- Verdant Engine Studio and Docs: Interactive 3D browser studio, progressive UORC LOD scrubber, glTF exporter, Diataxis documentation pages, Markdown twins, and llms.txt.
- Engine Verifier Guest: The Core-Wasm guest compiled from Engine.lex.tex that verifies progressive UORC streaming LODs and uor-matmul tensor kernels inside a Web Worker.
- Browser Viewport and WebGL2 Host: The visitor's universal browser canvas runtime presenting portable tensor framebuffers, depth-buffered WebGL2 3D draw calls, and pointer/keyboard input.
- Browser WebAudio Host: The browser audio graph rendering spatialized modal bar harmonics and room reverberation.
- GitHub Pages: Publishes the verified Engine Studio, documentation suite, and signed holo archive from CI.
- Agent Client: An automated coding agent reading llms.txt and Markdown documentation twins to build 3D applications on Verdant Engine.
- Verdant Tensor Semiring Substrate: crates/verdant-tensor: pure semiring tensor-logic substrate executing linear, tropical, audio and neuromorphic ring, and zero-decode coded_gemm SLM kernels.
- Prism ID and Stitch Multiplayer Fabric: Client-side WebCrypto Prism ID session signer, 2-of-3 XOR erasure fragment verifier, and BroadcastChannel and WebRTC rollback mesh.
- Verdant Wasm Worker Bridge: crates/verdant-wasm: zero-unsafe WebAssembly C-ABI simulation, lawful camera, progressive uorc_decompress streaming, and zero-copy OffscreenCanvas host.
- Verdant CLI Asset and Universe Packer: crates/verdant-cli: command-line binary glTF 2.0 importer, progressive UORC transcoder, universe sector bundler, story conservation verifier, and benchmark oracle.

## Design Views

- Streaming a 3D scene across progressive UORC LODs (Architecture::viewpointSystemsTensorStreaming).
- Components of Verdant Engine (Architecture::viewpointDeveloperOnboarding).
- Executing a verified browser frame (Architecture::viewpointSystemsPerformance).
- States of the engine verifier guest (Architecture::viewpointAuditorIntegrity).
- Mutual observation between player and neuromorphic world (Architecture::viewpointPlayerCybernetics).
- Synchronizing signed rollback journals and Stitch parity fragments (Architecture::viewpointCitizenStitchMesh).
- Stepping lawful swept physics, camera guards, and story conservation (Architecture::viewpointPlayerLawfulWorld, Architecture::viewpointDeveloperLawfulWorld, Design::processSolveUiConstraints, Design::processStepCyberneticObserver, Design::processRecoverStitchFragments, Design::processStepLawfulPhysicsAndCamera, Design::processAdvanceStoryConservation, Architecture::edgeCoreToWasm, Architecture::edgeWasmToStudio, Architecture::edgeCoreToCli).

## Features and Stakeholder Journeys

- A developer or player streams multi-LOD 3D scenes progressively via UORC from coarse collision proxies up to lossless SHA-256 chain-verified source resolution. (Tensor::streamingLods, Tensor::streamingLodsStrictlyMonotonic)
- Systems engineers execute exact Laurent and tropical uor-matmul tensor contractions and portable 16x16 software tile PBR rasterization without conditional browser GPU APIs. (Tensor::tensorKernels, Tensor::allSevenTensorKernelsValid)
- Game developers spawn entities, hierarchical transforms, and PBR materials into an archetype ECS indexed by a surface-area-heuristic BVH for raycasts and frustum culling. (verdant_core::ecs::SceneWorld, VE-03)
- Technical artists generate procedural terrain, skeletal rigs, morph targets, binary glTF 2.0 scenes, and portable WGSL shaders validated by Khronos gltf and W3C naga. (verdant_core::render::export_scene_glb, VE-04, VE-05)
- Symplectic Euler rigid bodies, player capsule controllers, and arc-length kinematic spline movers step deterministically over terrain heightfields. (verdant_core::physics::PhysicsWorld, VE-06)
- Spatial audio listeners render stereo modal bar harmonics and environmental soundscapes with inverse-distance attenuation and Sabine RT60 reverberation. (verdant_core::audio::AudioScene, VE-07)
- Systems engineers schedule acyclic worker job graphs within 60fps frame budgets and capture bit-exact IEEE-754 world snapshots with SHA-256 trailers. (verdant_core::runtime::WorldSnapshot, VE-08, VE-09)
- Developers, players, and coding agents author, import, and explore live 3D worlds in the self-hosted Verdant Engine Studio, executable Diataxis docs, and llms.txt. (Engine::dispatch, VE-10)
- UI and compliance architects solve responsive 2D panels and diegetic 3D interfaces via tropical and linear semiring constraints with proved 48px touch targets and WCAG 2.2 AA contrast. (Subsystems::uiWidgetCompliant)
- Browser players experience second-order cybernetic mutual attention and local zero-decode coded_gemm intent reasoning across capability-gated browser APIs. (Subsystems::cyberneticUiAndStitchActive)
- Multiplayer world citizens inhabit peer-to-peer rollback universes signed by zero-signin WebCrypto Prism IDs and recovered from 2-of-3 SHA-256 Stitch parity fragments. (Subsystems::stitchPairRecoverable)
- Players and developers inhabit high-definition vaulted worlds and 22-joint IK characters governed by non-penetrating swept physics, wall-guarded cameras, floating-origin cells, and conservation-verified story automatons. (Subsystems::lawfulHdUniversalEngineActive)

## Product Quality Model (ISO/IEC 25010:2023)

- Functional suitability: Verdant Engine executes exact uor-matmul tensor contractions, streams progressive UORC scenes, and exports standards-valid 3D assets.
- Interaction capability: Developers, players, and coding agents operate the 3D studio and documentation cleanly across devices.
- Maintainability: Every engine kernel is purely parametric with zero unexplained magic constants.
- Performance efficiency: Simulation, portable tensor rendering, UORC streaming, and Wasm verification execute within bounded frame and byte budgets.
- Reliability: Swept spatial collision, near-sphere camera guards, architectural portals, and story conservation automatons preserve faultless invariants.
- Subcharacteristics: Functional suitability: Functional completeness. Functional suitability: Functional correctness. Interaction capability: Inclusivity. Interaction capability: Learnability. Maintainability: Modifiability. Performance efficiency: Time behaviour. Reliability: Faultlessness.
- AccessibleStudio: Every studio and documentation page meets WCAG 2.2 AA and scores 100 in mobile Lighthouse accessibility.
- ValidTensorKernels: Every declared uor-matmul and software tile rasterizer kernel satisfies accumulator overflow bounds and tile alignment.
- MonotonicUorcStreaming: Progressive UORC scene streaming advances strictly monotonically from coarse proxy to lossless source resolution.
- StakeholderJourneys: Every declared engine stakeholder has at least one end-to-end journey through the product.
- ZeroConstantsKernel: Shipped engine Rust sources and constructive domain modules contain zero unexplained magic constants.
- BoundedFrameAndVerifier: The Core-Wasm engine verifier bounds requests to at most 64 bytes before evaluation.
- VerifiedEngineSubsystems: Every declared engine runtime subsystem satisfies stratum ordering and constructive runtime invariants.
- ValidCodedMatrixTiers: Every zero-decode uor_matmul::codec tier and HD LOD envelope satisfies lossless bit-width and monotonicity bounds.
- ArchitecturalWalkability: Every vaulted Citadel staircase riser and arched portal clears capsule step height, radius, and standing height bounds.
- AccessibilityScore: Lighthouse mobile accessibility score of the Engine Studio root page.
- InvalidTensorKernels: Count of declared engine tensor kernels failing accumulator bit-width or tile-alignment bounds.
- DisorderedStreamingLods: Count of UORC progressive LOD transitions violating strict resolution or work monotonicity.
- JourneysPerStakeholder: Minimum number of journeys declared for any engine stakeholder.
- MagicConstantViolations: Count of magic numeric literals detected by the permanent parametric ratchet.
- VerifierRequestCapBytes: Maximum byte length accepted by the Core-Wasm engine verifier.
- UnverifiedEngineSubsystems: Count of runtime subsystems failing constructive numbering, stratum bounds, or runtime invariants.
- InvalidCodedMatrixTiers: Count of zero-decode CodedMatrix streaming tiers or HD LOD steps failing validation.
- ImpassableArchitecturalPortals: Count of Citadel staircase or arched portal walkability violations against character capsule bounds.

## Application Security and Threat Model (ISO/IEC 27005 & ISO/IEC 27034)

- Verdant Engine security controls (0.1.0, 2026-11-15T00:00:00Z): Verdant Engine runs wholly client-side across universal browser viewports, WebAudio, and an isolated Core-Wasm Web Worker, validating portable WGSL shaders, binary glTF assets, UORC streams, and SHA-256 world snapshots.
- Security::assetVerdantCoreRuntime — Verdant Core runtime: The uor-matmul tensor, UORC streaming, ECS, physics, shader, glTF 2.0, and binary world-snapshot kernels in crates/verdant-core.
- Security::assetStudioAndDocsPages — Studio and documentation pages: The interactive 3D browser Engine Studio, Diataxis documentation pages, Markdown twins, and llms.txt.
- Security::assetVerifierGuestArchive — Engine verifier holo: The signed Core-Wasm guest archive verified inside the browser worker before dispatch.
- Security::assetPublishedPagesArtifact — Published Pages bundle: The static Engine Studio and holo bundle deployed by the GitHub Pages workflow.
- Security::assetStitchMultiplayerFabric — Prism ID and Stitch peer fabric: The non-extractable WebCrypto Prism ID signer, 2-of-3 Stitch parity shard verifier, and rollback sync channels.
- Security::assetVerdantWasmBridge — Verdant Wasm worker bridge: The WebAssembly C-ABI simulation, lawful camera, and progressive UORC worker runtime in crates/verdant-wasm.
- Security::threatInvalidShaderCompilation — Invalid or unsafe WGSL shader: A malformed or out-of-bounds portable shader source reaches compilation without front-end type and binding validation.
- Security::threatSnapshotTampering — Corrupted UORC stream or world snapshot: A truncated or modified UORC multi-LOD stream or binary world snapshot alters simulation state or entity transforms silently.
- Security::threatMalformedGlbExport — Non-conformant glTF 2.0 buffer layout: A degenerate mesh or misaligned buffer produces a binary glTF asset that downstream DCC tools reject or misread.
- Security::threatVerifierQueryExhaustion — Oversized verifier query: An unbounded byte payload sent to the Core-Wasm verifier exhausts guest memory or worker CPU budget.
- Security::threatStudioScriptInjection — Inline style or script injection: Unescaped markup or inline styles bypass Content-Security-Policy hashes in the browser Engine Studio.
- Security::threatHostSubstitution — Substituted static host: An untrusted server serves modified HTML in place of the repository's GitHub Pages deployment.
- Security::threatForgedPeerInputEnvelope — Forged multiplayer input journal: An unauthenticated peer injects spoofed rollback input commands to desynchronize or hijack world state.
- Security::threatCorruptedStitchParityShard — Tampered or duplicate Stitch fragment: A hostile relay or peer supplies duplicate fragment indices or corrupted parity bytes during UORC world recovery.
- Security::threatUngrantedContainerOrganAccess — Ungranted browser organ invocation: Audio, storage, or peer network organs activate without an explicit capability grant or after visitor denial.
- Security::threatWallClippingOrTunnelingBypass — Wall clipping or camera near-plane peeking: High-velocity movement or orbital camera angles clip through solid walls, ceilings, or floors to expose hidden geometry or tunnel out of bounds.
- Security::threatUniverseCoordinatePrecisionCollapse — Planetary coordinate precision collapse: Large world-space coordinates degrade IEEE-754 float precision or produce non-canonical cell offsets across multi-sector portals.
- Security::threatNarrativeSupplyInflationOrDeadlock — Narrative token duplication or quest deadlock: An unverified story or quest transition mints unaccounted items or traps players in a non-terminal state with no reachable completion path.
- Security::controlNagaShaderOracle — W3C naga WGSL validator: Every portable WGSL shader module is parsed and type-checked by naga::valid::Validator before acceptance. (cargo test -p repo-conformance engine_ve_05)
- Security::controlSnapshotSha256Trailer — Authenticated SHA-256 snapshot and UORC chain verifier: Every WorldSnapshot and progressive UORC frame verifies magic bytes, exact framing, and SHA-256 digests before decoding. (cargo test -p repo-conformance engine_ve_09)
- Security::controlKhronosGltfOracle — Khronos gltf 2.0 binary validator: Exported GLB payloads refuse empty geometry and pass gltf::Glb and gltf::import buffer validation. (cargo test -p repo-conformance engine_ve_04)
- Security::controlVerifierRequestCap — Bounded 64-byte Core-Wasm request cap: The Core-Wasm guest enforces requestMaximum := 64 and refuses unknown or empty queries deterministically. (prismpm --project app verify acceptance vectors)
- Security::controlStrictHashedCsp — Hash-locked Content-Security-Policy and worker verification: bootstrap/page.mjs hashes all styles and scripts into CSP with zero inline style mutations, and guest-worker.mjs verifies app.holo off the main thread. (cargo test -p repo-conformance frame_fr_01)
- Security::controlPrismIdJournalSignature — Prism ID signed journal envelope verifier: Every multiplayer FrameInputCommand batch verifies its Prism ID session envelope and SHA-256 chain before rollback admission. (cargo test -p repo-conformance engine_ve_24)
- Security::controlStitchFragmentCommitment — Stitch 2-of-3 SHA-256 fragment commitment verifier: Stitch recovery enforces distinct fragment indices under 3, matching manifest roots, and verified payload digests before accepting reconstructed bytes. (cargo test -p repo-conformance engine_ve_24)
- Security::controlCapabilityGrantLattice — Denial-wins browser capability grant gate: Every non-sandboxed browser container organ checks containerOrganAllowed so visitor denial always overrides prior grants. (cargo test -p repo-conformance engine_ve_22)
- Security::controlSweptBlasAndCameraGuard — Swept triangle BLAS CCD and near-sphere camera guard: Every movement and camera step sweeps capsules and near-plane diagonal guard spheres against TwoLevelBvh inside Rust, preventing wall peeking and tunneling. (cargo test -p repo-conformance engine_ve_25, cargo test -p repo-conformance engine_ve_26)
- Security::controlFloatingOriginCanonicalizer — Canonical i64 sector floating-origin verifier: UniversePosition canonicalizes local offsets strictly inside [-64m, +64m)^3 and evaluates camera-cell-relative GPU matrices. (cargo test -p repo-conformance engine_ve_29)
- Security::controlConservationAndReachabilityOracle — Story conservation and tropical reachability verifier: StoryAutomaton verifies exact integer supply conservation and tropical min-plus terminal reachability before admitting any narrative graph. (cargo test -p repo-conformance engine_ve_30)
- Security::threatHostSubstitution rationale: A static browser bundle cannot prevent substitution of its own HTTP origin; the holo archive carries verifiable cryptographic provenance and the repository rebuilds deterministically from source.

## Compliance Obligations

- ISO/IEC/IEEE 42010:2022 — Architecture description: Stakeholders, concerns, components, edges, viewpoints, and views of Verdant Engine resolve cleanly. (Architecture::architectureDescriptionHolds)
- ISO/PAS 19450:2015 and OMG UML 2.5.1 — Design views: OPM scene-streaming, cybernetic, and lawful world views plus UML component, sequence, Stitch rollback, and state-machine views conform to the architecture model. (Design::viewOpmSceneStreaming, Design::viewOpmCyberneticLoop, Design::viewOpmLawfulWorldAndStory, Design::viewUmlComponents, Design::viewUmlFrameSequence, Design::viewUmlStitchRollback, Design::viewUmlVerifierStates)
- ISO/IEC 25010:2023 — Product quality: Characteristics, subcharacteristics, requirements, and thresholds are numbered and verified by the Lean kernel. (Quality::qualityModelHolds)
- ISO/IEC 27034-1:2011 and ISO/IEC 27005:2022 — Application security and risk: Every browser, container organ, and peer runtime threat maps to an evidenced security control or explicit accepted risk. (Security::controlSet)
- Khronos glTF 2.0 and W3C WGSL — 3D asset and shader standards: Binary glTF 2.0 imports and exports plus portable WGSL shaders are validated mechanically by authoritative gltf and naga crates. (VE-04, VE-05, VE-19)
- UORC-SHA256-CHAIN-v1 and IEEE 754-2019 — Progressive UORC streaming and exact tensor contractions: All five progressive UORC and HD synthesis LOD tiers verify monotonic ascent and all fifteen unified uor-matmul tensor kernels satisfy exact accumulator bounds. (Tensor::streamingLodsStrictlyMonotonic, Tensor::hdStreamingLodsStrictlyMonotonic, Tensor::unifiedTensorAndCodedTiersVerified)
- WCAG 2.2 AA and W3C DTCG 2025.10 — Accessible studio, formal UI solver, and design tokens: Design tokens, 48px touch targets, and surface contrast ratios satisfy WCAG 2.2 SC 1.4.3 and SC 1.4.11 and pass mobile Lighthouse checks. (Brand::contrasts, Subsystems::uiWidgetCompliant)
- ISO 9241-11:2018 and ISO 24495-1:2023 — Context of use and plain-language documentation: User groups, goals, tasks, executable Diataxis documentation, Markdown twins, and llms.txt pass mechanical content checks. (Users::goals)
- prism/identity/4 and Stitch Erasure Custody — Sovereign Prism ID journals and 2-of-3 parity recovery: Peer input journals bind to non-extractable WebCrypto Prism IDs and world snapshots recover exactly from any two distinct Stitch fragments. (Subsystems::stitchPairRecoverable)
- Lawful Spatial, Optical, and Conservation Invariants — Swept triangle BLAS CCD, near-sphere camera guards, floating-origin cells, and story conservation: Every frame transition preserves surface clearance, near-plane non-clipping, Citadel architectural walkability, bounded cell offsets, and exact narrative supply conservation. (Subsystems::lawfulHdUniversalEngineActive, Subsystems::architecturalWalkabilityVerified, VE-25, VE-26, VE-29, VE-30)

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
- GlbIngestionAndUorcTranscoder: Full Khronos binary glTF 2.0 scene hierarchy, skeletal skin, keyframe clip, and PBR material importer transcoding directly into progressive multi-LOD UORC streams.
- XpbdJointsVehiclesAndOcclusion: Delassus and XPBD hinge, ball-socket, slider, and spring-damper rigid joints, swept capsule and four-wheel raycast vehicle controllers, and BVH acoustic occlusion.
- DeterministicRollbackAndInputJournal: Frame-indexed SHA-256 input journals, 64-frame ring-buffered rollback resimulation, and UORC-compressed delta world snapshots for lockstep and rollback netcode.
- FormalUiAndWcagCompliance: Tropical and exact linear 2D and 3D UI layout solver, diegetic BVH hit-testing, focus reachability closure, and WCAG 2.2 and 48px target compliance oracle.
- NeuromorphicObserverAndCodedSlm: Second-order cybernetic mutual-attention LIF and SSM state loop, zero-decode coded_gemm intent reasoning head, and capability-gated W3C container organs.
- PrismIdAndStitchMultiplayerMesh: Zero-signin WebCrypto Prism ID journal signing, 2-of-3 SHA-256 XOR Stitch erasure fragment recovery, and peer-to-peer WebRTC and BroadcastChannel rollback sync.
- TwoLevelBlasAndSweptCcd: Two-level entity TLAS and SAH triangle MeshBlas BVH with continuous swept-capsule collide-and-slide and two-plane acute crease projection.
- LawfulNearSphereCameraRig: Near-frustum diagonal guard sphere camera sweep preventing wall, floor, ceiling, and prop near-plane clipping across orbit and eye views.
- HighDefCitadelAndMultiPassPbr: Progressive LOD4 UORC vaulted Citadel Hall, parabolic Arch Bridge, sculpted cliff terrain, CSM PCF soft shadows, GGX micro-normals, and ACES tonemapping.
- ArticulatedHumanoidAndFootIk: Twenty-two-joint humanoid rig, normalized linear-blend skinning, analytical two-bone foot IK stair adaptation, and neuromorphic head-look possession.
- FloatingOriginUniverseAndStoryAutomaton: Exact i64 cell-relative floating-origin coordinates, multi-sector UORC streaming portals, and conservation-verified tropical story and app automatons.
- OntologicalTelosAndW5HGovernor: Constructive W5H stratum certificates, HostEnvelope and WitnessEnvelope telos loss evaluation, and non-coercive parametric LOD descent.
- PerfTierHoistedStratumConstsAndCodebookLut: OnceLock-hoisted constructive {0, 1} Prism kernel constants and 256-entry codebook product lookup tables executing inner contractions in machine registers with zero magic literals.
- PerfTierWordPackedSoaArchetypeEcs: Contiguous Structure-of-Arrays entity storage paired with 64-bit word-packed visibility, dirty-transform, and awake-physics bitmasks sweeping arbitrary entity counts in O(N/64).
- PerfTierContiguousSahFlatBvhAndToiOrder: Cache-line-aligned 32-byte linear SAH TLAS and triangle MeshBlas arrays with 64-slot stack traversal, near-child-first TOI pruning, and zero per-query heap allocations.
- PerfTierEpochSplitVboAndAsyncShadowCache: Epoch-guarded static STATIC_DRAW and dynamic STREAM_DRAW WebGL2 VAO split, asynchronous zero-gl.finish command pipelining, texel-snapped shadow caching, and worker frame-ack coalescing.
- PerfTierHardwareInstancedCrowdAndLatticeBatches: Hardware drawElementsInstanced batch buffers, lazy software 16x16 tile screen-space binning, and OntologicalRuntimeGovernor render-scale adaptation supporting tens of thousands of active instances.

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
- VerdantTensorCore: Pure no_std semiring tensor-logic substrate crate crates/verdant-tensor/Cargo.toml hosting linear, tropical, audio-ring, and zero-decode uor-matmul contractions.
- ve-lod-btn: Cycles progressive UORC and HD streaming level of detail from coarse collision proxy LOD0 through ultra-high-definition LOD4.
- ve-sim-btn: Toggles between demand-driven stepped inspection and continuous 60fps orbital and spline mover simulation.
- ve-cam-btn: Cycles lawful near-sphere-guarded Orbit, 3rd-Person Two-Bone Foot IK Explorer, and 1st-Person head-masked possession modes.
- ve-audio-btn: Toggles WebAudio Euler-Bernoulli modal bar harmonics and spatial room reverberation.
- ve-glb-btn: Serializes and downloads the active 3D scene as a standards-valid binary glTF 2.0 (.glb) asset.
- ve-snap-btn: Encodes an authenticated SHA-256 VDNT binary world snapshot and copies a shareable #vdnt= studio URL.
- ve-docs-link: Navigates directly from the Engine Studio bar to the Diataxis documentation suite.
- ve-uorc-btn: Decodes the progressive multi-LOD UORC scene stream natively in WebAssembly via uorc_decompress and runs zero-decode coded_gemm contractions.
- ve-studio-btn: Opens the live Verdant Studio inspector for GLB authoring, formal UI proof overlays, cybernetic attention, and Stitch peer custody.
- ve-render-btn: Switches viewport rendering live between 60fps WebGL2 hardware draw calls and the pure WebAssembly 16x16 software tile PBR rasterizer.
- ve-import-btn: Imports any external binary glTF 2.0 (.glb) model into SceneWorld and transcodes progressive multi-LOD UORC frames in WebAssembly.
- ve-mesh-btn: Signs deterministic input journals with the visitor Prism ID, shards snapshots into Stitch parity fragments, and syncs rollback peers.
- ve-prompt-btn: Routes natural-language world, camera, LOD4, and Chronicle intents locally via zero-decode coded_gemm SLM contractions.
- StudioThemeLightPaper: Warm paper light theme bound to canonical Hologram W3C DTCG CSS custom properties with zero hardcoded hex overrides and WCAG 2.2 AA contrast.
- StudioThemeDarkObsidian: Warm obsidian dark theme bound to canonical Hologram W3C DTCG CSS custom properties across studio panels, dialogs, and reactive 2D telemetry canvases.
- StudioThemeImmersiveGlass: Translucent backdrop-blurred immersive frame theme adapting studio rails, HUD pills, and workbench surfaces over the canonical Prism backdrop.
- WorkbenchTabGuidedJourneys: Interactive step-by-step stakeholder journeys executing live WebAssembly viewport, physics, shader, UI, security, and multiplayer actions.
- WorkbenchTabSceneOutliner: Live SceneWorld entity hierarchy outliner with one-click camera framing, binary glTF 2.0 import and OPFS caching, progressive UORC streaming, and WebGL2 or 16x16 tile PBR switching.
- WorkbenchTabCreateProject: Rapid game and app project scaffolder generating zero-magic-constant Cargo.toml, src/main.rs, WebAssembly browser harness, GLB scene, and SHA-256 VDNT snapshot files.
- WorkbenchTabPhysicsAndStory: Live TwoLevelBvh swept-capsule and near-sphere camera clearance monitor, 22-joint Foot IK inspector, OntologicalRuntimeGovernor W5H telos readout, and StoryAutomaton conservation stepper.
- WorkbenchTabUiAndNeural: Theme-aware tropical and linear 2D/3D UI compliance solver, 48px touch and WCAG 2.2 AA verifier, 8-neuron Q15 LIF spike raster, and local coded_gemm SLM intent tester.
- WorkbenchTabMeshAndTrust: Denial-wins FK-07 browser container capability grant matrix paired with non-extractable Foundry Prism ID session signing and 2-of-3 SHA-256 Stitch parity shard recovery.
- StudioJourneyKaiGameDev: Executable studio journey guiding game developers through 22-joint Foot IK possession, swept Citadel traversal, Chronicle interaction, and Rust game project scaffolding.
- StudioJourneyIrisTechArt: Executable studio journey guiding technical artists through progressive LOD0-to-LOD4 UORC streaming, WebGL2 vs 16x16 software tile PBR comparison, and Khronos GLB export.
- StudioJourneyOttoSystems: Executable studio journey guiding systems engineers through 15 unified uor-matmul semiring kernels, zero-copy WebAssembly scene buffers, and W5H frame-budget telos telemetry.
- StudioJourneyWrenPlayer: Executable studio journey guiding browser players through landmark camera orbits, modal spatial audio harmonics, and natural-language coded_gemm SLM world direction.
- StudioJourneyVeraSecurity: Executable studio journey guiding security auditors through FK-07 denial-wins capability gates, SHA-256 VDNT snapshot verification, and Core-Wasm acceptance queries.
- StudioJourneyMiraUiCompliance: Executable studio journey guiding UI compliance architects through tropical max-plus widget layout, 48px coarse target proofs, and Light, Dark, and Immersive token contrast.
- StudioJourneySolStitchMesh: Executable studio journey guiding multiplayer citizens through non-extractable Prism ID signing, 2-of-3 Stitch parity sharding, and single-shard-drop recovery.
- ArchetypeThirdPersonCitadelExplorer: Playable third-person action-exploration starter archetype combining a 22-joint Foot IK character, swept TwoLevelBvh collision, near-sphere spring-arm camera, and vaulted Citadel geometry.
- ArchetypeFirstPersonWalkthrough: First-person architectural walkthrough starter archetype featuring head-masked eye possession, LOD4 ultra-high-definition masonry, four-cascade shadows, and Sabine modal chamber acoustics.
- ArchetypeConservationChronicleRpg: Narrative RPG starter archetype pairing a deadlock-free tropical StoryAutomaton and exact integer ConservationLedger with grounded NPC dialogue and floating-origin universe portals.
- ArchetypeCompliantDiegetic3DApp: Spatial 3D application starter archetype solving screen-space and diegetic world UI panels via tropical constraints with proved 48px targets, WCAG 2.2 AA contrast, and signed VDNT state.
- LiveEmbedDocsViewport: Interactive 3D WebGL2 and software tile documentation landing embed rendering zero-copy Citadel buffers from cold-boot verdant-wasm.wasm with orbit, LOD0-to-LOD4, and camera controls.
- LiveEmbedFirstSceneObserver: Theme-aware getting-started live WebAssembly embed stepping the 8-neuron Q15 LIF mutual-attention observer and interactive landmark salience slots.
- LiveEmbedCustomGamePhysics: Theme-aware custom-game live WebAssembly embed stepping lawful TwoLevelBvh capsule CCD, near-sphere camera clearance, spline movers, and Woodworth binaural cues.
- LiveEmbedAssetPipelineLodStream: Theme-aware asset-pipeline live WebAssembly embed measuring monotonic LOD0-to-LOD4 triangle scaling, scene node counts, and UORC streaming envelopes.
- LiveEmbedArchitectureSemirings: Theme-aware architecture explanation live WebAssembly embed computing four-cascade shadow splits, Sabine RT60 reverberation, and Euler-Bernoulli modal ratios from cold-boot exports.
- LiveEmbedApiReferenceSnapshot: Progressive API reference live WebAssembly embed rendering instant cold-boot VDNT framing bounds and streaming the SHA-256 snapshot codec for clean vs bit-flipped tamper proofs.

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
