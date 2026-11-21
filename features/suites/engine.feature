Feature: engine

  Verdant Engine's standalone universal 3D simulation, portable tensor compute,
  progressive UORC multi-LOD streaming, asset, physics, spatial audio, runtime
  scheduling, binary world-snapshot, and Core-Wasm verification capabilities,
  including positive validation against authoritative standards oracles and typed
  refusals for every fallible boundary.

  @VE-01 @build
  Scenario: Progressive multi-LOD UORC scene streams advance monotonically to lossless SHA-256 chain-verified resolution and refuse regressive LODs or corrupted UORC frames
    Given a multi-LOD 3D benchmark scene seeded deterministically
    When progressive UORC streaming frames advance across Lod0CoarseProxy, Lod1MediumDetail, Lod2HighDetail, and Lod3LosslessSource
    Then every frame verifies UORC-SHA256-CHAIN-v1 integrity within bounded decode memory up to lossless source fidelity
    And stepping backward in LOD resolution or decoding a truncated UORC stream returns StreamingError

  @VE-02 @build
  Scenario: Exact Laurent and tropical uor-matmul tensor contractions and portable 16x16 software tile PBR rasterization execute deterministically and refuse mismatched dimensions or non-finite inputs
    Given homogeneous vertex batches, tropical bounding planes, modal audio frames, and a 3D scene camera
    When uor-matmul exact contractions and the portable 16x16 tile PBR rasterizer execute
    Then transformed vertices, tropical distance bounds, stereo audio mixes, and RGBA8 framebuffers are bit-identical across runs
    And mismatched matrix extents or non-finite tensor elements return TensorComputeError

  @VE-03 @build
  Scenario: Archetype ECS world and surface-area-heuristic BVH execute deterministic raycasts and frustum culling across arbitrary 3D scenes
    Given a 3D scene populated with hierarchical entities and bounding volumes
    When a surface-area-heuristic BVH is constructed over the world
    Then raycasts and perspective frustum queries return deterministic hits and visible entity sets

  @VE-04 @build
  Scenario: Procedural meshes, morph targets, and PBR scenes export to binary glTF 2.0 validated by the Khronos gltf oracle and refuse degenerate geometry
    Given procedural heightfields, columns, ribbons, morph targets, and metallic-roughness PBR materials
    When the scene is exported to binary glTF 2.0
    Then the exported GLB passes authoritative Khronos gltf validation with exact mesh and material counts
    And exporting an empty or out-of-bounds mesh returns AssetValidationError

  @VE-05 @build
  Scenario: PBR, terrain, fluid, and sky portable WGSL shader modules pass W3C naga validation and invalid shader sources are refused
    Given Verdant Engine's built-in PBR surface, fluid wave, and sky scattering portable WGSL shaders
    When each shader module is checked by naga
    Then all built-in shaders pass W3C WGSL parsing, type validation, and entry-point verification
    And malformed WGSL or a missing entry point returns ShaderValidationError

  @VE-06 @build
  Scenario: Symplectic Euler rigid bodies, capsule controllers, and arc-length kinematic spline movers step deterministically and refuse invalid substeps or non-finite timesteps
    Given a physics world with buoyant rigid bodies, a first-person capsule controller, and a closed-loop kinematic spline mover
    When the simulation steps across fixed frames
    Then rigid bodies, player kinematics, and spline mover traversal advance deterministically
    And non-positive mass or non-finite timesteps return PhysicsStepError

  @VE-07 @build
  Scenario: Spatial audio listener and modal bar emitters synthesize deterministic stereo buffers with distance attenuation and Sabine RT60 reverberation
    Given reflective chamber and damped exterior acoustic environments with Euler-Bernoulli modal bar emitters
    When binaural Woodworth cues and modal frequencies are evaluated
    Then interaural time/level differences, modal ratios, and Sabine RT60 decay times match analytical acoustics

  @VE-08 @build
  Scenario: Browser worker job graphs and frame-budget profilers validate acyclic schedules and refuse cyclic dependencies or invalid engine configurations
    Given a 60Hz browser engine configuration and multi-worker frame DAG
    When the job graph schedules frame stages and records timings
    Then acyclic frame schedules meet the 16.6ms budget
    And zero viewport dimensions or zero worker threads return EngineConfigError

  @VE-09 @build
  Scenario: Binary IEEE-754 world snapshots with SHA-256 trailers round-trip bit-identically and refuse truncated or tampered payloads
    Given a multi-LOD 3D scene state at lossless resolution
    When encoded to a VDNT binary snapshot and decoded back
    Then every scalar, node preset, and entity transform round-trips bit-identically
    And truncated or byte-tampered snapshots return SnapshotCodecError

  @VE-10 @build
  Scenario: The compiled Core-Wasm engine verifier proves monotonic UORC streaming and exact tensor kernel bounds in Lean without Classical.choice and answers queries deterministically
    Given app/src/Tensor.lex.tex and app/src/Engine.lex.tex
    When checked against site/app.json and LexLean axioms
    Then monotonic UORC streaming, seven-kernel accumulator validity, and invalid contraction refusal theorems hold with zero Classical.choice axioms
    And arithmetic overflow in the compiled kernel returns ComputeError

  @VE-11 @build
  Scenario: Skeletal joint hierarchies, normalized four-joint linear-blend skinning weights, and clamped keyframe clips deform procedural meshes deterministically and refuse cyclic joints or unnormalized weights
    Given a multi-joint skeletal rig, normalized four-joint skinning weights, and keyframe poses
    When keyframes are sampled and linear-blend skinning deforms a procedural mesh
    Then vertex positions and normals deform smoothly via the Prism-compiled interpolation kernel
    And cyclic joint parents or unnormalized skin weights return AnimationError

  @VE-12 @build
  Scenario: Four-cascade shadow frustum splits, GGX image-based lighting mip chains, five-pass acyclic render barrier graphs, and skinned shadow WGSL shaders validate cleanly and refuse invalid frusta or cyclic pass dependencies
    Given perspective near/far bounds, an environment probe resolution, and the five-pass PBR render graph
    When cascaded shadow splits, IBL mip levels, render barriers, and skinned shadow WGSL shaders are validated
    Then monotonic four-cascade splits, mip counts, acyclic barriers, and W3C WGSL modules pass validation
    And inverted frustum bounds or backward pass dependencies return RenderGraphError

  @VE-13 @build
  Scenario: Bounded 3D GJK convex hull collision detection, Baumgarte contact stabilization, and Coulomb friction cones resolve rigid-body contacts deterministically and refuse degenerate hulls
    Given convex polyhedral colliders in 3D world space
    When GJK Minkowski support intersection and sequential-impulse contact resolution execute
    Then overlapping hulls yield contact manifolds and Coulomb-clamped impulses via the Prism kernel
    And empty or non-finite convex hulls return CollisionError

  @VE-14 @build
  Scenario: Slope-filtered terrain navigation meshes and admissible A-star corridor pathfinding steer autonomous agents around obstacles and refuse unwalkable endpoints or disconnected goals
    Given a procedural terrain mesh with walkable slopes and steep cliff obstacles
    When a slope-filtered NavMesh is extracted and A-star corridor search runs
    Then agents receive deterministic multi-polygon corridors and unit steering vectors
    And disconnected polygon targets or out-of-bounds indices return NavigationError

  @VE-15 @build
  Scenario: The WebAssembly worker runtime bridge steps deterministic 3D physics, renders portable tensor framebuffers, encodes and restores SHA-256 VDNT world snapshots within Prism envelope bounds, and refuses invalid LOD ranks, ring overflows, or tampered payloads
    Given a WasmRuntimeBridge initialized at lossless LOD3 resolution
    When worker frames step, portable tensor framebuffers render, and VDNT snapshots are encoded and restored
    Then spline mover kinematics, BVH visibility, RGBA8 tiles, and restored world state remain bit-exact
    And invalid LOD ranks, command ring overflows, or tampered snapshot bytes return WasmBridgeError

  @VE-16 @build
  Scenario: The verdant CLI packs multi-LOD 3D scenes into validated progressive UORC streams, binary glTF 2.0, and SHA-256 VDNT snapshots, verifies portable WGSL shaders and headless benchmarks, and refuses invalid commands or corrupted assets
    Given the verdant CLI scene packer and asset validation subcommands
    When pack-scene, verify-uorc, validate-glb, validate-wgsl, snapshot-verify, and benchmark execute
    Then exported UORC streams, GLB containers, WGSL shaders, and VDNT snapshots pass authoritative checks
    And unknown subcommands, malformed shaders, or corrupted UORC/GLB files return CliCommandError

  @VE-17 @build
  Scenario: Unified verdant-tensor semiring contractions execute linear transforms, four-joint skinning, Delassus effective-mass operators, tropical min-plus navmesh shortest paths, and Hadamard FDN audio feedback deterministically and refuse mismatched dimensions
    Given the pure verdant-tensor semiring substrate backed by uor-matmul
    When linear transform, skinning, Delassus Jacobian-mass, tropical min-plus navmesh, and Q15 Hadamard FDN passes execute
    Then every subsystem contraction matches exact semiring invariants across 9 certified kernels
    And mismatched Jacobian or transform tensor shapes return TensorComputeError

  @VE-18 @build
  Scenario: Portable UORC progressive streams decode natively via uorc_decompress and uor_matmul coded_gemm contracts Grid geometry, Packed animation, and Runs IBL matrices without float expansion while refusing corrupted archives
    Given a stateless four-LOD portable UORC stream and CodedLodFrameBundle tiers
    When uorc_decompress decodes progressive LOD frames and coded_gemm contracts Grid, Packed, and Runs matrices
    Then decoded StandardSceneState snapshots and integer coded accumulators match bit-identically
    And truncated or tampered portable UORC frames return StreamingError

  @VE-19 @build
  Scenario: Khronos glTF 2.0 scene import, glb-to-UORC transcoding, XPBD articulated joints, swept capsule and four-wheel raycast vehicle controllers, and BVH acoustic occlusion execute deterministically and refuse invalid joint bodies
    Given exported Khronos GLB scenes, articulated XPBD joint rigs, four-wheel raycast vehicles, and BVH occluders
    When GLB import, progressive UORC transcoding, physics stepping, and occluded binaural raycasts run
    Then imported hierarchies, transcoded UORC streams, joint constraints, vehicle suspension, and occluded audio gains verify cleanly
    And joints referencing out-of-bounds rigid bodies return PhysicsStepError

  @VE-20 @build
  Scenario: SHA-256-chained deterministic input journals, bounded 64-frame rollback resimulation sessions, and stateless XOR-residual delta UORC snapshots synchronize game state bit-identically and refuse regressive or out-of-window frames
    Given a deterministic RollbackSession and sequential StandardSceneState snapshots
    When frame inputs advance, historical inputs are corrected within the 64-frame window, and delta UORC snapshots round-trip
    Then resimulated physics trajectories and decoded delta snapshots match fresh execution bit-identically
    And regressive input frames or out-of-window rollback targets return SnapshotCodecError

  @VE-21 @build
  Scenario: Formal 2D and diegetic 3D UI trees solve via tropical and linear semiring contractions with proved 48px touch targets, WCAG 2.2 AA contrast, and focus reachability while refusing non-compliant widgets
    Given the canonical Verdant Studio 2D and diegetic 3D UiTree
    When tropical_ui_box_extents, tropical_min_plus_all_pairs_closure, and uiWidgetCompliant solve and verify the layout
    Then all widgets resolve without overlap, meet 48px target and WCAG 2.2 AA contrast bounds, and index into a diegetic BVH
    And sub-48px targets, low-contrast widgets, or unreachable focus nodes return UiComplianceError

  @VE-22 @build
  Scenario: Second-order cybernetic LIF state-space observers, embedded coded_gemm SLM intent routers, and denial-wins browser container capability gates execute deterministically and refuse denied organs or invalid prompts
    Given a CyberneticObserverLoop, CodedSlmDirector, and ContainerGrantMatrix
    When player gaze and cadence observations step LIF state-space recurrence and natural-language prompts route through coded_gemm
    Then mutual-attention salience, gimbal orientation, and admitted EngineProposal intents reproduce bit-identically
    And explicitly denied container organs or empty prompts return CyberneticError

  @VE-23 @build
  Scenario: World-space SceneWorld hierarchies and imported Khronos GLB assets pack directly into zero-copy WebAssembly vertex, normal, color, and index buffers shared across WebGL2 and 16x16 software tile rasterization
    Given a multi-entity SceneWorld and an imported Khronos GLB scene
    When pack_world_meshes_to_buffers propagates exact world matrices and packs vertex, normal, color, and index buffers
    Then both hardware WebGL2 buffers and 16x16 software tile PBR rasterization render non-empty world-space geometry
    And empty mesh libraries return TensorComputeError

  @VE-24 @build
  Scenario: Prism ID signed input journal batches, 2-of-3 SHA-256 XOR Stitch erasure fragments, and survival-entropy peer trackers recover exact UORC world state and refuse tampered shards, duplicate indices, or forged signatures
    Given a PrismIdSessionEnvelope signed input journal and a UORC world stream split into three Stitch fragments
    When any two distinct fragments reconstruct the stream and peer liveness observations update SurvivalEntropyTracker
    Then all three 2-of-3 pairs recover bit-identical SHA-256 verified bytes and refresh urgency tracks live replica count
    And duplicate fragment indices, corrupted parity bytes, expired observations, or forged signatures return StitchCustodyError

  @VE-25 @build
  Scenario: Two-level entity TLAS and triangle MeshBlas acceleration structures execute continuous swept-capsule collide-and-slide, stair auto-stepping, and acute two-plane crease projection with zero wall penetration
    Given a SceneWorld with a hollow doorway mesh, an acute two-wall corner, and stepped stair risers indexed in TwoLevelBvh
    When PhysicsWorld::step_lawful sweeps a CapsuleController through the open doorway, into the acute corner, and up the stairs
    Then the capsule passes cleanly through the hollow opening, slides or halts without penetrating solid walls, and climbs walkable risers
    And a narrow rail between the capsule end caps still blocks motion
    And post-step sweptClearanceValid holds across every frame

  @VE-26 @build
  Scenario: Lawful spring-arm and first-person camera rigs sweep an exact near-frustum diagonal guard sphere against TwoLevelBvh and terrain so the near clip plane never slices through walls or floors and refuse invalid frusta
    Given a LawfulCameraRig orbiting an anchor inside a walled room and over uneven terrain
    When solve_frame sweeps the near-frustum diagonal guard sphere across 360-degree yaw and steep pitch angles
    Then cameraNearSphereGuarded holds and the camera eye never crosses a wall or dips within guard_radius_meters of any solid surface
    And an imported solid mesh blocks the camera until cleared, while the same mesh declared as a fluid permits passage
    And non-positive clip planes, out-of-range fields of view, or excessive spring-arm distances return CameraRigError

  @VE-27 @build
  Scenario: High-definition vaulted Citadel halls, parabolic stone arch bridges, sculpted multi-octave terrain, and canopy clusters scale monotonically from LOD0 to LOD4, pass Khronos glTF 2.0 validation, and admit lawful portal and stair traversal
    Given constructive HdSectorBundle scenes synthesized from LOD0 through ultra-high-definition LOD4
    When meshes validate, export to Khronos binary glTF 2.0, and PhysicsWorld::step_lawful traverses the Citadel stairs and arched portal
    Then triangle counts scale strictly monotonically above 45,000 triangles at LOD4, GLB export passes the Khronos oracle, and the capsule walks cleanly into the vaulted hall

  @VE-28 @build
  Scenario: Twenty-two-joint articulated humanoid characters deform via normalized four-joint linear-blend skinning and morph targets, adapt to stairs via analytical two-bone Foot IK, mask head geometry in first-person possession, and bind external GLB meshes
    Given a 22-joint ArticulatedCharacter with normalized LBS weights and three morph target channels
    When step_and_solve_ik rays feet onto asymmetric stair risers, skin_mesh_linear_blend deforms the mesh, and first-person vs third-person buffers pack
    Then knee flexion adapts asymmetrically, skinned normals remain unit-length, first-person mode hides head triangles, and empty external meshes return AnimationError

  @VE-29 @build
  Scenario: Floating-origin UniversePosition cell coordinates wrap local offsets into [-64m, +64m)^3 with sub-millimeter camera-relative f32 precision at astronomical sector indices, traverse momentum-preserving PortalGraph arches, and pack multi-sector Stitch universes
    Given a UniversePosition placed at an astronomical sector index and a canonical three-sector UniverseManager with SpatialPortals
    When local offsets translate across 128m cell boundaries, camera-relative matrices cast to f32, portals traverse with velocity, and verdant pack-universe runs
    Then universeLocalOffsetBounded holds on every step, camera-relative f32 error stays below 0.0001m, portal velocity rotates cleanly, and Stitch sector shards emit

  @VE-30 @build
  Scenario: StoryAutomaton and ConservationLedger verify tropical reachability and exact integer resource conservation across NPC dialogues, SLM proposals, and CLI audits and refuse deadlocked graphs, item duplication or underflow, out-of-range guards, and invalid universe portals
    Given a canonical Citadel Chronicle StoryAutomaton with a seeded ConservationLedger and CodedSlmDirector
    When spatial and SLM-guided interactions advance through the Chronicle and verdant verify-story audits reachability and conservation
    Then every live state reaches a milestone with conserved item supply across all transitions
    And deadlocked graphs, ledger underflow, out-of-range guards, or invalid portal definitions return NarrativeLawError

  @VE-31 @build
  Scenario: OntologicalRuntimeGovernor synthesizes constructive W5H certificates across all eight causal strata, evaluates HostEnvelope and WitnessEnvelope telos loss with non-coercive LOD0-to-LOD4 descent, and closes every Prism subsystem and Core-Wasm acceptance query
    Given an OntologicalRuntimeGovernor bound to a 60Hz EngineConfig and BrowserCapabilities
    When certify_all_eight_strata, evaluate_frame_telos, recommend_non_coercive_lod, and Core-Wasm dispatch execute across healthy, over-budget, and coercive-locked states
    Then all eight causal strata pass W5H verification with triadic agency product 30, healthy frames achieve zero telos loss and ascend to LOD4, over-budget frames descend unless coercively pinned, and summary:hd, lod:4, and scene:telos answer conformant

  @VE-32 @build
  Scenario: Verdant Studio themes, six-tab game-creation workbench, seven executable stakeholder journeys, four starter archetypes, and six live WebAssembly documentation embeds satisfy W3C DTCG token purity, Core-Wasm verification, and zero-error C-ABI export parity
    Given the Verdant Studio frontend modules, Diataxis live-embed harness, compiled WebAssembly binaries, and Core-Wasm guest kernel
    When scene:studio dispatches, Wasm export tables are audited against JS call sites, and Studio CSS and C-ABI outliner/CSM helpers execute
    Then Core-Wasm certifies 3 themes, 6 workbench tabs, 7 stakeholder journeys, 4 starter archetypes, 6 live doc embeds, and 13 studio controls, zero raw hex overrides appear in Studio stylesheets, and every cold-boot C-ABI call site resolves in verdant-wasm.wasm

  @VE-33 @build
  Scenario: Hoisted Stratum-0 Prism constants, 256-entry CodebookProductLut contractions, 64-bit word-packed SoaSceneTable sweeps, contiguous 32-byte SAH FlatBvh traversal, epoch-split static and dynamic WebGL2 VAOs without gl.finish stalls, and hardware drawElementsInstanced batching scale deterministically to arbitrary 10k-entity and 50k-triangle workloads
    Given hoisted VerifiedKernelConsts, a 256-entry CodebookProductLut, a 10000-entity word-packed SoaSceneTable, a LOD4 45000-triangle FlatBvh TwoLevelBvh, and the WebGL2 worker pipeline
    When Core-Wasm scene:perf dispatches, dirty-transform and frustum bitmasks sweep 10000 entities, 1000 raycasts and 100 sphere sweeps execute, and 10000 hardware instances stage in Wasm
    Then FlatBvhNode is 32 bytes, average BVH node visits stay under 2 percent of total nodes, static epochs remain stable across dynamic steps, gl.finish is absent, and verdant stress-benchmark succeeds

  @VE-34 @build
  Scenario: Prism algebraic monoid command canonicalization, eight-column 64-bit bit-plane archetype matrices, in-place tropical min-plus closure with next-hop paths, O(P) hash-indexed NavMesh extraction, sparse two-body XPBD Delassus solves, stride-8 rollback checkpointing, and content-addressed kappa transition memoization scale deterministically and refuse invalid coded or cyclic inputs
    Given the Core-Wasm scene:aqm query, a burst of redundant and self-inverse input commands, a 16384-entity ColumnBitPlaneTable, an HD walkable NavMesh, a multi-body XPBD rig, a stride-8 RollbackSessionJournal, and a KappaTransitionCache
    When algebraic monoid fusion, Kahn parallel layer scheduling, column bit-plane popcount queries, in-place tropical next-hop closure, sparse XPBD stepping, and coded/UORC admission gates execute
    Then Core-Wasm scene:aqm answers conformant, self-inverse and modular yaw commands fuse cleanly, 16384-entity bit-plane intersections match exact counts, tropical next-hop paths match A-star corridors, kappa transition hits elide redundant steps, and cyclic DAGs or float-decoded admissions are refused

  @VE-35 @build
  Scenario: Flat CSR prefix-sum software tile rasterization, spatial-grid multi-body broadphase collisions, multi-primitive Khronos GLB ingestion, WebGL2 context-loss recovery, deadlock-free worker frame-ack and RPC watchdogs, and 52-bit floating-origin horizon guards execute with zero steady-state inner heap allocations and refuse astronomical or degenerate inputs
    Given the Core-Wasm scene:stability query, a reusable TileRasterScratch CSR tile bin table, a 512-body PhysicsWorld spatial broadphase, a multi-primitive Khronos GLB asset, and the WebGL2 worker surface
    When CSR tile rendering runs across normal and near-clip geometry, 512 dynamic rigid spheres collide, multi-primitive GLB meshes import, and non-finite or beyond-52-bit sector coordinates are tested
    Then Core-Wasm scene:stability answers conformant, CSR tile capacities stabilize without per-frame inner Vec allocations, colliding spheres separate cleanly, all GLB primitives preserve geometry, WebGL2 context recovery and worker watchdogs are wired, and astronomical cell shifts return NarrativeLawError

  @VE-36 @build
  Scenario: Every post-boot engine boundary—world snapshots, #uorc= shareable URLs, OPFS GLB transcoding, Stitch 2-of-3 parity custody, rollback checkpoints, CLI archives, and zero-decode CodedMatrix bundles—operates exclusively over SHA-256-sealed UORC archives across native Rust and WebAssembly while refusing unsealed or tampered payloads
    Given the Core-Wasm scene:uorc-exclusive query, the portable UORC object codec in native Rust and WebAssembly, a LOD3 StandardSceneState, an external GLB asset, a Stitch 2-of-3 shard set, a CheckpointedRollbackJournal, and a CodedLodFrameBundle
    When WorldSnapshot::encode_uorc, Wasm snapshot_encode, GLB-to-UORC transcoding, Stitch UORC sharding, rollback checkpoint compression, zero-decode CodedMatrix UORC contraction, and verdant compress-uorc execute
    Then Core-Wasm scene:uorc-exclusive answers conformant, every post-boot wire payload carries the UORC magic header at a fraction of raw byte size, zero-decode contractions match exact accumulators without float expansion, Studio emits #uorc= URLs and .uorc OPFS caches, and tampered UORC seals are refused

  @VE-37 @build
  Scenario: Every packed vertex carries an explicit surface class derived from its PBR material rather than inferred from hue, dynamic actors stream positions, normals, and colours with per-attribute GPU capacity, and shading derivatives run in uniform control flow with a continuous detail fade, so characters render with their own colours, water, metal, and emissive glass stay distinct, and no seam line crosses the terrain
    Given the canonical PBR material library, a LOD2 HD Citadel sector, the four character archetypes, a live WasmRuntimeBridge, and site/scene-renderer.mjs
    When the sector, every archetype, and the bridge's dynamic actors are packed into GPU vertex streams and the WebGL2 PBR shader decodes them
    Then only the basin water is fluid, armour is opaque and visors are emissive glass, gold accents are metal, every class round-trips through the packed red channel, tile triangles decode plain albedo, the shader reads a flat class varying with uniform-flow derivatives and a 22-30 m detail fade, and the dynamic VAO keeps separate position, normal, colour, and index capacities

  @VE-38 @build
  Scenario: Imported Schema.org, W3C SOSA and PROV-O, Khronos glTF 2.0 HD PBR and skeletal skinning, W3C naga WebGL2 GLSL ES 3.00 transpilation, OMG DMN 1.5 model admission, and multi-domain world catalogs validate against SHA-256-locked external authority oracles while prohibiting external model runtimes outside Verdant Engine
    Given the five SHA-256-locked catalog corpora in catalogs/, the Khronos gltf and W3C naga external crates, a 22-joint ArticulatedCharacter, and the repository manifests
    When CatalogBundleRegistry loads and cross-validates every domain pack, Schema.org JSON-LD and W3C SOSA/PROV-O graphs are verified, skinned characters and HD PBR materials export to GLB, portable WGSL shaders transpile to WebGL2 GLSL ES 3.00, and OMG DMN 1.5 model admission and runtime exclusivity execute
    Then every catalog and domain pack passes its external validation oracle, skinned GLB exports preserve all 22 joints and KHR_materials extensions, naga emits valid GLSL ES 3.00, zero external model runtimes exist in Verdant, and malformed graphs, cyclic PROV-O lineage, or external ONNX/Transformers runtimes return OracleValidationError

  @VE-39 @build
  Scenario: The second-order Observer Prism holon composes strong kappa identities for itself, its five differentiated platform organs, its seven stakeholder Prism models, arbitrary Verdant model envelopes, native uor-matmul 16-route intent and ISO 24617-2 passage-grounded NPC dialogue with Unknown abstention, live NavMesh NPC directives, and five W3C SOSA/PROV-O observed communication channels across native Rust and WebAssembly
    Given Observer.lex.tex compiled into site/app.holo and prism_kernel.rs, ObserverPrismHolon, VerdantLanguageEngine, and WasmRuntimeBridge
    When the Observer inspects its five organs and seven stakeholder Prism models, admits arbitrary Verdant model envelopes, routes all 16 Studio and Engine intents via coded_gemm, evaluates grounded and unstated NPC questions, steps NavMesh NPC patrol directives, and observes Organ, Player-to-Player, Player-to-Agent, and Agent-to-Agent communications
    Then Core-Wasm scene:observer, scene:stakeholders, scene:language, and scene:channels answer conformant, every part and stakeholder carries a distinct 32-byte kappa identity, grounded questions cite cataloged facts while unstated questions abstain with Unknown, NPCs steer along walkable A-star corridors, all communications record valid Schema.org and W3C SOSA/PROV-O lineage, and unsigned player channels, ungrounded assertions, or non-Verdant model envelopes return ObserverModelError

  @VE-40 @build
  Scenario: Every Verdant surface—Verdant Studio, Verdant Catalogs, Verdant Engine Demo, the browser and Rust starter template, all four Studio Scaffolder archetypes, and every crate example—instantiates differentiated SHA-256-locked catalog world packs, Schema.org JSON-LD couplings, live Wasm NPC directives and ISO 24617-2 dialogue, and zero-bypass Wasm CodedSlmDirector routing with zero magic constants
    Given site/studio.mjs, site/engine-worker.mjs, site/prism-catalogs.mjs, site/demo, site/studio-scaffolder.mjs, templates/browser-3d-starter, and crates/verdant-core/examples
    When Studio dispatches prompts and NPC directives and dialogue to Wasm, Demo and Starter load cataloged world packs, Catalogs and pages emit Schema.org JSON-LD, and all templates, scaffolded archetypes, and examples are audited
    Then Studio contains zero handwritten JS regex prompt overrides and zero stakeholder/persona UI jargon, Worker exposes load-domain-pack, set-npc-directive, npc-dialogue, observer-status, and observe-comm over Wasm, LOD transitions preserve the active catalog domain pack, all starter, scaffolder, and example Rust sources instantiate ObserverPrismHolon or CatalogBundleRegistry with zero magic constants, and out-of-bounds domain or character indices return ObserverModelError

  @VE-41 @build
  Scenario: UHD 4K-to-8K scaling, 30-bit Morton spatially-clustered 64-triangle GPU meshlet culling with zero steady-state heap reallocations, 2x2 box-filtered multi-material PBR texture atlas packing, naga-verified WGSL compute culling and PBR shaders, zero-reallocation GPU index streaming, and the 64-frame hysteresis FrameBudgetOracle execute deterministically across native Rust, WebAssembly, and arbitrary 45,000-triangle LOD4 workloads
    Given a LOD4 45,000-triangle Citadel sector, interleaved multi-cluster geometry, multi-material PBR textures, the naga WGSL validator, FrameBudgetOracle, WasmRuntimeBridge, and site/scene-hd-renderer.mjs and site/scene-webgpu-renderer.mjs
    When build_meshlet_graph Morton-clusters triangles, evaluate_meshlet_visibility culls across repeated frames, pack_rgba_texture_atlas downsamples 4 and 16 materials, validate_uhd_shaders compiles WGSL, evaluate_uhd_display_profile evaluates 1080p through 8K, FrameBudgetOracle processes 1000 frames of borderline jitter and overload-recovery cycles, and Wasm UHD C-ABI exports execute
    Then Morton clustering tightens bounding radii, steady-state meshlet culling holds constant buffer capacity with zero heap reallocations and deterministic visibility ratios, box-filtered atlases respect host caps without seam bleed, naga validates cs_cull_meshlets and vs_uhd_pbr/fs_uhd_pbr, hysteresis DRS avoids thrashing on borderline jitter while stepping smoothly on sustained load, WebGL2 and WebGPU index streaming use sub-buffer updates without VRAM reallocation, and empty or degenerate inputs fail safely without panic

  @VE-42 @build
  Scenario: SHA-256-locked Stafford Beer VSM (S1-S5 + S3*) and ISO/IEC/IEEE 42010:2022 Viewpoint + ISO/IEC/IEEE 12207 ALM Managers enforce Total Ownership across every Verdant route, shipped crate, Observer part, catalog stream, and authority corpus, disallow contrived ungrounded or cross-contaminated directives, synthesize declarative Stitch App Catalog and Viewpoint contracts, and expose a signed bidirectional Operator Communications Channel across native Rust CLI and WebAssembly
    Given SHA-256-locked AUTH-VSM-CYBERNETICS and AUTH-ISO-42010-VIEWPOINTS catalogs, VerdantOperatorChannel, ObserverPrismHolon, verdant-cli operator commands, and WasmRuntimeBridge
    When Total Ownership audits all shipped routes, crates, Observer parts, streams, and authorities, signed Operator directives route to owning S1 Managers, Managers synthesize declarative Viewpoint and Stitch App Catalog manifests, and contrived or cross-contaminated directives are submitted
    Then every Verdant part maps cleanly to exactly one accountable Manager with zero orphans, in-viewpoint standards-backed prompts return signed ISO 24617-2 Inform plans with W3C PROV-O lineage, S3* audit passes across the live repository, contrived ungrounded or cross-contaminated prompts are formally refused citing the violated rule, and planted unmanaged orphan parts fail the Total Ownership check

  @VE-43 @build
  Scenario: Zero-account Prism-ID Encrypt-then-MAC boundary privacy with revocation epoch rotation, k-of-n systematic Reed-Solomon GF(2^8) erasure coding and autonomous parity repair, monotonic quorum release pointers, budgeted reciprocal Stitch shard admission, and in-Verdant W3C PROV-O Merkle revision control (VerdantVcs) uphold complete CIA and STRIDE security invariants across native Rust, WebAssembly, and Verdant MCP while refusing unauthorized decryption, revoked-epoch replay, pointer rollback, and over-budget floods
    Given AUTH-STITCH-SELF-HOSTING, PrismBoundaryCipher, ReedSolomonStitchCodec, StitchMonotonicPointer, StitchBudgetLedger, VerdantVcs, and verdant mcp
    When an accountless stakeholder encrypts private content before 4-of-8 Reed-Solomon sharding, drops 4 shards and recovers plus repairs missing parity, rotates the boundary epoch upon collaborator revocation, advances a monotonic quorum pointer, commits and checks out revisions in VerdantVcs, and submits hostile tampered shards, wrong-key reads, revoked-epoch replays, pointer rollbacks, and over-budget floods
    Then raw shards expose zero plaintext, any k-of-n subset reconstructs and repairs bit-for-bit, VerdantVcs preserves acyclic W3C PROV-O lineage over .uorc capsules, and unauthorized decryption, revoked-epoch access, rollback pointers, and over-budget shard bursts all return StitchCustodyError

  @VE-44 @build
  Scenario: Self-contained Verdant Release .holo capsules lift all verified Viewpoints, DTCG assets, Manager manifests, and WebAssembly runtimes into 4-of-8 STCH2 Reed-Solomon shards bound by quorum-signed monotonic FramePinManifest pointers, and the immutable <= 30 KiB Composable Blank Frame recovers and mounts them via ServiceWorker Virtual FS with zero loose UI files while refusing tampered capsules, pointer rollbacks, and unsafe deep-link URIs
    Given VerdantReleaseCapsule, StitchRsShard STCH2 binary framing, FramePinManifest, parse_stitch_deep_link_uri, VerdantOperatorChannel, and site/frame/
    When Verdant lifts its verified site into a portable .holo release capsule, encodes 4-of-8 STCH2 binary shards, drops 4 shards and recovers the capsule, audits the Composable Blank Frame footprint and file isolation, and submits tampered capsule bytes, rolled-back release epochs, and hostile javascript:/traversal deep-link URIs
    Then the Blank Frame stays at or below 30 KiB with zero loose UI files, 4-of-8 binary shard recovery restores every lifted file bit-for-bit matching its SHA-256 manifest and quorum pointer, valid stitch:// deep links normalize cleanly, and tampered bytes, rollback epochs, or unsafe URIs are strictly refused

  @VE-45 @build
  Scenario: Thirteen accountable VSM Managers enforce Total Ownership across every Verdant surface and every Prism stratum including Knowledge/Provenance, Change/Version, Security, Architecture, and Stakeholder Roadmap/KPI Managers, enabling the Observer to compute live ISO 15939 KPIs and Major/Minor stakeholder roadmap progress from its parts and advance SemVer Major, Minor, or Patch versions exclusively when the Ratchet is applied and all Release Gates pass
    Given AUTH-SEMVER-CHANGE-KPI, all 13 VSM Managers in iso42010-viewpoints.json, ChangeVersionManager, KnowledgeProvenanceManager, RoadmapKpiManager, VerdantOperatorChannel, and Observer.lex.tex
    When Total Ownership audits every Verdant route, crate, Observer part, stream, authority, and all 9 Prism strata/governance facets, the Observer evaluates live ISO 15939 KPIs and M1-M5 stakeholder roadmap progress across its parts, and the Change Manager evaluates Major, Minor, Patch, and Hold release candidates with and without the Ratchet or passing Release Gates
    Then all 13 Managers own 100 percent of Verdant and Prism facets with zero orphans or duplicates, the Observer reports accurate KPI pass rates and Major/Minor progress percentages from its Knowledge and Roadmap Managers, ChangeVersionManager advances SemVer Major/Minor/Patch versions only when the Ratchet is applied and all Release Gates pass, and unratcheted or gate-failing candidates are strictly held without incrementing the version

  @VE-46 @build
  Scenario: Verdant lifts 100 percent of its shipped Rust crates, all 26 Prism LexLean strata, all governance and authority registers, and all distilled Viewpoints into a 4-of-8 STCH2 Reed-Solomon sharded VerdantVcs Source DAG with zero unlifted workspace modules, recovers the full Source DAG from 4 surviving shards, and autonomously bootstraps a chained v(N+1) release through mgr-change-release with acyclic PROV-O lineage and monotonic prev_kappa chaining
    Given VerdantOperatorChannel, VerdantVcs, VerdantReleaseCapsule, ChangeVersionManager, and Observer.lex.tex
    When Verdant lifts its complete workspace into a 4-of-8 sharded VerdantVcs Source DAG, drops 4 of 8 source shards, recovers and verifies the Source DAG, and bootstraps a chained v(N+1) release through mgr-change-release with and without the Ratchet applied
    Then unlifted workspace modules equal zero, all 26 LexLean strata and shipped Rust crates are Merkle-verified inside VerdantVcs, 4 surviving shards reconstruct the full source DAG and mint a chained acyclic child commit plus v(N+1) .holo release pinned to prev_kappa_hex, and unratcheted v(N+1) bootstrap attempts are strictly refused

  @VE-47 @build
  Scenario: Verdant autonomously lifts itself via verdant operator ratchet-lift and MCP verdant_ratchet_lift—synthesizing all 13 Manager manifests, lifting 100 percent of shipped Rust, Prism LexLean, and Viewpoint UI modules into VerdantVcs, packing and 4-of-8 verifying the .holo release capsule, and emitting site/generated/self-host-ratchet-manifest.toml—so the tightened prism/parametric-ratchet/2 audit enforces zero unlifted UI files, zero loose Blank Frame assets, zero external user accounts, and zero unlifted workspace modules while failing closed on any out-of-band edit
    Given VerdantOperatorChannel, prism-fidelity parametric-ratchet v2, site/frame Blank Frame, and 13 accountable VSM Managers
    When Verdant runs execute_autonomous_ratchet_lift via verdant operator ratchet-lift or MCP verdant_ratchet_lift and prism-fidelity audits model/parametric-ratchet.toml
    Then Verdant produces site/generated/self-host-ratchet-manifest.toml with exact live SHA-256 digests for every shipped Rust, LexLean, and Viewpoint UI file, verifies 4-of-8 .holo recovery, satisfies all 4 zero self-hosting ceilings, and fails closed on any out-of-band source edit or loose Blank Frame asset

  @VE-48 @build
  Scenario: MultiOriginRelayPlan and StitchPeerExchangeSession enforce at least two independent HTTPS Stitch shard origins with parallel failover racing and StitchBudgetLedger-governed zero-account WebRTC/BroadcastChannel reciprocal peer shard seeding, refusing single-origin downgrade, unsafe schemes, tampered peer shards, and over-budget floods
    Given MultiOriginRelayPlan, StitchPeerExchangeSession, StitchBudgetLedger, site/frame/frame-boot.mjs, and site/frame/frame-pin.json
    When a Blank Frame races multiple independent Stitch shard origins with primary relay outage and exchanges verified STCH2 shards across browser peers under reciprocal budget limits
    Then failover recovers 100 percent of the .holo capsule from surviving origins, peers exchange verified shards strictly within reciprocal credit limits, and single-origin plans, unsafe schemes, tampered shards, and over-budget requests are refused

  @VE-49 @build
  Scenario: Verdant Studio exposes zero-account in-browser VerdantVcs commits with acyclic parent-to-child revision history, one-click self-verifying portable .holo app export, and encrypted 4-of-8 ?pin=stitch:// deep-link sharing across WebAssembly and Blank Frame without requiring any external user account or CLI
    Given verdant-wasm VCS/holo exports, Verdant Studio Share panel, local Prism ID key, and Composable Blank Frame
    When a creator authors a 3D scene in Studio, commits v1 and v2 to VerdantVcs, exports a portable .holo capsule, and generates a ?pin=stitch:// deep-link share URL
    Then VerdantVcs maintains verified acyclic commit history, the exported .holo capsule passes verifyAndUnpackHoloCapsule with every SHA-256 intact, and the deep link mounts cleanly with zero external accounts

  @VE-50 @build
  Scenario: Live browser 4K and 8K UHD rendering, multi-MB Khronos glTF 2.0 scenes, and quantized .uorc CodedMatrix AI workloads execute through PagedTensorArena and DynamicWorkloadGovernor in WebAssembly with bounded RAM ceilings, live telemetry in Studio, and zero main-thread lockups verified by an automated headless Chromium UHD workload gate
    Given PagedTensorArena, DynamicWorkloadGovernor, verdant_wasm_paged_workload_step_aux, Verdant Studio, and scripts/verify-uhd-workload.mjs
    When 4K (3840x2160) and 8K (7680x4320) 45000-triangle glTF and CodedMatrix AI workloads run through WebAssembly in headless Chromium
    Then PagedTensorArena enforces its bounded RAM ceiling via page eviction without freezing the browser event loop, Studio displays live dynamic scaling telemetry with zero manual resolution toggles, and the performance contract remains met

  @VE-51 @build
  Scenario: Verdant publishes a live RFC 2789 Cargo Sparse Registry endpoint for verdant-tensor, verdant-core, verdant-wasm, and verdant-cli, cites permanent Zenodo DOI archival provenance (https://zenodo.org/records/24383968), promotes Blank-Frame-first entry while preserving byte-locked frame.lock invariants, and tracks Milestones M6 through M8 across ten live ISO 15939 Observer KPIs
    Given AUTH-STITCH-SELF-HOSTING, AUTH-SEMVER-CHANGE-KPI, scripts/package-rust-crates.mjs, site/card.html, site/docs/index.html, and RoadmapKpiManager
    When Verdant packages its RFC 2789 Cargo Sparse Registry index, binds Zenodo archival provenance, exposes Blank-Frame-first entry, and evaluates Observer KPIs across Milestones M1-M8
    Then all four shipped Rust crates have valid RFC 2789 sparse index entries, Zenodo DOI archival is locked in authority provenance, frame.lock stays intact, and all ten Observer KPIs pass across M1-M8

  @VE-52 @build
  Scenario: Constructive Lean 4 modules Cadence.lex.tex and Release.lex.tex prove Verdant's autonomous release cadence and 6-channel distribution matrix with zero Classical.choice, verdant operator ratchet-lift keeps Cargo.toml workspace versions synchronized with release-ledger.json, and scripts/release/qualify-tag.mjs plus release-tag.yml enforce green-bootstrap autonomous tagging
    Given app/src/Cadence.lex.tex, app/src/Release.lex.tex, Cargo.toml, site/generated/release-ledger.json, scripts/release/qualify-tag.mjs, and .github/workflows/release-tag.yml
    When Verdant synchronizes workspace crate versions with its Change Manager ledger and runs the tag qualification gate with matching and mismatched versions
    Then Cadence and Release are grounded constructive LexLean strata, Cargo.toml matches release-ledger.json, qualify-tag passes on synchronized state, and mismatched tag or version attempts fail closed

  @VE-53 @build
  Scenario: Verdant commits its uorust MIR proof frontier register (frontier.toml) and enforces a shrink-only hand-written Rust surface ratchet inside prism-fidelity alongside prism/parametric-ratchet/2 that fails closed on any lifted-function regression or uncertified-function growth
    Given frontier.toml, model/parametric-ratchet.toml [frontier_ratchet], and prism-fidelity audit_parametric_ratchet
    When prism-fidelity audits the live repository frontier register and a synthetic regressed frontier register with an added residue function
    Then the live repository satisfies the shrink-only MIR proof frontier floors and ceilings and any added residue or lost lifted function fails closed

  @VE-54 @build
  Scenario: Verdant pins three CORS-verified independent Stitch shard origins (GitHub Pages, jsDelivr, and raw.githubusercontent via orphan stitch-mirror), verifies live RFC 2789 Cargo Sparse Registry client fetches and standalone .crate packaging via scripts/verify-sparse-registry.mjs, and delivers zero-account cross-browser ?pin=stitch://<kappa>/studio#holo=... sharing plus live VerdantVcs checkout
    Given site/frame/frame-pin.json, site/frame/frame-boot.mjs, site/studio/studio-page.mjs, scripts/verify-sparse-registry.mjs, and .github/workflows/release-stitch.yml
    When Verdant pins three CORS-verified shard origins, verifies standalone .crate packages over HTTP with real cargo, and hands off a zero-account Studio ?pin=stitch://<kappa>/studio#holo=... link to an isolated browser context
    Then all three origins are distinct HTTPS CORS-enabled relays, Cargo parses the sparse index cleanly, and the isolated browser verifies kappa and mounts the shared 3D app with zero accounts

  @VE-55 @build
  Scenario: Verdant's ratchet operates autonomously via npm run ratchet and verdant operator ratchet-lift --tighten: binding frontier.toml to live shipped Rust source SHA-256 (frontier_rust_tree_sha256), enforcing monotonic [surface_ratchet] ceilings on Wasm size, .holo capsule weight, and max hand-written Rust/JS file lines, one-way auto-tightening watermarks in model/parametric-ratchet.toml while refusing regressions, and providing lifted wrapping scalar primitives in verdant-tensor::scalars
    Given model/parametric-ratchet.toml, crates/fidelity/src/ratchet.rs, crates/verdant-core/src/ratchet_sync.rs, crates/verdant-tensor/src/scalars.rs, and scripts/verdant-ratchet.mjs
    When prism-fidelity audits the live repository, synthetic stale frontier_rust_tree_sha256 or regressed surface watermarks are tested, and verdant operator ratchet-lift runs one-way auto-tightening
    Then the live repository passes all four ratchet layers with frontier.toml bound to live Rust source SHA-256, lifted scalars satisfy both FI-08 and uorust, and stale frontier hashes or regressed Wasm/capsule/file-line watermarks fail closed

  @VE-56 @build
  Scenario: Verdant cuts site/verdant-uorc.wasm and release .holo capsule transfer weight by more than 1.5 MiB via size-optimized UORC profile tuning, expands FR-03 cold-boot headroom cleanly beneath 122880 bytes, lifts additional constructive Rust arithmetic helpers into frontier.toml, and auto-tightens [surface_ratchet] and [frontier_ratchet] watermarks monotonically
    Given Cargo.toml release profiles, site/verdant-wasm.wasm, site/verdant-uorc.wasm, frontier.toml, and model/parametric-ratchet.toml [surface_ratchet]
    When Verdant builds its WebAssembly runtimes, measures FR-03 cold-boot weight, and runs prism-fidelity audit_parametric_ratchet
    Then site/verdant-uorc.wasm is strictly below 5000000 bytes, [surface_ratchet] ceilings are monotonically tightened, and frontier.toml expands lifted constructive Rust functions while shrinking uncertified residue

  @VE-57 @build
  Scenario: Portable zero-account .holo user applications exported from Verdant Studio embed a self-contained interactive WebGL2 3D Canvas Player (crates/verdant-core/src/portable_player.rs) that renders the creator's exact authored objects, transforms, domain sky/ground palette, and orbit controls offline with zero external accounts or CDN fetches, verified end-to-end across isolated Chromium browser contexts
    Given crates/verdant-core/src/portable_player.rs, crates/verdant-core/src/vcs.rs, Verdant Studio, and scripts/verify-uhd-workload.mjs
    When a creator authors 3D objects in Studio, commits and packs a portable .holo app via studio_vcs_commit_and_pack_holo_json, and opens the ?pin=stitch://<kappa>/studio#holo=... share link in an isolated browser context
    Then the unpacked studio/index.html embeds the interactive WebGL2 3D player with safe HTML escaping, zero external network dependencies, live rendered frames across all authored 3D objects, and toggleable scene JSON inspection

  @VE-58 @build
  Scenario: Verdant decomposes its three largest monoliths (verdant-wasm/src/lib.rs, verdant-core/src/operator.rs, and site/studio/studio-page.mjs) into focused submodules that auto-tighten max_handwritten_rust_file_lines to <= 3525 and max_js_file_lines to <= 1980, gates naga behind the native wgsl-oracle Cargo feature so wasm32 builds exclude shader transpiler dependencies, syncs site/app.holo 1:1 with CI for widened FR-03 cold-boot headroom, and upgrades portable_player.rs to render all 5 Studio primitive geometries (Box, Platform, Pillar, Ramp, Beacon Orb) with ground grid and WASD/orbit controls offline
    Given crates/verdant-wasm/src/exports_cold.rs, crates/verdant-wasm/src/exports_uorc.rs, crates/verdant-core/src/operator_types.rs, crates/verdant-core/src/operator_release.rs, crates/verdant-core/src/operator_mcp.rs, site/studio/vcs-panel.mjs, site/studio/outliner.mjs, site/studio/observer-panel.mjs, site/studio/project-inspector.mjs, crates/verdant-core/Cargo.toml, and crates/verdant-core/src/portable_player.rs
    When prism-fidelity audits [surface_ratchet], verdant-core gates naga behind wgsl-oracle, and portable_player renders a multi-shape scene
    Then max_handwritten_rust_file_lines is at most 3525, max_js_file_lines is at most 1980, all decomposed submodules are owned in iso42010-viewpoints.json, and the offline WebGL2 player synthesizes all 5 Studio primitive shapes with WASD and drag-orbit navigation

  @VE-59 @build
  Scenario: Verdant expands its shrink-only uorust MIR proof frontier past 200 lifted constructive Rust functions (auto-tightening min_lifted_functions >= 200 while shrinking arithmetic and total residue) and closes all Cadence, Release, and self-hosted Stitch Security theorem verification records across app/src/{Cadence,Release,Security}.lex.tex and fidelity.toml
    Given frontier.toml, model/parametric-ratchet.toml [frontier_ratchet], fidelity.toml, app/src/Cadence.lex.tex, app/src/Release.lex.tex, and app/src/Security.lex.tex
    When prism-fidelity audits [frontier_ratchet] and fidelity.toml across all 28 canonical LexLean modules
    Then frontier.toml proves at least 200 lifted constructive Rust functions with shrunk residue and fidelity.toml closes all 705 meant verification obligations across Cadence, Release, and Security with zero Classical.choice

  @VE-60 @build
  Scenario: VerdantReleaseCapsule supports transparent RFC 1952 gzip body compression (native capsule-gzip feature + zero-dependency browser CompressionStream/DecompressionStream in Studio and Blank Frame) cutting release .holo capsule weight below 1800000 bytes with full SHA-256 and zero-trailing-byte enforcement, consolidates duplicate SHA-256/hex helpers into crates/verdant-core/src/digest.rs, and exposes deterministic VerdantVcs structural commit diffs (diff_commits) and branch ref management (create_branch, list_refs)
    Given crates/verdant-core/src/vcs.rs, crates/verdant-core/src/digest.rs, site/frame/frame-boot.mjs, site/studio/vcs-panel.mjs, and model/parametric-ratchet.toml [surface_ratchet]
    When VerdantReleaseCapsule packs and verifies gzipped and uncompressed capsules, rejects tampered or trailing bytes, and VerdantVcs diffs commits and creates branch refs
    Then max_release_capsule_bytes is at most 1800000 bytes, gzipped capsules round-trip bit-exact while rejecting tampering and trailing bytes, digest.rs is owned by mgr-security-trust, and VerdantVcs computes deterministic added/modified/removed commit diffs and branch refs

  @VE-61 @build
  Scenario: Verdant achieves 100% zero-residue formal model fidelity across all 28 canonical LexLean modules (705/705 derived, 705/705 meant, 9/9 measured, 0 open fidelity residue) with version-parametric Release.lex.tex distribution endpoints, and decomposes observer.rs, operator.rs, and conformance integration tests so max_handwritten_rust_file_lines tightens monotonically to <= 2200
    Given fidelity.toml, app/src/Release.lex.tex, crates/verdant-core/src/observer_language.rs, crates/verdant-core/src/observer_streams.rs, crates/verdant-core/src/operator_synth.rs, and model/parametric-ratchet.toml [surface_ratchet]
    When prism-fidelity scores all 28 canonical LexLean modules and audits [surface_ratchet]
    Then fidelity.toml has zero open residue (705/705 derived, 705/705 meant, 9/9 measured), Release.lex.tex uses version-parametric channel templates, and max_handwritten_rust_file_lines is at most 2200 with all decomposed submodules owned in iso42010-viewpoints.json

  @VE-62 @build
  Scenario: Verdant's Observer Executive includes a generative self-refinement engine (crates/verdant-core/src/observer_refine.rs, CLI verdant operator refine-plan, MCP verdant_observer_refine_plan, and sealed site/generated/observer-refinement-agenda.json surfaced in Studio's Observer panel) that diagnoses all 13 VSM Managers, owned source file modularity, frontier.toml proof blockers, fidelity closure, and Wasm/capsule/Blank Frame budgets while expanding the shrink-only uorust MIR proof frontier to >= 219 lifted constructive functions
    Given crates/verdant-core/src/observer_refine.rs, site/generated/observer-refinement-agenda.json, site/studio/observer-panel.mjs, frontier.toml, and VerdantOperatorChannel
    When Verdant synthesizes its generative refinement agenda via synthesize_refinement_agenda and prism-fidelity audits [frontier_ratchet]
    Then the agenda deterministically evaluates all 13 VSM Managers, parses top frontier.toml blockers, computes a reproducible agenda_kappa_hex, is sealed in immutability-seal.json, and frontier.toml proves at least 219 lifted constructive functions

  @VE-63 @build
  Scenario: Verdant lifts its entire validation, test-execution, and release-qualification lifecycle directly into Verdant (crates/verdant-core/src/lifecycle.rs, CLI verdant operator lifecycle-gate, MCP verdant_lifecycle_gate, sealed site/generated/lifecycle-gate-report.json, and qualify-tag.mjs delegation), lifts all 6 lifecycle categories (rust, verification-test, lexlean, feature-suite, model-governance, viewpoint) into VerdantVcs and self-host-ratchet-manifest.toml, and decomposes site/studio.mjs and site/studio/studio-page.mjs to auto-tighten max_js_file_lines <= 1680 with live Studio VCS commit diffs
    Given crates/verdant-core/src/lifecycle.rs, site/generated/lifecycle-gate-report.json, site/generated/self-host-ratchet-manifest.toml, scripts/release/qualify-tag.mjs, site/studio-media.mjs, site/studio/discovery-panel.mjs, and site/studio/vcs-panel.mjs
    When Verdant executes its internal lifecycle-gate on the live repository and with a mismatched release tag, and prism-fidelity audits [surface_ratchet]
    Then the internal lifecycle gate certifies all 6 lifted lifecycle categories (>= 130 lifted modules), refuses mismatched tags closed, is sealed in immutability-seal.json, and max_js_file_lines is tightened monotonically to at most 1680

  @VE-64 @build
  Scenario: Verdant lifts its entire end-to-end ratchet orchestrator directly into Verdant (execute_ratchet_cycle in crates/verdant-core/src/lifecycle.rs, CLI verdant operator ratchet-cycle, MCP verdant_ratchet_cycle, and thin scripts/verdant-ratchet.mjs delegate) and lifts all 7 lifecycle artifact categories including automation-script (scripts/**/*.mjs -> >= 150 total lifted modules) into VerdantVcs and site/generated/self-host-ratchet-manifest.toml
    Given crates/verdant-core/src/lifecycle.rs, crates/verdant-core/src/operator_mcp.rs, crates/verdant-cli/src/operator_cmd.rs, scripts/verdant-ratchet.mjs, and site/generated/self-host-ratchet-manifest.toml
    When Verdant verifies its lifted ratchet-cycle predicates, MCP tool surface, and 7-category lifted lifecycle coverage via execute_lifecycle_gate
    Then scripts/verdant-ratchet.mjs delegates directly to verdant operator ratchet-cycle, all 7 lifecycle categories (including automation-script) are lifted (>= 150 modules), and the MIR-lifted ratchet-cycle decision predicates hold constructively

  @VE-65 @build
  Scenario: Verdant executes its Observer-generated self-refinement agenda by expanding the shrink-only uorust MIR proof frontier to >= 246 lifted constructive Rust functions (shrinking residue <= 1179 and uncertified <= 1334) and decomposing site/engine-worker.mjs, site/studio/studio-page.mjs, and site/studio.mjs into owned submodules (engine-worker-codec.mjs, studio/stage-assets.mjs, studio-gameplay-panel.mjs) to auto-tighten max_js_file_lines <= 1525
    Given frontier.toml, model/parametric-ratchet.toml, site/engine-worker-codec.mjs, site/studio/stage-assets.mjs, site/studio-gameplay-panel.mjs, and catalogs/governance/iso42010-viewpoints.json
    When prism-fidelity audits [frontier_ratchet] and [surface_ratchet] and the Observer evaluates all 15 milestones (M1..M15)
    Then frontier.toml proves at least 246 lifted constructive functions with shrunk residue <= 1179, max_js_file_lines is tightened monotonically to at most 1525, all decomposed JS submodules are owned in iso42010-viewpoints.json, and all 13 Observer KPIs pass across M1..M15

  @VE-66 @build
  Scenario: Verdant lifts its full self-automation pipeline directly into Verdant (execute_automate_pipeline in crates/verdant-core/src/lifecycle.rs, CLI verdant operator automate, MCP verdant_automate, and thin scripts/verdant-automate.mjs delegate), diagnoses owned crate source files accurately inside ObserverRefinementAgenda, and expands the shrink-only uorust MIR proof frontier to >= 267 lifted constructive functions (shrinking residue <= 1178)
    Given crates/verdant-core/src/lifecycle.rs, crates/verdant-core/src/operator_mcp.rs, scripts/verdant-automate.mjs, crates/verdant-tensor/src/scalars.rs, and frontier.toml
    When Verdant verifies its lifted automation predicates, MCP verdant_automate surface, Observer refinement agenda, and [frontier_ratchet]
    Then scripts/verdant-automate.mjs delegates directly to verdant operator automate, MCP advertises verdant_automate, and frontier.toml proves at least 267 lifted constructive functions with residue <= 1178

  @VE-67 @build
  Scenario: Verdant breaks every hand-written Rust file strictly below 2000 lines (max_handwritten_rust_file_lines <= 1985 via 7 focused submodules hd_world_templates.rs, standards_registry.rs, observer_types.rs, uhd_shaders.rs, bridge_sim.rs, exports_cold_stage.rs, exports_uorc_studio.rs lifting >= 163 lifecycle modules) and breaks every shipped JS module strictly below 1400 lines (max_js_file_lines <= 1400 via catalog-streams-stitch.mjs, engine-worker-codec.mjs, and studio/stage-assets.mjs)
    Given model/parametric-ratchet.toml [surface_ratchet], site/generated/self-host-ratchet-manifest.toml, catalogs/governance/iso42010-viewpoints.json, and all decomposed Rust and JS submodules
    When prism-fidelity audits [surface_ratchet] and the Observer evaluates all 16 milestones (M1..M16)
    Then max_handwritten_rust_file_lines is at most 1985, max_js_file_lines is at most 1400, at least 163 lifecycle modules are lifted across all 7 categories, and all 13 Observer KPIs pass across M1..M16

  @VE-68 @build
  Scenario: Verdant lifts Wasm synthesis/verification (sync-wasm), deterministic RFC 2789 Cargo sparse crate packaging (package-crates), and live loopback HTTP Cargo sparse registry verification (verify-registry) natively into crates/verdant-core/src/build_ops.rs (CLI, MCP, in-process lifecycle.rs, and thin JS delegates), lifting >= 164 lifecycle modules and expanding the shrink-only uorust MIR proof frontier to >= 280 lifted constructive functions (residue <= 1177)
    Given crates/verdant-core/src/build_ops.rs, crates/verdant-core/src/operator_mcp.rs, scripts/sync-prism-wasm.mjs, scripts/package-rust-crates.mjs, scripts/verify-sparse-registry.mjs, and frontier.toml
    When Verdant runs execute_native_build_op SyncWasm check-only, inspects MCP tools/list, and audits [frontier_ratchet]
    Then all three scripts are thin delegates into verdant operator, MCP advertises verdant_sync_wasm/verdant_package_crates/verdant_verify_registry, >= 164 lifecycle modules are lifted, and frontier.toml proves >= 280 lifted constructive functions with residue <= 1177

  @VE-69 @build
  Scenario: Verdant decouples browser verdant-uorc.wasm from uorc_compress, uorc_search, and static warm-state blobs onto uorc-wasm-decode with an inline UORC-SHA256-CHAIN-v1 raw-archive verifier/encoder and certified uorc_decompress::run fallback, cutting site/verdant-uorc.wasm by > 700 KiB raw to <= 3950000 bytes (sub-1 MiB gzipped) and tightening max_release_capsule_bytes <= 1550000 bytes across M1..M17
    Given crates/verdant-wasm/Cargo.toml, crates/verdant-core/src/streaming.rs, site/verdant-uorc.wasm, model/parametric-ratchet.toml [surface_ratchet], and catalogs/governance/change-roadmap-kpi.json
    When prism-fidelity audits [surface_ratchet] and the Observer evaluates all 17 milestones (M1..M17)
    Then verdant-wasm uorc-decoder enables only verdant-core/uorc-wasm-decode, site/verdant-uorc.wasm is at most 3950000 bytes, max_release_capsule_bytes is at most 1550000 bytes, and all 13 Observer KPIs pass across M1..M17

  @VE-70 @build
  Scenario: Verdant realizes its Lean-proved GitHub-free sovereign governance Prism model (app/src/Governance.lex.tex, 838/838 fidelity, 10/10 ISO 15939 measures, 7 STRIDE threats/controls) in crates/verdant-core/src/authority.rs (CLI verdant operator authority + MCP verdant_authority_exec), enforcing Genesis Owner bootstrap, M-of-N Admin quorum with non-self-approval, monotonic prevEpochKappa chains, immediate revocation of departed Admin pending approvals and sole-attested standing grants, forward-secret boundary rekeying, anti-lockout succession, compromise quarantine, and >= 295 uorust MIR-lifted constructive functions (residue <= 1176)
    Given app/src/Governance.lex.tex, crates/verdant-core/src/authority.rs, crates/verdant-tensor/src/scalars.rs, crates/verdant-core/src/operator_mcp.rs, and frontier.toml
    When Verdant runs genesis bootstrap, M-of-N signing request review, Admin departure approval revocation, compromise quarantine, anti-lockout refusal, MCP verdant_authority_exec, and audits [frontier_ratchet]
    Then departed Admin unfinalized approvals and sole-attested standing grants revoke immediately while clean historical commits verify, MCP executes verdant_authority_exec, all 18 milestones M1..M18 pass, and frontier.toml proves >= 295 lifted functions with residue <= 1176

  @VE-71 @build
  Scenario: Verdant executes Milestone M19 self-refinement across its Observer agenda: decomposing uhd.rs, network.rs, lifecycle.rs, and observer.rs into focused submodules (max_handwritten_rust_file_lines <= 1650, >= 173 lifted lifecycle modules), decomposing engine-worker.mjs, studio-page.mjs, and studio.mjs (max_js_file_lines <= 1250), expanding the uorust MIR proof frontier to >= 345 lifted functions (residue <= 1169), and surfacing zero-account Sovereign Signing Requests & Admin Quorum review/revocation inside Verdant Studio (site/studio/authority-panel.mjs)
    Given crates/verdant-core/src/uhd_workload.rs, crates/verdant-core/src/network_custody.rs, crates/verdant-core/src/lifecycle_automate.rs, crates/verdant-core/src/observer_comms.rs, site/studio/authority-panel.mjs, site/studio/studio-boot.mjs, site/studio-director-panel.mjs, and frontier.toml
    When prism-fidelity audits [frontier_ratchet] and [surface_ratchet], Verdant evaluates its MIR-lifted governance/erasure/Wasm scalars, and the Observer evaluates all 19 milestones M1..M19
    Then max_handwritten_rust_file_lines is at most 1650, max_js_file_lines is at most 1250, frontier.toml proves at least 345 lifted functions with residue at most 1169, Studio exposes sovereign signing-request quorum controls, and all Observer KPIs pass across M1..M19

  @VE-72 @build
  Scenario: Verdant executes Milestone M20: closing the local-vs-CI release gate gap (conformance-engine in lifecycle-gate --full + ratchet-aware Observer refinement), dropping gltf image decoding from wasm32 onto browser-native createImageBitmap PBR decoding (verdant-uorc.wasm <= 3810000 B, capsule <= 1410000 B), binding VerdantVcs commits and VerdantReleaseCapsule .holo sealing to finalized ApprovalCertificates with unified Wasm/CLI/MCP JSON bridge, and shrinking uorust residue strictly below 1169 while lifting >= 380 constructive functions across M1..M20
    Given crates/verdant-core/src/lifecycle.rs, crates/verdant-core/src/observer_refine.rs, crates/verdant-core/src/gltf_io.rs, crates/verdant-core/src/authority_cert.rs, crates/verdant-core/src/vcs.rs, site/engine-worker-extended.mjs, site/studio/authority-panel.mjs, and frontier.toml
    When Verdant verifies ApprovalCertificate custody on commits and .holo capsules, audits [frontier_ratchet] and [surface_ratchet], and evaluates all 20 milestones M1..M20
    Then lifecycle-gate includes conformance-engine, site/verdant-uorc.wasm is at most 3810000 bytes, tampered certificates fail closed, frontier.toml proves at least 380 lifted functions with residue at most 1168 and uncertified at most 1324, and all Observer KPIs pass across M1..M20

  @VE-73 @build
  Scenario: Verdant executes Milestone M21 across its Observer refinement agenda: decomposing streaming.rs, ecs.rs, physics.rs, verdant-cli/src/lib.rs, and authority.rs (max_handwritten_rust_file_lines <= 1495, >= 178 lifted lifecycle modules), decomposing catalog-streams.mjs, studio-page.mjs, and engine-worker.mjs (max_js_file_lines <= 1125), eliminating existing uorust residue in place, and expanding frontier.toml to >= 420 lifted functions with residue <= 1164 and uncertified <= 1321 across M1..M21
    Given crates/verdant-core/src/streaming_uorc.rs, crates/verdant-core/src/ecs_tlas.rs, crates/verdant-core/src/physics_controller.rs, crates/verdant-cli/src/automate_cmd.rs, site/catalog-streams-local.mjs, site/studio/studio-viewport.mjs, and frontier.toml
    When prism-fidelity audits [frontier_ratchet] and [surface_ratchet], Verdant evaluates its MIR-lifted BVH/CCD/UORC/arena scalars, and the Observer evaluates all 21 milestones M1..M21
    Then max_handwritten_rust_file_lines is at most 1495, max_js_file_lines is at most 1125, frontier.toml proves at least 420 lifted functions with residue at most 1164 and uncertified at most 1321, and all Observer KPIs pass across M1..M21

  @VE-74 @build
  Scenario: Verdant breaks the 2.0 MiB browser Wasm wall in Milestone M22 (splitting uorc_decompress behind native-default uorc-ncm-decode to drop site/verdant-uorc.wasm <= 2097152 B and .holo capsule <= 1100000 B), wires --authority-ledger and --approval-cert ApprovalCertificate custody into verdant release pack and verdant operator lifecycle-gate, and expands frontier.toml to >= 450 lifted functions with residue <= 1164 and uncertified <= 1321 across M1..M22
    Given crates/verdant-core/Cargo.toml, crates/verdant-core/src/streaming_uorc.rs, crates/verdant-cli/src/lib.rs, crates/verdant-cli/src/operator_cmd.rs, site/verdant-uorc.wasm, and frontier.toml
    When Verdant runs lifecycle-gate and release pack with valid and tampered ApprovalCertificates, audits [frontier_ratchet] and [surface_ratchet], and evaluates all 22 milestones M1..M22
    Then site/verdant-uorc.wasm is at most 2097152 bytes, max_release_capsule_bytes is at most 1100000 bytes, CLI refuses tampered certificates, frontier.toml proves at least 450 lifted functions with residue at most 1164, and all Observer KPIs pass across M1..M22


  @VE-75 @build
  Scenario: Verdant breaks the 1.00 MiB .holo release capsule barrier in Milestone M23 (gating native-only VSM/Operator governance exports out of wasm32 to push site/verdant-uorc.wasm <= 1850000 B and .holo capsule < 1048576 B), breaks every hand-written Rust file <= 1365 lines (>= 180 lifted modules via collision_blas.rs, vcs_studio.rs, and companion consolidation), breaks every shipped JS file <= 1024 lines with zero extra JS requests, and expands frontier.toml to >= 480 lifted functions with residue <= 1162 and uncertified <= 1319 across M1..M23
    Given crates/verdant-wasm/src/exports_uorc_studio.rs, crates/verdant-core/src/collision_blas.rs, crates/verdant-core/src/vcs_studio.rs, site/verdant-uorc.wasm, site/studio.mjs, site/engine-worker.mjs, site/studio/studio-page.mjs, site/catalog-streams.mjs, and frontier.toml
    When prism-fidelity audits [frontier_ratchet] and [surface_ratchet], Verdant evaluates its MIR-lifted M23 capsule/modularity/governance scalars, and the Observer evaluates all 23 milestones M1..M23
    Then max_release_capsule_bytes is strictly below 1048576 bytes, site/verdant-uorc.wasm is at most 1850000 bytes, max_handwritten_rust_file_lines is at most 1365, max_js_file_lines is at most 1024, frontier.toml proves at least 480 lifted functions with residue at most 1162, and all Observer KPIs pass across M1..M23
