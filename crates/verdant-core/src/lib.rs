//! **Verdant Engine (`verdant-core`)** — Universal, Prism-mechanized 3D game engine kernel.
//!
//! `verdant-core` provides a deterministic, purely parametric 3D simulation, tensor-compute,
//! rendering, progressive `UORC` streaming, physics, and spatial-audio kernel engineered for
//! browser/WebAssembly runtimes and native Rust applications without relying on conditional
//! browser GPU APIs:
//!
//! - [`math`]: Right-handed metric linear algebra (`Vec2`, `Vec3`, `Quat`, `Mat4`, `Aabb`,
//!   `Ray`, `Frustum`), constructive number generators from `{0, 1}`, and deterministic PRNG.
//! - [`ecs`]: Hierarchical scene graph (`SceneWorld`, `EntityId`, `Transform`) and median-split
//!   `BoundingVolumeHierarchy` (`BVH`) for frustum culling and raycast scene picking.
//! - [`streaming`]: Progressive multi-LOD scene synthesis (`LodLevel`, `StandardSceneState`,
//!   `PhysicalEnvironment`, `TerrainReliefParams`, `SceneNodeKind`) and authoritative `UORC`
//!   (`UORC-SHA256-CHAIN-v1`) multi-object streaming up to lossless source resolution.
//! - [`tensor`]: Portable Prism + `uor-matmul` tensor kernels (`batch_transform_positions_exact`,
//!   `tropical_frustum_distance_bounds`, `mix_spatial_audio_channels_exact`, and tile-aligned
//!   `render_triangles_tile_framebuffer`).
//! - [`geometry`]: Parametric procedural B-Mesh synthesis (`ProceduralMesh`, `MorphTarget`) for
//!   heightfield terrains, fluted architectural columns, deformable ribbons, and faceted prisms.
//! - [`render`]: Metallic-roughness + thin-film `PbrMaterial` definitions and `WGSL` shaders
//!   verified by the authoritative `naga` compiler/validator.
//! - [`gltf_io`]: Khronos glTF 2.0 binary (`.glb`) scene export and specification validation via
//!   the authoritative `gltf` crate.
//! - [`physics`]: Symplectic Euler rigid-body dynamics, Archimedes submerged fluid buoyancy,
//!   first-person `CapsuleController`, harmonic spring joints, and `KinematicSplineBody`.
//! - [`audio`]: Woodworth spherical-head binaural spatial cues (`ITD`, `ILD`), analytical
//!   ISO 3382-1 Sabine/Eyring enclosure reverberation (`RT60`), and Euler-Bernoulli modal bars.
//! - [`runtime`]: Host capability envelope, multi-worker DAG `WorkerJobGraph`, refresh-derived
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
pub mod camera;
pub mod character;
pub mod collision;
pub mod cybernetics;
pub mod ecs;
pub mod error;
pub mod geometry;
pub mod gltf_io;
pub mod governance;
pub mod hd_world;
pub mod math;
pub mod narrative;
pub mod navmesh;
pub mod network;
pub mod observer;
pub mod operator;
pub mod physics;
pub mod ratchet_sync;
pub use verdant_tensor;
#[doc(hidden)]
pub use verdant_tensor::prism_kernel;
pub mod render;
pub mod runtime;
pub mod standards;
pub mod streaming;
pub mod tensor;
pub mod uhd;
pub mod ui;
pub mod universe;
pub mod vcs;

#[cfg(test)]
mod tests {
    use crate::audio::{
        evaluate_binaural_cue, synthesize_modal_bar_modes, AcousticEnvironment, ListenerPose,
        SpatialAudioSource,
    };
    use crate::geometry::canonical_scene_mesh_library;
    use crate::gltf_io::{export_scene_to_glb, validate_glb_bytes};
    use crate::math::{fibonacci_u64, nth_prime_u64, pow2_u64, Frustum, Mat4, Ray, Vec3};
    use crate::physics::{
        CapsuleController, KinematicSplineBody, PhysicsWorld, PhysicsWorldConfig, RigidBody,
    };
    use crate::render::{canonical_pbr_material_library, validate_builtin_engine_shaders};
    use crate::runtime::{EngineConfig, FrameBudgetProfiler, WorkerJobGraph, WorldSnapshot};
    use crate::streaming::{
        decode_uorc_stream_up_to_lod, pack_progressive_uorc_stream, verify_progressive_uorc_stream,
        LodLevel, SceneNodeKind, StandardSceneState,
    };
    use crate::tensor::{
        batch_transform_positions_exact, mix_spatial_audio_channels_exact,
        render_triangles_tile_framebuffer, tropical_frustum_distance_bounds,
        verified_engine_tensor_kernels, RasterTriangle,
    };

    #[test]
    fn constructive_generators_and_verified_tensor_kernels_match_spec() {
        assert_eq!(pow2_u64(0), 1);
        assert_eq!(pow2_u64(10), 1024);
        assert_eq!(fibonacci_u64(0), 0);
        assert_eq!(fibonacci_u64(8), 21);
        assert_eq!(nth_prime_u64(0), 2);
        assert_eq!(nth_prime_u64(1), 3);
        assert_eq!(nth_prime_u64(2), 5);

        let kernels = verified_engine_tensor_kernels().expect("7 certified tensor kernels");
        assert_eq!(kernels.len(), 7);
    }

    #[test]
    fn progressive_lod_streaming_tensor_kernels_and_glb_roundtrip() {
        let mut world = StandardSceneState::new_at_lod0(7).expect("lod0 seed");
        assert_eq!(world.active_lod, LodLevel::Lod0CoarseProxy);
        assert!(world.scene_nodes.is_empty());

        world.advance_to_lod(1).expect("lod1");
        assert_eq!(world.active_lod, LodLevel::Lod1MediumDetail);
        assert_eq!(world.scene_nodes.len(), 3);

        world.advance_to_lod(2).expect("lod2");
        assert!(world.foliage_count > 0);
        assert!(world.scene_nodes.len() > 3);

        world.advance_to_lod(3).expect("lod3 lossless");
        assert_eq!(world.active_lod, LodLevel::Lod3LosslessSource);
        assert_eq!(world.scene_nodes.len(), 7);
        assert!(world
            .scene_nodes
            .iter()
            .any(|n| n.kind == SceneNodeKind::KinematicSplineMover));
        assert!(world.advance_to_lod(1).is_err());

        // Progressive UORC multi-LOD stream encode + cryptographic chain verify + decode:
        let stream_bytes = pack_progressive_uorc_stream(7).expect("uorc encode");
        assert!(!stream_bytes.is_empty());
        let uorc_summary =
            verify_progressive_uorc_stream(&stream_bytes).expect("uorc chain verify");
        assert_eq!(uorc_summary.lod_object_count, 4);
        assert_eq!(uorc_summary.max_lod_rank, 3);
        let decoded_lod3 =
            decode_uorc_stream_up_to_lod(&stream_bytes, 3).expect("uorc progressive decode");
        assert_eq!(decoded_lod3, world);

        // Exact uor-matmul tensor kernels + tile rasterizer:
        let positions = [[1.0_f32, 2.0, 3.0], [-2.0, 0.5, 4.0]];
        let mat = Mat4::from_translation(Vec3::new(5.0, -1.0, 2.0));
        let transformed =
            batch_transform_positions_exact(&positions, &mat).expect("exact float gemm");
        assert_eq!(transformed[0], [6.0, 1.0, 5.0, 1.0]);

        let bounds =
            tropical_frustum_distance_bounds(&[[10, 20, 30], [5, 15, 25]], &[[1, 2, 3], [2, 1, 0]])
                .expect("tropical max-plus");
        assert_eq!(bounds.len(), 4);

        let mixed = mix_spatial_audio_channels_exact(2, 2, 2, &[100, -50, 25, 10], &[2, 1, 1, 2])
            .expect("exact integer audio mix");
        assert_eq!(mixed, vec![150, 0, 60, 45]);

        let vp = Mat4::perspective(1.0, 1.0, 0.1, 100.0)
            * Mat4::look_at(Vec3::new(0.0, 1.5, 6.0), Vec3::ZERO, Vec3::UP);
        let tri = RasterTriangle {
            positions: [[-1.0, -1.0, 0.0], [1.0, -1.0, 0.0], [0.0, 1.2, 0.0]],
            normal: [0.0, 0.0, 1.0],
            base_color: [0.35, 0.72, 0.88],
        };
        let fb =
            render_triangles_tile_framebuffer(64, 64, &vp, Vec3::new(0.4, 0.8, 0.5), 3, &[tri])
                .expect("tile framebuffer");
        assert!(fb.rasterized_triangles > 0);
        assert_eq!(fb.rgba8.len(), 64 * 64 * 4);

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

        // Physics + kinematic mover + buoyancy + audio + snapshot:
        let mut phys = PhysicsWorld::new(PhysicsWorldConfig::default());
        phys.bodies.push(
            RigidBody::new("FloatingBuoy", Vec3::new(6.0, -0.4, -5.0), 12.0, 0.5, 0.3)
                .expect("body"),
        );
        phys.player_capsule = Some(CapsuleController::new(Vec3::new(0.0, 3.0, 14.0)));
        phys.kinematic_mover = Some(KinematicSplineBody::new(
            world.mover_spline_waypoints.clone(),
            2.5,
        ));
        phys.step(1.0 / 60.0, Vec3::new(0.0, 0.0, -1.0))
            .expect("step");
        assert!(
            phys.kinematic_mover
                .as_ref()
                .expect("mover")
                .distance_traveled_meters
                > 0.0
        );

        let ac_env =
            AcousticEnvironment::reflective_chamber(world.params.environment.sound_speed_mps);
        assert!(
            ac_env.sabine_rt60_seconds()
                > AcousticEnvironment::damped_exterior(world.params.environment.sound_speed_mps)
                    .sabine_rt60_seconds()
        );
        let cue = evaluate_binaural_cue(
            &SpatialAudioSource {
                label: "ModalBarEmitter".to_string(),
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
        assert_eq!(synthesize_modal_bar_modes(110.0, 4).len(), 4);

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
