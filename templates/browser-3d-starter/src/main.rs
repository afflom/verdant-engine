//! Orbital Beacon Arena — playable 3D starter game and headless verification harness built on
//! `verdant-core`.

use std::path::PathBuf;
use verdant_core::animation::{
    skin_mesh_linear_blend, Joint, KeyframePose, SkeletalClip, Skeleton, SkinWeights4,
};
use verdant_core::audio::{
    evaluate_binaural_cue, synthesize_modal_bar_modes, AcousticEnvironment, ListenerPose,
    SpatialAudioSource,
};
use verdant_core::collision::{gjk_intersect_and_manifold, ConvexHullCollider};
use verdant_core::ecs::{EntityRole, SceneWorld, Transform};
use verdant_core::geometry::{
    generate_faceted_prism_mesh, generate_fluted_column_mesh, generate_terrain_patch_with_params,
    MorphTarget, TerrainMeshParams,
};
use verdant_core::gltf_io::{export_scene_to_glb, validate_glb_bytes};
use verdant_core::math::{Aabb, Quat, Ray, Vec3};
use verdant_core::navmesh::{steer_along_corridor, NavMesh};
use verdant_core::physics::{CapsuleController, PhysicsWorld, PhysicsWorldConfig, RigidBody};
use verdant_core::render::{
    canonical_pbr_material_library, compute_csm_splits, validate_builtin_engine_shaders,
    IblEnvironmentProbe, RenderFrameGraph,
};
use verdant_core::runtime::WorldSnapshot;
use verdant_core::streaming::{
    pack_progressive_uorc_stream, verify_progressive_uorc_stream, StandardSceneState,
    TerrainReliefParams,
};

/// Summary produced by stepping the Orbital Beacon Arena simulation.
#[derive(Clone, Debug)]
pub struct StarterSimulationReport {
    pub entity_count: usize,
    pub shader_count: usize,
    pub render_pass_count: usize,
    pub csm_cascade_count: usize,
    pub ibl_mip_levels: u32,
    pub walkable_polygons: usize,
    pub beacons_activated: usize,
    pub drone_steer_dir: Vec3,
    pub snapshot_bytes: usize,
    pub uorc_stream_bytes: usize,
    pub glb_bytes: Vec<u8>,
    pub glb_node_count: usize,
}

/// Builds and steps the Orbital Beacon Arena game world for verification and `.glb` export.
pub fn run_beacon_arena_simulation() -> Result<StarterSimulationReport, Box<dyn std::error::Error>>
{
    let shaders = validate_builtin_engine_shaders()?;
    let frame_graph = RenderFrameGraph::canonical_pbr_shadow_pipeline();
    let render_pass_count = frame_graph.validate()?;
    let csm_splits = compute_csm_splits(0.25, 140.0)?;
    let ibl_probe = IblEnvironmentProbe::from_resolution(256)?;

    let mut scene = SceneWorld::new();
    let terrain_mesh = generate_terrain_patch_with_params(
        7,
        &TerrainMeshParams {
            half_extent_meters: 20.0,
            grid_segments: 16,
            relief: TerrainReliefParams::standard_basin_valley(),
        },
    );
    let column_mesh = generate_fluted_column_mesh(0.65, 0.55, 4.5, 16, 16, 8);
    let mut beacon_mesh = generate_faceted_prism_mesh(0.5, 2.0);
    beacon_mesh.morph_targets.push(MorphTarget {
        name: "BeaconCrest".to_string(),
        position_deltas: vec![[0.0, 0.18, 0.0]; beacon_mesh.positions.len()],
    });

    // Apply two-joint skeletal LBS deformation to the central relay spire:
    let spire_skeleton = Skeleton::new(vec![
        Joint {
            name: "pedestal_root".to_string(),
            parent_index: 0,
            is_root: true,
            rest_translation: Vec3::ZERO,
            rest_rotation: Quat::IDENTITY,
        },
        Joint {
            name: "prism_apex".to_string(),
            parent_index: 0,
            is_root: false,
            rest_translation: Vec3::new(0.0, 1.0, 0.0),
            rest_rotation: Quat::IDENTITY,
        },
    ])?;
    let pulse_clip = SkeletalClip {
        name: "BeaconPulse".to_string(),
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
                joint_translations: vec![Vec3::ZERO, Vec3::new(0.0, 1.35, 0.0)],
                joint_rotations: vec![Quat::IDENTITY, Quat::from_axis_angle(Vec3::UP, 0.35)],
                morph_weights_permille: vec![1_000],
            },
        ],
    };
    let mid_pose = pulse_clip.sample(&spire_skeleton, 500)?;
    let skin_weights: Vec<SkinWeights4> = beacon_mesh
        .positions
        .iter()
        .map(|pos| {
            let upper = if pos[1] > 0.8 { 750 } else { 250 };
            SkinWeights4::new([0, 1, 0, 0], [1_000 - upper, upper, 0, 0], 2)
        })
        .collect::<Result<Vec<_>, _>>()?;
    let skinned_beacon_mesh =
        skin_mesh_linear_blend(&beacon_mesh, &spire_skeleton, &mid_pose, &skin_weights)?;

    let materials = canonical_pbr_material_library();

    let ground_id = scene.spawn(
        "StarterTerrain",
        EntityRole::TerrainPatch,
        None,
        Transform::default(),
        Aabb::new(Vec3::new(-20.0, -3.0, -20.0), Vec3::new(20.0, 5.0, 20.0)),
        (1.0, "Procedural basin valley heightfield"),
    );
    scene.attach_renderable(ground_id, 0, 0);

    let beacon_coords = [
        ("NorthRelayBeacon", Vec3::new(0.0, 0.0, -6.0)),
        ("EastRelayBeacon", Vec3::new(5.5, 0.0, 3.0)),
        ("WestRelayBeacon", Vec3::new(-5.5, 0.0, 3.0)),
    ];
    for (label, pos) in beacon_coords {
        let col_id = scene.spawn(
            format!("{label}Column"),
            EntityRole::StaticArchitecture,
            Some(ground_id),
            Transform::from_xyz(pos.x, pos.y, pos.z),
            Aabb::new(Vec3::new(-0.7, 0.0, -0.7), Vec3::new(0.7, 4.5, 0.7)),
            (1.0, "Fluted architectural beacon pedestal"),
        );
        scene.attach_renderable(col_id, 1, 1);

        let crystal_id = scene.spawn(
            label,
            EntityRole::StaticArchitecture,
            Some(col_id),
            Transform::from_xyz(0.0, 4.6, 0.0),
            Aabb::new(Vec3::new(-0.5, 0.0, -0.5), Vec3::new(0.5, 2.0, 0.5)),
            (1.0, "Faceted crystalline relay beacon"),
        );
        scene.attach_renderable(crystal_id, 2, 2);
    }

    let bvh = scene.build_bvh();
    let _hit = bvh
        .raycast(
            Ray::new(Vec3::new(0.0, 2.0, 8.0), Vec3::new(0.0, 0.0, -1.0)),
            40.0,
        )
        .ok_or("expected starter raycast to intersect NorthRelayBeaconColumn")?;

    // Build walkable terrain NavMesh and compute patrol drone A* corridor steering:
    let navmesh = NavMesh::from_mesh(&terrain_mesh)?;
    let start_poly = 0usize;
    let mid_poly = navmesh.polygons[start_poly].neighbors[0];
    let goal_poly = *navmesh.polygons[mid_poly]
        .neighbors
        .iter()
        .find(|&&n| n != start_poly)
        .unwrap_or(&mid_poly);
    let corridor = navmesh.find_path_astar(start_poly, goal_poly)?;
    let drone_steer_dir =
        steer_along_corridor(navmesh.polygons[start_poly].centroid, &corridor, 2.0)?;

    // Step player capsule & check 3D GJK convex hull contact against each Relay Beacon:
    let mut physics = PhysicsWorld::new(PhysicsWorldConfig::default());
    physics.player_capsule = Some(CapsuleController::new(Vec3::new(0.0, 2.5, 8.0)));
    physics.bodies.push(RigidBody::new(
        "KineticSphere",
        Vec3::new(1.5, 6.0, -4.0),
        4.0,
        0.4,
        0.65,
    )?);
    for _ in 0..30 {
        physics.step(1.0 / 60.0, Vec3::new(0.0, 0.0, -1.0))?;
    }

    let player_hull = ConvexHullCollider::from_box_half_extents(Vec3::new(0.6, 1.0, 0.6))?;
    let beacon_trigger_hull = ConvexHullCollider::from_box_half_extents(Vec3::new(1.2, 2.5, 1.2))?;
    let mut beacons_activated = 0usize;
    for (_, beacon_pos) in beacon_coords {
        let manifold = gjk_intersect_and_manifold(
            &beacon_trigger_hull,
            beacon_pos,
            &player_hull,
            beacon_pos + Vec3::new(0.4, 0.2, 0.3),
        )?;
        if manifold.intersecting {
            beacons_activated += 1;
        }
    }

    let acoustic_env = AcousticEnvironment::reflective_chamber(343.0);
    let _cue = evaluate_binaural_cue(
        &SpatialAudioSource {
            label: "BeaconModalEmitter".to_string(),
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
    let _modes = synthesize_modal_bar_modes(220.0, 4);

    let streamed_scene = StandardSceneState::synthesize_at_lod(7, 3)?;
    let snap_bytes = WorldSnapshot::encode(&streamed_scene)?;
    let restored = WorldSnapshot::decode(&snap_bytes)?;
    assert_eq!(restored.scene_nodes.len(), 7);

    let uorc_bytes = pack_progressive_uorc_stream(7)?;
    let uorc_summary = verify_progressive_uorc_stream(&uorc_bytes)?;
    assert_eq!(uorc_summary.lod_object_count, 4);

    let glb = export_scene_to_glb(
        &scene,
        &[terrain_mesh, column_mesh, skinned_beacon_mesh],
        &materials,
    )?;
    let glb_summary = validate_glb_bytes(&glb)?;

    Ok(StarterSimulationReport {
        entity_count: scene.entity_count(),
        shader_count: shaders.len(),
        render_pass_count,
        csm_cascade_count: csm_splits.len(),
        ibl_mip_levels: ibl_probe.mip_level_count,
        walkable_polygons: navmesh.polygons.len(),
        beacons_activated,
        drone_steer_dir,
        snapshot_bytes: snap_bytes.len(),
        uorc_stream_bytes: uorc_bytes.len(),
        glb_bytes: glb,
        glb_node_count: glb_summary.node_count,
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let report = run_beacon_arena_simulation()?;
    let out_path = PathBuf::from("custom-starter-world.glb");
    std::fs::write(&out_path, &report.glb_bytes)?;

    println!(
        "Verdant Orbital Beacon Arena OK: entities={} shaders={} passes={} csm={} ibl_mips={} nav_polys={} beacons={}/3 snap_bytes={} uorc_bytes={} glb_bytes={} glb_nodes={}",
        report.entity_count,
        report.shader_count,
        report.render_pass_count,
        report.csm_cascade_count,
        report.ibl_mip_levels,
        report.walkable_polygons,
        report.beacons_activated,
        report.snapshot_bytes,
        report.uorc_stream_bytes,
        report.glb_bytes.len(),
        report.glb_node_count,
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::run_beacon_arena_simulation;

    #[test]
    fn starter_beacon_arena_verifies_all_subsystems() {
        let report = run_beacon_arena_simulation().expect("starter simulation must succeed");
        assert_eq!(report.entity_count, 7);
        assert_eq!(report.shader_count, 4);
        assert_eq!(report.render_pass_count, 5);
        assert_eq!(report.csm_cascade_count, 4);
        assert_eq!(report.ibl_mip_levels, 9);
        assert!(report.walkable_polygons > 64);
        assert_eq!(report.beacons_activated, 3);
        assert!((report.drone_steer_dir.length() - 1.0).abs() < 1e-5);
        assert!(report.snapshot_bytes > 64);
        assert!(report.uorc_stream_bytes > 64);
        assert_eq!(report.glb_node_count, 7);
    }
}
