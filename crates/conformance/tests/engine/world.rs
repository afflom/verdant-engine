#![allow(unused_imports)]
use super::core::append_test_box_to_mesh;
use super::{repo_root, wasm_abi_lock};
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

    // A thin rail in a separate lane intersects only the middle of an upright capsule.
    append_test_box_to_mesh(
        &mut doorway,
        Vec3::new(9.0, 0.8, -0.05),
        Vec3::new(11.0, 0.9, 0.05),
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

    let lower = Vec3::new(10.0, 0.2, 3.0);
    let axis = Vec3::new(0.0, 1.4, 0.0);
    let crossing = Vec3::new(0.0, 0.0, -6.0);
    assert!(bvh.sweep_sphere(lower, 0.2, crossing).is_none());
    assert!(bvh.sweep_sphere(lower + axis, 0.2, crossing).is_none());
    let (rail, hit) = bvh
        .sweep_capsule_counted(lower, axis, 0.2, crossing)
        .0
        .expect("side of capsule hits rail");
    assert_eq!(rail, e_door);
    assert!((hit.toi - (3.0 - 0.25) / 6.0).abs() < 1.0e-6);
    assert!((0.79..0.91).contains(&hit.contact_point.y));

    // 1. Walk straight through the hollow doorway opening (`X = 0`, `Z: +2.0 -> -2.0`):
    let mut cfg = PhysicsWorldConfig::default();
    cfg.terrain_relief.basin_depth_meters = 0.0;
    cfg.terrain_relief.hill_amplitude_meters = 0.0;
    cfg.terrain_relief.outer_rim_curvature = 0.0;
    cfg.platform_deck = None;
    cfg.environment.water_level_meters = -10.0;

    let mut phys = PhysicsWorld::new(cfg);
    // Disjoint endpoints across the step still collide between the endpoints.
    for (x, vx) in [(-0.3, 16.0), (0.3, -16.0)] {
        let mut body =
            verdant_core::physics::RigidBody::new("probe", Vec3::new(x, 20.0, 0.0), 1.0, 0.1, 0.0)
                .unwrap();
        body.velocity.x = vx;
        phys.bodies.push(body);
    }
    phys.step_lawful(1.0 / 30.0, CharacterMoveIntent::default(), &bvh)
        .unwrap();
    assert!(phys.bodies[1].position.x - phys.bodies[0].position.x >= 0.2 - 1.0e-9);
    phys.bodies.clear();
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
                &|_, _| 0.0,
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
    {
        let _abi_guard = wasm_abi_lock();
        assert_eq!(verdant_wasm::verdant_wasm_init(7), 1);
        let c0 = verdant_wasm::verdant_wasm_csm_split_meters(0.25, 140.0, 0);
        let c3 = verdant_wasm::verdant_wasm_csm_split_meters(0.25, 140.0, 3);
        assert!(c0 > 0.25 && c3 > c0 && (c3 - 140.0).abs() < 1e-4);
        let out_len = verdant_wasm::verdant_wasm_scene_outliner_aux(8_200);
        assert!(out_len > 64, "scene_outliner_aux must stage entity table");
    }

    // 3. Audit Studio token purity, 13 DevRel controls, 7 functional Studio role workspaces, 4 external starter archetypes, 5 platform organs, & C-ABI parity:
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let card_html = std::fs::read_to_string(repo_root.join("site/card.html")).expect("card.html");
    let card_css = std::fs::read_to_string(repo_root.join("site/card.css")).expect("card.css");
    let studio_data_mjs =
        std::fs::read_to_string(repo_root.join("site/studio-data.mjs")).expect("studio-data.mjs");
    let studio_mjs =
        std::fs::read_to_string(repo_root.join("site/studio.mjs")).expect("studio.mjs");
    let catalogs_mjs = std::fs::read_to_string(repo_root.join("site/prism-catalogs.mjs"))
        .expect("prism-catalogs.mjs");
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
        contains_hex_color(&studio_data_mjs),
        None,
        "site/studio-data.mjs must use 100% W3C DTCG variables with zero raw hex colors"
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

    for role_key in [
        "dashboard",
        "code",
        "design",
        "audio",
        "narrative",
        "director",
        "gameplay",
    ] {
        assert!(
            studio_data_mjs.contains(&format!("roleKey: \"{role_key}\""))
                && studio_mjs.contains(&format!("key: \"{role_key}\"")),
            "Verdant Studio missing functional role workspace '{role_key}'"
        );
    }
    for organ_slug in ["engine", "studio", "catalogs", "demo", "docs"] {
        assert!(
            catalogs_mjs.contains(&format!("slug: \"{organ_slug}\"")),
            "site/prism-catalogs.mjs missing differentiated platform organ '{organ_slug}'"
        );
    }
    for arch_slug in [
        "sunspire-regatta",
        "meridian-observatory",
        "glasshouse-chronicle",
        "starfall-bastion",
    ] {
        assert!(
            studio_data_mjs.contains(&format!("slug: \"{arch_slug}\"")),
            "site/studio-data.mjs missing external starter archetype '{arch_slug}'"
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
    {
        let _abi_guard = wasm_abi_lock();
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
    }

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
    {
        let _abi_guard = wasm_abi_lock();
        assert!(verdant_wasm::verdant_wasm_init(7) == 1);
        assert!(verdant_wasm::verdant_wasm_aqm_benchmark_aux(16_384) > 40);
    }
}

/// `VE-35`: Flat CSR software tile rasterizer, 512-body spatial broadphase, multi-primitive GLB
/// ingestion, WebGL2 context recovery, worker watchdogs, and 52-bit floating-origin horizon guard.
#[test]
fn engine_ve_35() {
    use std::path::Path;
    use verdant_core::ecs::{SceneWorld, TwoLevelBvh};
    use verdant_core::error::{NarrativeLawError, TensorComputeError};
    use verdant_core::geometry::{generate_faceted_prism_mesh, generate_fluted_column_mesh};
    use verdant_core::gltf_io::{import_glb_to_scene, pack_multi_primitive_glb};
    use verdant_core::math::{Mat4, Vec3};
    use verdant_core::physics::{
        CapsuleController, CharacterMoveIntent, PhysicsWorld, PhysicsWorldConfig, RigidBody,
    };
    use verdant_core::render::canonical_pbr_material_library;
    use verdant_core::tensor::{
        render_triangles_tile_framebuffer_with_scratch, RasterTriangle, TileRasterScratch,
    };
    use verdant_core::universe::{UniverseCellCoord, UniversePosition};

    // 1. Core-Wasm `scene:stability` acceptance query & Lean-proved `stabilityAndRefinementDirect`:
    assert_eq!(
        verdant_core::prism_kernel::stabilityAndRefinementDirect(),
        Ok(true)
    );
    let verdict = verdant_core::prism_kernel::dispatch(b"scene:stability".to_vec())
        .expect("dispatch scene:stability");
    assert_eq!(
        std::str::from_utf8(&verdict).expect("utf8 verdict"),
        "conformant:csr_tile_bins=true,spatial_broadphase=true,context_recovery=true,watchdog_ms=15000,mantissa_bits=52"
    );

    // 2. Flat CSR `TileRasterScratch` steady-state zero heap growth + extreme near-clip safety + NaN refusal:
    let vp = Mat4::perspective(std::f64::consts::FRAC_PI_3, 128.0 / 64.0, 0.25, 200.0)
        * Mat4::look_at(Vec3::new(0.0, 2.0, 8.0), Vec3::ZERO, Vec3::UP);
    let tris = vec![
        RasterTriangle {
            positions: [[-1.5, -1.0, 0.0], [1.5, -1.0, 0.0], [0.0, 2.0, 0.0]],
            normal: [0.0, 0.0, 1.0],
            base_color: [0.8, 0.4, 0.2],
        },
        // Extreme off-screen / near-clip huge span triangle that must clamp safely without u32 overflow:
        RasterTriangle {
            positions: [
                [-1.0e5, -1.0e5, -20.0],
                [1.0e5, -1.0e5, -20.0],
                [0.0, 1.0e5, -20.0],
            ],
            normal: [0.0, 1.0, 0.0],
            base_color: [0.2, 0.7, 0.9],
        },
    ];
    let mut scratch = TileRasterScratch::new();
    let (tiles_1, rast_1) = render_triangles_tile_framebuffer_with_scratch(
        128,
        64,
        &vp,
        Vec3::new(0.4, 0.8, 0.5),
        3,
        &tris,
        &mut scratch,
    )
    .expect("CSR frame 1");
    assert_eq!(tiles_1, 32);
    assert!(rast_1 >= 1);
    let cap_after_frame_1 = scratch.total_allocated_bytes();
    let (tiles_2, rast_2) = render_triangles_tile_framebuffer_with_scratch(
        128,
        64,
        &vp,
        Vec3::new(0.4, 0.8, 0.5),
        3,
        &tris,
        &mut scratch,
    )
    .expect("CSR frame 2");
    assert_eq!((tiles_2, rast_2), (tiles_1, rast_1));
    assert_eq!(
        scratch.total_allocated_bytes(),
        cap_after_frame_1,
        "CSR tile arena must incur zero heap capacity growth in steady state"
    );

    let nan_sun_err = render_triangles_tile_framebuffer_with_scratch(
        64,
        64,
        &vp,
        Vec3::new(f64::NAN, 1.0, 0.0),
        3,
        &tris,
        &mut scratch,
    )
    .expect_err("NaN sun_direction must be refused");
    assert!(matches!(
        nan_sun_err,
        TensorComputeError::NonFiniteTensorEntry {
            operand: "sun_direction"
        }
    ));

    // 3. 512-body 3D spatial-grid broadphase sphere collisions + NaN intent & terminal speed clamping:
    let mut pw = PhysicsWorld::new(PhysicsWorldConfig::default());
    for i in 0..512usize {
        let gx = (i % 16) as f64;
        let gz = (i / 16) as f64;
        let mut rb = RigidBody::new(
            format!("s_{i}"),
            Vec3::new(gx * 0.72, 5.0, gz * 0.72),
            1.5,
            0.5,
            0.4,
        )
        .expect("valid sphere");
        rb.velocity = if i == 0 {
            Vec3::new(1.0e9, f64::NAN, -1.0e9)
        } else {
            Vec3::new(if i % 2 == 0 { 1.5 } else { -1.5 }, 0.0, 0.0)
        };
        pw.bodies.push(rb);
    }
    pw.player_capsule = Some(CapsuleController::new(Vec3::new(0.0, 3.0, 0.0)));
    let contacts = pw.resolve_dynamic_body_broadphase();
    assert!(
        contacts >= 256,
        "expected dense 512-sphere lattice contacts, got {contacts}"
    );

    let empty_bvh = TwoLevelBvh::build_solid(&SceneWorld::new(), &[]).expect("empty BVH");
    pw.step_lawful(
        1.0 / 60.0,
        CharacterMoveIntent {
            planar_wish_dir: Vec3::new(f64::NAN, f64::INFINITY, f64::NEG_INFINITY),
            speed_scale: f64::NAN,
            jump_requested: false,
        },
        &empty_bvh,
    )
    .expect("step_lawful with sanitized NaN intent");
    let cap = pw.player_capsule.as_ref().expect("capsule present");
    assert!(
        cap.foot_position.x.is_finite()
            && cap.foot_position.y.is_finite()
            && cap.foot_position.z.is_finite()
    );
    let max_speed = verdant_core::verdant_tensor::kernel_consts().physics_max_speed_mps;
    assert!(pw.bodies[0].velocity.length() <= max_speed + 1.0e-6);

    // 4. 52-bit exact IEEE-754 floating-origin horizon guard (`UniversePosition::new_canonical`):
    let ok_far = UniversePosition::new_canonical(
        UniverseCellCoord::ORIGIN,
        Vec3::new(1.0e9 * 128.0 + 12.5, 0.0, -3.25),
    )
    .expect("sub-horizon astronomical offset");
    assert!(ok_far.bounds_verified);

    let beyond_52 = ((1_u64 << 52) as f64) * 128.0 * 2.0;
    let horizon_err =
        UniversePosition::new_canonical(UniverseCellCoord::ORIGIN, Vec3::new(beyond_52, 0.0, 0.0))
            .expect_err("offset >= 2^52 cells must refuse precision loss");
    assert!(matches!(
        horizon_err,
        NarrativeLawError::InvalidPortalOrUniverseCell { .. }
    ));

    // 5. Multi-primitive Khronos `.glb` export + `import_glb_to_scene` primitive merging:
    let mats = canonical_pbr_material_library();
    let mesh_a = generate_faceted_prism_mesh(0.6, 1.2);
    let mesh_b = generate_fluted_column_mesh(0.4, 0.35, 1.6, 8, 16, 8);
    let expected_tris = mesh_a.triangle_count() + mesh_b.triangle_count();
    let multi_glb = pack_multi_primitive_glb("MultiAsset", &[(&mesh_a, 0), (&mesh_b, 1)], &mats)
        .expect("valid multi-primitive GLB");
    let imported = import_glb_to_scene(&multi_glb).expect("import multi-primitive GLB");
    assert_eq!(imported.meshes.len(), 1);
    assert_eq!(imported.meshes[0].triangle_count(), expected_tris);

    // 6. WebGL2 context recovery, worker frame-ack + RPC watchdogs, & live Wasm stability audit:
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let renderer_mjs = std::fs::read_to_string(repo_root.join("site/scene-renderer.mjs"))
        .expect("site/scene-renderer.mjs");
    assert!(renderer_mjs.contains("webglcontextlost"));
    assert!(renderer_mjs.contains("webglcontextrestored"));
    assert!(renderer_mjs.contains("isContextLost"));

    let worker_mjs = std::fs::read_to_string(repo_root.join("site/engine-worker.mjs"))
        .expect("site/engine-worker.mjs");
    assert!(worker_mjs.contains("frame-dropped"));
    assert!(worker_mjs.contains("WebAssembly.RuntimeError"));

    let card_mjs = std::fs::read_to_string(repo_root.join("site/card.mjs")).expect("site/card.mjs");
    assert!(card_mjs.contains("frame-dropped"));
    assert!(card_mjs.contains("15000"));

    {
        let _abi_guard = wasm_abi_lock();
        assert_eq!(verdant_wasm::verdant_wasm_init(7), 1);
        let aux_len = verdant_wasm::verdant_wasm_stability_audit_aux(512);
        assert!(aux_len > 40);
    }
}

/// `VE-36`: Exclusive post-boot `UORC` wire unification across `WorldSnapshot::encode_uorc`,
/// `#uorc=` Studio URLs, Wasm `snapshot_encode`, GLB-to-UORC transcoding, Stitch `2-of-3` parity
/// custody, stride-8 rollback checkpoints, zero-decode `CodedLodFrameBundle` UORC archives, and CLI.
#[test]
fn engine_ve_36() {
    use std::path::Path;
    use verdant_core::error::{SnapshotCodecError, StitchCustodyError};
    use verdant_core::geometry::generate_faceted_prism_mesh;
    use verdant_core::gltf_io::{pack_multi_primitive_glb, transcode_glb_to_progressive_uorc};
    use verdant_core::network::{recover_uorc_from_stitch_pair, shard_uorc_to_stitch_fragments};
    use verdant_core::render::canonical_pbr_material_library;
    use verdant_core::runtime::{CheckpointedRollbackJournal, WorldSnapshot};
    use verdant_core::streaming::{
        compress_uorc_portable, decompress_uorc_portable, is_uorc_archive, CodedLodFrameBundle,
        StandardSceneState,
    };

    // 1. Core-Wasm `scene:uorc-exclusive` verdict & Lean-proved `universalUorcRuntimeDirect`:
    assert_eq!(
        verdant_core::prism_kernel::universalUorcRuntimeDirect(),
        Ok(true)
    );
    let verdict = verdant_core::prism_kernel::dispatch(b"scene:uorc-exclusive".to_vec())
        .expect("dispatch scene:uorc-exclusive");
    assert_eq!(
        std::str::from_utf8(&verdict).expect("utf8 verdict"),
        "conformant:uorc_exclusive=true,cold_boot_exempt=true,wasm_uorc_encode=true,zero_decode_coded=true,url_hash=uorc"
    );

    // 2. `WorldSnapshot::encode_uorc` compression & transparent `WorldSnapshot::decode` + tamper refusal:
    let world = StandardSceneState::synthesize_at_lod(7, 3).expect("LOD3 world");
    let raw_vdnt = WorldSnapshot::encode(&world).expect("inner VDNT wire");
    let uorc_snap = WorldSnapshot::encode_uorc(&world).expect("UORC snapshot archive");
    assert!(is_uorc_archive(&uorc_snap));
    assert!(
        uorc_snap.len() * 2 < raw_vdnt.len(),
        "UORC snapshot ({}) must significantly compress raw VDNT ({})",
        uorc_snap.len(),
        raw_vdnt.len()
    );
    let restored = WorldSnapshot::decode(&uorc_snap).expect("decode UORC snapshot");
    assert_eq!(restored, world);

    let mut tampered_uorc = uorc_snap.clone();
    let last_idx = tampered_uorc.len() - 1;
    tampered_uorc[last_idx] ^= 0x5A;
    let tamper_err =
        WorldSnapshot::decode(&tampered_uorc).expect_err("tampered UORC seal must be refused");
    assert!(matches!(
        tamper_err,
        SnapshotCodecError::IntegrityDigestMismatch { .. }
    ));

    // 3. Zero-decode `CodedLodFrameBundle` UORC archive (`pack_to_uorc` -> `unpack_from_uorc` -> `contract_zero_decode`):
    let bundle = CodedLodFrameBundle::synthesize(7, 3).expect("LOD3 coded bundle");
    let cdmx_uorc = bundle.pack_to_uorc();
    assert!(is_uorc_archive(&cdmx_uorc));
    let unpacked_bundle =
        CodedLodFrameBundle::unpack_from_uorc(&cdmx_uorc).expect("unpack CDMX from UORC");
    let q_weights = vec![3_i32; bundle.inner_k];
    let exact_ref = bundle
        .contract_zero_decode(&q_weights, &q_weights, &q_weights)
        .expect("reference zero-decode contraction");
    let exact_unpacked = unpacked_bundle
        .contract_zero_decode(&q_weights, &q_weights, &q_weights)
        .expect("unpacked UORC zero-decode contraction");
    assert_eq!(exact_unpacked, exact_ref);

    // 4. Pure-UORC `Stitch` 2-of-3 parity sharding & recovery + tampered inner UORC seal refusal:
    let frags = shard_uorc_to_stitch_fragments(&raw_vdnt).expect("auto-UORC Stitch sharding");
    assert!(frags[0].payload.len() < raw_vdnt.len() / 3);
    let (rec_uorc, rec_plain) =
        recover_uorc_from_stitch_pair(&frags[0], &frags[2]).expect("2-of-3 UORC recovery");
    assert!(is_uorc_archive(&rec_uorc));
    assert_eq!(rec_plain, raw_vdnt);

    let mut bad_plain_uorc = compress_uorc_portable(b"valid-uorc-frame");
    let b_last = bad_plain_uorc.len() - 1;
    bad_plain_uorc[b_last] ^= 0x01;
    let bad_frags =
        verdant_core::network::shard_bytes_to_stitch_fragments(&bad_plain_uorc).expect("shard raw");
    let bad_rec = recover_uorc_from_stitch_pair(&bad_frags[0], &bad_frags[1])
        .expect_err("corrupted inner UORC seal must fail Stitch UORC recovery");
    assert!(matches!(
        bad_rec,
        StitchCustodyError::PayloadDigestMismatch { .. }
    ));

    // 5. UORC-compressed stride-8 `CheckpointedRollbackJournal`:
    let mut rj = CheckpointedRollbackJournal::new_uorc(&raw_vdnt, b"uorc-genesis");
    for step in 1..=12_u8 {
        let mut next_bytes = raw_vdnt.clone();
        next_bytes[0] = next_bytes[0].wrapping_add(step);
        rj.record_uorc_step(1_u8, &[step], &next_bytes);
    }
    assert!(rj.total_checkpoint_uorc_bytes() < raw_vdnt.len());
    let (reconstructed_11, replay_steps, _) = rj
        .reconstruct_uorc_frame(11, |prev, delta| {
            let mut out = prev.to_vec();
            out[0] = out[0].wrapping_add(*delta);
            out
        })
        .expect("reconstruct UORC frame 11");
    assert_eq!(replay_steps, 3);
    assert_eq!(reconstructed_11[0], raw_vdnt[0].wrapping_add(11));

    // 6. CLI `compress-uorc` / `decompress-uorc` + Wasm `snapshot_encode` (`b"UORC"`) & Studio `#uorc=` wiring:
    let tmp = std::env::temp_dir().join("verdant_ve36_uorc_cli");
    let _ = std::fs::create_dir_all(&tmp);
    let raw_path = tmp.join("sample.bin");
    let uorc_path = tmp.join("sample.uorc");
    let out_path = tmp.join("sample.out");
    std::fs::write(&raw_path, &raw_vdnt).expect("write raw sample");
    let comp_out = verdant_cli::run_cli(&[
        "compress-uorc".to_string(),
        raw_path.to_string_lossy().into_owned(),
        uorc_path.to_string_lossy().into_owned(),
    ])
    .expect("CLI compress-uorc");
    assert!(comp_out.contains("UORC COMPRESSED OK"));
    let decomp_out = verdant_cli::run_cli(&[
        "decompress-uorc".to_string(),
        uorc_path.to_string_lossy().into_owned(),
        out_path.to_string_lossy().into_owned(),
    ])
    .expect("CLI decompress-uorc");
    assert!(decomp_out.contains("UORC DECOMPRESSED OK"));
    assert_eq!(std::fs::read(&out_path).expect("read out"), raw_vdnt);
    let _ = decompress_uorc_portable(&std::fs::read(&uorc_path).expect("read uorc"))
        .expect("portable decompress");
    let _ = std::fs::remove_dir_all(&tmp);

    let mats = canonical_pbr_material_library();
    let pm = generate_faceted_prism_mesh(0.5, 1.0);
    let glb = pack_multi_primitive_glb("UorcGlb", &[(&pm, 0)], &mats).expect("pack glb");
    let glb_uorc = transcode_glb_to_progressive_uorc(&glb, 7).expect("transcode glb to uorc");
    assert!(is_uorc_archive(&glb_uorc));

    {
        let _abi_guard = wasm_abi_lock();
        assert_eq!(verdant_wasm::verdant_wasm_init(7), 1);
        let wasm_snap_len = verdant_wasm::verdant_wasm_snapshot_encode();
        assert!(wasm_snap_len > 36 && (wasm_snap_len as usize) < raw_vdnt.len() / 2);
        assert_eq!(verdant_wasm::verdant_wasm_io_get_byte(0), u32::from(b'U'));
        assert_eq!(verdant_wasm::verdant_wasm_io_get_byte(1), u32::from(b'O'));
        assert_eq!(verdant_wasm::verdant_wasm_io_get_byte(2), u32::from(b'R'));
        assert_eq!(verdant_wasm::verdant_wasm_io_get_byte(3), u32::from(b'C'));
        assert_eq!(verdant_wasm::verdant_wasm_snapshot_decode_and_restore(), 1);
        assert!(verdant_wasm::verdant_wasm_uorc_exclusive_audit_aux() > 40);
    }

    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let studio_mjs =
        std::fs::read_to_string(repo_root.join("site/studio.mjs")).expect("site/studio.mjs");
    let studio_media_mjs = std::fs::read_to_string(repo_root.join("site/studio-media.mjs"))
        .expect("site/studio-media.mjs");
    assert!(studio_mjs.contains("#uorc=") || studio_media_mjs.contains("#uorc="));
    let card_mjs = std::fs::read_to_string(repo_root.join("site/card.mjs")).expect("site/card.mjs");
    assert!(card_mjs.contains(".uorc"));
}

/// Counts packed vertices per `GpuSurfaceClass` index (`[opaque, fluid, metal, emissive, invalid]`).
fn surface_class_histogram(colors_rgb: &[f32]) -> [usize; 5] {
    use verdant_core::render::GpuSurfaceClass;
    let mut hist = [0usize; 5];
    for rgb in colors_rgb.chunks_exact(3) {
        let (class, albedo_r) = GpuSurfaceClass::split_red(rgb[0]);
        assert!(
            (0.0..1.0).contains(&albedo_r),
            "albedo red must decode into [0, 1)"
        );
        assert!((0.0..=1.0).contains(&rgb[1]) && (0.0..=1.0).contains(&rgb[2]));
        hist[(class as usize).min(4)] += 1;
    }
    hist
}

/// `VE-37`: Explicit surface differentiation and artifact-free shading. Every packed vertex
/// carries a material-derived `GpuSurfaceClass` (never inferred from hue); dynamic actors keep
/// per-attribute GPU stream capacity so characters render with their own colours; and
/// screen-space derivatives run in uniform control flow with a continuous detail fade so no
/// seam line appears across the terrain.
#[test]
fn engine_ve_37() {
    use std::path::Path;
    use verdant_core::character::{ArticulatedCharacter, CharacterArchetype};
    use verdant_core::hd_world::synthesize_hd_citadel_sector;
    use verdant_core::render::{
        canonical_pbr_material_library, pack_world_meshes_to_buffers_opts, GpuSurfaceClass,
        PackedSceneRenderBuffers,
    };
    use verdant_core::streaming::TerrainReliefParams;
    use verdant_wasm::WasmRuntimeBridge;

    // 1. Classes come from physical material properties, not colour:
    let lib = canonical_pbr_material_library();
    let classes: Vec<_> = lib.iter().map(GpuSurfaceClass::from_material).collect();
    assert_eq!(
        classes,
        vec![
            GpuSurfaceClass::OpaqueDielectric, // TerrainAggregate
            GpuSurfaceClass::Fluid,            // DielectricFluid
            GpuSurfaceClass::EmissiveGlass,    // CrystallineDielectric
            GpuSurfaceClass::OpaqueDielectric, // SubsurfaceCanopy
            GpuSurfaceClass::OpaqueDielectric, // ArchitecturalStone
        ]
    );
    let mut gold = lib[4].clone();
    gold.metallic_factor = 1.0;
    assert_eq!(
        GpuSurfaceClass::from_material(&gold),
        GpuSurfaceClass::Metal
    );

    // 2. Wire encoding round-trips every class, including saturated albedo:
    for class in [
        GpuSurfaceClass::OpaqueDielectric,
        GpuSurfaceClass::Fluid,
        GpuSurfaceClass::Metal,
        GpuSurfaceClass::EmissiveGlass,
    ] {
        for r in [0.0_f32, 0.5, 1.0] {
            let enc = class.encode([r, 0.25, 1.0]);
            let (idx, albedo_r) = GpuSurfaceClass::split_red(enc[0]);
            assert_eq!(idx, class as u32);
            assert!(albedo_r < 1.0 && (albedo_r - r).abs() < 0.01);
        }
    }

    // 3. Static HD sector: only the basin water is fluid; terrain/stone/foliage stay opaque:
    let sector =
        synthesize_hd_citadel_sector(7, &TerrainReliefParams::standard_basin_valley(), -1.35, 2);
    let packed = pack_world_meshes_to_buffers_opts(&sector.scene, &sector.meshes, &lib, false)
        .expect("pack HD sector");
    let hist = surface_class_histogram(&packed.colors_rgb);
    assert_eq!(hist[1], 4, "exactly the four water-quad vertices are fluid");
    assert_eq!(hist[2] + hist[3] + hist[4], 0);
    assert!(hist[0] > 1000);

    // 4. Characters: armour is opaque (never water), the visor is emissive glass:
    for archetype in [
        CharacterArchetype::Explorer,
        CharacterArchetype::Archivist,
        CharacterArchetype::BridgeEngineer,
        CharacterArchetype::ObservatoryKeeper,
    ] {
        let ch = ArticulatedCharacter::new_humanoid("probe", archetype, Vec3::ZERO, 2)
            .expect("humanoid");
        let mut out = PackedSceneRenderBuffers::default();
        ch.append_skinned_to_buffers_opts(&mut out, false, false)
            .expect("skin");
        let h = surface_class_histogram(&out.colors_rgb);
        assert_eq!(
            h[1] + h[2] + h[4],
            0,
            "{archetype:?} has no fluid/metal/invalid"
        );
        assert!(h[3] > 0, "{archetype:?} visor is emissive glass");
        assert!(h[0] > h[3], "{archetype:?} body is opaque dielectric");
    }

    // 5. Live bridge: dynamic actors are differentiated and mirrored consistently:
    let mut bridge = WasmRuntimeBridge::new(7).expect("bridge");
    bridge.set_lod_rank(2).expect("lod2");
    bridge.rebuild_scene_buffers(0.0).expect("rebuild");
    let dyn_hist = surface_class_histogram(&bridge.dynamic_buffers.colors_rgb);
    assert_eq!(
        dyn_hist[1] + dyn_hist[4],
        0,
        "no dynamic actor renders as water"
    );
    assert!(dyn_hist[0] > 0 && dyn_hist[2] > 0 && dyn_hist[3] > 0);
    let scene_hist = surface_class_histogram(&bridge.scene_buffers.colors_rgb);
    assert_eq!(scene_hist[1], 4);
    bridge.scene_buffers.ensure_tile_triangles_built();
    assert!(bridge
        .scene_buffers
        .tile_triangles
        .iter()
        .all(|t| t.base_color.iter().all(|c| (0.0..=1.0).contains(c))));

    // 6. WebGL2 shader + stream contracts:
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let renderer = std::fs::read_to_string(repo_root.join("site/scene-renderer.mjs"))
        .expect("site/scene-renderer.mjs");
    assert!(renderer.contains("flat out float v_m") && renderer.contains("flat in float v_m"));
    assert!(renderer.contains("v_m==1.0") && renderer.contains("v_m==2.0"));
    assert!(!renderer.contains("v_col.r*1.35"), "no hue-inferred water");
    assert!(
        !renderer.contains("d<30.0"),
        "no per-pixel branch around derivatives"
    );
    assert!(renderer.contains("smoothstep(22.0,30.0,d)"));
    for cap in ["\"capPos\"", "\"capNorm\"", "\"capCol\"", "\"capIdx\""] {
        assert!(renderer.contains(cap), "dynamic stream capacity {cap}");
    }
}

/// `VE-38`: Imported Schema.org, W3C SOSA and PROV-O, Khronos glTF 2.0 HD PBR and skeletal skinning, W3C naga WebGL2 GLSL ES 3.00 transpilation, OMG DMN 1.5 model admission, and multi-domain world catalogs validate against SHA-256-locked external authority oracles while prohibiting external model runtimes outside Verdant Engine
#[test]
fn engine_ve_38() {
    use std::collections::{BTreeMap, BTreeSet};
    use std::path::Path;
    use verdant_core::error::OracleValidationError;
    use verdant_core::gltf_io::{
        export_hd_scene_to_glb, export_skinned_character_to_glb, import_glb_to_scene,
        validate_glb_bytes,
    };
    use verdant_core::render::{
        transpile_builtin_engine_shaders_webgl2, transpile_wgsl_to_webgl2_glsl, ShaderStageKind,
        WGSL_PBR_SURFACE_SHADER,
    };
    use verdant_core::standards::{CatalogBundleRegistry, DmnAdmissionInput};

    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let registry = CatalogBundleRegistry::load_from_repo_dir(&repo_root)
        .expect("all 10 SHA-256-locked authority catalogs load and cross-validate");
    assert_eq!(registry.file_locks.len(), 10);
    assert_eq!(registry.domain_packs.domains.len(), 4);
    assert_eq!(registry.khronos_hd.materials.len(), 8);
    assert_eq!(registry.khronos_hd.rigs.len(), 4);
    assert_eq!(registry.language_dmn.intent_routes.len(), 16);

    // Embedded compile-time registry matches disk registry:
    let embedded = CatalogBundleRegistry::embedded().expect("embedded Wasm-ready catalog registry");
    assert_eq!(embedded.file_locks, registry.file_locks);

    // 1. Instantiate all 4 differentiated domain packs (`Sunspire`, `Meridian`, `Glasshouse`, `Starfall`)
    // and verify distinct seeds, distinct Sabine acoustics, distinct conserved items, and live Schema.org + SOSA/PROV-O validity:
    let mut seeds = BTreeSet::new();
    let mut rt60_buckets = BTreeSet::new();
    for slug in [
        "sunspire-regatta",
        "meridian-observatory",
        "glasshouse-chronicle",
        "starfall-bastion",
    ] {
        let pack = registry
            .instantiate_domain_pack(slug, Some(2))
            .unwrap_or_else(|e| panic!("failed to instantiate domain `{slug}`: {e}"));
        assert!(
            seeds.insert(pack.spec.seed),
            "each domain pack must have a distinct seed"
        );
        let rt60_ms = (pack.acoustic_environment.sabine_rt60_seconds() * 1000.0).round() as i64;
        rt60_buckets.insert(rt60_ms);
        assert!(!pack.characters.is_empty());
        assert!(pack.item_ledger.total_supply() > 0);

        let schema_rep = registry
            .schema_org
            .validate_schema_org_jsonld(&pack.schema_org_jsonld)
            .expect("domain Schema.org JSON-LD graph conforms");
        assert!(schema_rep.entity_count >= 5);

        let sosa_rep = registry
            .sosa_provo
            .validate_sosa_provo_turtle(&pack.sosa_provo_turtle)
            .expect("domain W3C SOSA/PROV-O Turtle graph conforms");
        assert_eq!(sosa_rep.status, "conforms");
        assert_eq!(sosa_rep.observation_count, 1);
        assert_eq!(sosa_rep.provenance_entity_count, 2);

        // Export HD domain scene with ratified `KHR_materials_*` extensions and verify via Khronos `gltf`:
        let scene_glb =
            export_hd_scene_to_glb(&pack.sector.scene, &pack.sector.meshes, &pack.hd_materials)
                .expect("export HD scene GLB with Khronos PBR extensions");
        let glb_summary = validate_glb_bytes(&scene_glb).expect("Khronos glTF 2.0 oracle");
        assert!(glb_summary.mesh_count >= 8);
    }
    assert_eq!(
        rt60_buckets.len(),
        4,
        "all 4 domains must exhibit distinct Sabine acoustic reverberation profiles"
    );

    // 2. Export 22-joint Skinned Character to `.glb` (`skins`, `inverseBindMatrices`, `JOINTS_0`, `WEIGHTS_0`, `COLOR_0`)
    // and round-trip through `import_glb_to_scene` (`gltf::import_slice` oracle):
    let meridian = registry
        .instantiate_domain_pack("meridian-observatory", Some(2))
        .expect("meridian pack");
    let lyra = &meridian.characters[0];
    let skinned_glb = export_skinned_character_to_glb(lyra, &meridian.hd_materials[2])
        .expect("export 22-joint skinned character GLB");
    let reimported = import_glb_to_scene(&skinned_glb).expect("reimport skinned character GLB");
    assert_eq!(reimported.skeletons.len(), 1);
    assert_eq!(reimported.skeletons[0].joints.len(), 22);
    assert_eq!(
        reimported.skin_weights[0].len(),
        lyra.bind_mesh.vertex_count(),
        "every vertex round-tripped normalized 4-joint LBS weights"
    );

    // 3. Authoritative W3C `naga` WGSL -> WebGL2 GLSL ES 3.00 (`#version 300 es`) transpilation:
    let transpiled =
        transpile_builtin_engine_shaders_webgl2().expect("transpile all 4 WGSL shaders to WebGL2");
    assert_eq!(transpiled.len(), 8);
    for shader in &transpiled {
        assert!(
            shader.glsl_source.starts_with("#version 300 es"),
            "shader `{}` ({:?}) must emit WebGL2 `#version 300 es`",
            shader.shader_label,
            shader.stage
        );
        assert!(shader.glsl_source.contains("void main()"));
    }

    // 4. OMG DMN 1.5 `FIRST`-hit model admission & full-repo Verdant runtime exclusivity audit:
    let admitted = registry
        .language_dmn
        .evaluate_dmn_admission(&DmnAdmissionInput {
            model_format: "verdant-uorc".to_string(),
            observer_halted: false,
            source_locked: true,
            route_granted: true,
            runtime_state: "match".to_string(),
            weights_state: "match".to_string(),
        })
        .expect("admitted Verdant UORC model");
    assert_eq!(admitted.rule_id, "R8-ADMIT-VERDANT");
    assert_eq!(admitted.outcome, "run");

    let ask_route = registry
        .language_dmn
        .evaluate_dmn_admission(&DmnAdmissionInput {
            model_format: "verdant-coded-matrix".to_string(),
            observer_halted: false,
            source_locked: true,
            route_granted: false,
            runtime_state: "match".to_string(),
            weights_state: "match".to_string(),
        })
        .expect("ungranted route asks FK-07 consent");
    assert_eq!(ask_route.outcome, "ask");

    for rel_manifest in [
        "package.json",
        "Cargo.toml",
        "crates/verdant-core/Cargo.toml",
        "crates/verdant-wasm/Cargo.toml",
        "site/card.mjs",
        "site/studio.mjs",
        "site/engine-worker.mjs",
        "templates/browser-3d-starter/Cargo.toml",
        "templates/browser-3d-starter/game.mjs",
    ] {
        let text = std::fs::read_to_string(repo_root.join(rel_manifest)).expect(rel_manifest);
        registry
            .language_dmn
            .verify_verdant_runtime_exclusivity(rel_manifest, &text)
            .unwrap_or_else(|e| panic!("runtime exclusivity failed on `{rel_manifest}`: {e}"));
    }

    // --- R5 Negative Refusal Assertions (`OracleValidationError`) ---

    // Negative 1: Tampered catalog bytes rejected by SHA-256 lock (`CatalogIntegrityMismatch`):
    let auth_toml = std::fs::read_to_string(repo_root.join("model/authorities.toml"))
        .expect("authorities.toml");
    let mut bad_files: BTreeMap<&str, &[u8]> = BTreeMap::new();
    bad_files.insert(
        "catalogs/schema-org/vocabulary.json",
        b"{\"tampered\":true}",
    );
    assert!(matches!(
        CatalogBundleRegistry::from_locked_bytes(&auth_toml, &bad_files),
        Err(OracleValidationError::CatalogIntegrityMismatch { .. })
    ));

    // Negative 2: Invalid Schema.org property domain (`gameItem` on `Person`) refused (`SchemaOrgInvalid`):
    let bad_schema = serde_json::json!({
        "@context": "https://schema.org",
        "@type": "Person",
        "name": "InvalidPerson",
        "gameItem": "Sextant"
    });
    assert!(matches!(
        registry.schema_org.validate_schema_org_jsonld(&bad_schema),
        Err(OracleValidationError::SchemaOrgInvalid { .. })
    ));

    // Negative 3: W3C SOSA Observation missing `sosa:madeBySensor` refused (`SosaProvoConstraintViolated`):
    let missing_sensor_ttl = "@prefix sosa: <http://www.w3.org/ns/sosa/> .\n\
        <urn:obs:1> a sosa:Observation ; sosa:hasFeatureOfInterest <urn:place:1> ; sosa:hasSimpleResult \"42\" .";
    assert!(matches!(
        registry
            .sosa_provo
            .validate_sosa_provo_turtle(missing_sensor_ttl),
        Err(OracleValidationError::SosaProvoConstraintViolated { .. })
    ));

    // Negative 4: Cyclic W3C PROV-O `prov:wasDerivedFrom` chain (`A -> B -> A`) refused (`SosaProvoConstraintViolated`):
    let cyclic_prov_ttl = "@prefix prov: <http://www.w3.org/ns/prov#> .\n\
        <urn:ent:a> a prov:Entity ; prov:wasDerivedFrom <urn:ent:b> .\n\
        <urn:ent:b> a prov:Entity ; prov:wasDerivedFrom <urn:ent:a> .";
    assert!(matches!(
        registry
            .sosa_provo
            .validate_sosa_provo_turtle(cyclic_prov_ttl),
        Err(OracleValidationError::SosaProvoConstraintViolated { .. })
    ));

    // Negative 5: Unknown shader entry point refused by `naga` WebGL2 transpiler (`GlslTranspilationFailed`):
    assert!(matches!(
        transpile_wgsl_to_webgl2_glsl(
            "pbr_surface.wgsl",
            WGSL_PBR_SURFACE_SHADER,
            "nonexistent_entry",
            ShaderStageKind::Vertex
        ),
        Err(OracleValidationError::GlslTranspilationFailed { .. })
    ));

    // Negative 6: Prohibited external ONNX / Transformers runtime refused by DMN & manifest scanner (`ProhibitedExternalModelRuntime`):
    assert!(matches!(
        registry
            .language_dmn
            .evaluate_dmn_admission(&DmnAdmissionInput {
                model_format: "onnx".to_string(),
                observer_halted: false,
                source_locked: true,
                route_granted: true,
                runtime_state: "match".to_string(),
                weights_state: "match".to_string(),
            }),
        Err(OracleValidationError::ProhibitedExternalModelRuntime { .. })
    ));
    assert!(matches!(
        registry.language_dmn.verify_verdant_runtime_exclusivity(
            "planted-bypass.mjs",
            "import { pipeline } from '@huggingface/transformers';"
        ),
        Err(OracleValidationError::ProhibitedExternalModelRuntime { .. })
    ));
}

/// `VE-39`: Second-order Observer Prism holon, five platform organs, seven stakeholder Prism models, arbitrary Verdant model envelopes, native 16-route intent & ISO 24617-2 passage-grounded NPC dialogue with Unknown abstention, live NavMesh NPC directives, and five W3C SOSA/PROV-O observed communication channels
#[test]
fn engine_ve_39() {
    use std::collections::BTreeSet;
    use verdant_core::cybernetics::{CodedSlmDirector, ContainerGrantMatrix, ContainerOrgan};
    use verdant_core::ecs::TwoLevelBvh;
    use verdant_core::error::ObserverModelError;
    use verdant_core::network::PrismIdSessionEnvelope;
    use verdant_core::observer::{
        sha256_payload_digest, ArbitraryModelDomainKind, CommunicationChannelKind,
        ObserverOrganKind, ObserverPrismHolon, VerdantLanguageEngine, VerdantModelEnvelope,
    };
    use verdant_core::prism_kernel;

    // 1. Verify Prism kernel Observer theorems & basis-prime product (`2*3*5*7*11 = 2310`):
    assert_eq!(prism_kernel::observerOrganBasisProduct(), Ok(2310));
    assert!(prism_kernel::dmnModelRouteAdmitted(
        true, true, true, true, true, true
    ));
    assert!(!prism_kernel::dmnModelRouteAdmitted(
        true, true, false, true, true, true
    ));

    // 2. Construct ObserverPrismHolon and verify strong distinct 32-byte `kappa` across Observer + 5 Organs + 7 Stakeholders:
    let mut holon = ObserverPrismHolon::new("sunspire-regatta").expect("ObserverPrismHolon init");
    assert_eq!(holon.organ_basis_product, 2310);
    assert_eq!(holon.organs.len(), 5);
    assert_eq!(holon.stakeholders.len(), 7);

    let mut distinct_kappas = BTreeSet::new();
    assert!(distinct_kappas.insert(holon.observer_kappa_hex.clone()));
    for organ in &holon.organs {
        assert!(
            distinct_kappas.insert(organ.kappa_hex.clone()),
            "every platform organ must have a distinct kappa identity"
        );
    }
    let mut workspace_bijection = BTreeSet::new();
    for sh in &holon.stakeholders {
        assert!(
            distinct_kappas.insert(sh.kappa_hex.clone()),
            "every stakeholder Prism model must have a distinct kappa identity"
        );
        assert!(
            workspace_bijection.insert(sh.workspace_index),
            "stakeholder models must map bijectively onto the 7 Studio workspaces"
        );
    }
    assert_eq!(workspace_bijection.len(), 7);
    assert_eq!(distinct_kappas.len(), 1 + 5 + 7);

    // 3. Extrapolate to arbitrary Verdant Prism model envelopes across all 7 domain families:
    let mut grants = ContainerGrantMatrix::default();
    grants.grant(ContainerOrgan::AudioWorkletSynth);
    for (idx, (fam, fmt)) in [
        (ArbitraryModelDomainKind::Geometry3D, "verdant-uorc"),
        (
            ArbitraryModelDomainKind::CodedLanguage,
            "verdant-coded-matrix",
        ),
        (
            ArbitraryModelDomainKind::NeuromorphicSsm,
            "verdant-coded-matrix",
        ),
        (
            ArbitraryModelDomainKind::ConservationAutomaton,
            "verdant-holo",
        ),
        (ArbitraryModelDomainKind::SpatialAcoustics, "verdant-uorc"),
        (ArbitraryModelDomainKind::TropicalUi, "verdant-holo"),
        (ArbitraryModelDomainKind::RigidBodyPhysics, "verdant-uorc"),
    ]
    .into_iter()
    .enumerate()
    {
        let env = VerdantModelEnvelope::new_sealed(
            format!("PRISM-ARB-{idx}"),
            fam,
            ObserverOrganKind::Engine,
            ContainerOrgan::WebAssemblyCore,
            fmt,
            format!("canonical-verdant-model-payload-{idx}").into_bytes(),
            serde_json::json!({
                "@context": "https://schema.org",
                "@id": format!("urn:verdant:arb:{idx}"),
                "@type": "CreativeWork",
                "name": format!("ArbitraryVerdantModel_{idx}"),
                "encodingFormat": fmt
            }),
        );
        let k_hex = holon
            .admit_arbitrary_prism_model(&env, &grants)
            .expect("admit arbitrary Verdant Prism model");
        assert!(distinct_kappas.insert(k_hex));
    }
    assert_eq!(holon.admitted_model_kappas.len(), 7);

    // 4. Native 16-route `CodedSlmDirector` (`uor-matmul` `Packed<Grid<16>, 2>`):
    let slm = CodedSlmDirector::new();
    for (prompt, expected_slug) in [
        ("focus observer core gimbal", "focus-observer"),
        ("drive 4-wheel suspension rover", "drive-vehicle"),
        ("switch to 16x16 software tile raster", "render-tile-wasm"),
        ("enable webgl2 hardware pbr shader", "render-webgl2"),
        ("stream lossless uorc lod3", "stream-uorc-lod3"),
        ("subdivide ultra lod4 masonry", "stream-uorc-lod4"),
        ("sync stitch parity shards", "stitch-shard-sync"),
        ("inspect wcag ui contrast layout", "inspect-formal-ui"),
        ("step xpbd pendulum physics impulse", "step-xpbd-physics"),
        (
            "open producer dashboard traceability",
            "open-role-dashboard",
        ),
        ("scaffold new rust starter code tar", "open-role-code"),
        ("open scene outliner brand tokens", "open-role-design"),
        ("inspect sabine reverb room acoustics", "open-role-audio"),
        ("open cinematography camera director", "open-role-director"),
        ("open gameplay tournament npc patrol", "open-role-gameplay"),
        ("interact chronicle archivist quest", "narrative-interact"),
    ] {
        let prop = slm
            .route_prompt(prompt, None, &grants)
            .unwrap_or_else(|e| panic!("failed routing `{prompt}`: {e}"));
        assert_eq!(prop.intent.slug(), expected_slug, "prompt `{prompt}`");
    }

    // 5. Passage-grounded NPC Q&A, `"Unknown"` abstention, live NavMesh A* patrol, & 5-channel communications:
    let origin = "https://afflom.github.io";
    let q_grounded = "How many Brass Tidal Sextants does Captain Soren hold in the Harbor Vault?";
    let q_digest = sha256_payload_digest(q_grounded.as_bytes());
    let q_sess = PrismIdSessionEnvelope::sign_deterministic(
        "prism:player-alpha",
        b"device-secret-alpha",
        origin,
        1_000,
        2_000,
        &q_digest,
    );
    let grounded_turn = holon
        .converse_player_with_npc(
            "urn:verdant:player:alpha",
            (&q_sess, origin, 1_100),
            0,
            q_grounded,
        )
        .expect("grounded NPC question");
    assert!(!grounded_turn.abstained_unknown);
    assert!(grounded_turn.matched_evidence_facts >= 1);
    assert!(grounded_turn
        .reply_text
        .contains("three Brass Tidal Sextants"));
    assert!(!grounded_turn.cited_fact_ids.is_empty());

    // Unstated fact MUST deterministically abstain with `"Unknown"` (zero hallucination):
    let q_unstated = "What is Captain Soren's favorite pizza topping on Mars?";
    let u_digest = sha256_payload_digest(q_unstated.as_bytes());
    let u_sess = PrismIdSessionEnvelope::sign_deterministic(
        "prism:player-alpha",
        b"device-secret-alpha",
        origin,
        1_000,
        2_000,
        &u_digest,
    );
    let abstained_turn = holon
        .converse_player_with_npc(
            "urn:verdant:player:alpha",
            (&u_sess, origin, 1_100),
            0,
            q_unstated,
        )
        .expect("unstated question abstains cleanly");
    assert!(abstained_turn.abstained_unknown);
    assert_eq!(abstained_turn.response_act_code, 7);
    assert_eq!(abstained_turn.reply_text, "Unknown");
    assert!(abstained_turn.cited_fact_ids.is_empty());

    // Actionable directive request + live A* NavMesh corridor locomotion:
    let req_dir = "Calibrate Fresnel Beacon";
    let r_digest = sha256_payload_digest(req_dir.as_bytes());
    let r_sess = PrismIdSessionEnvelope::sign_deterministic(
        "prism:player-alpha",
        b"device-secret-alpha",
        origin,
        1_000,
        2_000,
        &r_digest,
    );
    let dir_turn = holon
        .converse_player_with_npc(
            "urn:verdant:player:alpha",
            (&r_sess, origin, 1_100),
            0,
            req_dir,
        )
        .expect("directive request");
    assert!(dir_turn.world_mutated);
    assert_eq!(dir_turn.mutated_directive_index, Some(1));

    let bvh = TwoLevelBvh::build_solid(
        &holon.active_domain.sector.scene,
        &holon.active_domain.sector.meshes,
    )
    .expect("domain TwoLevelBvh");
    for step in 0..30 {
        holon.step_npc_patrols(1.0 / 30.0, f64::from(step) / 30.0, Vec3::ZERO, &bvh);
    }
    assert!(
        holon.npc_agents[0].distance_walked_meters > 0.5,
        "NPC walked along its A* NavMesh corridor: {}m",
        holon.npc_agents[0].distance_walked_meters
    );

    // Player <-> Player signed communication channel:
    let peer_msg = "Meet at the East Arch Abutment before high tide";
    let peer_digest = sha256_payload_digest(peer_msg.as_bytes());
    let peer_sess = PrismIdSessionEnvelope::sign_deterministic(
        "prism:player-alpha",
        b"device-secret-alpha",
        origin,
        1_000,
        2_000,
        &peer_digest,
    );
    let peer_rec = holon
        .observe_communication(
            CommunicationChannelKind::PlayerToPlayer,
            "urn:verdant:player:alpha",
            "urn:verdant:player:beta",
            "en",
            0,
            peer_msg,
            Some((&peer_sess, origin, 1_200)),
            1,
        )
        .expect("PlayerToPlayer signed channel");
    assert!(peer_rec.prism_signed);

    // Verify all 5 communication channel kinds were experienced & recorded by the Observer:
    let observed_kinds: BTreeSet<CommunicationChannelKind> =
        holon.communications.iter().map(|c| c.channel).collect();
    assert_eq!(observed_kinds.len(), 5);

    // 6. Verify C-ABI exports on `verdant_wasm` (`load_domain_pack`, `set_npc_directive`, `dialogue_turn_io`, `observer_status_aux`):
    {
        let _wasm_guard = wasm_abi_lock();
        assert_eq!(verdant_wasm::verdant_wasm_init(7), 1);
        let status_len = verdant_wasm::verdant_wasm_load_domain_pack(1);
        assert!(status_len > 100);
        let dir_len = verdant_wasm::verdant_wasm_set_npc_directive(0, 1);
        assert!(dir_len > 10);
        verdant_wasm::verdant_wasm_io_clear();
        for &b in b"What is the Sabine reverberation time of the Vaulted Stone Rotunda?" {
            verdant_wasm::verdant_wasm_io_push_byte(u32::from(b));
        }
        let reply_len = verdant_wasm::verdant_wasm_dialogue_turn_io(0);
        assert!(reply_len > 40);
        assert!(verdant_wasm::verdant_wasm_observer_status_aux() > 100);
    }

    // --- R5 Negative Refusal Assertions (`ObserverModelError`) ---

    // Negative 1: Non-Verdant external `onnx` model envelope refused (`InvalidPrismModelEnvelope`):
    let bad_onnx_env = VerdantModelEnvelope::new_sealed(
        "EXT-ONNX-BYPASS",
        ArbitraryModelDomainKind::CodedLanguage,
        ObserverOrganKind::Engine,
        ContainerOrgan::WebAssemblyCore,
        "onnx",
        b"external-onnx-weights".to_vec(),
        serde_json::json!({
            "@context": "https://schema.org",
            "@type": "CreativeWork",
            "name": "ExternalOnnxModel"
        }),
    );
    assert!(matches!(
        holon.admit_arbitrary_prism_model(&bad_onnx_env, &grants),
        Err(ObserverModelError::InvalidPrismModelEnvelope { .. })
    ));

    // Negative 2: Forged `kappa` digest refused (`InvalidPrismModelEnvelope`):
    let mut forged_env = VerdantModelEnvelope::new_sealed(
        "FORGED-KAPPA",
        ArbitraryModelDomainKind::Geometry3D,
        ObserverOrganKind::Engine,
        ContainerOrgan::WebAssemblyCore,
        "verdant-uorc",
        b"authentic-payload".to_vec(),
        serde_json::json!({
            "@context": "https://schema.org",
            "@type": "CreativeWork",
            "name": "ForgedModel"
        }),
    );
    forged_env.kappa[0] ^= 0xFF;
    assert!(matches!(
        holon.admit_arbitrary_prism_model(&forged_env, &grants),
        Err(ObserverModelError::InvalidPrismModelEnvelope { .. })
    ));

    // Negative 3: Ungrounded factual assertion (`matched == 0 && !abstained`) refused (`EpistemicHallucinationRefused`):
    assert!(matches!(
        VerdantLanguageEngine::verify_dialogue_turn_claim(
            "CHAR-MARINER-CAPTAIN",
            0,
            0,
            false,
            false
        ),
        Err(ObserverModelError::EpistemicHallucinationRefused { .. })
    ));

    // Negative 4: Unsigned `PlayerToPlayer` message refused (`CommunicationEnvelopeRejected`):
    assert!(matches!(
        holon.observe_communication(
            CommunicationChannelKind::PlayerToPlayer,
            "urn:verdant:player:alpha",
            "urn:verdant:player:beta",
            "en",
            0,
            "Unsigned spoofed peer message",
            None,
            0,
        ),
        Err(ObserverModelError::CommunicationEnvelopeRejected { .. })
    ));

    // Negative 5: Out-of-bounds NPC directive refused (`DirectiveOutOfBounds`):
    assert!(matches!(
        holon.set_npc_directive(0, 99),
        Err(ObserverModelError::DirectiveOutOfBounds { .. })
    ));

    // Negative 6: Unknown domain pack slug refused (`UnknownDomainOrCharacter`):
    assert!(matches!(
        holon.switch_domain_pack("nonexistent-universe-999", None),
        Err(ObserverModelError::UnknownDomainOrCharacter { .. })
    ));
}

#[test]
fn engine_ve_40() {
    use verdant_core::error::ObserverModelError;
    use verdant_core::observer::ObserverPrismHolon;
    use verdant_core::standards::CatalogBundleRegistry;
    use verdant_wasm::{
        verdant_wasm_active_lod_rank, verdant_wasm_active_seed, verdant_wasm_aux_len,
        verdant_wasm_aux_ptr, verdant_wasm_dialogue_turn_io, verdant_wasm_init,
        verdant_wasm_io_clear, verdant_wasm_io_push_byte, verdant_wasm_load_domain_pack,
        verdant_wasm_observer_status_aux, verdant_wasm_set_lod_rank,
        verdant_wasm_set_npc_directive,
    };

    let root = repo_root();

    // 1. Verify Studio modules have zero JS regex prompt overrides (`effectiveSlug =`) and wire live Wasm NPC/domain/dialogue RPCs:
    let mut studio_js = String::new();
    for rel in [
        "site/studio.mjs",
        "site/studio-gameplay-panel.mjs",
        "site/studio-director-panel.mjs",
        "site/studio-media.mjs",
    ] {
        studio_js.push_str(
            &std::fs::read_to_string(root.join(rel)).unwrap_or_else(|e| panic!("read {rel}: {e}")),
        );
        studio_js.push('\n');
    }
    assert!(
        !studio_js.contains("effectiveSlug"),
        "routeStudioPrompt must dispatch purely on Wasm CodedSlmDirector slug without JS regex overrides"
    );
    for required_rpc in [
        "load-domain-pack",
        "set-npc-directive",
        "npc-dialogue",
        "observer-status",
    ] {
        assert!(
            studio_js.contains(required_rpc),
            "Studio modules missing live Wasm Observer/Catalog RPC {required_rpc}"
        );
    }

    // 2. Verify Worker modules wire all 5 Observer/Catalog Wasm C-ABI calls:
    let mut worker_js = String::new();
    for rel in [
        "site/engine-worker.mjs",
        "site/engine-worker-codec.mjs",
        "site/engine-worker-extended.mjs",
    ] {
        worker_js.push_str(
            &std::fs::read_to_string(root.join(rel)).unwrap_or_else(|e| panic!("read {rel}: {e}")),
        );
        worker_js.push('\n');
    }
    for required_sym in [
        "verdant_wasm_load_domain_pack",
        "verdant_wasm_set_npc_directive",
        "verdant_wasm_dialogue_turn_io",
        "verdant_wasm_observer_status_aux",
        "verdant_wasm_observe_comm_io",
    ] {
        assert!(
            worker_js.contains(required_sym),
            "Worker modules missing Wasm export call {required_sym}"
        );
    }

    // 3. Verify Schema.org JSON-LD metadata on `/studio/`, `/catalogs/`, and `/demo/` against `CatalogBundleRegistry`:
    let registry = CatalogBundleRegistry::embedded().expect("load embedded catalogs");
    for page_rel in [
        "site/studio/index.html",
        "site/catalogs/index.html",
        "site/demo/index.html",
    ] {
        let html = std::fs::read_to_string(root.join(page_rel)).expect("read surface html");
        let tag = "<script type=\"application/ld+json\">";
        let start = html.find(tag).expect("missing ld+json script tag") + tag.len();
        let end = html[start..].find("</script>").expect("closing script") + start;
        let parsed: serde_json::Value =
            serde_json::from_str(html[start..end].trim()).expect("valid JSON-LD");
        registry
            .schema_org
            .validate_schema_org_jsonld(&parsed)
            .unwrap_or_else(|e| panic!("{page_rel} JSON-LD failed Schema.org oracle: {e}"));
    }

    // 4. Verify `site/prism-catalogs.mjs` registers `Observer.lex.tex` and all 4 new Holo routes:
    let catalogs_js = std::fs::read_to_string(root.join("site/prism-catalogs.mjs"))
        .expect("read site/prism-catalogs.mjs");
    for token in [
        "app/src/Observer.lex.tex",
        "scene:observer",
        "scene:stakeholders",
        "scene:language",
        "scene:channels",
        "STD-OBSERVER-HOLON",
    ] {
        assert!(
            catalogs_js.contains(token),
            "site/prism-catalogs.mjs missing Observer catalog token {token}"
        );
    }

    // 5. Verify starter template, Studio Scaffolder, and all 3 examples instantiate imported catalogs:
    for rust_rel in [
        "templates/browser-3d-starter/src/main.rs",
        "crates/verdant-core/examples/tensor_uorc_showcase.rs",
        "crates/verdant-core/examples/custom_game_arena.rs",
        "crates/verdant-core/examples/headless_benchmark.rs",
    ] {
        let src = std::fs::read_to_string(root.join(rust_rel)).expect("read rust source");
        assert!(
            src.contains("ObserverPrismHolon") || src.contains("CatalogBundleRegistry"),
            "{rust_rel} must instantiate imported catalogs via ObserverPrismHolon or CatalogBundleRegistry"
        );
    }
    let scaffolder_js = std::fs::read_to_string(root.join("site/studio-scaffolder.mjs"))
        .expect("read site/studio-scaffolder.mjs");
    for slug in [
        "sunspire-regatta",
        "meridian-observatory",
        "glasshouse-chronicle",
        "starfall-bastion",
    ] {
        assert!(
            scaffolder_js.contains(slug) && scaffolder_js.contains("ObserverPrismHolon"),
            "site/studio-scaffolder.mjs missing catalog domain pack {slug}"
        );
    }

    // 6. Verify `verdant-wasm` C-ABI preserves active catalog domain pack across LOD transitions:
    let _abi_guard = wasm_abi_lock();
    assert_eq!(verdant_wasm_init(7), 1);
    assert!(
        verdant_wasm_load_domain_pack(2) > 0,
        "load glasshouse-chronicle"
    );
    let glass_seed = verdant_wasm_active_seed();
    assert_eq!(glass_seed, 108);
    assert_eq!(
        verdant_wasm_set_lod_rank(4),
        1,
        "upgrade glasshouse-chronicle to LOD4"
    );
    assert_eq!(verdant_wasm_active_lod_rank(), 4);
    assert_eq!(
        verdant_wasm_active_seed(),
        108,
        "LOD transition must preserve active catalog domain"
    );
    assert!(verdant_wasm_set_npc_directive(0, 1) > 0);
    verdant_wasm_io_clear();
    for &b in b"What botanical seed vault is conserved here?" {
        verdant_wasm_io_push_byte(u32::from(b));
    }
    assert!(verdant_wasm_dialogue_turn_io(0) > 0);
    let obs_len = verdant_wasm_observer_status_aux() as usize;
    let obs_ptr = verdant_wasm_aux_ptr() as usize as *const u8;
    assert!(obs_len > 0 && verdant_wasm_aux_len() as usize == obs_len && !obs_ptr.is_null());

    // 7. Negative refusals (`ObserverModelError`): out-of-bounds NPC index & invalid domain pack:
    let mut holon = ObserverPrismHolon::new("sunspire-regatta").expect("holon");
    assert!(matches!(
        holon.set_npc_directive(99, 0),
        Err(ObserverModelError::UnknownDomainOrCharacter { .. })
    ));
    assert_eq!(
        verdant_wasm_load_domain_pack(99),
        0,
        "Wasm C-ABI must refuse out-of-bounds domain pack index"
    );
}

/// `VE-41`: UHD 4K-to-8K display scaling, 30-bit Morton spatially-clustered 64-triangle GPU
/// meshlet culling with zero steady-state heap reallocations, 2x2 box-filtered multi-material PBR
/// texture atlas packing, `naga`-verified WGSL compute culling & PBR shaders, zero-reallocation
/// GPU index streaming, and the 64-frame hysteresis `FrameBudgetOracle`.
#[test]
fn engine_ve_41() {
    use verdant_core::hd_world::synthesize_hd_citadel_sector;
    use verdant_core::streaming::{PhysicalEnvironment, TerrainReliefParams};
    use verdant_core::uhd::{
        build_meshlet_graph, evaluate_lod_pyramid_telemetry, evaluate_meshlet_visibility,
        evaluate_uhd_display_profile, pack_rgba_texture_atlas, validate_uhd_shaders,
        FrameBudgetOracle, MAX_ATLAS_TILES, MAX_MESHLET_TRIANGLES,
    };
    use verdant_wasm::{
        verdant_wasm_aux_len, verdant_wasm_aux_ptr, verdant_wasm_frame_stability_aux,
        verdant_wasm_init, verdant_wasm_record_frame_time_us, verdant_wasm_scene_rebuild_buffers,
        verdant_wasm_set_lod_rank, verdant_wasm_tensor_memory_status_aux,
        verdant_wasm_tensor_paged_benchmark_aux, verdant_wasm_uhd_cull_meshlets,
        verdant_wasm_uhd_lod_pyramid_aux, verdant_wasm_uhd_profile_aux,
        verdant_wasm_uhd_visible_idx_count, verdant_wasm_uhd_visible_idx_ptr,
        verdant_wasm_workload_governor_step_aux,
    };

    // 1. Authoritative W3C `naga` WGSL oracle validates compute meshlet cull, 4-channel PBR + HDRI IBL, and 5-tap CAS resolve shaders:
    let uhd_shaders = validate_uhd_shaders().expect("UHD WGSL shaders must validate under naga");
    assert_eq!(uhd_shaders.len(), 3);
    assert_eq!(uhd_shaders[0].label, "uhd_meshlet_cull.wgsl");
    assert_eq!(uhd_shaders[1].label, "uhd_pbr_cas.wgsl");
    assert_eq!(uhd_shaders[2].label, "uhd_cas_resolve.wgsl");

    // 2. Arbitrary 45,000+ triangle LOD4 Citadel sector -> Morton 64-tri MeshletGraph +
    //    zero-reallocation steady-state visibility culling across 32 camera orbit sweeps:
    let relief = TerrainReliefParams::standard_basin_valley();
    let env = PhysicalEnvironment::temperate_terrestrial();
    let lod4 = synthesize_hd_citadel_sector(7, &relief, env.water_level_meters, 4);
    let mut flat_pos: Vec<f32> = Vec::new();
    let mut flat_idx: Vec<u32> = Vec::new();
    for mesh in &lod4.meshes {
        let base_v = (flat_pos.len() / 3) as u32;
        for p in &mesh.positions {
            flat_pos.extend_from_slice(p);
        }
        for &i in &mesh.indices {
            flat_idx.push(base_v + i);
        }
    }
    assert!(
        flat_idx.len() / 3 >= 45_000,
        "LOD4 workload must exceed 45,000 triangles"
    );

    let graph = build_meshlet_graph(&flat_pos, &flat_idx);
    assert_eq!(graph.total_triangles, flat_idx.len() / 3);
    assert_eq!(graph.reordered_indices.len(), flat_idx.len());
    assert!(
        graph.clusters.len() >= 700,
        "LOD4 45k+ tri scene must partition into 700+ 64-tri meshlets"
    );

    let mut visible_indices: Vec<u32> = Vec::new();
    let mut stable_cap = 0usize;
    let mut saw_culling = false;
    let mut first_pose_tel = None;
    let mut first_pose_indices = Vec::new();

    for step in 0..32 {
        let yaw = (step as f64) * (std::f64::consts::TAU / 32.0);
        let eye = Vec3::new(yaw.cos() * 24.0, 9.0, yaw.sin() * 24.0);
        let target = Vec3::new(yaw.cos() * 14.0, 2.0, yaw.sin() * 14.0);
        let frustum = Frustum::from_perspective(
            eye,
            (target - eye).normalized(),
            Vec3::new(0.0, 1.0, 0.0),
            std::f64::consts::FRAC_PI_3,
            16.0 / 9.0,
            0.25,
            120.0,
        );
        let tel = evaluate_meshlet_visibility(
            &graph,
            &flat_idx,
            &frustum,
            eye,
            2160,
            (std::f64::consts::FRAC_PI_3 * 0.5).tan(),
            1.0,
            &mut visible_indices,
        );
        assert_eq!(
            tel.visible_meshlets
                + tel.culled_frustum
                + tel.culled_backface_cone
                + tel.culled_subpixel,
            tel.total_meshlets
        );
        assert_eq!(visible_indices.len(), tel.visible_triangles * 3);
        if tel.cull_ratio_permille > 0 {
            saw_culling = true;
        }
        if step == 0 {
            stable_cap = visible_indices.capacity();
            first_pose_tel = Some(tel);
            first_pose_indices = visible_indices.clone();
        } else {
            assert_eq!(
                visible_indices.capacity(),
                stable_cap,
                "steady-state meshlet culling must perform ZERO heap reallocations"
            );
        }
    }
    assert!(
        saw_culling,
        "Morton meshlet culling must eliminate offscreen/backface/subpixel clusters on LOD4"
    );
    // Exact determinism on re-evaluating pose 0:
    let eye0 = Vec3::new(24.0, 9.0, 0.0);
    let target0 = Vec3::new(14.0, 2.0, 0.0);
    let frustum0 = Frustum::from_perspective(
        eye0,
        (target0 - eye0).normalized(),
        Vec3::new(0.0, 1.0, 0.0),
        std::f64::consts::FRAC_PI_3,
        16.0 / 9.0,
        0.25,
        120.0,
    );
    let replay_tel = evaluate_meshlet_visibility(
        &graph,
        &flat_idx,
        &frustum0,
        eye0,
        2160,
        (std::f64::consts::FRAC_PI_3 * 0.5).tan(),
        1.0,
        &mut visible_indices,
    );
    assert_eq!(Some(replay_tel), first_pose_tel);
    assert_eq!(visible_indices, first_pose_indices);

    // 3. Interleaved un-optimized triangle tightening via 30-bit Morton sorting:
    let mut inter_pos = Vec::new();
    let mut inter_idx = Vec::new();
    for t in 0..MAX_MESHLET_TRIANGLES {
        let x = (t as f32) * 0.05;
        let b0 = (inter_pos.len() / 3) as u32;
        inter_pos.extend_from_slice(&[x, 0.0, 0.0, x + 0.04, 0.0, 0.0, x + 0.02, 0.04, 0.0]);
        inter_idx.extend_from_slice(&[b0, b0 + 1, b0 + 2]);
        let b1 = (inter_pos.len() / 3) as u32;
        inter_pos.extend_from_slice(&[x, 0.0, 50.0, x + 0.04, 0.0, 50.0, x + 0.02, 0.04, 50.0]);
        inter_idx.extend_from_slice(&[b1, b1 + 1, b1 + 2]);
    }
    let inter_graph = build_meshlet_graph(&inter_pos, &inter_idx);
    assert_eq!(inter_graph.clusters.len(), 2);
    assert!(
        inter_graph.clusters.iter().all(|c| c.radius < 2.5),
        "Morton spatial sort must tighten interleaved geometry into compact spheres"
    );

    // 4. 2x2 Box-filtered multi-material PBR atlas (4 & 16 distinct textures) + safe empty refusal:
    assert!(pack_rgba_texture_atlas(&[], 512).is_none());
    let mut checker = vec![240u8; 16 * 16 * 4];
    for px in (0..16 * 16).step_by(2) {
        checker[px * 4] = 80;
        checker[px * 4 + 1] = 80;
        checker[px * 4 + 2] = 80;
    }
    let solid_tiles: Vec<Vec<u8>> = (0..MAX_ATLAS_TILES)
        .map(|i| vec![((i as u8) * 15).saturating_add(16); 16 * 16 * 4])
        .collect();
    let mut refs: Vec<(u32, u32, &[u8])> = vec![(16, 16, &checker)];
    for tile in solid_tiles.iter().skip(1) {
        refs.push((16, 16, tile.as_slice()));
    }
    let atlas16 = pack_rgba_texture_atlas(&refs, 32).expect("pack 16-material 4x4 atlas");
    assert_eq!(atlas16.grid_dim, 4);
    assert_eq!(atlas16.tile_count, 16);
    assert!(atlas16.width <= 32 && atlas16.height <= 32);
    assert!(
        atlas16.rgba[0] > 80 && atlas16.rgba[0] < 240,
        "2x2 box downsampling must filter checker texels smoothly"
    );
    let uv0 = atlas16.tile_transforms[0].apply(0.0, 0.0);
    let uv0_max = atlas16.tile_transforms[0].apply(0.999, 0.999);
    assert!(
        uv0[0] > 0.0 && uv0_max[0] < 0.25,
        "half-texel bleed guard must keep tile 0 strictly inside [0, 0.25)"
    );
    let uv_nan = atlas16.tile_transforms[0].apply(f32::NAN, f32::INFINITY);
    assert!(uv_nan[0].is_finite() && uv_nan[1].is_finite());

    // 5. UHD 1080p -> 4K -> 5K -> 8K display ladder + zero-dimension viewport guard:
    let p_zero = evaluate_uhd_display_profile(0, 0, false, false, 6000);
    assert_eq!(p_zero.width_px, 1);
    assert_eq!(p_zero.height_px, 1);
    let p_4k = evaluate_uhd_display_profile(3840, 2160, false, true, 6500);
    let p_8k = evaluate_uhd_display_profile(7680, 4320, false, true, 9000);
    let p_sw = evaluate_uhd_display_profile(7680, 4320, true, false, 18000);
    assert_eq!(p_4k.display_class, "UHD 4K 2160p");
    assert_eq!(p_8k.display_class, "UHD 8K 4320p");
    assert_eq!(p_8k.megapixels_x100, 3317);
    assert_eq!(p_sw.preferred_backend, "webgl2-software-bounded");
    assert_eq!(p_sw.drs_scale_permille, 650);

    // 6. 1,000-frame zero-allocation `FrameBudgetOracle` stability & anti-thrash hysteresis proof:
    let mut oracle = FrameBudgetOracle::new(16_667);
    // 200 frames of borderline jitter around 16.67ms must trigger ZERO DRS thrashing:
    for i in 0..200 {
        oracle.record_frame(if i % 2 == 0 { 15_000 } else { 18_000 });
    }
    assert_eq!(
        oracle.report().drs_transitions,
        0,
        "hysteresis deadband must eliminate DRS thrashing on borderline frames"
    );
    // Sustained 20.5ms overload steps cleanly down through every rung to 650 permille:
    for _ in 0..20 {
        oracle.record_frame(20_500);
    }
    assert_eq!(oracle.active_drs_permille(), 650);
    // Sustained 8.5ms fast recovery climbs cleanly back to 1000 permille and stabilizes:
    for _ in 0..780 {
        oracle.record_frame(8_500);
    }
    let final_rep = oracle.report();
    assert_eq!(final_rep.sample_count, 64);
    assert_eq!(final_rep.drs_scale_permille, 1000);
    assert_eq!(final_rep.p50_us, 8_500);
    assert_eq!(final_rep.p95_us, 8_500);
    assert_eq!(final_rep.jitter_us, 0);
    assert_eq!(final_rep.budget_compliance_permille, 1000);
    assert!(final_rep.stable);

    // 7. Live Wasm C-ABI parity for UHD meshlets, 8K profile, and FrameBudgetOracle:
    let _abi_guard = wasm_abi_lock();
    assert_eq!(verdant_wasm_init(7), 1);
    assert_eq!(verdant_wasm_set_lod_rank(3), 1);
    assert!(verdant_wasm_scene_rebuild_buffers(0.0) > 0);
    let m_len = verdant_wasm_uhd_cull_meshlets(0.0, 6.0, 22.0, 0.0, 2.0, 0.0, 16.0 / 9.0, 2160, 10);
    assert!(m_len > 0);
    assert!(verdant_wasm_uhd_visible_idx_count() > 0);
    assert!(verdant_wasm_uhd_visible_idx_ptr() > 0);
    assert!(verdant_wasm_uhd_profile_aux(7680, 4320, 0, 1, 8000) > 0);
    for _ in 0..16 {
        assert!(verdant_wasm_record_frame_time_us(8_200) >= 650);
    }
    let stab_len = verdant_wasm_frame_stability_aux() as usize;
    assert!(
        stab_len > 0 && verdant_wasm_aux_len() as usize == stab_len && verdant_wasm_aux_ptr() != 0,
        "Wasm C-ABI must publish non-empty stability oracle JSON into aux_buffer"
    );
    let lod_len = verdant_wasm_uhd_lod_pyramid_aux() as usize;
    assert!(
        lod_len > 0 && verdant_wasm_aux_len() as usize == lod_len,
        "Wasm C-ABI must publish hierarchical LOD pyramid JSON into aux_buffer"
    );
    let rep_json = serde_json::to_string(&final_rep).expect("serialize FrameStabilityReport");
    assert!(rep_json.contains("\"stable\":true"));
    assert!(rep_json.contains("\"drsTransitions\":"));

    // 8. Verify monotonic hierarchical LOD pyramid over the full 45,000+ tri LOD4 Citadel sector:
    let flat_norm: Vec<f32> = lod4
        .meshes
        .iter()
        .flat_map(|m| m.normals.iter().flat_map(|n| n.iter().copied()))
        .collect();
    let flat_col: Vec<f32> = vec![0.6_f32; flat_pos.len()];
    let lod_pyr = evaluate_lod_pyramid_telemetry(&flat_pos, &flat_norm, &flat_col, &flat_idx);
    assert_eq!(lod_pyr.lod0_triangles, flat_idx.len() / 3);
    assert!(
        lod_pyr.lod1_triangles < lod_pyr.lod0_triangles && lod_pyr.lod1_triangles > 0,
        "LOD1 must strictly reduce arbitrary 45k+ tri LOD4 geometry"
    );
    assert!(
        lod_pyr.lod2_triangles < lod_pyr.lod1_triangles && lod_pyr.lod2_triangles > 0,
        "LOD2 must strictly reduce LOD1 geometry"
    );

    // 9. Verify zero-reallocation GPU index streaming & full WGSL compute/4-ch PBR/CAS parity:
    let root = repo_root();
    let hd_js = std::fs::read_to_string(root.join("site/scene-hd-renderer.mjs"))
        .expect("read site/scene-hd-renderer.mjs");
    assert!(
        hd_js.contains("updateStaticIndices") && hd_js.contains("gl.bufferSubData(E, 0, ix)"),
        "scene-hd-renderer.mjs must stream culled indices via zero-reallocation gl.bufferSubData"
    );
    let wgpu_js = std::fs::read_to_string(root.join("site/scene-webgpu-renderer.mjs"))
        .expect("read site/scene-webgpu-renderer.mjs");
    assert!(
        wgpu_js.contains("updateStaticIndices")
            && wgpu_js.contains("writeOrGrow")
            && wgpu_js.contains("cs_cull_meshlets")
            && wgpu_js.contains("fs_cas_resolve")
            && wgpu_js.contains("u_normal_tex")
            && wgpu_js.contains("u_emissive_tex")
            && wgpu_js.contains("u_env_tex"),
        "scene-webgpu-renderer.mjs must maintain full 4-ch PBR + HDRI IBL + compute cull + CAS parity"
    );

    // 10. ISO/IEC 25010:2023 + ITU-R BT.2020/2100 DynamicWorkloadGovernor & PagedTensorArena
    //     beyond-8K (16K+) + arbitrary AI workload scaling with zero manual Studio resolution toggles:
    let gov_len = verdant_wasm_workload_governor_step_aux(15360, 8640, 0, 1, 6_500) as usize;
    assert!(gov_len > 0 && verdant_wasm_aux_len() as usize == gov_len);
    let mem_len = verdant_wasm_tensor_memory_status_aux() as usize;
    assert!(mem_len > 0 && verdant_wasm_aux_len() as usize == mem_len);
    let bench_len = verdant_wasm_tensor_paged_benchmark_aux(64, 512) as usize;
    assert!(bench_len > 0 && verdant_wasm_aux_len() as usize == bench_len);
    let studio_html = std::fs::read_to_string(root.join("site/studio/index.html"))
        .expect("read site/studio/index.html");
    assert!(
        !studio_html.contains("id=\"studio-lod-btn\"")
            && !studio_html.contains("data-wiz-lod")
            && studio_html.contains("id=\"studio-scale-badge\""),
        "Verdant Studio must scale dynamically without superfluous manual video-resolution toggles"
    );
}
