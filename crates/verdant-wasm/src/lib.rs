//! Safe WebAssembly runtime bridge (`verdant-wasm`) over `verdant-core` and `prism_kernel`.
//!
//! Exposes a zero-`unsafe`-block C-ABI surface for dedicated browser `OffscreenCanvas` simulation
//! workers, `AudioWorklet` synthesis nodes, and `#vdnt=` SHA-256 world snapshot sharing, as well
//! as a typed Rust [`WasmRuntimeBridge`] API covered by conformance test `VE-15`.

use std::fmt;
use std::sync::{Mutex, OnceLock};

use verdant_core::audio::{
    evaluate_binaural_cue, AcousticEnvironment, ListenerPose, SpatialAudioSource,
};
use verdant_core::collision::{gjk_intersect_and_manifold, ConvexHullCollider};
use verdant_core::ecs::BoundingVolumeHierarchy;
use verdant_core::genesis::{terrain_height_with_params, CausalWorldState, CosmicEpoch};
use verdant_core::geometry::canonical_scene_mesh_library;
use verdant_core::math::{Frustum, Vec3};
use verdant_core::navmesh::NavMesh;
use verdant_core::physics::{
    CapsuleController, KinematicSplineRover, PhysicsWorld, PhysicsWorldConfig, RigidBody,
};
use verdant_core::prism_kernel;
use verdant_core::render::compute_csm_splits;
use verdant_core::runtime::WorldSnapshot;

/// Refusal returned when a browser worker command, epoch rank, or `VDNT` snapshot envelope is invalid.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WasmBridgeError {
    /// Requested cosmic epoch rank exceeds `0..=3`.
    InvalidEpochRank {
        /// Supplied rank.
        rank: u32,
    },
    /// Snapshot payload failed Prism bridge envelope bounds (`0 < len <= snapshotUrlMaxBytes`) or SHA-256 verification.
    InvalidSnapshotEnvelope {
        /// Observed byte length.
        byte_len: usize,
        /// Diagnostic reason.
        reason: String,
    },
    /// Worker command ring capacity was exceeded before draining.
    CommandRingOverflow {
        ///Configured capacity from `prism_kernel::workerCommandRingCapacity()`.
        capacity: usize,
    },
}

impl fmt::Display for WasmBridgeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidEpochRank { rank } => {
                write!(f, "invalid cosmic epoch rank {rank}; expected 0..=3")
            }
            Self::InvalidSnapshotEnvelope { byte_len, reason } => {
                write!(
                    f,
                    "invalid VDNT snapshot envelope ({byte_len} bytes): {reason}"
                )
            }
            Self::CommandRingOverflow { capacity } => {
                write!(
                    f,
                    "worker command ring exceeded capacity of {capacity} commands"
                )
            }
        }
    }
}

impl std::error::Error for WasmBridgeError {}

/// Live simulation, culling, acoustic, and snapshot state hosted inside the Wasm worker.
pub struct WasmRuntimeBridge {
    /// Causal world state at the active cosmic expansion epoch.
    pub world: CausalWorldState,
    /// Active cosmic expansion rank (`0..=3`).
    pub epoch_rank: u32,
    /// Deterministic rigid-body, capsule, and perimeter sentinel rover physics world.
    pub physics: PhysicsWorld,
    /// Prebuilt SAH BVH over the world scene graph.
    pub bvh: BoundingVolumeHierarchy,
    /// Walkable polygon count extracted from the sanctuary terrain mesh.
    pub walkable_polygons: usize,
    /// Representative Builder monument hull for GJK proximity checks.
    pub rotunda_hull: ConvexHullCollider,
    /// Player hull for GJK proximity checks.
    pub player_hull: ConvexHullCollider,
    /// Whether the player hull currently overlaps the central monument hull.
    pub monument_contact: bool,
    /// Queued command count in the current frame window.
    pub queued_commands: usize,
    /// Cached binaural direct gain from the most recent `evaluate_binaural` call.
    pub last_binaural_gain: f64,
    /// Cached binaural ITD in microseconds from the most recent `evaluate_binaural` call.
    pub last_binaural_itd_micros: f64,
    /// Byte staging buffer for safe Wasm host<->guest snapshot and query exchange.
    pub io_buffer: Vec<u8>,
}

fn scale_for_epoch_rank(rank: u32) -> Result<f64, WasmBridgeError> {
    if rank > 3 {
        return Err(WasmBridgeError::InvalidEpochRank { rank });
    }
    let _spec = prism_kernel::epochVisualByRank(u64::from(rank))
        .map_err(|_| WasmBridgeError::InvalidEpochRank { rank })?;
    let quartiles = prism_kernel::sFour()
        .map_err(|_| WasmBridgeError::InvalidEpochRank { rank })?
        .max(1) as f64;
    Ok(f64::from(rank + 1) / quartiles)
}

impl WasmRuntimeBridge {
    /// Initializes a deterministic `t_obs` sanctuary runtime from `seed`.
    pub fn new(seed: u64) -> Result<Self, WasmBridgeError> {
        let mut world = CausalWorldState::new_cosmogenic(seed.max(1)).map_err(|e| {
            WasmBridgeError::InvalidSnapshotEnvelope {
                byte_len: 0,
                reason: e.to_string(),
            }
        })?;
        world
            .advance_to(1.0)
            .map_err(|e| WasmBridgeError::InvalidSnapshotEnvelope {
                byte_len: 0,
                reason: e.to_string(),
            })?;

        let physics = Self::build_physics(&world);
        let bvh = world.scene.build_bvh();
        let meshes = canonical_scene_mesh_library(world.seed);
        let walkable_polygons = NavMesh::from_mesh(&meshes[0])
            .map(|nm| nm.polygons.len())
            .unwrap_or(0);
        let rotunda_hull = ConvexHullCollider::from_box_half_extents(Vec3::new(3.5, 3.0, 3.5))
            .expect("valid rotunda box");
        let player_hull = ConvexHullCollider::from_box_half_extents(Vec3::new(0.4, 0.9, 0.4))
            .expect("valid player box");

        Ok(Self {
            world,
            epoch_rank: 3,
            physics,
            bvh,
            walkable_polygons,
            rotunda_hull,
            player_hull,
            monument_contact: false,
            queued_commands: 0,
            last_binaural_gain: 0.0,
            last_binaural_itd_micros: 0.0,
            io_buffer: Vec::with_capacity(4096),
        })
    }

    fn build_physics(world: &CausalWorldState) -> PhysicsWorld {
        let cfg = PhysicsWorldConfig {
            terrain_seed: world.seed,
            ..Default::default()
        };
        let mut physics = PhysicsWorld::new(cfg);
        if let Ok(buoy) = RigidBody::new(
            "LagoonResonatorBuoy",
            Vec3::new(5.5, 0.2, 8.5),
            12.0,
            0.45,
            0.35,
        ) {
            physics.bodies.push(buoy);
        }
        physics.player_capsule = Some(CapsuleController::new(Vec3::new(0.0, 2.2, 16.0)));
        if !world.rover_perimeter_waypoints.is_empty() {
            physics.sentinel_rover = Some(KinematicSplineRover::new(
                world.rover_perimeter_waypoints.clone(),
                2.6,
            ));
        }
        physics
    }

    /// Enqueues a host command against `prism_kernel::workerCommandRingCapacity()`.
    pub fn record_command(&mut self) -> Result<(), WasmBridgeError> {
        let cap = prism_kernel::workerCommandRingCapacity()
            .expect("verified Prism workerCommandRingCapacity") as usize;
        if self.queued_commands >= cap {
            return Err(WasmBridgeError::CommandRingOverflow { capacity: cap });
        }
        self.queued_commands += 1;
        Ok(())
    }

    /// Rebuilds causal world state at `rank` (`0..=3`) using monotonic expansion from seed.
    pub fn set_epoch_rank(&mut self, rank: u32) -> Result<(), WasmBridgeError> {
        let target_scale = scale_for_epoch_rank(rank)?;
        let mut fresh = CausalWorldState::new_cosmogenic(self.world.seed).map_err(|e| {
            WasmBridgeError::InvalidSnapshotEnvelope {
                byte_len: 0,
                reason: e.to_string(),
            }
        })?;
        for step_rank in 0..=rank {
            let s = scale_for_epoch_rank(step_rank)?;
            if s > fresh.scale_factor {
                fresh.advance_to(s.min(target_scale)).map_err(|e| {
                    WasmBridgeError::InvalidSnapshotEnvelope {
                        byte_len: 0,
                        reason: e.to_string(),
                    }
                })?;
            }
        }
        self.bvh = fresh.scene.build_bvh();
        if rank == 3
            && self.physics.sentinel_rover.is_none()
            && !fresh.rover_perimeter_waypoints.is_empty()
        {
            self.physics.sentinel_rover = Some(KinematicSplineRover::new(
                fresh.rover_perimeter_waypoints.clone(),
                2.6,
            ));
        }
        self.world = fresh;
        self.epoch_rank = rank;
        Ok(())
    }

    /// Advances physics, capsule movement, rover spline patrol, and GJK monument contact.
    pub fn step_frame(
        &mut self,
        dt_seconds: f64,
        move_forward: f64,
        move_strafe: f64,
        walk_yaw_rad: f64,
    ) -> Result<(), WasmBridgeError> {
        self.queued_commands = 0;
        let clamped_dt = dt_seconds.clamp(0.001, 0.05);
        let wish = Vec3::new(
            walk_yaw_rad.sin() * move_forward - walk_yaw_rad.cos() * move_strafe,
            0.0,
            walk_yaw_rad.cos() * move_forward + walk_yaw_rad.sin() * move_strafe,
        );
        self.physics.step(clamped_dt, wish).map_err(|e| {
            WasmBridgeError::InvalidSnapshotEnvelope {
                byte_len: 0,
                reason: e.to_string(),
            }
        })?;

        if let Some(capsule) = &self.physics.player_capsule {
            if let Ok(manifold) = gjk_intersect_and_manifold(
                &self.rotunda_hull,
                Vec3::new(0.0, 2.2, 0.0),
                &self.player_hull,
                capsule.foot_position,
            ) {
                self.monument_contact = manifold.intersecting;
            }
        }
        Ok(())
    }

    /// Encodes the active `CausalWorldState` into a `VDNT` + SHA-256 snapshot verified by
    /// `prism_kernel::wasmBridgeEnvelopeValid`.
    pub fn encode_snapshot(&self) -> Result<Vec<u8>, WasmBridgeError> {
        let bytes = WorldSnapshot::encode(&self.world).map_err(|e| {
            WasmBridgeError::InvalidSnapshotEnvelope {
                byte_len: 0,
                reason: e.to_string(),
            }
        })?;
        let quantum = prism_kernel::audioWorkletQuantumFrames()
            .expect("verified Prism audioWorkletQuantumFrames");
        let channels = prism_kernel::audioStereoChannels();
        let envelope_ok =
            prism_kernel::wasmBridgeEnvelopeValid(quantum, channels, bytes.len() as u64)
                .unwrap_or(false);
        if !envelope_ok {
            return Err(WasmBridgeError::InvalidSnapshotEnvelope {
                byte_len: bytes.len(),
                reason: "snapshot exceeds Prism URL bridge envelope".to_string(),
            });
        }
        Ok(bytes)
    }

    /// Verifies SHA-256 digest and Prism bridge envelope, then restores `CausalWorldState`.
    pub fn decode_and_restore_snapshot(&mut self, wire: &[u8]) -> Result<(), WasmBridgeError> {
        let quantum = prism_kernel::audioWorkletQuantumFrames()
            .expect("verified Prism audioWorkletQuantumFrames");
        let channels = prism_kernel::audioStereoChannels();
        let envelope_ok =
            prism_kernel::wasmBridgeEnvelopeValid(quantum, channels, wire.len() as u64)
                .unwrap_or(false);
        if !envelope_ok {
            return Err(WasmBridgeError::InvalidSnapshotEnvelope {
                byte_len: wire.len(),
                reason: "payload outside Prism bridge byte envelope".to_string(),
            });
        }
        let restored =
            WorldSnapshot::decode(wire).map_err(|e| WasmBridgeError::InvalidSnapshotEnvelope {
                byte_len: wire.len(),
                reason: e.to_string(),
            })?;
        self.epoch_rank = match restored.active_epoch {
            CosmicEpoch::Cosmogenic => 0,
            CosmicEpoch::LithicHydrologic => 1,
            CosmicEpoch::BioticEcological => 2,
            CosmicEpoch::AgenticArchitectural => 3,
        };
        self.bvh = restored.scene.build_bvh();
        self.physics = Self::build_physics(&restored);
        self.world = restored;
        Ok(())
    }
}

fn global_bridge() -> &'static Mutex<Option<WasmRuntimeBridge>> {
    static STATE: OnceLock<Mutex<Option<WasmRuntimeBridge>>> = OnceLock::new();
    STATE.get_or_init(|| Mutex::new(None))
}

/// Initializes the global Wasm engine bridge with `seed` (`1` = success, `0` = refusal).
#[no_mangle]
pub extern "C" fn verdant_wasm_init(seed: u64) -> u32 {
    match WasmRuntimeBridge::new(seed) {
        Ok(bridge) => {
            if let Ok(mut guard) = global_bridge().lock() {
                *guard = Some(bridge);
                return 1;
            }
            0
        }
        Err(_) => 0,
    }
}

/// Sets the active cosmic expansion epoch rank (`0..=3`).
#[no_mangle]
pub extern "C" fn verdant_wasm_set_epoch_rank(rank: u32) -> u32 {
    let Ok(mut guard) = global_bridge().lock() else {
        return 0;
    };
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };
    if bridge.record_command().is_err() {
        return 0;
    }
    u32::from(bridge.set_epoch_rank(rank).is_ok())
}

/// Advances simulation by `dt_micros` with player milli-inputs (`1` = success, `0` = refusal).
#[no_mangle]
pub extern "C" fn verdant_wasm_step(
    dt_micros: u32,
    move_fwd_milli: i32,
    move_strafe_milli: i32,
    walk_yaw_milli: i32,
) -> u32 {
    let Ok(mut guard) = global_bridge().lock() else {
        return 0;
    };
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };
    let dt_sec = f64::from(dt_micros) / 1_000_000.0;
    let fwd = f64::from(move_fwd_milli) / 1_000.0;
    let strafe = f64::from(move_strafe_milli) / 1_000.0;
    let yaw = f64::from(walk_yaw_milli) / 1_000.0;
    u32::from(bridge.step_frame(dt_sec, fwd, strafe, yaw).is_ok())
}

/// Current autonomous Perimeter Sentinel Rover X coordinate in world meters.
#[no_mangle]
pub extern "C" fn verdant_wasm_rover_x() -> f64 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| {
            g.as_ref()?
                .physics
                .sentinel_rover
                .as_ref()
                .map(|r| r.position.x)
        })
        .unwrap_or(18.0)
}

/// Current autonomous Perimeter Sentinel Rover Y coordinate in world meters.
#[no_mangle]
pub extern "C" fn verdant_wasm_rover_y() -> f64 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| {
            g.as_ref()?
                .physics
                .sentinel_rover
                .as_ref()
                .map(|r| r.position.y)
        })
        .unwrap_or(1.5)
}

/// Current autonomous Perimeter Sentinel Rover Z coordinate in world meters.
#[no_mangle]
pub extern "C" fn verdant_wasm_rover_z() -> f64 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| {
            g.as_ref()?
                .physics
                .sentinel_rover
                .as_ref()
                .map(|r| r.position.z)
        })
        .unwrap_or(0.0)
}

/// Current autonomous Perimeter Sentinel Rover heading yaw in radians.
#[no_mangle]
pub extern "C" fn verdant_wasm_rover_yaw() -> f64 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| {
            let f = g.as_ref()?.physics.sentinel_rover.as_ref()?.heading;
            Some(f.x.atan2(f.z))
        })
        .unwrap_or(0.0)
}

/// Cumulative distance traveled in meters by the autonomous Perimeter Sentinel Rover.
#[no_mangle]
pub extern "C" fn verdant_wasm_rover_distance() -> f64 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| {
            g.as_ref()?
                .physics
                .sentinel_rover
                .as_ref()
                .map(|r| r.distance_traveled_meters)
        })
        .unwrap_or(0.0)
}

/// Current player capsule eye-level X coordinate in world meters.
#[no_mangle]
pub extern "C" fn verdant_wasm_player_x() -> f64 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| {
            g.as_ref()?
                .physics
                .player_capsule
                .as_ref()
                .map(|c| c.foot_position.x)
        })
        .unwrap_or(0.0)
}

/// Current player capsule eye-level Y coordinate in world meters.
#[no_mangle]
pub extern "C" fn verdant_wasm_player_y() -> f64 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| {
            g.as_ref()?
                .physics
                .player_capsule
                .as_ref()
                .map(|c| c.foot_position.y + c.eye_height_meters)
        })
        .unwrap_or(2.2)
}

/// Current player capsule eye-level Z coordinate in world meters.
#[no_mangle]
pub extern "C" fn verdant_wasm_player_z() -> f64 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| {
            g.as_ref()?
                .physics
                .player_capsule
                .as_ref()
                .map(|c| c.foot_position.z)
        })
        .unwrap_or(16.0)
}

/// Evaluates continuous sanctuary terrain elevation at `(x, z)` scaled by `amp`.
#[no_mangle]
pub extern "C" fn verdant_wasm_terrain_height(x: f64, z: f64, amp: f64) -> f64 {
    let Ok(guard) = global_bridge().lock() else {
        return 0.0;
    };
    let Some(bridge) = guard.as_ref() else {
        return 0.0;
    };
    terrain_height_with_params(bridge.world.seed, &bridge.world.params.terrain, x, z) * amp
}

/// Returns `1` when the player hull is within GJK contact of the central Rotunda monument, else `0`.
#[no_mangle]
pub extern "C" fn verdant_wasm_monument_contact() -> u32 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| Some(u32::from(g.as_ref()?.monument_contact)))
        .unwrap_or(0)
}

/// Counts visible scene entities inside the perspective camera frustum via SAH BVH culling.
#[no_mangle]
pub extern "C" fn verdant_wasm_bvh_visible_count(
    eye_x: f64,
    eye_y: f64,
    eye_z: f64,
    fwd_x: f64,
    fwd_y: f64,
    fwd_z: f64,
    aspect: f64,
) -> u32 {
    let Ok(guard) = global_bridge().lock() else {
        return 0;
    };
    let Some(bridge) = guard.as_ref() else {
        return 0;
    };
    let frustum = Frustum::from_perspective(
        Vec3::new(eye_x, eye_y, eye_z),
        Vec3::new(fwd_x, fwd_y, fwd_z),
        Vec3::UP,
        std::f64::consts::PI / 3.4,
        aspect.max(0.2),
        0.25,
        140.0,
    );
    bridge.bvh.query_frustum(&frustum).len() as u32
}

/// Walkable terrain NavMesh polygon count verified by `prism_kernel::navTriangleWalkable`.
#[no_mangle]
pub extern "C" fn verdant_wasm_navmesh_polygon_count() -> u32 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| Some(g.as_ref()?.walkable_polygons as u32))
        .unwrap_or(0)
}

/// Returns `AudioWorklet` render quantum frame count (`128`) from `prism_kernel`.
#[no_mangle]
pub extern "C" fn verdant_wasm_audio_quantum_frames() -> u32 {
    prism_kernel::audioWorkletQuantumFrames().unwrap_or(128) as u32
}

/// Returns stereo channel count (`2`) from `prism_kernel`.
#[no_mangle]
pub extern "C" fn verdant_wasm_audio_channels() -> u32 {
    prism_kernel::audioStereoChannels() as u32
}

/// Returns Euler-Bernoulli modal frequency numerator `(2n + 1)^2` from `prism_kernel`.
#[no_mangle]
pub extern "C" fn verdant_wasm_chime_mode_numer(mode_index: u32) -> u32 {
    prism_kernel::eulerBernoulliModeNumer(u64::from(mode_index)).unwrap_or(9) as u32
}

/// Returns Euler-Bernoulli modal frequency denominator (`9`) from `prism_kernel`.
#[no_mangle]
pub extern "C" fn verdant_wasm_chime_mode_denom() -> u32 {
    prism_kernel::eulerBernoulliModeDenom() as u32
}

/// Evaluates Sabine reverberation `RT60` in milliseconds via `prism_kernel::sabineRt60Millis`.
#[no_mangle]
pub extern "C" fn verdant_wasm_sabine_rt60_millis(volume_m3: u32, absorption_milli_m2: u32) -> u32 {
    prism_kernel::sabineRt60Millis(u64::from(volume_m3), u64::from(absorption_milli_m2))
        .unwrap_or(0) as u32
}

/// Evaluates Woodworth spherical-head stereo pan `[-1, 1]` and caches gain + ITD.
#[allow(clippy::too_many_arguments)]
#[no_mangle]
pub extern "C" fn verdant_wasm_binaural_pan(
    src_x: f64,
    src_y: f64,
    src_z: f64,
    eye_x: f64,
    eye_y: f64,
    eye_z: f64,
    fwd_x: f64,
    fwd_y: f64,
    fwd_z: f64,
) -> f64 {
    let env = AcousticEnvironment::travertine_rotunda(343.0);
    let cue = evaluate_binaural_cue(
        &SpatialAudioSource {
            label: "SanctuaryEmitter".to_string(),
            position: Vec3::new(src_x, src_y, src_z),
            fundamental_hz: 220.0,
            reference_gain: 1.0,
        },
        &ListenerPose {
            position: Vec3::new(eye_x, eye_y, eye_z),
            forward: Vec3::new(fwd_x, fwd_y, fwd_z),
            up: Vec3::UP,
        },
        &env,
    );
    if let Ok(mut guard) = global_bridge().lock() {
        if let Some(bridge) = guard.as_mut() {
            bridge.last_binaural_gain = cue.direct_gain;
            bridge.last_binaural_itd_micros = cue.itd_seconds * 1_000_000.0;
        }
    }
    cue.azimuth_sin
}

/// Returns direct distance-attenuated gain from the most recent `verdant_wasm_binaural_pan` call.
#[no_mangle]
pub extern "C" fn verdant_wasm_binaural_gain() -> f64 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| Some(g.as_ref()?.last_binaural_gain))
        .unwrap_or(0.0)
}

/// Returns Woodworth ITD in microseconds from the most recent `verdant_wasm_binaural_pan` call.
#[no_mangle]
pub extern "C" fn verdant_wasm_binaural_itd_micros() -> f64 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| Some(g.as_ref()?.last_binaural_itd_micros))
        .unwrap_or(0.0)
}

/// Returns the far split distance in meters for cascade `cascade_idx` (`0..4`).
#[no_mangle]
pub extern "C" fn verdant_wasm_csm_split_meters(
    near_meters: f64,
    far_meters: f64,
    cascade_idx: u32,
) -> f64 {
    compute_csm_splits(near_meters, far_meters)
        .ok()
        .and_then(|s| s.get(cascade_idx as usize).map(|c| c.far_meters))
        .unwrap_or(far_meters)
}

/// Clears the safe host<->guest byte staging buffer.
#[no_mangle]
pub extern "C" fn verdant_wasm_io_clear() {
    if let Ok(mut guard) = global_bridge().lock() {
        if let Some(bridge) = guard.as_mut() {
            bridge.io_buffer.clear();
        }
    }
}

/// Appends one byte (`0..=255`) to the safe host<->guest staging buffer.
#[no_mangle]
pub extern "C" fn verdant_wasm_io_push_byte(byte: u32) {
    if let Ok(mut guard) = global_bridge().lock() {
        if let Some(bridge) = guard.as_mut() {
            let max_bytes = prism_kernel::snapshotUrlMaxBytes().unwrap_or(16_384) as usize;
            if bridge.io_buffer.len() < max_bytes {
                bridge.io_buffer.push((byte & 0xFF) as u8);
            }
        }
    }
}

/// Current byte length of the safe host<->guest staging buffer.
#[no_mangle]
pub extern "C" fn verdant_wasm_io_len() -> u32 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| Some(g.as_ref()?.io_buffer.len() as u32))
        .unwrap_or(0)
}

/// Reads one byte at `index` from the safe host<->guest staging buffer.
#[no_mangle]
pub extern "C" fn verdant_wasm_io_get_byte(index: u32) -> u32 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| {
            g.as_ref()?
                .io_buffer
                .get(index as usize)
                .copied()
                .map(u32::from)
        })
        .unwrap_or(0)
}

/// Encodes an authenticated `VDNT` + SHA-256 `WorldSnapshot` into the staging buffer and returns length.
#[no_mangle]
pub extern "C" fn verdant_wasm_snapshot_encode() -> u32 {
    let Ok(mut guard) = global_bridge().lock() else {
        return 0;
    };
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };
    match bridge.encode_snapshot() {
        Ok(bytes) => {
            let len = bytes.len() as u32;
            bridge.io_buffer = bytes;
            len
        }
        Err(_) => 0,
    }
}

/// Verifies and restores the `VDNT` + SHA-256 `WorldSnapshot` currently in the staging buffer (`1` = ok, `0` = refused).
#[no_mangle]
pub extern "C" fn verdant_wasm_snapshot_decode_and_restore() -> u32 {
    let Ok(mut guard) = global_bridge().lock() else {
        return 0;
    };
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };
    let wire = bridge.io_buffer.clone();
    u32::from(bridge.decode_and_restore_snapshot(&wire).is_ok())
}

/// Active cosmic expansion epoch rank (`0..=3`) after initialization or snapshot restore.
#[no_mangle]
pub extern "C" fn verdant_wasm_active_epoch_rank() -> u32 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| Some(g.as_ref()?.epoch_rank))
        .unwrap_or(3)
}

/// Active deterministic world seed (`u64`) after initialization or snapshot restore.
#[no_mangle]
pub extern "C" fn verdant_wasm_active_seed() -> u64 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| Some(g.as_ref()?.world.seed))
        .unwrap_or(7)
}

/// Number of synthesized Builder architectural blueprints in the active epoch (`0` or `7`).
#[no_mangle]
pub extern "C" fn verdant_wasm_blueprint_count() -> u32 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| Some(g.as_ref()?.world.blueprints.len() as u32))
        .unwrap_or(0)
}

/// World-space X coordinate in meters of Builder blueprint `index`.
#[no_mangle]
pub extern "C" fn verdant_wasm_blueprint_x(index: u32) -> f64 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| {
            Some(
                g.as_ref()?
                    .world
                    .blueprints
                    .get(index as usize)?
                    .world_position
                    .x,
            )
        })
        .unwrap_or(0.0)
}

/// World-space Y coordinate in meters of Builder blueprint `index`.
#[no_mangle]
pub extern "C" fn verdant_wasm_blueprint_y(index: u32) -> f64 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| {
            Some(
                g.as_ref()?
                    .world
                    .blueprints
                    .get(index as usize)?
                    .world_position
                    .y,
            )
        })
        .unwrap_or(0.0)
}

/// World-space Z coordinate in meters of Builder blueprint `index`.
#[no_mangle]
pub extern "C" fn verdant_wasm_blueprint_z(index: u32) -> f64 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| {
            Some(
                g.as_ref()?
                    .world
                    .blueprints
                    .get(index as usize)?
                    .world_position
                    .z,
            )
        })
        .unwrap_or(0.0)
}

/// Compressive and overturning safety factor (`>= 2.0`) of Builder blueprint `index`.
#[no_mangle]
pub extern "C" fn verdant_wasm_blueprint_safety_factor(index: u32) -> f64 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| {
            Some(
                g.as_ref()?
                    .world
                    .blueprints
                    .get(index as usize)?
                    .safety_factor,
            )
        })
        .unwrap_or(0.0)
}

/// Fundamental modal resonance frequency in hertz of Builder blueprint `index`.
#[no_mangle]
pub extern "C" fn verdant_wasm_blueprint_modal_hz(index: u32) -> f64 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| {
            Some(
                g.as_ref()?
                    .world
                    .blueprints
                    .get(index as usize)?
                    .modal_frequency_hz,
            )
        })
        .unwrap_or(0.0)
}

/// Returns worker command ring capacity (`64`) from `prism_kernel`.
#[no_mangle]
pub extern "C" fn verdant_wasm_worker_ring_capacity() -> u32 {
    prism_kernel::workerCommandRingCapacity().unwrap_or(64) as u32
}

/// Returns maximum URL-shareable `VDNT` snapshot byte budget (`16384`) from `prism_kernel`.
#[no_mangle]
pub extern "C" fn verdant_wasm_snapshot_max_bytes() -> u32 {
    prism_kernel::snapshotUrlMaxBytes().unwrap_or(16_384) as u32
}
