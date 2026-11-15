//! Conformance suite for `verdant-core` and the Verdant Engine verifier (`VE-01`..`VE-14`).

use std::path::PathBuf;

use verdant_core::animation::{
    skin_mesh_linear_blend, Joint, KeyframePose, SkeletalClip, Skeleton, SkinWeights4,
};
use verdant_core::audio::{
    evaluate_binaural_cue, synthesize_euler_bernoulli_chime_modes, AcousticEnvironment,
    ListenerPose, SpatialAudioSource,
};
use verdant_core::collision::{
    gjk_intersect_and_manifold, solve_contact_impulse, ConvexHullCollider,
};
use verdant_core::ecs::{EntityRole, SceneWorld, Transform};
use verdant_core::error::{
    AnimationError, AssetValidationError, CollisionError, EngineConfigError, NavigationError,
    PhysicsStepError, RenderGraphError, ShaderValidationError, SnapshotCodecError,
    StructuralEquilibriumError, WorldEpochError,
};
use verdant_core::genesis::{
    heyman_middle_third_limit, verify_blueprint_equilibrium, CausalWorldState, CosmicEpoch,
    StructureKind,
};
use verdant_core::geometry::{canonical_scene_mesh_library, MorphTarget, ProceduralMesh};
use verdant_core::gltf_io::{export_scene_to_glb, validate_glb_bytes};
use verdant_core::math::{Aabb, Frustum, Quat, Ray, Vec3};
use verdant_core::navmesh::{steer_along_corridor, NavMesh};
use verdant_core::physics::{
    CapsuleController, KinematicSplineRover, PhysicsWorld, PhysicsWorldConfig, RigidBody,
};
use verdant_core::render::{
    canonical_pbr_material_library, compute_csm_splits, validate_builtin_engine_shaders,
    validate_wgsl_shader, IblEnvironmentProbe, RenderFrameGraph, WGSL_PBR_SURFACE_SHADER,
};
use verdant_core::runtime::{EngineConfig, FrameBudgetProfiler, WorkerJobGraph, WorldSnapshot};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("crates/conformance is two below root")
        .to_path_buf()
}

/// `VE-01`: Monotonic cosmological expansion across epochs and `WorldEpochError` refusal.
#[test]
fn engine_ve_01() {
    let mut world = CausalWorldState::new_cosmogenic(11).expect("valid cosmogenic seed");
    assert_eq!(world.active_epoch, CosmicEpoch::Cosmogenic);
    assert_eq!(world.blueprints.len(), 0);

    world.advance_to(0.42).expect("advance to LithicHydrologic");
    assert_eq!(world.active_epoch, CosmicEpoch::LithicHydrologic);

    world.advance_to(0.72).expect("advance to BioticEcological");
    assert_eq!(world.active_epoch, CosmicEpoch::BioticEcological);
    assert!(world.flora_count > 0);

    // Negative refusal 1: stepping backward in cosmological expansion is refused:
    let backward = world
        .advance_to(0.30)
        .expect_err("backward cosmic scale must fail");
    assert!(matches!(
        backward,
        WorldEpochError::NonMonotonicExpansion { .. }
    ));

    // Negative refusal 2: out-of-bounds scale factor (`> 1.0` or `<= 0.0`) is refused:
    let invalid_scale = world.advance_to(1.25).expect_err("scale > 1.0 must fail");
    assert!(matches!(
        invalid_scale,
        WorldEpochError::InvalidScaleFactor { .. }
    ));

    world.advance_to(1.0).expect("advance to t_obs");
    assert_eq!(world.active_epoch, CosmicEpoch::AgenticArchitectural);
}

/// `VE-02`: Seven Builder structures + perimeter rover equilibrium and `StructuralEquilibriumError` refusal.
#[test]
fn engine_ve_02() {
    let mut world = CausalWorldState::new_cosmogenic(7).expect("seed");
    world.advance_to(1.0).expect("evolve to t_obs");

    assert_eq!(world.builder_guilds.len(), 5);
    assert_eq!(world.blueprints.len(), 7);
    assert!(world
        .blueprints
        .iter()
        .any(|b| b.kind == StructureKind::PerimeterSentinelRover));

    for bp in &world.blueprints {
        assert!(
            bp.thrust_eccentricity_ratio <= heyman_middle_third_limit(),
            "blueprint {:?} violated Heyman middle-third rule",
            bp.kind
        );
        assert!(bp.safety_factor >= 2.0);
        assert!(bp.net_engineering_utility > 0.0);
    }

    // Negative refusal: an overly slender span whose thrust line exits the middle third is refused:
    let err = verify_blueprint_equilibrium(48.0, 2.0, 0.18, 50.0, 2500.0, 9.81, 0.5)
        .expect_err("slender arch must violate Heyman middle third");
    assert!(matches!(
        err,
        StructuralEquilibriumError::ThrustLineOutsideMiddleThird { .. }
            | StructuralEquilibriumError::InsufficientSafetyFactor { .. }
    ));
}

/// `VE-03`: Archetype ECS hierarchical scene graph and SAH BVH raycast / frustum queries.
#[test]
fn engine_ve_03() {
    let mut scene = SceneWorld::new();
    let root_id = scene.spawn(
        "ArenaFloor",
        EntityRole::TerrainFeature,
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
        EntityRole::EngineeredStructure,
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
    let mut world = CausalWorldState::new_cosmogenic(5).expect("seed");
    world.advance_to(1.0).expect("t_obs");
    let mut meshes = canonical_scene_mesh_library(world.seed);
    let materials = canonical_pbr_material_library();

    let base_count = meshes[2].vertex_count();
    meshes[2].morph_targets.push(MorphTarget {
        name: "BloomOpen".to_string(),
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

/// `VE-06`: Symplectic Euler physics, capsule controller, perimeter rover, and `PhysicsStepError` refusal.
#[test]
fn engine_ve_06() {
    let mut world = CausalWorldState::new_cosmogenic(13).expect("seed");
    world.advance_to(1.0).expect("t_obs");

    let mut phys = PhysicsWorld::new(PhysicsWorldConfig::default());
    phys.bodies.push(
        RigidBody::new("LagoonFloat", Vec3::new(4.0, -0.35, -4.0), 10.0, 0.45, 0.35)
            .expect("valid rigid body"),
    );
    phys.player_capsule = Some(CapsuleController::new(Vec3::new(0.0, 2.5, 12.0)));
    phys.sentinel_rover = Some(KinematicSplineRover::new(
        world.rover_perimeter_waypoints.clone(),
        3.0,
    ));

    for _ in 0..30 {
        phys.step(1.0 / 60.0, Vec3::new(0.0, 0.0, -1.0))
            .expect("deterministic step");
    }
    let rover = phys.sentinel_rover.as_ref().expect("rover active");
    assert!(rover.distance_traveled_meters > 1.0);

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

/// `VE-07`: Woodworth binaural spatial audio, Sabine RT60 reverberation, and Euler-Bernoulli modal chimes.
#[test]
fn engine_ve_07() {
    let rotunda = AcousticEnvironment::travertine_rotunda(343.0);
    let meadow = AcousticEnvironment::open_meadow(343.0);
    assert!(rotunda.sabine_rt60_seconds() > meadow.sabine_rt60_seconds());
    assert!(rotunda.eyring_rt60_seconds() > 0.0);

    let listener = ListenerPose {
        position: Vec3::ZERO,
        forward: Vec3::new(0.0, 0.0, -1.0),
        up: Vec3::UP,
    };
    let source_right = SpatialAudioSource {
        label: "ModalChimePergola".to_string(),
        position: Vec3::new(6.0, 1.8, -2.0),
        fundamental_hz: 440.0,
        reference_gain: 1.0,
    };
    let cue = evaluate_binaural_cue(&source_right, &listener, &rotunda);
    assert!(cue.azimuth_sin > 0.0);
    assert!(cue.ild_db > 0.0);
    assert!(cue.itd_seconds > 0.0);

    let modes = synthesize_euler_bernoulli_chime_modes(440.0, 4);
    assert_eq!(modes.len(), 4);
    assert!(modes[1] > modes[0]);
    assert!(rotunda.sabine_rt60_prism_millis() > meadow.sabine_rt60_prism_millis());
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
    let mut world = CausalWorldState::new_cosmogenic(19).expect("seed");
    world.advance_to(1.0).expect("t_obs");

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
    let genesis = std::fs::read_to_string(root.join("app/src/Genesis.lex.tex")).unwrap();
    let engine = std::fs::read_to_string(root.join("app/src/Engine.lex.tex")).unwrap();
    let app_json: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(root.join("site/app.json")).unwrap())
            .unwrap();

    let theorems = app_json["theorems"].as_array().expect("theorems array");
    assert!(!theorems.is_empty());
    for thm in theorems {
        let name = thm["name"].as_str().unwrap();
        assert!(
            genesis.contains(&format!("name: {name},")),
            "Genesis.lex.tex must declare theorem `{name}` advertised in site/app.json"
        );
    }

    assert!(engine.contains("entryRoot := \"VerdantEngine.Engine.dispatch\""));
    assert!(engine.contains("blueprint:6"));
    assert!(engine.contains("scene:epoch:3"));
    assert!(engine.contains("scene:blueprint:6"));
    assert!(engine.contains("scene:acoustics"));
    assert!(engine.contains("scene:pipeline"));
    assert!(engine.contains("refused:epoch-precedes-builders"));
    assert!(engine.contains("refused:heyman-middle-third-exceeded"));
    assert!(engine.contains("\"invalid request\""));
    assert!(!genesis.contains("Classical.choice"));
    assert!(!engine.contains("Classical.choice"));

    // Verify that `verdant-core` executes the exact Lean-compiled Prism kernel (`prism_kernel`):
    let summary_bytes = verdant_core::prism_kernel::dispatch(b"summary".to_vec())
        .expect("prism_kernel dispatch summary");
    assert_eq!(
        std::str::from_utf8(&summary_bytes).unwrap(),
        "conformant:epochs=4,blueprints=7,scale=1.0,zero-constants=true"
    );
    let scene_rover = verdant_core::prism_kernel::dispatch(b"scene:blueprint:6".to_vec())
        .expect("prism_kernel dispatch scene:blueprint:6");
    assert_eq!(
        std::str::from_utf8(&scene_rover).unwrap(),
        "conformant:id=6,name=PerimeterSentinelRover,guild=Automation,pos=180/15/0,dist=11,rover=1"
    );
    let refused_bytes = verdant_core::prism_kernel::dispatch(b"blueprint:unstable".to_vec())
        .expect("prism_kernel dispatch blueprint:unstable");
    assert_eq!(
        std::str::from_utf8(&refused_bytes).unwrap(),
        "refused:heyman-middle-third-exceeded"
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

    // Pick a connected pair of polygons via neighbor walk:
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

/// `VE-15`: `WasmRuntimeBridge` steps sanctuary physics, encodes/restores SHA-256 `VDNT` snapshots, verifies `site/verdant-wasm.wasm`, and refuses invalid inputs.
#[test]
fn engine_ve_15() {
    use verdant_wasm::{WasmBridgeError, WasmRuntimeBridge};

    let mut bridge = WasmRuntimeBridge::new(7).expect("WasmRuntimeBridge::new");
    assert_eq!(bridge.epoch_rank, 3);
    assert_eq!(bridge.world.blueprints.len(), 7);
    assert!(bridge.walkable_polygons > 100);
    let tobs_snap = bridge
        .encode_snapshot()
        .expect("t_obs snapshot fits Prism URL bridge envelope");
    assert_eq!(&tobs_snap[0..4], b"VDNT");

    let rover_before = bridge
        .physics
        .sentinel_rover
        .as_ref()
        .expect("rover")
        .position;
    bridge
        .step_frame(1.0 / 60.0, 1.0, 0.0, 0.0)
        .expect("step_frame");
    let rover_after = bridge
        .physics
        .sentinel_rover
        .as_ref()
        .expect("rover")
        .position;
    assert!((rover_after - rover_before).length() > 1e-4);

    bridge.set_epoch_rank(1).expect("lithic epoch");
    assert_eq!(bridge.epoch_rank, 1);
    assert_eq!(bridge.world.blueprints.len(), 0);
    let snap = bridge.encode_snapshot().expect("encode VDNT snapshot");
    assert_eq!(&snap[0..4], b"VDNT");

    bridge.set_epoch_rank(3).expect("back to t_obs");
    assert_eq!(bridge.world.blueprints.len(), 7);
    bridge
        .decode_and_restore_snapshot(&snap)
        .expect("restore VDNT snapshot");
    assert_eq!(bridge.epoch_rank, 1);
    assert_eq!(bridge.world.blueprints.len(), 0);

    // Negative refusal 1: invalid epoch rank (> 3):
    let bad_epoch = bridge
        .set_epoch_rank(9)
        .expect_err("invalid epoch rank must fail");
    assert!(matches!(
        bad_epoch,
        WasmBridgeError::InvalidEpochRank { rank: 9 }
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

/// `VE-16`: `verdant-cli` asset/shader/snapshot validator, sanctuary packer, headless benchmark, and `CliCommandError` refusals.
#[test]
fn engine_ve_16() {
    use verdant_cli::{run_cli, CliCommandError};
    use verdant_core::render::WGSL_PBR_SURFACE_SHADER;

    let tmp_dir = std::env::temp_dir().join(format!("verdant_cli_ve16_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&tmp_dir);
    std::fs::create_dir_all(&tmp_dir).expect("temp dir");

    let glb_path = tmp_dir.join("sanctuary.glb");
    let vdnt_path = tmp_dir.join("sanctuary.glb.vdnt");
    let wgsl_path = tmp_dir.join("pbr_terrain.wgsl");
    std::fs::write(&wgsl_path, WGSL_PBR_SURFACE_SHADER).expect("write wgsl");

    let pack_out = run_cli(&[
        "export-sanctuary".to_string(),
        glb_path.to_string_lossy().into_owned(),
        "7".to_string(),
        "1.0".to_string(),
    ])
    .expect("export-sanctuary succeeds");
    assert!(pack_out.contains("PACKED OK:"));
    assert!(pack_out.contains("blueprints=7"));

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

    let bench_out =
        run_cli(&["benchmark".to_string(), "16".to_string()]).expect("benchmark succeeds");
    assert!(bench_out.contains("BENCHMARK OK: frames=16"));
    assert!(bench_out.contains("passes=5"));
    assert!(bench_out.contains("csm_cascades=4"));

    // Negative refusal 1: unknown subcommand returns `CliCommandError::InvalidUsage`:
    let bad_cmd = run_cli(&["unknown-subcmd".to_string()]).expect_err("unknown subcommand fails");
    assert!(matches!(bad_cmd, CliCommandError::InvalidUsage { .. }));

    // Negative refusal 2: corrupted `.glb` rejected with `CliCommandError::ValidationRejected`:
    let corrupt_glb = tmp_dir.join("corrupt.glb");
    std::fs::write(&corrupt_glb, b"not-a-valid-glb-container").expect("write corrupt glb");
    let bad_glb = run_cli(&[
        "validate-glb".to_string(),
        corrupt_glb.to_string_lossy().into_owned(),
    ])
    .expect_err("corrupt glb must be refused");
    assert!(matches!(
        bad_glb,
        CliCommandError::ValidationRejected { .. }
    ));

    let _ = std::fs::remove_dir_all(&tmp_dir);
}
