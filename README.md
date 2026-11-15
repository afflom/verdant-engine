# Verdant Engine

<p align="center">
  <a href="https://github.com/afflom/verdant-engine/actions/workflows/bootstrap.yml"><img src="https://github.com/afflom/verdant-engine/actions/workflows/bootstrap.yml/badge.svg?branch=main" alt="Engine Gate"></a>
  <img src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue" alt="License: MIT OR Apache-2.0">
</p>

**Verdant Engine** (`verdant-tensor`, `verdant-core`, `verdant-wasm`, `verdant-cli`) is a standalone, commercial-grade universal 3D browser, WebAssembly, and native game engine modeled, mechanized, and verified end-to-end with Prism.

Rather than relying on conditional browser GPU APIs that are absent on many mobile and desktop clients, Verdant Engine distills a **pure tensor-logic substrate** (`crates/verdant-tensor`) over **`uor-matmul`** (*decode -> accumulate without intermediate rounding -> encode once* across exact Laurent `(+, *)`, tropical `(max, +)` and `(min, +)`, quantized audio rings, and zero-decode `uor_matmul::codec` streams), paired with a `16x16` tile-binned software PBR rasterizer, universal WebGL2/Canvas2D presentation, and **native WebAssembly `uorc_decompress` progressive `LOD0 -> LOD3` streaming** (`UORC-SHA256-CHAIN-v1`).

---

## Pure Tensor-Logic Substrate (`crates/verdant-tensor`)

Every core engine pass reduces to Prism-scheduled semiring contractions over `uor-matmul` verified in `app/src/Tensor.lex.tex`:

| Semiring Family | Module | Unified Engine Contractions |
| --- | --- | --- |
| **Linear Exact `(+, *)`** | `linear`, `raster` | Hierarchical ECS `Mat4` propagation, 4-joint normalized LBS skinning, Delassus constraint effective mass (`J * M^-1 * J^T`), and `16x16` tile-binned PBR/CSM/IBL rasterization |
| **Tropical `(max, +)` & `(min, +)`** | `tropical` | Batch Gribb-Hartmann frustum plane clearance, 3D GJK directional support argmax, and tropical `(min, +)` matrix squaring for exact all-pairs NavMesh shortest-path closure |
| **Quantized Audio Ring** | `audio_ring` | Multi-emitter modal FIR convolution, Woodworth spherical-head ITD/ILD stereo panning, and 4-channel Hadamard Sabine/Eyring feedback delay network (`FDN`) reverb |
| **Zero-Decode `uor_matmul::codec`** | `coded` | Direct `coded_gemm` over `Grid` quantized geometry, sub-byte `Packed<Grid<16>, 2>` nibble animation tracks, and sparse `Runs<Grid<16>, 16>` IBL harmonics without float scratch buffers |

---

## Core Engine Subsystems (`crates/verdant-core`)

| Subsystem | Module | Key Types & Standards |
| --- | --- | --- |
| **Linear Algebra & Spatial Transforms** | `math` | `Vec2`, `Vec3`, `Vec4`, `Quat`, `Mat4`, `Aabb`, `Ray`, `Frustum` (right-handed `-Z` forward, `+Y` up) |
| **Deterministic ECS & Spatial Queries** | `ecs` | `SceneWorld`, `EntityId` (generational slot allocator), exact tensor hierarchy propagation, `BoundingVolumeHierarchy` (tropical frustum + Möller-Trumbore ray queries) |
| **Exact Tensor Kernels & Tile PBR** | `tensor` | `verified_engine_tensor_kernels`, `verified_unified_tensor_kernels` (9 kernels), `verified_coded_stream_tiers` (3 tiers), `render_triangles_tile_framebuffer` (`16x16` RGBA8 + depth) |
| **Native Wasm UORC & Zero-Decode Streaming** | `streaming` | `LodLevel` (`LOD0 -> LOD3`), `pack_progressive_uorc_stream`, `pack_portable_progressive_uorc_stream`, `decode_portable_uorc_stream_up_to_lod` (`uorc_decompress::run`), `CodedLodFrameBundle` |
| **Procedural Geometry & Morph Targets** | `geometry` | `ProceduralMesh`, `MorphTarget`, `generate_terrain_mesh`, `generate_fluted_entasis_column_mesh`, `generate_faceted_prism_mesh`, `generate_deformable_ribbon_mesh` |
| **Skeletal & Morph Animation** | `animation` | `Skeleton`, `Joint`, `SkinWeights4`, `SkeletalClip`, `KeyframePose`, `skin_mesh_linear_blend` (acyclic joint DAGs, exact 4-joint LBS contraction) |
| **Microfacet PBR, Frame Graph & WGSL** | `render` | `PbrMaterial`, `RenderFrameGraph` (5-pass acyclic barrier graph), `compute_csm_splits` (4 cascades), `IblEnvironmentProbe`, `validate_wgsl_shader` (W3C `naga`) |
| **Binary glTF 2.0 Import, Export & UORC Transcode** | `gltf_io` | `export_scene_to_glb`, `import_glb_to_scene` (meshes, skeletons, keyframe clips, PBR materials), `transcode_glb_to_progressive_uorc`, `validate_glb_bytes` (Khronos `gltf`) |
| **3D Convex Collision & Impulse Solver** | `collision` | `ConvexHullCollider`, `gjk_intersect_and_manifold` (tropical GJK support maps), `solve_contact_impulse` (Baumgarte stabilization, Coulomb friction cones) |
| **XPBD Joints, Swept Capsule & Raycast Vehicles** | `physics` | `PhysicsWorld`, `ArticulatedJoint` (`Hinge`, `BallSocket`, `Slider`, `SpringDamper` via Delassus `J * M^-1 * J^T`), swept BVH `CapsuleController`, 4-wheel `RaycastVehicle`, `KinematicSplineBody` |
| **NavMesh & Tropical Corridor Pathfinding** | `navmesh` | `NavMesh`, `NavPolygon`, `all_pairs_shortest_paths_milli` (tropical `(min, +)` closure), `steer_along_corridor` (bounded admissible `A*` corridor search) |
| **Spatial Audio, Occlusion & Modal Synthesis** | `audio` | `AcousticEnvironment` (Sabine & Eyring `RT60`), `evaluate_binaural_cue_with_occlusion` (BVH-raycasted acoustic low-pass attenuation), Hadamard `FDN`, `synthesize_modal_bar_modes` |
| **Job Graph, Rollback Netcode & Delta UORC** | `runtime` | `WorkerJobGraph` (6-stage DAG), `FrameBudgetProfiler` (`16,666 us` KPI), `WorldSnapshot` (`VDNT` + SHA-256), `DeterministicInputJournal`, 64-frame `RollbackSession`, `DeltaUorcSnapshot` |
| **Verified Prism Engine Kernel** | `prism_kernel` | Constructively proved `{0, 1}` engine invariants, LOD transition guards, XPBD/rollback bounds, and tensor accumulator bounds projected mechanically from Lean 4 (`Tensor.lex.tex`, `Subsystems.lex.tex`) |
| **Typed Engine Diagnostics** | `error` | `StreamingError`, `TensorComputeError`, `AssetValidationError`, `ShaderValidationError`, `PhysicsStepError`, `EngineConfigError`, `SnapshotCodecError`, `AnimationError`, `CollisionError`, `NavigationError`, `RenderGraphError` |

---

## Commercial Developer Kit, Studio, Starter Template & CLI

1. **Interactive 3D Browser Engine Studio (`site/`)**:
   - Real-time 60fps OffscreenCanvas worker (`site/engine-worker.mjs` + `site/verdant-wasm.wasm` + lazy-streamed `site/verdant-uorc.wasm`) inside the `120 KiB` mobile cold-boot budget, with WebGL2 rendering and automatic Prism `16x16` software tile fallback, progressive **LOD Scrubber** (`LOD0` -> `LOD3`), live **`.uorc` Wasm Decoder**, interactive **4-tab Architecture & Semiring Visual Inspector** (`#ve-viz-btn`), **Orbit / First-Person `WASD` Walk** camera switcher, `AudioWorklet` modal synthesis, `.glb` export, and `#vdnt=` SHA-256 URL snapshot sharing.
2. **Runnable Engine Examples (`crates/verdant-core/examples/`)**:
   - `cargo run -p verdant-core --example tensor_uorc_showcase` — Executes exact `uor-matmul` contractions, renders a `16x16` software tile PBR framebuffer, packs and verifies a 4-tier progressive `UORC` stream, validates built-in WGSL shaders, and exports binary glTF 2.0.
   - `cargo run -p verdant-core --example custom_game_arena` — Builds a custom 3D physics arena with capsule locomotion, rigid-body collisions, spatial audio, BVH queries, and `.glb` export.
   - `cargo run -p verdant-core --example headless_benchmark` — Profiles multi-worker frame scheduling, physics simulation throughput, and `VDNT` binary snapshot round-trips against the `16,666 us` (60 fps) frame budget KPI.
3. **Standalone Starter Template & CLI Packer (`templates/browser-3d-starter/`, `crates/verdant-cli`)**:
   ```sh
   cargo run --manifest-path templates/browser-3d-starter/Cargo.toml
   cargo run -p verdant-cli -- pack-scene scene.glb
   cargo run -p verdant-cli -- import-glb scene.glb
   cargo run -p verdant-cli -- transcode-glb scene.glb scene-progressive.uorc
   cargo run -p verdant-cli -- verify-uorc scene.glb.uorc
   cargo run -p verdant-cli -- benchmark
   ```
4. **Diátaxis Developer Documentation (`site/docs/` & `site/llms.txt`)**:
   - Dark-mode-first Diátaxis documentation with interactive architecture, zero-decode streaming, semiring matrix, and rollback netcode diagrams, mechanically checked against ISO 24495-1 plain-language and WCAG contrast gates (`node scripts/content/check.mjs site`).

---

## Engineering & Verification Discipline

- **Zero Magic Constants (`FI-08`)**: `model/parametric-ratchet.toml` locks every unexplained literal ceiling strictly to `0` across all shipped Rust crates (`crates/verdant-tensor/src/`, `crates/verdant-core/src/`, `crates/verdant-wasm/src/`, and `crates/verdant-cli/src/`) and all constructive backend specification modules (`Tensor.lex.tex`, `Subsystems.lex.tex`, `DevRel.lex.tex`).
- **100% Closed Prism Fidelity (`FI-01..FI-08`)**: `fidelity.toml` maintains `0` open questions across all three formal traceability axes (`derived = 100%`, `meant = 100%`, `measured = 100%`).
- **Authoritative Standards Oracles (`VE-01`, `VE-04`, `VE-05`, `VE-18`, `VE-19`)**: Progressive streams verify through `uorc` and `uorc_decompress` (`UORC-SHA256-CHAIN-v1`), imported and exported `.glb` binaries validate through Khronos `gltf`, and shipped WGSL shaders validate through W3C `naga`.
- **Single Normative Gate (`just vv`)**: Runs `cargo fmt --check`, `xtask validate` (`R1`–`R6`), SDK lock & template policy checks, provenance and zero-constants ratchet checks (`just fidelity`), workspace `clippy -D warnings`, all unit and conformance tests (`VE-01..VE-20`, `FR-01..FR-03`, `FI-01..FI-08`), `cargo deny`, and the verified WebAssembly guest archive build (`just app`).

See [CONFORMANCE.md](CONFORMANCE.md) for the generated capability ledger and [ENGINE-VERIFICATION.md](ENGINE-VERIFICATION.md) for the architectural and formal verification report.

---

## Repository Layout

| Path | Purpose |
| --- | --- |
| `crates/verdant-tensor/` | Pure `no_std + alloc` tensor-logic substrate over `uor-matmul` (`linear`, `coded`, `tropical`, `audio_ring`, `raster`, `prism_kernel`). |
| `crates/verdant-core/` | Standalone universal 3D game engine library (`math`, `ecs`, `tensor`, `streaming`, `geometry`, `animation`, `render`, `gltf_io`, `collision`, `navmesh`, `physics`, `audio`, `runtime`, `prism_kernel`, `error`) and runnable examples. |
| `crates/verdant-wasm/` | Safe WebAssembly simulation, portable tensor tile rasterization, native `uorc_decompress` + `coded_gemm` streaming, spatial acoustics, and `VDNT` snapshot worker bridge. |
| `crates/verdant-cli/` | Command-line `.glb -> .uorc` transcoder, progressive `.uorc` + `.glb` + `.vdnt` scene packer, Khronos glTF 2.0 & W3C WGSL validator, and headless 60fps benchmark (`verdant`). |
| `templates/browser-3d-starter/` | Ready-to-fork playable 3D Orbital Beacon Arena starter template built on `verdant-core`. |
| `site/` | Interactive 3D OffscreenCanvas + AudioWorklet Engine Studio (`card.html`, `card.mjs`, `visuals.mjs`, `engine-worker.mjs`, `scene-renderer.mjs`, `audio-worklet.mjs`), Diátaxis documentation (`site/docs/`), and `llms.txt`. |
| `features/suites/` | Executable Gherkin specifications (`engine.feature`, `frame.feature`, `fidelity.feature`). |
| `crates/conformance/` | End-to-end conformance and negative-refusal test suites (`VE-01..VE-20`, `FR-01..FR-03`, `CM-02`, `CM-03`). |
| `crates/fidelity/` | Provenance citation auditor, ISO/IEC 15939 measurement scorer, and permanent Zero-Constants Parametric Ratchet (`FI-01..FI-08`). |
| `app/` | Backend formal specification and Lean 4 verified Core-Wasm guest verifier (`Engine.lex.tex`, `Tensor.lex.tex`, `Subsystems.lex.tex`, `DevRel.lex.tex`). |
| `model/` | Normative capability register (`ids.toml`), zero-constants ratchet ceilings (`parametric-ratchet.toml`), charter, and content model. |

Licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE).
