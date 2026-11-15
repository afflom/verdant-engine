# Verdant Engine Verification & Engineering Report

This report documents the verification boundary, standards validation oracles, performance budgets, causal expansion and structural statics theorems, and zero-constants enforcement governing **Verdant Engine** (`verdant-core` and its browser Engine Studio).

---

## 1. Engine Conformance Matrix (`VE-01` – `VE-10`)

Every capability of `crates/verdant-core` is registered in `model/ids.toml`, specified as an executable Gherkin scenario in `features/suites/engine.feature`, and verified with both positive assertions and typed negative-refusal tests in `crates/conformance/tests/engine.rs`:

| ID | Subsystem | Positive Verification | Typed Refusal & Negative Test |
| --- | --- | --- | --- |
| `VE-01` | ECS & Spatial BVH | Generational `EntityId` lifecycle, parent-child `Mat4` world transforms, 6-plane frustum culling, and Möller-Trumbore raycasting | `EntityError::StaleEntityGeneration` (`engine_ve_01`) |
| `VE-02` | Causal Expansion & Builder Statics | Monotonic `t_0 -> t_obs` scale-factor progression (`0.05 -> 0.28 -> 0.64 -> 1.0`) and Heyman middle-third statics (`e/t <= 1/6`, `SF >= 2.0`) across all 7 sanctuary structures and perimeter rover | `GenesisError::NonMonotonicScaleFactor`, `GenesisError::MiddleThirdViolated` (`engine_ve_02`) |
| `VE-03` | Procedural Geometry & L-Systems | Deterministic FBM amphitheater terrain, UV spheres, columns, boxes, phyllotactic vascular flora, and morph-target deltas | `GeometryError::InsufficientGridResolution`, `GeometryError::MorphTargetVertexCountMismatch` (`engine_ve_03`) |
| `VE-04` | Binary Khronos glTF 2.0 Export | Self-contained `.glb` generation parsed and validated by the official `gltf` 1.4 crate (JSON chunk, BIN padding, accessors, bounds, PBR materials) | `GltfExportError::EmptySceneExport`, `GltfExportError::IndexOutOfBounds` (`engine_ve_04`) |
| `VE-05` | Microfacet PBR & W3C WGSL Shaders | Energy-conserving Cook-Torrance GGX BRDF (`0 <= R <= 1`) and `naga` 29 front-end + module validation for PBR surface and water ripple shaders | `ShaderValidationError::WgslParseError`, `ShaderValidationError::MissingEntryPoint` (`engine_ve_05`) |
| `VE-06` | Rigid-Body, Capsule & Spline Rover Physics | Fixed-step symplectic Euler integration, impulse collision restitution, player capsule ground clamping, and closed-loop rover patrol | `PhysicsStepError::InvalidBodyMassOrRadius`, `PhysicsStepError::InvalidTimeStepSeconds` (`engine_ve_06`) |
| `VE-07` | Spatial Audio & Modal Synthesis | Woodworth spherical-head binaural ITD/ILD cues, Sabine & Eyring `RT60` reverberation, and Euler-Bernoulli chime eigenfrequencies | Positive acoustic separation and monotonic modal progression (`engine_ve_07`) |
| `VE-08` | Multi-Worker Job Graph & Frame KPI | Kahn topological critical-path scheduling across worker threads and 60Hz (`16,666 us`) / 120Hz (`8,333 us`) frame-budget profiling | Cyclic dependency rejection, `EngineConfigError::ZeroViewportDimension`, `EngineConfigError::ZeroWorkerThreads` (`engine_ve_08`) |
| `VE-09` | Bit-Exact `VDNT` World Snapshots | Deterministic little-endian IEEE-754 snapshot serialization with SHA-256 integrity verification | `SnapshotCodecError::TruncatedSnapshot`, `SnapshotCodecError::IntegrityDigestMismatch` (`engine_ve_09`) |
| `VE-10` | Formal Core-Wasm Engine Verifier | Constructive Lean 4 proofs (`epochsStrictlyMonotonic`, `allSevenBlueprintsEquilibrium`, `unstableAndPreBuilderRefused`), zero `Classical.choice` axioms, and sealed byte-request acceptance vectors | Out-of-epoch (`refused:epoch-precedes-builders`), unstable (`refused:heyman-middle-third-exceeded`), and unknown (`invalid request`) refusals (`engine_ve_10`) |

---

## 2. Permanent Zero-Constants Parametric Ratchet (`FI-08`)

`model/parametric-ratchet.toml` permanently locks every literal ceiling to `0`:

- `max_ontology_magic_numbers = 0`
- `max_constructive_lex_magic_numbers = 0`
- `max_total_lex_magic_numbers = 0`
- `max_rust_unexplained_floats = 0`
- `max_rust_unexplained_ints = 0`
- `max_rust_files_with_magic = 0`

On every `just fidelity` and `just vv` run, `crates/fidelity/src/ratchet.rs` scans all 23 backend `.lex.tex` modules and all 11 Rust modules in `crates/verdant-core/src/`. Any unexplained numeric literal outside named `const` declarations fails the build immediately, and `zero_constants_parametric_ratchet_fi_08` verifies that planted magic float and integer literals in temporary workspaces are deterministically caught and refused.

---

## 3. Browser Frame, Responsive Viewport & Wire-Weight Gates (`FR-01` – `FR-03`)

The interactive 3D WebGL2 Engine Studio (`site/card.html`, `site/card.css`, `site/card.mjs`) and its Diátaxis documentation (`site/docs/`, `site/llms.txt`) are gated by four automated checks:

1. **Brand & Frame Integrity (`FR-01`, `FR-02`)**: `bootstrap/brand-sync.mjs --check`, `bootstrap/brand-fonts.py --check`, and `bootstrap/frame-sync.mjs --check` verify byte-for-byte lock alignment against `brand.lock` and `frame.lock`, zero inline styles, and zero stylesheet color literals.
2. **Material 3 Window-Class Matrix**: `bootstrap/screens.mjs` renders all 6 window classes (`compact 360x780`, `compact-height 720x400`, `medium`, `expanded`, `large`, `extra-large`) across dark, light, and 200%-zoom configurations, enforcing zero viewport scroll overflow and 48px touch targets.
3. **First-Visit Wire Budget (`FR-03`)**: `bootstrap/weight.mjs` verifies that a cold mobile visit stays strictly within the 100 KiB frame budget, 120 KiB application budget (including `app.holo`, 3D studio code, and JSON metadata), and 28-request ceiling.
4. **Diátaxis & ISO 24495-1 Content Gate**: `scripts/content/check.mjs` audits every generated documentation page (`CS-01`–`CS-11`) for structural navigation, copyable code examples, plain-language readability, and valid internal cross-references.

---

## 4. Reproducing the Complete Verification Gate

Inside the digest-pinned SDK container (`prismpm.lock`):

```sh
just vv
```

To run the standalone starter template and engine examples:

```sh
cargo run -p verdant-core --example cosmic_sanctuary
cargo run -p verdant-core --example custom_game_arena
cargo run -p verdant-core --example headless_benchmark
cargo run --manifest-path templates/browser-3d-starter/Cargo.toml
```
