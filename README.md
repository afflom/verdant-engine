# Verdant Engine

<p align="center">
  <a href="https://github.com/afflom/verdant-engine/actions/workflows/bootstrap.yml"><img src="https://github.com/afflom/verdant-engine/actions/workflows/bootstrap.yml/badge.svg?branch=main" alt="Engine Gate"></a>
  <img src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue" alt="License: MIT OR Apache-2.0">
</p>

**Verdant Engine** (`verdant-core`, `verdant-wasm`, `verdant-cli`) is a standalone, commercial-grade universal 3D browser and WebAssembly game engine modeled, mechanized, and verified end-to-end with Prism.

Rather than relying on conditional browser GPU APIs that are absent on many mobile and desktop clients, Verdant Engine hosts its core tensor contractions portably through **Prism + `uor-matmul`** (*decode -> accumulate without intermediate rounding -> encode once* across exact Laurent/integer `(+, *)`, tropical `(max, +)`, and quantized audio rings) paired with a `16x16` tile-binned software PBR rasterizer, universal WebGL2/Canvas2D presentation, and **progressive multi-LOD `UORC` (`UORC-SHA256-CHAIN-v1`) streaming** up to lossless source fidelity.

---

## Core Engine Subsystems (`crates/verdant-core`)

| Subsystem | Module | Key Types & Standards |
| --- | --- | --- |
| **Linear Algebra & Spatial Transforms** | `math` | `Vec2`, `Vec3`, `Vec4`, `Quat`, `Mat4`, `Aabb`, `Ray`, `Frustum` (right-handed `-Z` forward, `+Y` up) |
| **Deterministic ECS & Spatial Queries** | `ecs` | `SceneWorld`, `EntityId` (generational slot allocator), parent-child hierarchy propagation, `BoundingVolumeHierarchy` (6-plane Gribb-Hartmann frustum + Möller-Trumbore ray queries) |
| **Exact Tensor Kernels & Tile PBR** | `tensor` | `verified_engine_tensor_kernels`, `batch_transform_positions_exact`, `tropical_frustum_distance_bounds`, `mix_spatial_audio_channels_exact`, `render_triangles_tile_framebuffer` (`16x16` RGBA8 + depth) |
| **Progressive Multi-LOD UORC Streaming** | `streaming` | `LodLevel` (`Lod0CoarseProxy -> Lod1MediumDetail -> Lod2HighDetail -> Lod3LosslessSource`), `StandardSceneState`, `pack_progressive_uorc_stream`, `verify_progressive_uorc_stream`, `decode_uorc_stream_up_to_lod` |
| **Procedural Geometry & Morph Targets** | `geometry` | `ProceduralMesh`, `MorphTarget`, `generate_terrain_mesh`, `generate_fluted_entasis_column_mesh`, `generate_faceted_prism_mesh`, `generate_deformable_ribbon_mesh` |
| **Skeletal & Morph Animation** | `animation` | `Skeleton`, `Joint`, `SkinWeights4`, `SkeletalClip`, `KeyframePose`, `skin_mesh_linear_blend` (acyclic joint DAGs, 4-joint normalized LBS weights) |
| **Microfacet PBR, Frame Graph & WGSL** | `render` | `PbrMaterial`, `RenderFrameGraph` (5-pass acyclic barrier graph), `compute_csm_splits` (4 cascades), `IblEnvironmentProbe`, `validate_wgsl_shader` (W3C `naga`) |
| **Binary glTF 2.0 Scene Export** | `gltf_io` | `export_scene_to_glb`, `validate_glb_bytes` (Khronos glTF 2.0 binary `.glb` verified through the official `gltf` parser) |
| **3D Convex Collision & Impulse Solver** | `collision` | `ConvexHullCollider`, `gjk_intersect_and_manifold`, `solve_contact_impulse` (bounded 3D GJK Minkowski simplex, Baumgarte stabilization, Coulomb friction cones) |
| **Rigid-Body, Capsule & Spline Physics** | `physics` | `PhysicsWorld`, `RigidBody` (symplectic Euler + Archimedes fluid buoyancy), `CapsuleController`, `KinematicSplineBody` |
| **NavMesh & Corridor Pathfinding** | `navmesh` | `NavMesh`, `NavPolygon`, `steer_along_corridor` (slope-filtered walkable terrain polygons, bounded admissible A* corridor search) |
| **Spatial Audio & Modal Synthesis** | `audio` | `AcousticEnvironment` (Sabine & Eyring `RT60`), `ListenerPose`, `SpatialAudioSource`, `evaluate_binaural_cue` (Woodworth ITD/ILD), `synthesize_modal_bar_modes` |
| **Worker Job Graph & Binary Snapshots** | `runtime` | `EngineConfig`, `WorkerJobGraph` (Kahn topological critical-path scheduler), `FrameBudgetProfiler` (`16,666 us` / `8,333 us` KPI gate), `WorldSnapshot` (`VDNT` + SHA-256 digest) |
| **Verified Prism Engine Kernel** | `prism_kernel` | Constructively proved `{0, 1}` engine invariants, LOD transition guards, and tensor accumulator bounds projected mechanically from Lean 4 (`Tensor.lex.tex`, `Subsystems.lex.tex`) |
| **Typed Engine Diagnostics** | `error` | `StreamingError`, `TensorComputeError`, `AssetValidationError`, `ShaderValidationError`, `PhysicsStepError`, `EngineConfigError`, `SnapshotCodecError`, `AnimationError`, `CollisionError`, `NavigationError`, `RenderGraphError` |

---

## Commercial Developer Kit, Studio, Starter Template & CLI

1. **Interactive 3D Browser Engine Studio (`site/`)**:
   - Real-time 60fps OffscreenCanvas worker (`site/engine-worker.mjs` + `site/verdant-wasm.wasm`) with WebGL2 hardware rendering and automatic Prism `16x16` software tile fallback, progressive **LOD Scrubber** (`LOD0` Proxy -> `LOD1` -> `LOD2` -> `LOD3` Lossless UORC), **Orbit / First-Person `WASD` Walk** camera switcher, `AudioWorklet` Euler-Bernoulli modal synthesis, one-click binary `.glb` download, and `#vdnt=` SHA-256 URL snapshot sharing.
2. **Runnable Engine Examples (`crates/verdant-core/examples/`)**:
   - `cargo run -p verdant-core --example tensor_uorc_showcase` — Executes exact `uor-matmul` contractions, renders a `16x16` software tile PBR framebuffer, packs and verifies a 4-tier progressive `UORC` stream (`~10:1` lossless compression), validates built-in WGSL shaders, and exports binary glTF 2.0.
   - `cargo run -p verdant-core --example custom_game_arena` — Builds a custom 3D physics arena with capsule locomotion, rigid-body collisions, spatial audio, BVH queries, and `.glb` export.
   - `cargo run -p verdant-core --example headless_benchmark` — Profiles multi-worker frame scheduling, physics simulation throughput, and `VDNT` binary snapshot round-trips against the `16,666 us` (60 fps) frame budget KPI.
3. **Standalone Starter Template & CLI Packer (`templates/browser-3d-starter/`, `crates/verdant-cli`)**:
   ```sh
   cargo run --manifest-path templates/browser-3d-starter/Cargo.toml
   cargo run -p verdant-cli -- pack-scene scene.glb
   cargo run -p verdant-cli -- verify-uorc scene.glb.uorc
   cargo run -p verdant-cli -- benchmark
   ```
4. **Diátaxis Developer Documentation (`site/docs/` & `site/llms.txt`)**:
   - Complete Getting Started tutorial, Custom World & Progressive UORC Streaming guides, API & Frame Budget Reference, Tensor & UORC Architecture explanation, and machine-readable `llms.txt`, mechanically checked against ISO 24495-1 plain-language and accessibility gates (`node scripts/content/check.mjs site`).

---

## Engineering & Verification Discipline

- **Zero Magic Constants (`FI-08`)**: `model/parametric-ratchet.toml` locks every unexplained literal ceiling strictly to `0` across all shipped Rust crates (`crates/verdant-core/src/`, `crates/verdant-wasm/src/`, and `crates/verdant-cli/src/`) and all constructive backend specification modules (`Tensor.lex.tex`, `Subsystems.lex.tex`, `DevRel.lex.tex`).
- **100% Closed Prism Fidelity (`FI-01..FI-08`)**: `fidelity.toml` maintains `0` open questions across all three formal traceability axes (`derived = 100%`, `meant = 100%`, `measured = 100%`).
- **Authoritative Standards Oracles (`VE-01`, `VE-04`, `VE-05`)**: Progressive streams verify through `uorc` (`UORC-SHA256-CHAIN-v1`), exported `.glb` binaries validate through Khronos `gltf`, and shipped WGSL shaders validate through W3C `naga`.
- **Single Normative Gate (`just vv`)**: Runs `cargo fmt --check`, `xtask validate` (`R1`–`R6`), SDK lock & template policy checks, provenance and zero-constants ratchet checks (`just fidelity`), workspace `clippy -D warnings`, all unit and conformance tests (`VE-01..VE-16`, `FR-01..FR-03`, `FI-01..FI-08`), `cargo deny`, and the verified WebAssembly guest archive build (`just app`).

See [CONFORMANCE.md](CONFORMANCE.md) for the generated capability ledger and [ENGINE-VERIFICATION.md](ENGINE-VERIFICATION.md) for the architectural and formal verification report.

---

## Repository Layout

| Path | Purpose |
| --- | --- |
| `crates/verdant-core/` | Standalone universal 3D game engine library (`math`, `ecs`, `tensor`, `streaming`, `geometry`, `animation`, `render`, `gltf_io`, `collision`, `navmesh`, `physics`, `audio`, `runtime`, `prism_kernel`, `error`) and runnable examples. |
| `crates/verdant-wasm/` | Safe WebAssembly simulation, portable tensor tile rasterization, culling, spatial acoustics, and `VDNT` snapshot worker bridge. |
| `crates/verdant-cli/` | Command-line progressive `.uorc` + `.glb` + `.vdnt` scene packer, Khronos glTF 2.0 & W3C WGSL validator, and headless 60fps benchmark (`verdant`). |
| `templates/browser-3d-starter/` | Ready-to-fork playable 3D Orbital Beacon Arena starter template built on `verdant-core`. |
| `site/` | Interactive 3D OffscreenCanvas + AudioWorklet Engine Studio (`card.html`, `card.mjs`, `engine-worker.mjs`, `scene-renderer.mjs`, `audio-worklet.mjs`), Diátaxis documentation (`site/docs/`), and `llms.txt`. |
| `features/suites/` | Executable Gherkin specifications (`engine.feature`, `frame.feature`, `fidelity.feature`). |
| `crates/conformance/` | End-to-end conformance and negative-refusal test suites (`VE-01..VE-16`, `FR-01..FR-03`, `CM-02`, `CM-03`). |
| `crates/fidelity/` | Provenance citation auditor, ISO/IEC 15939 measurement scorer, and permanent Zero-Constants Parametric Ratchet (`FI-01..FI-08`). |
| `app/` | Backend formal specification and Lean 4 verified Core-Wasm guest verifier (`Engine.lex.tex`, `Tensor.lex.tex`, `Subsystems.lex.tex`, `DevRel.lex.tex`). |
| `model/` | Normative capability register (`ids.toml`), zero-constants ratchet ceilings (`parametric-ratchet.toml`), charter, and content model. |

Licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE).
