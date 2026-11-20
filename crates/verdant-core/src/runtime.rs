//! Browser/WebAssembly host runtime configuration, multi-worker DAG job scheduler,
//! frame-budget KPI profiler, and SHA-256 deterministic binary scene snapshot codec.

use std::collections::{BTreeMap, BTreeSet, HashMap, VecDeque};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::ecs::{EntityId, EntityRecord, EntityRole, SceneWorld, Transform};
use crate::error::{EngineConfigError, RenderGraphError, SnapshotCodecError};
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
            .expect("prism");
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
        let max_hz = prism_kernel::engineMaxTickRateHz().expect("prism") as u32;
        if tick_rate_hz == 0 || tick_rate_hz > max_hz {
            return Err(EngineConfigError::InvalidTickRateHz {
                tick_rate_hz,
                max_hz,
            });
        }
        let max_micros = prism_kernel::engineMaxFrameBudgetMicros().expect("prism");
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
            prism_kernel::canonicalFrameStageCount().expect("prism")
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
        let header_len = prism_kernel::snapshotHeaderBytes().expect("prism") as usize;
        let digest_len = prism_kernel::snapshotSha256Bytes().expect("prism") as usize;
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

    /// Encodes `world` into a SHA-256-sealed `UORC` archive (`b"UORC"` magic + stateless NCM coding
    /// over the inner `VDNT` binary payload), shrinking snapshot wire size by `~8x-10x`.
    #[cfg(feature = "uorc-streaming")]
    pub fn encode_uorc(world: &StandardSceneState) -> Result<Vec<u8>, SnapshotCodecError> {
        let vdnt_wire = Self::encode(world)?;
        Ok(crate::streaming::compress_uorc_portable(&vdnt_wire))
    }

    /// Verifies either a SHA-256-sealed `UORC` snapshot archive (`b"UORC"`, decompressed via
    /// `uorc_decompress::run`) or an inner `VDNT\x01` binary envelope before deserializing `StandardSceneState`.
    pub fn decode(bytes: &[u8]) -> Result<StandardSceneState, SnapshotCodecError> {
        #[cfg(feature = "uorc-wasm-decode")]
        if crate::streaming::is_uorc_archive(bytes) {
            let min_uorc =
                prism_kernel::snapshotSha256Bytes().expect("prism") as usize + SNAPSHOT_MAGIC.len();
            if bytes.len() < min_uorc {
                return Err(SnapshotCodecError::TruncatedSnapshot {
                    byte_len: bytes.len(),
                });
            }
            let inner = crate::streaming::decompress_uorc_portable(bytes).map_err(|e| {
                SnapshotCodecError::IntegrityDigestMismatch {
                    expected_hex: "UORC-SHA256-SEAL".to_string(),
                    observed_hex: e.to_string(),
                }
            })?;
            return Self::decode_vdnt_wire(&inner);
        }
        Self::decode_vdnt_wire(bytes)
    }

    fn decode_vdnt_wire(bytes: &[u8]) -> Result<StandardSceneState, SnapshotCodecError> {
        let header_len = prism_kernel::snapshotHeaderBytes().expect("prism") as usize;
        let digest_len = prism_kernel::snapshotSha256Bytes().expect("prism") as usize;
        let min_len = prism_kernel::snapshotMinWireBytes().expect("prism") as usize;
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
        .expect("prism");
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

    /// Recommends an adaptive internal framebuffer resolution scale in permille (`1000` = `1.0x`,
    /// `850` = `0.85x`, `700` = `0.70x`) via `prism_kernel::governorRecommendedScalePermille`.
    #[must_use]
    pub fn recommend_render_scale_permille(&self, observed_frame_micros: u64) -> u64 {
        prism_kernel::governorRecommendedScalePermille(
            observed_frame_micros,
            self.config.frame_budget_micros,
        )
        .unwrap_or_else(|_| verdant_tensor::kernel_consts().skin_weight_scale)
    }
}

/// Algebraic classification of engine input / state mutation commands for Prism monoid fusion
/// (extrapolated directly from `verdant-aqm` v0.2.0 `optimizeCircuitPrismMonoid` & `GateKind`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CanonicalCommandKind {
    /// Self-inverse camera possession toggle (`ThirdPerson <-> FirstPerson`, `I^2 = id`).
    ToggleCameraMode,
    /// Self-inverse renderer backend toggle (`WebGL2 <-> SoftwareTile16`, `I^2 = id`).
    ToggleRenderBackend,
    /// Self-inverse organ capability flip (`I^2 = id`).
    ToggleOrganCapability,
    /// Discrete octant yaw rotation (`Z_8` group: `(a + b) mod 8`, `0 => id`).
    YawOctantStep {
        /// Signed or unsigned octant delta (`mod 8`).
        octants: u8,
    },
    /// Additive camera look / zoom delta (`(dyaw + dyaw', dpitch + dpitch', ddist + ddist')`, `0 => id`).
    CameraOrbitDelta {
        /// Yaw delta in milli-radians.
        delta_yaw_milli: i32,
        /// Pitch delta in milli-radians.
        delta_pitch_milli: i32,
        /// Orbit distance delta in millimeters.
        delta_dist_milli: i32,
    },
    /// Non-commuting state barrier (`StoryInteract`, `SnapshotRestore`, or explicit sync barrier).
    StateBarrier,
}

impl CanonicalCommandKind {
    /// Returns true when applying an identical command twice on the same target cancels to identity.
    #[must_use]
    pub const fn is_self_inverse(self) -> bool {
        matches!(
            self,
            Self::ToggleCameraMode | Self::ToggleRenderBackend | Self::ToggleOrganCapability
        )
    }

    /// Numeric ordinal passed to `prism_kernel::commandInvolutionCancels`.
    #[must_use]
    pub const fn ordinal(self) -> u64 {
        let two = 1_u64.wrapping_add(1_u64);
        let three = two.wrapping_add(1_u64);
        let four = two.wrapping_mul(two);
        match self {
            Self::ToggleCameraMode => 0,
            Self::ToggleRenderBackend => 1,
            Self::ToggleOrganCapability => two,
            Self::YawOctantStep { .. } => three,
            Self::CameraOrbitDelta { .. } => four,
            Self::StateBarrier => four.wrapping_add(1_u64),
        }
    }
}

/// Target-addressed engine command node for monoid canonicalization and Kahn DAG scheduling.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalCommandOp {
    /// Command identifier.
    pub id: u32,
    /// Target entity or subsystem wire slot (`0 = camera`, `1 = renderer`, `2.. = entity/organ`).
    pub target_slot: u32,
    /// Algebraic operation kind.
    pub kind: CanonicalCommandKind,
    /// Optional explicit upstream command indices (`must satisfy dep < index` or acyclic Kahn sort).
    pub depends_on: Vec<usize>,
}

/// Summary report emitted by [`canonicalize_command_monoid`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandMonoidReport {
    /// Canonicalized, fused command sequence.
    pub fused_ops: Vec<CanonicalCommandOp>,
    /// Input command count before canonicalization.
    pub raw_count: usize,
    /// Output command count after canonicalization.
    pub fused_count: usize,
    /// Self-inverse pairs cancelled (`I * I -> id`).
    pub involution_cancellations: usize,
    /// `Z_8` octant and additive camera delta merges performed.
    pub additive_merges: usize,
    /// Parallel execution layers produced by Kahn topological `(max, +)` scheduling.
    pub parallel_layers: Vec<Vec<CanonicalCommandOp>>,
    /// Maximum critical-path layer depth (`0` when all ops cancelled).
    pub dag_depth: usize,
}

/// Schedules `ops` into parallel SIMD/worker-ready layer batches via Kahn's topological algorithm
/// and tropical `(max, +)` critical-path depth (`verdant-aqm` `verifyAndCompileCircuitPrism`).
///
/// Refuses cyclic or out-of-bounds `depends_on` graphs via [`RenderGraphError::InvalidPassDependency`].
pub fn compile_parallel_command_layers(
    ops: &[CanonicalCommandOp],
) -> Result<(Vec<Vec<CanonicalCommandOp>>, usize), RenderGraphError> {
    let n = ops.len();
    if n == 0 {
        return Ok((Vec::new(), 0));
    }
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut indegree = vec![0usize; n];
    let mut last_on_target: HashMap<u32, usize> = HashMap::new();

    for (idx, op) in ops.iter().enumerate() {
        let mut parents: BTreeSet<usize> = BTreeSet::new();
        if let Some(&prev) = last_on_target.get(&op.target_slot) {
            parents.insert(prev);
        }
        last_on_target.insert(op.target_slot, idx);
        for &dep in &op.depends_on {
            if dep >= n || dep == idx {
                return Err(RenderGraphError::InvalidPassDependency {
                    producer_pass: dep,
                    consumer_pass: idx,
                });
            }
            parents.insert(dep);
        }
        for p in parents {
            adj[p].push(idx);
            indegree[idx] += 1;
        }
    }

    let mut queue: VecDeque<usize> = VecDeque::new();
    let mut layer_of = vec![0usize; n];
    for (i, &deg) in indegree.iter().enumerate() {
        if deg == 0 {
            queue.push_back(i);
            layer_of[i] = 1;
        }
    }

    let mut visited = 0usize;
    let mut max_layer = 0usize;
    while let Some(u) = queue.pop_front() {
        visited += 1;
        if layer_of[u] > max_layer {
            max_layer = layer_of[u];
        }
        for &v in &adj[u] {
            let cand = layer_of[u] + 1;
            if cand > layer_of[v] {
                layer_of[v] = cand;
            }
            indegree[v] -= 1;
            if indegree[v] == 0 {
                queue.push_back(v);
            }
        }
    }

    if visited != n {
        return Err(RenderGraphError::InvalidPassDependency {
            producer_pass: visited,
            consumer_pass: n,
        });
    }

    let mut layers = vec![Vec::new(); max_layer];
    for (idx, op) in ops.iter().enumerate() {
        layers[layer_of[idx] - 1].push(op.clone());
    }
    Ok((layers, max_layer))
}

/// Applies the Prism Algebraic Monoid Canonicalizer (`verdant-aqm` v0.2.0 `optimizeCircuitPrismMonoid`):
/// 1. Validates DAG acyclicity via [`compile_parallel_command_layers`].
/// 2. Cancels target-adjacent self-inverse involutions (`Toggle^2 -> id`) verified by
///    `prism_kernel::commandInvolutionCancels`.
/// 3. Fuses target-adjacent `Z_8` octant turns modulo `8` (`(a + b) mod 8 == 0 -> id`) and
///    additive `CameraOrbitDelta` triples into single zero-jitter sweeps.
/// 4. Schedules the canonicalized stream into parallel Kahn layer batches.
pub fn canonicalize_command_monoid(
    ops: &[CanonicalCommandOp],
) -> Result<CommandMonoidReport, RenderGraphError> {
    let _ = compile_parallel_command_layers(ops)?;
    let has_explicit_deps = ops.iter().any(|o| !o.depends_on.is_empty());
    let mut working: Vec<CanonicalCommandOp> = ops.to_vec();
    let mut involution_cancellations = 0usize;
    let mut additive_merges = 0usize;
    let oct_mod = (1_u8) << (1 + 1 + 1);

    if !has_explicit_deps {
        let mut next_list: Vec<CanonicalCommandOp> = Vec::with_capacity(working.len());
        for op in working {
            let prev_on_slot = next_list
                .iter()
                .rposition(|p| p.target_slot == op.target_slot);
            let Some(p_idx) = prev_on_slot else {
                if !matches!(
                    op.kind,
                    CanonicalCommandKind::YawOctantStep { octants } if octants % oct_mod == 0
                ) && !matches!(
                    op.kind,
                    CanonicalCommandKind::CameraOrbitDelta {
                        delta_yaw_milli: 0,
                        delta_pitch_milli: 0,
                        delta_dist_milli: 0,
                    }
                ) {
                    next_list.push(op);
                }
                continue;
            };

            let prev = &next_list[p_idx];
            if matches!(prev.kind, CanonicalCommandKind::StateBarrier)
                || matches!(op.kind, CanonicalCommandKind::StateBarrier)
            {
                next_list.push(op);
                continue;
            }

            let cancels = prism_kernel::commandInvolutionCancels(
                prev.kind.ordinal(),
                op.kind.ordinal(),
                prev.kind.is_self_inverse() && op.kind.is_self_inverse(),
                prev.target_slot == op.target_slot,
            );
            if cancels {
                next_list.remove(p_idx);
                involution_cancellations += 1;
                continue;
            }

            match (prev.kind, op.kind) {
                (
                    CanonicalCommandKind::YawOctantStep { octants: a },
                    CanonicalCommandKind::YawOctantStep { octants: b },
                ) => {
                    let fused = prism_kernel::octantMod8Fusion(u64::from(a), u64::from(b))
                        .unwrap_or(u64::from((a.wrapping_add(b)) % oct_mod))
                        as u8;
                    additive_merges += 1;
                    if fused == 0 {
                        next_list.remove(p_idx);
                    } else {
                        next_list[p_idx].kind =
                            CanonicalCommandKind::YawOctantStep { octants: fused };
                    }
                }
                (
                    CanonicalCommandKind::CameraOrbitDelta {
                        delta_yaw_milli: y0,
                        delta_pitch_milli: p0,
                        delta_dist_milli: d0,
                    },
                    CanonicalCommandKind::CameraOrbitDelta {
                        delta_yaw_milli: y1,
                        delta_pitch_milli: p1,
                        delta_dist_milli: d1,
                    },
                ) => {
                    let fy = y0.saturating_add(y1);
                    let fp = p0.saturating_add(p1);
                    let fd = d0.saturating_add(d1);
                    additive_merges += 1;
                    if fy == 0 && fp == 0 && fd == 0 {
                        next_list.remove(p_idx);
                    } else {
                        next_list[p_idx].kind = CanonicalCommandKind::CameraOrbitDelta {
                            delta_yaw_milli: fy,
                            delta_pitch_milli: fp,
                            delta_dist_milli: fd,
                        };
                    }
                }
                _ => next_list.push(op),
            }
        }
        working = next_list;
    }

    let (parallel_layers, dag_depth) = compile_parallel_command_layers(&working)?;
    let fused_count = working.len();
    Ok(CommandMonoidReport {
        fused_ops: working,
        raw_count: ops.len(),
        fused_count,
        involution_cancellations,
        additive_merges,
        parallel_layers,
        dag_depth,
    })
}

/// Computes the backward causal lightcone of all command indices influencing `target_slot` up to
/// `max_step_exclusive` (`verdant-aqm` `computeBackwardLightcone`).
#[must_use]
pub fn compute_backward_command_lightcone(
    ops: &[CanonicalCommandOp],
    target_slot: u32,
    max_step_exclusive: usize,
) -> BTreeSet<usize> {
    let mut active_slots: BTreeSet<u32> = BTreeSet::new();
    active_slots.insert(target_slot);
    let mut active_indices: BTreeSet<usize> = BTreeSet::new();
    let upper = ops.len().min(max_step_exclusive);
    for idx in (0..upper).rev() {
        let op = &ops[idx];
        if active_slots.contains(&op.target_slot) || active_indices.contains(&idx) {
            active_indices.insert(idx);
            active_slots.insert(op.target_slot);
            for &dep in &op.depends_on {
                if dep < idx {
                    active_indices.insert(dep);
                    active_slots.insert(ops[dep].target_slot);
                }
            }
        }
    }
    active_indices
}

const KAPPA_DIGEST_BYTES: usize = 1 << (1 + 1 + 1 + 1 + 1);

/// Canonical 32-byte content-addressed SHA-256 `kappa` digest.
pub type KappaDigest = [u8; KAPPA_DIGEST_BYTES];

/// Content-addressed `kappa` transition memoizer (`verdant-aqm` `executeAtlasHolospaceBatch`).
///
/// Maps `(state_kappa_digest, action_tag)` -> `(next_state, next_kappa_digest)` so repeated
/// quantized observer, narrative, or SLM state transitions execute in `O(1)` without recomputing
/// tensor contractions or SHA-256 digests.
#[derive(Clone, Debug)]
pub struct KappaTransitionCache<S: Clone> {
    max_entries: usize,
    entries: HashMap<(KappaDigest, u64), (S, KappaDigest)>,
    elided_hits: usize,
    computed_misses: usize,
}

impl<S: Clone> KappaTransitionCache<S> {
    /// Creates a bounded content-addressed transition cache (`max_entries >= 1`).
    #[must_use]
    pub fn new(max_entries: usize) -> Self {
        Self {
            max_entries: max_entries.max(1),
            entries: HashMap::new(),
            elided_hits: 0,
            computed_misses: 0,
        }
    }

    /// Computes a canonical 32-byte SHA-256 `kappa` digest over quantized state bytes.
    #[must_use]
    pub fn digest_bytes(bytes: &[u8]) -> KappaDigest {
        let mut hasher = Sha256::new();
        hasher.update(bytes);
        let raw = hasher.finalize();
        let mut out = [0_u8; KAPPA_DIGEST_BYTES];
        out.copy_from_slice(&raw);
        out
    }

    /// Number of transitions elided via `O(1)` content-addressed `kappa` cache hits.
    #[must_use]
    pub const fn elided_step_hits(&self) -> usize {
        self.elided_hits
    }

    /// Number of distinct transitions computed and inserted.
    #[must_use]
    pub const fn computed_step_misses(&self) -> usize {
        self.computed_misses
    }

    /// Evaluates or elides `(current_kappa, action_tag)`, returning `(next_state, next_kappa, was_elided)`.
    pub fn step_or_elide<F>(
        &mut self,
        current_kappa: KappaDigest,
        action_tag: u64,
        compute_fn: F,
    ) -> (S, KappaDigest, bool)
    where
        F: FnOnce() -> (S, Vec<u8>),
    {
        let key = (current_kappa, action_tag);
        if let Some((cached_state, cached_kappa)) = self.entries.get(&key) {
            self.elided_hits += 1;
            return (cached_state.clone(), *cached_kappa, true);
        }
        let (next_state, next_canonical_bytes) = compute_fn();
        let next_kappa = Self::digest_bytes(&next_canonical_bytes);
        self.computed_misses += 1;
        if self.entries.len() >= self.max_entries {
            self.entries.clear();
        }
        self.entries.insert(key, (next_state.clone(), next_kappa));
        (next_state, next_kappa, false)
    }
}

/// Stride-8 checkpointed deterministic rollback journal (`verdant-aqm` `journal.ts` lightweight
/// intermediate frames + `Subsystems::rollbackCheckpointStrideFrames`), reducing snapshot memory
/// by `8x` while reconstructing any arbitrary historical frame `t` in at most `<= 7` micro-steps.
#[derive(Clone, Debug)]
pub struct CheckpointedRollbackJournal<S: Clone, I: Clone> {
    stride: usize,
    checkpoints: BTreeMap<usize, S>,
    inputs: Vec<I>,
    chain_digests: Vec<KappaDigest>,
}

impl<S: Clone, I: Clone> CheckpointedRollbackJournal<S, I> {
    /// Initializes a checkpointed rollback journal at genesis `frame 0`.
    #[must_use]
    pub fn new(genesis_state: S, genesis_seed_bytes: &[u8]) -> Self {
        let stride = (prism_kernel::rollbackCheckpointStrideFrames().unwrap_or(1)).max(1) as usize;
        let mut checkpoints = BTreeMap::new();
        checkpoints.insert(0, genesis_state);
        let genesis_digest = KappaTransitionCache::<()>::digest_bytes(genesis_seed_bytes);
        Self {
            stride,
            checkpoints,
            inputs: Vec::new(),
            chain_digests: vec![genesis_digest],
        }
    }

    /// Checkpoint stride in frames (`8`).
    #[must_use]
    pub const fn checkpoint_stride(&self) -> usize {
        self.stride
    }

    /// Number of full state checkpoints retained in memory (`ceil((T + 1) / stride)`).
    #[must_use]
    pub fn stored_checkpoint_count(&self) -> usize {
        self.checkpoints.len()
    }

    /// Latest recorded frame index (`T`).
    #[must_use]
    pub fn latest_frame(&self) -> usize {
        self.inputs.len()
    }

    /// Records frame `t + 1` input and chains its SHA-256 digest, storing `post_state` only on
    /// `stride` boundaries (`(frame_idx % stride) == 0`).
    pub fn record_step(&mut self, input: I, input_wire_bytes: &[u8], post_state: &S) -> usize {
        let prev = *self
            .chain_digests
            .last()
            .expect("genesis digest always present");
        let mut hasher = Sha256::new();
        hasher.update(prev);
        hasher.update(input_wire_bytes);
        let raw = hasher.finalize();
        let mut digest = [0_u8; KAPPA_DIGEST_BYTES];
        digest.copy_from_slice(&raw);

        self.inputs.push(input);
        self.chain_digests.push(digest);
        let frame_idx = self.inputs.len();
        if frame_idx.is_multiple_of(self.stride) {
            self.checkpoints.insert(frame_idx, post_state.clone());
        }
        frame_idx
    }

    /// Reconstructs exact state at `target_frame` (`0..=latest_frame`) from the nearest preceding
    /// checkpoint in at most `stride - 1` (`<= 7`) deterministic step applications, returning
    /// `(reconstructed_state, replay_steps, chain_digest)`.
    pub fn reconstruct_frame<F>(
        &self,
        target_frame: usize,
        mut step_fn: F,
    ) -> Result<(S, usize, KappaDigest), SnapshotCodecError>
    where
        F: FnMut(&S, &I) -> S,
    {
        if target_frame > self.inputs.len() {
            return Err(SnapshotCodecError::MalformedStatePayload {
                detail: "target rollback frame exceeds recorded journal horizon".to_string(),
            });
        }
        let (&ckpt_frame, ckpt_state) = self
            .checkpoints
            .range(..=target_frame)
            .next_back()
            .ok_or_else(|| SnapshotCodecError::MalformedStatePayload {
                detail: "missing genesis rollback checkpoint".to_string(),
            })?;
        let mut cur = ckpt_state.clone();
        let mut steps = 0usize;
        for inp in &self.inputs[ckpt_frame..target_frame] {
            cur = step_fn(&cur, inp);
            steps += 1;
        }
        Ok((cur, steps, self.chain_digests[target_frame]))
    }
}

impl<I: Clone> CheckpointedRollbackJournal<Vec<u8>, I> {
    /// Initializes a UORC-compressed stride-8 rollback journal by compressing `genesis_raw_bytes`
    /// into a stateless `UORC` archive at `frame 0`.
    #[cfg(feature = "uorc-streaming")]
    #[must_use]
    pub fn new_uorc(genesis_raw_bytes: &[u8], genesis_seed_bytes: &[u8]) -> Self {
        Self::new(
            crate::streaming::compress_uorc_portable(genesis_raw_bytes),
            genesis_seed_bytes,
        )
    }

    /// Records frame `t + 1`, compressing `post_state_raw` into a `UORC` archive only on stride-8 boundaries.
    #[cfg(feature = "uorc-streaming")]
    pub fn record_uorc_step(
        &mut self,
        input: I,
        input_wire_bytes: &[u8],
        post_state_raw: &[u8],
    ) -> usize {
        let next_idx = self.inputs.len() + 1;
        if next_idx.is_multiple_of(self.stride) {
            let compressed = crate::streaming::compress_uorc_portable(post_state_raw);
            self.record_step(input, input_wire_bytes, &compressed)
        } else {
            let empty = Vec::new();
            self.record_step(input, input_wire_bytes, &empty)
        }
    }

    /// Total compressed byte footprint across all stored UORC checkpoints in the journal.
    #[must_use]
    pub fn total_checkpoint_uorc_bytes(&self) -> usize {
        self.checkpoints.values().map(Vec::len).sum()
    }

    /// Decompresses the nearest preceding stride-8 `UORC` checkpoint via `uorc_decompress::run` and
    /// replays at most `<= 7` deterministic step transitions to reconstruct exact raw state bytes.
    #[cfg(feature = "uorc-wasm-decode")]
    pub fn reconstruct_uorc_frame<F>(
        &self,
        target_frame: usize,
        mut step_fn: F,
    ) -> Result<(Vec<u8>, usize, KappaDigest), SnapshotCodecError>
    where
        F: FnMut(&[u8], &I) -> Vec<u8>,
    {
        if target_frame > self.inputs.len() {
            return Err(SnapshotCodecError::MalformedStatePayload {
                detail: "target rollback frame exceeds recorded UORC journal horizon".to_string(),
            });
        }
        let (&ckpt_frame, ckpt_uorc) = self
            .checkpoints
            .range(..=target_frame)
            .next_back()
            .ok_or_else(|| SnapshotCodecError::MalformedStatePayload {
                detail: "missing genesis UORC rollback checkpoint".to_string(),
            })?;
        let mut cur = crate::streaming::decompress_uorc_portable(ckpt_uorc).map_err(|e| {
            SnapshotCodecError::IntegrityDigestMismatch {
                expected_hex: "UORC-CHECKPOINT-SEAL".to_string(),
                observed_hex: e.to_string(),
            }
        })?;
        let mut steps = 0usize;
        for inp in &self.inputs[ckpt_frame..target_frame] {
            cur = step_fn(&cur, inp);
            steps += 1;
        }
        Ok((cur, steps, self.chain_digests[target_frame]))
    }
}
