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
- Players and developers inhabit high-definition vaulted worlds and 22-joint IK characters governed by non-penetrating swept physics, wall-guarded cameras, floating-origin cells, and conservation-verified story automatons. A rendered GLB node retains its physical role and transformed triangle boundary in camera and movement queries. The full player capsule sweeps that boundary; modeled non-solid roles allow passage. (Subsystems::lawfulHdUniversalEngineActive)

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
- PerfTierAqmMonoidBitPlanesAndKappaCache: Algebraic monoid command canonicalization, eight-column 64-bit bit-plane archetype matrices, in-place tropical closure with next-hop paths, sparse two-body XPBD Delassus solves, and content-addressed kappa transition memoization.
- PerfTierRockSolidStabilityCsrRasterAndBroadphase: Flat CSR prefix-sum software tile binning, spatial-grid multi-body broadphase collisions, multi-primitive GLB merging, WebGL2 context-loss recovery, and deadlock-free worker watchdog guards.
- PerfTierUniversalUorcExclusiveRuntime: Exclusive post-boot UORC wire, OPFS, URL, Stitch custody, GLB transcoding, rollback checkpoint, and zero-decode CodedMatrix streaming across native Rust and WebAssembly.

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

## Observer Holon, Stakeholder Prism Models, and Observed Communications

- ObserverRootHolon: Self-observing second-order Prism holon composing its five platform organs, seven stakeholder Prism models, character profiles, and observed communication channels.
- ObserverOrganEngine: Dedicated WebAssembly and Rust 3D model runtime hosting uor-matmul tensor contractions, UORC streaming, swept collision, and native language reasoning.
- ObserverOrganStudio: Seven-workspace browser authoring workbench coupling role-specific inspectors, catalog domain switching, and live Observer communication telemetry.
- ObserverOrganCatalogs: SHA-256-locked registry of Schema.org, W3C SOSA and PROV-O, Khronos glTF 2.0, ISO 24617-2, OMG DMN 1.5, and multi-domain world packs.
- ObserverOrganDemo: Catalog-instantiated multi-domain showcase worlds featuring possessed 22-joint characters, Sabine acoustics, and conserved item quests.
- ObserverOrganReference: Executable Diataxis tutorials, how-to guides, architecture explanations, API references, and live cold-boot WebAssembly embeds.
- StakeholderModelGameDeveloper: Full Prism model of the game developer authoring Rust and WebAssembly 3D worlds, scaffolding cataloged starter domains, and verifying deterministic physics.
- StakeholderModelTechnicalArtist: Full Prism model of the technical artist curating Khronos glTF 2.0 HD PBR materials, 22-joint humanoid rigs, WebGL2 shaders, and progressive UORC streams.
- StakeholderModelSystemsEngineer: Full Prism model of the engine systems engineer governing uor-matmul semiring kernels, W5H telos frame budgets, CSR tile rasterization, and zero-constant code.
- StakeholderModelBrowserPlayer: Full Prism model of the browser player inhabiting possessed characters, conversing with grounded NPCs, and steering mutual-attention world salience.
- StakeholderModelSecurityAuditor: Full Prism model of the security auditor enforcing OMG DMN 1.5 Verdant-only runtime admission, FK-07 denial-wins grants, and SHA-256 authority locks.
- StakeholderModelUiComplianceArchitect: Full Prism model of the UI and compliance architect proving tropical max-plus widget layouts, 48px touch targets, W3C DTCG tokens, and WCAG 2.2 AA contrast.
- StakeholderModelMultiplayerCitizen: Full Prism model of the multiplayer citizen exchanging Prism ID signed peer messages, spatial audio cues, and 2-of-3 Stitch parity world shards.
- ChannelOrganToOrgan: Observed inter-organ message channel coupling Engine, Studio, Catalogs, Demo, and Reference states under W3C SOSA and PROV-O lineage.
- ChannelPlayerToPlayer: Prism ID signed peer-to-peer communication channel between human players and citizens synchronized over BroadcastChannel and Stitch rollback journals.
- ChannelPlayerToAgent: Prism ID signed ISO 24617-2 dialogue channel from a human player to a cataloged NPC or coding agent evaluated natively in Verdant Engine.
- ChannelAgentToPlayer: Passage-grounded response channel from an NPC or agent to a player returning cited world facts or deterministic Unknown abstention.
- ChannelAgentToAgent: Observed multi-agent coordination channel between cataloged NPCs steering A-star NavMesh patrols and conserved item handoffs.

## Self-Hosted Stitch Network, Zero-Account CIA Parity, and Composable Blank Frame

- Security::assetImmutableBlankFrame — Immutable Blank Frame: Frozen universal browser shell pinned declaratively via frame-pin.json to Stitch-hosted Verdant releases or third-party deep links, verifying BLAKE3, Prism ID quorum signatures, and acceptance vectors off-thread before mounting via a same-origin ServiceWorker virtual filesystem.
- Security::assetStakeholderPrivateBoundary — Stakeholder Private Boundary: Zero-account stakeholder content encrypted before k-of-n Reed-Solomon sharding under a Prism ID boundary key with revocation epoch rotation, granting identical confidentiality, integrity, and availability without external user accounts.
- Security::assetVerdantVcsAndReleases — Verdant VCS and Release Capsules: In-Verdant W3C PROV-O content-addressed Merkle revision control, signed monotonic release pointers, self-verifying .holo capsules, and optional GitHub Packages .holo and Rust .crate distribution adapters.
- Security::threatUnauthorizedPrivateBoundaryRead — Unauthorized private boundary read: An untrusted Stitch relay, peer browser, or public mirror inspects stored k-of-n shards to read a user's private project or world without authorization.
- Security::threatRevokedPeerRetentionOrReplay — Revoked peer retention or replay: A previously shared collaborator whose access was revoked attempts to decrypt new revisions or forge updates using an old boundary key or stale session.
- Security::threatReleasePointerRollbackOrEquivocation — Release pointer rollback or equivocation: A hostile relay or compromised mirror serves an older signed release pointer to pin a Blank Frame to a superseded version.
- Security::threatBlankFrameHydrationInjection — Blank Frame hydration injection: An attacker injects unverified markup, script, or Wasm into the Blank Frame or ServiceWorker virtual filesystem before holo verification completes.
- Security::threatStitchStorageFloodOrSybilExhaustion — Stitch storage flood or Sybil exhaustion: A Sybil attacker with zero accounts floods public-benefit Stitch relays or peer browsers with junk shards to exhaust free storage or bandwidth.
- Security::threatShardAttritionOrColdDisappearance — Shard attrition or cold disappearance: Peer browsers close tabs or a host goes offline, dropping live shards below the k-of-n reconstruction threshold.
- Security::threatDeepLinkFramePhishingOrOriginConfusion — Deep-link frame phishing or origin confusion: A malicious third-party pinned frame tricks a user into signing a Prism ID action for another origin or leaks a private boundary key across origins.
- Security::threatUnverifiedRepoDriftOrBypass — Unverified repo drift or release bypass: Workspace files change out-of-band or a release ships without being published to Stitch and verified through the Blank Frame.
- Security::controlEncryptBeforeShardBoundary — Encrypt-before-shard AEAD boundary gate: Mandatory Encrypt-then-MAC AEAD encryption and fixed-block padding under per-boundary Prism ID keys occur before k-of-n Reed-Solomon sharding so untrusted peers hold only authenticated ciphertext. (cargo test -p repo-conformance engine_ve_43)
- Security::controlBoundaryEpochRotationAndRevocation — Boundary epoch rotation and revocation gate: Collaborator revocation increments the boundary epoch, rotates keys forward-secretly, binds epoch into AEAD associated data, and refuses stale-epoch writes. (cargo test -p repo-conformance engine_ve_43)
- Security::controlMonotonicEpochQuorumPointer — Monotonic epoch quorum release pointer gate: The Blank Frame persists the highest verified release epoch in OPFS, requires Prism ID threshold quorum signatures and prevKappa chaining, and refuses any rollback epoch. (cargo test -p repo-conformance engine_ve_43)
- Security::controlZeroTrustSwVirtualFsMount — Zero-trust ServiceWorker virtual filesystem mount: Strict hashed CSP blocks inline scripts and eval while the boot Worker verifies BLAKE3, SHA-256, Prism ID signatures, and acceptance vectors before mounting verified capsule files. (cargo test -p repo-conformance frame_fr_01)
- Security::controlStitchBudgetAndReciprocalAdmission — Stitch budget and reciprocal custody admission gate: Enforces identity/stitch.lock budget reservation, settlement, per-Prism-ID byte and rate quotas, and bounded reciprocal LRU storage without requiring external accounts. (cargo test -p repo-conformance engine_ve_43)
- Security::controlKOfNReedSolomonAndAutonomousRepair — Systematic k-of-n Reed-Solomon custody and autonomous repair: 4-of-8 GF(2^8) Reed-Solomon sharding across local OPFS, WebRTC Stitch swarm, and public relays reconstructs data after losing up to half of all shards and regenerates missing parity autonomously. (cargo test -p repo-conformance engine_ve_43)
- Security::controlOriginScopedPrismIdAudience — Origin-scoped Prism ID audience and pin verification: Non-extractable WebCrypto Prism IDs bind audience_origin and display the active pinned Stitch URI so third-party frames cannot forge cross-origin signatures. (cargo test -p repo-conformance engine_ve_24)
- Security::controlSelfHostingRatchetAndPullThroughGate — Self-hosting ratchet and pull-through release gate: Tightens parametric-ratchet ceilings, verifies the cryptographic View Immutability Seal, and requires every release to pull cleanly from k-of-n Stitch shards through the Blank Frame. (cargo test -p repo-conformance engine_ve_42)

## Autonomous Release Cadence and Multi-Channel Distribution

- SemVer milestone lineage: v0.4.0 (epoch 1) -> v1.0.0 (epoch 2) -> v1.1.0 (epoch 3) -> v2.0.0 (epoch 4) -> v2.1.0 (epoch 5), advancing monotonically only when main is green, the ratchet is applied, and all release gates pass.
- github-pages-stitch-shards: https://afflom.github.io/verdant-engine/stitch/v{version}/shards
- jsdelivr-stitch-mirror: https://cdn.jsdelivr.net/gh/afflom/verdant-engine@stitch-mirror/v{version}/shards
- raw-github-stitch-mirror: https://raw.githubusercontent.com/afflom/verdant-engine/stitch-mirror/v{version}/shards
- cargo-sparse-registry: sparse+https://afflom.github.io/verdant-engine/crates/index/
- ghcr-oci-release-bundle: ghcr.io/afflom/verdant-engine/verdant-release:v{version}
- zenodo-permanent-doi-archive: https://zenodo.org/records/24383968

## Sovereign Post-GitHub Governance, Ownership, Access Control, and Threat Model

### Governance Concept and Formal Principles
- GenesisOwnerAuthorityRoot: The repository owner initializes authority epoch one with a non-extractable Prism ID genesis root detached from GitHub and bootstraps the initial Sovereign Admin quorum.
- ContentAddressedSigningRequest: A developer signs and submits a content-addressed signing request binding target commit kappa, action scope, current authority epoch, and expiry epoch directly to Verdant.
- AdminReviewApproveOrReject: Active Verdant Sovereign Admins inspect the verifiable change, ratchet verdict, and scope before emitting a signed PROV-O approve or reject attestation.
- ThresholdMOfNAdminQuorum: Once multi-admin custody is active, privileged mutations enforce an M-of-N threshold quorum with at least two independent active Admin signatures and separation of duties.
- MonotonicMembershipEpochChain: Every Admin admission, resignation, revocation, or policy change strictly increments the authority epoch and chains prevEpochKappa so rollback and split-brain forks are impossible.
- DepartedAdminApprovalRevocation: When an Admin leaves Verdant, future signing authority terminates immediately at the new epoch, unfinalized pending approvals and sole-attested standing grants from that Admin are invalidated, and private workspace keys rotate forward-secretly.
- HistoricalEpochVerificationAndQuarantine: Commits finalized under a valid quorum prior to clean Admin departure remain verifiable at their historical epoch while compromise revocations quarantine suspect artifacts from the compromise floor epoch.
- AntiLockoutSuccessionGuard: Admin departure or removal is refused if remaining active Admins would drop to zero or below the active quorum threshold without first onboarding a successor or atomically ratifying a valid lower threshold.
- GitHubFreeGroupCoordination: Issues, RFC proposals, milestones, path-scoped review ownership, and role grants persist as signed W3C PROV-O envelopes across VerdantVcs and Stitch parity shards with zero GitHub identity dependency.

### Stakeholders, Concerns, Components, Features, Design Views, Quality, and Compliance
- Verdant Sovereign Admin: Custodian of the sovereign Prism ID authority quorum who reviews contributor signing requests, signs or rejects proposals, manages epoch membership, and rotates revocation boundaries without GitHub.
- Project Contributor: Developer or collaborator with a local Prism ID who authors VerdantVcs commits, issues, RFCs, and signed approval requests for Admin review without external accounts.
- GitHub-free sovereign ownership and access control: Genesis repo owner authority transitions to an M-of-N Prism ID admin quorum with separation of duties, monotonic membership epochs, and immediate revocation of departed admin approvals.
- Decentralized group project management and signing requests: Developers submit content-addressed signing requests, issues, RFCs, and release proposals over VerdantVcs and Stitch shards for verifiable Admin review without GitHub.
- Sovereign Authority and Revocation Ledger: Append-only PROV-O epoch ledger anchoring the genesis owner Prism ID, M-of-N admin quorum state, monotonic revocation epochs, and anti-lockout succession proofs.
- Signing Request and Coordination Gate: Verifies developer signing requests, Admin approve and reject attestations, departed-admin pending approval invalidation, and path-scoped group coordination rules.
- Feature — Sovereign signing requests: Project contributors with local Prism IDs submit content-addressed signing requests for merges, releases, role grants, and group coordination items to Verdant for verifiable Admin review without GitHub.
- Feature — Admin quorum and departure revocation: Verdant Sovereign Admins review, sign, or reject signing requests under an M-of-N authority quorum anchored in the genesis repo owner, immediately revoking a departed Admin's future signing authority and invalidating their pending or standing approvals.
- Feature — Decentralized group project coordination: Teams coordinate issues, RFC proposals, milestones, path-scoped review ownership, and release sign-offs as signed W3C PROV-O envelopes stored inside VerdantVcs and Stitch erasure shards.
- Design::viewOpmSovereignSigningAndRevocation — Submitting signing requests, reviewing under M-of-N Admin quorum, and revoking departed Admin approvals (Architecture::viewpointAdminSovereignAuthority).
- Design::viewUmlSigningRequestReview — Contributor signing request review, quorum sign or reject, and epoch verification sequence (Architecture::viewpointContributorSigningCoordination).
- Design::viewUmlAdminAuthorityLifecycle — Sovereign Admin authority lifecycle from Genesis Owner through active quorum custody to departed revocation (Architecture::viewpointAdminSovereignAuthority).
- Quality characteristic — Security: Sovereign Prism ID authority epochs, M-of-N Admin quorums, signed request review, and departed-admin approval revocation enforce GitHub-free access control and accountability.
- Quality subcharacteristic — Accountability: Security: Accountability.
- Quality requirement — SovereignGovernanceQuorum: GitHub-free signing requests, M-of-N Admin quorums, monotonic authority epochs, and departed-admin approval revocations satisfy every constructive governance invariant.
- Quality measure — GovernancePolicyViolations: Count of sovereign authority, quorum threshold, signing-request, or departed-admin revocation invariant violations.
- Compliance obligation — ISO/IEC 27001:2022, ISO/IEC 27005:2022, and W3C PROV-O Sovereign Governance — GitHub-free sovereign ownership, M-of-N Admin quorum, signing requests, and departed-admin revocation: Sovereign authority epochs anchor in the genesis owner Prism ID, require independent M-of-N Admin review of contributor signing requests, and revoke departed Admin approvals immediately without GitHub.

### Sovereign Governance STRIDE Threat Model, Assets, Risks, and Evidenced Controls
- Security::assetSovereignAuthorityLedger — Sovereign Authority and Revocation Ledger: Append-only PROV-O authority epoch ledger anchoring the genesis repo owner Prism ID, active M-of-N Admin quorum set, monotonic revocation epochs, and succession rules independent of GitHub.
- Security::assetSigningRequestQueue — Signing Request and Group Coordination Queue: Content-addressed queue of developer signing requests, Admin approve and reject attestations, role grants, issues, RFCs, and path-scoped review obligations stored across VerdantVcs and Stitch shards.
- Security::threatForgedSigningRequestOrAdminSpoof — Forged signing request or Admin impersonation: An attacker submits an unauthenticated signing request or impersonates an active Verdant Sovereign Admin without possessing a valid non-extractable Prism ID key.
- Security::threatAuthorityEpochSplitBrainOrRollback — Authority epoch rollback or split-brain fork: A hostile mirror or rogue peer serves an older authority epoch or forked Admin set to resurrect a departed Admin or weaken the M-of-N threshold.
- Security::threatRepudiatedAdminApprovalOrPolicyEdit — Repudiated Admin verdict or silent policy edit: An Admin or contributor denies signing an approve or reject verdict, role grant, or group coordination policy change because actions lack immutable cryptographic lineage.
- Security::threatDepartedAdminPrivateBoundaryLeak — Departed Admin private workspace decryption: An Admin who leaves Verdant retains stale private boundary keys and decrypts confidential proposals, issues, or branches authored after departure.
- Security::threatAdminQuorumLockoutOrRequestFlood — Admin quorum lockout or signing-request flood: Last-Admin departure, key loss dropping active Admins below threshold M, or unthrottled Sybil signing-request spam permanently deadlocks governance or exhausts Admin review capacity.
- Security::threatRevokedAdminSignatureOrStaleApprovalReplay — Revoked Admin signature or stale approval replay: A departed or revoked Admin signs new requests, replays a stale pre-departure approval on an unfinalized request, or exploits a sole-attested standing grant after leaving Verdant.
- Security::threatRogueSingleAdminOrSelfApprovalBypass — Rogue single-Admin takeover or self-approval bypass: A single compromised Admin or contributor unilaterally self-approves a privileged merge, release, role grant, or Admin addition without independent M-of-N quorum review.
- Security::controlPrismIdSigningRequestAuth — Prism ID signing-request and Admin key authenticator: Requires non-extractable Prism ID signatures binding request kappa, action scope, current authority epoch, and expiry epoch before admitting any request or Admin review. (Governance::signingRequestStateMachineVerified)
- Security::controlMonotonicAuthorityEpochChain — Monotonic prevEpochKappa authority epoch chain: Chains every membership transition via prevEpochKappa from genesis epoch one, requiring strict epoch monotonicity and active quorum signatures while refusing rollback or split-brain forks. (Governance::sovereignGovernanceRulesComplete)
- Security::controlProvoSignedReviewLedger — Immutable W3C PROV-O signed review and coordination ledger: Records every contributor signing request, Admin approve or reject attestation, role grant, and work-item update as an immutable signed PROV-O envelope across VerdantVcs and Stitch shards. (Governance::sovereignGovernanceRulesComplete)
- Security::controlDepartureForwardSecretRekey — Forward-secret boundary rekey on Admin departure: Rotates AEAD private project boundary keys immediately whenever an Admin resigns or is revoked so departed Admins cannot decrypt subsequent proposals, branches, or issues. (Governance::adminDepartureRevocationVerified)
- Security::controlAntiLockoutSuccessionAndQuotaGate — Anti-lockout succession guard and Stitch submission quotas: Refuses any Admin departure or removal that would leave zero Admins or fewer active Admins than threshold M, and enforces per-Prism-ID Stitch rate quotas on incoming signing requests. (Governance::adminDepartureRevocationVerified)
- Security::controlImmediateDepartureApprovalRevocation — Immediate departed-Admin approval invalidation and compromise quarantine: Terminates a departed Admin's signing authority at the new epoch, subtracts their signatures from pending unfinalized requests, revokes sole-attested standing grants, and quarantines compromise-window commits while preserving clean historical finalized epochs. (Governance::adminDepartureRevocationVerified)
- Security::controlThresholdQuorumSeparationOfDuties — M-of-N Admin quorum and non-self-approval gate: Requires at least two independent active Admin signatures under an M-of-N threshold once multi-admin custody is active, excluding author self-approval from satisfying privileged quorum. (Governance::signingRequestStateMachineVerified)
