//! **Verdant Engine (`verdant-core`)** — Multi-purpose 3D browser and WebAssembly game engine.
//!
//! `verdant-core` provides a deterministic, purely parametric 3D simulation, rendering, asset,
//! physics, and spatial-audio kernel engineered for WebGPU/WebGL2 browser hosts and native Rust
//! applications:
//!
//! - [`math`]: Right-handed metric linear algebra (`Vec2`, `Vec3`, `Quat`, `Mat4`, `Aabb`,
//!   `Ray`, `Frustum`), constructive number generators from `{0, 1}`, and deterministic PRNG.
//! - [`ecs`]: Hierarchical scene graph (`SceneWorld`, `EntityId`, `Transform`) and median-split
//!   `BoundingVolumeHierarchy` (`BVH`) for frustum culling and raycast scene picking.
//! - [`genesis`]: Causal cosmological-to-architectural world synthesis (`CosmicEpoch`,
//!   `PhysicalEnvironment`, `WorldGenesisParams`, `BuilderDiscipline`, `ArchitecturalBlueprint`)
//!   with Heyman middle-third masonry equilibrium (`e/t <= 1/6`) and compressive safety checks.
//! - [`geometry`]: Parametric procedural B-Mesh synthesis (`ProceduralMesh`, `MorphTarget`) for
//!   heightfield terrains, Vitruvian fluted entasis columns, botanical petals, and quartz crystals.
//! - [`render`]: Metallic-roughness + thin-film `PbrMaterial` definitions and W3C WebGPU (`WGSL`)
//!   shaders verified by the authoritative `naga` compiler/validator.
//! - [`gltf_io`]: Khronos glTF 2.0 binary (`.glb`) scene export and specification validation via
//!   the authoritative `gltf` crate.
//! - [`physics`]: Symplectic Euler rigid-body dynamics, Archimedes submerged fluid buoyancy,
//!   first-person `CapsuleController`, harmonic spring joints, and `KinematicSplineRover`.
//! - [`audio`]: Woodworth spherical-head binaural spatial cues (`ITD`, `ILD`), analytical
//!   ISO 3382-1 Sabine/Eyring enclosure reverberation (`RT60`), and Euler-Bernoulli modal chimes.
//! - [`runtime`]: Browser capability envelope, multi-worker DAG `WorkerJobGraph`, refresh-derived
//!   `FrameBudgetProfiler`, and SHA-256 integrity-checked `WorldSnapshot` serialization.
//! - [`animation`]: Skeletal joint hierarchies, normalized four-joint linear-blend skinning, and
//!   keyframe/morph interpolation backed by `prism_kernel`.
//! - [`collision`]: Bounded 3D Gilbert-Johnson-Keerthi (`GJK`) convex collision manifolds and
//!   Baumgarte + Coulomb contact impulse resolution backed by `prism_kernel`.
//! - [`navmesh`]: Slope-filtered walkable `NavMesh` extraction and admissible `A*` corridor
//!   pathfinding backed by `prism_kernel`.
//! - [`error`]: Domain-typed refusal classifications for every fallible operation.

#![forbid(unsafe_code)]

pub mod animation;
pub mod audio;
pub mod collision;
pub mod ecs;
pub mod error;
pub mod genesis;
pub mod geometry;
pub mod gltf_io;
pub mod math;
pub mod navmesh;
pub mod physics;
#[doc(hidden)]
#[allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    unused_parens,
    unused_variables,
    clippy::all
)]
pub mod prism_kernel;
pub mod render;
pub mod runtime;

#[cfg(test)]
mod tests {
    use crate::audio::{
        evaluate_binaural_cue, synthesize_euler_bernoulli_chime_modes, AcousticEnvironment,
        ListenerPose, SpatialAudioSource,
    };
    use crate::genesis::{
        heyman_middle_third_limit, verify_blueprint_equilibrium, CausalWorldState, CosmicEpoch,
        StructureKind,
    };
    use crate::geometry::canonical_scene_mesh_library;
    use crate::gltf_io::{export_scene_to_glb, validate_glb_bytes};
    use crate::math::{fibonacci_u64, nth_prime_u64, pow2_u64, Frustum, Ray, Vec3};
    use crate::physics::{
        CapsuleController, KinematicSplineRover, PhysicsWorld, PhysicsWorldConfig, RigidBody,
    };
    use crate::render::{canonical_pbr_material_library, validate_builtin_engine_shaders};
    use crate::runtime::{EngineConfig, FrameBudgetProfiler, WorkerJobGraph, WorldSnapshot};

    #[test]
    fn constructive_generators_match_number_theory() {
        assert_eq!(pow2_u64(0), 1);
        assert_eq!(pow2_u64(10), 1024);
        assert_eq!(fibonacci_u64(0), 0);
        assert_eq!(fibonacci_u64(8), 21);
        assert_eq!(nth_prime_u64(0), 2);
        assert_eq!(nth_prime_u64(1), 3);
        assert_eq!(nth_prime_u64(2), 5);
        assert!((heyman_middle_third_limit() - 1.0 / 6.0).abs() < 1e-12);
    }

    #[test]
    fn causal_world_evolves_monotonically_to_builders_and_glb() {
        let mut world = CausalWorldState::new_cosmogenic(7).expect("cosmogenic seed");
        assert_eq!(world.active_epoch, CosmicEpoch::Cosmogenic);
        assert!(world.blueprints.is_empty());

        world.advance_to(0.45).expect("lithic epoch");
        assert_eq!(world.active_epoch, CosmicEpoch::LithicHydrologic);

        world.advance_to(0.70).expect("biotic epoch");
        assert_eq!(world.active_epoch, CosmicEpoch::BioticEcological);
        assert!(world.flora_count > 0);

        world.advance_to(1.0).expect("observation instant t_obs");
        assert_eq!(world.active_epoch, CosmicEpoch::AgenticArchitectural);
        assert_eq!(world.builder_guilds.len(), 5);
        assert_eq!(world.blueprints.len(), 7);
        assert!(world
            .blueprints
            .iter()
            .any(|b| b.kind == StructureKind::PerimeterSentinelRover));
        for bp in &world.blueprints {
            assert!(bp.thrust_eccentricity_ratio <= heyman_middle_third_limit());
            assert!(bp.safety_factor >= 2.0);
            assert!(bp.net_engineering_utility > 0.0);
        }

        // Verify equilibrium refusal on overly thin span:
        assert!(verify_blueprint_equilibrium(40.0, 2.0, 0.2, 50.0, 2500.0, 9.8, 0.5).is_err());

        // Validate WGSL shaders via Naga and GLB export via gltf:
        let shaders = validate_builtin_engine_shaders().expect("builtin WGSL valid");
        assert_eq!(shaders.len(), 4);

        let meshes = canonical_scene_mesh_library(world.seed);
        let materials = canonical_pbr_material_library();
        let glb = export_scene_to_glb(&world.scene, &meshes, &materials).expect("export glb");
        let summary = validate_glb_bytes(&glb).expect("gltf oracle");
        assert_eq!(summary.mesh_count, meshes.len());
        assert_eq!(summary.material_count, materials.len());

        // BVH raycast & frustum culling:
        let bvh = world.scene.build_bvh();
        let hit = bvh.raycast(
            Ray::new(Vec3::new(0.0, 20.0, 0.0), Vec3::new(0.0, -1.0, 0.0)),
            100.0,
        );
        assert!(hit.is_some());
        let frustum = Frustum::from_perspective(
            Vec3::new(0.0, 6.0, 24.0),
            Vec3::new(0.0, -0.15, -1.0),
            Vec3::UP,
            1.0,
            16.0 / 9.0,
            0.1,
            200.0,
        );
        assert!(!bvh.query_frustum(&frustum).is_empty());

        // Physics + perimeter rover + buoyancy + audio + snapshot:
        let mut phys = PhysicsWorld::new(PhysicsWorldConfig::default());
        phys.bodies.push(
            RigidBody::new("FloatingBuoy", Vec3::new(6.0, -0.4, -5.0), 12.0, 0.5, 0.3)
                .expect("body"),
        );
        phys.player_capsule = Some(CapsuleController::new(Vec3::new(0.0, 3.0, 14.0)));
        phys.sentinel_rover = Some(KinematicSplineRover::new(
            world.rover_perimeter_waypoints.clone(),
            2.5,
        ));
        phys.step(1.0 / 60.0, Vec3::new(0.0, 0.0, -1.0))
            .expect("step");
        assert!(
            phys.sentinel_rover
                .as_ref()
                .expect("rover")
                .distance_traveled_meters
                > 0.0
        );

        let ac_env =
            AcousticEnvironment::travertine_rotunda(world.params.environment.sound_speed_mps);
        assert!(
            ac_env.sabine_rt60_seconds()
                > AcousticEnvironment::open_meadow(world.params.environment.sound_speed_mps)
                    .sabine_rt60_seconds()
        );
        let cue = evaluate_binaural_cue(
            &SpatialAudioSource {
                label: "ChimePergola".to_string(),
                position: Vec3::new(5.0, 2.0, 0.0),
                fundamental_hz: 440.0,
                reference_gain: 1.0,
            },
            &ListenerPose {
                position: Vec3::ZERO,
                forward: Vec3::new(0.0, 0.0, -1.0),
                up: Vec3::UP,
            },
            &ac_env,
        );
        assert!(cue.itd_seconds > 0.0);
        assert_eq!(synthesize_euler_bernoulli_chime_modes(110.0, 4).len(), 4);

        let cfg = EngineConfig::from_refresh_rate(1920, 1080, 60, 60, 4).expect("cfg");
        let graph = WorkerJobGraph::canonical_frame_pipeline(900, 1800, 1100, 700, 600, 1200);
        let (_, makespan) = graph.schedule(cfg.worker_threads).expect("dag schedule");
        let mut profiler = FrameBudgetProfiler::new(&cfg);
        profiler.record_frame(makespan);
        assert!(profiler.meets_budget_kpi());

        let snap = WorldSnapshot::encode(&world).expect("encode");
        let decoded = WorldSnapshot::decode(&snap).expect("decode");
        assert_eq!(decoded, world);
    }
}
