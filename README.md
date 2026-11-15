# Verdant Engine

<p align="center">
  <a href="https://github.com/afflom/verdant-engine/actions/workflows/bootstrap.yml"><img src="https://github.com/afflom/verdant-engine/actions/workflows/bootstrap.yml/badge.svg?branch=main" alt="Engine Gate"></a>
  <img src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue" alt="License: MIT OR Apache-2.0">
</p>

**Verdant Engine** (`verdant-core`) is a standalone, commercial-grade 3D browser and WebAssembly game engine in the Unreal Engine tradition. Extracted from *Verdant Resonance* as a general-purpose 3D engine, Verdant Engine ships a deterministic Entity-Component-System (`SceneWorld`), AABB bounding-volume spatial queries (`BoundingVolumeHierarchy`), procedural terrain and L-system geometry (`ProceduralMesh`, `MorphTarget`), Cook-Torrance microfacet PBR materials (`PbrMaterial`), W3C WGSL shaders validated by `naga`, binary Khronos glTF 2.0 `.glb` export validated by `gltf`, symplectic Euler rigid-body and player-capsule physics (`PhysicsWorld`, `CapsuleController`), binaural Woodworth spatial audio with Sabine/Eyring reverberation and Euler-Bernoulli modal synthesis, acyclic multi-worker frame scheduling (`WorkerJobGraph`, `FrameBudgetProfiler`), and bit-exact IEEE-754 binary world snapshots (`WorldSnapshot`).

Beyond arbitrary custom 3D arenas and interactive applications, Verdant Engine natively models **causal world expansion (`t_0 -> t_obs`)**: terrain relief, mirror lagoons, mineral quartz outcrops, vascular flora, and seven non-natural architectural and kinetic structures—including the **Harmonic Travertine Rotunda**, **Solar Meridian Sundial**, **Modal Chime Pergola**, **Piezoelectric Torsion Chamber**, **Mirror Lagoon Pier Causeway**, **Geodesic Travertine Path Network**, and **Autonomous Perimeter Sentinel Rover**—emerge from five specialized **Builder Disciplines** (`LithicMasonry`, `CelestialGnomonics`, `ModalAcoustics`, `HydrologicalEngineering`, `KineticAutomation`) only when structural equilibrium (`e/t <= 1/6`, safety factor `>= 2.0`) and positive multi-objective utility hold.

---

## Core Engine Subsystems (`crates/verdant-core`)

| Subsystem | Module | Key Types & Standards |
| --- | --- | --- |
| **Linear Algebra & Spatial Transforms** | `math` | `Vec2`, `Vec3`, `Vec4`, `Quat`, `Mat4`, `Transform`, `Aabb`, `Frustum` (right-handed `-Z` forward, `+Y` up, reversed-Z WebGPU projection) |
| **Deterministic ECS & Spatial Queries** | `ecs` | `SceneWorld`, `EntityId` (generational slot allocator), `Parent` hierarchy propagation, `BoundingVolumeHierarchy` (6-plane Gribb-Hartmann frustum + Möller-Trumbore ray queries) |
| **Causal World Expansion & Builder Statics** | `genesis` | `CosmicEpoch` (`Cosmogenic -> LithicHydrologic -> BioticEcological -> AgenticArchitectural`), `BuilderDiscipline`, `ArchitecturalBlueprint`, `CausalWorldState` |
| **Procedural Geometry & Foliage** | `geometry` | `ProceduralMesh`, `Vertex`, `MorphTarget`, `generate_fbm_terrain`, `generate_uv_sphere`, `generate_cylinder`, `generate_box`, `generate_phyllotactic_flora`, `merge_meshes` |
| **Microfacet PBR & WGSL Shaders** | `render` | `PbrMaterial` (GGX NDF, Smith-Schlick geometry, Schlick Fresnel), `DirectionalSunLight`, `PointLight`, `AtmosphericScattering`, `WGSL_PBR_SURFACE_SHADER`, `WGSL_WATER_RIPPLE_SHADER`, `validate_wgsl_shader` (`naga` 29) |
| **Binary glTF 2.0 Scene Export** | `gltf_io` | `GltfSceneNode`, `export_glb`, `validate_glb` (Khronos glTF 2.0 binary `.glb` verified through the official `gltf` 1.4 parser) |
| **Rigid-Body, Capsule & Rover Physics** | `physics` | `PhysicsWorld`, `RigidBody` (symplectic Euler + impulse restitution), `CapsuleController` (slope-clamped ground contact), `KinematicSplineRover` (arc-length perimeter patrol) |
| **Spatial Audio & Modal Synthesis** | `audio` | `AcousticEnvironment` (Sabine & Eyring `RT60`), `ListenerPose`, `SpatialAudioSource`, `evaluate_binaural_cue` (Woodworth spherical-head ITD/ILD), `synthesize_euler_bernoulli_chime_modes` |
| **Worker Job Graph & Binary Snapshots** | `runtime` | `EngineConfig`, `WorkerJobGraph` (Kahn topological critical-path scheduler), `FrameBudgetProfiler` (`16.66ms` / `8.33ms` KPI gate), `WorldSnapshot` (`VDNT` + SHA-256 integrity digest) |
| **Typed Engine Diagnostics** | `error` | `EntityError`, `GenesisError`, `GeometryError`, `ShaderValidationError`, `GltfExportError`, `PhysicsStepError`, `EngineConfigError`, `SnapshotCodecError` |

---

## Commercial Developer Kit, Studio, Templates & Examples

1. **Interactive 3D Browser Engine Studio (`site/`)**:
   - Real-time 60fps WebGL2 3D viewport featuring procedural amphitheater terrain, animated Fresnel mirror lagoon water, mineral quartz outcrops, phyllotactic trees, all six stationary Builder monuments with rotating Cardan astrolabe rings, and the autonomous perimeter sentinel rover orbiting the sanctuary spline.
   - Interactive **Expansion Epoch Scrubber** (`t_0` Seed -> Lithic & Lagoon -> Biotic Flora -> `t_obs` Builders), **Orbit / First-Person `WASD` Walk** camera switcher, procedural WebAudio Euler-Bernoulli modal chimes, one-click binary `.glb` download, and in-worker structural verification.
2. **Runnable Engine Examples (`crates/verdant-core/examples/`)**:
   - `cargo run -p verdant-core --example cosmic_sanctuary` — Advances a seeded world from `t_0` to `t_obs`, audits all seven Builder blueprints, steps sentinel rover physics, validates WGSL shaders, and exports a binary glTF 2.0 scene.
   - `cargo run -p verdant-core --example custom_game_arena` — Builds an arbitrary custom 3D arena (central monolith, ring pillars, metallic spheres, BVH frustum/ray queries, capsule locomotion, spatial audio, and `.glb` export) with zero dependency on *Verdant Resonance* lore.
   - `cargo run -p verdant-core --example headless_benchmark` — Profiles multi-worker frame scheduling, 1,024-entity ECS + BVH spatial queries, physics simulation throughput, and `VDNT` binary snapshot round-trips against the `16,666 us` (60 fps) frame budget KPI.
3. **Standalone Starter Template (`templates/browser-3d-starter/`)**:
   - Drop-in Rust + WebGL2 starter project demonstrating custom procedural 3D world authoring, `.glb` asset generation, and live browser preview:
     ```sh
     cargo run --manifest-path templates/browser-3d-starter/Cargo.toml
     ```
4. **Diátaxis Developer Documentation (`site/docs/` & `site/llms.txt`)**:
   - Complete Tutorial, How-To Guide, API & Binary Format Reference, Architecture & Builder Cosmology Explanation, and Machine-Readable Agent Index (`llms.txt`), mechanically checked against ISO 24495-1 plain-language and Diátaxis structural gates (`node scripts/content/check.mjs site`).

---

## Engineering & Verification Discipline

- **Zero Magic Constants (`FI-08`)**: `model/parametric-ratchet.toml` holds every unexplained literal ceiling strictly to `0` across all 11 Rust modules in `crates/verdant-core/src/` and all constructive backend specification modules. Every physical, optical, acoustic, and geometric parameter enters through typed structs or named physical constants.
- **Authoritative Standards Oracles (`VE-04`, `VE-05`)**: Every exported `.glb` binary is parsed and validated by the Khronos `gltf` crate, and every shipped WGSL shader is parsed and type-checked by W3C `naga`—never by hand-rolled string or byte checks.
- **Single Normative Gate (`just vv`)**: Runs `cargo fmt --check`, `xtask validate` (`R1`–`R6`), SDK lock & template policy checks, provenance and zero-constants ratchet checks (`just fidelity`), workspace `clippy -D warnings`, all unit and conformance tests (`VE-01..VE-10`, `FR-01..FR-03`, `FI-01..FI-08`), `cargo deny`, and the verified WebAssembly guest archive build (`just app`).

See [CONFORMANCE.md](CONFORMANCE.md) for the generated capability ledger and [ENGINE-VERIFICATION.md](ENGINE-VERIFICATION.md) for the architectural and formal verification report.

---

## Repository Layout

| Path | Purpose |
| --- | --- |
| `crates/verdant-core/` | Standalone 3D game engine library (`math`, `ecs`, `genesis`, `geometry`, `render`, `gltf_io`, `physics`, `audio`, `runtime`, `error`) and runnable examples. |
| `templates/browser-3d-starter/` | Ready-to-fork 3D browser and `.glb` starter project built on `verdant-core`. |
| `site/` | Interactive 3D WebGL2 Engine Studio (`card.html`, `card.css`, `card.mjs`), Diátaxis documentation (`site/docs/`), and `llms.txt`. |
| `features/suites/` | Executable Gherkin specifications (`engine.feature`, `frame.feature`, `fidelity.feature`). |
| `crates/conformance/` | End-to-end conformance and negative-refusal test suites (`VE-01..VE-10`, `FR-01..FR-03`, `CM-02`, `CM-03`). |
| `crates/fidelity/` | Provenance citation auditor, ISO/IEC 15939 measurement scorer, and permanent Zero-Constants Parametric Ratchet (`FI-01..FI-08`). |
| `app/` | Backend formal specification and Lean 4 verified Core-Wasm guest verifier (`Engine.lex.tex`, `Genesis.lex.tex`). |
| `model/` | Normative capability register (`ids.toml`), zero-constants ratchet ceilings (`parametric-ratchet.toml`), charter, and content model. |

Licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE).
