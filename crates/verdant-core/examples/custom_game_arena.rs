//! Instantiates the low-gravity `starfall-bastion` basalt arena from Verdant's SHA-256-locked
//! domain catalog inside `ObserverPrismHolon`, steps A* `NavMesh` NPC directives + 22-joint Foot IK
//! patrols + `PhysicsWorld::step_lawful`, evaluates ISO 24617-2 grounded NPC dialogue (and `"Unknown"`
//! abstention), and exports a 22-joint skinned `.glb` character.
//! Zero unexplained magic constants (`FI-08`): all scalars derive from `{0, 1}` or `prism_kernel`.

use verdant_core::audio::{
    evaluate_binaural_cue, AcousticEnvironment, ListenerPose, SpatialAudioSource,
};
use verdant_core::ecs::TwoLevelBvh;
use verdant_core::gltf_io::{export_skinned_character_to_glb, validate_glb_bytes};
use verdant_core::math::{Ray, Vec3};
use verdant_core::network::PrismIdSessionEnvelope;
use verdant_core::observer::{sha256_payload_digest, ObserverPrismHolon};
use verdant_core::physics::{
    CapsuleController, CharacterMoveIntent, PhysicsWorld, PhysicsWorldConfig, RigidBody,
};
use verdant_core::prism_kernel;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut holon = ObserverPrismHolon::new("starfall-bastion")?;
    let seed = holon.active_domain.spec.seed;
    let env = holon.active_domain.physical_environment;
    let relief = holon.active_domain.terrain_relief;

    let bvh = TwoLevelBvh::build_solid(
        &holon.active_domain.sector.scene,
        &holon.active_domain.sector.meshes,
    )?;
    let two = prism_kernel::sTwo() as f64;
    let five = prism_kernel::sFive().unwrap_or(1) as f64;
    let six = prism_kernel::sSix().unwrap_or(1) as f64;
    let ten = prism_kernel::sTen().unwrap_or(1) as f64;
    let half = 1.0 / two;

    let aim_ray = Ray::new(Vec3::new(0.0, two, six), Vec3::new(0.0, -half, -1.0));
    let ray_hit_dist = bvh
        .tlas()
        .raycast(aim_ray, six * ten)
        .map_or(0.0, |h| h.distance_meters);

    let mut phys = PhysicsWorld::new(PhysicsWorldConfig {
        terrain_seed: seed,
        environment: env,
        terrain_relief: relief,
        platform_deck: None,
        ..Default::default()
    });
    let spawn = Vec3::new(0.0, two, five);
    phys.player_capsule = Some(CapsuleController::new(spawn));
    phys.bodies.push(RigidBody::new(
        "BasaltJumpOrb",
        Vec3::new(two, six, -two),
        five,
        half,
        1.0 - half * half,
    )?);

    let nav_state = holon.set_npc_directive(0, 1)?;
    let dt = 1.0 / (six * ten);
    let ticks = (prism_kernel::sThree() * prism_kernel::sTen().unwrap_or(1)) as usize;
    for _ in 0..ticks {
        phys.step_lawful(
            dt,
            CharacterMoveIntent {
                planar_wish_dir: Vec3::new(0.0, 0.0, -1.0),
                speed_scale: 1.0,
                jump_requested: false,
            },
            &bvh,
        )?;
        holon.step_npc_patrols(dt, 0.0, spawn + Vec3::new(0.0, 1.0 + half, 0.0), &bvh);
    }

    let fund_hz = prism_kernel::wasmBinauralFundamentalHz().unwrap_or(1) as f64;
    let cue = evaluate_binaural_cue(
        &SpatialAudioSource {
            label: "BastionRelayPylon".to_string(),
            position: Vec3::new(0.0, 1.0 + half, -ten),
            fundamental_hz: fund_hz * (two + 1.0),
            reference_gain: 1.0,
        },
        &ListenerPose {
            position: phys.player_capsule.as_ref().expect("player").eye_position(),
            forward: Vec3::new(0.0, 0.0, -1.0),
            up: Vec3::UP,
        },
        &AcousticEnvironment::reflective_chamber(env.sound_speed_mps),
    );

    let origin = "https://afflom.github.io";
    let issued = prism_kernel::skinWeightScale().unwrap_or(1);
    let q_grounded = "What gravity and basalt arena rules govern Starfall Bastion?";
    let sig_ok = PrismIdSessionEnvelope::sign_deterministic(
        "prism:arena-challenger",
        &seed.to_le_bytes(),
        origin,
        issued,
        issued + issued,
        &sha256_payload_digest(q_grounded.as_bytes()),
    );
    let turn_ok = holon.converse_player_with_npc(
        "urn:verdant:player:challenger",
        (&sig_ok, origin, issued + 1),
        0,
        q_grounded,
    )?;

    let q_unknown = "Who painted the purple bicycle on Jupiter?";
    let sig_un = PrismIdSessionEnvelope::sign_deterministic(
        "prism:arena-challenger",
        &seed.to_le_bytes(),
        origin,
        issued,
        issued + issued,
        &sha256_payload_digest(q_unknown.as_bytes()),
    );
    let turn_un = holon.converse_player_with_npc(
        "urn:verdant:player:challenger",
        (&sig_un, origin, issued + 1),
        0,
        q_unknown,
    )?;
    assert!(turn_un.abstained_unknown && turn_un.reply_text == "Unknown");

    let skinned_glb = export_skinned_character_to_glb(
        &holon.active_domain.characters[0],
        &holon.active_domain.hd_materials[0],
    )?;
    let glb_summary = validate_glb_bytes(&skinned_glb)?;

    println!(
        "Starfall Bastion Arena OK: gravity={:.2}m/s^2 ray_dist={:.2}m waypoints={} gain={:.3} evidence={} abstained_unknown={} skinned_glb={}B({} nodes)",
        env.gravity_mps2,
        ray_hit_dist,
        nav_state.corridor_waypoints.len(),
        cue.direct_gain,
        turn_ok.matched_evidence_facts,
        turn_un.abstained_unknown,
        skinned_glb.len(),
        glb_summary.node_count
    );
    Ok(())
}
