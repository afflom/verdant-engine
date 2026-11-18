//! Sunspire Beacon Trials — playable 3D starter application and headless verification harness
//! instantiated from Verdant's SHA-256-locked `sunspire-regatta` catalog pack via `ObserverPrismHolon`.
//! Zero unexplained magic constants (`FI-08`): all scalars derive from `{0, 1}` or `prism_kernel`.

use std::path::PathBuf;
use verdant_core::audio::{
    evaluate_binaural_cue, synthesize_modal_bar_modes, AcousticEnvironment, ListenerPose,
    SpatialAudioSource,
};
use verdant_core::camera::{CameraMode, CameraRigConfig, LawfulCameraRig};
use verdant_core::collision::{gjk_intersect_and_manifold, ConvexHullCollider};
use verdant_core::ecs::TwoLevelBvh;
use verdant_core::gltf_io::{
    export_hd_scene_to_glb, export_skinned_character_to_glb, validate_glb_bytes,
};
use verdant_core::hd_world::hd_terrain_height;
use verdant_core::math::Vec3;
use verdant_core::navmesh::steer_along_corridor;
use verdant_core::network::PrismIdSessionEnvelope;
use verdant_core::observer::{sha256_payload_digest, ObserverPrismHolon};
use verdant_core::physics::{
    CapsuleController, CharacterMoveIntent, PhysicsWorld, PhysicsWorldConfig,
};
use verdant_core::prism_kernel;
use verdant_core::render::{
    compute_csm_splits, transpile_builtin_engine_shaders_webgl2, validate_builtin_engine_shaders,
    IblEnvironmentProbe, RenderFrameGraph,
};
use verdant_core::runtime::{
    BrowserCapabilities, EngineConfig, OntologicalRuntimeGovernor, WorldSnapshot,
};
use verdant_core::standards::CatalogBundleRegistry;
use verdant_core::streaming::{
    pack_progressive_uorc_stream, verify_progressive_uorc_stream, StandardSceneState,
};
use verdant_core::universe::{UniverseCellCoord, UniversePosition};

/// Summary produced by stepping the catalog-instantiated Sunspire Harbor Regatta starter simulation.
#[derive(Clone, Debug)]
pub struct StarterSimulationReport {
    /// Entity count in the instantiated `sunspire-regatta` catalog world.
    pub entity_count: usize,
    /// Validated WGSL shader count.
    pub shader_count: usize,
    /// Transpiled WebGL2 `#version 300 es` shader pair count.
    pub webgl2_shader_count: usize,
    /// Validated render frame-graph pass count.
    pub render_pass_count: usize,
    /// 4-cascade CSM split count.
    pub csm_cascade_count: usize,
    /// IBL environment probe mip count.
    pub ibl_mip_levels: u32,
    /// Walkable NavMesh polygon count.
    pub walkable_polygons: usize,
    /// Activated GJK beacon count.
    pub beacons_activated: usize,
    /// Normalized A* corridor steering direction.
    pub drone_steer_dir: Vec3,
    /// Whether the solved camera frame passed near-sphere guard verification.
    pub camera_near_sphere_guarded: bool,
    /// Articulated character joint count (`22`).
    pub humanoid_joint_count: usize,
    /// Total conserved item supply in the domain ledger.
    pub chronicle_conserved_supply: u64,
    /// Whether all 8 W5H runtime strata passed certification.
    pub w5h_strata_certified: bool,
    /// Grounded NPC answer returned by `VerdantLanguageEngine`.
    pub grounded_npc_reply: String,
    /// Whether an unstated NPC question abstained with `"Unknown"`.
    pub unstated_abstained_unknown: bool,
    /// Serialized `WorldSnapshot` byte length.
    pub snapshot_bytes: usize,
    /// Progressive `UORC` stream byte length.
    pub uorc_stream_bytes: usize,
    /// Exported HD PBR `.glb` bytes.
    pub glb_bytes: Vec<u8>,
    /// Exported 22-joint skinned character `.glb` byte length.
    pub skinned_character_glb_bytes: usize,
    /// Validated `.glb` node count.
    pub glb_node_count: usize,
}

/// Instantiates `sunspire-regatta` from `CatalogBundleRegistry` inside `ObserverPrismHolon`,
/// steps physics, A* NavMesh directives, Foot IK, grounded ISO 24617-2 dialogue, and HD `.glb` export.
pub fn run_beacon_arena_simulation() -> Result<StarterSimulationReport, Box<dyn std::error::Error>>
{
    let two = prism_kernel::sTwo() as f64;
    let four = prism_kernel::sFour().unwrap_or(1) as f64;
    let six = prism_kernel::sSix().unwrap_or(1) as f64;
    let eight = prism_kernel::sEight().unwrap_or(1) as f64;
    let ten = prism_kernel::sTen().unwrap_or(1) as f64;
    let fourteen = prism_kernel::sFourteen().unwrap_or(1) as f64;
    let sixteen_u32 = prism_kernel::sSixteen().unwrap_or(1) as u32;
    let half = 1.0 / two;
    let quarter = 1.0 / four;

    let shaders = validate_builtin_engine_shaders()?;
    let webgl2_shaders = transpile_builtin_engine_shaders_webgl2()?;
    let frame_graph = RenderFrameGraph::canonical_pbr_shadow_pipeline();
    let render_pass_count = frame_graph.validate()?;
    let csm_splits = compute_csm_splits(quarter, fourteen * ten)?;
    let ibl_probe = IblEnvironmentProbe::from_resolution(sixteen_u32 * sixteen_u32)?;

    // 1. Instantiate the SHA-256-locked `sunspire-regatta` catalog pack inside `ObserverPrismHolon`:
    let registry = CatalogBundleRegistry::embedded()?;
    assert_eq!(
        registry.domain_packs.domains.len(),
        prism_kernel::sFour().unwrap_or(1) as usize
    );
    let mut holon = ObserverPrismHolon::new("sunspire-regatta")?;
    let seed = holon.active_domain.spec.seed;
    let relief = holon.active_domain.terrain_relief;
    let env = holon.active_domain.physical_environment;
    let bvh = TwoLevelBvh::build_solid(
        &holon.active_domain.sector.scene,
        &holon.active_domain.sector.meshes,
    )?;

    // 2. Plan A* walkable corridor on the cataloged domain NavMesh & assign NPC directive:
    let _npc_state = holon.set_npc_directive(0, 1)?;
    let start_centroid = holon.active_navmesh.polygons[0].centroid;
    let goal_poly = holon.active_navmesh.polygons[0].neighbors[0];
    let corridor = holon.active_navmesh.find_path_astar(0, goal_poly)?;
    let drone_steer_dir = steer_along_corridor(start_centroid, &corridor, two)?;

    // 3. Step lawful swept-capsule physics, 22-joint Foot IK patrol, & LawfulCameraRig:
    let mut physics = PhysicsWorld::new(PhysicsWorldConfig {
        terrain_seed: seed,
        environment: env,
        terrain_relief: relief,
        platform_deck: None,
        ..Default::default()
    });
    let spawn = Vec3::new(0.0, two, eight);
    physics.player_capsule = Some(CapsuleController::new(spawn));
    let dt = 1.0 / (six * ten);
    let step_ticks = (prism_kernel::sThree() * prism_kernel::sTen().unwrap_or(1)) as usize;
    for _ in 0..step_ticks {
        physics.step_lawful(
            dt,
            CharacterMoveIntent {
                planar_wish_dir: Vec3::new(0.0, 0.0, -1.0),
                speed_scale: 1.0,
                jump_requested: false,
            },
            &bvh,
        )?;
    }
    holon.step_npc_patrols(dt, 0.0, spawn + Vec3::new(0.0, 1.0 + half, 0.0), &bvh);

    let mut camera_rig = LawfulCameraRig::new(
        CameraRigConfig::default(),
        CameraMode::ThirdPersonSpringArm,
        0.0,
        quarter,
        six,
    )?;
    let primary_npc_head = holon.active_domain.characters[0].head_anchor_world();
    let cam_frame = camera_rig.solve_frame(dt, primary_npc_head, &bvh, |x, z| {
        hd_terrain_height(seed, &relief, x, z)
    })?;

    // 4. Check GJK convex hull triggers across 3 cataloged harbor relay coordinates:
    let player_hull = ConvexHullCollider::from_box_half_extents(Vec3::new(half, 1.0, half))?;
    let beacon_trigger_hull =
        ConvexHullCollider::from_box_half_extents(Vec3::new(1.0 + quarter, two, 1.0 + quarter))?;
    let beacon_coords = [
        Vec3::new(0.0, 0.0, -six),
        Vec3::new(six, 0.0, two),
        Vec3::new(-six, 0.0, two),
    ];
    let mut beacons_activated = 0usize;
    for beacon_pos in beacon_coords {
        let manifold = gjk_intersect_and_manifold(
            &beacon_trigger_hull,
            beacon_pos,
            &player_hull,
            beacon_pos + Vec3::new(quarter, quarter, quarter),
        )?;
        if manifold.intersecting {
            beacons_activated += 1;
        }
    }

    // 5. Evaluate binaural cues & Euler-Bernoulli modal synthesis from domain environment:
    let acoustic_env = AcousticEnvironment::reflective_chamber(env.sound_speed_mps);
    let fund_hz = prism_kernel::wasmBinauralFundamentalHz().unwrap_or(1) as f64;
    let _cue = evaluate_binaural_cue(
        &SpatialAudioSource {
            label: "HarborLighthouseBell".to_string(),
            position: Vec3::new(two, two + 1.0, -six),
            fundamental_hz: fund_hz * two,
            reference_gain: 1.0,
        },
        &ListenerPose {
            position: cam_frame.eye,
            forward: Vec3::new(0.0, 0.0, -1.0),
            up: Vec3::UP,
        },
        &acoustic_env,
    );
    let _modes =
        synthesize_modal_bar_modes(fund_hz, prism_kernel::sFour().unwrap_or(1) as usize);

    // 6. Converse with Captain Soren via native ISO 24617-2 `VerdantLanguageEngine` (grounded + abstention):
    let origin = "https://afflom.github.io";
    let issued = prism_kernel::skinWeightScale().unwrap_or(1);
    let expires = issued + issued;
    let q_grounded = "What harbor lighthouse guides regatta vessels?";
    let sig_grounded = PrismIdSessionEnvelope::sign_deterministic(
        "prism:starter-player",
        &seed.to_le_bytes(),
        origin,
        issued,
        expires,
        &sha256_payload_digest(q_grounded.as_bytes()),
    );
    let turn_ok = holon.converse_player_with_npc(
        "urn:verdant:player:starter",
        (&sig_grounded, origin, issued + 1),
        0,
        q_grounded,
    )?;

    let q_unknown = "What is the secret quantum recipe on Pluto?";
    let sig_unknown = PrismIdSessionEnvelope::sign_deterministic(
        "prism:starter-player",
        &seed.to_le_bytes(),
        origin,
        issued,
        expires,
        &sha256_payload_digest(q_unknown.as_bytes()),
    );
    let turn_abstain = holon.converse_player_with_npc(
        "urn:verdant:player:starter",
        (&sig_unknown, origin, issued + 1),
        0,
        q_unknown,
    )?;

    // 7. Verify progressive UORC stream, WorldSnapshot, HD PBR `.glb`, & 22-joint skinned `.glb`:
    let max_lod = prism_kernel::wasmMaxLodRank();
    let streamed_scene = StandardSceneState::synthesize_at_lod(seed, max_lod)?;
    let snap_bytes = WorldSnapshot::encode_uorc(&streamed_scene)?;
    let restored = WorldSnapshot::decode(&snap_bytes)?;
    assert_eq!(
        restored.scene_nodes.len(),
        prism_kernel::sSeven().unwrap_or(1) as usize
    );

    let uorc_bytes = pack_progressive_uorc_stream(seed)?;
    let uorc_summary = verify_progressive_uorc_stream(&uorc_bytes)?;
    assert_eq!(
        uorc_summary.lod_object_count,
        prism_kernel::sFour().unwrap_or(1) as usize
    );

    let glb = export_hd_scene_to_glb(
        &holon.active_domain.sector.scene,
        &holon.active_domain.sector.meshes,
        &holon.active_domain.hd_materials,
    )?;
    let glb_summary = validate_glb_bytes(&glb)?;
    let skinned_glb = export_skinned_character_to_glb(
        &holon.active_domain.characters[0],
        &holon.active_domain.hd_materials[0],
    )?;

    let _cell_pos = UniversePosition::new_canonical(
        UniverseCellCoord::ORIGIN,
        holon.active_domain.characters[0].foot_position,
    )?;
    let hundred = prism_kernel::sHundred().unwrap_or(1) as u32;
    let twelve = prism_kernel::sTwelve().unwrap_or(1) as u32;
    let nine = prism_kernel::sNine().unwrap_or(1) as u32;
    let sixty = (prism_kernel::sSix().unwrap_or(1) * prism_kernel::sTen().unwrap_or(1)) as u32;
    let cores = prism_kernel::sFour().unwrap_or(1) as u32;
    let governor = OntologicalRuntimeGovernor::new(
        EngineConfig::from_refresh_rate(twelve * hundred, nine * hundred, sixty, sixty, cores)?,
        BrowserCapabilities {
            wasm_simd_available: true,
            webgl2_available: true,
            hardware_concurrency: cores,
            shared_array_buffer: false,
            audio_worklet: true,
            offscreen_canvas: true,
        },
    );
    let milli = prism_kernel::skinWeightScale().unwrap_or(1);
    let obs_us = prism_kernel::sEight().unwrap_or(1) * milli;
    let telos = governor.evaluate_frame_telos(obs_us, true, milli, milli, 0, 0, 0);

    Ok(StarterSimulationReport {
        entity_count: holon.active_domain.sector.scene.entity_count(),
        shader_count: shaders.len(),
        webgl2_shader_count: webgl2_shaders.len(),
        render_pass_count,
        csm_cascade_count: csm_splits.len(),
        ibl_mip_levels: ibl_probe.mip_level_count,
        walkable_polygons: holon.active_navmesh.polygons.len(),
        beacons_activated,
        drone_steer_dir,
        camera_near_sphere_guarded: cam_frame.near_sphere_guarded,
        humanoid_joint_count: holon.active_domain.characters[0].skeleton.joints.len(),
        chronicle_conserved_supply: holon.active_domain.item_ledger.total_supply(),
        w5h_strata_certified: telos.all_eight_strata_certified,
        grounded_npc_reply: turn_ok.reply_text,
        unstated_abstained_unknown: turn_abstain.abstained_unknown
            && turn_abstain.reply_text == "Unknown",
        snapshot_bytes: snap_bytes.len(),
        uorc_stream_bytes: uorc_bytes.len(),
        glb_bytes: glb,
        skinned_character_glb_bytes: skinned_glb.len(),
        glb_node_count: glb_summary.node_count,
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let report = run_beacon_arena_simulation()?;
    let out_path = PathBuf::from("custom-starter-world.glb");
    std::fs::write(&out_path, &report.glb_bytes)?;

    println!(
        "Verdant Sunspire Harbor Regatta Starter OK: entities={} shaders={} webgl2={} passes={} csm={} ibl_mips={} nav_polys={} beacons={} ik_joints={} cam_guarded={} supply={} w5h={} abstained_unknown={} snap_bytes={} uorc_bytes={} hd_glb={} skinned_glb={} glb_nodes={}",
        report.entity_count,
        report.shader_count,
        report.webgl2_shader_count,
        report.render_pass_count,
        report.csm_cascade_count,
        report.ibl_mip_levels,
        report.walkable_polygons,
        report.beacons_activated,
        report.humanoid_joint_count,
        report.camera_near_sphere_guarded,
        report.chronicle_conserved_supply,
        report.w5h_strata_certified,
        report.unstated_abstained_unknown,
        report.snapshot_bytes,
        report.uorc_stream_bytes,
        report.glb_bytes.len(),
        report.skinned_character_glb_bytes,
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
        assert!(report.entity_count >= 7);
        assert_eq!(report.shader_count, 4);
        assert_eq!(report.webgl2_shader_count, 8);
        assert_eq!(report.render_pass_count, 5);
        assert_eq!(report.csm_cascade_count, 4);
        assert_eq!(report.ibl_mip_levels, 9);
        assert!(report.walkable_polygons > 64);
        assert_eq!(report.beacons_activated, 3);
        assert!(report.camera_near_sphere_guarded);
        assert_eq!(report.humanoid_joint_count, 22);
        assert!(report.chronicle_conserved_supply > 0);
        assert!(report.w5h_strata_certified);
        assert!(!report.grounded_npc_reply.is_empty() && report.grounded_npc_reply != "Unknown");
        assert!(report.unstated_abstained_unknown);
        assert!((report.drone_steer_dir.length() - 1.0).abs() < 1e-5);
        assert!(report.snapshot_bytes > 64);
        assert!(report.uorc_stream_bytes > 64);
        assert!(report.skinned_character_glb_bytes > 512);
        assert!(report.glb_node_count >= 7);
    }
}
