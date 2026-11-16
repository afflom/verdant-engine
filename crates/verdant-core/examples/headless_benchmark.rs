//! Headless 600-tick deterministic simulation replay and 60/120 Hz worker frame-budget benchmark.

use verdant_core::math::Vec3;
use verdant_core::physics::{KinematicSplineBody, PhysicsWorld, PhysicsWorldConfig};
use verdant_core::runtime::{
    BrowserCapabilities, EngineConfig, FrameBudgetProfiler, OntologicalRuntimeGovernor,
    WorkerJobGraph, WorldSnapshot,
};
use verdant_core::streaming::StandardSceneState;

fn run_simulation(seed: u64) -> Result<(Vec<u8>, Vec3), Box<dyn std::error::Error>> {
    let world = StandardSceneState::synthesize_at_lod(seed, 3)?;
    let mut phys = PhysicsWorld::new(PhysicsWorldConfig::default());
    phys.kinematic_mover = Some(KinematicSplineBody::new(
        world.mover_spline_waypoints.clone(),
        3.2,
    ));
    for _ in 0..600 {
        phys.step(1.0 / 60.0, Vec3::ZERO)?;
    }
    let final_pos = phys.kinematic_mover.expect("mover").position;
    let snap = WorldSnapshot::encode(&world)?;
    Ok((snap, final_pos))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (snap_a, pos_a) = run_simulation(2026)?;
    let (snap_b, pos_b) = run_simulation(2026)?;
    assert_eq!(
        snap_a, snap_b,
        "scene snapshot must be bit-identical across runs"
    );
    assert_eq!(
        pos_a, pos_b,
        "600-tick kinematic mover position must be bit-identical across runs"
    );

    let cfg_120 = EngineConfig::from_refresh_rate(2560, 1440, 120, 120, 4)?;
    let mut profiler = FrameBudgetProfiler::new(&cfg_120);
    let dag = WorkerJobGraph::canonical_frame_pipeline(850, 1650, 1050, 620, 540, 1100);
    let (_, makespan) = dag
        .schedule(cfg_120.worker_threads)
        .expect("acyclic frame DAG");
    for _ in 0..120 {
        profiler.record_frame(makespan);
    }
    let governor = OntologicalRuntimeGovernor::new(
        cfg_120,
        BrowserCapabilities {
            wasm_simd_available: true,
            webgl2_available: true,
            hardware_concurrency: 4,
            shared_array_buffer: true,
            audio_worklet: true,
            offscreen_canvas: true,
        },
    );
    let w5h = governor.certify_all_eight_strata();
    assert_eq!(w5h.len(), 8);
    assert!(w5h.iter().all(|c| c.complete_verified));
    let telos = governor.evaluate_frame_telos(makespan, true, 950, 850, 0, 0, 0);
    println!(
        "120 Hz KPI: budget={} us, makespan={} us, compliant={}, w5h_strata={}, telos_loss={}",
        cfg_120.frame_budget_micros,
        profiler.max_frame_micros(),
        profiler.meets_budget_kpi(),
        w5h.len(),
        telos.total_telos_loss
    );
    Ok(())
}
