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
    ) -> Result<EngineConfig, EngineConfigError> {
        if viewport_width == 0 || viewport_height == 0 {
            return Err(EngineConfigError::ZeroViewportDimension {
                width: viewport_width,
                height: viewport_height,
            });
        }
        let two_u32 = 1_u32.wrapping_add(1_u32);
        let five_u32 = two_u32.wrapping_mul(two_u32).wrapping_add(1_u32);
        let ten_u32 = two_u32.wrapping_mul(five_u32);
        let max_hz = ten_u32.wrapping_mul(ten_u32).wrapping_mul(ten_u32);
        if tick_rate_hz == 0 || tick_rate_hz > max_hz {
            return Err(EngineConfigError::InvalidTickRateHz {
                tick_rate_hz,
                max_hz,
            });
        }
        let k_u64 = max_hz as u64;
        let max_micros = k_u64.wrapping_mul(k_u64);
        if frame_budget_micros == 0 || frame_budget_micros > max_micros {
            return Err(EngineConfigError::InvalidFrameBudgetMicros {
                budget_micros: frame_budget_micros,
                max_micros,
            });
        }
        if worker_threads == 0 {
            return Err(EngineConfigError::ZeroWorkerThreads);
        }
        Ok(EngineConfig {
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
        self.samples_micros
            .iter()
            .fold(0_u64, |acc, &v| if v > acc { v } else { acc })
    }

    /// Returns whether every recorded frame stayed within `budget_micros`.
    #[must_use]
    pub const fn meets_budget_kpi(&self) -> bool {
        self.over_budget_frames == 0 && !self.samples_micros.is_empty()
    }
}

const SNAPSHOT_MAGIC: &[u8; DIM_HOMOGENEOUS] = b"VDNT";
const SNAPSHOT_VERSION: u8 = 1;

use crate::digest::hex_encode as bytes_to_hex;

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

fn encode_scene_payload(world: &StandardSceneState) -> Result<Vec<u8>, SnapshotCodecError> {
    let mut buf = Vec::new();
    macro_rules! w_u8 {
        ($v:expr) => {
            buf.push($v)
        };
    }
    macro_rules! w_u64 {
        ($v:expr) => {
            buf.extend_from_slice(&($v as u64).to_le_bytes())
        };
    }
    macro_rules! w_f64 {
        ($v:expr) => {
            buf.extend_from_slice(&($v as f64).to_le_bytes())
        };
    }
    macro_rules! w_str {
        ($s:expr) => {{
            let bytes = ($s).as_bytes();
            buf.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
            buf.extend_from_slice(bytes);
        }};
    }
    macro_rules! w_vec3 {
        ($v:expr) => {{
            let v: Vec3 = $v;
            w_f64!(v.x);
            w_f64!(v.y);
            w_f64!(v.z);
        }};
    }
    macro_rules! w_enum {
        ($table:expr, $val:expr) => {{
            let idx = $table.iter().position(|item| item == $val).ok_or_else(|| {
                SnapshotCodecError::MalformedStatePayload {
                    detail: "unrecognized enum variant during snapshot encode".to_string(),
                }
            })?;
            w_u8!(idx as u8);
        }};
    }
    macro_rules! w_opt_u64 {
        ($v:expr) => {
            match $v {
                None => w_u8!(0),
                Some(val) => {
                    w_u8!(1);
                    w_u64!(val);
                }
            }
        };
    }

    w_u64!(world.seed);
    let env = &world.params.environment;
    w_f64!(env.gravity_mps2);
    w_f64!(env.fluid_density_kg_m3);
    w_f64!(env.sound_speed_mps);
    w_f64!(env.key_light_temperature_kelvin);
    w_f64!(env.key_light_irradiance_w_m2);
    w_f64!(env.water_level_meters);

    let ter = &world.params.terrain;
    w_f64!(ter.basin_depth_meters);
    w_f64!(ter.basin_variance_m2);
    w_f64!(ter.basin_center_x);
    w_f64!(ter.basin_center_z);
    w_f64!(ter.hill_amplitude_meters);
    w_f64!(ter.hill_frequency_per_meter);
    w_f64!(ter.outer_rim_curvature);
    w_f64!(ter.outer_rim_cap_m2);

    w_u64!(world.params.max_foliage_instances);
    w_u64!(world.params.max_agent_count);
    w_u64!(world.params.crystal_prop_count);
    w_u64!(world.params.spline_waypoint_count);
    w_f64!(world.params.spline_orbit_radius_meters);

    w_enum!(ALL_LOD_LEVELS, &world.active_lod);
    w_f64!(world.resolution_scale);
    w_u64!(world.foliage_count);
    w_u64!(world.agent_count);

    w_u64!(world.scene_nodes.len());
    for node in &world.scene_nodes {
        w_u64!(node.preset_index);
        w_str!(&node.id_slug);
        w_str!(&node.title);
        w_enum!(ALL_SCENE_NODE_KINDS, &node.kind);
        w_vec3!(node.world_position);
        w_f64!(node.span_meters);
        w_f64!(node.height_meters);
        w_f64!(node.orbit_distance_meters);
        w_f64!(node.modal_frequency_hz);
        w_u8!(u8::from(node.follow_mover));
    }

    w_u64!(world.mover_spline_waypoints.len());
    for &wp in &world.mover_spline_waypoints {
        w_vec3!(wp);
    }

    w_u64!(world.scene.next_id);
    w_u64!(world.scene.entities.len());
    for rec in world.scene.entities.values() {
        w_u64!(rec.id.0);
        w_str!(&rec.name);
        w_enum!(ALL_ENTITY_ROLES, &rec.role);
        w_opt_u64!(rec.parent.map(|p| p.0));
        w_u64!(rec.children.len());
        for child in &rec.children {
            w_u64!(child.0);
        }
        w_vec3!(rec.local_transform.translation);
        let q = rec.local_transform.rotation;
        w_f64!(q.x);
        w_f64!(q.y);
        w_f64!(q.z);
        w_f64!(q.w);
        w_vec3!(rec.local_transform.scale);
        w_vec3!(rec.local_bounds.min);
        w_vec3!(rec.local_bounds.max);
        w_opt_u64!(rec.mesh_index.map(|m| m as u64));
        w_opt_u64!(rec.material_index.map(|m| m as u64));
        w_f64!(rec.emerged_at_lod_scale);
        w_str!(&rec.provenance_tag);
    }
    Ok(buf)
}

fn decode_scene_payload(payload: &[u8]) -> Result<StandardSceneState, SnapshotCodecError> {
    let mut cursor = 0usize;
    macro_rules! r_exact {
        ($len:expr) => {{
            let len: usize = $len;
            let end = cursor.checked_add(len).ok_or_else(|| {
                SnapshotCodecError::MalformedStatePayload {
                    detail: "snapshot cursor overflow".to_string(),
                }
            })?;
            if end > payload.len() {
                return Err(SnapshotCodecError::MalformedStatePayload {
                    detail: "unexpected end of binary scene payload".to_string(),
                });
            }
            let slice = &payload[cursor..end];
            cursor = end;
            slice
        }};
    }
    macro_rules! r_u8 {
        () => {
            r_exact!(1)[0]
        };
    }
    macro_rules! r_u32 {
        () => {{
            let raw = r_exact!(std::mem::size_of::<u32>());
            let mut arr = [0_u8; std::mem::size_of::<u32>()];
            arr.copy_from_slice(raw);
            u32::from_le_bytes(arr)
        }};
    }
    macro_rules! r_u64 {
        () => {{
            let raw = r_exact!(std::mem::size_of::<u64>());
            let mut arr = [0_u8; std::mem::size_of::<u64>()];
            arr.copy_from_slice(raw);
            u64::from_le_bytes(arr)
        }};
    }
    macro_rules! r_usize {
        () => {
            r_u64!() as usize
        };
    }
    macro_rules! r_f64 {
        () => {{
            let raw = r_exact!(std::mem::size_of::<f64>());
            let mut arr = [0_u8; std::mem::size_of::<f64>()];
            arr.copy_from_slice(raw);
            f64::from_le_bytes(arr)
        }};
    }
    macro_rules! r_str {
        () => {{
            let len = r_u32!() as usize;
            let raw = r_exact!(len);
            std::str::from_utf8(raw).map(str::to_owned).map_err(|err| {
                SnapshotCodecError::MalformedStatePayload {
                    detail: err.to_string(),
                }
            })?
        }};
    }
    macro_rules! r_vec3 {
        () => {
            Vec3::new(r_f64!(), r_f64!(), r_f64!())
        };
    }
    macro_rules! r_enum {
        ($table:expr) => {{
            let idx = r_u8!() as usize;
            $table
                .get(idx)
                .copied()
                .ok_or_else(|| SnapshotCodecError::MalformedStatePayload {
                    detail: "invalid enum ordinal in binary scene payload".to_string(),
                })?
        }};
    }
    macro_rules! r_opt_u64 {
        () => {
            match r_u8!() {
                0 => None,
                1 => Some(r_u64!()),
                _ => {
                    return Err(SnapshotCodecError::MalformedStatePayload {
                        detail: "invalid option tag in binary scene payload".to_string(),
                    })
                }
            }
        };
    }

    let seed = r_u64!();
    let environment = PhysicalEnvironment {
        gravity_mps2: r_f64!(),
        fluid_density_kg_m3: r_f64!(),
        sound_speed_mps: r_f64!(),
        key_light_temperature_kelvin: r_f64!(),
        key_light_irradiance_w_m2: r_f64!(),
        water_level_meters: r_f64!(),
    };
    let terrain = TerrainReliefParams {
        basin_depth_meters: r_f64!(),
        basin_variance_m2: r_f64!(),
        basin_center_x: r_f64!(),
        basin_center_z: r_f64!(),
        hill_amplitude_meters: r_f64!(),
        hill_frequency_per_meter: r_f64!(),
        outer_rim_curvature: r_f64!(),
        outer_rim_cap_m2: r_f64!(),
    };
    let params = SceneBuildParams {
        environment,
        terrain,
        max_foliage_instances: r_usize!(),
        max_agent_count: r_usize!(),
        crystal_prop_count: r_usize!(),
        spline_waypoint_count: r_usize!(),
        spline_orbit_radius_meters: r_f64!(),
    };
    let active_lod = r_enum!(ALL_LOD_LEVELS);
    let resolution_scale = r_f64!();
    let foliage_count = r_usize!();
    let agent_count = r_usize!();

    let node_len = r_usize!();
    let mut scene_nodes = Vec::with_capacity(node_len.min(payload.len()));
    for _ in 0..node_len {
        let preset_index = r_u64!();
        let id_slug = r_str!();
        let title = r_str!();
        let kind = r_enum!(ALL_SCENE_NODE_KINDS);
        let world_position = r_vec3!();
        let span_meters = r_f64!();
        let height_meters = r_f64!();
        let orbit_distance_meters = r_f64!();
        let modal_frequency_hz = r_f64!();
        let follow_mover = match r_u8!() {
            0 => false,
            1 => true,
            _ => {
                return Err(SnapshotCodecError::MalformedStatePayload {
                    detail: "invalid boolean byte in binary scene payload".to_string(),
                });
            }
        };
        scene_nodes.push(SceneNodeInstance {
            preset_index,
            id_slug,
            title,
            kind,
            world_position,
            span_meters,
            height_meters,
            orbit_distance_meters,
            modal_frequency_hz,
            follow_mover,
        });
    }

    let wp_len = r_usize!();
    let mut mover_spline_waypoints = Vec::with_capacity(wp_len.min(payload.len()));
    for _ in 0..wp_len {
        mover_spline_waypoints.push(r_vec3!());
    }

    let next_id = r_u64!();
    let ent_len = r_usize!();
    let mut entities = BTreeMap::new();
    for _ in 0..ent_len {
        let id = EntityId(r_u64!());
        let name = r_str!();
        let role = r_enum!(ALL_ENTITY_ROLES);
        let parent = r_opt_u64!().map(EntityId);
        let child_len = r_usize!();
        let mut children = Vec::with_capacity(child_len.min(payload.len()));
        for _ in 0..child_len {
            children.push(EntityId(r_u64!()));
        }
        let local_transform = Transform {
            translation: r_vec3!(),
            rotation: Quat {
                x: r_f64!(),
                y: r_f64!(),
                z: r_f64!(),
                w: r_f64!(),
            },
            scale: r_vec3!(),
        };
        let local_bounds = Aabb::new(r_vec3!(), r_vec3!());
        let mesh_index = r_opt_u64!().map(|v| v as usize);
        let material_index = r_opt_u64!().map(|v| v as usize);
        let emerged_at_lod_scale = r_f64!();
        let provenance_tag = r_str!();
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
    if cursor != payload.len() {
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
    #[cfg(any(feature = "uorc-streaming", feature = "uorc-wasm-decode"))]
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

#[path = "runtime_monoid.rs"]
mod runtime_monoid;
pub use runtime_monoid::*;
