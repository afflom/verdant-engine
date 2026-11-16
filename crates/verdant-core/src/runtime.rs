//! Browser/WebAssembly host runtime configuration, multi-worker DAG job scheduler,
//! frame-budget KPI profiler, and SHA-256 deterministic binary scene snapshot codec.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::ecs::{EntityId, EntityRecord, EntityRole, SceneWorld, Transform};
use crate::error::{EngineConfigError, SnapshotCodecError};
use crate::math::{Aabb, Quat, Vec3, DIM_HOMOGENEOUS};
use crate::physics::PhysicsWorld;
use crate::prism_kernel;
use crate::streaming::{
    LodLevel, PhysicalEnvironment, SceneBuildParams, SceneNodeInstance, SceneNodeKind,
    StandardSceneState, TerrainReliefParams,
};

/// Browser / WebAssembly host capability envelope discovered at startup.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BrowserCapabilities {
    /// Whether 128-bit WebAssembly SIMD tensor acceleration is available.
    pub wasm_simd_available: bool,
    /// Whether `WebGL2RenderingContext` viewport presentation is available.
    pub webgl2_available: bool,
    /// Logical hardware concurrency (`navigator.hardwareConcurrency`).
    pub hardware_concurrency: u32,
    /// Whether cross-origin isolated `SharedArrayBuffer` workers are enabled.
    pub shared_array_buffer: bool,
    /// Whether `AudioWorklet` real-time spatial DSP is available.
    pub audio_worklet: bool,
    /// Whether `OffscreenCanvas` worker presentation is available.
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
    /// Unique stage identifier (`ecs_transforms`, `bvh_rebuild`, `physics_step`, `frustum_cull`, `spatial_audio`, `frame_present`).
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

    /// Builds the canonical 6-stage parallel frame pipeline (`transforms -> {physics, bvh} -> {cull, audio} -> frame_present`).
    #[must_use]
    pub fn canonical_frame_pipeline(
        transforms_us: u64,
        physics_us: u64,
        bvh_us: u64,
        cull_us: u64,
        audio_us: u64,
        present_us: u64,
    ) -> Self {
        let mut g = Self::new();
        g.add_job("ecs_transforms", transforms_us, &[]);
        g.add_job("physics_step", physics_us, &["ecs_transforms"]);
        g.add_job("bvh_rebuild", bvh_us, &["ecs_transforms"]);
        g.add_job("frustum_cull", cull_us, &["bvh_rebuild"]);
        g.add_job("spatial_audio", audio_us, &["physics_step"]);
        g.add_job(
            "frame_present",
            present_us,
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

const ALL_LOD_LEVELS: &[LodLevel] = &[
    LodLevel::Lod0CoarseProxy,
    LodLevel::Lod1MediumDetail,
    LodLevel::Lod2HighDetail,
    LodLevel::Lod3LosslessSource,
];

const ALL_SCENE_NODE_KINDS: &[SceneNodeKind] = &[
    SceneNodeKind::CentralArchPavilion,
    SceneNodeKind::AxialMarkerTower,
    SceneNodeKind::ModalAcousticsFrame,
    SceneNodeKind::ArticulatedRigBay,
    SceneNodeKind::ElevatedDeckBridge,
    SceneNodeKind::NavigablePathGrid,
    SceneNodeKind::KinematicSplineMover,
];

const ALL_ENTITY_ROLES: &[EntityRole] = &[
    EntityRole::DirectionalLight,
    EntityRole::TerrainPatch,
    EntityRole::FluidVolume,
    EntityRole::SurfaceFoliage,
    EntityRole::AutonomousAgent,
    EntityRole::StaticArchitecture,
    EntityRole::ArticulatedRig,
    EntityRole::KinematicMover,
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

    fn write_bool(&mut self, v: bool) {
        self.write_u8(u8::from(v));
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
                detail: "unexpected end of binary scene payload".to_string(),
            });
        }
        let slice = &self.bytes[self.cursor..end];
        self.cursor = end;
        Ok(slice)
    }

    fn read_u8(&mut self) -> Result<u8, SnapshotCodecError> {
        Ok(self.read_exact(1)?[0])
    }

    fn read_bool(&mut self) -> Result<bool, SnapshotCodecError> {
        match self.read_u8()? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(SnapshotCodecError::MalformedStatePayload {
                detail: "invalid boolean byte in binary scene payload".to_string(),
            }),
        }
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
                detail: "invalid enum ordinal in binary scene payload".to_string(),
            })
    }

    fn read_opt_u64(&mut self) -> Result<Option<u64>, SnapshotCodecError> {
        match self.read_u8()? {
            0 => Ok(None),
            1 => Ok(Some(self.read_u64()?)),
            _ => Err(SnapshotCodecError::MalformedStatePayload {
                detail: "invalid option tag in binary scene payload".to_string(),
            }),
        }
    }
}

fn encode_scene_payload(world: &StandardSceneState) -> Result<Vec<u8>, SnapshotCodecError> {
    let mut w = BinWriter::new();
    w.write_u64(world.seed);
    let env = &world.params.environment;
    w.write_f64(env.gravity_mps2);
    w.write_f64(env.fluid_density_kg_m3);
    w.write_f64(env.sound_speed_mps);
    w.write_f64(env.key_light_temperature_kelvin);
    w.write_f64(env.key_light_irradiance_w_m2);
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

    w.write_usize(world.params.max_foliage_instances);
    w.write_usize(world.params.max_agent_count);
    w.write_usize(world.params.crystal_prop_count);
    w.write_usize(world.params.spline_waypoint_count);
    w.write_f64(world.params.spline_orbit_radius_meters);

    w.write_enum(ALL_LOD_LEVELS, &world.active_lod)?;
    w.write_f64(world.resolution_scale);
    w.write_usize(world.foliage_count);
    w.write_usize(world.agent_count);

    w.write_usize(world.scene_nodes.len());
    for node in &world.scene_nodes {
        w.write_u64(node.preset_index);
        w.write_str(&node.id_slug);
        w.write_str(&node.title);
        w.write_enum(ALL_SCENE_NODE_KINDS, &node.kind)?;
        w.write_vec3(node.world_position);
        w.write_f64(node.span_meters);
        w.write_f64(node.height_meters);
        w.write_f64(node.orbit_distance_meters);
        w.write_f64(node.modal_frequency_hz);
        w.write_bool(node.follow_mover);
    }

    w.write_usize(world.mover_spline_waypoints.len());
    for &wp in &world.mover_spline_waypoints {
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
        w.write_f64(rec.emerged_at_lod_scale);
        w.write_str(&rec.provenance_tag);
    }
    Ok(w.buf)
}

fn decode_scene_payload(payload: &[u8]) -> Result<StandardSceneState, SnapshotCodecError> {
    let mut r = BinReader::new(payload);
    let seed = r.read_u64()?;
    let environment = PhysicalEnvironment {
        gravity_mps2: r.read_f64()?,
        fluid_density_kg_m3: r.read_f64()?,
        sound_speed_mps: r.read_f64()?,
        key_light_temperature_kelvin: r.read_f64()?,
        key_light_irradiance_w_m2: r.read_f64()?,
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
    let params = SceneBuildParams {
        environment,
        terrain,
        max_foliage_instances: r.read_usize()?,
        max_agent_count: r.read_usize()?,
        crystal_prop_count: r.read_usize()?,
        spline_waypoint_count: r.read_usize()?,
        spline_orbit_radius_meters: r.read_f64()?,
    };
    let active_lod = r.read_enum(ALL_LOD_LEVELS)?;
    let resolution_scale = r.read_f64()?;
    let foliage_count = r.read_usize()?;
    let agent_count = r.read_usize()?;

    let node_len = r.read_usize()?;
    let mut scene_nodes = Vec::with_capacity(node_len.min(payload.len()));
    for _ in 0..node_len {
        scene_nodes.push(SceneNodeInstance {
            preset_index: r.read_u64()?,
            id_slug: r.read_string()?,
            title: r.read_string()?,
            kind: r.read_enum(ALL_SCENE_NODE_KINDS)?,
            world_position: r.read_vec3()?,
            span_meters: r.read_f64()?,
            height_meters: r.read_f64()?,
            orbit_distance_meters: r.read_f64()?,
            modal_frequency_hz: r.read_f64()?,
            follow_mover: r.read_bool()?,
        });
    }

    let wp_len = r.read_usize()?;
    let mut mover_spline_waypoints = Vec::with_capacity(wp_len.min(payload.len()));
    for _ in 0..wp_len {
        mover_spline_waypoints.push(r.read_vec3()?);
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
        let emerged_at_lod_scale = r.read_f64()?;
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
                emerged_at_lod_scale,
                provenance_tag,
            },
        );
    }
    if r.cursor != payload.len() {
        return Err(SnapshotCodecError::MalformedStatePayload {
            detail: "trailing bytes in binary scene payload".to_string(),
        });
    }
    Ok(StandardSceneState {
        seed,
        params,
        active_lod,
        resolution_scale,
        foliage_count,
        agent_count,
        scene_nodes,
        mover_spline_waypoints,
        scene: SceneWorld { next_id, entities },
    })
}

/// Deterministic `VDNT` v1 + SHA-256 binary scene snapshot envelope for save/load & replay verification.
pub struct WorldSnapshot;

impl WorldSnapshot {
    /// Encodes `world` into a self-verifying `VDNT\x01` + length + IEEE-754 binary payload + 32-byte SHA-256 trailer.
    pub fn encode(world: &StandardSceneState) -> Result<Vec<u8>, SnapshotCodecError> {
        let payload = encode_scene_payload(world)?;
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

    /// Verifies the `VDNT\x01` header and SHA-256 integrity digest before deserializing `StandardSceneState`.
    pub fn decode(bytes: &[u8]) -> Result<StandardSceneState, SnapshotCodecError> {
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

        decode_scene_payload(payload)
    }
}

/// Quantized per-frame player input command for deterministic lockstep, replay, and rollback netcode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameInputCommand {
    /// Strictly advancing simulation frame index.
    pub frame_index: u64,
    /// Player slot identifier (`0..=255`).
    pub player_slot: u8,
    /// Forward wish input in milli-units (`-1000..=1000`).
    pub move_fwd_milli: i32,
    /// Strafe wish input in milli-units (`-1000..=1000`).
    pub move_strafe_milli: i32,
    /// Heading yaw angle in milli-radians.
    pub yaw_milli: i32,
    /// Bitmask of discrete action triggers (`jump`, `interact`, `throttle`, `brake`).
    pub action_mask: u32,
}

impl FrameInputCommand {
    /// Converts quantized milli-inputs into a world-space horizontal wish vector.
    #[must_use]
    pub fn wish_direction(&self) -> Vec3 {
        let scale = prism_kernel::skinWeightScale().unwrap_or(1) as f64;
        let fwd = f64::from(self.move_fwd_milli) / scale;
        let strafe = f64::from(self.move_strafe_milli) / scale;
        let yaw = f64::from(self.yaw_milli) / scale;
        Vec3::new(
            yaw.sin() * fwd - yaw.cos() * strafe,
            0.0,
            yaw.cos() * fwd + yaw.sin() * strafe,
        )
    }

    fn feed_sha256(&self, hasher: &mut Sha256) {
        hasher.update(self.frame_index.to_le_bytes());
        hasher.update([self.player_slot]);
        hasher.update(self.move_fwd_milli.to_le_bytes());
        hasher.update(self.move_strafe_milli.to_le_bytes());
        hasher.update(self.yaw_milli.to_le_bytes());
        hasher.update(self.action_mask.to_le_bytes());
    }
}

/// SHA-256-chained frame-indexed input log guarded by `prism_kernel::inputJournalSequenceValid`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeterministicInputJournal {
    commands: Vec<FrameInputCommand>,
    chain_digest: Vec<u8>,
}

impl Default for DeterministicInputJournal {
    fn default() -> Self {
        Self::new()
    }
}

impl DeterministicInputJournal {
    /// Creates an empty deterministic input journal with a zero-initialized SHA-256 chain seed.
    #[must_use]
    pub fn new() -> Self {
        let digest_len = prism_kernel::snapshotSha256Bytes().unwrap_or(1) as usize;
        Self {
            commands: Vec::new(),
            chain_digest: vec![0_u8; digest_len],
        }
    }

    /// Appends `cmd` if `prism_kernel::inputJournalSequenceValid` certifies strict frame progression.
    pub fn append(&mut self, cmd: FrameInputCommand) -> Result<(), SnapshotCodecError> {
        let is_first = self.commands.is_empty();
        let prev_frame = self.commands.last().map_or(0, |c| c.frame_index);
        if !prism_kernel::inputJournalSequenceValid(prev_frame, cmd.frame_index, is_first) {
            return Err(SnapshotCodecError::MalformedStatePayload {
                detail: format!(
                    "non-increasing input journal frame {} after frame {prev_frame}",
                    cmd.frame_index
                ),
            });
        }
        let mut hasher = Sha256::new();
        hasher.update(&self.chain_digest);
        cmd.feed_sha256(&mut hasher);
        self.chain_digest = hasher.finalize().to_vec();
        self.commands.push(cmd);
        Ok(())
    }

    /// Ordered slice of recorded input commands.
    #[must_use]
    pub fn commands(&self) -> &[FrameInputCommand] {
        &self.commands
    }

    /// Current 32-byte chained SHA-256 digest over all recorded commands.
    #[must_use]
    pub fn chain_digest_bytes(&self) -> &[u8] {
        &self.chain_digest
    }

    /// Lowercase hexadecimal representation of [`Self::chain_digest_bytes`].
    #[must_use]
    pub fn chain_digest_hex(&self) -> String {
        bytes_to_hex(&self.chain_digest)
    }

    fn recompute_chain_digest(&mut self) {
        let digest_len = prism_kernel::snapshotSha256Bytes().unwrap_or(1) as usize;
        let mut digest = vec![0_u8; digest_len];
        for cmd in &self.commands {
            let mut hasher = Sha256::new();
            hasher.update(&digest);
            cmd.feed_sha256(&mut hasher);
            digest = hasher.finalize().to_vec();
        }
        self.chain_digest = digest;
    }
}

/// Ring-buffered deterministic rollback & resimulation session bounded by `prism_kernel::rollbackMaxWindowFrames()`.
#[derive(Clone, Debug)]
pub struct RollbackSession {
    /// Live deterministic physics world at `current_frame`.
    pub physics: PhysicsWorld,
    /// Fixed deterministic simulation timestep in seconds.
    pub dt_seconds: f64,
    /// Current simulated frame index.
    pub current_frame: u64,
    /// Verified frame-indexed input journal.
    pub journal: DeterministicInputJournal,
    history: VecDeque<(u64, PhysicsWorld)>,
}

impl RollbackSession {
    /// Initializes a rollback session at frame `0` with fixed `dt_seconds`.
    #[must_use]
    pub fn new(physics: PhysicsWorld, dt_seconds: f64) -> Self {
        Self {
            physics,
            dt_seconds,
            current_frame: 0,
            journal: DeterministicInputJournal::new(),
            history: VecDeque::new(),
        }
    }

    /// Maximum rollback window in frames (`64`) derived from `prism_kernel::rollbackMaxWindowFrames()`.
    #[must_use]
    pub fn max_window_frames() -> usize {
        prism_kernel::rollbackMaxWindowFrames().unwrap_or(1) as usize
    }

    /// Records `cmd` at `self.current_frame`, snapshots pre-step `PhysicsWorld` into the bounded
    /// history ring, and steps simulation deterministically by `self.dt_seconds`.
    pub fn advance_frame(&mut self, mut cmd: FrameInputCommand) -> Result<(), SnapshotCodecError> {
        cmd.frame_index = self.current_frame;
        self.journal.append(cmd)?;
        let cap = Self::max_window_frames();
        if self.history.len() >= cap {
            self.history.pop_front();
        }
        self.history
            .push_back((self.current_frame, self.physics.clone()));
        self.physics
            .step(self.dt_seconds, cmd.wish_direction())
            .map_err(|err| SnapshotCodecError::MalformedStatePayload {
                detail: err.to_string(),
            })?;
        self.current_frame += 1;
        Ok(())
    }

    /// Rolls back to `corrected_cmd.frame_index` (verified by `prism_kernel::rollbackFrameWithinWindow`),
    /// replaces that frame's command in the journal, and resimulates forward to `self.current_frame`,
    /// returning the count of resimulated frames.
    pub fn rollback_and_resimulate(
        &mut self,
        corrected_cmd: FrameInputCommand,
    ) -> Result<usize, SnapshotCodecError> {
        if self.current_frame == 0 || corrected_cmd.frame_index >= self.current_frame {
            return Err(SnapshotCodecError::MalformedStatePayload {
                detail: format!(
                    "rollback target frame {} is not behind current frame {}",
                    corrected_cmd.frame_index, self.current_frame
                ),
            });
        }
        let last_stepped = self.current_frame - 1;
        let within_window =
            prism_kernel::rollbackFrameWithinWindow(last_stepped, corrected_cmd.frame_index)
                .unwrap_or(false);
        if !within_window {
            return Err(SnapshotCodecError::MalformedStatePayload {
                detail: format!(
                    "rollback target frame {} outside {}-frame window from {last_stepped}",
                    corrected_cmd.frame_index,
                    Self::max_window_frames()
                ),
            });
        }

        let hist_pos = self
            .history
            .iter()
            .position(|(f, _)| *f == corrected_cmd.frame_index)
            .ok_or_else(|| SnapshotCodecError::MalformedStatePayload {
                detail: format!(
                    "frame {} not present in rollback ring buffer",
                    corrected_cmd.frame_index
                ),
            })?;

        let cmd_pos = self
            .journal
            .commands
            .iter()
            .position(|c| c.frame_index == corrected_cmd.frame_index)
            .ok_or_else(|| SnapshotCodecError::MalformedStatePayload {
                detail: format!(
                    "frame {} missing from deterministic input journal",
                    corrected_cmd.frame_index
                ),
            })?;

        self.journal.commands[cmd_pos] = corrected_cmd;
        self.journal.recompute_chain_digest();

        let restored_physics = self.history[hist_pos].1.clone();
        self.history.truncate(hist_pos);
        self.physics = restored_physics;

        let replay_cmds: Vec<FrameInputCommand> = self.journal.commands[cmd_pos..].to_vec();
        for step_cmd in &replay_cmds {
            let cap = Self::max_window_frames();
            if self.history.len() >= cap {
                self.history.pop_front();
            }
            self.history
                .push_back((step_cmd.frame_index, self.physics.clone()));
            self.physics
                .step(self.dt_seconds, step_cmd.wish_direction())
                .map_err(|err| SnapshotCodecError::MalformedStatePayload {
                    detail: err.to_string(),
                })?;
        }

        Ok(replay_cmds.len())
    }
}

/// Stateless XOR-residual `UORC-SHA256-CHAIN-v1` delta snapshot codec between two `StandardSceneState`s.
pub struct DeltaUorcSnapshot;

impl DeltaUorcSnapshot {
    /// Encodes the bytewise XOR residual from `base` to `next` into a stateless `UORC-SHA256-CHAIN-v1`
    /// archive ready for low-bandwidth rollback/network state synchronization.
    #[cfg(feature = "uorc-streaming")]
    pub fn encode_delta(
        base: &StandardSceneState,
        next: &StandardSceneState,
    ) -> Result<Vec<u8>, SnapshotCodecError> {
        let base_wire = WorldSnapshot::encode(base)?;
        let next_wire = WorldSnapshot::encode(next)?;
        let residual: Vec<u8> = next_wire
            .iter()
            .enumerate()
            .map(|(i, &b)| b ^ base_wire.get(i).copied().unwrap_or(0))
            .collect();

        let max_strats = if cfg!(debug_assertions) {
            1
        } else {
            prism_kernel::sEight().unwrap_or(1) as usize
        };
        let mut strategies = Vec::new();
        for pair in &uorc::portfolio::V04_PAIR {
            for strat in pair {
                if strat.state == 0
                    && !strat.x86
                    && strategies.len() < max_strats
                    && !strategies.contains(&Some(*strat))
                {
                    strategies.push(Some(*strat));
                }
            }
        }
        Ok(uorc::compress_with_strategies(
            &residual,
            &[residual.len() as u64],
            &[],
            &strategies,
        ))
    }

    /// Decompresses `delta_uorc` via `uorc_decompress::run`, reconstructs the target `VDNT` wire
    /// against `base`, and verifies its SHA-256 trailer via [`WorldSnapshot::decode`].
    #[cfg(feature = "uorc-wasm-decode")]
    pub fn decode_delta(
        base: &StandardSceneState,
        delta_uorc: &[u8],
    ) -> Result<StandardSceneState, SnapshotCodecError> {
        if delta_uorc.is_empty() {
            return Err(SnapshotCodecError::TruncatedSnapshot { byte_len: 0 });
        }
        let base_wire = WorldSnapshot::encode(base)?;
        let shift = prism_kernel::uorcDefaultDecodeWork().unwrap_or(1) as u32;
        let work = 1_u64 << shift;
        let residual = match uorc_decompress::run(
            uorc_decompress::Bytes::lit(delta_uorc),
            uorc_decompress::List::nil(),
            uorc_decompress::List::nil(),
            work,
        ) {
            Ok(Some(out)) => out.octets().to_vec(),
            Ok(None) => {
                return Err(SnapshotCodecError::MalformedStatePayload {
                    detail: "uorc_decompress::run rejected delta snapshot seal or work budget"
                        .to_string(),
                });
            }
            Err(_) => {
                return Err(SnapshotCodecError::MalformedStatePayload {
                    detail: "uorc_decompress::run overflow on delta snapshot".to_string(),
                });
            }
        };
        let next_wire: Vec<u8> = residual
            .iter()
            .enumerate()
            .map(|(i, &r)| r ^ base_wire.get(i).copied().unwrap_or(0))
            .collect();
        WorldSnapshot::decode(&next_wire)
    }
}

/// Constructive Stratum-0 `W5H` (`What`, `Why`, `Who`, `Where`, `When`, `How`) certificate for a causal stratum (`0..=7`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StratumW5HCertificate {
    /// Causal stratum index (`0..=7`).
    pub stratum_rank: u64,
    /// Constructive prime atom (`nthPrimeConstructive(stratum_rank)`).
    pub intrinsic_prime: u64,
    /// Composite UOR signature (`intrinsic_prime * factor`).
    pub composite_uor_sig: u64,
    /// Triadic agency prime product (`Observer * Medium * Player = 2 * 3 * 5 = 30`).
    pub triadic_agency_product: u64,
    /// Whether `Foundation.Ontology.V1.Model::verifyW5HCertificateComplete` holds.
    pub complete_verified: bool,
}

/// Evaluated Stratum-0 Holon Telos loss decomposition produced by [`OntologicalRuntimeGovernor::evaluate_frame_telos`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HolonTelosReport {
    /// Count of uncertified nodes, ungrounded modules, or magic root literals.
    pub epistemic_opacity_debt: u64,
    /// Sensory legibility deficit across viewport or audio channels.
    pub sensory_legibility_gap: u64,
    /// Player control or camera dissonance penalty.
    pub player_dissonance: u64,
    /// Microseconds by which `observed_frame_micros` exceeded `frame_budget_micros` (`0` when within budget).
    pub host_ceiling_excess: u64,
    /// Total additive telos loss (`opacity + legibility + dissonance + ceiling_excess`).
    pub total_telos_loss: u64,
    /// Whether zero epistemic opacity debt and active witness presence hold simultaneously.
    pub triadic_connection_active: bool,
    /// Whether `Tensor::strataCertifiedUpTo(7)` holds in the compiled Prism kernel.
    pub all_eight_strata_certified: bool,
}

/// Stratum-0 Ontological Telos and `W5H` runtime governor binding [`EngineConfig`] and [`BrowserCapabilities`]
/// directly to `Foundation.Ontology.V1.Model` (`HostEnvelope`, `WitnessEnvelope`, `evaluateHolonTelosState`,
/// and `verifyW5HCertificateComplete`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OntologicalRuntimeGovernor {
    /// Validated viewport, tick-rate, and frame-budget envelope.
    pub config: EngineConfig,
    /// Discovered host hardware and browser container capabilities.
    pub capabilities: BrowserCapabilities,
}

impl OntologicalRuntimeGovernor {
    /// Constructs a new Stratum-0 Ontological Runtime Governor.
    #[must_use]
    pub const fn new(config: EngineConfig, capabilities: BrowserCapabilities) -> Self {
        Self {
            config,
            capabilities,
        }
    }

    /// Synthesizes and verifies constructive `W5H` certificates across all eight causal strata (`0..=7`)
    /// via `prism_kernel::nthPrimeConstructive`, `triadicAgencyPrimeProduct`, and `verifyW5HCertificateComplete`.
    #[must_use]
    pub fn certify_all_eight_strata(&self) -> Vec<StratumW5HCertificate> {
        let stratum_count = prism_kernel::sEight().unwrap_or(1);
        let agency = prism_kernel::triadicAgencyPrimeProduct(true, true, true).unwrap_or(1 + 1);
        let four = (1_u64 + 1_u64) * (1_u64 + 1_u64);
        let five = four + 1_u64;
        let mut certs = Vec::with_capacity(stratum_count as usize);
        for rank in 0..stratum_count {
            let prime = prism_kernel::nthPrimeConstructive(rank).unwrap_or(1 + 1);
            let uor_sig = prime.saturating_mul(rank + 1);
            let boundary_pole = rank % four;
            let structural_arity = rank + 1;
            let temporal_regime = rank % five;
            let verified = prism_kernel::verifyW5HCertificateComplete(
                prime,
                uor_sig,
                rank,
                agency,
                boundary_pole,
                structural_arity,
                temporal_regime,
                true,
            )
            .unwrap_or(false);
            certs.push(StratumW5HCertificate {
                stratum_rank: rank,
                intrinsic_prime: prime,
                composite_uor_sig: uor_sig,
                triadic_agency_product: agency,
                complete_verified: verified,
            });
        }
        certs
    }

    /// Evaluates Stratum-0 Holon Telos state over `HostEnvelope` and `WitnessEnvelope` via
    /// `prism_kernel::evaluateHolonTelosState`.
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate_frame_telos(
        &self,
        observed_frame_micros: u64,
        active_presence: bool,
        camera_stability_permille: u64,
        focus_dwell_permille: u64,
        opacity_debt: u64,
        legibility_gap: u64,
        player_dissonance: u64,
    ) -> HolonTelosReport {
        let cores = u64::from(self.capabilities.hardware_concurrency.max(1));
        let mem_units =
            prism_kernel::pow2Constructive(prism_kernel::sSix().unwrap_or(1)).unwrap_or(1);
        let thousand = prism_kernel::pow10Constructive(1 + 1 + 1).unwrap_or(1);
        let sample_hz = prism_kernel::sFortyEight()
            .unwrap_or(1)
            .saturating_mul(thousand);
        let quantum = prism_kernel::audioWorkletQuantumFrames().unwrap_or(1);
        let host = prism_kernel::HostEnvelope {
            logicalCores: cores,
            memoryUnits: mem_units,
            frameBudgetMicros: self.config.frame_budget_micros,
            observedFrameMicros: observed_frame_micros,
            audioSampleRateHz: sample_hz,
            audioQuantumFrames: quantum,
        };

        let eight = prism_kernel::sEight().unwrap_or(1);
        let headroom = prism_kernel::sNine()
            .unwrap_or(1)
            .saturating_mul(prism_kernel::sHundred().unwrap_or(1));
        let witness = prism_kernel::WitnessEnvelope {
            criticalBands: eight,
            inputPollMillis: eight,
            cameraStabilityRatio: camera_stability_permille,
            focusDwellRatio: focus_dwell_permille,
            audioHeadroomRatio: headroom,
            activePresence: active_presence,
        };

        let eval = prism_kernel::evaluateHolonTelosState(
            host,
            witness,
            opacity_debt,
            legibility_gap,
            player_dissonance,
        )
        .expect("verified Prism kernel evaluateHolonTelosState");
        let max_stratum = prism_kernel::sSeven().unwrap_or(0);
        let all_eight_strata_certified =
            prism_kernel::strataCertifiedUpTo(max_stratum).unwrap_or(false);

        HolonTelosReport {
            epistemic_opacity_debt: eval.epistemicOpacityDebt,
            sensory_legibility_gap: eval.sensoryLegibilityGap,
            player_dissonance: eval.playerDissonance,
            host_ceiling_excess: eval.hostCeilingExcess,
            total_telos_loss: eval.totalTelosLoss,
            triadic_connection_active: eval.triadicConnectionActive,
            all_eight_strata_certified,
        }
    }

    /// Recommends the next progressive LOD rank (`0..=4`) using non-coercive telos descent:
    /// when `coercive_lock_count > 0` (explicit user LOD pin), `current_lod_rank` is preserved unchanged;
    /// otherwise over-budget frames step down cleanly while generous frame headroom steps up toward `LOD4`.
    #[must_use]
    pub fn recommend_non_coercive_lod(
        &self,
        current_lod_rank: u64,
        observed_frame_micros: u64,
        coercive_lock_count: u64,
    ) -> u64 {
        let max_lod = prism_kernel::wasmHdMaxLodRank().unwrap_or(1);
        let clamped = current_lod_rank.min(max_lod);
        if coercive_lock_count > 0 {
            return clamped;
        }
        let thousand = prism_kernel::pow10Constructive(1 + 1 + 1).unwrap_or(1);
        let report =
            self.evaluate_frame_telos(observed_frame_micros, true, thousand, thousand, 0, 0, 0);
        if report.host_ceiling_excess > 0 {
            clamped.saturating_sub(1)
        } else if observed_frame_micros.saturating_mul(1 + 1) < self.config.frame_budget_micros
            && clamped < max_lod
        {
            clamped + 1
        } else {
            clamped
        }
    }
}
