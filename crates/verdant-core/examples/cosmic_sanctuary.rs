//! Evolves a world scene from early cosmic expansion (`a = 0.125`) to `t_obs` (`a = 1.0`),
//! verifies all differentiated Builder blueprints, validates WGSL shaders, steps physics
//! and the perimeter sentinel rover, and exports a Khronos glTF 2.0 `.glb` binary.

use verdant_core::genesis::CausalWorldState;
use verdant_core::geometry::canonical_scene_mesh_library;
use verdant_core::gltf_io::{export_scene_to_glb, validate_glb_bytes};
use verdant_core::math::Vec3;
use verdant_core::physics::{KinematicSplineRover, PhysicsWorld, PhysicsWorldConfig};
use verdant_core::render::canonical_pbr_material_library;
use verdant_core::render::validate_builtin_engine_shaders;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut world = CausalWorldState::new_cosmogenic(42)?;
    println!(
        "Initial epoch: {:?} (a = {:.3})",
        world.active_epoch, world.scale_factor
    );

    for target_a in [0.45, 0.70, 1.00] {
        world.advance_to(target_a)?;
        println!(
            "Advanced to a = {:.2} -> {:?}: entities={}, flora={}, builders={}, blueprints={}",
            world.scale_factor,
            world.active_epoch,
            world.scene.entity_count(),
            world.flora_count,
            world.builder_guilds.len(),
            world.blueprints.len()
        );
    }

    for bp in &world.blueprints {
        println!(
            "  Blueprint `{}` ({:?}): e/t={:.4}, SF={:.1}, utility={:.3}",
            bp.title,
            bp.lead_discipline,
            bp.thrust_eccentricity_ratio,
            bp.safety_factor,
            bp.net_engineering_utility
        );
    }

    let shaders = validate_builtin_engine_shaders()?;
    println!(
        "Validated {} built-in W3C WGSL shaders via Naga",
        shaders.len()
    );

    let mut phys = PhysicsWorld::new(PhysicsWorldConfig::default());
    phys.sentinel_rover = Some(KinematicSplineRover::new(
        world.rover_perimeter_waypoints.clone(),
        3.0,
    ));
    for _ in 0..60 {
        phys.step(1.0 / 60.0, Vec3::ZERO)?;
    }
    if let Some(rover) = &phys.sentinel_rover {
        println!(
            "Perimeter sentinel rover traveled {:.2} m -> pos=({:.2}, {:.2}, {:.2})",
            rover.distance_traveled_meters, rover.position.x, rover.position.y, rover.position.z
        );
    }

    let meshes = canonical_scene_mesh_library(world.seed);
    let materials = canonical_pbr_material_library();
    let glb = export_scene_to_glb(&world.scene, &meshes, &materials)?;
    let summary = validate_glb_bytes(&glb)?;
    println!(
        "Exported & verified Khronos glTF 2.0 binary: {} bytes ({} nodes, {} meshes, {} materials)",
        glb.len(),
        summary.node_count,
        summary.mesh_count,
        summary.material_count
    );
    Ok(())
}
