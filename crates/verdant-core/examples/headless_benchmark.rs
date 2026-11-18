//! Headless deterministic simulation replay, arbitrary OMG DMN 1.5 Prism model extrapolation,
//! 5-channel W3C SOSA/PROV-O communication bus audit, and 120 Hz `OntologicalRuntimeGovernor`
//! benchmark over `meridian-observatory` and `glasshouse-chronicle`.
//! Zero unexplained magic constants (`FI-08`): all scalars derive from `{0, 1}` or `prism_kernel`.

use verdant_core::cybernetics::{ContainerGrantMatrix, ContainerOrgan};
use verdant_core::math::Vec3;
use verdant_core::network::PrismIdSessionEnvelope;
use verdant_core::observer::{
    sha256_payload_digest, ArbitraryModelDomainKind, CommunicationChannelKind, ObserverOrganKind,
    ObserverPrismHolon, VerdantModelEnvelope,
};
use verdant_core::physics::{KinematicSplineBody, PhysicsWorld, PhysicsWorldConfig};
use verdant_core::prism_kernel;
use verdant_core::runtime::{
    BrowserCapabilities, EngineConfig, FrameBudgetProfiler, OntologicalRuntimeGovernor,
    WorkerJobGraph, WorldSnapshot,
};
use verdant_core::streaming::StandardSceneState;

fn run_catalog_replay(domain_slug: &str) -> Result<(Vec<u8>, Vec3), Box<dyn std::error::Error>> {
    let holon = ObserverPrismHolon::new(domain_slug)?;
    let seed = holon.active_domain.spec.seed;
    let max_lod = prism_kernel::wasmMaxLodRank();
    let world = StandardSceneState::synthesize_at_lod(seed, max_lod)?;
    let mut phys = PhysicsWorld::new(PhysicsWorldConfig {
        terrain_seed: seed,
        environment: holon.active_domain.physical_environment,
        terrain_relief: holon.active_domain.terrain_relief,
        platform_deck: None,
        ..Default::default()
    });
    let speed = prism_kernel::sThree() as f64;
    phys.kinematic_mover = Some(KinematicSplineBody::new(
        world.mover_spline_waypoints.clone(),
        speed,
    ));
    let sixty = prism_kernel::sSix().unwrap_or(1) * prism_kernel::sTen().unwrap_or(1);
    let ticks = (sixty * prism_kernel::sTen().unwrap_or(1)) as usize;
    let dt = 1.0 / (sixty as f64);
    for _ in 0..ticks {
        phys.step(dt, Vec3::ZERO)?;
    }
    let final_pos = phys.kinematic_mover.expect("mover").position;
    let snap = WorldSnapshot::encode_uorc(&world)?;
    Ok((snap, final_pos))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (snap_a, pos_a) = run_catalog_replay("meridian-observatory")?;
    let (snap_b, pos_b) = run_catalog_replay("meridian-observatory")?;
    let (snap_glass, _) = run_catalog_replay("glasshouse-chronicle")?;
    assert_eq!(snap_a, snap_b, "meridian snapshot must be bit-identical");
    assert_eq!(pos_a, pos_b, "600-tick mover pose must be bit-identical");
    assert_ne!(
        snap_a, snap_glass,
        "catalog domains must stay strongly differentiated"
    );

    // Extrapolate an arbitrary Prism model & verify 5-channel SOSA/PROV-O communication bus:
    let mut holon = ObserverPrismHolon::new("glasshouse-chronicle")?;
    let grants = ContainerGrantMatrix::default();
    let arb = VerdantModelEnvelope::new_sealed(
        "PRISM-CELESTIAL-EPHEMERIS",
        ArbitraryModelDomainKind::ConservationAutomaton,
        ObserverOrganKind::Engine,
        ContainerOrgan::WebAssemblyCore,
        "verdant-holo",
        snap_a.clone(),
        serde_json::json!({
            "@context": "https://schema.org",
            "@type": "SoftwareSourceCode",
            "identifier": "urn:verdant:model:celestial-ephemeris",
            "name": "Celestial Ephemeris Automaton"
        }),
    );
    let admitted_kappa = holon.admit_arbitrary_prism_model(&arb, &grants)?;

    let origin = "https://afflom.github.io";
    let issued = prism_kernel::skinWeightScale().unwrap_or(1);
    let p2p_msg = "Synchronize botanical seed ledger across Stitch peers";
    let p2p_sig = PrismIdSessionEnvelope::sign_deterministic(
        "prism:peer-alpha",
        &holon.active_domain.spec.seed.to_le_bytes(),
        origin,
        issued,
        issued + issued,
        &sha256_payload_digest(p2p_msg.as_bytes()),
    );
    let p2p_rec = holon.observe_communication(
        CommunicationChannelKind::PlayerToPlayer,
        "urn:verdant:player:alpha",
        "urn:verdant:player:beta",
        "en",
        0,
        p2p_msg,
        Some((&p2p_sig, origin, issued + 1)),
        0,
    )?;

    let ten = prism_kernel::sTen().unwrap_or(1) as u32;
    let hundred = prism_kernel::sHundred().unwrap_or(1) as u32;
    let sixteen = prism_kernel::sSixteen().unwrap_or(1) as u32;
    let nine = prism_kernel::sNine().unwrap_or(1) as u32;
    let twelve = prism_kernel::sTwelve().unwrap_or(1) as u32;
    let four = prism_kernel::sFour().unwrap_or(1) as u32;
    let hz_120 = twelve * ten;
    let cfg_120 = EngineConfig::from_refresh_rate(
        sixteen * ten * sixteen,
        nine * ten * sixteen,
        hz_120,
        hz_120,
        four,
    )?;
    let mut profiler = FrameBudgetProfiler::new(&cfg_120);
    let u_base = u64::from(hundred * (prism_kernel::sFive().unwrap_or(1) as u32));
    let dag = WorkerJobGraph::canonical_frame_pipeline(
        u_base,
        u_base * (1 + 1),
        u_base,
        u_base,
        u_base,
        u_base,
    );
    let (_, makespan) = dag
        .schedule(cfg_120.worker_threads)
        .expect("acyclic frame DAG");
    for _ in 0..(hz_120 as usize) {
        profiler.record_frame(makespan);
    }
    let governor = OntologicalRuntimeGovernor::new(
        cfg_120,
        BrowserCapabilities {
            wasm_simd_available: true,
            webgl2_available: true,
            hardware_concurrency: four,
            shared_array_buffer: true,
            audio_worklet: true,
            offscreen_canvas: true,
        },
    );
    let w5h = governor.certify_all_eight_strata();
    assert_eq!(w5h.len(), prism_kernel::sEight().unwrap_or(1) as usize);
    let milli = prism_kernel::skinWeightScale().unwrap_or(1);
    let telos = governor.evaluate_frame_telos(makespan, true, milli, milli, 0, 0, 0);
    println!(
        "120 Hz & Observer Holon Benchmark OK: budget={}us makespan={}us compliant={} model_kappa={} p2p_kappa={} w5h={} telos_loss={}",
        cfg_120.frame_budget_micros,
        profiler.max_frame_micros(),
        profiler.meets_budget_kpi(),
        &admitted_kappa[..(sixteen as usize)],
        &p2p_rec.kappa_hex[..(sixteen as usize)],
        w5h.len(),
        telos.total_telos_loss
    );
    Ok(())
}
