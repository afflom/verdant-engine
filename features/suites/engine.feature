Feature: engine

  Verdant Engine's standalone 3D simulation, rendering, asset, physics, spatial
  audio, runtime scheduling, binary world-snapshot, and Core-Wasm verification
  capabilities, including positive validation against authoritative standards
  oracles and typed refusals for every fallible boundary.

  @VE-01 @build
  Scenario: World expansion advances monotonically across cosmological epochs to t_obs and refuses out-of-order or out-of-bounds scale factors
    Given a cosmogenic world seeded deterministically
    When cosmic expansion advances across LithicHydrologic, BioticEcological, and AgenticArchitectural epochs to scale factor 1.0
    Then terrain, water, flora, fauna, and Builder guilds emerge monotonically in causal order
    And stepping backward in cosmic scale or supplying an out-of-bounds scale factor returns WorldEpochError

  @VE-02 @build
  Scenario: All seven Builder artifacts and the autonomous perimeter rover satisfy Heyman middle-third statics at t_obs while unstable or pre-Builder blueprints are refused
    Given a world evolved to t_obs with five differentiated Builder guilds
    When all seven constructed sanctuary structures and the autonomous perimeter sentinel rover are inspected
    Then every blueprint satisfies Heyman middle-third thrust eccentricity, at least 2.0 safety factor, and positive engineering utility
    And an unstable span whose thrust exits the middle third returns StructuralEquilibriumError

  @VE-03 @build
  Scenario: Archetype ECS world and surface-area-heuristic BVH execute deterministic raycasts and frustum culling across arbitrary 3D scenes
    Given a 3D scene populated with hierarchical entities and bounding volumes
    When a surface-area-heuristic BVH is constructed over the world
    Then raycasts and perspective frustum queries return deterministic hits and visible entity sets

  @VE-04 @build
  Scenario: Procedural meshes, morph targets, and PBR scenes export to binary glTF 2.0 validated by the Khronos gltf oracle and refuse degenerate geometry
    Given procedural sanctuary meshes, morph targets, and metallic-roughness PBR materials
    When the scene is exported to binary glTF 2.0
    Then the exported GLB passes authoritative Khronos gltf validation with exact mesh and material counts
    And exporting an empty or out-of-bounds mesh returns AssetValidationError

  @VE-05 @build
  Scenario: PBR, terrain, water, and sky WGSL shader modules pass W3C naga validation and invalid shader sources are refused
    Given Verdant Engine's built-in PBR surface, water lagoon, and sky scattering WGSL shaders
    When each shader module is checked by naga
    Then all built-in shaders pass W3C WGSL parsing, type validation, and entry-point verification
    And malformed WGSL or a missing entry point returns ShaderValidationError

  @VE-06 @build
  Scenario: Symplectic Euler rigid bodies, capsule controllers, and autonomous spline rovers step deterministically and refuse invalid substeps or non-finite timesteps
    Given a physics world with buoyant rigid bodies, a first-person capsule controller, and the perimeter sentinel rover
    When the simulation steps across fixed frames
    Then rigid bodies, player kinematics, and rover spline traversal advance deterministically
    And non-positive mass or non-finite timesteps return PhysicsStepError

  @VE-07 @build
  Scenario: Spatial audio listener and modal resonators synthesize deterministic stereo buffers with distance attenuation and Sabine RT60 reverberation
    Given travertine rotunda and open meadow acoustic environments with Euler-Bernoulli modal chimes
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
    Given a world state evolved to present observation t_obs
    When encoded to a VDNT binary snapshot and decoded back
    Then every scalar, blueprint, and entity transform round-trips bit-identically
    And truncated or byte-tampered snapshots return SnapshotCodecError

  @VE-10 @build
  Scenario: The compiled Core-Wasm engine verifier proves monotonic expansion and Builder equilibrium in Lean without Classical.choice and answers queries deterministically
    Given app/src/Genesis.lex.tex and app/src/Engine.lex.tex
    When checked against site/app.json and LexLean axioms
    Then monotonic expansion, seven-blueprint equilibrium, and unstable refusal theorems hold with zero Classical.choice axioms
    And arithmetic overflow in the compiled kernel returns ComputeError
