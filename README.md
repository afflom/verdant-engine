# Verdant Engine

<p align="center">
  <a href="https://github.com/afflom/verdant-engine/actions/workflows/bootstrap.yml"><img src="https://github.com/afflom/verdant-engine/actions/workflows/bootstrap.yml/badge.svg?branch=main" alt="Engine Gate"></a>
  <img src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue" alt="License: MIT OR Apache-2.0">
</p>

**Verdant Engine** (`verdant-tensor`, `verdant-core`, `verdant-wasm`, `verdant-cli`) is a standalone, universal 3D browser, WebAssembly, and native game and interface engine modeled, mechanized, and verified end-to-end with Prism.

Rather than relying on conditional browser GPU APIs that are absent on many mobile and desktop clients, Verdant Engine distills a **pure tensor-logic substrate** (`crates/verdant-tensor`) over **`uor-matmul`** (*decode -> accumulate without intermediate rounding -> encode once* across exact Laurent `(+, *)`, tropical `(max, +)` and `(min, +)`, quantized audio and neuromorphic `Q15` state-space rings, and zero-decode `uor_matmul::codec` streams), paired with a `16x16` tile-binned software PBR rasterizer, zero-copy 3-pass WebGL2 presentation, **native WebAssembly `uorc_decompress` progressive `LOD0 -> LOD4` HD streaming**, formal 2D/3D UI compliance, an embedded `coded_gemm` language/intent model, **executable stakeholder journeys + a rapid Rust/Wasm Game & App Scaffolder**, and **Foundry `Prism ID` + `afflom/stitch` 2-of-3 parity multiplayer custody**.

---

## Pure Tensor-Logic Substrate (`crates/verdant-tensor`)

Every core engine pass reduces to Prism-scheduled semiring contractions over `uor-matmul` verified in `app/src/Tensor.lex.tex` (15 unified kernels):

| Semiring Family | Module | Unified Engine Contractions |
| --- | --- | --- |
| **Linear Exact `(+, *)`** | `linear`, `raster` | Hierarchical ECS `Mat4` propagation, 4-joint normalized LBS skinning, analytical Two-Bone Foot IK (`kernel13`), Delassus constraint effective mass (`J * M^-1 * J^T`), `16x16` tile-binned PBR/CSM/IBL rasterization, and flex UI distribution |
| **Tropical `(max, +)` & `(min, +)`** | `tropical`, `neural` | Batch Gribb-Hartmann frustum plane clearance, 3D GJK directional support argmax, swept-sphere BLAS `TOI` pruning (`kernel12`), NavMesh all-pairs shortest-path closure, intrinsic 2D/3D UI box solving, and deadlock-free Chronicle reachability (`kernel14`) |
| **Quantized Audio & Neuromorphic Rings** | `audio_ring`, `neural` | Multi-emitter modal FIR convolution, Woodworth spherical-head ITD/ILD stereo panning, 4-channel Hadamard Sabine/Eyring `FDN` reverb, and rollback-exact `Q15` Leaky-Integrate-and-Fire (`LIF`) mutual-attention recurrence |
| **Zero-Decode `uor_matmul::codec`** | `coded`, `neural` | Direct `coded_gemm` over `Grid` quantized geometry, sub-byte `Packed<Grid<16>, 2>` nibble animation tracks & embedded SLM reasoning weights, and sparse `Runs<Grid<16>, 16>` IBL harmonics without float scratch buffers |

---

## Core Engine Subsystems (`crates/verdant-core`)

| Subsystem | Module | Key Types & Standards |
| --- | --- | --- |
| **Linear Algebra & Spatial Transforms** | `math` | `Vec2`, `Vec3`, `Vec4`, `Quat`, `Mat4`, `Aabb`, `Ray`, `Frustum` (right-handed `-Z` forward, `+Y` up) |
| **Deterministic ECS & Two-Level BVH** | `ecs` | `SceneWorld`, `EntityId`, exact tensor hierarchy propagation, `BoundingVolumeHierarchy`, `TwoLevelBvh` (`Entity TLAS` + triangle `MeshBlas` pruned via tropical `kernel12`) |
| **Exact Tensor Kernels & Tile PBR** | `tensor` | `verified_engine_tensor_kernels`, `verified_unified_tensor_kernels` (15 kernels), `verified_coded_matrix_tiers` (3 tiers), `render_triangles_tile_framebuffer` (`16x16` RGBA8 + depth) |
| **Lawful Camera Rig & Guard Sphere** | `camera` | `LawfulCameraRig` (`ThirdPersonSpringArm`, `FirstPersonEye`), `CameraRigConfig`, exact near-frustum diagonal guard sphere (`cameraNearSphereGuarded` — zero wall or floor peeking) |
| **HD Environments & Multi-Octave Relief** | `hd_world` | `synthesize_hd_citadel_sector` (`LOD0 -> LOD4`, 45k–65k+ smooth triangles), `generate_vaulted_citadel_hall_mesh`, `generate_parabolic_arch_bridge_mesh`, `generate_hd_sculpted_terrain_mesh`, `generate_sculpted_canopy_tree_mesh` |
| **22-Joint Characters, Foot IK & Possession** | `character` | `ArticulatedCharacter` (`Explorer`, `Archivist`, `BridgeEngineer`, `ObservatoryKeeper`), `HumanoidJointId`, normalized 4-joint LBS + 3 morph targets, analytical Two-Bone Foot IK, 1st-person head masking, `bind_external_mesh` |
| **Floating-Origin Universes & Portals** | `universe` | `UniverseCellCoord` (`i64^3`), `UniversePosition` (`[-64m, +64m)^3` `universeLocalOffsetBounded`), camera-cell-relative GPU matrices, momentum-preserving `PortalGraph`, `UniverseManager` |
| **Conservation-Verified Stories & Apps** | `narrative` | `StoryAutomaton` (tropical `(min, +)` deadlock-freedom proof at construction), `ConservationLedger` (`conservationLawValid`), SLM-grounded `advance_from_slm_prompt` |
| **Native Wasm UORC & Zero-Decode Streaming** | `streaming` | `LodLevel` (`LOD0 -> LOD3`), `pack_progressive_uorc_stream`, `pack_portable_progressive_uorc_stream`, `decode_portable_uorc_stream_up_to_lod` (`uorc_decompress::run`), `CodedLodFrameBundle` |
| **Procedural Geometry & Morph Targets** | `geometry` | `ProceduralMesh`, `MorphTarget`, `generate_terrain_patch_mesh`, `generate_fluted_column_mesh`, `generate_faceted_prism_mesh`, `generate_deformable_ribbon_mesh` |
| **Skeletal & Morph Animation** | `animation` | `Skeleton`, `Joint`, `SkinWeights4`, `SkeletalClip`, `KeyframePose`, `skin_mesh_linear_blend` (acyclic joint DAGs, exact 4-joint LBS contraction) |
| **Microfacet PBR, Frame Graph & WGSL** | `render` | `PbrMaterial`, `PackedSceneRenderBuffers`, `pack_world_meshes_to_buffers`, `RenderFrameGraph` (5-pass acyclic barrier graph), `compute_csm_splits` (4 cascades), `validate_wgsl_shader` (W3C `naga`) |
| **Binary glTF 2.0 Import, Export & UORC Transcode** | `gltf_io` | `export_scene_to_glb`, `import_glb_to_scene` (meshes, skeletons, keyframe clips, PBR materials), `transcode_glb_to_progressive_uorc`, `validate_glb_bytes` (Khronos `gltf`) |
| **3D Convex Collision & Impulse Solver** | `collision` | `ConvexHullCollider`, `gjk_intersect_and_manifold` (tropical GJK support maps), `solve_contact_impulse` (Baumgarte stabilization, Coulomb friction cones) |
| **XPBD Joints, Swept Capsule & Raycast Vehicles** | `physics` | `PhysicsWorld`, `RigidJointConstraint` (`Hinge`, `BallSocket`, `Slider`, `SpringDamper` via Delassus `J * M^-1 * J^T`), swept BVH `CapsuleController`, 4-wheel `RaycastVehicle`, `KinematicSplineBody` |
| **NavMesh & Tropical Corridor Pathfinding** | `navmesh` | `NavMesh`, `NavPolygon`, `all_pairs_shortest_paths_tropical` (tropical `(min, +)` closure), `steer_along_corridor` (bounded admissible `A*` corridor search) |
| **Spatial Audio, Occlusion & Modal Synthesis** | `audio` | `AcousticEnvironment` (Sabine & Eyring `RT60`), `evaluate_binaural_cue_with_occlusion` (BVH-raycasted acoustic low-pass attenuation), Hadamard `FDN`, `synthesize_modal_bar_modes` |
| **Formal 2D/3D UI Modeling & WCAG Compliance** | `ui` | `UiTree`, `UiWidget`, `UiSurfaceSpace` (`ScreenSpace` + `DiegeticWorld`), `SolvedUiRect` (`>= 48x48` CSS px coarse targets, WCAG 2.2 AA contrast, tropical focus closure, diegetic BVH picking) |
| **Second-Order Cybernetics, SLM & Container Grants** | `cybernetics` | `CyberneticObserverLoop` (`PlayerObserverSample <-> WorldAttentionDirective`), `CodedSlmDirector` (local `coded_gemm` intent router), `ContainerGrantMatrix` (`FK-07` denial-wins lattice) |
| **Sovereign Identity & Stitch Multiplayer Custody** | `network` | `PrismIdSessionEnvelope`, `SignedJournalBatch`, `StitchFragment` (`STCH` 2-of-3 `(A, B, A ⊕ B)` SHA-256 parity shards), `SurvivalEntropyTracker` |
| **Job Graph, W5H Governor & Rollback** | `runtime` | `OntologicalRuntimeGovernor` (8-stratum `W5H` + Telos loss), `WorkerJobGraph` (6-stage DAG), `FrameBudgetProfiler` (`16,666 us` KPI), `WorldSnapshot` (`VDNT` + SHA-256), `DeterministicInputJournal`, 64-frame `RollbackSession`, `DeltaUorcSnapshot` |
| **Verified Prism Engine Kernel** | `prism_kernel` | Constructively proved `{0, 1}` engine invariants, LOD transition guards, UI/LIF/Stitch bounds, and tensor accumulator bounds projected mechanically from Lean 4 (`Tensor.lex.tex`, `Subsystems.lex.tex`) |
| **Typed Engine Diagnostics** | `error` | `StreamingError`, `TensorComputeError`, `AssetValidationError`, `ShaderValidationError`, `PhysicsStepError`, `EngineConfigError`, `SnapshotCodecError`, `AnimationError`, `CollisionError`, `NavigationError`, `RenderGraphError`, `UiComplianceError`, `CyberneticError`, `StitchCustodyError` |

---

## Self-Hosting Browser Studio, Starter Template & CLI

1. **100% Dogfooded 3D Browser Engine Studio (`site/`)**:
   - Every 3D triangle, physics body, UI compliance box, neuromorphic LIF attention tick, `coded_gemm` SLM command, and Stitch 2-of-3 parity shard on GitHub Pages runs directly inside `verdant-wasm` (`site/engine-worker.mjs` + `site/verdant-wasm.wasm` + on-demand `site/verdant-uorc.wasm`) within the `120 KiB` mobile cold-boot budget.
   - Drag-and-drop `.glb` import + `OPFS` caching, live WebGL2 ↔ pure Wasm `16x16` software tile PBR switcher, local natural-language SLM prompt bar, **6-tab Verdant Studio Workbench** (7 Executable Stakeholder Journeys, Live `SceneWorld` Entity Outliner, Rapid Rust + Wasm Game/App Project Scaffolder across 4 archetypes, Physics/Chronicle, UI/Neural, and Mesh/Trust in `site/studio.mjs`), and **Foundry `Prism ID` + `afflom/stitch` Mesh** custody (`site/stitch-mesh.mjs`) bound to Hologram-Warm Light, Dark, and Immersive tokens.
2. **Runnable Engine Examples (`crates/verdant-core/examples/`)**:
   - `cargo run -p verdant-core --example tensor_uorc_showcase`
   - `cargo run -p verdant-core --example custom_game_arena`
   - `cargo run -p verdant-core --example headless_benchmark`
3. **Standalone Starter Template & CLI Packer (`templates/browser-3d-starter/`, `crates/verdant-cli`)**:
   ```sh
   cargo run --manifest-path templates/browser-3d-starter/Cargo.toml
   cargo run -p verdant-cli -- pack-scene scene.glb
   cargo run -p verdant-cli -- import-glb scene.glb
   cargo run -p verdant-cli -- transcode-glb scene.glb scene-progressive.uorc
   cargo run -p verdant-cli -- verify-ui 1280 900
   cargo run -p verdant-cli -- route-intent "switch to software 16x16 tile raster"
   cargo run -p verdant-cli -- stitch-shard scene-progressive.uorc ./shards
   cargo run -p verdant-cli -- stitch-recover ./shards/shard_0.stch ./shards/shard_2.stch recovered.uorc
   cargo run -p verdant-cli -- verify-story
   cargo run -p verdant-cli -- pack-universe ./universe-out
   cargo run -p verdant-cli -- benchmark
   ```
4. **Executable Diátaxis Developer Documentation (`site/docs/` & `site/llms.txt`)**:
   - Every Diátaxis page mounts `site/docs/live-embed.mjs` over `site/verdant-wasm.wasm` to execute live `16x16` tile PBR renders, `Q15` LIF spike rasters, physics steps, progressive LOD sweeps, and SHA-256 snapshot tamper tests directly in the browser, mechanically checked against ISO 24495-1 plain-language and WCAG contrast gates (`node scripts/content/check.mjs site`).

---

## Engineering & Verification Discipline

- **Zero Magic Constants (`FI-08`)**: `model/parametric-ratchet.toml` locks every unexplained literal ceiling strictly to `0` across all shipped Rust crates (`crates/verdant-tensor/src/`, `crates/verdant-core/src/`, `crates/verdant-wasm/src/`, and `crates/verdant-cli/src/`) and all constructive backend specification modules (`Model.lex.tex`, `Tensor.lex.tex`, `Subsystems.lex.tex`, `DevRel.lex.tex`, `Measures.lex.tex`, `ThreatModel.lex.tex`).
- **100% Closed Prism Fidelity (`FI-01..FI-08`)**: `fidelity.toml` maintains `0` open questions across all three formal traceability axes (`derived = 100%`, `meant = 100%`, `measured = 100%`).
- **Single Normative Gate (`just vv`)**: Runs `cargo fmt --check`, `xtask validate` (`R1`–`R6`), SDK lock & template policy checks, provenance and zero-constants ratchet checks (`just fidelity`), workspace `clippy -D warnings`, all unit and conformance tests (`VE-01..VE-32`, `FR-01..FR-03`, `FI-01..FI-08`), `cargo deny`, and the verified WebAssembly guest archive build (`just app`).

See [CONFORMANCE.md](CONFORMANCE.md) for the generated capability ledger and [ENGINE-VERIFICATION.md](ENGINE-VERIFICATION.md) for the architectural and formal verification report.

---

## Repository Layout

| Path | Purpose |
| --- | --- |
| `crates/verdant-tensor/` | Pure `no_std + alloc` tensor-logic substrate over `uor-matmul` (`linear`, `coded`, `tropical`, `audio_ring`, `neural`, `raster`, `prism_kernel`). |
| `crates/verdant-core/` | Standalone universal 3D game and UI engine library (`math`, `ecs`, `tensor`, `streaming`, `geometry`, `animation`, `render`, `gltf_io`, `collision`, `navmesh`, `physics`, `audio`, `ui`, `cybernetics`, `network`, `runtime`, `prism_kernel`, `error`) and runnable examples. |
| `crates/verdant-wasm/` | Safe WebAssembly zero-copy scene packer, portable `16x16` tile rasterizer, native `uorc_decompress` + `coded_gemm` streaming, LIF/SLM director, formal UI solver, and Stitch custody worker bridge. |
| `crates/verdant-cli/` | Command-line `.glb -> .uorc` transcoder, progressive `.uorc` + `.glb` + `.vdnt` packer, UI verifier, SLM router, Stitch sharder, and headless 60fps benchmark (`verdant`). |
| `templates/browser-3d-starter/` | Ready-to-fork playable 3D Orbital Beacon Arena starter template built on `verdant-core`. |
| `site/` | 100% dogfooded 3D OffscreenCanvas + AudioWorklet Engine Studio (`card.html`, `card.mjs`, `studio.mjs`, `stitch-mesh.mjs`, `engine-worker.mjs`, `scene-renderer.mjs`, `audio-worklet.mjs`), executable Diátaxis documentation (`site/docs/`), and `llms.txt`. |
| `features/suites/` | Executable Gherkin specifications (`engine.feature`, `frame.feature`, `fidelity.feature`). |
| `crates/conformance/` | End-to-end conformance and negative-refusal test suites (`VE-01..VE-32`, `FR-01..FR-03`, `CM-02`, `CM-03`). |
| `crates/fidelity/` | Provenance citation auditor, ISO/IEC 15939 measurement scorer, and permanent Zero-Constants Parametric Ratchet (`FI-01..FI-08`). |
| `app/` | Full-stratum formal specification and Lean 4 verified Core-Wasm guest verifier (`Architecture`, `Design`, `Tensor`, `Subsystems`, `Features`, `Security`, `Compliance`, `DevRel`, `Engine`, `Brand`, `Users`, `Provenance`, `Fidelity`). |
| `model/` | Normative capability register (`ids.toml`), zero-constants ratchet ceilings (`parametric-ratchet.toml`), charter, and content model. |

Licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE).
