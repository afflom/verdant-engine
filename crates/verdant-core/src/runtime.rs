//! Browser/WebAssembly host runtime configuration, multi-worker DAG job scheduler,
//! frame-budget KPI profiler, and SHA-256 deterministic binary world snapshot codec.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::ecs::{EntityId, EntityRecord, EntityRole, SceneWorld, Transform};
use crate::error::{EngineConfigError, SnapshotCodecError};
use crate::genesis::{
    ArchitecturalBlueprint, BuilderDiscipline, BuilderGuildSpec, BuilderOptimizationWeights,
    CausalWorldState, CosmicEpoch, LithicSubstrate, PhysicalEnvironment, StructureKind,
    TerrainReliefParams, WorldGenesisParams,
};
use crate::math::{Aabb, Quat, Vec3, DIM_HOMOGENEOUS};
use crate::prism_kernel;

/// Browser / WebAssembly host capability envelope discovered at startup.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BrowserCapabilities {
    /// Whether `navigator.gpu` (`WebGPU`) is available.
    pub webgpu_available: bool,
    /// Whether `WebGL2RenderingContext` fallback is available.
    pub webgl2_available: bool,
    /// Logical hardware concurrency (`navigator.hardwareConcurrency`).
    pub hardware_concurrency: u32,
    /// Whether cross-origin isolated `SharedArrayBuffer` workers are enabled.
    pub shared_array_buffer: bool,
    /// Whether `AudioWorklet` real-time spatial DSP is available.
    pub audio_worklet: bool,
    /// Whether `OffscreenCanvas` worker rendering is available.
    pub offscreen_canvas: bool,
}

/// Validated runtime configuration derived from the host viewport and refresh envelope.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EngineConfig {
    /// Render target width in physical pixels (`> 0`).
    pub viewport_width: u32,
    /// Render target height in physical pixels (`> 0`).
    pub viewport_height: u32,
    /// Fixed deterministic simulation tick rate in hertz (`1..=1000`).
    pub tick_rate_hz: u32,
    /// Target frame budget in microseconds (`1..=1_000_000`, e.g. `16_666` at `60 Hz`).
    pub frame_budget_micros: u64,
    /// Background worker pool thread count (`>= 1`).
    pub worker_threads: u32,
}

impl EngineConfig {
    /// Constructs and validates an `EngineConfig`, deriving `frame_budget_micros` from `target_refresh_hz`.
    pub fn from_refresh_rate(
        viewport_width: u32,
        viewport_height: u32,
        tick_rate_hz: u32,
        target_refresh_hz: u32,
        worker_threads: u32,
    ) -> Result<Self, EngineConfigError> {
        let budget = crate::prism_kernel::frameBudgetMicrosForHz(u64::from(target_refresh_hz))
            .expect("verified Prism kernel frameBudgetMicrosForHz");
        Self::new(
            viewport_width,
            viewport_height,
            tick_rate_hz,
            budget,
            worker_threads,
        )
    }

    /// Validates explicit engine runtime configuration bounds.
    pub fn new(
        viewport_width: u32,
        viewport_height: u32,
        tick_rate_hz: u32,
        frame_budget_micros: u64,
        worker_threads: u32,
    ) -> Result<Self, EngineConfigError> {
        if viewport_width == 0 || viewport_height == 0 {
            return Err(EngineConfigError::ZeroViewportDimension {
                width: viewport_width,
                height: viewport_height,
            });
        }
        let max_hz = prism_kernel::engineMaxTickRateHz()
            .expect("verified Prism kernel engineMaxTickRateHz") as u32;
        if tick_rate_hz == 0 || tick_rate_hz > max_hz {
            return Err(EngineConfigError::InvalidTickRateHz {
                tick_rate_hz,
                max_hz,
            });
        }
        let max_micros = prism_kernel::engineMaxFrameBudgetMicros()
            .expect("verified Prism kernel engineMaxFrameBudgetMicros");
        if frame_budget_micros == 0 || frame_budget_micros > max_micros {
            return Err(EngineConfigError::InvalidFrameBudgetMicros {
                budget_micros: frame_budget_micros,
                max_micros,
            });
        }
        if worker_threads == 0 {
            return Err(EngineConfigError::ZeroWorkerThreads);
        }
        debug_assert_eq!(
            prism_kernel::engineConfigBoundsValid(
                u64::from(viewport_width),
                u64::from(viewport_height),
                u64::from(tick_rate_hz),
                frame_budget_micros,
                u64::from(worker_threads),
            ),
            Ok(true)
        );
        Ok(Self {
            viewport_width,
            viewport_height,
            tick_rate_hz,
            frame_budget_micros,
            worker_threads,
        })
    }
}

/// Stage node in the engine's per-frame multi-worker task graph.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct JobNode {
    /// Unique stage identifier (`ecs_transforms`, `bvh_rebuild`, `physics_step`, `cull_frustum`, `audio_mix`, `gpu_submit`).
    pub id: String,
    /// Estimated execution cost in microseconds.
    pub estimated_micros: u64,
    /// Prerequisite stage identifiers that must complete before this job starts.
    pub dependencies: Vec<String>,
}

/// Scheduled worker lane assignment produced by [`WorkerJobGraph::schedule`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScheduledJob {
    /// Stage identifier.
    pub id: String,
    /// Assigned worker lane index in `0..worker_threads`.
    pub worker_lane: u32,
    /// Start offset from frame start in microseconds.
    pub start_micros: u64,
    /// Completion offset from frame start in microseconds.
    pub finish_micros: u64,
}

/// Directed acyclic per-frame worker job graph with critical-path makespan evaluation.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkerJobGraph {
    nodes: BTreeMap<String, JobNode>,
}

impl WorkerJobGraph {
    /// Creates an empty frame job graph.
    #[must_use]
    pub fn new() -> Self {
        Self {
            nodes: BTreeMap::new(),
        }
    }

    /// Inserts a stage node into the frame DAG.
    pub fn add_job(&mut self, id: impl Into<String>, estimated_micros: u64, deps: &[&str]) {
        let key = id.into();
        self.nodes.insert(
            key.clone(),
            JobNode {
                id: key,
                estimated_micros,
                dependencies: deps.iter().map(|&s| s.to_string()).collect(),
            },
        );
    }

    /// Builds the canonical 6-stage parallel frame pipeline (`transforms -> {physics, bvh} -> {cull, audio} -> gpu_submit`).
    #[must_use]
    pub fn canonical_frame_pipeline(
        transforms_us: u64,
        physics_us: u64,
        bvh_us: u64,
        cull_us: u64,
        audio_us: u64,
        gpu_submit_us: u64,
    ) -> Self {
        let mut g = Self::new();
        g.add_job("ecs_transforms", transforms_us, &[]);
        g.add_job("physics_step", physics_us, &["ecs_transforms"]);
        g.add_job("bvh_rebuild", bvh_us, &["ecs_transforms"]);
        g.add_job("frustum_cull", cull_us, &["bvh_rebuild"]);
        g.add_job("spatial_audio", audio_us, &["physics_step"]);
        g.add_job(
            "gpu_command_submit",
            gpu_submit_us,
            &["physics_step", "frustum_cull", "spatial_audio"],
        );
        debug_assert_eq!(
            g.nodes.len() as u64,
            prism_kernel::canonicalFrameStageCount()
                .expect("verified Prism kernel canonicalFrameStageCount")
        );
        g
    }

    /// Schedules all DAG stages deterministically across `worker_threads`, returning `(schedule, makespan_micros)`.
    /// Returns `None` if the dependency graph contains a cycle or unknown dependency.
    #[must_use]
    pub fn schedule(&self, worker_threads: u32) -> Option<(Vec<ScheduledJob>, u64)> {
        let lanes = worker_threads.max(1) as usize;
        let mut indegree: BTreeMap<String, usize> = BTreeMap::new();
        let mut dependents: BTreeMap<String, Vec<String>> = BTreeMap::new();

        for (id, node) in &self.nodes {
            indegree.entry(id.clone()).or_insert(0);
            for dep in &node.dependencies {
                if !self.nodes.contains_key(dep) {
                    return None;
                }
                *indegree.entry(id.clone()).or_insert(0) += 1;
                dependents.entry(dep.clone()).or_default().push(id.clone());
            }
        }

        let mut ready: VecDeque<String> = indegree
            .iter()
            .filter_map(|(k, &deg)| if deg == 0 { Some(k.clone()) } else { None })
            .collect();

        let mut lane_free_at = vec![0_u64; lanes];
        let mut job_finish: BTreeMap<String, u64> = BTreeMap::new();
        let mut scheduled = Vec::with_capacity(self.nodes.len());
        let mut visited: BTreeSet<String> = BTreeSet::new();

        while let Some(id) = ready.pop_front() {
            visited.insert(id.clone());
            let node = self.nodes.get(&id)?;
            let dep_ready_at = node
                .dependencies
                .iter()
                .filter_map(|d| job_finish.get(d).copied())
                .max()
                .unwrap_or(0);

            // Choose earliest-available worker lane deterministically:
            let mut best_lane = 0usize;
            for l in 1..lanes {
                if lane_free_at[l] < lane_free_at[best_lane] {
                    best_lane = l;
                }
            }
            let start = lane_free_at[best_lane].max(dep_ready_at);
            let finish = start.saturating_add(node.estimated_micros);
            lane_free_at[best_lane] = finish;
            job_finish.insert(id.clone(), finish);
            scheduled.push(ScheduledJob {
                id: id.clone(),
                worker_lane: best_lane as u32,
                start_micros: start,
                finish_micros: finish,
            });

            if let Some(children) = dependents.get(&id) {
                for child in children {
                    if let Some(deg) = indegree.get_mut(child) {
                        *deg -= 1;
                        if *deg == 0 {
                            ready.push_back(child.clone());
                        }
                    }
                }
            }
        }

        if visited.len() != self.nodes.len() {
            return None;
        }
        let makespan = lane_free_at.into_iter().max().unwrap_or(0);
        Some((scheduled, makespan))
    }
}

/// Real-time frame-budget KPI profiler tracking p95/max frame times and draw-call compliance.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameBudgetProfiler {
    /// Configured target frame budget in microseconds.
    pub budget_micros: u64,
    /// Recorded frame durations in microseconds.
    samples_micros: Vec<u64>,
    /// Count of frames that exceeded `budget_micros`.
    pub over_budget_frames: usize,
}

impl FrameBudgetProfiler {
    /// Creates a new profiler bound to `config.frame_budget_micros`.
    #[must_use]
    pub const fn new(config: &EngineConfig) -> Self {
        Self {
            budget_micros: config.frame_budget_micros,
            samples_micros: Vec::new(),
            over_budget_frames: 0,
        }
    }

    /// Records a completed frame's wall/worker makespan in microseconds.
    pub fn record_frame(&mut self, frame_micros: u64) {
        if frame_micros > self.budget_micros {
            self.over_budget_frames += 1;
        }
        self.samples_micros.push(frame_micros);
    }

    /// Number of recorded frames.
    #[must_use]
    pub fn frame_count(&self) -> usize {
        self.samples_micros.len()
    }

    /// Arithmetic mean frame duration in microseconds.
    #[must_use]
    pub fn mean_frame_micros(&self) -> u64 {
        if self.samples_micros.is_empty() {
            return 0;
        }
        let sum: u128 = self.samples_micros.iter().map(|&v| u128::from(v)).sum();
        (sum / (self.samples_micros.len() as u128)) as u64
    }

    /// Maximum observed frame duration in microseconds.
    #[must_use]
    pub fn max_frame_micros(&self) -> u64 {
        self.samples_micros.iter().copied().max().unwrap_or(0)
    }

    /// Returns whether every recorded frame stayed within `budget_micros`.
    #[must_use]
    pub const fn meets_budget_kpi(&self) -> bool {
        self.over_budget_frames == 0 && !self.samples_micros.is_empty()
    }
}

const SNAPSHOT_MAGIC: &[u8; DIM_HOMOGENEOUS] = b"VDNT";
const SNAPSHOT_VERSION: u8 = 1;

fn bytes_to_hex(bytes: &[u8]) -> String {
    let hex_chars = b"0123456789abcdef";
    let shift = (1_u8 + 1_u8) * (1_u8 + 1_u8);
    let mask = (1_u8 << shift) - 1_u8;
    let mut out = String::with_capacity(bytes.len() * (prism_kernel::sTwo() as usize));
    for &b in bytes {
        out.push(char::from(hex_chars[(b >> shift) as usize]));
        out.push(char::from(hex_chars[(b & mask) as usize]));
    }
    out
}

const ALL_EPOCHS: &[CosmicEpoch] = &[
    CosmicEpoch::Cosmogenic,
    CosmicEpoch::LithicHydrologic,
    CosmicEpoch::BioticEcological,
    CosmicEpoch::AgenticArchitectural,
];

const ALL_DISCIPLINES: &[BuilderDiscipline] = &[
    BuilderDiscipline::LithicMasonry,
    BuilderDiscipline::CelestialGnomonics,
    BuilderDiscipline::ModalAcoustics,
    BuilderDiscipline::HydrologicalEngineering,
    BuilderDiscipline::KineticAutomation,
];

const ALL_STRUCTURE_KINDS: &[StructureKind] = &[
    StructureKind::HarmonicRotundaPavilion,
    StructureKind::SolarMeridianSundial,
    StructureKind::ResonantChimePergola,
    StructureKind::QuartzPendulumChamber,
    StructureKind::HydrologicalPierCauseway,
    StructureKind::TravertinePathNetwork,
    StructureKind::PerimeterSentinelRover,
];

const ALL_ENTITY_ROLES: &[EntityRole] = &[
    EntityRole::CelestialLight,
    EntityRole::TerrainFeature,
    EntityRole::WaterBody,
    EntityRole::BotanicalOrganism,
    EntityRole::AutonomousFauna,
    EntityRole::BuilderGuild,
    EntityRole::EngineeredStructure,
    EntityRole::KineticMachine,
    EntityRole::PlayerRig,
];

struct BinWriter {
    buf: Vec<u8>,
}

impl BinWriter {
    fn new() -> Self {
        Self { buf: Vec::new() }
    }

    fn write_u8(&mut self, v: u8) {
        self.buf.push(v);
    }

    fn write_u32(&mut self, v: u32) {
        self.buf.extend_from_slice(&v.to_le_bytes());
    }

    fn write_u64(&mut self, v: u64) {
        self.buf.extend_from_slice(&v.to_le_bytes());
    }

    fn write_usize(&mut self, v: usize) {
        self.write_u64(v as u64);
    }

    fn write_f64(&mut self, v: f64) {
        self.buf.extend_from_slice(&v.to_le_bytes());
    }

    fn write_str(&mut self, s: &str) {
        let bytes = s.as_bytes();
        self.write_u32(bytes.len() as u32);
        self.buf.extend_from_slice(bytes);
    }

    fn write_vec3(&mut self, v: Vec3) {
        self.write_f64(v.x);
        self.write_f64(v.y);
        self.write_f64(v.z);
    }

    fn write_quat(&mut self, q: Quat) {
        self.write_f64(q.x);
        self.write_f64(q.y);
        self.write_f64(q.z);
        self.write_f64(q.w);
    }

    fn write_aabb(&mut self, b: Aabb) {
        self.write_vec3(b.min);
        self.write_vec3(b.max);
    }

    fn write_enum<T: PartialEq>(&mut self, table: &[T], val: &T) -> Result<(), SnapshotCodecError> {
        let idx = table.iter().position(|item| item == val).ok_or_else(|| {
            SnapshotCodecError::MalformedStatePayload {
                detail: "unrecognized enum variant during snapshot encode".to_string(),
            }
        })?;
        self.write_u8(idx as u8);
        Ok(())
    }

    fn write_opt_u64(&mut self, v: Option<u64>) {
        match v {
            None => self.write_u8(0),
            Some(val) => {
                self.write_u8(1);
                self.write_u64(val);
            }
        }
    }
}

struct BinReader<'a> {
    bytes: &'a [u8],
    cursor: usize,
}

impl<'a> BinReader<'a> {
    const fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, cursor: 0 }
    }

    fn read_exact(&mut self, len: usize) -> Result<&'a [u8], SnapshotCodecError> {
        let end = self.cursor.checked_add(len).ok_or_else(|| {
            SnapshotCodecError::MalformedStatePayload {
                detail: "snapshot cursor overflow".to_string(),
            }
        })?;
        if end > self.bytes.len() {
            return Err(SnapshotCodecError::MalformedStatePayload {
                detail: "unexpected end of binary world payload".to_string(),
            });
        }
        let slice = &self.bytes[self.cursor..end];
        self.cursor = end;
        Ok(slice)
    }

    fn read_u8(&mut self) -> Result<u8, SnapshotCodecError> {
        Ok(self.read_exact(1)?[0])
    }

    fn read_u32(&mut self) -> Result<u32, SnapshotCodecError> {
        let raw = self.read_exact(std::mem::size_of::<u32>())?;
        let mut arr = [0_u8; std::mem::size_of::<u32>()];
        arr.copy_from_slice(raw);
        Ok(u32::from_le_bytes(arr))
    }

    fn read_u64(&mut self) -> Result<u64, SnapshotCodecError> {
        let raw = self.read_exact(std::mem::size_of::<u64>())?;
        let mut arr = [0_u8; std::mem::size_of::<u64>()];
        arr.copy_from_slice(raw);
        Ok(u64::from_le_bytes(arr))
    }

    fn read_usize(&mut self) -> Result<usize, SnapshotCodecError> {
        Ok(self.read_u64()? as usize)
    }

    fn read_f64(&mut self) -> Result<f64, SnapshotCodecError> {
        let raw = self.read_exact(std::mem::size_of::<f64>())?;
        let mut arr = [0_u8; std::mem::size_of::<f64>()];
        arr.copy_from_slice(raw);
        Ok(f64::from_le_bytes(arr))
    }

    fn read_string(&mut self) -> Result<String, SnapshotCodecError> {
        let len = self.read_u32()? as usize;
        let raw = self.read_exact(len)?;
        std::str::from_utf8(raw).map(str::to_owned).map_err(|err| {
            SnapshotCodecError::MalformedStatePayload {
                detail: err.to_string(),
            }
        })
    }

    fn read_vec3(&mut self) -> Result<Vec3, SnapshotCodecError> {
        Ok(Vec3::new(
            self.read_f64()?,
            self.read_f64()?,
            self.read_f64()?,
        ))
    }

    fn read_quat(&mut self) -> Result<Quat, SnapshotCodecError> {
        Ok(Quat {
            x: self.read_f64()?,
            y: self.read_f64()?,
            z: self.read_f64()?,
            w: self.read_f64()?,
        })
    }

    fn read_aabb(&mut self) -> Result<Aabb, SnapshotCodecError> {
        Ok(Aabb::new(self.read_vec3()?, self.read_vec3()?))
    }

    fn read_enum<T: Copy>(&mut self, table: &[T]) -> Result<T, SnapshotCodecError> {
        let idx = self.read_u8()? as usize;
        table
            .get(idx)
            .copied()
            .ok_or_else(|| SnapshotCodecError::MalformedStatePayload {
                detail: "invalid enum ordinal in binary world payload".to_string(),
            })
    }

    fn read_opt_u64(&mut self) -> Result<Option<u64>, SnapshotCodecError> {
        match self.read_u8()? {
            0 => Ok(None),
            1 => Ok(Some(self.read_u64()?)),
            _ => Err(SnapshotCodecError::MalformedStatePayload {
                detail: "invalid option tag in binary world payload".to_string(),
            }),
        }
    }
}

fn encode_world_payload(world: &CausalWorldState) -> Result<Vec<u8>, SnapshotCodecError> {
    let mut w = BinWriter::new();
    w.write_u64(world.seed);
    let env = &world.params.environment;
    w.write_f64(env.gravity_mps2);
    w.write_f64(env.fluid_density_kg_m3);
    w.write_f64(env.sound_speed_mps);
    w.write_f64(env.stellar_temperature_kelvin);
    w.write_f64(env.solar_irradiance_w_m2);
    w.write_f64(env.water_level_meters);

    let ter = &world.params.terrain;
    w.write_f64(ter.basin_depth_meters);
    w.write_f64(ter.basin_variance_m2);
    w.write_f64(ter.basin_center_x);
    w.write_f64(ter.basin_center_z);
    w.write_f64(ter.hill_amplitude_meters);
    w.write_f64(ter.hill_frequency_per_meter);
    w.write_f64(ter.outer_rim_curvature);
    w.write_f64(ter.outer_rim_cap_m2);

    w.write_f64(world.params.min_builder_ecological_maturity);
    w.write_usize(world.params.max_flora_count);
    w.write_usize(world.params.max_fauna_count);
    w.write_usize(world.params.mineral_outcrop_count);
    w.write_usize(world.params.rover_waypoint_count);
    w.write_f64(world.params.rover_patrol_radius_meters);

    w.write_f64(world.scale_factor);
    w.write_enum(ALL_EPOCHS, &world.active_epoch)?;

    w.write_f64(world.substrate.stellar_temperature_kelvin);
    w.write_f64(world.substrate.solar_irradiance_w_m2);
    w.write_f64(world.substrate.travertine_reserve_tons);
    w.write_f64(world.substrate.quartz_reserve_tons);
    w.write_f64(world.substrate.basalt_reserve_tons);
    w.write_f64(world.substrate.metallic_trim_reserve_tons);
    w.write_f64(world.substrate.lagoon_max_depth_meters);

    w.write_f64(world.ecological_maturity);
    w.write_usize(world.flora_count);
    w.write_usize(world.fauna_count);

    w.write_usize(world.builder_guilds.len());
    for g in &world.builder_guilds {
        w.write_str(&g.name);
        w.write_enum(ALL_DISCIPLINES, &g.discipline)?;
        w.write_f64(g.weights.shelter);
        w.write_f64(g.weights.acoustic_resonance);
        w.write_f64(g.weights.celestial_alignment);
        w.write_f64(g.weights.geodesic_traversal);
        w.write_f64(g.weights.focal_framing);
        w.write_f64(g.weights.material_cost_penalty);
        w.write_str(&g.governing_science);
    }

    w.write_usize(world.blueprints.len());
    for bp in &world.blueprints {
        w.write_str(&bp.id_slug);
        w.write_str(&bp.title);
        w.write_enum(ALL_STRUCTURE_KINDS, &bp.kind)?;
        w.write_enum(ALL_DISCIPLINES, &bp.lead_discipline)?;
        w.write_vec3(bp.world_position);
        w.write_f64(bp.span_meters);
        w.write_f64(bp.height_meters);
        w.write_f64(bp.section_thickness_meters);
        w.write_f64(bp.thrust_eccentricity_ratio);
        w.write_f64(bp.safety_factor);
        w.write_f64(bp.net_engineering_utility);
        w.write_f64(bp.modal_frequency_hz);
        w.write_str(&bp.civilizational_purpose);
    }

    w.write_usize(world.rover_perimeter_waypoints.len());
    for &wp in &world.rover_perimeter_waypoints {
        w.write_vec3(wp);
    }

    w.write_u64(world.scene.next_id);
    w.write_usize(world.scene.entities.len());
    for rec in world.scene.entities.values() {
        w.write_u64(rec.id.0);
        w.write_str(&rec.name);
        w.write_enum(ALL_ENTITY_ROLES, &rec.role)?;
        w.write_opt_u64(rec.parent.map(|p| p.0));
        w.write_usize(rec.children.len());
        for child in &rec.children {
            w.write_u64(child.0);
        }
        w.write_vec3(rec.local_transform.translation);
        w.write_quat(rec.local_transform.rotation);
        w.write_vec3(rec.local_transform.scale);
        w.write_aabb(rec.local_bounds);
        w.write_opt_u64(rec.mesh_index.map(|m| m as u64));
        w.write_opt_u64(rec.material_index.map(|m| m as u64));
        w.write_f64(rec.emerged_at_scale);
        w.write_str(&rec.provenance_tag);
    }
    Ok(w.buf)
}

fn decode_world_payload(payload: &[u8]) -> Result<CausalWorldState, SnapshotCodecError> {
    let mut r = BinReader::new(payload);
    let seed = r.read_u64()?;
    let environment = PhysicalEnvironment {
        gravity_mps2: r.read_f64()?,
        fluid_density_kg_m3: r.read_f64()?,
        sound_speed_mps: r.read_f64()?,
        stellar_temperature_kelvin: r.read_f64()?,
        solar_irradiance_w_m2: r.read_f64()?,
        water_level_meters: r.read_f64()?,
    };
    let terrain = TerrainReliefParams {
        basin_depth_meters: r.read_f64()?,
        basin_variance_m2: r.read_f64()?,
        basin_center_x: r.read_f64()?,
        basin_center_z: r.read_f64()?,
        hill_amplitude_meters: r.read_f64()?,
        hill_frequency_per_meter: r.read_f64()?,
        outer_rim_curvature: r.read_f64()?,
        outer_rim_cap_m2: r.read_f64()?,
    };
    let params = WorldGenesisParams {
        environment,
        terrain,
        min_builder_ecological_maturity: r.read_f64()?,
        max_flora_count: r.read_usize()?,
        max_fauna_count: r.read_usize()?,
        mineral_outcrop_count: r.read_usize()?,
        rover_waypoint_count: r.read_usize()?,
        rover_patrol_radius_meters: r.read_f64()?,
    };
    let scale_factor = r.read_f64()?;
    let active_epoch = r.read_enum(ALL_EPOCHS)?;
    let substrate = LithicSubstrate {
        stellar_temperature_kelvin: r.read_f64()?,
        solar_irradiance_w_m2: r.read_f64()?,
        travertine_reserve_tons: r.read_f64()?,
        quartz_reserve_tons: r.read_f64()?,
        basalt_reserve_tons: r.read_f64()?,
        metallic_trim_reserve_tons: r.read_f64()?,
        lagoon_max_depth_meters: r.read_f64()?,
    };
    let ecological_maturity = r.read_f64()?;
    let flora_count = r.read_usize()?;
    let fauna_count = r.read_usize()?;

    let guild_len = r.read_usize()?;
    let mut builder_guilds = Vec::with_capacity(guild_len.min(payload.len()));
    for _ in 0..guild_len {
        builder_guilds.push(BuilderGuildSpec {
            name: r.read_string()?,
            discipline: r.read_enum(ALL_DISCIPLINES)?,
            weights: BuilderOptimizationWeights {
                shelter: r.read_f64()?,
                acoustic_resonance: r.read_f64()?,
                celestial_alignment: r.read_f64()?,
                geodesic_traversal: r.read_f64()?,
                focal_framing: r.read_f64()?,
                material_cost_penalty: r.read_f64()?,
            },
            governing_science: r.read_string()?,
        });
    }

    let bp_len = r.read_usize()?;
    let mut blueprints = Vec::with_capacity(bp_len.min(payload.len()));
    for _ in 0..bp_len {
        blueprints.push(ArchitecturalBlueprint {
            id_slug: r.read_string()?,
            title: r.read_string()?,
            kind: r.read_enum(ALL_STRUCTURE_KINDS)?,
            lead_discipline: r.read_enum(ALL_DISCIPLINES)?,
            world_position: r.read_vec3()?,
            span_meters: r.read_f64()?,
            height_meters: r.read_f64()?,
            section_thickness_meters: r.read_f64()?,
            thrust_eccentricity_ratio: r.read_f64()?,
            safety_factor: r.read_f64()?,
            net_engineering_utility: r.read_f64()?,
            modal_frequency_hz: r.read_f64()?,
            civilizational_purpose: r.read_string()?,
        });
    }

    let wp_len = r.read_usize()?;
    let mut rover_perimeter_waypoints = Vec::with_capacity(wp_len.min(payload.len()));
    for _ in 0..wp_len {
        rover_perimeter_waypoints.push(r.read_vec3()?);
    }

    let next_id = r.read_u64()?;
    let ent_len = r.read_usize()?;
    let mut entities = BTreeMap::new();
    for _ in 0..ent_len {
        let id = EntityId(r.read_u64()?);
        let name = r.read_string()?;
        let role = r.read_enum(ALL_ENTITY_ROLES)?;
        let parent = r.read_opt_u64()?.map(EntityId);
        let child_len = r.read_usize()?;
        let mut children = Vec::with_capacity(child_len.min(payload.len()));
        for _ in 0..child_len {
            children.push(EntityId(r.read_u64()?));
        }
        let local_transform = Transform {
            translation: r.read_vec3()?,
            rotation: r.read_quat()?,
            scale: r.read_vec3()?,
        };
        let local_bounds = r.read_aabb()?;
        let mesh_index = r.read_opt_u64()?.map(|v| v as usize);
        let material_index = r.read_opt_u64()?.map(|v| v as usize);
        let emerged_at_scale = r.read_f64()?;
        let provenance_tag = r.read_string()?;
        entities.insert(
            id,
            EntityRecord {
                id,
                name,
                role,
                parent,
                children,
                local_transform,
                local_bounds,
                mesh_index,
                material_index,
                emerged_at_scale,
                provenance_tag,
            },
        );
    }
    if r.cursor != payload.len() {
        return Err(SnapshotCodecError::MalformedStatePayload {
            detail: "trailing bytes in binary world payload".to_string(),
        });
    }
    Ok(CausalWorldState {
        seed,
        params,
        scale_factor,
        active_epoch,
        substrate,
        ecological_maturity,
        flora_count,
        fauna_count,
        builder_guilds,
        blueprints,
        rover_perimeter_waypoints,
        scene: SceneWorld { next_id, entities },
    })
}

/// Deterministic `VDNT` v1 + SHA-256 binary world snapshot envelope for save/load & replay verification.
pub struct WorldSnapshot;

impl WorldSnapshot {
    /// Encodes `world` into a self-verifying `VDNT\x01` + length + IEEE-754 binary payload + 32-byte SHA-256 trailer.
    pub fn encode(world: &CausalWorldState) -> Result<Vec<u8>, SnapshotCodecError> {
        let payload = encode_world_payload(world)?;
        let header_len = prism_kernel::snapshotHeaderBytes()
            .expect("verified Prism kernel snapshotHeaderBytes") as usize;
        let digest_len = prism_kernel::snapshotSha256Bytes()
            .expect("verified Prism kernel snapshotSha256Bytes") as usize;
        debug_assert_eq!(digest_len, Sha256::output_size());
        let len_bytes = (payload.len() as u32).to_le_bytes();
        let mut wire = Vec::with_capacity(header_len + payload.len() + digest_len);
        wire.extend_from_slice(SNAPSHOT_MAGIC);
        wire.push(SNAPSHOT_VERSION);
        wire.extend_from_slice(&len_bytes);
        wire.extend_from_slice(&payload);

        let digest = Sha256::digest(&wire);
        wire.extend_from_slice(&digest);
        debug_assert_eq!(
            prism_kernel::snapshotWireFramingValid(wire.len() as u64, payload.len() as u64),
            Ok(true)
        );
        Ok(wire)
    }

    /// Verifies the `VDNT\x01` header and SHA-256 integrity digest before deserializing `CausalWorldState`.
    pub fn decode(bytes: &[u8]) -> Result<CausalWorldState, SnapshotCodecError> {
        let header_len = prism_kernel::snapshotHeaderBytes()
            .expect("verified Prism kernel snapshotHeaderBytes") as usize;
        let digest_len = prism_kernel::snapshotSha256Bytes()
            .expect("verified Prism kernel snapshotSha256Bytes") as usize;
        let min_len = prism_kernel::snapshotMinWireBytes()
            .expect("verified Prism kernel snapshotMinWireBytes") as usize;
        if bytes.len() < min_len {
            return Err(SnapshotCodecError::TruncatedSnapshot {
                byte_len: bytes.len(),
            });
        }
        if &bytes[..SNAPSHOT_MAGIC.len()] != SNAPSHOT_MAGIC
            || bytes[SNAPSHOT_MAGIC.len()] != SNAPSHOT_VERSION
        {
            return Err(SnapshotCodecError::InvalidHeaderMagic);
        }

        let split_at = bytes.len() - digest_len;
        let (signed_prefix, expected_digest) = bytes.split_at(split_at);
        let observed_digest = Sha256::digest(signed_prefix);
        if observed_digest.as_slice() != expected_digest {
            return Err(SnapshotCodecError::IntegrityDigestMismatch {
                expected_hex: bytes_to_hex(expected_digest),
                observed_hex: bytes_to_hex(observed_digest.as_slice()),
            });
        }

        let len_start = SNAPSHOT_MAGIC.len() + 1;
        let mut len_arr = [0_u8; DIM_HOMOGENEOUS];
        len_arr.copy_from_slice(&signed_prefix[len_start..header_len]);
        let declared_len = u32::from_le_bytes(len_arr) as usize;
        let payload = &signed_prefix[header_len..];
        let framing_ok =
            prism_kernel::snapshotWireFramingValid(bytes.len() as u64, declared_len as u64)
                .unwrap_or(false);
        if !framing_ok || payload.len() != declared_len {
            return Err(SnapshotCodecError::TruncatedSnapshot {
                byte_len: bytes.len(),
            });
        }

        decode_world_payload(payload)
    }
}
