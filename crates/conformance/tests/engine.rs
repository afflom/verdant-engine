//! Conformance suite for `verdant-core`, `verdant-wasm`, `verdant-cli`, and the Verdant Engine verifier (`VE-01`..`VE-16`).

use std::path::PathBuf;

use verdant_core::animation::{
    skin_mesh_linear_blend, Joint, KeyframePose, SkeletalClip, Skeleton, SkinWeights4,
};
use verdant_core::audio::{
    evaluate_binaural_cue, synthesize_modal_bar_modes, AcousticEnvironment, ListenerPose,
    SpatialAudioSource,
};
use verdant_core::collision::{
    gjk_intersect_and_manifold, solve_contact_impulse, ConvexHullCollider,
};
use verdant_core::ecs::{EntityRole, SceneWorld, Transform};
use verdant_core::error::{
    AnimationError, AssetValidationError, CollisionError, EngineConfigError, NavigationError,
    PhysicsStepError, RenderGraphError, ShaderValidationError, SnapshotCodecError, StreamingError,
    TensorComputeError,
};
use verdant_core::geometry::{canonical_scene_mesh_library, MorphTarget, ProceduralMesh};
use verdant_core::gltf_io::{export_scene_to_glb, validate_glb_bytes};
use verdant_core::math::{Aabb, Frustum, Mat4, Quat, Ray, Vec3};
use verdant_core::navmesh::{steer_along_corridor, NavMesh};
use verdant_core::physics::{
    CapsuleController, KinematicSplineBody, PhysicsWorld, PhysicsWorldConfig, RigidBody,
};
use verdant_core::render::{
    canonical_pbr_material_library, compute_csm_splits, validate_builtin_engine_shaders,
    validate_wgsl_shader, IblEnvironmentProbe, RenderFrameGraph, WGSL_PBR_SURFACE_SHADER,
};
use verdant_core::runtime::{EngineConfig, FrameBudgetProfiler, WorkerJobGraph, WorldSnapshot};
use verdant_core::streaming::{
    decode_uorc_stream_up_to_lod, pack_progressive_uorc_stream, verify_progressive_uorc_stream,
    LodLevel, SceneNodeKind, StandardSceneState,
};
use verdant_core::tensor::{
    batch_transform_positions_exact, mix_spatial_audio_channels_exact,
    render_triangles_tile_framebuffer, tropical_frustum_distance_bounds,
    verified_engine_tensor_kernels, RasterTriangle,
};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("crates/conformance is two below root")
        .to_path_buf()
}

/// `VE-01`: Monotonic multi-LOD `UORC` (`UORC-SHA256-CHAIN-v1`) streaming and `StreamingError` refusals.
#[test]
fn engine_ve_01() {
    let mut world = StandardSceneState::new_at_lod0(11).expect("valid LOD0 seed");
    assert_eq!(world.active_lod, LodLevel::Lod0CoarseProxy);
    assert!(world.scene_nodes.is_empty());

    world.advance_to_lod(1).expect("advance to LOD1");
    assert_eq!(world.active_lod, LodLevel::Lod1MediumDetail);
    assert_eq!(world.scene_nodes.len(), 3);

    world.advance_to_lod(2).expect("advance to LOD2");
    assert_eq!(world.active_lod, LodLevel::Lod2HighDetail);
    assert!(world.foliage_count > 0);

    // Negative refusal 1: stepping backward in progressive LOD resolution is refused:
    let backward = world
        .advance_to_lod(1)
        .expect_err("backward LOD transition must fail");
    assert!(matches!(
        backward,
        StreamingError::NonMonotonicLodTransition { .. }
    ));

    // Negative refusal 2: out-of-bounds LOD rank (`> 3`) is refused:
    let invalid_rank = world.advance_to_lod(5).expect_err("LOD rank > 3 must fail");
    assert!(matches!(
        invalid_rank,
        StreamingError::InvalidLodRank { .. }
    ));

    world
        .advance_to_lod(3)
        .expect("advance to lossless LOD3 source");
    assert_eq!(world.active_lod, LodLevel::Lod3LosslessSource);
    assert_eq!(world.scene_nodes.len(), 7);
    assert!(world
        .scene_nodes
        .iter()
        .any(|n| n.kind == SceneNodeKind::KinematicSplineMover));

    let stream = pack_progressive_uorc_stream(11).expect("pack 4-LOD UORC stream");
    let summary = verify_progressive_uorc_stream(&stream).expect("verify UORC SHA-256 chain");
    assert_eq!(summary.lod_object_count, 4);
    assert_eq!(summary.max_lod_rank, 3);
    assert!(summary.compressed_bytes < summary.decoded_total_bytes);

    let restored_lod3 =
        decode_uorc_stream_up_to_lod(&stream, 3).expect("progressive decode up to LOD3");
    assert_eq!(restored_lod3, world);

    // Negative refusal 3: truncated / corrupted UORC stream is refused:
    let corrupt_err = verify_progressive_uorc_stream(&stream[..stream.len() / 2])
        .expect_err("truncated UORC stream must fail");
    assert!(matches!(
        corrupt_err,
        StreamingError::UorcArchiveRejected { .. }
    ));
}

/// `VE-02`: Exact `uor-matmul` tensor contractions, portable `16x16` tile PBR rasterizer, and `TensorComputeError` refusals.
#[test]
fn engine_ve_02() {
    let kernels = verified_engine_tensor_kernels().expect("7 certified tensor kernels");
    assert_eq!(kernels.len(), 7);
    for k in &kernels {
        assert!(k.lhs_cols > 0);
        assert!(k.accum_bits >= k.min_accum_bits);
        assert!(k.tile_aligned);
    }

    let verts = [[1.0_f32, -2.0, 3.5], [0.0, 4.0, -1.0]];
    let xf = Mat4::from_translation(Vec3::new(3.0, 5.0, -2.0));
    let out_verts = batch_transform_positions_exact(&verts, &xf).expect("exact homogeneous batch");
    assert_eq!(out_verts[0], [4.0, 3.0, 1.5, 1.0]);
    assert_eq!(out_verts[1], [3.0, 9.0, -3.0, 1.0]);

    let trop = tropical_frustum_distance_bounds(
        &[[100, 200, 50], [-50, 10, 300]],
        &[[5, 15, 25], [40, -10, 2]],
    )
    .expect("tropical max-plus bounds");
    assert_eq!(trop, vec![215, 190, 325, 302]);

    let audio =
        mix_spatial_audio_channels_exact(2, 2, 2, &[120, -40, 30, 20], &[2, 1, 1, 3]).expect("q15");
    assert_eq!(audio, vec![200, 0, 80, 90]);

    let vp = Mat4::perspective(1.0, 1.0, 0.25, 100.0)
        * Mat4::look_at(Vec3::new(0.0, 1.5, 6.0), Vec3::ZERO, Vec3::UP);
    let tri = RasterTriangle {
        positions: [[-1.2, -1.0, 0.0], [1.2, -1.0, 0.0], [0.0, 1.4, 0.0]],
        normal: [0.0, 0.0, 1.0],
        base_color: [0.4, 0.7, 0.9],
    };
    let fb_a = render_triangles_tile_framebuffer(64, 64, &vp, Vec3::new(0.3, 0.8, 0.5), 3, &[tri])
        .expect("tile framebuffer A");
    let fb_b = render_triangles_tile_framebuffer(64, 64, &vp, Vec3::new(0.3, 0.8, 0.5), 3, &[tri])
        .expect("tile framebuffer B");
    assert_eq!(fb_a, fb_b);
    assert!(fb_a.rasterized_triangles > 0);
    assert_eq!(fb_a.tiles_shaded, 16);

    // Negative refusal 1: empty / mismatched tensor extents:
    let empty_err =
        batch_transform_positions_exact(&[], &xf).expect_err("empty vertex batch must fail");
    assert!(matches!(
        empty_err,
        TensorComputeError::ZeroOrMismatchedExtents { .. }
    ));

    // Negative refusal 2: non-finite tensor entry:
    let nan_err = batch_transform_positions_exact(&[[f32::NAN, 0.0, 0.0]], &xf)
        .expect_err("NaN vertex entry must fail");
    assert!(matches!(
        nan_err,
        TensorComputeError::NonFiniteTensorEntry { .. }
    ));

    // Negative refusal 3: non-multiple of 16px tile dimensions:
    let tile_err = render_triangles_tile_framebuffer(50, 64, &vp, Vec3::UP, 3, &[tri])
        .expect_err("50px width is not 16px tile-aligned");
    assert!(matches!(
        tile_err,
        TensorComputeError::InvalidTileDimensions { .. }
    ));
}

/// `VE-03`: Archetype ECS hierarchical scene graph and SAH BVH raycast / frustum queries.
#[test]
fn engine_ve_03() {
    let mut scene = SceneWorld::new();
    let root_id = scene.spawn(
        "ArenaFloor",
        EntityRole::TerrainPatch,
        None,
        Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            rotation: Quat::IDENTITY,
            scale: Vec3::new(1.0, 1.0, 1.0),
        },
        Aabb::new(Vec3::new(-25.0, -1.0, -25.0), Vec3::new(25.0, 0.0, 25.0)),
        (0.25, "custom:floor"),
    );
    let tower_id = scene.spawn(
        "SignalTower",
        EntityRole::StaticArchitecture,
        Some(root_id),
        Transform {
            translation: Vec3::new(0.0, 4.0, -8.0),
            rotation: Quat::IDENTITY,
            scale: Vec3::new(1.0, 1.0, 1.0),
        },
        Aabb::new(Vec3::new(-1.5, -4.0, -1.5), Vec3::new(1.5, 4.0, 1.5)),
        (1.0, "custom:tower"),
    );

    let bvh = scene.build_bvh();
    let hit = bvh
        .raycast(
            Ray::new(Vec3::new(0.0, 4.0, 10.0), Vec3::new(0.0, 0.0, -1.0)),
            100.0,
        )
        .expect("ray hits tower");
    assert_eq!(hit.entity, tower_id);

    let frustum = Frustum::from_perspective(
        Vec3::new(0.0, 5.0, 16.0),
        Vec3::new(0.0, -0.1, -1.0),
        Vec3::UP,
        1.05,
        16.0 / 9.0,
        0.1,
        120.0,
    );
    let visible = bvh.query_frustum(&frustum);
    assert!(visible.contains(&root_id));
    assert!(visible.contains(&tower_id));
}

/// `VE-04`: Procedural meshes, morph targets, Khronos `gltf` `.glb` oracle, and `AssetValidationError` refusal.
#[test]
fn engine_ve_04() {
    let world = StandardSceneState::synthesize_at_lod(5, 3).expect("LOD3 scene");
    let mut meshes = canonical_scene_mesh_library(world.seed);
    let materials = canonical_pbr_material_library();

    let base_count = meshes[2].vertex_count();
    meshes[2].morph_targets.push(MorphTarget {
        name: "RibbonFlex".to_string(),
        position_deltas: vec![[0.0_f32, 0.12_f32, 0.0_f32]; base_count],
    });
    meshes[2].validate().expect("valid morph target mesh");

    let glb = export_scene_to_glb(&world.scene, &meshes, &materials).expect("glb export");
    let verified = validate_glb_bytes(&glb).expect("authoritative Khronos gltf validation");
    assert_eq!(verified.mesh_count, meshes.len());
    assert_eq!(verified.material_count, materials.len());

    // Negative refusal: empty mesh and out-of-bounds triangle index return `AssetValidationError`:
    let empty_mesh = ProceduralMesh {
        name: "EmptyBrokenMesh".to_string(),
        positions: Vec::new(),
        normals: Vec::new(),
        uvs: Vec::new(),
        colors: Vec::new(),
        indices: Vec::new(),
        morph_targets: Vec::new(),
    };
    let err = export_scene_to_glb(&world.scene, &[empty_mesh], &materials)
        .expect_err("empty mesh must be refused");
    assert!(matches!(
        err,
        AssetValidationError::EmptyMeshTopology { .. }
    ));

    let bad_index_mesh = ProceduralMesh {
        name: "OutOfBoundsIndexMesh".to_string(),
        positions: vec![[0.0, 0.0, 0.0]; 3],
        normals: vec![[0.0, 1.0, 0.0]; 3],
        uvs: vec![[0.0, 0.0]; 3],
        colors: vec![[1.0, 1.0, 1.0, 1.0]; 3],
        indices: vec![0, 1, 9],
        morph_targets: Vec::new(),
    };
    let idx_err = bad_index_mesh.validate().expect_err("bad index refused");
    assert!(matches!(
        idx_err,
        AssetValidationError::InvalidTriangleIndices { .. }
    ));
}

/// `VE-05`: W3C `naga` WGSL shader validation and `ShaderValidationError` refusal.
#[test]
fn engine_ve_05() {
    let modules = validate_builtin_engine_shaders().expect("all built-in WGSL shaders valid");
    assert_eq!(modules.len(), 4);

    // Negative refusal 1: malformed WGSL syntax rejected by `naga`:
    let syntax_err = validate_wgsl_shader("broken.wgsl", "@vertex fn vs_main( -> {}", &["vs_main"])
        .expect_err("broken WGSL must fail");
    assert!(matches!(
        syntax_err,
        ShaderValidationError::WgslParseRejected { .. }
    ));

    // Negative refusal 2: missing required entry point rejected:
    let missing_ep = validate_wgsl_shader(
        "pbr_missing_compute.wgsl",
        WGSL_PBR_SURFACE_SHADER,
        &["vs_main", "cs_nonexistent"],
    )
    .expect_err("missing entry point must fail");
    assert!(matches!(
        missing_ep,
        ShaderValidationError::MissingEntryPoint { .. }
    ));
}

/// `VE-06`: Symplectic Euler physics, capsule controller, kinematic spline mover, and `PhysicsStepError` refusal.
#[test]
fn engine_ve_06() {
    let world = StandardSceneState::synthesize_at_lod(13, 3).expect("LOD3 scene");

    let mut phys = PhysicsWorld::new(PhysicsWorldConfig::default());
    phys.bodies.push(
        RigidBody::new("FluidBuoy", Vec3::new(4.0, -0.35, -4.0), 10.0, 0.45, 0.35)
            .expect("valid rigid body"),
    );
    phys.player_capsule = Some(CapsuleController::new(Vec3::new(0.0, 2.5, 12.0)));
    phys.kinematic_mover = Some(KinematicSplineBody::new(
        world.mover_spline_waypoints.clone(),
        3.0,
    ));

    for _ in 0..30 {
        phys.step(1.0 / 60.0, Vec3::new(0.0, 0.0, -1.0))
            .expect("deterministic step");
    }
    let mover = phys.kinematic_mover.as_ref().expect("mover active");
    assert!(mover.distance_traveled_meters > 1.0);

    // Negative refusal 1: non-positive mass refused:
    let mass_err =
        RigidBody::new("ZeroMass", Vec3::ZERO, 0.0, 0.5, 0.2).expect_err("zero mass must fail");
    assert!(matches!(
        mass_err,
        PhysicsStepError::InvalidBodyMassOrRadius { .. }
    ));

    // Negative refusal 2: non-finite / negative timestep refused:
    let dt_err = phys
        .step(-0.016, Vec3::ZERO)
        .expect_err("negative timestep must fail");
    assert!(matches!(
        dt_err,
        PhysicsStepError::InvalidTimeStepSeconds { .. }
    ));
}

/// `VE-07`: Woodworth binaural spatial audio, Sabine RT60 reverberation, and Euler-Bernoulli modal bars.
#[test]
fn engine_ve_07() {
    let chamber = AcousticEnvironment::reflective_chamber(343.0);
    let exterior = AcousticEnvironment::damped_exterior(343.0);
    assert!(chamber.sabine_rt60_seconds() > exterior.sabine_rt60_seconds());
    assert!(chamber.eyring_rt60_seconds() > 0.0);

    let listener = ListenerPose {
        position: Vec3::ZERO,
        forward: Vec3::new(0.0, 0.0, -1.0),
        up: Vec3::UP,
    };
    let source_right = SpatialAudioSource {
        label: "ModalAcousticsFrame".to_string(),
        position: Vec3::new(6.0, 1.8, -2.0),
        fundamental_hz: 440.0,
        reference_gain: 1.0,
    };
    let cue = evaluate_binaural_cue(&source_right, &listener, &chamber);
    assert!(cue.azimuth_sin > 0.0);
    assert!(cue.ild_db > 0.0);
    assert!(cue.itd_seconds > 0.0);

    let modes = synthesize_modal_bar_modes(440.0, 4);
    assert_eq!(modes.len(), 4);
    assert!(modes[1] > modes[0]);
    assert!(chamber.sabine_rt60_prism_millis() > exterior.sabine_rt60_prism_millis());
}

/// `VE-08`: Acyclic worker job graph, frame-budget KPI profiler, and `EngineConfigError` refusal.
#[test]
fn engine_ve_08() {
    let cfg = EngineConfig::from_refresh_rate(1920, 1080, 60, 60, 4).expect("valid 60Hz config");
    let graph = WorkerJobGraph::canonical_frame_pipeline(450, 900, 500, 350, 250, 600);
    let (schedule, makespan) = graph
        .schedule(cfg.worker_threads)
        .expect("canonical frame pipeline is acyclic");
    assert!(!schedule.is_empty());

    let mut profiler = FrameBudgetProfiler::new(&cfg);
    profiler.record_frame(makespan);
    assert!(profiler.meets_budget_kpi());

    let mut cyclic = WorkerJobGraph::new();
    cyclic.add_job("a", 100, &["b"]);
    cyclic.add_job("b", 100, &["a"]);
    assert!(cyclic.schedule(2).is_none());

    let vp_err = EngineConfig::from_refresh_rate(0, 1080, 60, 60, 4)
        .expect_err("zero viewport width must fail");
    assert!(matches!(
        vp_err,
        EngineConfigError::ZeroViewportDimension { .. }
    ));

    let worker_err =
        EngineConfig::from_refresh_rate(1920, 1080, 60, 60, 0).expect_err("zero workers must fail");
    assert_eq!(worker_err, EngineConfigError::ZeroWorkerThreads);
}

/// `VE-09`: Bit-exact IEEE-754 `VDNT` binary `WorldSnapshot` round-trip and `SnapshotCodecError` refusal.
#[test]
fn engine_ve_09() {
    let world = StandardSceneState::synthesize_at_lod(19, 3).expect("LOD3 scene");

    let wire = WorldSnapshot::encode(&world).expect("encode binary snapshot");
    let restored = WorldSnapshot::decode(&wire).expect("decode binary snapshot");
    assert_eq!(restored, world);

    let trunc_err = WorldSnapshot::decode(&wire[..10]).expect_err("truncated wire must fail");
    assert!(matches!(
        trunc_err,
        SnapshotCodecError::TruncatedSnapshot { .. }
    ));

    let mut tampered = wire.clone();
    tampered[12] ^= 0x01;
    let tamper_err = WorldSnapshot::decode(&tampered).expect_err("tampered wire must fail");
    assert!(matches!(
        tamper_err,
        SnapshotCodecError::IntegrityDigestMismatch { .. }
    ));
}

/// `VE-10`: Core-Wasm engine verifier specification, theorem alignment with `site/app.json`, and zero `Classical.choice`.
#[test]
fn engine_ve_10() {
    let root = repo_root();
    let tensor_lex = std::fs::read_to_string(root.join("app/src/Tensor.lex.tex")).unwrap();
    let engine = std::fs::read_to_string(root.join("app/src/Engine.lex.tex")).unwrap();
    let app_json: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(root.join("site/app.json")).unwrap())
            .unwrap();

    let theorems = app_json["theorems"].as_array().expect("theorems array");
    assert!(!theorems.is_empty());
    for thm in theorems {
        let name = thm["name"].as_str().unwrap();
        let rel_where = thm["where"].as_str().expect("theorem where path");
        let lex_src = std::fs::read_to_string(root.join(rel_where)).expect("read theorem module");
        assert!(
            lex_src.contains(&format!("name: {name},")),
            "{rel_where} must declare theorem `{name}` advertised in site/app.json"
        );
    }

    assert!(engine.contains("entryRoot := \"VerdantEngine.Engine.dispatch\""));
    assert!(engine.contains("kernel:6"));
    assert!(engine.contains("scene:lod:3"));
    assert!(engine.contains("scene:node:6"));
    assert!(engine.contains("scene:acoustics"));
    assert!(engine.contains("scene:pipeline"));
    assert!(engine.contains("scene:uorc"));
    assert!(engine.contains("refused:zero-extent-contraction"));
    assert!(engine.contains("refused:accumulator-below-laurent-bound"));
    assert!(engine.contains("\"invalid request\""));
    assert!(!tensor_lex.contains("Classical.choice"));
    assert!(!engine.contains("Classical.choice"));

    // Verify that `verdant-core` executes the exact Lean-compiled Prism kernel (`prism_kernel`):
    let summary_bytes = verdant_core::prism_kernel::dispatch(b"summary".to_vec())
        .expect("prism_kernel dispatch summary");
    assert_eq!(
        std::str::from_utf8(&summary_bytes).unwrap(),
        "conformant:lods=4,kernels=7,lossless=1.0,zero-constants=true"
    );
    let scene_mover = verdant_core::prism_kernel::dispatch(b"scene:node:6".to_vec())
        .expect("prism_kernel dispatch scene:node:6");
    assert_eq!(
        std::str::from_utf8(&scene_mover).unwrap(),
        "conformant:id=6,name=KinematicSplineController,pos=180/15/0,dist=11,mover=1"
    );
    let refused_bytes = verdant_core::prism_kernel::dispatch(b"kernel:zerodim".to_vec())
        .expect("prism_kernel dispatch kernel:zerodim");
    assert_eq!(
        std::str::from_utf8(&refused_bytes).unwrap(),
        "refused:zero-extent-contraction"
    );
    let underflow_bytes = verdant_core::prism_kernel::dispatch(b"kernel:underflow".to_vec())
        .expect("prism_kernel dispatch kernel:underflow");
    assert_eq!(
        std::str::from_utf8(&underflow_bytes).unwrap(),
        "refused:accumulator-below-laurent-bound"
    );

    // Negative test for `ComputeError`: arithmetic overflow in `pow2Constructive` returns `Err(ComputeError)`:
    let overflow = verdant_core::prism_kernel::pow2Constructive(1_000)
        .expect_err("overflowing constructive power must return ComputeError");
    assert_eq!(
        overflow,
        verdant_core::prism_kernel::ComputeError::MulOverflow
    );

    // Verify deepened Prism subsystem acceptance queries directly in `prism_kernel::dispatch`:
    for (query, expected) in [
        (
            &b"scene:animation"[..],
            "conformant:joints_per_vertex=4,weight_scale=1000,morph_limit=8,acyclic=true",
        ),
        (
            &b"scene:rendergraph"[..],
            "conformant:csm_cascades=4,splits=1/3/7/15,denom=15,ibl_mips_256=9,passes=5",
        ),
        (
            &b"scene:collision"[..],
            "conformant:simplex=4,gjk_iters=32,baumgarte=1/5,coulomb_cone=true",
        ),
        (
            &b"scene:navmesh"[..],
            "conformant:walkable_ny_min=700/1000,max_nodes=1024,admissible_astar=true",
        ),
        (
            &b"scene:wasmbridge"[..],
            "conformant:worklet_quantum=128,channels=2,ring_slots=64,snapshot_url_max=32768",
        ),
        (
            &b"scene:uorc"[..],
            "conformant:tile_px=16,rgba=4,uorc_work=40,uorc_steps=4,chain=UORC-SHA256-CHAIN-v1",
        ),
    ] {
        let out = verdant_core::prism_kernel::dispatch(query.to_vec()).expect("dispatch");
        assert_eq!(std::str::from_utf8(&out).unwrap(), expected);
    }
}

/// `VE-11`: Skeletal joint hierarchies, normalized four-joint LBS skinning, keyframe sampling, and `AnimationError` refusal.
#[test]
fn engine_ve_11() {
    let skeleton = Skeleton::new(vec![
        Joint {
            name: "root".to_string(),
            parent_index: 0,
            is_root: true,
            rest_translation: Vec3::ZERO,
            rest_rotation: Quat::IDENTITY,
        },
        Joint {
            name: "arm".to_string(),
            parent_index: 0,
            is_root: false,
            rest_translation: Vec3::new(0.0, 1.0, 0.0),
            rest_rotation: Quat::IDENTITY,
        },
    ])
    .expect("valid acyclic skeleton");

    let clip = SkeletalClip {
        name: "LiftArm".to_string(),
        duration_millis: 1_000,
        keyframes: vec![
            KeyframePose {
                time_millis: 0,
                joint_translations: vec![Vec3::ZERO, Vec3::new(0.0, 1.0, 0.0)],
                joint_rotations: vec![Quat::IDENTITY, Quat::IDENTITY],
                morph_weights_permille: vec![0],
            },
            KeyframePose {
                time_millis: 1_000,
                joint_translations: vec![Vec3::ZERO, Vec3::new(0.0, 3.0, 0.0)],
                joint_rotations: vec![
                    Quat::IDENTITY,
                    Quat::from_axis_angle(Vec3::new(0.0, 0.0, 1.0), 0.5),
                ],
                morph_weights_permille: vec![1_000],
            },
        ],
    };

    let mid_pose = clip.sample(&skeleton, 500).expect("midpoint pose");
    assert!((mid_pose.joint_translations[1].y - 2.0).abs() < 1e-3);
    assert_eq!(mid_pose.morph_weights_permille[0], 500);

    let mesh = ProceduralMesh {
        name: "LimbQuad".to_string(),
        positions: vec![[0.0, 1.0, 0.0], [0.5, 1.0, 0.0], [0.0, 2.0, 0.0]],
        normals: vec![[0.0, 0.0, 1.0]; 3],
        uvs: vec![[0.0, 0.0], [1.0, 0.0], [0.0, 1.0]],
        colors: vec![[1.0, 1.0, 1.0, 1.0]; 3],
        indices: vec![0, 1, 2],
        morph_targets: vec![MorphTarget {
            name: "Crest".to_string(),
            position_deltas: vec![[0.0, 0.2, 0.0]; 3],
        }],
    };
    let weights = vec![
        SkinWeights4::new([0, 1, 0, 0], [500, 500, 0, 0], skeleton.joints.len()).unwrap(),
        SkinWeights4::new([1, 0, 0, 0], [1_000, 0, 0, 0], skeleton.joints.len()).unwrap(),
        SkinWeights4::new([1, 0, 0, 0], [1_000, 0, 0, 0], skeleton.joints.len()).unwrap(),
    ];
    let deformed =
        skin_mesh_linear_blend(&mesh, &skeleton, &mid_pose, &weights).expect("skinned mesh");
    assert!(deformed.positions[2][1] > mesh.positions[2][1]);

    // Negative refusal 1: cyclic / out-of-order parent joint refused:
    let bad_skel = Skeleton::new(vec![
        Joint {
            name: "root".to_string(),
            parent_index: 0,
            is_root: true,
            rest_translation: Vec3::ZERO,
            rest_rotation: Quat::IDENTITY,
        },
        Joint {
            name: "self_parent".to_string(),
            parent_index: 1,
            is_root: false,
            rest_translation: Vec3::ZERO,
            rest_rotation: Quat::IDENTITY,
        },
    ])
    .expect_err("self-parented joint must be refused");
    assert!(matches!(
        bad_skel,
        AnimationError::InvalidJointHierarchy { .. }
    ));

    // Negative refusal 2: unnormalized skinning weights refused:
    let bad_weights = SkinWeights4::new([0, 1, 0, 0], [400, 400, 0, 0], skeleton.joints.len())
        .expect_err("weights summing to 800 != 1000 must fail");
    assert!(matches!(
        bad_weights,
        AnimationError::UnnormalizedOrInvalidSkinWeights { .. }
    ));
}

/// `VE-12`: Cascaded shadow frustum splits, IBL mip chains, five-pass render graph, and `RenderGraphError` refusal.
#[test]
fn engine_ve_12() {
    let splits = compute_csm_splits(0.5, 150.5).expect("valid CSM splits");
    assert_eq!(splits.len(), 4);
    assert!((splits[0].near_meters - 0.5).abs() < 1e-3);
    assert!((splits[3].far_meters - 150.5).abs() < 1e-3);
    for pair in splits.windows(2) {
        assert!(pair[1].far_meters > pair[0].far_meters);
    }

    let probe = IblEnvironmentProbe::from_resolution(256).expect("256px cubemap");
    assert_eq!(probe.mip_level_count, 9);

    let graph = RenderFrameGraph::canonical_pbr_shadow_pipeline();
    assert_eq!(graph.validate().expect("canonical 5-pass graph"), 5);

    // Negative refusal 1: inverted shadow bounds refused:
    let bounds_err = compute_csm_splits(50.0, 10.0).expect_err("far < near must fail");
    assert!(matches!(
        bounds_err,
        RenderGraphError::InvalidShadowFrustumBounds { .. }
    ));

    // Negative refusal 2: backward render-pass barrier edge refused:
    let mut bad_graph = RenderFrameGraph::canonical_pbr_shadow_pipeline();
    bad_graph.dependencies.push((3, 1));
    let dep_err = bad_graph.validate().expect_err("backward pass edge fails");
    assert!(matches!(
        dep_err,
        RenderGraphError::InvalidPassDependency { .. }
    ));
}

/// `VE-13`: 3D GJK convex hull collision manifolds, Prism contact solver, and `CollisionError` refusal.
#[test]
fn engine_ve_13() {
    let box_a = ConvexHullCollider::from_box_half_extents(Vec3::new(1.0, 1.0, 1.0)).unwrap();
    let box_b = ConvexHullCollider::from_box_half_extents(Vec3::new(1.0, 1.0, 1.0)).unwrap();

    let hit =
        gjk_intersect_and_manifold(&box_a, Vec3::ZERO, &box_b, Vec3::new(1.2, 0.0, 0.0)).unwrap();
    assert!(hit.intersecting);
    assert!(hit.penetration_meters > 0.7 && hit.penetration_meters < 0.9);

    let sep =
        gjk_intersect_and_manifold(&box_a, Vec3::ZERO, &box_b, Vec3::new(3.5, 0.0, 0.0)).unwrap();
    assert!(!sep.intersecting);
    assert_eq!(sep.penetration_meters, 0.0);

    let solved = solve_contact_impulse(4.0, 3.0, 5.0, 0.5, 0.4, 0.05, 0.01, 1.0 / 60.0)
        .expect("valid contact impulse");
    assert!(solved.normal_impulse_ns > 0.0);
    assert!(solved.friction_cone_valid);
    assert!(solved.tangent_impulse_ns <= solved.normal_impulse_ns * 0.4 + 1e-3);

    // Negative refusal: empty hull refused with `CollisionError`:
    let empty_err = ConvexHullCollider::new(Vec::new()).expect_err("empty hull must fail");
    assert_eq!(
        empty_err,
        CollisionError::EmptyOrDegenerateHull { vertex_count: 0 }
    );
}

/// `VE-14`: Slope-filtered terrain `NavMesh`, admissible `A*` corridor pathfinding, and `NavigationError` refusal.
#[test]
fn engine_ve_14() {
    let meshes = canonical_scene_mesh_library(7);
    let nav = NavMesh::from_mesh(&meshes[0]).expect("walkable terrain navmesh");
    assert!(nav.polygons.len() > 16);

    let start = 0usize;
    let mid = nav.polygons[start].neighbors[0];
    let goal = *nav.polygons[mid]
        .neighbors
        .iter()
        .find(|&&n| n != start)
        .unwrap_or(&mid);
    let corridor = nav
        .find_path_astar(start, goal)
        .expect("connected corridor");
    assert!(corridor.polygon_indices.len() >= 2);
    let dir = steer_along_corridor(nav.polygons[start].centroid, &corridor, 2.0)
        .expect("unit steering vector");
    assert!((dir.length() - 1.0).abs() < 1e-6);

    // Negative refusal 1: out-of-bounds polygon index refused:
    let oob = nav
        .find_path_astar(0, nav.polygons.len() + 5)
        .expect_err("out-of-bounds polygon must fail");
    assert!(matches!(oob, NavigationError::InvalidPolygonIndex { .. }));

    // Negative refusal 2: disconnected island polygon refused:
    let mut disconnected = nav.clone();
    let island_idx = disconnected.polygons.len();
    disconnected
        .polygons
        .push(verdant_core::navmesh::NavPolygon {
            index: island_idx,
            vertices: [
                Vec3::new(90.0, 0.0, 90.0),
                Vec3::new(91.0, 0.0, 90.0),
                Vec3::new(90.0, 0.0, 91.0),
            ],
            centroid: Vec3::new(90.3, 0.0, 90.3),
            normal_y_milli: 1_000,
            neighbors: Vec::new(),
        });
    let unreach = disconnected
        .find_path_astar(0, island_idx)
        .expect_err("disconnected island must fail");
    assert!(matches!(unreach, NavigationError::UnreachableGoal { .. }));
}

/// `VE-15`: `WasmRuntimeBridge` steps physics, renders portable tensor framebuffers, encodes/restores SHA-256 `VDNT` & `UORC`, verifies `site/verdant-wasm.wasm`, and refuses invalid inputs.
#[test]
fn engine_ve_15() {
    use verdant_wasm::{WasmBridgeError, WasmRuntimeBridge};

    let mut bridge = WasmRuntimeBridge::new(7).expect("WasmRuntimeBridge::new");
    assert_eq!(bridge.lod_rank, 3);
    assert_eq!(bridge.world.scene_nodes.len(), 7);
    assert!(bridge.walkable_polygons > 100);
    let lod3_snap = bridge
        .encode_snapshot()
        .expect("LOD3 snapshot fits Prism URL bridge envelope");
    assert_eq!(&lod3_snap[0..4], b"VDNT");

    let fb_bytes = bridge
        .render_tensor_framebuffer(64, 64, Vec3::new(0.0, 8.0, 22.0), Vec3::ZERO)
        .expect("portable tensor framebuffer");
    assert_eq!(fb_bytes, 64 * 64 * 4);

    let mover_before = bridge
        .physics
        .kinematic_mover
        .as_ref()
        .expect("mover")
        .position;
    bridge
        .step_frame(1.0 / 60.0, 1.0, 0.0, 0.0)
        .expect("step_frame");
    let mover_after = bridge
        .physics
        .kinematic_mover
        .as_ref()
        .expect("mover")
        .position;
    assert!((mover_after - mover_before).length() > 1e-4);

    bridge.set_lod_rank(0).expect("coarse LOD0 proxy");
    assert_eq!(bridge.lod_rank, 0);
    assert!(bridge.world.scene_nodes.is_empty());
    let snap = bridge.encode_snapshot().expect("encode VDNT snapshot");
    assert_eq!(&snap[0..4], b"VDNT");

    bridge.set_lod_rank(3).expect("back to lossless LOD3");
    assert_eq!(bridge.world.scene_nodes.len(), 7);
    bridge
        .decode_and_restore_snapshot(&snap)
        .expect("restore VDNT snapshot");
    assert_eq!(bridge.lod_rank, 0);
    assert!(bridge.world.scene_nodes.is_empty());

    let uorc_stream = bridge.encode_uorc_stream().expect("encode UORC stream");
    bridge
        .decode_and_restore_uorc(&uorc_stream, 3)
        .expect("restore lossless LOD3 from UORC stream");
    assert_eq!(bridge.lod_rank, 3);
    assert_eq!(bridge.world.scene_nodes.len(), 7);

    // Negative refusal 1: invalid LOD rank (> 3):
    let bad_lod = bridge
        .set_lod_rank(9)
        .expect_err("invalid LOD rank must fail");
    assert!(matches!(
        bad_lod,
        WasmBridgeError::InvalidLodRank { rank: 9 }
    ));

    // Negative refusal 2: tampered SHA-256 snapshot byte refused:
    let mut tampered = snap;
    let last = tampered.len() - 1;
    tampered[last] ^= 0x5A;
    let bad_snap = bridge
        .decode_and_restore_snapshot(&tampered)
        .expect_err("tampered snapshot must fail");
    assert!(matches!(
        bad_snap,
        WasmBridgeError::InvalidSnapshotEnvelope { .. }
    ));

    // Negative refusal 3: worker command ring capacity overflow refused:
    let mut overflow_bridge = WasmRuntimeBridge::new(7).expect("bridge");
    for _ in 0..64 {
        overflow_bridge.record_command().expect("within ring");
    }
    let ring_err = overflow_bridge
        .record_command()
        .expect_err("65th queued command must refuse");
    assert!(matches!(
        ring_err,
        WasmBridgeError::CommandRingOverflow { capacity: 64 }
    ));

    // Verify committed site/verdant-wasm.wasm binary artifact has valid WebAssembly header:
    let wasm_path = repo_model::repo_root()
        .join("site")
        .join("verdant-wasm.wasm");
    let wasm_bytes = std::fs::read(&wasm_path).expect("site/verdant-wasm.wasm must exist");
    assert_eq!(&wasm_bytes[0..4], b"\0asm");
}

/// `VE-16`: `verdant-cli` asset/shader/snapshot/UORC validator, progressive scene packer, headless benchmark, and `CliCommandError` refusals.
#[test]
fn engine_ve_16() {
    use verdant_cli::{run_cli, CliCommandError};

    let tmp_dir = std::env::temp_dir().join(format!("verdant_cli_ve16_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&tmp_dir);
    std::fs::create_dir_all(&tmp_dir).expect("temp dir");

    let glb_path = tmp_dir.join("standard_scene.glb");
    let vdnt_path = tmp_dir.join("standard_scene.glb.vdnt");
    let uorc_path = tmp_dir.join("standard_scene.glb.uorc");
    let wgsl_path = tmp_dir.join("pbr_surface.wgsl");
    std::fs::write(&wgsl_path, WGSL_PBR_SURFACE_SHADER).expect("write wgsl");

    let pack_out = run_cli(&[
        "pack-scene".to_string(),
        glb_path.to_string_lossy().into_owned(),
        "7".to_string(),
        "3".to_string(),
    ])
    .expect("pack-scene succeeds");
    assert!(pack_out.contains("PACKED OK:"));
    assert!(pack_out.contains("nodes=7"));

    let uorc_out = run_cli(&[
        "verify-uorc".to_string(),
        uorc_path.to_string_lossy().into_owned(),
    ])
    .expect("verify-uorc succeeds");
    assert!(uorc_out.contains("UORC OK:"));
    assert!(uorc_out.contains("lod_objects=4"));

    let glb_out = run_cli(&[
        "validate-glb".to_string(),
        glb_path.to_string_lossy().into_owned(),
    ])
    .expect("validate-glb succeeds");
    assert!(glb_out.contains("GLB OK:"));

    let wgsl_out = run_cli(&[
        "validate-wgsl".to_string(),
        wgsl_path.to_string_lossy().into_owned(),
        "vs_main".to_string(),
        "fs_main".to_string(),
    ])
    .expect("validate-wgsl succeeds");
    assert!(wgsl_out.contains("WGSL OK:"));

    let snap_out = run_cli(&[
        "snapshot-verify".to_string(),
        vdnt_path.to_string_lossy().into_owned(),
    ])
    .expect("snapshot-verify succeeds");
    assert!(snap_out.contains("VDNT OK:"));
    assert!(snap_out.contains("url_bridge_ready=true"));

    let bench_out = run_cli(&["benchmark".to_string(), "12".to_string()]).expect("benchmark");
    assert!(bench_out.contains("BENCHMARK OK: frames=12"));

    let ui_out = run_cli(&[
        "verify-ui".to_string(),
        "1280".to_string(),
        "900".to_string(),
    ])
    .expect("verify-ui succeeds");
    assert!(ui_out.contains("UI OK: viewport=1280x900"));

    let intent_out = run_cli(&[
        "route-intent".to_string(),
        "switch".to_string(),
        "to".to_string(),
        "software".to_string(),
        "16x16".to_string(),
        "tile".to_string(),
        "raster".to_string(),
    ])
    .expect("route-intent succeeds");
    assert!(intent_out.contains("INTENT OK: slug=render-tile-wasm"));

    let shards_dir = tmp_dir.join("shards");
    let shard_out = run_cli(&[
        "stitch-shard".to_string(),
        uorc_path.to_string_lossy().into_owned(),
        shards_dir.to_string_lossy().into_owned(),
    ])
    .expect("stitch-shard succeeds");
    assert!(shard_out.contains("STITCH SHARD OK:"));
    let recovered_uorc = tmp_dir.join("recovered.uorc");
    let rec_out = run_cli(&[
        "stitch-recover".to_string(),
        shards_dir
            .join("shard_0.stch")
            .to_string_lossy()
            .into_owned(),
        shards_dir
            .join("shard_2.stch")
            .to_string_lossy()
            .into_owned(),
        recovered_uorc.to_string_lossy().into_owned(),
    ])
    .expect("stitch-recover succeeds");
    assert!(rec_out.contains("STITCH RECOVER OK: pair=(0,2)"));
    assert_eq!(
        std::fs::read(&recovered_uorc).expect("read recovered"),
        std::fs::read(&uorc_path).expect("read original uorc")
    );

    // Negative refusal 1: unknown subcommand:
    let unknown_err =
        run_cli(&["nonexistent-cmd".to_string()]).expect_err("unknown command must fail");
    assert!(matches!(unknown_err, CliCommandError::InvalidUsage { .. }));

    // Negative refusal 2: broken WGSL shader rejected:
    let bad_wgsl_path = tmp_dir.join("broken.wgsl");
    std::fs::write(&bad_wgsl_path, "@vertex fn vs_main( -> {}").expect("write bad wgsl");
    let bad_wgsl_err = run_cli(&[
        "validate-wgsl".to_string(),
        bad_wgsl_path.to_string_lossy().into_owned(),
    ])
    .expect_err("broken WGSL must fail");
    assert!(matches!(
        bad_wgsl_err,
        CliCommandError::ValidationRejected { .. }
    ));

    // Negative refusal 3: corrupted UORC file rejected:
    let bad_uorc_path = tmp_dir.join("corrupt.uorc");
    std::fs::write(&bad_uorc_path, b"NOT_UORC").expect("write bad uorc");
    let bad_uorc_err = run_cli(&[
        "verify-uorc".to_string(),
        bad_uorc_path.to_string_lossy().into_owned(),
    ])
    .expect_err("corrupted UORC must fail");
    assert!(matches!(
        bad_uorc_err,
        CliCommandError::ValidationRejected { .. }
    ));

    let _ = std::fs::remove_dir_all(&tmp_dir);
}

/// `VE-17`: Unified `verdant-tensor` semiring contractions (`(+, *)`, tropical `(max, +)` / `(min, +)`, `Q15` Hadamard FDN audio ring) and `TensorComputeError` refusals.
#[test]
fn engine_ve_17() {
    use verdant_core::audio::synthesize_binaural_reverb_quantum_exact;
    use verdant_tensor::{
        delassus_effective_mass_matrix, verified_coded_matrix_tiers,
        verified_unified_tensor_kernels,
    };

    let kernels = verified_unified_tensor_kernels().expect("15 unified semiring kernels");
    assert_eq!(kernels.len(), 15);
    let tiers = verified_coded_matrix_tiers().expect("3 zero-decode coded matrix tiers");
    assert_eq!(tiers.len(), 3);
    assert!(tiers.iter().all(|t| t.zero_scratch));

    // Hierarchical scene propagation via exact `(+, *)` tensor contraction:
    let world = StandardSceneState::synthesize_at_lod(7, 3).expect("LOD3 scene");
    let world_mats = world
        .scene
        .propagate_all_world_matrices()
        .expect("hierarchical propagation");
    assert_eq!(world_mats.len(), world.scene.entity_count());

    // Delassus effective-mass contraction `K = J * M^-1 * J^T`:
    let jacobian = [1.0, 0.0, 0.0, -1.0, 0.0, 0.0];
    let inv_masses = [0.5, 0.5, 0.5, 0.25, 0.25, 0.25];
    let k_mat =
        delassus_effective_mass_matrix(&jacobian, &inv_masses, 1, 6).expect("Delassus matrix");
    assert_eq!(k_mat.len(), 1);
    assert!((k_mat[0] - 0.75).abs() < 1e-3);

    // Tropical `(min, +)` all-pairs shortest-path closure on NavMesh:
    let meshes = canonical_scene_mesh_library(7);
    let mut small_mesh = meshes[0].clone();
    small_mesh.indices.truncate(3 * 24);
    let nav = NavMesh::from_mesh(&small_mesh).expect("walkable small navmesh");
    let closure = nav
        .all_pairs_shortest_paths_tropical()
        .expect("tropical min-plus all-pairs closure");
    assert_eq!(closure.len(), nav.polygons.len() * nav.polygons.len());
    assert_eq!(closure[0], Some(0));

    // Quantized Q15 modal FIR + Woodworth panning + 4-channel Hadamard FDN reverb ring:
    let dry: Vec<i16> = (0..16)
        .map(|i| if i == 0 { 16_000 } else { 1_200 })
        .collect();
    let taps = [16_384_i16, 8_192, 4_096, 2_048];
    let env = AcousticEnvironment::reflective_chamber(343.2);
    let src = SpatialAudioSource {
        label: "ModalBar".to_string(),
        position: Vec3::new(4.0, 1.5, -3.0),
        fundamental_hz: 440.0,
        reference_gain: 1.0,
    };
    let listener = ListenerPose {
        position: Vec3::new(0.0, 1.7, 0.0),
        forward: Vec3::new(0.0, 0.0, -1.0),
        up: Vec3::UP,
    };
    let quantum = synthesize_binaural_reverb_quantum_exact(&dry, &taps, &src, &listener, &env)
        .expect("exact audio-ring FDN quantum");
    assert_eq!(quantum.stereo_frames_accum.len(), 32);
    assert_eq!(quantum.fdn_reverb_tank.len(), 16);

    // Negative refusal: mismatched Delassus Jacobian dimensions return `TensorComputeError`:
    let bad_delassus = delassus_effective_mass_matrix(&[1.0, 0.0], &inv_masses, 2, 6)
        .expect_err("mismatched Jacobian rows/cols must refuse");
    assert!(matches!(
        bad_delassus,
        TensorComputeError::ZeroOrMismatchedExtents { .. }
    ));
}

/// `VE-18`: Portable `.uorc` progressive streams via `uorc_decompress::run`, `uor_matmul::codec` (`Grid`, `Packed`, `Runs`) zero-decode contractions, and `StreamingError` refusals.
#[test]
fn engine_ve_18() {
    use verdant_core::streaming::{
        decode_portable_uorc_single_frame, decode_portable_uorc_stream_up_to_lod,
        pack_portable_progressive_uorc_stream, split_self_delimiting_uorc_archives,
        CodedLodFrameBundle,
    };

    let stream = pack_portable_progressive_uorc_stream(7).expect("portable 4-LOD UORC stream");
    let frames = split_self_delimiting_uorc_archives(&stream).expect("4 self-delimiting frames");
    assert_eq!(frames.len(), 4);

    let lod3_full =
        decode_portable_uorc_stream_up_to_lod(&stream, 3).expect("monotonic 4-frame decode");
    let lod3_fast =
        decode_portable_uorc_single_frame(&stream, 3).expect("single-frame LOD3 decode");
    assert_eq!(lod3_full, lod3_fast);
    assert_eq!(lod3_fast.active_lod, LodLevel::Lod3LosslessSource);
    assert_eq!(lod3_fast.scene_nodes.len(), 7);

    // Zero-decode `uor_matmul::coded_gemm` contractions across Grid, Packed, and Runs:
    let coded = CodedLodFrameBundle::synthesize(7, 3).expect("coded LOD3 bundle");
    let summary = coded
        .contract_zero_decode(&[1, 2, 3, 4], &[4, 3, 2, 1], &[1, -1, 1, -1])
        .expect("zero-decode coded_gemm contractions");
    assert_eq!(summary.geometry_accum.len(), 4);
    assert_eq!(summary.animation_accum.len(), 4);
    assert_eq!(summary.ibl_accum.len(), 4);

    // Negative refusal: tampered portable `.uorc` seal refused by `uorc_decompress::run`:
    let mut corrupted = stream;
    let tail = corrupted.len() - 1;
    corrupted[tail] ^= 0xFF;
    let err = decode_portable_uorc_single_frame(&corrupted, 3)
        .expect_err("tampered UORC SHA-256 chain seal must fail");
    assert!(matches!(err, StreamingError::UorcArchiveRejected { .. }));
}

/// `VE-19`: Full `.glb` scene import, `.glb -> .uorc` transcoding, XPBD joints, swept capsule & 4-wheel raycast vehicle controllers, BVH acoustic occlusion, and `PhysicsStepError` refusals.
#[test]
fn engine_ve_19() {
    use verdant_cli::run_cli;
    use verdant_core::audio::evaluate_binaural_cue_with_occlusion;
    use verdant_core::gltf_io::{import_glb_to_scene, transcode_glb_to_progressive_uorc};
    use verdant_core::physics::{JointKind, RaycastVehicle, RigidJointConstraint};
    use verdant_core::streaming::decode_portable_uorc_single_frame;

    // 1. Export a rich scene to `.glb`, import back into `SceneWorld`, and transcode `.glb -> .uorc`:
    let mut source_state = StandardSceneState::synthesize_at_lod(7, 2).expect("LOD2 scene");
    let meshes = canonical_scene_mesh_library(7);
    let materials = canonical_pbr_material_library();
    let eids: Vec<_> = source_state.scene.entities().map(|e| e.id).collect();
    for (idx, eid) in eids.into_iter().take(meshes.len()).enumerate() {
        source_state
            .scene
            .attach_renderable(eid, idx, idx % materials.len());
    }
    let glb_bytes = export_scene_to_glb(&source_state.scene, &meshes, &materials).expect("export");
    let imported = import_glb_to_scene(&glb_bytes).expect("import_glb_to_scene");
    assert_eq!(imported.meshes.len(), meshes.len());
    assert!(!imported.materials.is_empty());
    assert!(imported.scene.entity_count() >= meshes.len());

    let transcoded_uorc =
        transcode_glb_to_progressive_uorc(&glb_bytes, 7).expect("transcode .glb -> .uorc");
    let restored_lod3 =
        decode_portable_uorc_single_frame(&transcoded_uorc, 3).expect("decode transcoded UORC");
    assert!(restored_lod3.scene.entity_count() > source_state.scene.entity_count());

    // Exercise CLI `import-glb` and `transcode-glb`:
    let tmp_dir = std::env::temp_dir().join(format!("verdant_ve19_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&tmp_dir);
    std::fs::create_dir_all(&tmp_dir).expect("temp dir");
    let glb_file = tmp_dir.join("authored.glb");
    let uorc_file = tmp_dir.join("authored.uorc");
    std::fs::write(&glb_file, &glb_bytes).expect("write glb");
    let cli_imp = run_cli(&[
        "import-glb".to_string(),
        glb_file.to_string_lossy().into_owned(),
    ])
    .expect("CLI import-glb");
    assert!(cli_imp.contains("GLB IMPORTED OK:"));
    let cli_tr = run_cli(&[
        "transcode-glb".to_string(),
        glb_file.to_string_lossy().into_owned(),
        uorc_file.to_string_lossy().into_owned(),
        "7".to_string(),
    ])
    .expect("CLI transcode-glb");
    assert!(cli_tr.contains("GLB TRANSCODED UORC OK:"));
    let _ = std::fs::remove_dir_all(&tmp_dir);

    // 2. XPBD bilateral joint constraint + 4-wheel raycast suspension vehicle + swept BVH capsule:
    let mut physics = PhysicsWorld::new(PhysicsWorldConfig::default());
    physics.bodies.push(
        RigidBody::new("AnchorLink", Vec3::new(0.0, 4.0, 0.0), 10.0, 0.4, 0.2).expect("body A"),
    );
    physics.bodies.push(
        RigidBody::new("PendulumBob", Vec3::new(3.2, 4.0, 0.0), 5.0, 0.4, 0.2).expect("body B"),
    );
    assert_eq!(JointKind::Hinge.locked_dofs(), 5);
    assert_eq!(JointKind::BallSocket.locked_dofs(), 3);
    physics.rigid_joints.push(RigidJointConstraint {
        label: "ArmHinge".to_string(),
        body_a: 0,
        body_b: 1,
        kind: JointKind::Hinge,
        axis_world: Vec3::new(0.0, 0.0, 1.0),
        target_distance_meters: 2.0,
        compliance_permille: 1,
    });
    let mut vehicle =
        RaycastVehicle::new("ChassisRig", Vec3::new(0.0, 1.0, 6.0), 850.0).expect("vehicle");
    vehicle.throttle = 1.0;
    vehicle.steering_radians = 0.15;
    physics.vehicles.push(vehicle);

    for _ in 0..15 {
        physics.step(1.0 / 60.0, Vec3::ZERO).expect("step physics");
    }
    let sep = (physics.bodies[1].position - physics.bodies[0].position).length();
    assert!(
        sep < 3.0,
        "XPBD joint pulled separation from 3.2m toward 2.0m (got {sep})"
    );
    assert!(physics.vehicles[0].chassis_position.z > 6.01);

    // Swept capsule against BVH wall + BVH acoustic occlusion:
    let mut wall_scene = SceneWorld::new();
    wall_scene.spawn(
        "AcousticWall",
        EntityRole::StaticArchitecture,
        None,
        Transform::from_xyz(0.0, 1.5, 0.0),
        Aabb::new(Vec3::new(-4.0, -1.5, -0.5), Vec3::new(4.0, 2.5, 0.5)),
        (1.0, "Wall occluder"),
    );
    let wall_bvh = wall_scene.build_bvh();
    let mut capsule = CapsuleController::new(Vec3::new(0.0, 0.0, 2.0));
    capsule
        .step_swept_against_bvh(0.25, Vec3::new(0.0, 0.0, -1.0), 9.81, &wall_bvh, |_, _| 0.0)
        .expect("swept capsule step");
    assert!(
        capsule.foot_position.z > 0.5,
        "swept capsule stopped before penetrating wall"
    );

    let env = AcousticEnvironment::reflective_chamber(343.2);
    let src = SpatialAudioSource {
        label: "BehindWall".to_string(),
        position: Vec3::new(0.0, 1.5, -4.0),
        fundamental_hz: 330.0,
        reference_gain: 1.0,
    };
    let listener = ListenerPose {
        position: Vec3::new(0.0, 1.5, 4.0),
        forward: Vec3::new(0.0, 0.0, -1.0),
        up: Vec3::UP,
    };
    let unoccluded = evaluate_binaural_cue(&src, &listener, &env);
    let (occluded_cue, is_occluded) =
        evaluate_binaural_cue_with_occlusion(&src, &listener, &env, &wall_bvh);
    assert!(is_occluded);
    assert!(occluded_cue.direct_gain < unoccluded.direct_gain);

    // Negative refusal: XPBD joint referencing non-existent body index returns `PhysicsStepError`:
    physics.rigid_joints.push(RigidJointConstraint {
        label: "DanglingJoint".to_string(),
        body_a: 0,
        body_b: 99,
        kind: JointKind::BallSocket,
        axis_world: Vec3::UP,
        target_distance_meters: 1.0,
        compliance_permille: 1,
    });
    let joint_err = physics
        .step(1.0 / 60.0, Vec3::ZERO)
        .expect_err("out-of-bounds joint body must refuse");
    assert!(matches!(
        joint_err,
        PhysicsStepError::InvalidBodyMassOrRadius { .. }
    ));
}

/// `VE-20`: SHA-256-chained `DeterministicInputJournal`, 64-frame `RollbackSession` resimulation, stateless `DeltaUorcSnapshot`, and `SnapshotCodecError` refusals.
#[test]
fn engine_ve_20() {
    use verdant_core::runtime::{
        DeltaUorcSnapshot, DeterministicInputJournal, FrameInputCommand, RollbackSession,
    };

    // 1. Rollback session vs clean forward oracle:
    let make_world = || {
        let mut pw = PhysicsWorld::new(PhysicsWorldConfig::default());
        pw.player_capsule = Some(CapsuleController::new(Vec3::new(0.0, 2.0, 10.0)));
        pw
    };
    let dt = 1.0 / 60.0;
    let mut rollback = RollbackSession::new(make_world(), dt);
    assert_eq!(RollbackSession::max_window_frames(), 64);

    for f in 0..12 {
        rollback
            .advance_frame(FrameInputCommand {
                frame_index: f,
                player_slot: 0,
                move_fwd_milli: if f < 6 { 1_000 } else { 0 },
                move_strafe_milli: 0,
                yaw_milli: 0,
                action_mask: 0,
            })
            .expect("advance frame");
    }

    // Correct frame 4 to strafe right (`move_strafe_milli = 800`) and resimulate `4..12`:
    let corrected = FrameInputCommand {
        frame_index: 4,
        player_slot: 0,
        move_fwd_milli: 1_000,
        move_strafe_milli: 800,
        yaw_milli: 0,
        action_mask: 1,
    };
    let resim_count = rollback
        .rollback_and_resimulate(corrected)
        .expect("rollback within 64-frame window");
    assert_eq!(resim_count, 8);

    // Run a fresh session with the exact corrected command history and verify bit-identity:
    let mut oracle = RollbackSession::new(make_world(), dt);
    for cmd in rollback.journal.commands().iter().copied() {
        oracle.advance_frame(cmd).expect("oracle step");
    }
    assert_eq!(
        rollback.journal.chain_digest_hex(),
        oracle.journal.chain_digest_hex()
    );
    assert_eq!(
        rollback.physics.player_capsule,
        oracle.physics.player_capsule
    );

    // 2. Stateless XOR-residual `DeltaUorcSnapshot` round-trip:
    let lod1 = StandardSceneState::synthesize_at_lod(7, 1).expect("LOD1");
    let lod2 = StandardSceneState::synthesize_at_lod(7, 2).expect("LOD2");
    let delta_bytes = DeltaUorcSnapshot::encode_delta(&lod1, &lod2).expect("encode delta UORC");
    assert_eq!(&delta_bytes[0..4], b"UORC");
    let reconstructed =
        DeltaUorcSnapshot::decode_delta(&lod1, &delta_bytes).expect("decode delta UORC");
    assert_eq!(reconstructed, lod2);

    // Negative refusal 1: regressive input journal frame refused:
    let mut journal = DeterministicInputJournal::new();
    journal
        .append(FrameInputCommand {
            frame_index: 5,
            player_slot: 0,
            move_fwd_milli: 0,
            move_strafe_milli: 0,
            yaw_milli: 0,
            action_mask: 0,
        })
        .expect("initial frame 5");
    let reg_err = journal
        .append(FrameInputCommand {
            frame_index: 5,
            player_slot: 0,
            move_fwd_milli: 0,
            move_strafe_milli: 0,
            yaw_milli: 0,
            action_mask: 0,
        })
        .expect_err("duplicate/regressive frame must refuse");
    assert!(matches!(
        reg_err,
        SnapshotCodecError::MalformedStatePayload { .. }
    ));

    // Negative refusal 2: rollback beyond 64-frame window refused:
    let mut long_session = RollbackSession::new(make_world(), dt);
    for f in 0..70 {
        long_session
            .advance_frame(FrameInputCommand {
                frame_index: f,
                player_slot: 0,
                move_fwd_milli: 500,
                move_strafe_milli: 0,
                yaw_milli: 0,
                action_mask: 0,
            })
            .expect("step");
    }
    let window_err = long_session
        .rollback_and_resimulate(FrameInputCommand {
            frame_index: 2,
            player_slot: 0,
            move_fwd_milli: -500,
            move_strafe_milli: 0,
            yaw_milli: 0,
            action_mask: 0,
        })
        .expect_err("frame 2 is >64 frames behind frame 69");
    assert!(matches!(
        window_err,
        SnapshotCodecError::MalformedStatePayload { .. }
    ));
}

/// `VE-21`: Formal 2D/3D `UiTree` semiring layout, diegetic BVH picking, WCAG/48px compliance, and `UiComplianceError` refusals.
#[test]
fn engine_ve_21() {
    use verdant_core::error::UiComplianceError;
    use verdant_core::math::Ray;
    use verdant_core::ui::UiTree;

    let world = StandardSceneState::synthesize_at_lod(7, 3).expect("LOD3 world");
    let first_id = world.scene.entities().next().map(|e| e.id);
    let tree = UiTree::canonical_studio_ui_tree(first_id);
    let solved = tree
        .solve_and_verify(1280, 900, Some(&world.scene))
        .expect("canonical studio UI tree is compliant");
    assert_eq!(solved.len(), tree.widgets.len());
    for rect in &solved {
        if tree.widgets[rect.widget_index].interactive {
            assert!(rect.width_px >= 48);
            assert!(rect.height_px >= 48);
        }
    }

    let diegetic_bvh = UiTree::build_diegetic_bvh(&solved);
    assert_eq!(diegetic_bvh.len(), 1);
    let base_pos = solved[1].diegetic_bounds.expect("diegetic bounds").center();
    let hit = diegetic_bvh.raycast(
        Ray::new(
            base_pos + Vec3::new(0.0, 0.0, 5.0),
            Vec3::new(0.0, 0.0, -1.0),
        ),
        20.0,
    );
    assert!(hit.is_some());

    // Negative refusal 1: interactive widget below 48px touch target:
    let mut small_tree = tree.clone();
    small_tree.widgets[4].min_width_px = 32;
    small_tree.widgets[4].min_height_px = 32;
    small_tree.widgets[5].min_height_px = 32;
    small_tree.widgets[2].min_height_px = 32;
    let target_err = small_tree
        .solve_and_verify(360, 200, None)
        .expect_err("32px interactive height must fail");
    assert!(matches!(
        target_err,
        UiComplianceError::SubMinimumTouchTarget { .. }
    ));

    // Negative refusal 2: low WCAG contrast ratio refused:
    let mut low_contrast = tree.clone();
    low_contrast.widgets[4].contrast_milli = 2_100;
    let contrast_err = low_contrast
        .solve_and_verify(1280, 900, None)
        .expect_err("2.1:1 text contrast must fail");
    assert!(matches!(
        contrast_err,
        UiComplianceError::InsufficientContrastRatio { .. }
    ));

    // Negative refusal 3: disconnected interactive focus node refused:
    let mut disconnected = tree;
    disconnected.widgets[8].focus_neighbors.clear();
    disconnected.widgets[7].focus_neighbors.clear();
    let focus_err = disconnected
        .solve_and_verify(1280, 900, None)
        .expect_err("unreachable focus widget must fail");
    assert!(matches!(
        focus_err,
        UiComplianceError::UnreachableFocusWidget { .. }
    ));
}

/// `VE-22`: Second-order cybernetic LIF/SSM observer loop, `coded_gemm` SLM intent director, denial-wins grants, and `CyberneticError` refusals.
#[test]
fn engine_ve_22() {
    use verdant_core::cybernetics::{
        CodedSlmDirector, ContainerGrantMatrix, ContainerOrgan, CyberneticObserverLoop,
        EngineIntentKind, PlayerObserverSample,
    };
    use verdant_core::error::CyberneticError;

    // 1. Bit-identical second-order cybernetic observer recurrence:
    let mut loop_a = CyberneticObserverLoop::new(7).expect("observer loop A");
    let mut loop_b = CyberneticObserverLoop::new(7).expect("observer loop B");
    let sample = PlayerObserverSample {
        gazed_entity_slot: 3,
        gaze_dwell_millis: 1_400,
        camera_angular_speed_milli: 120,
        locomotion_speed_milli: 800,
        cadence_focus_milli: 750,
        audio_alignment_milli: 400,
        ui_focus_slot: 0,
        frame_dt_millis: 16,
    };
    let dir_a = loop_a.step_observation(sample, 520).expect("step A");
    let dir_b = loop_b.step_observation(sample, 520).expect("step B");
    assert_eq!(dir_a, dir_b);
    assert_eq!(dir_a.attended_entity_slot, 3);
    assert_eq!(dir_a.foveated_lod_rank, 3);
    assert_eq!(dir_a.observer_gimbal_yaw_milli, -520);

    // 2. Embedded zero-decode `coded_gemm` SLM proposal routing:
    let director = CodedSlmDirector::new();
    let mut grants = ContainerGrantMatrix::default();
    assert!(grants.is_allowed(ContainerOrgan::WebAssemblyCore));
    assert!(!grants.is_allowed(ContainerOrgan::WebRtcStitchMesh));
    grants.grant(ContainerOrgan::WebRtcStitchMesh);
    assert!(grants.is_allowed(ContainerOrgan::WebRtcStitchMesh));

    let veh_prop = director
        .route_prompt(
            "drive the 4-wheel suspension vehicle",
            Some(&sample),
            &grants,
        )
        .expect("route vehicle prompt");
    assert_eq!(veh_prop.intent, EngineIntentKind::DriveSuspensionVehicle);
    assert!(veh_prop.score_milli >= 100);

    let tile_prop = director
        .route_prompt("switch to software 16x16 tile raster", None, &grants)
        .expect("route tile raster prompt");
    assert_eq!(tile_prop.intent, EngineIntentKind::SwitchTileRasterizer);

    // Negative refusal 1: denial always wins over grant (`FK-07`):
    grants.deny(ContainerOrgan::WebRtcStitchMesh);
    let deny_err = grants
        .require_organ(ContainerOrgan::WebRtcStitchMesh)
        .expect_err("denied organ must fail");
    assert_eq!(
        deny_err,
        CyberneticError::ContainerOrganDenied {
            organ_name: "webrtc-stitch-mesh"
        }
    );

    // Negative refusal 2: empty prompt rejected:
    let empty_err = director
        .route_prompt("   ", None, &grants)
        .expect_err("empty prompt must reject");
    assert!(matches!(
        empty_err,
        CyberneticError::ProposalRejected { .. }
    ));
}

/// `VE-23`: Zero-copy world-space `PackedSceneRenderBuffers` from `SceneWorld` and imported `.glb`, shared across WebGL2 and `16x16` tile rasterization.
#[test]
fn engine_ve_23() {
    use verdant_core::gltf_io::{export_scene_to_glb, import_glb_to_scene};
    use verdant_core::render::{canonical_pbr_material_library, pack_world_meshes_to_buffers};
    use verdant_core::tensor::render_triangles_tile_framebuffer;

    let world = StandardSceneState::synthesize_at_lod(7, 3).expect("LOD3 world");
    let meshes = canonical_scene_mesh_library(world.seed);
    let materials = canonical_pbr_material_library();

    let packed =
        pack_world_meshes_to_buffers(&world.scene, &meshes, &materials).expect("pack world meshes");
    assert!(packed.vertex_count() > 500);
    assert!(packed.triangle_count() > 500);
    assert_eq!(packed.positions_xyz.len(), packed.normals_xyz.len());
    assert_eq!(packed.positions_xyz.len(), packed.colors_rgb.len());
    assert_eq!(packed.tile_triangles.len(), packed.triangle_count());

    // Round-trip through binary `.glb` -> `import_glb_to_scene` -> `pack_world_meshes_to_buffers`:
    let glb_bytes = export_scene_to_glb(&world.scene, &meshes, &materials).expect("export glb");
    let imported = import_glb_to_scene(&glb_bytes).expect("import glb");
    let imported_packed =
        pack_world_meshes_to_buffers(&imported.scene, &imported.meshes, &imported.materials)
            .expect("pack imported glb scene");
    assert_eq!(imported_packed.triangle_count(), packed.triangle_count());

    // Render those exact packed triangles through the pure Wasm/CPU `16x16` software tile PBR rasterizer:
    let vp = Mat4::perspective(1.0, 1.0, 0.2, 180.0)
        * Mat4::look_at(Vec3::new(0.0, 14.0, 28.0), Vec3::ZERO, Vec3::UP);
    let fb = render_triangles_tile_framebuffer(
        64,
        64,
        &vp,
        Vec3::new(0.4, 0.8, 0.3),
        3,
        &imported_packed.tile_triangles[..64.min(imported_packed.tile_triangles.len())],
    )
    .expect("tile rasterize packed world");
    assert!(fb.rasterized_triangles > 0);

    // Negative refusal: empty mesh library rejected:
    let empty_err = pack_world_meshes_to_buffers(&world.scene, &[], &materials)
        .expect_err("empty mesh library must refuse");
    assert!(matches!(
        empty_err,
        TensorComputeError::ZeroOrMismatchedExtents { .. }
    ));
}

/// `VE-24`: `PrismIdSessionEnvelope` signed journals, `afflom/stitch` 2-of-3 `(A, B, A ⊕ B)` parity recovery, survival-entropy tracking, and `StitchCustodyError` refusals.
#[test]
fn engine_ve_24() {
    use verdant_core::error::StitchCustodyError;
    use verdant_core::network::{
        recover_bytes_from_stitch_pair, shard_bytes_to_stitch_fragments, PeerObservationRecord,
        SignedJournalBatch, SurvivalEntropyTracker,
    };
    use verdant_core::runtime::FrameInputCommand;

    // 1. Prism ID signed input journal verification:
    let cmds = vec![
        FrameInputCommand {
            frame_index: 0,
            player_slot: 0,
            move_fwd_milli: 1_000,
            move_strafe_milli: 0,
            yaw_milli: 150,
            action_mask: 1,
        },
        FrameInputCommand {
            frame_index: 1,
            player_slot: 0,
            move_fwd_milli: 800,
            move_strafe_milli: -200,
            yaw_milli: 160,
            action_mask: 0,
        },
    ];
    let origin = "https://afflom.github.io";
    let signed = SignedJournalBatch::sign_commands(
        cmds,
        "prism:citizen-alpha",
        b"ephemeral-nonextractable-device-seed",
        origin,
        1_700_000_000,
        1_700_600_000,
    )
    .expect("sign journal batch");
    signed
        .verify_admission(origin, 1_700_000_500)
        .expect("valid Prism ID journal admitted");

    // 2. Stitch 2-of-3 `(A, B, A ⊕ B)` erasure sharding & recovery across odd-length payload:
    let payload = b"UORC-VERDANT-WORLD-STREAM-ODD-LENGTH-777!";
    let [frag_a, frag_b, frag_p] =
        shard_bytes_to_stitch_fragments(payload).expect("3 Stitch shards");
    assert_eq!(
        recover_bytes_from_stitch_pair(&frag_a, &frag_b).expect("(A, B)"),
        payload
    );
    assert_eq!(
        recover_bytes_from_stitch_pair(&frag_a, &frag_p).expect("(A, Parity)"),
        payload
    );
    assert_eq!(
        recover_bytes_from_stitch_pair(&frag_b, &frag_p).expect("(B, Parity)"),
        payload
    );

    // 3. Survival-entropy peer tracker:
    let mut tracker = SurvivalEntropyTracker::default();
    for (peer_id, shard_idx) in [(1, 0), (2, 1), (3, 2)] {
        tracker
            .record_observation(
                PeerObservationRecord {
                    peer_id,
                    sequence: 1,
                    observed_at_sec: 100,
                    expires_at_sec: 200,
                    held_fragment_index: shard_idx,
                    authenticated: true,
                },
                120,
            )
            .expect("fresh peer");
    }
    assert_eq!(tracker.refresh_urgency_permille(120), 0);

    // Negative refusal 1: duplicate fragment index `(1, 1)` refused (`duplicateIndexCannotRecover`):
    let dup_err = recover_bytes_from_stitch_pair(&frag_b, &frag_b)
        .expect_err("duplicate fragment index must fail");
    assert!(matches!(
        dup_err,
        StitchCustodyError::IncompatibleFragmentPair { .. }
    ));

    // Negative refusal 2: tampered parity byte refused via SHA-256 commitment:
    let mut corrupt_p = frag_p;
    corrupt_p.payload[0] ^= 0xFF;
    let tamper_err = recover_bytes_from_stitch_pair(&frag_a, &corrupt_p)
        .expect_err("corrupted parity shard must fail");
    assert!(matches!(
        tamper_err,
        StitchCustodyError::PayloadDigestMismatch { .. }
    ));

    // Negative refusal 3: forged signature or wrong audience refused:
    let aud_err = signed
        .verify_admission("https://evil.example", 1_700_000_500)
        .expect_err("wrong origin audience must fail");
    assert!(matches!(
        aud_err,
        StitchCustodyError::InvalidJournalSignature { .. }
    ));

    // Negative refusal 4: expired or unauthenticated peer observation refused:
    let stale_err = tracker
        .record_observation(
            PeerObservationRecord {
                peer_id: 9,
                sequence: 1,
                observed_at_sec: 100,
                expires_at_sec: 110,
                held_fragment_index: 0,
                authenticated: true,
            },
            120,
        )
        .expect_err("expired observation must fail");
    assert_eq!(
        stale_err,
        StitchCustodyError::StalePeerObservation { peer_id: 9 }
    );
}

fn append_test_box_to_mesh(
    mesh: &mut verdant_core::geometry::ProceduralMesh,
    min: Vec3,
    max: Vec3,
) {
    let corners = [
        // +Z front
        (
            [min.x, min.y, max.z],
            [max.x, min.y, max.z],
            [max.x, max.y, max.z],
            [min.x, max.y, max.z],
            [0.0_f32, 0.0, 1.0],
        ),
        // -Z back
        (
            [max.x, min.y, min.z],
            [min.x, min.y, min.z],
            [min.x, max.y, min.z],
            [max.x, max.y, min.z],
            [0.0_f32, 0.0, -1.0],
        ),
        // +X right
        (
            [max.x, min.y, max.z],
            [max.x, min.y, min.z],
            [max.x, max.y, min.z],
            [max.x, max.y, max.z],
            [1.0_f32, 0.0, 0.0],
        ),
        // -X left
        (
            [min.x, min.y, min.z],
            [min.x, min.y, max.z],
            [min.x, max.y, max.z],
            [min.x, max.y, min.z],
            [-1.0_f32, 0.0, 0.0],
        ),
        // +Y top
        (
            [min.x, max.y, max.z],
            [max.x, max.y, max.z],
            [max.x, max.y, min.z],
            [min.x, max.y, min.z],
            [0.0_f32, 1.0, 0.0],
        ),
        // -Y bottom
        (
            [min.x, min.y, min.z],
            [max.x, min.y, min.z],
            [max.x, min.y, max.z],
            [min.x, min.y, max.z],
            [0.0_f32, -1.0, 0.0],
        ),
    ];

    for (p0, p1, p2, p3, nrm) in corners {
        let base = mesh.positions.len() as u32;
        for p in [p0, p1, p2, p3] {
            mesh.positions.push([p[0] as f32, p[1] as f32, p[2] as f32]);
            mesh.normals.push(nrm);
            mesh.uvs.push([0.5, 0.5]);
            mesh.colors.push([0.8, 0.8, 0.8, 1.0]);
        }
        mesh.indices
            .extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
    }
}

/// `VE-25`: Two-level entity TLAS + triangle `MeshBlas` BVH, hollow doorway traversal, acute 2-plane crease projection, and stair auto-stepping.
#[test]
fn engine_ve_25() {
    use verdant_core::ecs::{EntityRole, SceneWorld, Transform, TwoLevelBvh};
    use verdant_core::geometry::ProceduralMesh;
    use verdant_core::physics::{
        CapsuleController, CharacterMoveIntent, PhysicsWorld, PhysicsWorldConfig,
    };

    // Build a compound hollow doorway mesh at Z = 0:
    // Left pier: X in [-3.0, -1.0], Right pier: X in [1.0, 3.0], Overhead lintel: Y in [2.6, 3.6].
    // Opening in X in (-1.0, 1.0), Y in [0.0, 2.6] is completely hollow inside the mesh's AABB!
    let mut doorway = ProceduralMesh {
        name: "HollowDoorway".to_string(),
        positions: Vec::new(),
        normals: Vec::new(),
        uvs: Vec::new(),
        colors: Vec::new(),
        indices: Vec::new(),
        morph_targets: Vec::new(),
    };
    append_test_box_to_mesh(
        &mut doorway,
        Vec3::new(-3.0, 0.0, -0.4),
        Vec3::new(-1.0, 3.6, 0.4),
    );
    append_test_box_to_mesh(
        &mut doorway,
        Vec3::new(1.0, 0.0, -0.4),
        Vec3::new(3.0, 3.6, 0.4),
    );
    append_test_box_to_mesh(
        &mut doorway,
        Vec3::new(-3.0, 2.6, -0.4),
        Vec3::new(3.0, 3.6, 0.4),
    );

    // Acute V-corner at Z = -6.0 (two walls angled inward):
    let mut wall_slab = ProceduralMesh {
        name: "WallSlab".to_string(),
        positions: Vec::new(),
        normals: Vec::new(),
        uvs: Vec::new(),
        colors: Vec::new(),
        indices: Vec::new(),
        morph_targets: Vec::new(),
    };
    append_test_box_to_mesh(
        &mut wall_slab,
        Vec3::new(-3.0, 0.0, -0.25),
        Vec3::new(3.0, 3.0, 0.25),
    );

    // Two-step staircase at X = 6.0 (`0.24m` risers <= `0.35m` max step):
    let mut stairs = ProceduralMesh {
        name: "WalkableStairs".to_string(),
        positions: Vec::new(),
        normals: Vec::new(),
        uvs: Vec::new(),
        colors: Vec::new(),
        indices: Vec::new(),
        morph_targets: Vec::new(),
    };
    append_test_box_to_mesh(
        &mut stairs,
        Vec3::new(5.0, 0.0, -1.0),
        Vec3::new(7.0, 0.24, 0.5),
    );
    append_test_box_to_mesh(
        &mut stairs,
        Vec3::new(5.0, 0.0, -6.0),
        Vec3::new(7.0, 0.48, -1.0),
    );

    doorway.validate().expect("valid doorway mesh");
    wall_slab.validate().expect("valid wall slab");
    stairs.validate().expect("valid stairs mesh");
    let meshes = vec![doorway, wall_slab, stairs];

    let mut scene = SceneWorld::new();
    let e_door = scene.spawn(
        "doorway",
        EntityRole::StaticArchitecture,
        None,
        Transform::default(),
        meshes[0].bounds(),
        (1.0, "test"),
    );
    scene.attach_renderable(e_door, 0, 0);

    let e_corner_l = scene.spawn(
        "corner_left",
        EntityRole::StaticArchitecture,
        None,
        Transform {
            translation: Vec3::new(-1.1, 0.0, -6.0),
            rotation: Quat::from_axis_angle(Vec3::UP, 0.9),
            scale: Vec3::new(1.0, 1.0, 1.0),
        },
        meshes[1].bounds(),
        (1.0, "test"),
    );
    scene.attach_renderable(e_corner_l, 1, 0);

    let e_corner_r = scene.spawn(
        "corner_right",
        EntityRole::StaticArchitecture,
        None,
        Transform {
            translation: Vec3::new(1.1, 0.0, -6.0),
            rotation: Quat::from_axis_angle(Vec3::UP, -0.9),
            scale: Vec3::new(1.0, 1.0, 1.0),
        },
        meshes[1].bounds(),
        (1.0, "test"),
    );
    scene.attach_renderable(e_corner_r, 1, 0);

    let e_stairs = scene.spawn(
        "stairs",
        EntityRole::StaticArchitecture,
        None,
        Transform::default(),
        meshes[2].bounds(),
        (1.0, "test"),
    );
    scene.attach_renderable(e_stairs, 2, 0);

    let bvh = TwoLevelBvh::build_solid(&scene, &meshes).expect("TwoLevelBvh built");
    assert_eq!(bvh.instance_count(), 4);

    // 1. Walk straight through the hollow doorway opening (`X = 0`, `Z: +2.0 -> -2.0`):
    let mut cfg = PhysicsWorldConfig::default();
    cfg.terrain_relief.basin_depth_meters = 0.0;
    cfg.terrain_relief.hill_amplitude_meters = 0.0;
    cfg.terrain_relief.outer_rim_curvature = 0.0;
    cfg.platform_deck = None;
    cfg.environment.water_level_meters = -10.0;

    let mut phys = PhysicsWorld::new(cfg);
    phys.player_capsule = Some(CapsuleController::new(Vec3::new(0.0, 0.0, 2.0)));
    for _ in 0..40 {
        let rep = phys
            .step_lawful(
                1.0 / 30.0,
                CharacterMoveIntent {
                    planar_wish_dir: Vec3::new(0.0, 0.0, -1.0),
                    speed_scale: 1.0,
                    jump_requested: false,
                },
                &bvh,
            )
            .expect("lawful step through open doorway");
        assert!(rep.clearance_verified);
    }
    let pos_after_door = phys.player_capsule.as_ref().unwrap().foot_position;
    assert!(
        pos_after_door.z < -1.5,
        "capsule walked cleanly through hollow doorway opening: z={}",
        pos_after_door.z
    );

    // 2. Walk directly into solid left doorway pier (`X = -2.0`, `Z: +2.0 -> -2.0`) — must be 100% blocked!
    phys.player_capsule = Some(CapsuleController::new(Vec3::new(-2.0, 0.0, 2.0)));
    for _ in 0..40 {
        let rep = phys
            .step_lawful(
                1.0 / 30.0,
                CharacterMoveIntent {
                    planar_wish_dir: Vec3::new(0.0, 0.0, -1.0),
                    speed_scale: 1.0,
                    jump_requested: false,
                },
                &bvh,
            )
            .expect("lawful step into solid pier");
        assert!(rep.clearance_verified);
    }
    let pos_blocked = phys.player_capsule.as_ref().unwrap().foot_position;
    assert!(
        pos_blocked.z >= 0.88,
        "solid stone pier blocked capsule from passing through: z={}",
        pos_blocked.z
    );

    // 3. Drive capsule into the acute two-wall V-corner (`Z: -3.5 -> -8.0`) — zero tunneling & crease projection!
    phys.player_capsule = Some(CapsuleController::new(Vec3::new(0.05, 0.0, -3.5)));
    let mut saw_crease = false;
    for _ in 0..45 {
        let rep = phys
            .step_lawful(
                1.0 / 30.0,
                CharacterMoveIntent {
                    planar_wish_dir: Vec3::new(0.0, 0.0, -1.0),
                    speed_scale: 1.5,
                    jump_requested: false,
                },
                &bvh,
            )
            .expect("lawful step into acute V-corner");
        if rep.crease_projected {
            saw_crease = true;
        }
        assert!(
            rep.clearance_verified,
            "clearance violated in acute corner: {}",
            rep.min_clearance_meters
        );
    }
    assert!(
        saw_crease,
        "2-plane acute corner triggered crease projection"
    );
    let pos_corner = phys.player_capsule.as_ref().unwrap().foot_position;
    assert!(
        pos_corner.z > -6.5,
        "acute V-corner halted capsule without wall tunneling: z={}",
        pos_corner.z
    );

    // 4. Climb the two-step staircase (`X = 6.0`, `Z: +1.6 -> -2.0`):
    phys.player_capsule = Some(CapsuleController::new(Vec3::new(6.0, 0.0, 1.6)));
    for _ in 0..45 {
        let rep = phys
            .step_lawful(
                1.0 / 30.0,
                CharacterMoveIntent {
                    planar_wish_dir: Vec3::new(0.0, 0.0, -1.0),
                    speed_scale: 1.0,
                    jump_requested: false,
                },
                &bvh,
            )
            .expect("lawful stair climb");
        assert!(rep.clearance_verified);
    }
    let pos_stairs = phys.player_capsule.as_ref().unwrap().foot_position;
    assert!(
        pos_stairs.y >= 0.45 && pos_stairs.z < -1.1,
        "capsule auto-stepped onto upper stair tread: pos={pos_stairs:?}"
    );
}

/// `VE-26`: `LawfulCameraRig` near-frustum guard-sphere sweep preventing wall/floor peeking and `CameraRigError` refusals.
#[test]
fn engine_ve_26() {
    use verdant_core::camera::{CameraMode, CameraRigConfig, LawfulCameraRig};
    use verdant_core::ecs::{EntityRole, SceneWorld, Transform, TwoLevelBvh};
    use verdant_core::error::CameraRigError;
    use verdant_core::geometry::ProceduralMesh;

    // Build an enclosed room box (`X in [-3.0, 3.0], Z in [-3.0, 3.0], Y in [0.0, 4.0]`) from 4 solid walls + ceiling:
    let mut room = ProceduralMesh {
        name: "EnclosedWalledRoom".to_string(),
        positions: Vec::new(),
        normals: Vec::new(),
        uvs: Vec::new(),
        colors: Vec::new(),
        indices: Vec::new(),
        morph_targets: Vec::new(),
    };
    append_test_box_to_mesh(
        &mut room,
        Vec3::new(-3.2, 0.0, 2.8),
        Vec3::new(3.2, 4.0, 3.2),
    ); // +Z wall (`z = 2.8` inner face)
    append_test_box_to_mesh(
        &mut room,
        Vec3::new(-3.2, 0.0, -3.2),
        Vec3::new(3.2, 4.0, -2.8),
    ); // -Z wall
    append_test_box_to_mesh(
        &mut room,
        Vec3::new(2.8, 0.0, -3.2),
        Vec3::new(3.2, 4.0, 3.2),
    ); // +X wall
    append_test_box_to_mesh(
        &mut room,
        Vec3::new(-3.2, 0.0, -3.2),
        Vec3::new(-2.8, 4.0, 3.2),
    ); // -X wall
    append_test_box_to_mesh(
        &mut room,
        Vec3::new(-3.2, 3.8, -3.2),
        Vec3::new(3.2, 4.2, 3.2),
    ); // ceiling (`y = 3.8` inner face)
    room.validate().expect("valid room mesh");

    let meshes = vec![room];
    let mut scene = SceneWorld::new();
    let e_room = scene.spawn(
        "room",
        EntityRole::StaticArchitecture,
        None,
        Transform::default(),
        meshes[0].bounds(),
        (1.0, "test"),
    );
    scene.attach_renderable(e_room, 0, 0);
    let bvh = TwoLevelBvh::build_solid(&scene, &meshes).expect("room TwoLevelBvh");

    let cfg = CameraRigConfig::default();
    let mut rig = LawfulCameraRig::new(cfg, CameraMode::ThirdPersonSpringArm, 0.0, 0.35, 9.0)
        .expect("valid camera rig");

    // Anchor inside the room at `(0, 1.6, 0)` while requesting a 9m orbit (well outside the 2.8m walls!):
    let anchor = Vec3::new(0.0, 1.6, 0.0);
    for step in 0..36 {
        rig.yaw_radians = (step as f64) * (std::f64::consts::TAU / 36.0);
        rig.pitch_radians = if step % 2 == 0 { 0.65 } else { -0.45 };
        let frame = rig
            .solve_frame(1.0 / 60.0, anchor, &bvh, |_, _| 0.0)
            .expect("lawful camera frame");
        assert!(
            frame.near_sphere_guarded,
            "Lean near-sphere guard must hold"
        );
        assert!(
            frame.obstructed,
            "9m spring arm inside 2.8m room must detect wall"
        );
        let max_allowed = 2.8 - frame.guard_radius_meters + 1e-3;
        assert!(
            frame.eye.x.abs() <= max_allowed && frame.eye.z.abs() <= max_allowed,
            "camera eye never peeks through room walls: eye={:?}, max_allowed={max_allowed}",
            frame.eye
        );
        assert!(
            frame.eye.y >= frame.guard_radius_meters - 1e-3
                && frame.eye.y <= 3.8 - frame.guard_radius_meters + 1e-3,
            "camera eye stays strictly between floor and ceiling: y={}",
            frame.eye.y
        );
    }

    // First-person eye mode pressed right up against the `+X` inner wall (`x = 2.79`):
    rig.mode = CameraMode::FirstPersonEye;
    let fp_frame = rig
        .solve_frame(1.0 / 60.0, Vec3::new(2.79, 1.6, 0.0), &bvh, |_, _| 0.0)
        .expect("first-person wall-guarded frame");
    assert!(fp_frame.near_sphere_guarded);
    assert!(
        fp_frame.eye.x <= 2.8 - fp_frame.guard_radius_meters + 1e-3,
        "first-person eye pushed safely back from wall by guard radius: x={}",
        fp_frame.eye.x
    );

    // Negative refusals (`CameraRigError`):
    let mut bad_cfg = cfg;
    bad_cfg.z_near_meters = 0.0;
    assert!(matches!(
        LawfulCameraRig::new(bad_cfg, CameraMode::FirstPersonEye, 0.0, 0.0, 0.0),
        Err(CameraRigError::InvalidOpticalFrustum { .. })
    ));

    let mut inverted_cfg = cfg;
    inverted_cfg.z_far_meters = inverted_cfg.z_near_meters;
    assert!(matches!(
        LawfulCameraRig::new(inverted_cfg, CameraMode::FirstPersonEye, 0.0, 0.0, 0.0),
        Err(CameraRigError::InvalidOpticalFrustum { .. })
    ));

    assert!(matches!(
        LawfulCameraRig::new(cfg, CameraMode::ThirdPersonSpringArm, 0.0, 0.0, 9999.0),
        Err(CameraRigError::InvalidSpringArmDistance { .. })
    ));
}

/// `VE-27`: High-definition `hd_world.rs` Citadel Hall, Parabolic Arch Bridge, sculpted terrain (`LOD0..LOD4`), Khronos `.glb` oracle, and walkable arched portal traversal.
#[test]
fn engine_ve_27() {
    use verdant_core::ecs::TwoLevelBvh;
    use verdant_core::gltf_io::{export_scene_to_glb, validate_glb_bytes};
    use verdant_core::hd_world::{hd_terrain_height, synthesize_hd_citadel_sector};
    use verdant_core::physics::{
        CapsuleController, CharacterMoveIntent, PhysicsWorld, PhysicsWorldConfig,
    };
    use verdant_core::render::canonical_pbr_material_library;
    use verdant_core::streaming::{PhysicalEnvironment, TerrainReliefParams};

    let relief = TerrainReliefParams::standard_basin_valley();
    let env = PhysicalEnvironment::temperate_terrestrial();

    let mut prev_tris = 0usize;
    for rank in 0..=4 {
        let sector = synthesize_hd_citadel_sector(7, &relief, env.water_level_meters, rank);
        for mesh in &sector.meshes {
            mesh.validate()
                .unwrap_or_else(|e| panic!("LOD{rank} mesh `{}` invalid: {e}", mesh.name));
        }
        let bvh = TwoLevelBvh::build_solid(&sector.scene, &sector.meshes)
            .expect("solid TwoLevelBvh for HD sector");
        let total_tris = bvh.total_surface_triangles();
        assert!(
            total_tris > prev_tris,
            "LOD{rank} surface triangles ({total_tris}) must exceed previous ({prev_tris})"
        );
        prev_tris = total_tris;
        if rank == 4 {
            assert!(
                total_tris >= 45_000,
                "LOD4 ultra-high-definition sector must exceed 45k solid triangles, got {total_tris}"
            );
        }
    }

    // Export HD Citadel + Arch Bridge sector to binary glTF 2.0 and validate with Khronos `gltf`:
    let sector_lod2 = synthesize_hd_citadel_sector(7, &relief, env.water_level_meters, 2);
    let materials = canonical_pbr_material_library();
    let glb = export_scene_to_glb(&sector_lod2.scene, &sector_lod2.meshes, &materials)
        .expect("export HD Citadel sector GLB");
    let summary = validate_glb_bytes(&glb).expect("Khronos glTF 2.0 validation of HD Citadel");
    assert!(summary.mesh_count >= 4);

    // Traverse South Citadel staircase (`Z = +9.2 -> 0.0`, `X = 0.0`) right through the arched portal into the vaulted hall!
    let bvh = TwoLevelBvh::build_solid(&sector_lod2.scene, &sector_lod2.meshes)
        .expect("LOD2 Citadel TwoLevelBvh");
    let mut cfg = PhysicsWorldConfig {
        terrain_seed: 7,
        environment: env,
        terrain_relief: relief,
        platform_deck: None,
        ..Default::default()
    };
    cfg.environment.water_level_meters = -10.0;
    let mut phys = PhysicsWorld::new(cfg);
    let start_y = hd_terrain_height(7, &relief, 0.0, 9.2);
    phys.player_capsule = Some(CapsuleController::new(Vec3::new(0.0, start_y, 9.2)));

    for _ in 0..85 {
        let rep = phys
            .step_lawful(
                1.0 / 30.0,
                CharacterMoveIntent {
                    planar_wish_dir: Vec3::new(0.0, 0.0, -1.0),
                    speed_scale: 1.0,
                    jump_requested: false,
                },
                &bvh,
            )
            .expect("lawful climb up Citadel stairs and through arched portal");
        assert!(rep.clearance_verified);
    }
    let inside_pos = phys.player_capsule.as_ref().unwrap().foot_position;
    let podium_y = hd_terrain_height(7, &relief, 0.0, 0.0) + 0.60;
    assert!(
        inside_pos.z < 2.0 && (inside_pos.y - podium_y).abs() < 0.12,
        "player climbed Citadel stairs and entered vaulted hall interior: pos={inside_pos:?}, expected_y={podium_y}"
    );
}

/// `VE-28`: 22-joint `ArticulatedCharacter`, normalized LBS skinning, 3 morph targets, analytical Two-Bone Foot IK on stairs, 1st-person head masking, and `.glb` binding.
#[test]
fn engine_ve_28() {
    use verdant_core::character::{ArticulatedCharacter, CharacterArchetype};
    use verdant_core::ecs::{EntityRole, SceneWorld, Transform, TwoLevelBvh};
    use verdant_core::error::AnimationError;
    use verdant_core::geometry::ProceduralMesh;
    use verdant_core::render::PackedSceneRenderBuffers;

    let mut explorer =
        ArticulatedCharacter::new_humanoid("Explorer", CharacterArchetype::Explorer, Vec3::ZERO, 4)
            .expect("22-joint Explorer at LOD4");
    assert_eq!(explorer.skeleton.joints.len(), 22);
    assert_eq!(explorer.bind_mesh.morph_targets.len(), 3);
    assert!(
        explorer.bind_mesh.triangle_count() >= 3_500,
        "LOD4 humanoid must have >= 3,500 smooth triangles, got {}",
        explorer.bind_mesh.triangle_count()
    );
    explorer
        .bind_mesh
        .validate()
        .expect("LOD4 humanoid bind mesh valid");

    // Place left foot (`x < 0`) over a `0.26m` stair block while right foot (`x > 0`) stands at `y = 0.0`:
    let mut step_block = ProceduralMesh {
        name: "AsymmetricStep".to_string(),
        positions: Vec::new(),
        normals: Vec::new(),
        uvs: Vec::new(),
        colors: Vec::new(),
        indices: Vec::new(),
        morph_targets: Vec::new(),
    };
    append_test_box_to_mesh(
        &mut step_block,
        Vec3::new(-1.0, 0.0, -1.0),
        Vec3::new(-0.02, 0.26, 1.0),
    );
    let meshes = vec![step_block];
    let mut scene = SceneWorld::new();
    let e_step = scene.spawn(
        "left_step",
        EntityRole::StaticArchitecture,
        None,
        Transform::default(),
        meshes[0].bounds(),
        (1.0, "test"),
    );
    scene.attach_renderable(e_step, 0, 0);
    let bvh = TwoLevelBvh::build_solid(&scene, &meshes).expect("step TwoLevelBvh");

    let mut ik = explorer.last_ik;
    for frame in 0..10 {
        ik = explorer
            .step_and_solve_ik(
                1.0 / 30.0,
                Vec3::new(0.0, 0.26, 0.0),
                Vec3::ZERO,
                true,
                (frame as f64) * (1.0 / 30.0),
                Some(Vec3::new(2.0, 1.8, 3.0)),
                &bvh,
                |_, _| 0.0,
            )
            .expect("step_and_solve_ik");
    }
    assert!(ik.left_planted);
    assert!(
        ik.pelvis_sink_meters < -0.05,
        "pelvis sank smoothly toward lower right foot: {}",
        ik.pelvis_sink_meters
    );
    assert!(
        ik.left_knee_bend_radians > ik.right_knee_bend_radians + 0.15,
        "raised left leg bent more than extended right leg: L={}, R={}",
        ik.left_knee_bend_radians,
        ik.right_knee_bend_radians
    );

    let skinned = explorer
        .evaluate_skinned_mesh()
        .expect("uor-matmul LBS skinned humanoid");
    skinned
        .validate()
        .expect("skinned humanoid preserves unit normals and topology");

    let mut tp_buf = PackedSceneRenderBuffers::default();
    let mut fp_buf = PackedSceneRenderBuffers::default();
    let tp_tris = explorer
        .append_skinned_to_buffers(&mut tp_buf, false)
        .expect("3rd-person character buffers");
    let fp_tris = explorer
        .append_skinned_to_buffers(&mut fp_buf, true)
        .expect("1st-person head-masked character buffers");
    assert!(
        fp_tris < tp_tris && fp_tris > 0,
        "1st-person head masking hides helmet/visor triangles: fp={fp_tris} < tp={tp_tris}"
    );

    // Bind external mesh + negative refusal for empty mesh:
    let bound = ArticulatedCharacter::bind_external_mesh(
        "ImportedHero",
        CharacterArchetype::Archivist,
        Vec3::ZERO,
        meshes[0].clone(),
    )
    .expect("bound external GLB mesh");
    assert_eq!(bound.skeleton.joints.len(), 22);

    let empty = ProceduralMesh {
        name: "Empty".to_string(),
        positions: Vec::new(),
        normals: Vec::new(),
        uvs: Vec::new(),
        colors: Vec::new(),
        indices: Vec::new(),
        morph_targets: Vec::new(),
    };
    assert!(matches!(
        ArticulatedCharacter::bind_external_mesh(
            "BadImport",
            CharacterArchetype::Explorer,
            Vec3::ZERO,
            empty
        ),
        Err(AnimationError::InvalidClipChannel { .. })
    ));
}

/// `VE-29`: Floating-origin `UniversePosition`, camera-cell-relative `f32` precision at astronomical
/// sector indices, momentum-preserving `PortalGraph` traversal, and CLI `pack-universe`.
#[test]
fn engine_ve_29() {
    use verdant_cli::run_cli;
    use verdant_core::math::{Quat, Vec3};
    use verdant_core::universe::{
        canonical_cell_span_meters, PortalGraph, SpatialPortal, UniverseCellCoord, UniverseManager,
        UniversePosition,
    };

    assert!((canonical_cell_span_meters() - 128.0).abs() < 1e-9);

    // Astronomical sector coordinate (10^9 sectors * 128m = 1.28e11 meters, ~0.85 AU from origin!):
    let far_cell = UniverseCellCoord::new(1_000_000_000, -42_000, 999_999_999);
    let cam_pos = UniversePosition::new_canonical(far_cell, Vec3::new(12.3456, 3.25, -18.75))
        .expect("canonical astronomical camera pos");
    assert!(cam_pos.bounds_verified);

    // Translate +90m across the +64m cell boundary -> wraps into cx + 1 with exact local offset:
    let stepped = cam_pos
        .translate(Vec3::new(90.0, 0.0, 0.0))
        .expect("cell wrap");
    assert!(stepped.bounds_verified);
    assert_eq!(stepped.cell.cx, far_cell.cx + 1);
    assert!(stepped.local.x >= -64.0 && stepped.local.x < 64.0);
    let disp = stepped.displacement_from(&cam_pos);
    assert!((disp.x - 90.0).abs() < 1e-9);

    // Verify sub-millimeter f32 precision in camera-cell space even at 1.28e11 meters:
    let sub_mm_target = cam_pos
        .translate(Vec3::new(0.1234, -0.0567, 0.8901))
        .expect("nearby entity at astronomical cell");
    let m =
        sub_mm_target.camera_relative_matrix(&cam_pos, Quat::IDENTITY, Vec3::new(1.0, 1.0, 1.0));
    let rel_f32 = [
        m.cols[3][0] as f32,
        m.cols[3][1] as f32,
        m.cols[3][2] as f32,
    ];
    let exact_rel = sub_mm_target.relative_to_camera_cell(&cam_pos);
    assert!((f64::from(rel_f32[0]) - exact_rel.x).abs() < 1e-4);
    assert!((f64::from(rel_f32[1]) - exact_rel.y).abs() < 1e-4);
    assert!((f64::from(rel_f32[2]) - exact_rel.z).abs() < 1e-4);

    // Momentum-preserving portal traversal with 90-degree yaw rotation:
    let portal = SpatialPortal {
        portal_id: "quarter_turn_gate".to_string(),
        source_cell: UniverseCellCoord::ORIGIN,
        source_local: Vec3::new(0.0, 0.0, -32.0),
        trigger_radius_meters: 2.0,
        target_cell: UniverseCellCoord::new(0, 0, -1),
        target_arrival_local: Vec3::new(0.0, 0.0, 20.0),
        yaw_delta_radians: std::f64::consts::FRAC_PI_2,
        federated_uri: None,
    };
    let graph = PortalGraph::new(vec![portal]).expect("valid portal graph");
    let at_gate =
        UniversePosition::new_canonical(UniverseCellCoord::ORIGIN, Vec3::new(0.25, 0.0, -31.5))
            .expect("at gate");
    let (arrived, rot_vel, _) = graph
        .try_traverse(&at_gate, Vec3::new(0.0, 0.0, -5.0))
        .expect("traverse ok")
        .expect("portal triggered");
    assert_eq!(arrived.cell, UniverseCellCoord::new(0, 0, -1));
    assert!(
        (rot_vel.length() - 5.0).abs() < 1e-9,
        "momentum magnitude preserved"
    );
    assert!(
        (rot_vel.x - (-5.0)).abs() < 1e-6,
        "+90 deg Y-rotation maps -Z velocity to -X"
    );

    let universe = UniverseManager::canonical_citadel_universe(7).expect("canonical universe");
    assert_eq!(universe.sectors.len(), 3);
    assert_eq!(universe.portal_graph.portals.len(), 2);

    let tmp_dir =
        std::env::temp_dir().join(format!("verdant_universe_ve29_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&tmp_dir);
    let out = run_cli(&[
        "pack-universe".to_string(),
        tmp_dir.to_string_lossy().into_owned(),
        "7".to_string(),
    ])
    .expect("verdant pack-universe");
    assert!(out.contains("UNIVERSE PACKED OK: sectors=3 portals=2"));
    assert!(tmp_dir.join("universe-manifest.json").exists());
    assert!(tmp_dir.join("sector_0_0_0_shard_0.stch").exists());
    let _ = std::fs::remove_dir_all(&tmp_dir);
}

/// `VE-30`: `StoryAutomaton` + `ConservationLedger` tropical reachability & exact resource conservation,
/// SLM-grounded narrative routing, Wasm lawful bridge integration, CLI `verify-story`, and `NarrativeLawError` refusals.
#[test]
fn engine_ve_30() {
    use verdant_cli::run_cli;
    use verdant_core::cybernetics::CodedSlmDirector;
    use verdant_core::error::NarrativeLawError;
    use verdant_core::math::Vec3;
    use verdant_core::narrative::{
        canonical_citadel_chronicle_automaton, ConservationLedger, StoryAutomaton, StoryEdge,
        StoryNode,
    };
    use verdant_core::universe::{PortalGraph, SpatialPortal, UniverseCellCoord, UniversePosition};
    use verdant_wasm::WasmRuntimeBridge;

    let (mut chronicle, report) =
        canonical_citadel_chronicle_automaton(7).expect("canonical Citadel Chronicle");
    assert_eq!(chronicle.nodes.len(), 4);
    assert!(report.reachable_live_states >= 4);
    let initial_supply = chronicle.ledger.total_supply();
    assert!(initial_supply > 0);

    // Negative refusal 1 (`GuardRefused`): player standing far outside interaction radius:
    let far_err = chronicle
        .try_advance(
            &chronicle.available_edges()[0].action_id.clone(),
            Vec3::new(50.0, 0.0, 50.0),
        )
        .expect_err("out-of-range NPC interaction must refuse");
    assert!(matches!(far_err, NarrativeLawError::GuardRefused { .. }));

    // Step 1 via `CodedSlmDirector` (`coded_gemm` grounded proposal at NPC anchor):
    let director = CodedSlmDirector::new();
    let e0_anchor = chronicle.available_edges()[0].world_anchor;
    let step1 = chronicle
        .advance_from_slm_prompt(
            "speak with the archivist at the observer core",
            e0_anchor,
            &director,
        )
        .expect("SLM-guided Chronicle step 1");
    assert_eq!(chronicle.current_state, step1.to_state);
    assert_eq!(chronicle.ledger.total_supply(), initial_supply);

    // Advance remaining Chronicle steps via spatial `interact_nearest` and verify invariant supply:
    while !chronicle.active_node().is_milestone {
        let next_anchor = chronicle.available_edges()[0].world_anchor;
        let edge = chronicle
            .interact_nearest(next_anchor)
            .expect("lawful interaction")
            .expect("edge in range");
        assert!(!edge.dialogue_line.is_empty());
        assert_eq!(
            chronicle.ledger.total_supply(),
            initial_supply,
            "pure transfers strictly conserve world item supply"
        );
    }
    chronicle.audit_live().expect("live audit at milestone");

    // CLI `verify-story` verification:
    let cli_out = run_cli(&["verify-story".to_string(), "7".to_string()]).expect("verify-story");
    assert!(cli_out.contains("STORY AUTOMATON OK: states=4"));

    // End-to-end `WasmRuntimeBridge` lawful step + camera guard + HD LOD4 verification:
    let mut bridge = WasmRuntimeBridge::new(7).expect("lawful WasmRuntimeBridge");
    bridge
        .step_frame_lawful(1.0 / 60.0, 1.0, 0.0, std::f64::consts::PI, 1.0, false)
        .expect("lawful step");
    assert!(bridge.last_camera_frame.near_sphere_guarded);
    bridge
        .set_lod_rank(4)
        .expect("upgrade WasmRuntimeBridge to LOD4 HD");
    assert_eq!(bridge.lod_rank, 4);
    assert!(bridge.scene_buffers.triangle_count() > 45_000);

    // Negative refusal 2 (`UnreachableOrDeadlockedState`): planted trapped sink node `state 1` with no path to milestone `state 2`:
    let bad_nodes = vec![
        StoryNode {
            index: 0,
            slug: "start".to_string(),
            title: "Start".to_string(),
            objective: "Begin".to_string(),
            is_milestone: false,
        },
        StoryNode {
            index: 1,
            slug: "deadlock_trap".to_string(),
            title: "Trap".to_string(),
            objective: "Stuck".to_string(),
            is_milestone: false,
        },
        StoryNode {
            index: 2,
            slug: "goal".to_string(),
            title: "Goal".to_string(),
            objective: "Done".to_string(),
            is_milestone: true,
        },
    ];
    let bad_edges = vec![StoryEdge {
        from_state: 0,
        to_state: 1,
        action_id: "enter_trap".to_string(),
        actor_name: "Gate".to_string(),
        dialogue_line: "Trapped".to_string(),
        cost_milli: 100,
        world_anchor: Vec3::ZERO,
        interaction_radius_meters: 3.0,
        from_account: None,
        to_account: None,
        transfer_amount: 0,
        mint_amount: 0,
        burn_amount: 0,
    }];
    let deadlock_err = StoryAutomaton::new(bad_nodes, bad_edges, 0, ConservationLedger::new())
        .expect_err("deadlocked story branch must be refused at construction");
    assert!(matches!(
        deadlock_err,
        NarrativeLawError::UnreachableOrDeadlockedState { .. }
    ));

    // Negative refusal 3 (`ConservationViolated`): debiting more tokens than account holds:
    let mut ledger = ConservationLedger::new();
    ledger.seed_account("vault", 2).expect("seed vault");
    let underflow_err = ledger
        .apply_conserved_delta(Some("vault"), Some("player"), 5, 0, 0)
        .expect_err("debiting 5 from balance 2 must refuse");
    assert!(matches!(
        underflow_err,
        NarrativeLawError::ConservationViolated { .. }
    ));

    // Negative refusal 4 (`InvalidPortalOrUniverseCell`): non-finite coordinate and zero-radius portal:
    assert!(matches!(
        UniversePosition::new_canonical(UniverseCellCoord::ORIGIN, Vec3::new(f64::NAN, 0.0, 0.0)),
        Err(NarrativeLawError::InvalidPortalOrUniverseCell { .. })
    ));
    assert!(matches!(
        PortalGraph::new(vec![SpatialPortal {
            portal_id: "bad_zero_radius".to_string(),
            source_cell: UniverseCellCoord::ORIGIN,
            source_local: Vec3::ZERO,
            trigger_radius_meters: 0.0,
            target_cell: UniverseCellCoord::new(1, 0, 0),
            target_arrival_local: Vec3::ZERO,
            yaw_delta_radians: 0.0,
            federated_uri: None,
        }]),
        Err(NarrativeLawError::InvalidPortalOrUniverseCell { .. })
    ));
}

/// `VE-31`: Stratum-0 `OntologicalRuntimeGovernor`, 8-stratum `W5H` certificates, `HostEnvelope`/`WitnessEnvelope` telos loss, non-coercive `LOD0..LOD4` descent, and full Prism closure.
#[test]
fn engine_ve_31() {
    use verdant_core::ecs::TwoLevelBvh;
    use verdant_core::hd_world::synthesize_hd_citadel_sector;
    use verdant_core::math::verify_constructive_math_with_prism_kernel;
    use verdant_core::runtime::{BrowserCapabilities, OntologicalRuntimeGovernor};
    use verdant_core::streaming::TerrainReliefParams;
    use verdant_core::tensor::prism_tensor_substrate_certified;
    use verdant_tensor::prism_coded_tiers_certified;

    let cfg = EngineConfig::from_refresh_rate(1920, 1080, 60, 60, 4).expect("60Hz config");
    let caps = BrowserCapabilities {
        wasm_simd_available: true,
        webgl2_available: true,
        hardware_concurrency: 4,
        shared_array_buffer: true,
        audio_worklet: true,
        offscreen_canvas: true,
    };
    let governor = OntologicalRuntimeGovernor::new(cfg, caps);

    // 1. Constructive W5H certificates across all 8 causal strata (`0..=7`):
    let certs = governor.certify_all_eight_strata();
    assert_eq!(certs.len(), 8);
    let primes: Vec<u64> = certs.iter().map(|c| c.intrinsic_prime).collect();
    assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19]);
    for (idx, cert) in certs.iter().enumerate() {
        assert_eq!(cert.stratum_rank, idx as u64);
        assert_eq!(cert.triadic_agency_product, 30);
        assert!(
            cert.complete_verified,
            "stratum {idx} W5HCertificate must verify"
        );
    }

    // 2. Healthy frame telos evaluation & non-coercive LOD0..LOD4 ascent/descent:
    let healthy = governor.evaluate_frame_telos(8_200, true, 960, 840, 0, 0, 0);
    assert_eq!(healthy.total_telos_loss, 0);
    assert_eq!(healthy.host_ceiling_excess, 0);
    assert!(healthy.triadic_connection_active);
    assert!(healthy.all_eight_strata_certified);
    assert_eq!(governor.recommend_non_coercive_lod(3, 5_000, 0), 4);

    // Over-budget frame (`20,000 us > 16,666 us`) steps down when unpinned, or preserves pin when coercive_lock_count > 0:
    let overloaded = governor.evaluate_frame_telos(20_000, true, 900, 800, 0, 0, 0);
    assert_eq!(overloaded.host_ceiling_excess, 20_000 - 16_666);
    assert_eq!(overloaded.total_telos_loss, 20_000 - 16_666);
    assert_eq!(governor.recommend_non_coercive_lod(3, 20_000, 0), 2);
    assert_eq!(governor.recommend_non_coercive_lod(3, 20_000, 1), 3);

    // Non-zero epistemic opacity debt disables triadic connection:
    let opaque = governor.evaluate_frame_telos(8_200, true, 960, 840, 2, 0, 0);
    assert!(!opaque.triadic_connection_active);
    assert_eq!(opaque.epistemic_opacity_debt, 2);

    // 3. Cross-verify newly bound domain modules (`math`, `ecs`, `tensor`, `coded`, `hd_world`):
    assert!(verify_constructive_math_with_prism_kernel());
    assert!(prism_tensor_substrate_certified());
    assert!(prism_coded_tiers_certified());
    let sector =
        synthesize_hd_citadel_sector(7, &TerrainReliefParams::standard_basin_valley(), -1.35, 1);
    let bvh = TwoLevelBvh::build_solid(&sector.scene, &sector.meshes).expect("TwoLevelBvh");
    assert!(bvh.prism_bvh_invariants_valid());

    // 4. Verify all newly added Core-Wasm queries in `prism_kernel::dispatch`:
    for (query, expected) in [
        (
            &b"summary:hd"[..],
            "conformant:lods=5,kernels=15,subsystems=27,lossless=2.0,zero-constants=true",
        ),
        (
            &b"lod:4"[..],
            "conformant:lod=Lod4UltraHighDef,res=16/8,work=80,lossless=true",
        ),
        (
            &b"scene:lod:4"[..],
            "conformant:rank=4,res=16/8,density=200/100,shadows=1,ibl=1,lossless=1,sky=20/34/44,fog=34/44/48",
        ),
        (
            &b"scene:telos"[..],
            "conformant:w5h_strata=8,telos_loss=0,triadic_active=true,walkable_citadel=true",
        ),
    ] {
        let out = verdant_core::prism_kernel::dispatch(query.to_vec()).expect("dispatch");
        assert_eq!(std::str::from_utf8(&out).unwrap(), expected);
    }
}

/// `VE-32`: Verdant Studio themes, six-tab game-creation workbench, seven executable stakeholder journeys, four starter archetypes, and six live WebAssembly documentation embeds satisfy W3C DTCG token purity, Core-Wasm verification, and zero-error C-ABI export parity
#[test]
fn engine_ve_32() {
    use std::collections::BTreeSet;
    use std::path::Path;

    fn read_leb128_u32(bytes: &[u8], pos: &mut usize) -> u32 {
        let mut res = 0_u32;
        let mut shift = 0_u32;
        loop {
            let b = bytes[*pos];
            *pos += 1;
            res |= u32::from(b & 0x7f) << shift;
            if (b & 0x80) == 0 {
                return res;
            }
            shift += 7;
        }
    }

    fn parse_wasm_exports(bytes: &[u8]) -> BTreeSet<String> {
        assert_eq!(&bytes[0..4], b"\0asm", "valid Wasm magic header");
        let mut pos = 8_usize;
        let mut exports = BTreeSet::new();
        while pos < bytes.len() {
            let sec_id = bytes[pos];
            pos += 1;
            let sec_len = read_leb128_u32(bytes, &mut pos) as usize;
            let sec_end = pos + sec_len;
            if sec_id == 7 {
                let count = read_leb128_u32(bytes, &mut pos);
                for _ in 0..count {
                    let name_len = read_leb128_u32(bytes, &mut pos) as usize;
                    let name = std::str::from_utf8(&bytes[pos..pos + name_len])
                        .expect("utf8 Wasm export")
                        .to_string();
                    pos += name_len;
                    let _kind = bytes[pos];
                    pos += 1;
                    let _idx = read_leb128_u32(bytes, &mut pos);
                    exports.insert(name);
                }
            }
            pos = sec_end;
        }
        exports
    }

    fn collect_prefixed_symbols(js: &str, receiver: &str) -> BTreeSet<String> {
        let needle = format!("{receiver}.verdant_wasm_");
        let mut found = BTreeSet::new();
        let mut offset = 0_usize;
        while let Some(rel) = js[offset..].find(&needle) {
            let abs = offset + rel;
            let prev_is_ident = abs > 0
                && (js.as_bytes()[abs - 1].is_ascii_alphanumeric()
                    || js.as_bytes()[abs - 1] == b'_');
            let sym_start = abs + receiver.len() + 1;
            let after = &js[sym_start..];
            let end = after
                .find(|c: char| !c.is_ascii_alphanumeric() && c != '_')
                .unwrap_or(after.len());
            if !prev_is_ident {
                found.insert(after[..end].to_string());
            }
            offset = sym_start + end;
        }
        found
    }

    fn contains_hex_color(src: &str) -> Option<String> {
        let bytes = src.as_bytes();
        for i in 0..bytes.len() {
            if bytes[i] == b'#' {
                // Skip HTML ID selectors like `#ve-canvas` (`ve-` is not all hex digits because `v` is not hex):
                let tail = &src[i + 1..];
                let hex_len = tail.bytes().take_while(|c| c.is_ascii_hexdigit()).count();
                let next_non_id = tail
                    .as_bytes()
                    .get(hex_len)
                    .copied()
                    .is_none_or(|c| !c.is_ascii_alphanumeric() && c != b'-' && c != b'_');
                if matches!(hex_len, 3 | 4 | 6 | 8) && next_non_id {
                    return Some(src[i..=i + hex_len].to_string());
                }
            }
        }
        None
    }

    // 1. Core-Wasm `scene:studio` & constructive DevRel predicates (positive + negative refusals):
    let verdict = verdant_core::prism_kernel::dispatch(b"scene:studio".to_vec())
        .expect("dispatch scene:studio");
    assert_eq!(
        std::str::from_utf8(&verdict).unwrap(),
        "conformant:themes=3,tabs=6,journeys=7,archetypes=4,doc_embeds=6,controls=13"
    );
    assert_eq!(verdant_core::prism_kernel::studioSurfaceDirect(), Ok(true));
    assert!(verdant_core::prism_kernel::studioThemeTokenPure(true, true));
    assert!(!verdant_core::prism_kernel::studioThemeTokenPure(
        false, true
    ));
    assert_eq!(
        verdant_core::prism_kernel::starterArchetypeValid(4, 1, true),
        Ok(true)
    );
    assert_eq!(
        verdant_core::prism_kernel::starterArchetypeValid(5, 0, true),
        Ok(false)
    );
    assert!(verdant_core::prism_kernel::liveDocEmbedAdmissible(
        5, 6, true, true
    ));
    assert!(!verdant_core::prism_kernel::liveDocEmbedAdmissible(
        6, 6, true, true
    ));

    // 2. Verify live C-ABI `verdant_wasm_csm_split_meters` & `verdant_wasm_scene_outliner_aux`:
    assert_eq!(verdant_wasm::verdant_wasm_init(7), 1);
    let c0 = verdant_wasm::verdant_wasm_csm_split_meters(0.25, 140.0, 0);
    let c3 = verdant_wasm::verdant_wasm_csm_split_meters(0.25, 140.0, 3);
    assert!(c0 > 0.25 && c3 > c0 && (c3 - 140.0).abs() < 1e-4);
    let out_len = verdant_wasm::verdant_wasm_scene_outliner_aux(8_200);
    assert!(out_len > 64, "scene_outliner_aux must stage entity table");

    // 3. Audit Studio token purity, 13 DevRel controls, 6 Workbench tabs, 7 Journeys, 4 Archetypes, & C-ABI parity:
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let card_html = std::fs::read_to_string(repo_root.join("site/card.html")).expect("card.html");
    let card_css = std::fs::read_to_string(repo_root.join("site/card.css")).expect("card.css");
    let studio_mjs =
        std::fs::read_to_string(repo_root.join("site/studio.mjs")).expect("studio.mjs");
    let stitch_mjs =
        std::fs::read_to_string(repo_root.join("site/stitch-mesh.mjs")).expect("stitch-mesh.mjs");
    let worker_mjs = std::fs::read_to_string(repo_root.join("site/engine-worker.mjs"))
        .expect("engine-worker.mjs");
    let live_embed_mjs = std::fs::read_to_string(repo_root.join("site/docs/live-embed.mjs"))
        .expect("site/docs/live-embed.mjs");

    assert_eq!(
        contains_hex_color(&card_css),
        None,
        "site/card.css must use 100% W3C DTCG variables with zero raw hex colors"
    );
    assert_eq!(
        contains_hex_color(&studio_mjs),
        None,
        "site/studio.mjs must use 100% W3C DTCG variables with zero raw hex colors"
    );
    assert_eq!(
        contains_hex_color(&stitch_mjs),
        None,
        "site/stitch-mesh.mjs must use 100% W3C DTCG variables with zero raw hex colors"
    );
    assert!(
        !live_embed_mjs.contains(".catch(() => {})"),
        "site/docs/live-embed.mjs must never silently swallow WebAssembly errors"
    );

    for control_id in [
        "ve-lod-btn",
        "ve-sim-btn",
        "ve-cam-btn",
        "ve-audio-btn",
        "ve-glb-btn",
        "ve-snap-btn",
        "ve-docs-link",
        "ve-uorc-btn",
        "ve-studio-btn",
        "ve-render-btn",
        "ve-import-btn",
        "ve-mesh-btn",
        "ve-prompt-btn",
    ] {
        assert!(
            card_html.contains(&format!("id=\"{control_id}\"")),
            "site/card.html missing modeled DevRel control #{control_id}"
        );
    }

    for tab_key in ["journeys", "outliner", "create", "physics", "ui", "mesh"] {
        assert!(
            studio_mjs.contains(&format!("key: \"{tab_key}\"")),
            "site/studio.mjs missing Workbench tab '{tab_key}'"
        );
    }
    for persona_id in ["kai", "iris", "otto", "wren", "vera", "mira", "sol"] {
        assert!(
            studio_mjs.contains(&format!("id: \"{persona_id}\"")),
            "site/studio.mjs missing executable stakeholder journey '{persona_id}'"
        );
    }
    for arch_slug in [
        "citadel-explorer",
        "architectural-walkthrough",
        "conservation-chronicle-rpg",
        "compliant-diegetic-app",
    ] {
        assert!(
            studio_mjs.contains(&format!("slug: \"{arch_slug}\"")),
            "site/studio.mjs missing starter archetype '{arch_slug}'"
        );
    }

    // 4. Static WebAssembly C-ABI export verification across cold-boot & uorc binaries:
    let cold_exports = parse_wasm_exports(
        &std::fs::read(repo_root.join("site/verdant-wasm.wasm")).expect("cold wasm"),
    );
    let uorc_exports = parse_wasm_exports(
        &std::fs::read(repo_root.join("site/verdant-uorc.wasm")).expect("uorc wasm"),
    );

    for sym in collect_prefixed_symbols(&live_embed_mjs, "ex")
        .union(&collect_prefixed_symbols(&live_embed_mjs, "exCold"))
        .chain(collect_prefixed_symbols(&worker_mjs, "wasm").iter())
    {
        assert!(
            cold_exports.contains(sym),
            "cold-boot site/verdant-wasm.wasm is missing required C-ABI export '{sym}'"
        );
    }
    for sym in collect_prefixed_symbols(&live_embed_mjs, "uex")
        .union(&collect_prefixed_symbols(&worker_mjs, "uorcWasm"))
    {
        assert!(
            uorc_exports.contains(sym),
            "site/verdant-uorc.wasm is missing required C-ABI export '{sym}'"
        );
    }
}

/// `VE-33`: Hoisted Stratum-0 Prism constants, 256-entry CodebookProductLut contractions, 64-bit word-packed SoaSceneTable sweeps, contiguous 32-byte SAH FlatBvh traversal, epoch-split static and dynamic WebGL2 VAOs without gl.finish stalls, and hardware drawElementsInstanced batching scale deterministically to arbitrary 10k-entity and 50k-triangle workloads
#[test]
fn engine_ve_33() {
    use std::path::Path;
    use verdant_core::collision::FlatBvhNode;
    use verdant_core::ecs::{EntityId, SoaSceneTable, TwoLevelBvh};
    use verdant_core::hd_world::synthesize_hd_citadel_sector;
    use verdant_core::streaming::TerrainReliefParams;
    use verdant_core::verdant_tensor::{
        contract_coded_pair_lut_into, kernel_consts, CodebookProductLut, WordBitset,
    };

    // 1. Core-Wasm `scene:perf` & Lean-verified arbitrary-workload predicates (positive + negative):
    let verdict =
        verdant_core::prism_kernel::dispatch(b"scene:perf".to_vec()).expect("dispatch scene:perf");
    assert_eq!(
        std::str::from_utf8(&verdict).unwrap(),
        "conformant:word_bits=64,sah_bins=8,bvh_stack=64,codebook_lut=256,epoch_elision=true,max_inflight=2"
    );
    assert_eq!(
        verdant_core::prism_kernel::workloadPerformanceDirect(),
        Ok(true)
    );
    assert!(verdant_core::prism_kernel::epochElisionAdmissible(
        7, 7, false
    ));
    assert!(!verdant_core::prism_kernel::epochElisionAdmissible(
        6, 7, false
    ));
    assert!(!verdant_core::prism_kernel::epochElisionAdmissible(
        7, 7, true
    ));
    assert!(verdant_core::prism_kernel::workerInFlightAdmissible(2));
    assert!(!verdant_core::prism_kernel::workerInFlightAdmissible(3));
    assert_eq!(
        verdant_core::prism_kernel::governorRecommendedScalePermille(9_000, 16_666),
        Ok(1000)
    );
    assert_eq!(
        verdant_core::prism_kernel::governorRecommendedScalePermille(22_000, 16_666),
        Ok(850)
    );
    assert_eq!(
        verdant_core::prism_kernel::governorRecommendedScalePermille(40_000, 16_666),
        Ok(700)
    );

    // 2. Hoisted `VerifiedKernelConsts` (`OnceLock` pointer identity) + `256`-entry `CodebookProductLut`:
    let kc_a = kernel_consts();
    let kc_b = kernel_consts();
    assert!(
        std::ptr::eq(kc_a, kc_b),
        "OnceLock must hoist constants once"
    );
    assert_eq!(kc_a.skin_weight_scale, 1000);
    assert_eq!(kc_a.word_bit_width, 64);
    assert_eq!(kc_a.codebook_lut_len, 256);
    assert_eq!(kc_a.sah_bin_count, 8);

    let cb_lhs: [i32; 16] = core::array::from_fn(|i| (i as i32) - 8);
    let cb_rhs: [i32; 16] = core::array::from_fn(|i| (i as i32) * 3 - 15);
    let lut = CodebookProductLut::new(&cb_lhs, &cb_rhs);
    let mut lut_out = [0_i64; 4];
    contract_coded_pair_lut_into(&[2, 11, 5, 14], &[3, 7, 9, 1], 2, 2, 2, &lut, &mut lut_out)
        .expect("zero-alloc LUT contraction");
    assert_eq!(
        lut_out[0],
        i64::from(cb_lhs[2]) * i64::from(cb_rhs[3]) + i64::from(cb_lhs[11]) * i64::from(cb_rhs[9])
    );
    assert!(contract_coded_pair_lut_into(&[1], &[1], 2, 2, 2, &lut, &mut lut_out).is_err());

    // 3. `10,000`-entity `SoaSceneTable` + `64`-bit `WordBitset` (`O(dirty + N/64)` sweep):
    let entity_total = 10_000usize;
    let mut soa = SoaSceneTable::with_capacity(entity_total);
    let unit_bounds = Aabb::new(Vec3::new(-0.5, -0.5, -0.5), Vec3::new(0.5, 0.5, 0.5));
    for i in 0..entity_total {
        let gx = (i % 100) as f64;
        let gz = (i / 100) as f64;
        soa.spawn_dense(
            EntityId((i + 1) as u64),
            EntityRole::StaticArchitecture,
            None,
            Transform::from_xyz(gx * 2.0 - 100.0, 0.0, -gz * 2.0),
            unit_bounds,
            (Some(0), [0.8, 0.9, 1.0, 1.0]),
        );
    }
    assert_eq!(soa.len(), entity_total);
    assert_eq!(soa.visible_mask.words().len(), 157); // ceil(10_000 / 64) == 157 u64 words!
    assert_eq!(soa.propagate_dirty_transforms(), 0);
    for slot in 0..32 {
        soa.set_local_transform(slot, Transform::from_xyz(slot as f64, 2.5, -5.0));
    }
    assert_eq!(
        soa.propagate_dirty_transforms(),
        32,
        "WordBitset dirty propagation must touch strictly the 32 mutated slots"
    );
    assert_eq!(soa.propagate_dirty_transforms(), 0);

    let frustum = Frustum::from_perspective(
        Vec3::new(0.0, 4.0, 12.0),
        Vec3::new(0.0, -0.1, -1.0),
        Vec3::UP,
        1.05,
        16.0 / 9.0,
        0.25,
        90.0,
    );
    let vis_count = soa.cull_frustum_word_packed(&frustum);
    assert!(vis_count > 32 && vis_count < entity_total);
    let mut inst_buf = Vec::new();
    let compacted = soa.compact_visible_instances_for_mesh(0, &mut inst_buf);
    assert_eq!(compacted, vis_count);
    assert_eq!(inst_buf.len(), vis_count * 16);

    let mut wb = WordBitset::with_len(130);
    wb.set(0);
    wb.set(64);
    wb.set(129);
    assert_eq!(wb.count_ones(), 3);

    // 4. `FlatBvhNode` 32-byte layout + `LOD4` `45,000+` triangle `TwoLevelBvh` `O(log N)` pruning proof:
    assert_eq!(
        std::mem::size_of::<FlatBvhNode>(),
        32,
        "FlatBvhNode must be 32 bytes (two nodes per 64-byte L1 cache line)"
    );
    let sector =
        synthesize_hd_citadel_sector(7, &TerrainReliefParams::standard_basin_valley(), -1.35, 4);
    assert!(sector.scene.entity_count() >= 5);
    let bvh = TwoLevelBvh::build_solid(&sector.scene, &sector.meshes).expect("LOD4 TwoLevelBvh");
    assert!(bvh.total_surface_triangles() > 45_000);
    let total_nodes = bvh.total_bvh_nodes();
    assert!(total_nodes > 10_000);

    let mut total_ray_visits = 0usize;
    let mut ray_hits = 0usize;
    for i in 0..1_000usize {
        let fx = ((i % 40) as f64) * 0.5 - 10.0;
        let fz = ((i / 40) as f64) * 0.5 - 6.0;
        let ray = Ray::new(Vec3::new(fx, 24.0, fz), Vec3::new(0.0, -1.0, 0.0));
        let (hit, visited) = bvh.raycast_exact_counted(ray, 64.0);
        if hit.is_some() {
            ray_hits += 1;
        }
        total_ray_visits += visited;
    }
    let avg_ray_visits = total_ray_visits / 1_000;
    assert!(
        ray_hits > 800,
        "vertical rays over Citadel basin must hit terrain/masonry"
    );
    assert!(
        avg_ray_visits * 50 < total_nodes,
        "O(log N) FlatBvh raycast must visit < 2% of total nodes (avg={avg_ray_visits}, total={total_nodes})"
    );

    let mut total_sweep_visits = 0usize;
    for i in 0..100usize {
        let fx = ((i % 10) as f64) - 4.5;
        let (_, visited) =
            bvh.sweep_sphere_counted(Vec3::new(fx, 3.0, 8.0), 0.4, Vec3::new(0.0, -2.5, -1.0));
        total_sweep_visits += visited;
    }
    let avg_sweep_visits = total_sweep_visits / 100;
    assert!(
        avg_sweep_visits * 40 < total_nodes,
        "O(log N) FlatBvh sphere sweep must visit < 2.5% of total nodes (avg={avg_sweep_visits}, total={total_nodes})"
    );

    // 5. Wasm epoch-split static/dynamic buffers, 10k hardware instancing, & CLI `stress-benchmark`:
    assert_eq!(verdant_wasm::verdant_wasm_init(7), 1);
    let ep0 = verdant_wasm::verdant_wasm_static_epoch();
    let s_verts = verdant_wasm::verdant_wasm_static_vert_count();
    let d_verts = verdant_wasm::verdant_wasm_dyn_vert_count();
    assert!(ep0 >= 1 && s_verts > 1_000 && d_verts > 20);
    assert_eq!(
        verdant_wasm::verdant_wasm_step_lawful(16_000, 800, 0, 0, 1_000, 0),
        1
    );
    verdant_wasm::verdant_wasm_scene_rebuild_buffers(0.1);
    let ep1 = verdant_wasm::verdant_wasm_static_epoch();
    assert_eq!(
        ep0, ep1,
        "static_epoch must remain unchanged across dynamic character/physics frames (0B static VBO re-upload)"
    );
    let inst_tris = verdant_wasm::verdant_wasm_configure_instanced_workload(10_000);
    assert_eq!(verdant_wasm::verdant_wasm_inst_count(), 10_000);
    assert_eq!(inst_tris, 80_000);
    assert_eq!(
        verdant_wasm::verdant_wasm_configure_instanced_workload(0),
        0
    );

    let cli_out = verdant_cli::run_cli(&["stress-benchmark".to_string(), "10000".to_string()])
        .expect("verdant stress-benchmark 10000");
    assert!(
        cli_out.contains("STRESS BENCHMARK OK: soa_entities=10000 word_masks=157 dirty_updated=32")
    );

    // 6. Verify WebGL2 pipeline has zero `gl.finish` stalls, split STATIC_DRAW/STREAM_DRAW VAOs, shadow blit cache, & worker frameAck backpressure:
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let renderer_mjs = std::fs::read_to_string(repo_root.join("site/scene-renderer.mjs"))
        .expect("site/scene-renderer.mjs");
    let card_mjs = std::fs::read_to_string(repo_root.join("site/card.mjs")).expect("site/card.mjs");
    assert!(
        !renderer_mjs.contains("gl.finish("),
        "site/scene-renderer.mjs must never stall the GPU pipeline with synchronous gl.finish()"
    );
    assert!(renderer_mjs.contains("uploadSplitBuffers"));
    assert!(renderer_mjs.contains("gl.STATIC_DRAW"));
    assert!(renderer_mjs.contains("gl.STREAM_DRAW"));
    assert!(renderer_mjs.contains("drawElementsInstanced"));
    assert!(renderer_mjs.contains("blitFramebuffer"));
    assert!(card_mjs.contains("workerFramesInFlight"));
}

#[test]
fn engine_ve_34() {
    use std::path::Path;
    use verdant_core::ecs::{SceneWorld, TwoLevelBvh};
    use verdant_core::error::RenderGraphError;
    use verdant_core::geometry::generate_terrain_patch_mesh;
    use verdant_core::math::Vec3;
    use verdant_core::navmesh::NavMesh;
    use verdant_core::physics::{
        CharacterMoveIntent, JointKind, PhysicsWorld, PhysicsWorldConfig, RigidBody,
        RigidJointConstraint,
    };
    use verdant_core::runtime::{
        canonicalize_command_monoid, compute_backward_command_lightcone, CanonicalCommandKind,
        CanonicalCommandOp, CheckpointedRollbackJournal, KappaTransitionCache,
    };
    use verdant_core::verdant_tensor::{
        contract_packed_nibbles_lut_into, contract_sparse_runs_lut_into,
        decide_coded_matmul_admission, decide_uorc_challenger_admission, CodebookProductLut,
        CodedAdmissionVerdict, ColumnBitPlaneTable, SparseCodedRun, UorcChallengerVerdict,
        WordBitset,
    };

    // 1. Core-Wasm `scene:aqm` acceptance query & Lean-projected `aqmWorkloadExtrapolationsDirect`:
    assert_eq!(
        verdant_core::prism_kernel::aqmWorkloadExtrapolationsDirect(),
        Ok(true)
    );
    let verdict =
        verdant_core::prism_kernel::dispatch(b"scene:aqm".to_vec()).expect("dispatch scene:aqm");
    assert_eq!(
        std::str::from_utf8(&verdict).unwrap(),
        "conformant:monoid_fusion=true,bit_planes=8,checkpoint_stride=8,kappa_cache=true,coded_admission=true"
    );

    // 2. Column-transposed 8-plane `ColumnBitPlaneTable` over `16,384` entities (`O(N / 64)`):
    let n_entities = 16_384usize;
    let mut planes = ColumnBitPlaneTable::new(8, n_entities);
    assert_eq!(planes.planes_per_col(), 256);
    let mut expected_matches = 0usize;
    for idx in 0..n_entities {
        let p0 = true;
        let p1 = idx % 2 == 0;
        let p2 = idx % 3 != 0;
        let excl = idx % 8 == 0;
        planes.set(0, idx, p0);
        planes.set(1, idx, p1);
        planes.set(2, idx, p2);
        planes.set(3, idx, excl);
        if p0 && p1 && p2 && !excl {
            expected_matches += 1;
        }
    }
    let mut mask = WordBitset::with_len(n_entities);
    let matched = planes.query_archetype_into(&[0, 1, 2], &[3], &mut mask);
    assert_eq!(matched, expected_matches);
    assert!(planes.popcount_intersection(1, 2) > 5_000);
    planes.xor_column_into(1, 4);
    assert_eq!(planes.popcount_intersection(1, 4), n_entities / 2);

    // 3. Prism Algebraic Monoid Command Canonicalizer + Kahn DAG layers + cycle refusal (`R5`):
    let ops = vec![
        CanonicalCommandOp {
            id: 0,
            target_slot: 0,
            kind: CanonicalCommandKind::ToggleCameraMode,
            depends_on: Vec::new(),
        },
        CanonicalCommandOp {
            id: 1,
            target_slot: 0,
            kind: CanonicalCommandKind::ToggleCameraMode,
            depends_on: Vec::new(),
        },
        CanonicalCommandOp {
            id: 2,
            target_slot: 1,
            kind: CanonicalCommandKind::YawOctantStep { octants: 3 },
            depends_on: Vec::new(),
        },
        CanonicalCommandOp {
            id: 3,
            target_slot: 1,
            kind: CanonicalCommandKind::YawOctantStep { octants: 5 },
            depends_on: Vec::new(),
        },
        CanonicalCommandOp {
            id: 4,
            target_slot: 0,
            kind: CanonicalCommandKind::CameraOrbitDelta {
                delta_yaw_milli: 120,
                delta_pitch_milli: -30,
                delta_dist_milli: 200,
            },
            depends_on: Vec::new(),
        },
        CanonicalCommandOp {
            id: 5,
            target_slot: 0,
            kind: CanonicalCommandKind::CameraOrbitDelta {
                delta_yaw_milli: 80,
                delta_pitch_milli: 30,
                delta_dist_milli: -50,
            },
            depends_on: Vec::new(),
        },
    ];
    let monoid = canonicalize_command_monoid(&ops).expect("acyclic monoid stream");
    assert_eq!(monoid.raw_count, 6);
    assert_eq!(monoid.involution_cancellations, 1);
    assert_eq!(monoid.additive_merges, 2);
    assert_eq!(monoid.fused_count, 1);
    assert_eq!(
        monoid.fused_ops[0].kind,
        CanonicalCommandKind::CameraOrbitDelta {
            delta_yaw_milli: 200,
            delta_pitch_milli: 0,
            delta_dist_milli: 150,
        }
    );
    let cone = compute_backward_command_lightcone(&ops, 0, ops.len());
    assert!(cone.contains(&0) && cone.contains(&5) && !cone.contains(&2));

    let cyclic_ops = vec![
        CanonicalCommandOp {
            id: 0,
            target_slot: 0,
            kind: CanonicalCommandKind::ToggleCameraMode,
            depends_on: vec![1],
        },
        CanonicalCommandOp {
            id: 1,
            target_slot: 1,
            kind: CanonicalCommandKind::ToggleRenderBackend,
            depends_on: vec![0],
        },
    ];
    assert!(matches!(
        canonicalize_command_monoid(&cyclic_ops),
        Err(RenderGraphError::InvalidPassDependency { .. })
    ));

    // 4. `O(P)` hash-indexed `NavMesh::from_mesh`, `BinaryHeap` `A*`, & in-place tropical `next_hop` closure:
    let nav_terrain = generate_terrain_patch_mesh(7, 24.0, 16);
    let nav = NavMesh::from_mesh(&nav_terrain).expect("O(P) hash-indexed NavMesh");
    assert!(nav.polygons.len() >= 64);
    let goal = nav.polygons.len() - 1;
    let astar = nav.find_path_astar(0, goal).expect("BinaryHeap A*");
    let (trop_corridor, relaxations) = nav
        .shortest_corridor_with_path_tropical(0, goal)
        .expect("in-place tropical next_hop path");
    assert!(relaxations > 0);
    assert_eq!(trop_corridor.polygon_indices.first(), Some(&0));
    assert_eq!(trop_corridor.polygon_indices.last(), Some(&goal));
    assert!(
        (trop_corridor.total_cost_meters - astar.total_cost_meters).abs() < 1e-2,
        "in-place tropical next_hop corridor cost ({}) must match admissible A* ({})",
        trop_corridor.total_cost_meters,
        astar.total_cost_meters
    );

    // 5. Sparse 2-body stack Delassus XPBD solver (`0` dense DOF allocs):
    let mut pw = PhysicsWorld::new(PhysicsWorldConfig::default());
    pw.bodies.push(
        RigidBody::new("b0", Vec3::new(0.0, 6.0, 0.0), 2.0, 0.4, 0.3).expect("rigid body b0"),
    );
    pw.bodies.push(
        RigidBody::new("b1", Vec3::new(0.0, 4.2, 0.0), 1.5, 0.4, 0.3).expect("rigid body b1"),
    );
    pw.rigid_joints.push(RigidJointConstraint {
        label: "ball_j0".to_string(),
        body_a: 0,
        body_b: 1,
        kind: JointKind::BallSocket,
        target_distance_meters: 1.5,
        axis_world: Vec3::UP,
        compliance_permille: 5,
    });
    let empty_bvh = TwoLevelBvh::build_solid(&SceneWorld::new(), &[]).expect("empty TwoLevelBvh");
    pw.step_lawful(1.0 / 60.0, CharacterMoveIntent::default(), &empty_bvh)
        .expect("sparse XPBD step 1");
    pw.step_lawful(1.0 / 60.0, CharacterMoveIntent::default(), &empty_bvh)
        .expect("sparse XPBD step 2");
    let sep = (pw.bodies[1].position - pw.bodies[0].position).length();
    assert!(sep > 0.5 && sep < 2.2);

    // 6. Stride-8 `CheckpointedRollbackJournal` & `KappaTransitionCache`:
    let mut journal: CheckpointedRollbackJournal<i64, i64> =
        CheckpointedRollbackJournal::new(100, b"genesis-seed-7");
    assert_eq!(journal.checkpoint_stride(), 8);
    let mut running = 100_i64;
    let mut reference_states = vec![running];
    for f in 1..=32_i64 {
        running += f * 3;
        reference_states.push(running);
        journal.record_step(f * 3, &f.to_le_bytes(), &running);
    }
    assert_eq!(journal.stored_checkpoint_count(), 5);
    let (reconstructed_29, replay_steps, _) = journal
        .reconstruct_frame(29, |s, delta| s + delta)
        .expect("reconstruct frame 29");
    assert_eq!(reconstructed_29, reference_states[29]);
    assert_eq!(replay_steps, 5);
    assert!(journal.reconstruct_frame(33, |s, d| s + d).is_err());

    let mut kcache: KappaTransitionCache<u64> = KappaTransitionCache::new(16);
    let k_init = KappaTransitionCache::<u64>::digest_bytes(b"observer-q-state");
    let (v1, _, hit1) = kcache.step_or_elide(k_init, 42, || (999_u64, b"next-999".to_vec()));
    let (v2, _, hit2) = kcache.step_or_elide(k_init, 42, || (0_u64, b"unreachable".to_vec()));
    assert!(!hit1 && hit2 && v1 == 999 && v2 == 999 && kcache.elided_step_hits() == 1);

    // 7. Sub-byte `Packed<Grid<16>, 2>` nibble LUT, `O(nnz)` `SparseCodedRun`, & Coded/UORC Admission Gates:
    let mut cb = [0_i32; 16];
    for (i, slot) in cb.iter_mut().enumerate() {
        *slot = (i as i32) - 4;
    }
    let lut = CodebookProductLut::new(&cb, &cb);
    let lhs_pack = [0x21_u8, 0x65_u8]; // codes [1, 2, 5, 6] -> [-3, -2, 1, 2]
    let rhs_pack = [0x43_u8, 0x87_u8]; // codes [3, 4, 7, 8] -> [-1, 0, 3, 4]
    let mut out_nib = [0_i64; 1];
    let bytes_read =
        contract_packed_nibbles_lut_into(&lhs_pack, &rhs_pack, 1, 4, 1, &lut, &mut out_nib)
            .expect("packed nibble LUT");
    assert_eq!(bytes_read, 4);
    assert_eq!(out_nib[0], 14);

    let runs = [
        SparseCodedRun {
            col: 1,
            code_idx: 6,
        },
        SparseCodedRun {
            col: 3,
            code_idx: 9,
        },
    ];
    let mut out_sparse = [0_i64; 1];
    let (visited, skipped) =
        contract_sparse_runs_lut_into(&[0, 2], &runs, &[0, 5, 0, 8], 4, &lut, &mut out_sparse)
            .expect("sparse runs LUT");
    assert_eq!(visited, 2);
    assert_eq!(skipped, 2);
    assert_eq!(out_sparse[0], 22);

    assert_eq!(
        decide_coded_matmul_admission(64, 32, false, true),
        CodedAdmissionVerdict::AdmittedZeroDecode
    );
    assert_eq!(
        decide_coded_matmul_admission(64, 32, true, true),
        CodedAdmissionVerdict::RefusedFloatDecode
    );
    assert_eq!(
        decide_coded_matmul_admission(16, 32, false, true),
        CodedAdmissionVerdict::RefusedAccumulatorOverflow
    );
    assert_eq!(
        decide_uorc_challenger_admission(b"VDNT", b"VDNT", true, 120, 96),
        UorcChallengerVerdict::AdmitChallenger
    );
    assert_eq!(
        decide_uorc_challenger_admission(b"VDNT", b"BAD!", true, 120, 96),
        UorcChallengerVerdict::RefuseLossyReconstruction
    );
    assert_eq!(
        decide_uorc_challenger_admission(b"VDNT", b"VDNT", true, 96, 120),
        UorcChallengerVerdict::RefuseSizeRegression
    );

    // 8. Verify WebGL2 `gl.fenceSync` non-blocking GPU pacing & worker `verdant_wasm_aqm_benchmark_aux`:
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let renderer_mjs = std::fs::read_to_string(repo_root.join("site/scene-renderer.mjs"))
        .expect("site/scene-renderer.mjs");
    assert!(renderer_mjs.contains("fenceSync"));
    assert!(verdant_wasm::verdant_wasm_aqm_benchmark_aux(16_384) > 40);
}
