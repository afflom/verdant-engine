//! Conformance suite for `verdant-core` and the Verdant Engine verifier (`VE-01`..`VE-10`).

use std::path::PathBuf;

use verdant_core::audio::{
    evaluate_binaural_cue, synthesize_euler_bernoulli_chime_modes, AcousticEnvironment,
    ListenerPose, SpatialAudioSource,
};
use verdant_core::ecs::{EntityRole, SceneWorld, Transform};
use verdant_core::error::{
    AssetValidationError, EngineConfigError, PhysicsStepError, ShaderValidationError,
    SnapshotCodecError, StructuralEquilibriumError, WorldEpochError,
};
use verdant_core::genesis::{
    heyman_middle_third_limit, verify_blueprint_equilibrium, CausalWorldState, CosmicEpoch,
    StructureKind,
};
use verdant_core::geometry::{canonical_scene_mesh_library, MorphTarget, ProceduralMesh};
use verdant_core::gltf_io::{export_scene_to_glb, validate_glb_bytes};
use verdant_core::math::{Aabb, Frustum, Quat, Ray, Vec3};
use verdant_core::physics::{
    CapsuleController, KinematicSplineRover, PhysicsWorld, PhysicsWorldConfig, RigidBody,
};
use verdant_core::render::{
    canonical_pbr_material_library, validate_builtin_engine_shaders, validate_wgsl_shader,
    WGSL_PBR_SURFACE_SHADER,
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
    assert_eq!(modules.len(), 3);

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
    assert!(engine.contains("refused:epoch-precedes-builders"));
    assert!(engine.contains("refused:heyman-middle-third-exceeded"));
    assert!(engine.contains("\"invalid request\""));
    assert!(!genesis.contains("Classical.choice"));
    assert!(!engine.contains("Classical.choice"));
}
