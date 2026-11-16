# Verdant Engine Verification & Engineering Report

This report documents the verification boundary, standards validation oracles, unified semiring tensor contraction and zero-decode `UORC` streaming theorems, XPBD articulation and deterministic rollback invariants, performance budgets, and zero-constants enforcement governing **Verdant Engine** (`verdant-tensor`, `verdant-core`, `verdant-wasm`, `verdant-cli`, and the browser Engine Studio).

---

## 1. Engine Conformance Matrix (`VE-01` – `VE-31`)

Every capability of `crates/verdant-tensor`, `crates/verdant-core`, `crates/verdant-wasm`, and `crates/verdant-cli` is registered in `model/ids.toml`, specified as an executable Gherkin scenario in `features/suites/engine.feature`, mechanized across all 8 Prism strata (`app/src/*.lex.tex`), and verified with both positive assertions and typed negative-refusal tests in `crates/conformance/tests/engine.rs`:

| ID | Subsystem | Positive Verification | Typed Refusal & Negative Test |
| --- | --- | --- | --- |
| `VE-01` | Progressive Multi-LOD `UORC` Streaming | Monotonic `LOD0 -> LOD1 -> LOD2 -> LOD3` synthesis and `UORC-SHA256-CHAIN-v1` multi-object lossless round-trip via `uorc` | `StreamingError::NonMonotonicLodTransition`, `StreamingError::InvalidLodRank`, `StreamingError::UorcArchiveRejected` (`engine_ve_01`) |
| `VE-02` | Exact `uor-matmul` Tensor Kernels & Tile PBR | Certified 7-kernel baseline + 15-kernel unified specification, exact `N x 4` homogeneous transforms, tropical `(max, +)` frustum bounds, Q15 audio mixing, and `16x16` RGBA8 tile rasterization | `TensorComputeError::ZeroOrMismatchedExtents`, `TensorComputeError::NonFiniteTensorEntry`, `TensorComputeError::InvalidTileDimensions` (`engine_ve_02`) |
| `VE-03` | Archetype ECS & SAH BVH | Hierarchical `Mat4` world transforms via exact `verdant-tensor` contractions, 6-plane Gribb-Hartmann frustum culling, and Möller-Trumbore raycasting | Deterministic miss and out-of-frustum exclusion (`engine_ve_03`) |
| `VE-04` | Procedural Geometry & Binary glTF 2.0 | Heightfields, columns, ribbons, morph targets, and `.glb` export validated by the official Khronos `gltf` 1.4 crate | `AssetValidationError::EmptyMeshTopology`, `AssetValidationError::InvalidTriangleIndices` (`engine_ve_04`) |
| `VE-05` | Microfacet PBR & Portable WGSL Shaders | Energy-conserving Cook-Torrance GGX BRDF and W3C `naga` front-end + module validation across all 4 built-in shaders | `ShaderValidationError::WgslParseRejected`, `ShaderValidationError::MissingEntryPoint` (`engine_ve_05`) |
| `VE-06` | Rigid-Body, Capsule & Spline Physics | Fixed-step symplectic Euler integration, Archimedes buoyancy, capsule ground clamping, and closed-loop kinematic spline motion | `PhysicsStepError::InvalidBodyMassOrRadius`, `PhysicsStepError::InvalidTimeStepSeconds` (`engine_ve_06`) |
| `VE-07` | Spatial Audio & Modal Bar Synthesis | Woodworth spherical-head binaural ITD/ILD cues, Sabine & Eyring `RT60` reverberation, and Euler-Bernoulli modal bar eigenfrequencies | Positive acoustic separation and monotonic modal progression (`engine_ve_07`) |
| `VE-08` | Multi-Worker Job Graph & Frame KPI | Kahn topological critical-path scheduling across worker threads and 60Hz (`16,666 us`) frame-budget profiling | Cyclic dependency rejection, `EngineConfigError::ZeroViewportDimension`, `EngineConfigError::ZeroWorkerThreads` (`engine_ve_08`) |
| `VE-09` | Bit-Exact `VDNT` World Snapshots | Deterministic little-endian IEEE-754 snapshot serialization with 32-byte SHA-256 integrity verification | `SnapshotCodecError::TruncatedSnapshot`, `SnapshotCodecError::IntegrityDigestMismatch` (`engine_ve_09`) |
| `VE-10` | Formal Core-Wasm Engine Verifier | Constructive Lean 4 proofs (`streamingLodsStrictlyMonotonic`, `hdStreamingLodsStrictlyMonotonic`, `unifiedTensorAndCodedTiersVerified`, `architecturalWalkabilityVerified`, `engineSubsystemsVerified`, `invalidTensorKernelsRefused`), zero `Classical.choice` axioms, and 41 sealed byte-request vectors | `refused:zero-extent-contraction`, `refused:accumulator-below-laurent-bound`, `invalid request`, and `ComputeError::MulOverflow` (`engine_ve_10`) |
| `VE-11` | Skeletal Rigging & Linear-Blend Skinning | Acyclic joint trees, normalized 4-joint LBS weights contracted via `verdant_tensor::skin_vertices_linear_blend_exact`, and keyframe/morph sampling | `AnimationError::InvalidJointHierarchy`, `AnimationError::UnnormalizedOrInvalidSkinWeights` (`engine_ve_11`) |
| `VE-12` | Cascaded Shadows, IBL & Render Graph | Monotonic 4-cascade CSM frustum splits, GGX IBL mip counts, and 5-pass acyclic render barrier validation | `RenderGraphError::InvalidShadowFrustumBounds`, `RenderGraphError::InvalidPassDependency` (`engine_ve_12`) |
| `VE-13` | 3D GJK Collision & Impulse Solver | Bounded 3D GJK Minkowski support manifolds via `verdant_tensor::tropical_gjk_support_batch`, Baumgarte stabilization, and Coulomb friction cones | `CollisionError::EmptyOrDegenerateHull` (`engine_ve_13`) |
| `VE-14` | Slope-Filtered NavMesh & `A*` Steering | Walkable terrain polygon extraction, tropical `(min, +)` all-pairs closure, admissible `A*` corridor search, and unit steering vectors | `NavigationError::InvalidPolygonIndex`, `NavigationError::UnreachableGoal` (`engine_ve_14`) |
| `VE-15` | WebAssembly Worker Runtime Bridge | Deterministic `WasmRuntimeBridge` physics stepping, portable `16x16` tensor framebuffer rendering, and `VDNT`/`UORC` round-trips | `WasmBridgeError::InvalidLodRank`, `WasmBridgeError::CommandRingOverflow`, `WasmBridgeError::InvalidSnapshotEnvelope` (`engine_ve_15`) |
| `VE-16` | `verdant-cli` Scene Packer & Validator | `pack-scene`, `import-glb`, `transcode-glb`, `pack-universe`, `verify-story`, `verify-uorc`, `validate-glb`, `validate-wgsl`, `snapshot-verify`, and headless `benchmark` | `CliCommandError::InvalidUsage`, `CliCommandError::ValidationRejected` (`engine_ve_16`) |
| `VE-17` | Unified Semiring Tensor Substrate (`verdant-tensor`) | Exact Delassus `J * M^-1 * J^T` contraction, tropical `(max, +)` GJK batch support, tropical `(min, +)` NavMesh closure matching `A*`, and modal FIR + Hadamard `FDN` reverb | `TensorComputeError::ZeroOrMismatchedExtents` (`engine_ve_17`) |
| `VE-18` | Native Browser `uorc_decompress` + `uor_matmul::codec` Zero-Decode | Portable 4-frame `UORC-SHA256-CHAIN-v1` decoding via `uorc_decompress::run` and zero-scratch `coded_gemm` over `Grid`, `Packed`, and `Runs` | `StreamingError::UorcArchiveRejected`, `TensorComputeError::ZeroOrMismatchedExtents` (`engine_ve_18`) |
| `VE-19` | Full `.glb` Ingestion, `.uorc` Transcoding, XPBD Joints & Vehicles | Khronos `.glb` scene/skeleton/clip/material import, `.glb -> .uorc` progressive transcoding, XPBD Hinge/BallSocket/Slider/SpringDamper joints, swept BVH capsule, 4-wheel `RaycastVehicle`, and BVH acoustic occlusion | `AssetValidationError::GltfValidationFailed`, `PhysicsStepError::InvalidJointConfiguration` (`engine_ve_19`) |
| `VE-20` | Deterministic Rollback Session & Delta `UORC` Snapshots | Strictly ordered SHA-256 `DeterministicInputJournal`, 64-frame `RollbackSession` bit-exact resimulation parity, and XOR-residual `DeltaUorcSnapshot` round-trips | Out-of-order journal frame refusal, out-of-window rollback refusal, and tampered delta SHA-256 rejection (`engine_ve_20`) |
| `VE-21` | Formal 2D & Diegetic 3D UI Layout + WCAG 2.2 Oracle | Tropical `(min, +)` and linear constraint layout solving, diegetic 3D world-space BVH ray picking, `>= 48px` touch targets, and WCAG 2.2 AA contrast verification | `UiComplianceError::TouchTargetTooSmall`, `UiComplianceError::ContrastBelowThreshold`, `UiComplianceError::FocusUnreachable` (`engine_ve_21`) |
| `VE-22` | Neuromorphic LIF Salience & Zero-Decode `coded_gemm` SLM | Q15 leaky-integrate-and-fire (`7/8` decay) mutual-attention recurrence, `Packed`/`Runs` `CodedSlmDirector` intent routing, and denial-wins browser container capability gating | `CyberneticError::ContainerOrganDenied`, `CyberneticError::EmptyOrOversizedPrompt` (`engine_ve_22`) |
| `VE-23` | Zero-Copy Packed World-Space Mesh Buffers | Hierarchical `SceneWorld` and imported `.glb` packing into contiguous `f32`/`u32` WebAssembly vertex, normal, color, and index buffers shared by WebGL2 and `16x16` tile PBR | Empty and out-of-bounds mesh rejection (`engine_ve_23`) |
| `VE-24` | Sovereign Prism ID Journals & 2-of-3 Stitch Custody | Non-extractable WebCrypto `prism/identity/4` journal envelopes, 2-of-3 SHA-256 XOR `StitchWorldCustody` recovery, and survival-entropy peer liveness | `StitchCustodyError::DuplicateOrInvalidFragment`, `StitchCustodyError::DigestMismatch`, `StitchCustodyError::InvalidJournalSignature` (`engine_ve_24`) |
| `VE-25` | Two-Level `MeshBlas` & Swept Collide-and-Slide CCD | SAH entity TLAS + triangle `MeshBlas` BVH with continuous swept-sphere/capsule TOI, `0.35m` stair auto-stepping, and acute two-plane `n1 × n2` crease sliding | Guaranteed non-penetration (`clearance >= r_capsule + margin`) (`engine_ve_25`) |
| `VE-26` | Lawful Near-Sphere Camera Rig | Spring-arm orbit, 3rd-person, and 1st-person eye cameras sweeping an exact near-frustum diagonal guard sphere against `TwoLevelBvh` and terrain | `CameraRigError::InvalidClipBounds`, `CameraRigError::InvalidFieldOfView`, `CameraRigError::ExcessiveArmDistance` (`engine_ve_26`) |
| `VE-27` | High-Definition `LOD0..LOD4` Citadel & Arch Bridge | Constructive Groin-Vaulted Citadel Hall, Parabolic Stone Arch Bridge, multi-octave sculpted terrain, and canopy clusters (`> 45,000` tris at `LOD4`), Khronos `.glb` validated | Portal and staircase walkability verified (`engine_ve_27`) |
| `VE-28` | 22-Joint Articulated Humanoid, Two-Bone Foot IK & Possession | Normalized 4-joint LBS skinning + morphs, analytical cosine-law two-bone Foot IK stair adaptation, 1st-person head masking, and external `.glb` rig binding | `AnimationError::EmptySkinnedMesh` (`engine_ve_28`) |
| `VE-29` | Floating-Origin `UniversePosition` & `PortalGraph` | Exact `i64` 3D sector coordinates (`128m` cells) with bounded `[-64m, +64m)^3` local offsets, sub-millimeter `f32` camera-relative matrices, momentum-preserving portals, and CLI `pack-universe` | Non-finite coordinate and invalid portal rejection (`engine_ve_29`) |
| `VE-30` | Verified `StoryAutomaton` & `ConservationLedger` | Tropical `(min, +)` milestone reachability closure, exact integer supply conservation (`before + minted - burned == after`), SLM-guided NPC dialogue, and CLI `verify-story` | `NarrativeLawError::UnreachableOrDeadlockedState`, `NarrativeLawError::ConservationViolated`, `NarrativeLawError::InvalidPortalOrUniverseCell` (`engine_ve_30`) |
| `VE-31` | Stratum-0 Ontological Telos, W5H Certificates & Full Prism Closure | `OntologicalRuntimeGovernor` synthesizing constructive `W5HCertificate`s across all 8 causal strata (`0..7`), `HostEnvelope`/`WitnessEnvelope` telos loss evaluation, non-coercive parametric LOD descent, and full Core-Wasm `summary:hd` / `lod:4` / `scene:telos` dispatch | Coercive coordinate lock, uncertified stratum opacity debt, and over-budget host ceiling excess detection (`engine_ve_31`) |

---

## 2. Permanent Zero-Constants Parametric Ratchet (`FI-08`)

`model/parametric-ratchet.toml` permanently locks every literal ceiling to `0`:

- `max_ontology_root_literal_gt_one_count = 0`
- `max_ontology_root_string_field_count = 0`
- `max_classical_choice_count = 0`
- `max_ungrounded_lex_module_count = 0`
- `max_domain_lex_magic_literal_count = 0`
- `max_engine_rust_magic_literal_count = 0`

On every `just fidelity` and `just vv` run, `crates/fidelity/src/ratchet.rs` scans all 25 backend `.lex.tex` modules and all shipped Rust source files across `crates/verdant-tensor/src/`, `crates/verdant-core/src/`, `crates/verdant-wasm/src/`, and `crates/verdant-cli/src/`. Any unexplained numeric literal outside `{0, 1}` / `{0.0, 1.0}` fails the build immediately, and `zero_constants_parametric_ratchet_fi_08` verifies that planted magic float and integer literals in temporary workspaces are deterministically caught and refused.

---

## 3. Browser Frame, Responsive Viewport & Wire-Weight Gates (`FR-01` – `FR-03`)

The interactive 3D Engine Studio (`site/card.html`, `site/card.css`, `site/card.mjs`, `site/engine-worker.mjs`, `site/scene-renderer.mjs`, `site/studio.mjs`, `site/stitch-mesh.mjs`) and its Diátaxis documentation (`site/docs/`, `site/llms.txt`) are gated by four automated checks:

1. **Brand & Frame Integrity (`FR-01`, `FR-02`)**: `bootstrap/brand-sync.mjs --check`, `bootstrap/brand-fonts.py --check`, and `bootstrap/frame-sync.mjs --check` verify byte-for-byte lock alignment against `brand.lock` and `frame.lock`, zero inline styles, and zero stylesheet color literals.
2. **Material 3 Window-Class Matrix**: `bootstrap/screens.mjs` renders all 6 window classes (`compact 360x780`, `compact-height 720x400`, `medium`, `expanded`, `large`, `extra-large`) across dark, light, and 200%-zoom configurations, enforcing zero viewport scroll overflow and 48px touch targets.
3. **First-Visit Wire Budget (`FR-03`)**: `bootstrap/weight.mjs` verifies that a cold mobile visit stays strictly within the 100 KiB frame budget, 120 KiB application budget (`verdant-uorc.wasm` lazy-loads on demand so cold boot stays under budget), and 28-request ceiling.
4. **Diátaxis & ISO 24495-1 Content Gate**: `scripts/content/check.mjs` audits every generated documentation page (`CS-01`–`CS-11`) for structural navigation, WCAG AA contrast across dark and light modes, copyable code examples, plain-language readability, and valid internal cross-references.

---

## 4. Reproducing the Complete Verification Gate

Inside the digest-pinned SDK container (`prismpm.lock`):

```sh
just vv
```

To run the standalone starter template, CLI transcoder/packer, and engine examples:

```sh
cargo run -p verdant-core --example tensor_uorc_showcase
cargo run -p verdant-core --example custom_game_arena
cargo run -p verdant-core --example headless_benchmark
cargo run --manifest-path templates/browser-3d-starter/Cargo.toml
cargo run -p verdant-cli -- benchmark
```
