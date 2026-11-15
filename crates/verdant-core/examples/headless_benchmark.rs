//! Headless 600-tick deterministic simulation replay and 60/120 Hz worker frame-budget benchmark.

use verdant_core::genesis::CausalWorldState;
use verdant_core::math::Vec3;
use verdant_core::physics::{KinematicSplineRover, PhysicsWorld, PhysicsWorldConfig};
use verdant_core::runtime::{EngineConfig, FrameBudgetProfiler, WorkerJobGraph, WorldSnapshot};

fn run_simulation(seed: u64) -> Result<(Vec<u8>, Vec3), Box<dyn std::error::Error>> {
    let world = CausalWorldState::synthesize_at_scale(seed, 1.0)?;
    let mut phys = PhysicsWorld::new(PhysicsWorldConfig::default());
    phys.sentinel_rover = Some(KinematicSplineRover::new(
        world.rover_perimeter_waypoints.clone(),
        3.2,
    ));
    for _ in 0..600 {
        phys.step(1.0 / 60.0, Vec3::ZERO)?;
    }
    let final_pos = phys.sentinel_rover.expect("rover").position;
    let snap = WorldSnapshot::encode(&world)?;
    Ok((snap, final_pos))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (snap_a, pos_a) = run_simulation(2026)?;
    let (snap_b, pos_b) = run_simulation(2026)?;
    assert_eq!(
        snap_a, snap_b,
        "world snapshot must be bit-identical across runs"
    );
    assert_eq!(
        pos_a, pos_b,
        "600-tick rover position must be bit-identical across runs"
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
    println!(
        "120 Hz KPI: budget={} us, makespan={} us, compliant={}",
        cfg_120.frame_budget_micros,
        profiler.max_frame_micros(),
        profiler.meets_budget_kpi()
    );
    Ok(())
}
