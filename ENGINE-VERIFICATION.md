# Verdant Engine Verification & Engineering Report

This report documents the verification boundary, standards validation oracles, unified semiring tensor contraction and zero-decode `UORC` streaming theorems, XPBD articulation and deterministic rollback invariants, performance budgets, and zero-constants enforcement governing **Verdant Engine** (`verdant-tensor`, `verdant-core`, `verdant-wasm`, `verdant-cli`, and the browser Engine Studio).

---

## 1. Engine Conformance Matrix (`VE-01` – `VE-32`)

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
| `VE-10` | Formal Core-Wasm Engine Verifier | Constructive Lean 4 proofs (`streamingLodsStrictlyMonotonic`, `hdStreamingLodsStrictlyMonotonic`, `unifiedTensorAndCodedTiersVerified`, `architecturalWalkabilityVerified`, `engineSubsystemsVerified`, `invalidTensorKernelsRefused`), zero `Classical.choice` axioms, and 42 sealed byte-request vectors | `refused:zero-extent-contraction`, `refused:accumulator-below-laurent-bound`, `invalid request`, and `ComputeError::MulOverflow` (`engine_ve_10`) |
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
| `VE-32` | Verdant Studio Themes, 6-Tab Workbench, Journeys, Scaffolder & Live Wasm Docs | Core-Wasm `scene:studio` verification (`themes=3,tabs=6,journeys=7,archetypes=4,doc_embeds=6,controls=13`), 100% Hologram-Warm W3C DTCG token purity, live `SceneWorld` outliner C-ABI, 4 zero-constants game/app archetypes, and static cold-boot vs UORC WebAssembly export parity | Raw hex color refusal, out-of-range archetype/embed refusal, silent catch refusal, and missing C-ABI export detection (`engine_ve_32`) |

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
5. **End-to-End Live WebAssembly Surface Gate**: `scripts/verify-live-surfaces.mjs` drives headless Chromium across the root Studio (`Light`, `Dark`, and `Immersive` themes, 6-tab Workbench, executable Journeys, SceneWorld Outliner, Project Scaffolder, and Stitch Mesh) and all 6 `/docs/*` pages, verifying zero runtime errors and interactive readiness on every live WebAssembly figure.

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

## VTK-16 symmetric block encoding

The pinned GGML function `quantize_row_q8_0_ref` supplies all 34 expected
bytes of each of 271 blocks. These inputs cover signed zeros, scale underflow,
halfway cases of both signs, binary16 scale overflow and mixed F32 exponents.
Four additional inputs and a wrong-length block exercise the modeled refusals. Stored scale and lane
codes match in all 9,214 byte comparisons. The first registered test failed
to compile because the public encoder was missing. Flipping each encoded
lane's low bit made the oracle reject block 3, byte 2 (17 versus 16).
The restored encoder passed. Reproduce the test with
`cargo test --locked -p verdant-tensor --lib symmetric_block_encoding_vtk_16`.
The independent Prism scope exports via `just certified-export q8 tensor-q8`.
[GGML reference](https://github.com/ggml-org/llama.cpp/blob/9a080f44b503c64f66c84a6274b9d9ee5d5b09d5/ggml/src/ggml-quants.c#L273)

Prism attestation `313a4bfed6b4563a4e405bf58fb4c88ee5bd7676f8dbe476353d23e444cf512a`
binds the final block layout and arithmetic instructions. The scoped oracle,
Clippy and cargo-deny pass. The Engine fidelity gate reports 1,168 native
residue functions against 1,160 and 1,326 uncertified functions against 1,318.
The preceding PR measured 1,167 and 1,325 respectively.

## VTK-17 symmetric row contraction

The public row API accepts encoded weight bytes and typed activation blocks.
Verdant's existing uor-matmul dependency supplies the exact signed integer dot.
The Prism projection widens each binary16 scale, multiplies the scales, weights
the integer dot and accumulates in F32 block order. It admits matching complete
32-lane rows before reading them. Attestation
`74a961acf6a2947760f675e2e936d63e9d2c89f49e08051d8d284dc3538fea20`
binds both exported roots.

The pinned CPU's Q8_0 vector-dot callback supplies the expected F32 word for
256 rows, including empty rows, signed extreme coefficients, cancellation,
subnormal scales, infinities and NaNs. Five malformed layouts are refused.
Binding both operands to the weight codes fails row 2 (1151092021 versus
3274873258). Restore and run `cargo test --locked -p verdant-tensor --lib
symmetric_row_dot_vtk_17` to repeat the oracle.
[Upstream scalar reduction](https://github.com/ggml-org/llama.cpp/blob/9a080f44b503c64f66c84a6274b9d9ee5d5b09d5/ggml/src/ggml-cpu/quants.c#L499)

The restored encoder and contraction oracles, component Clippy, and cargo-deny
passed in the locked Engine SDK. Fidelity reports 1,169 native residue
functions against a ceiling of 1,160 and 1,327 uncertified functions against
1,318. This row API adds one native bridge to the preceding checkpoint.

## VTK-18 packed four-bit row contraction

At registration `69ca3ad`, the public packed-row API was missing. The
restored API contracts 32-lane symmetric four-bit weight blocks against
prepared signed-byte activations. Prism determines nibble placement and
the order of the two F32 products and addition. Existing certified
conversion and scalar primitives execute each instruction. The Q8 and Q4
paths share the exact signed integer dot product.

The pinned GGML CPU Q4_0 vector-dot callback supplied every expected output
for 256 rows, including mixed signs, cancellation, asymmetric nibbles,
subnormal scales, empty rows, infinities and NaNs. Five invalid extents were
refused. Swapping the low and high lane halves failed on row 2 (1094989686
versus 3246198874). Restoring the lane order passed. VTK-17 passed its
original 256-row comparison against the separate Q8_0 callback in the same
run. Packed-plan attestation
`921073e9e0db94e98395260de8e0b4fe3b3884bbe31d18396ad726512998a386`.
[Certified package](generated/tensor-packed/result.json)

The locked-SDK component Clippy check passed. The full Engine fidelity gate
reports 1,171 native residue functions against 1,160 and 1,329 uncertified
functions against 1,318. The parent measured 1,169 and 1,327.

## VTK-19 binary32 exponential

The public `linear::exponential_f32` accepts one binary32 word. The Prism
reduction handles NaNs, signed zeros and infinities. For the remaining range
it represents the input magnitude with 50 fractional bits and divides by
round(2⁵⁰ ln 2). This yields a power-of-two scale and a nonnegative remainder
below ln 2. A 12-term Taylor recurrence evaluates the reduced exponential
with 31 fractional bits. The certified scaling stage rounds to nearest even
at the final normal or subnormal precision. The Rust adapter passes words
between the three compiled Prism roots.

The imported oracle runs the unchanged `ggml_vec_exp_f32` from llama.cpp
`9a080f44`. The 166,856 distinct inputs cover every binary32 exponent with
both signs, neighboring words around powers of two in the result, and a
1/256-spaced sweep of the active range. The check requires the reference
zero, infinity or NaN class at those endpoints and allows one ULP for finite
positive outputs. [Imported authority](https://github.com/ggml-org/llama.cpp/blob/9a080f44b503c64f66c84a6274b9d9ee5d5b09d5/ggml/src/ggml-cpu/vec.h#L985)

All 166,856 comparisons passed. Omitting the final recurrence term failed
on input `0x34fffffe`. Verdant returned `0x3f800002` against the oracle's
`0x3f800004`. Restoring the term passed the suite. Scoped Clippy and
cargo-deny also passed. The three compiled roots carry attestation
`7ae660150f63832441d4ec0310f032f5a6c08bf612e1a8f58c2f8e2d1ccf62b3`.
[Certified artifacts](generated/tensor-exponential/result.json)

The native source lift includes this module and its Rust adapter. Verdant's
full release gate stops at fidelity. The refreshed frontier measures 1,172
native residue functions against 1,160 and 1,330 uncertified functions
against 1,318. The PR #15 parent measured 1,171 and 1,329.

## VTK-20 binary32 SwiGLU

Verdant accepts two binary32 values and returns their gated product. The
compiled Prism program sequences negation, exponential, addition, division
and multiplication. The public adapter transports the intermediate words
between the certified scalar operations. GGML descriptors, operand selection
and row addressing remain in the consumer. [Certified artifact](generated/tensor-swiglu/result.json)

All 110,700 operand pairs matched the imported scalar GGML oracle within
four ULP for finite nonzero outputs, with exact signs for zeros and infinities
and matching NaN classification. Skipping addition produced `0x80000001`
for the smallest positive subnormal gated by −1. The oracle returned
`0x80000000`. Restoring the stage passed the suite, scoped Clippy and
cargo-deny. [Imported evaluator](https://github.com/ggml-org/llama.cpp/blob/9a080f44b503c64f66c84a6274b9d9ee5d5b09d5/ggml/src/ggml-cpu/vec.cpp#L500)

The refreshed native frontier measures 1,173 residual functions and 1,331
uncertified functions against ceilings of 1,160 and 1,318. PR #16 measured
1,172 and 1,330. The published release gate remains open.

## VTK-21 binary32 sine and cosine

`linear::sin_cos_f32` returns both results from one range reduction. Five
compiled Prism roots classify the input, reduce the angle, square the
reduced argument, evaluate its two magnitudes, and restore quadrant signs.
Tiny inputs preserve the sine word and return one for cosine. Nonfinite
inputs return a NaN pair. The Rust adapter passes values between the roots.
[Certified package](generated/tensor-circular/result.json)

The reducer multiplies the binary32 significand by a window of the first
256 fractional bits of 2/π. It retains the quadrant and 61 fractional
bits before folding around the nearest multiple of π/2. The constants
were computed with Machin's formula and checked against the leading digits
in fdlibm. On the resulting interval the model uses fixed-point series for
cosine and sinc. Keeping the fractional magnitude separately until the
final rounding preserves small sines close to a quadrant boundary.
[Range-reduction constant](https://netlib.org/fdlibm/e_rem_pio2.c)

The pinned, unchanged `ggml_vec_sin_f32` and `ggml_vec_cos_f32` supplied
520,568 results for 260,284 distinct binary32 words. Inputs covered every
exponent field, all widened binary16 and bfloat16 encodings, neighbors of
8,193 multiples of π/2, and large angles close to quadrant boundaries.
Every finite nonzero result agreed within one ULP, with the reference
sign. Signed zeros matched exactly and all NaN classifications matched.
Inverting the reducer's reflection flag returned `0xb9000000` for the sine
of `0x39000000`; the oracle returned `0x39000000`. Restoring the flag
passed the same batch. Scoped Clippy and cargo-deny passed.
[GGML scalar functions](https://github.com/ggml-org/llama.cpp/blob/9a080f44b503c64f66c84a6274b9d9ee5d5b09d5/ggml/src/ggml-cpu/vec.h#L913)

The frontier measures 1,174 residual functions and 1,332 uncertified
functions against ceilings of 1,160 and 1,318. PR #17 measured 1,173
and 1,331. The published release remains open.

## VTK-22 binary32 logarithms and reciprocal roots

Eight certified Prism roots implement the binary32 logarithm and the negative reciprocal-root power. Both use a fixed-point logarithm with 55 fractional bits. The power uses a 16-term exponential recurrence before final binary32 rounding. The locked SDK exported result `8ce219d6438e79ee7fa9ba4f4ebad55277daaa58f14d189bae3d49fd1aedee98` with attestation `fa69b72b2fe05ed6049b55cef917c3967091c0f53093c22280dccb5bff3676f8`.

VTK-22 compared 24,578 inputs with the imported GGML vector logarithm, covering every exponent, both sides of one and IEEE special cases. It also observed the unmodified GGML rotary kernel's power call on 13 positive bases and 20 degrees. All 260 ratios met the one-ULP bound. Bases 10,000 and 1,000,000 matched at every tested degree bit for bit. The earlier Q31 calculation returned `0x3f5dafd6` for base 10,000 at degree 64, one word below the upstream result. Reversing the exponent sign in the corrected adapter returned the smallest positive subnormal in place of positive infinity. Restoring it passed the same oracle, scoped Clippy and cargo-deny. [Imported logarithm](https://github.com/ggml-org/llama.cpp/blob/9a080f44b503c64f66c84a6274b9d9ee5d5b09d5/ggml/src/ggml-cpu/vec.h) · [Imported frequency calculation](https://github.com/ggml-org/llama.cpp/blob/9a080f44b503c64f66c84a6274b9d9ee5d5b09d5/ggml/src/ggml-cpu/ops.cpp#L6369)

The full locked-SDK gate reached the native fidelity check. The refreshed frontier measured 1,177 residual and 1,335 uncertified functions against ceilings of 1,160 and 1,318. The preceding patch measured 1,174 and 1,332.

## VTK-23 plane rotations

The certified program returns the register addresses for each coordinate pair
and each frequency configuration. Rust supplies the input words and interprets
the actions with Verdant primitives. The imported GGML rotary kernel passed
`cargo test --locked -p verdant-tensor --lib plane_rotation_vtk_23` in the
locked SDK. Reading the first result register twice produced 0.75390226 for
GGML's −0.6569866 at dimension 16, position −7, lane 0. Restoring the
instruction-selected registers passed the same oracle. Scoped Clippy and
cargo-deny passed. The outstanding engine fidelity checks concern the native
frontier and self-host manifest. [Prism result and attestation](generated/tensor-plane/result.json)

## VTK-24 streaming weighted mean

The locked SDK certified the pivot, coordinate blend and reciprocal-mass
exports. The unchanged GGML CPU FLASH_ATTN_EXT evaluator supplied 5,940
reference output coordinates over 360 graphs. Unit queries made each key
the observation score. Values covered widths 1, 4, 13 and 48, increasing,
decreasing and tied scores, large differences, masked and all-masked rows,
and absent, negative and positive sinks. Maximum permitted absolute error
was 0.000001. Passing a mismatched width preserved the empty accumulator.

Reversing the new-maximum branch in the coordinate blend returned 0
where GGML returned −0.75. Restoring the branch passed the batch. Scoped
Clippy and cargo-deny passed. The full Engine gate reached the fidelity
ratchet and refused the logarithm and rotary stack. Release acceptance
remains open. [Prism artifact](generated/tensor-weighted/result.json)

## VTK-25 binary32 sigmoid and SiLU

The exported Logistic step reuses the certified exponential and scalar
operations of SwiGLU. It selects either one or the input as the numerator
and returns unary results before the optional multiplication by a gate.
Verdant exposes `sigmoid_f32` and `silu_f32`. The existing SwiGLU entry
point uses the same driver. [Prism export](generated/tensor-logistic/result.json)

The pinned GGML functions `ggml_vec_sigmoid_f32` and `ggml_silu_f32` supplied
18,460 reference results across 9,230 input words. The test checks signed
zeros, infinity and NaN classes and a four-ULP bound on other finite results.
Selecting the input as the sigmoid numerator returned zero for +0. The
imported function returned 0.5. Restoring the unit numerator passed the
entire corpus. All 110,700 existing gated-product comparisons also passed.
[CPU scalar definitions](https://github.com/ggml-org/llama.cpp/blob/9a080f44b503c64f66c84a6274b9d9ee5d5b09d5/ggml/src/ggml-cpu/vec.h)

The engine operator regenerated the self-host manifest with `ratchet-lift
--isolated`. It counted 213 lifted modules and zero unlifted workspace
modules. The fidelity gate now reports only the Rust frontier, whose recorded
counts are 1,177 residue and 1,335 uncertified functions against ceilings of
1,160 and 1,318. Its source-tree binding also needs regeneration.

## VTK-26 grouped activation blocks

Verdant encodes 256 binary32 lanes into the Q8_K scale, signed coefficients and sixteen signed partial sums. The Prism model selects the first signed absolute maximum, rounds ties to even and emits the scalar operations. The public adapter accepts a fixed-size array. [Certified export](generated/tensor-grouped/result.json)

All 38,544 bytes of 132 blocks matched the pinned GGML reference, including both signs of zero, tied extrema in both orders, halfway values and 128 seeded blocks. The reference leaves partial sums untouched for an all-zero block. Both outputs were zero-initialized. The encoder also refused each nonfinite input and the unrepresentable reciprocal. [Reference encoder](https://github.com/ggml-org/llama.cpp/blob/9a080f44b503c64f66c84a6274b9d9ee5d5b09d5/ggml/src/ggml-quants.c#L2784)

Reversing the extremum traversal changed byte 3 of block 2 from 63 to 191. Restoring input order passed the full grouped block regression. The existing 32-lane encoder regression, locked tensor Clippy and dependency checks passed.

The isolated engine lift bound the grouped encoder and its storage dimensions. The engine-wide fidelity gate still reports the existing native frontier (1,177 residue and 1,335 uncertified functions against ceilings of 1,160 and 1,318) and a stale frontier source digest. This leaves the full engine release gate open.

## VTK-27 grouped two-bit row product

Verdant accepts Q2_K weight bytes and prepared Q8_K activation blocks. Prism selects the packed plane, group scale and minimum, then orders the binary32 operations. Two integer contractions evaluate the scaled coefficients and the group minimum contributions. The caller supplies storage slices. [Certified export](generated/tensor-groupeddot/result.json)

All 97 row results, spanning zero through four blocks, matched the imported GGML CPU vector product bit for bit. Both mismatched-length probes were refused. Replacing the minimum nibble with the scale nibble returned −1,096 instead of −1,615.1875 on row 1. Restoring the correct field passed the row product and activation encoder regressions. [Reference kernel](https://github.com/ggml-org/llama.cpp/blob/9a080f44b503c64f66c84a6274b9d9ee5d5b09d5/ggml/src/ggml-cpu/quants.c#L613)

Tensor Clippy, cargo-deny and the isolated engine lift passed. The complete engine gate reaches the same native-frontier failure recorded under VTK-26. The grouped product introduces no additional fidelity diagnostic.

### VTK-28 binary32 hyperbolic tangent

Verdant supplies the reusable binary32 activation for unary operations and attention score capping. The Prism model preserves signed zeros and saturates both tails. Near zero it evaluates the odd Taylor polynomial through degree 15 with certified binary32 arithmetic. Elsewhere it uses the existing binary32 exponential at twice the negative input magnitude. The GGML CPU function supplied 128,870 independent expected words. Signed zeros, saturation at ±1 and NaN classes matched. All other finite results were within four representable steps. Reversing the exponent sign returned −0.46250105 for input 0x3f002000 where the CPU returned +0.4625011. Restoring the exponent passed the full sample, tensor Clippy and cargo-deny. The isolated engine lift passed. [GGML scalar oracle](https://github.com/ggml-org/llama.cpp/blob/9a080f44b503c64f66c84a6274b9d9ee5d5b09d5/ggml/src/ggml-cpu/vec.h#L944)

### VTK-29 Gaussian activations

Verdant evaluates binary32 GELU using its certified tangent and GELU_QUICK using its certified logistic function. Each activation was compared on 128,774 distinct inputs with the pinned GGML scalar helpers, including every BF16 encoding widened to binary32, a grid across the active range and subnormal boundaries. Replacing the quick path with the tangent path failed input 0x36da0000. The restored test passed, as did tensor Clippy, cargo-deny and the isolated engine lift. The preservation attestation is `e97afee5223fe4ef32529f361c878991f05b9fff9b091f4653f9c7f210fc55c4`. [Certified result](generated/tensor-gelu/result.json) · [Independent scalar functions](https://github.com/ggml-org/llama.cpp/blob/9a080f44b503c64f66c84a6274b9d9ee5d5b09d5/ggml/src/ggml-cpu/vec.h)

### VTK-30 attention head slopes

The Verdant function takes a finite binary32 bias, a positive head count and an in-range head index. The pinned GGML CPU softmax supplied the per-head reference for 18 head counts from 1 to 1,537 and 12 positive biases. All 40,452 comparisons passed, with a maximum difference of one binary32 step and bit-exact zero and unit outputs. The test also covers nonpositive biases and typed failures for nonfinite biases, empty groups and invalid indices. Reversing head order returned 0.98922795 instead of 0.9945994 for head zero of two at bias 0.015625. The restored oracle, tensor Clippy and cargo-deny passed. The preservation attestation is `3a8e2cd53ae8c4072b044b5b4c3d9bb4249df1c6ef745215e893b6b76e27b530`. [Certified roots](generated/tensor-slopes/result.json) · [Imported GGML evaluator](https://github.com/ggml-org/llama.cpp/blob/9a080f44b503c64f66c84a6274b9d9ee5d5b09d5/ggml/src/ggml-cpu/ops.cpp#L5857)

### VTK-31 binary32 row ranking

`RowRankingF32` reuses its storage across rows. Prism defines admissible selection sizes and maps each binary32 encoding to its order key. Stable byte passes preserve input order for ties. The Engine places NaNs last in both directions.

The locked-SDK test compared 1,079 selected indices over 22 row selections with the pinned GGML CPU ARGSORT and TOP_K kernels. It compared selected values for ties and checked index order independently. Reversing direction failed the first unequal comparison with positive versus negative infinity. Restoring it passed. The test also checked row reuse, NaNs, empty selections and invalid sizes. The isolated lift covered 222 modules. The repository fidelity gate still reports the recorded Rust frontier counts of 1,177 and 1,335 against ceilings of 1,160 and 1,318 and a stale frontier source digest. [Certified model](app/src/RowRanking.lex.tex) · [Oracle and negative cases](crates/verdant-tensor/tests/models/matrix_product_tests.rs)

### VTK-32 normalized binary32 exponential rows

The certified model checks score words, selects the maximum and centers the input. The tensor primitive reuses the Verdant exponential, wide accumulator, division and binary32 multiplication. An optional extra score contributes to the denominator. The API takes binary32 slices and has no GGML dependency.

The locked SDK compared 4,164 output probabilities in 21 rows of widths 1 through 1,024 against the imported GGML CPU softmax. The inputs covered negative infinity, signed zero, repeated values and optional extra scores. NaN, positive infinity and an undefined nonempty mass were refused. Omitting the extra mass produced 1.0 for a probability for which the CPU returned 0.5621765. Restoring it passed. The model, targeted Clippy, dependency checks and isolated self-host lift passed. The full fidelity gate still reports the inherited frontier counts and stale Rust source digest.

### VTK-33 grouped affine row products

The packed four- and five-bit products take weight bytes and typed grouped signed-byte activation blocks. The Prism model selects scale and minimum fields, nibble and high-bit planes, and arithmetic instructions. Verdant reuses its existing activation encoder, integer dot product and binary32 arithmetic. The public API receives no graph, device or GGML tensor descriptors.

The pinned CPU produced identical binary32 words for 98 inputs of each format, including zero through five blocks, varied signed scales and unrelated high bits. Inconsistent byte and activation counts returned typed errors. Discarding the five-bit high plane changed row 1 from CPU word 1194307488 to 1200470161. Restoration passed both formats. The existing grouped two-bit regression, model, targeted Clippy, dependency checks and isolated self-host lift passed.

The full fidelity check reported these frontier findings.

- frontier.toml: hand-written Rust residue grew: 1177 > ceiling 1160
- frontier.toml: uncertified Rust surface grew: 1335 > ceiling 1318
- frontier.toml: frontier.toml is stale against live Rust source tree (recorded 5d011918bdb9d2a1 != live a6682851511226af; run `npm run ratchet`)

### VTK-34 binary32 softplus

The locked SDK checked 123,837 distinct input words against the pinned GGML CPU unary implementation. The corpus includes every widened F16 and BF16 word, stratified binary32 words, subnormals, infinities, NaNs and the positive cutoff neighborhood. Values above the cutoff, zeros and infinities matched exactly. The remaining finite results were within four binary32 steps, and NaNs remained NaNs. The primitive combines the existing certified exponential and logarithm with a compensated logarithm-of-one-plus calculation.

Discarding the exponential during compensation returned 0.19314718 for zero input, where the CPU returned 0.6931472. Restoration passed. The stacked grouped affine regression, model, targeted Clippy, dependency checks and isolated lift passed.

The full fidelity check reported these frontier findings.

- frontier.toml: hand-written Rust residue grew: 1177 > ceiling 1160
- frontier.toml: uncertified Rust surface grew: 1335 > ceiling 1318
- frontier.toml: frontier.toml is stale against live Rust source tree (recorded 5d011918bdb9d2a1 != live 2e6374e23a32ccd2; run `npm run ratchet`)

### VTK-35 sign and two-bit row products

The certified model defines the coefficient fields and number of signed-byte activation blocks per packed weight block. Verdant uses its existing integer dot, half scale conversion and binary32 accumulation. Each input is a packed weight row and a slice of prepared activation blocks.

The pinned CPU agreed bit for bit on 112 rows per format, including zero through six weight blocks, signed scales and all packed bit positions. Inconsistent byte and activation counts returned typed errors. Removing the coefficient offset changed sign-dot row 1 to 3276111872 where the CPU returned 1116323840. Restoring it passed both formats and the grouped affine regression. The model, targeted Clippy, dependency checks and isolated lift passed.

The full fidelity check reported these frontier findings.

- frontier.toml: hand-written Rust residue grew: 1177 > ceiling 1160
- frontier.toml: uncertified Rust surface grew: 1335 > ceiling 1318
- frontier.toml: frontier.toml is stale against live Rust source tree (recorded 5d011918bdb9d2a1 != live 99d5248e5e205b4e; run `npm run ratchet`)

### VTK-36 compact symmetric row products

The pinned GGML CPU dot functions supplied 224 binary32 results for Q5_0 and IQ4_NL coefficients and signed-byte activation blocks. All 224 words matched over zero through six blocks. Partial blocks and mismatched activation counts returned typed errors. Dropping the high 16 flag bits failed five-bit row 1 with observed word 1160574976 and reference word 1134018560. Restoration and the existing four-bit and signed-byte dot regressions passed.

The public entry point selects the coefficient encoding and reuses the scale and contraction logic. Its inputs are weight bytes and activation blocks. The isolated Verdant lift reported 232 modules with zero unlifted workspace or UI modules. The regenerated frontier and fidelity audit passed with 583 lifted functions, 1,152 residue functions and 1,310 uncertified functions. The existing ceilings were retained. [GGML CPU dot authority](https://github.com/ggml-org/llama.cpp/blob/9a080f44b503c64f66c84a6274b9d9ee5d5b09d5/ggml/src/ggml-cpu/quants.c)

### VTK-37 affine four- and five-bit row products

The pinned GGML reference encoder supplied 9,756 bytes from 271 Q8_1 blocks. The imported CPU vector-dot functions supplied 224 binary32 words from Q4_1 and Q5_1 weight rows of zero through six blocks. Every encoded byte and output word matched. Partial weight blocks, wrong lane counts and mismatched activation counts returned typed errors.

Multiplying the code sum by the stored binary16 step instead of the original binary32 step failed the Q8_1 byte comparison. Restoring the original step passed both VTK-37 tests and the existing symmetric encoding and compact product regressions. The engine API accepts numerical rows. GGML graph selection and tensor addressing remain with the consumer.

The complete locked-SDK `just vv` passed on `0c70cf1d`, including 85 engine conformance tests, 68 tensor tests, dependency checks and the Prism proof. The verified application attestation is `c042402611bc9473d0de0c22172e8c13c7251af510bf8e27c966358a20c9074e`.

### VTK-38 block-float row products

The `linear::dot_block_float_row` API contracts E2M1 coefficients with signed-byte activation blocks. It accepts either one E8M0 scale per 32 coefficients or four unsigned E4M3 scales per 64 coefficients. Prism checks the block counts, selects the coefficient table entries and defines the binary32 accumulation. The imported GGML CPU vector products matched 512 rows per encoding across zero through six blocks and all scale bytes. Reusing the low nibble for both lane halves changed MXFP4 row 1 from `2234220544` to `2225029120`. Restoring separate indices passed all five packed-product regression tests. [CPU authority](https://github.com/ggml-org/llama.cpp/blob/9a080f44b503c64f66c84a6274b9d9ee5d5b09d5/ggml/src/ggml-cpu/quants.c#L303)

Prism certified the export with attestation `3c6431afca879dca5ec9e996076a9d757ad2628a697c834f634eca23c4c29771`. The isolated source lift, fidelity check and tensor Clippy passed. The frontier records 583 lifted and 1,152 residual functions. Core and Tensor share the bounded power-of-two implementation, including zero for exponents at or above the word width.

## VTK-39 grouped two-bit codebook rows

The pinned llama.cpp CPU type traits supply the independent IQ2_XXS, IQ2_XS and IQ2_S vector products. Each reference row uses the upstream Q8_K reference encoder on the original activations. All 294 binary32 results matched bit-for-bit across zero to five blocks, and each encoding rejected both malformed lengths. Reusing the first grid coefficient for every lane failed IQ2_XXS row 1 (1170642016 instead of 1170701216). Restoring lane selection passed VTK-39 and the existing Q4_K/Q5_K regression.

The certified grouped product extension uses the existing imported grid and sign tables. The SDK attestation is `9d86a2a8a6e022e89c94da60784d6a83d8ea6aba6153edf2c7144f20c67da2c5`. The isolated source lift, fidelity checks and tensor Clippy passed at the existing frontier limits.

## VTK-40 fixed-width weight block encoding

All 1,104 centered and affine four- and five-bit blocks matched every byte emitted by the four unmodified reference encoders from the pinned GGML archive. The 276 inputs per encoding include signed zero, tied positive and negative extrema in both orders, constant rows, narrow and overflowing half scales and mixed-exponent random values. Nonfinite lanes and malformed destination widths preserve the output. The symmetric byte path delegates to the existing VTK-16 encoder.

Replacing every upper nibble’s source with the lower lane failed Q4_0 block 3 at byte 2 (153 versus 137). Restoring the lane passed VTK-40, VTK-16 and VTK-37 together. Run `cargo test --locked -p verdant-tensor --lib low_bit_block_encoding_vtk_40` inside the pinned SDK.
