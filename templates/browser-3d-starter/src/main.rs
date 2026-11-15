//! Custom 3D game world starter built on `verdant-core`.

use std::path::PathBuf;
use verdant_core::audio::{
    evaluate_binaural_cue, synthesize_euler_bernoulli_chime_modes, AcousticEnvironment,
    ListenerPose, SpatialAudioSource,
};
use verdant_core::ecs::{EntityRole, SceneWorld, Transform};
use verdant_core::genesis::{CausalWorldState, TerrainReliefParams};
use verdant_core::geometry::{
    generate_fluted_entasis_column_mesh, generate_hexagonal_quartz_mesh,
    generate_terrain_patch_with_params, TerrainMeshParams,
};
use verdant_core::gltf_io::{export_scene_to_glb, validate_glb_bytes};
use verdant_core::math::{Aabb, Ray, Vec3};
use verdant_core::physics::{CapsuleController, PhysicsWorld, PhysicsWorldConfig, RigidBody};
use verdant_core::render::{canonical_pbr_material_library, validate_builtin_engine_shaders};
use verdant_core::runtime::WorldSnapshot;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let shaders = validate_builtin_engine_shaders()?;

    let mut scene = SceneWorld::new();
    let terrain_mesh = generate_terrain_patch_with_params(
        7,
        &TerrainMeshParams {
            half_extent_meters: 20.0,
            grid_segments: 16,
            relief: TerrainReliefParams::sanctuary_amphitheater(),
        },
    );
    let column_mesh = generate_fluted_entasis_column_mesh(0.65, 0.55, 4.5, 16, 16, 8);
    let beacon_mesh = generate_hexagonal_quartz_mesh(0.5, 2.0);
    let materials = canonical_pbr_material_library();

    let ground_id = scene.spawn(
        "StarterTerrain",
        EntityRole::TerrainFeature,
        None,
        Transform::default(),
        Aabb::new(Vec3::new(-20.0, -3.0, -20.0), Vec3::new(20.0, 5.0, 20.0)),
        (1.0, "Procedural starter terrain"),
    );
    scene.attach_renderable(ground_id, 0, 0);

    let column_id = scene.spawn(
        "StarterMonumentColumn",
        EntityRole::EngineeredStructure,
        Some(ground_id),
        Transform::from_xyz(0.0, 0.0, -6.0),
        Aabb::new(Vec3::new(-0.7, 0.0, -0.7), Vec3::new(0.7, 4.5, 0.7)),
        (1.0, "Fluted stone support column"),
    );
    scene.attach_renderable(column_id, 1, 1);

    let beacon_id = scene.spawn(
        "GoalCrystalBeacon",
        EntityRole::EngineeredStructure,
        Some(column_id),
        Transform::from_xyz(0.0, 4.6, 0.0),
        Aabb::new(Vec3::new(-0.5, 0.0, -0.5), Vec3::new(0.5, 2.0, 0.5)),
        (1.0, "Goal crystal objective beacon"),
    );
    scene.attach_renderable(beacon_id, 2, 2);

    let bvh = scene.build_bvh();
    let hit = bvh
        .raycast(
            Ray::new(Vec3::new(0.0, 2.0, 8.0), Vec3::new(0.0, 0.0, -1.0)),
            40.0,
        )
        .ok_or("expected starter raycast to intersect monument column")?;

    let mut physics = PhysicsWorld::new(PhysicsWorldConfig::default());
    physics.player_capsule = Some(CapsuleController::new(Vec3::new(0.0, 2.5, 8.0)));
    physics.bodies.push(RigidBody::new(
        "StarterDynamicOrb",
        Vec3::new(1.5, 6.0, -4.0),
        4.0,
        0.4,
        0.65,
    )?);
    for _ in 0..30 {
        physics.step(1.0 / 60.0, Vec3::new(0.0, 0.0, -1.0))?;
    }

    let acoustic_env = AcousticEnvironment::travertine_rotunda(343.0);
    let cue = evaluate_binaural_cue(
        &SpatialAudioSource {
            label: "BeaconChime".to_string(),
            position: Vec3::new(2.0, 3.0, -6.0),
            fundamental_hz: 440.0,
            reference_gain: 1.0,
        },
        &ListenerPose {
            position: Vec3::new(0.0, 1.7, 4.0),
            forward: Vec3::new(0.0, 0.0, -1.0),
            up: Vec3::UP,
        },
        &acoustic_env,
    );
    let modes = synthesize_euler_bernoulli_chime_modes(220.0, 4);

    let causal_world = CausalWorldState::synthesize_at_scale(7, 1.0)?;
    let snap_bytes = WorldSnapshot::encode(&causal_world)?;
    let restored = WorldSnapshot::decode(&snap_bytes)?;

    let glb = export_scene_to_glb(&scene, &[terrain_mesh, column_mesh, beacon_mesh], &materials)?;
    let glb_summary = validate_glb_bytes(&glb)?;
    let out_path = PathBuf::from("custom-starter-world.glb");
    std::fs::write(&out_path, &glb)?;

    println!(
        "Verdant Starter OK: entities={} hit={:?} shaders={} orb_y={:.3} itd_s={:.6} modes={} snap_bytes={} restored_blueprints={} glb_bytes={} glb_nodes={}",
        scene.entity_count(),
        hit.entity,
        shaders.len(),
        physics.bodies[0].position.y,
        cue.itd_seconds,
        modes.len(),
        snap_bytes.len(),
        restored.blueprints.len(),
        glb.len(),
        glb_summary.node_count,
    );
    Ok(())
}
