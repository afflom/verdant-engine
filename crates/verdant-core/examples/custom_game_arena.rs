//! Demonstrates `verdant-core` as a multi-purpose 3D game engine for an arbitrary
//! low-gravity orbital platforming arena with custom meshes, PBR materials, BVH raycasting,
//! capsule player physics, spatial audio, and glTF 2.0 `.glb` export.

use verdant_core::audio::{
    evaluate_binaural_cue, AcousticEnvironment, ListenerPose, SpatialAudioSource,
};
use verdant_core::ecs::{EntityRole, SceneWorld, Transform};
use verdant_core::genesis::{PhysicalEnvironment, TerrainReliefParams};
use verdant_core::geometry::{generate_fluted_entasis_column_mesh, generate_hexagonal_quartz_mesh};
use verdant_core::gltf_io::export_scene_to_glb;
use verdant_core::math::{Aabb, Ray, Vec3};
use verdant_core::physics::{CapsuleController, PhysicsWorld, PhysicsWorldConfig, RigidBody};
use verdant_core::render::PbrMaterial;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut scene = SceneWorld::new();
    let pillar_mesh = generate_fluted_entasis_column_mesh(0.8, 0.7, 6.0, 16, 20, 10);
    let beacon_mesh = generate_hexagonal_quartz_mesh(0.6, 2.2);

    let materials = vec![
        PbrMaterial {
            name: "BrushedTitaniumDeck".to_string(),
            base_color_factor: [0.62, 0.66, 0.72, 1.0],
            metallic_factor: 0.85,
            roughness_factor: 0.28,
            emissive_factor: [0.0, 0.0, 0.0],
            ior: 1.5,
            double_sided: false,
        },
        PbrMaterial {
            name: "PlasmaRelayCrystal".to_string(),
            base_color_factor: [0.25, 0.85, 0.98, 0.95],
            metallic_factor: 0.1,
            roughness_factor: 0.1,
            emissive_factor: [0.1, 0.4, 0.6],
            ior: 1.55,
            double_sided: true,
        },
    ];

    for i in 0..4 {
        let x = (i as f64 - 1.5) * 8.0;
        let id = scene.spawn(
            format!("ArenaPillar_{i}"),
            EntityRole::EngineeredStructure,
            None,
            Transform::from_xyz(x, 0.0, -10.0),
            Aabb::new(Vec3::new(-0.8, 0.0, -0.8), Vec3::new(0.8, 6.0, 0.8)),
            (1.0, "Custom low-gravity arena structural column"),
        );
        scene.attach_renderable(id, 0, 0);
    }

    let beacon_id = scene.spawn(
        "CentralObjectiveBeacon",
        EntityRole::EngineeredStructure,
        None,
        Transform::from_xyz(0.0, 1.0, -10.0),
        Aabb::new(Vec3::new(-0.6, 0.0, -0.6), Vec3::new(0.6, 2.2, 0.6)),
        (1.0, "Interactive arena capture beacon"),
    );
    scene.attach_renderable(beacon_id, 1, 1);

    let bvh = scene.build_bvh();
    let aim_ray = Ray::new(Vec3::new(0.0, 1.8, 5.0), Vec3::new(0.0, 0.0, -1.0));
    let hit = bvh.raycast(aim_ray, 50.0).expect("ray hits central beacon");
    println!(
        "Crosshair raycast hit entity {:?} at {:.2} m",
        hit.entity, hit.distance_meters
    );

    let low_g_env = PhysicalEnvironment {
        gravity_mps2: 3.71,
        fluid_density_kg_m3: 1000.0,
        sound_speed_mps: 240.0,
        stellar_temperature_kelvin: 5778.0,
        solar_irradiance_w_m2: 590.0,
        water_level_meters: -10.0,
    };
    let mut phys = PhysicsWorld::new(PhysicsWorldConfig {
        terrain_seed: 99,
        environment: low_g_env,
        terrain_relief: TerrainReliefParams::sanctuary_amphitheater(),
        max_dt_seconds: 0.1,
        linear_damping: 0.2,
        causeway_deck: None,
    });
    phys.player_capsule = Some(CapsuleController::new(Vec3::new(0.0, 2.0, 5.0)));
    phys.bodies.push(RigidBody::new(
        "JumpOrb",
        Vec3::new(2.0, 6.0, -4.0),
        5.0,
        0.4,
        0.75,
    )?);

    for _ in 0..30 {
        phys.step(1.0 / 60.0, Vec3::new(0.0, 0.0, -1.0))?;
    }

    let cue = evaluate_binaural_cue(
        &SpatialAudioSource {
            label: "BeaconHum".to_string(),
            position: Vec3::new(0.0, 1.5, -10.0),
            fundamental_hz: 330.0,
            reference_gain: 1.0,
        },
        &ListenerPose {
            position: phys.player_capsule.as_ref().expect("player").eye_position(),
            forward: Vec3::new(0.0, 0.0, -1.0),
            up: Vec3::UP,
        },
        &AcousticEnvironment::travertine_rotunda(low_g_env.sound_speed_mps),
    );
    println!(
        "Spatial audio direct gain={:.3}, ITD={:.6} s",
        cue.direct_gain, cue.itd_seconds
    );

    let glb = export_scene_to_glb(&scene, &[pillar_mesh, beacon_mesh], &materials)?;
    println!(
        "Custom arena exported to valid Khronos glTF 2.0 (.glb): {} bytes",
        glb.len()
    );
    Ok(())
}
