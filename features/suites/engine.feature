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
