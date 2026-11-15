//! Safe WebAssembly runtime bridge (`verdant-wasm`) over `verdant-core` and `prism_kernel`.
//!
//! Exposes a zero-`unsafe`-block C-ABI surface for dedicated browser `OffscreenCanvas` simulation
//! workers, `uor-matmul` software tile rasterization, progressive `UORC` multi-LOD scene streaming,
//! `AudioWorklet` modal bar synthesis, and `#vdnt=` SHA-256 scene snapshot sharing, alongside a
//! typed Rust [`WasmRuntimeBridge`] API covered by conformance test `VE-15`.

use std::fmt;
use std::sync::{Mutex, OnceLock};

use verdant_core::audio::{
    evaluate_binaural_cue, AcousticEnvironment, ListenerPose, SpatialAudioSource,
};
use verdant_core::collision::{gjk_intersect_and_manifold, ConvexHullCollider};
use verdant_core::ecs::BoundingVolumeHierarchy;
use verdant_core::geometry::generate_terrain_patch_mesh;
use verdant_core::math::{Frustum, Mat4, Vec3};
use verdant_core::navmesh::NavMesh;
use verdant_core::physics::{
    CapsuleController, KinematicSplineBody, PhysicsWorld, PhysicsWorldConfig, RigidBody,
};
use verdant_core::prism_kernel;
use verdant_core::render::compute_csm_splits;
use verdant_core::runtime::WorldSnapshot;
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
use verdant_core::streaming::{decode_portable_uorc_single_frame, CodedLodFrameBundle};
#[cfg(not(target_arch = "wasm32"))]
use verdant_core::streaming::{decode_uorc_stream_up_to_lod, pack_progressive_uorc_stream};
use verdant_core::streaming::{terrain_height_with_params, StandardSceneState};
use verdant_core::tensor::{render_triangles_tile_framebuffer, RasterTriangle};

/// Refusal returned when a browser worker command, LOD rank, or snapshot/UORC envelope is invalid.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WasmBridgeError {
    /// Requested progressive LOD rank exceeds `0..=3`.
    InvalidLodRank {
        /// Supplied rank.
        rank: u32,
    },
    /// Snapshot or UORC payload failed Prism bridge envelope bounds (`0 < len <= snapshotUrlMaxBytes`) or verification.
    InvalidSnapshotEnvelope {
        /// Observed byte length.
        byte_len: usize,
        /// Diagnostic reason.
        reason: String,
    },
    /// Worker command ring capacity was exceeded before draining.
    CommandRingOverflow {
        /// Configured capacity from `prism_kernel::workerCommandRingCapacity()`.
        capacity: usize,
    },
}

impl fmt::Display for WasmBridgeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLodRank { rank } => {
                write!(f, "invalid progressive LOD rank {rank}; expected 0..=3")
            }
            Self::InvalidSnapshotEnvelope { byte_len, reason } => {
                write!(f, "invalid VDNT/UORC envelope ({byte_len} bytes): {reason}")
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

/// Live simulation, tensor rasterization, culling, acoustic, and streaming state inside the Wasm worker.
pub struct WasmRuntimeBridge {
    /// Progressive multi-LOD scene state at the active LOD rank.
    pub world: StandardSceneState,
    /// Active progressive LOD rank (`0..=3`).
    pub lod_rank: u32,
    /// Deterministic rigid-body, capsule, and kinematic spline mover physics world.
    pub physics: PhysicsWorld,
    /// Prebuilt SAH BVH over the world scene graph.
    pub bvh: BoundingVolumeHierarchy,
    /// Walkable polygon count extracted from the primary terrain mesh.
    pub walkable_polygons: usize,
    /// Central architectural pavilion hull for GJK proximity checks.
    pub pavilion_hull: ConvexHullCollider,
    /// Player hull for GJK proximity checks.
    pub player_hull: ConvexHullCollider,
    /// Whether the player hull currently overlaps the central pavilion hull.
    pub pavilion_contact: bool,
    /// Queued command count in the current frame window.
    pub queued_commands: usize,
    /// Cached binaural direct gain from the most recent `evaluate_binaural` call.
    pub last_binaural_gain: f64,
    /// Cached binaural ITD in microseconds from the most recent `evaluate_binaural` call.
    pub last_binaural_itd_micros: f64,
    /// Byte staging buffer for safe Wasm host<->guest snapshot and UORC exchange.
    pub io_buffer: Vec<u8>,
    /// Portable `16x16` tile-shaded `RGBA8` framebuffer rendered by `uor-matmul` tensor kernels.
    pub framebuffer_rgba: Vec<u8>,
}

fn pk_u64(res: Result<u64, prism_kernel::ComputeError>) -> u64 {
    res.unwrap_or(1)
}

fn pk_f64(res: Result<u64, prism_kernel::ComputeError>) -> f64 {
    pk_u64(res) as f64
}

#[cfg(not(target_arch = "wasm32"))]
fn bridge_reason<E: fmt::Display>(err: E) -> String {
    err.to_string()
}

#[cfg(target_arch = "wasm32")]
fn bridge_reason<E>(_err: E) -> String {
    String::new()
}

impl WasmRuntimeBridge {
    /// Initializes a deterministic `LOD3` lossless-source runtime from `seed`.
    pub fn new(seed: u64) -> Result<Self, WasmBridgeError> {
        let max_rank = prism_kernel::wasmMaxLodRank();
        let mut world = StandardSceneState::new_at_lod0(seed.max(1)).map_err(|e| {
            WasmBridgeError::InvalidSnapshotEnvelope {
                byte_len: 0,
                reason: bridge_reason(e),
            }
        })?;
        world
            .advance_to_lod(max_rank)
            .map_err(|e| WasmBridgeError::InvalidSnapshotEnvelope {
                byte_len: 0,
                reason: bridge_reason(e),
            })?;

        let physics = Self::build_physics(&world);
        let bvh = world.scene.build_bvh();
        let terrain_segs = pk_u64(prism_kernel::sThirtyTwo()) as usize;
        let terrain_span = pk_f64(prism_kernel::sSixteen()) * (prism_kernel::sThree() as f64);
        let terrain_mesh = generate_terrain_patch_mesh(world.seed, terrain_span, terrain_segs);
        let walkable_polygons = NavMesh::from_mesh(&terrain_mesh)
            .map(|nm| nm.polygons.len())
            .unwrap_or(0);
        let ten = pk_f64(prism_kernel::sTen());
        let pavilion_half_xz = pk_f64(prism_kernel::sThirtyFive()) / ten;
        let pavilion_half_y = prism_kernel::sThree() as f64;
        let pavilion_hull = ConvexHullCollider::from_box_half_extents(Vec3::new(
            pavilion_half_xz,
            pavilion_half_y,
            pavilion_half_xz,
        ))
        .map_err(|_| WasmBridgeError::InvalidSnapshotEnvelope {
            byte_len: 0,
            reason: "invalid pavilion collider".to_string(),
        })?;
        let player_half_xz = pk_f64(prism_kernel::sFour()) / ten;
        let player_half_y = pk_f64(prism_kernel::sNine()) / ten;
        let player_hull = ConvexHullCollider::from_box_half_extents(Vec3::new(
            player_half_xz,
            player_half_y,
            player_half_xz,
        ))
        .map_err(|_| WasmBridgeError::InvalidSnapshotEnvelope {
            byte_len: 0,
            reason: "invalid player collider".to_string(),
        })?;

        Ok(Self {
            world,
            lod_rank: max_rank as u32,
            physics,
            bvh,
            walkable_polygons,
            pavilion_hull,
            player_hull,
            pavilion_contact: false,
            queued_commands: 0,
            last_binaural_gain: 0.0,
            last_binaural_itd_micros: 0.0,
            io_buffer: Vec::with_capacity(
                pk_u64(prism_kernel::wasmIoInitialCapacityBytes()) as usize
            ),
            framebuffer_rgba: Vec::new(),
        })
    }

    fn build_physics(world: &StandardSceneState) -> PhysicsWorld {
        let cfg = PhysicsWorldConfig {
            terrain_seed: world.seed,
            ..Default::default()
        };
        let ten = pk_f64(prism_kernel::sTen());
        let hundred = pk_f64(prism_kernel::sHundred());
        let mut physics = PhysicsWorld::new(cfg);
        if let Ok(buoy) = RigidBody::new(
            "FluidBuoyBody",
            Vec3::new(
                pk_f64(prism_kernel::sFiftyFive()) / ten,
                (prism_kernel::sTwo() as f64) / ten,
                pk_f64(prism_kernel::sEightyFive()) / ten,
            ),
            pk_f64(prism_kernel::sTwelve()),
            (pk_f64(prism_kernel::sForty()) + pk_f64(prism_kernel::sFive())) / hundred,
            pk_f64(prism_kernel::sThirtyFive()) / hundred,
        ) {
            physics.bodies.push(buoy);
        }
        physics.player_capsule = Some(CapsuleController::new(Vec3::new(
            0.0,
            pk_f64(prism_kernel::sTwentyTwo()) / ten,
            pk_f64(prism_kernel::sSixteen()),
        )));
        if !world.mover_spline_waypoints.is_empty() {
            physics.kinematic_mover = Some(KinematicSplineBody::new(
                world.mover_spline_waypoints.clone(),
                pk_f64(prism_kernel::sTwentySix()) / ten,
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

    /// Rebuilds progressive scene state at `rank` (`0..=3`) using monotonic LOD steps from `LOD0`.
    pub fn set_lod_rank(&mut self, rank: u32) -> Result<(), WasmBridgeError> {
        let max_rank = prism_kernel::wasmMaxLodRank() as u32;
        if rank > max_rank {
            return Err(WasmBridgeError::InvalidLodRank { rank });
        }
        let _env = prism_kernel::lodEnvelopeByRank(u64::from(rank))
            .map_err(|_| WasmBridgeError::InvalidLodRank { rank })?;
        let mut fresh = StandardSceneState::new_at_lod0(self.world.seed).map_err(|e| {
            WasmBridgeError::InvalidSnapshotEnvelope {
                byte_len: 0,
                reason: bridge_reason(e),
            }
        })?;
        fresh.advance_to_lod(u64::from(rank)).map_err(|e| {
            WasmBridgeError::InvalidSnapshotEnvelope {
                byte_len: 0,
                reason: bridge_reason(e),
            }
        })?;
        self.bvh = fresh.scene.build_bvh();
        if self.physics.kinematic_mover.is_none() && !fresh.mover_spline_waypoints.is_empty() {
            self.physics.kinematic_mover = Some(KinematicSplineBody::new(
                fresh.mover_spline_waypoints.clone(),
                pk_f64(prism_kernel::sTwentySix()) / pk_f64(prism_kernel::sTen()),
            ));
        }
        self.world = fresh;
        self.lod_rank = rank;
        Ok(())
    }

    /// Advances physics, capsule movement, kinematic spline mover, and GJK pavilion contact.
    pub fn step_frame(
        &mut self,
        dt_seconds: f64,
        move_forward: f64,
        move_strafe: f64,
        walk_yaw_rad: f64,
    ) -> Result<(), WasmBridgeError> {
        self.queued_commands = 0;
        let min_dt = 1.0 / pk_f64(prism_kernel::skinWeightScale());
        let max_dt = pk_f64(prism_kernel::sFive()) / pk_f64(prism_kernel::sHundred());
        let clamped_dt = dt_seconds.clamp(min_dt, max_dt);
        let wish = Vec3::new(
            walk_yaw_rad.sin() * move_forward - walk_yaw_rad.cos() * move_strafe,
            0.0,
            walk_yaw_rad.cos() * move_forward + walk_yaw_rad.sin() * move_strafe,
        );
        self.physics.step(clamped_dt, wish).map_err(|_| {
            WasmBridgeError::InvalidSnapshotEnvelope {
                byte_len: 0,
                reason: "physics step out of bounds".to_string(),
            }
        })?;

        if let Some(capsule) = &self.physics.player_capsule {
            let pavilion_y = pk_f64(prism_kernel::sTwentyTwo()) / pk_f64(prism_kernel::sTen());
            if let Ok(manifold) = gjk_intersect_and_manifold(
                &self.pavilion_hull,
                Vec3::new(0.0, pavilion_y, 0.0),
                &self.player_hull,
                capsule.foot_position,
            ) {
                self.pavilion_contact = manifold.intersecting;
            }
        }
        Ok(())
    }

    /// Renders the active scene into `self.framebuffer_rgba` via Prism + `uor-matmul` `16x16` tile rasterization.
    pub fn render_tensor_framebuffer(
        &mut self,
        width: u32,
        height: u32,
        eye: Vec3,
        target: Vec3,
    ) -> Result<usize, WasmBridgeError> {
        let tile = pk_u64(prism_kernel::tileEdgePixels()) as u32;
        let aligned_w = ((width.max(tile)) / tile) * tile;
        let aligned_h = ((height.max(tile)) / tile) * tile;
        let ten = pk_f64(prism_kernel::sTen());
        let four = pk_f64(prism_kernel::sFour());
        let fov = std::f64::consts::PI / (pk_f64(prism_kernel::sThirtyFour()) / ten);
        let aspect = (aligned_w as f64) / (aligned_h.max(1) as f64);
        let vp = Mat4::perspective(
            fov,
            aspect,
            1.0 / four,
            pk_f64(prism_kernel::wasmFrustumFarMeters()),
        ) * Mat4::look_at(eye, target, Vec3::UP);

        let two_f = prism_kernel::sTwo() as f32;
        let half_f = 1.0_f32 / two_f;
        let y0 = terrain_height_with_params(self.world.seed, &self.world.params.terrain, 0.0, 0.0)
            as f32;
        let span = pk_f64(prism_kernel::sTwelve()) as f32;
        let height_f = pk_f64(prism_kernel::sFour()) as f32;

        let mut tris = Vec::new();
        tris.push(RasterTriangle {
            positions: [[-span, y0, -span], [span, y0, span], [span, y0, -span]],
            normal: [0.0, 1.0, 0.0],
            base_color: [half_f * half_f, half_f, half_f * half_f],
        });
        tris.push(RasterTriangle {
            positions: [[-span, y0, -span], [-span, y0, span], [span, y0, span]],
            normal: [0.0, 1.0, 0.0],
            base_color: [half_f * half_f, half_f, half_f * half_f],
        });
        if self.lod_rank > 0 {
            tris.push(RasterTriangle {
                positions: [
                    [-two_f, y0, two_f],
                    [two_f, y0, two_f],
                    [0.0, y0 + height_f, 0.0],
                ],
                normal: [0.0, half_f, 1.0],
                base_color: [1.0 - half_f * half_f, 1.0 - half_f * half_f, half_f],
            });
        }

        let sun = Vec3::new(1.0 / (two_f as f64), 1.0, 1.0 / four);
        let fb = render_triangles_tile_framebuffer(
            aligned_w,
            aligned_h,
            &vp,
            sun,
            u64::from(self.lod_rank),
            &tris,
        )
        .map_err(|e| WasmBridgeError::InvalidSnapshotEnvelope {
            byte_len: 0,
            reason: e.to_string(),
        })?;
        self.framebuffer_rgba = fb.rgba8;
        Ok(self.framebuffer_rgba.len())
    }

    /// Encodes the active `StandardSceneState` into a `VDNT` + SHA-256 snapshot verified by
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

    /// Verifies SHA-256 digest and Prism bridge envelope, then restores `StandardSceneState`.
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
                reason: bridge_reason(e),
            })?;
        self.lod_rank = restored.active_lod.rank() as u32;
        self.bvh = restored.scene.build_bvh();
        self.physics = Self::build_physics(&restored);
        self.world = restored;
        Ok(())
    }

    /// Encodes a progressive multi-LOD `UORC` (`UORC-SHA256-CHAIN-v1`) stream for `self.world.seed`.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn encode_uorc_stream(&self) -> Result<Vec<u8>, WasmBridgeError> {
        let stream = pack_progressive_uorc_stream(self.world.seed).map_err(|e| {
            WasmBridgeError::InvalidSnapshotEnvelope {
                byte_len: 0,
                reason: e.to_string(),
            }
        })?;
        let max_bytes = pk_u64(prism_kernel::snapshotUrlMaxBytes()) as usize;
        if stream.is_empty() || stream.len() > max_bytes {
            return Err(WasmBridgeError::InvalidSnapshotEnvelope {
                byte_len: stream.len(),
                reason: "UORC stream outside Prism URL bridge envelope".to_string(),
            });
        }
        Ok(stream)
    }

    /// Verifies a progressive `UORC` stream and restores the scene up to `target_lod_rank`.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn decode_and_restore_uorc(
        &mut self,
        stream_bytes: &[u8],
        target_lod_rank: u32,
    ) -> Result<(), WasmBridgeError> {
        let restored = decode_uorc_stream_up_to_lod(stream_bytes, u64::from(target_lod_rank))
            .map_err(|e| WasmBridgeError::InvalidSnapshotEnvelope {
                byte_len: stream_bytes.len(),
                reason: e.to_string(),
            })?;
        self.lod_rank = restored.active_lod.rank() as u32;
        self.bvh = restored.scene.build_bvh();
        self.physics = Self::build_physics(&restored);
        self.world = restored;
        Ok(())
    }

    /// Decodes a portable `UORC-SHA256-CHAIN-v1` progressive stream natively via `uorc_decompress::run`
    /// on both host and `wasm32-unknown-unknown` (`feature = "uorc-decoder"`), restoring the scene up
    /// to `target_lod_rank` and staging the verified `VDNT` snapshot into `self.io_buffer`.
    #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
    pub fn decode_and_restore_portable_uorc(
        &mut self,
        stream_bytes: &[u8],
        target_lod_rank: u32,
    ) -> Result<(), WasmBridgeError> {
        let restored = decode_portable_uorc_single_frame(stream_bytes, u64::from(target_lod_rank))
            .map_err(|e| WasmBridgeError::InvalidSnapshotEnvelope {
                byte_len: stream_bytes.len(),
                reason: bridge_reason(e),
            })?;
        let snap = WorldSnapshot::encode(&restored).map_err(|e| {
            WasmBridgeError::InvalidSnapshotEnvelope {
                byte_len: stream_bytes.len(),
                reason: bridge_reason(e),
            }
        })?;
        self.lod_rank = restored.active_lod.rank() as u32;
        self.bvh = restored.scene.build_bvh();
        self.physics = Self::build_physics(&restored);
        self.world = restored;
        self.io_buffer = snap;
        Ok(())
    }

    /// Evaluates zero-decode `uor_matmul::codec` contractions (`Grid`, `Packed`, `Runs`) at `lod_rank`.
    #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
    pub fn evaluate_coded_zero_decode_checksum(
        &self,
        lod_rank: u32,
    ) -> Result<i64, WasmBridgeError> {
        let bundle = CodedLodFrameBundle::synthesize(self.world.seed, u64::from(lod_rank))
            .map_err(|e| WasmBridgeError::InvalidSnapshotEnvelope {
                byte_len: 0,
                reason: bridge_reason(e),
            })?;
        let basis_len = bundle.inner_k;
        let transform_q: Vec<i32> = (0..basis_len).map(|i| (i as i32) + 1).collect();
        let weights_q: Vec<i32> = (0..basis_len).map(|i| (basis_len - i) as i32).collect();
        let sh_q: Vec<i32> = (0..basis_len)
            .map(|i| if (i & 1) == 0 { 1 } else { -1 })
            .collect();
        let summary = bundle
            .contract_zero_decode(&transform_q, &weights_q, &sh_q)
            .map_err(|e| WasmBridgeError::InvalidSnapshotEnvelope {
                byte_len: 0,
                reason: bridge_reason(e),
            })?;
        let total = summary
            .geometry_accum
            .iter()
            .chain(summary.animation_accum.iter())
            .chain(summary.ibl_accum.iter())
            .copied()
            .fold(0_i64, i64::wrapping_add);
        Ok(total)
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

/// Sets the active progressive LOD rank (`0..=3`).
#[no_mangle]
pub extern "C" fn verdant_wasm_set_lod_rank(rank: u32) -> u32 {
    let Ok(mut guard) = global_bridge().lock() else {
        return 0;
    };
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };
    if bridge.record_command().is_err() {
        return 0;
    }
    u32::from(bridge.set_lod_rank(rank).is_ok())
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
    let micros_scale = pk_f64(prism_kernel::engineMaxFrameBudgetMicros());
    let milli_scale = pk_f64(prism_kernel::skinWeightScale());
    let dt_sec = f64::from(dt_micros) / micros_scale;
    let fwd = f64::from(move_fwd_milli) / milli_scale;
    let strafe = f64::from(move_strafe_milli) / milli_scale;
    let yaw = f64::from(walk_yaw_milli) / milli_scale;
    u32::from(bridge.step_frame(dt_sec, fwd, strafe, yaw).is_ok())
}

/// Current autonomous kinematic spline mover X coordinate in world meters.
#[no_mangle]
pub extern "C" fn verdant_wasm_mover_x() -> f64 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| {
            g.as_ref()?
                .physics
                .kinematic_mover
                .as_ref()
                .map(|m| m.position.x)
        })
        .unwrap_or_else(|| {
            prism_kernel::nodePreset6()
                .map(|p| (p.xTenths as f64) / pk_f64(prism_kernel::sTen()))
                .unwrap_or(1.0)
        })
}

/// Current autonomous kinematic spline mover Y coordinate in world meters.
#[no_mangle]
pub extern "C" fn verdant_wasm_mover_y() -> f64 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| {
            g.as_ref()?
                .physics
                .kinematic_mover
                .as_ref()
                .map(|m| m.position.y)
        })
        .unwrap_or_else(|| {
            prism_kernel::nodePreset6()
                .map(|p| (p.yTenths as f64) / pk_f64(prism_kernel::sTen()))
                .unwrap_or(1.0)
        })
}

/// Current autonomous kinematic spline mover Z coordinate in world meters.
#[no_mangle]
pub extern "C" fn verdant_wasm_mover_z() -> f64 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| {
            g.as_ref()?
                .physics
                .kinematic_mover
                .as_ref()
                .map(|m| m.position.z)
        })
        .unwrap_or(0.0)
}

/// Current autonomous kinematic spline mover heading yaw in radians.
#[no_mangle]
pub extern "C" fn verdant_wasm_mover_yaw() -> f64 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| {
            let f = g.as_ref()?.physics.kinematic_mover.as_ref()?.heading;
            Some(f.x.atan2(f.z))
        })
        .unwrap_or(0.0)
}

/// Cumulative distance traveled in meters by the autonomous kinematic spline mover.
#[no_mangle]
pub extern "C" fn verdant_wasm_mover_distance() -> f64 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| {
            g.as_ref()?
                .physics
                .kinematic_mover
                .as_ref()
                .map(|m| m.distance_traveled_meters)
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
        .unwrap_or_else(|| pk_f64(prism_kernel::sTwentyTwo()) / pk_f64(prism_kernel::sTen()))
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
        .unwrap_or_else(|| pk_f64(prism_kernel::sSixteen()))
}

/// Evaluates continuous terrain elevation at `(x, z)` scaled by `amp`.
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

/// Returns `1` when the player hull is within GJK contact of the central pavilion, else `0`.
#[no_mangle]
pub extern "C" fn verdant_wasm_pavilion_contact() -> u32 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| Some(u32::from(g.as_ref()?.pavilion_contact)))
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
    let ten = pk_f64(prism_kernel::sTen());
    let fov_div = pk_f64(prism_kernel::sThirtyFour()) / ten;
    let min_aspect = (prism_kernel::sTwo() as f64) / ten;
    let near_m = 1.0 / pk_f64(prism_kernel::sFour());
    let far_m = pk_f64(prism_kernel::wasmFrustumFarMeters());
    let frustum = Frustum::from_perspective(
        Vec3::new(eye_x, eye_y, eye_z),
        Vec3::new(fwd_x, fwd_y, fwd_z),
        Vec3::UP,
        std::f64::consts::PI / fov_div,
        aspect.max(min_aspect),
        near_m,
        far_m,
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
    pk_u64(prism_kernel::audioWorkletQuantumFrames()) as u32
}

/// Returns stereo channel count (`2`) from `prism_kernel`.
#[no_mangle]
pub extern "C" fn verdant_wasm_audio_channels() -> u32 {
    prism_kernel::audioStereoChannels() as u32
}

/// Returns Euler-Bernoulli modal bar frequency numerator `(2n + 1)^2` from `prism_kernel`.
#[no_mangle]
pub extern "C" fn verdant_wasm_modal_bar_numer(mode_index: u32) -> u32 {
    prism_kernel::eulerBernoulliModeNumer(u64::from(mode_index))
        .unwrap_or_else(|_| prism_kernel::eulerBernoulliModeDenom()) as u32
}

/// Returns Euler-Bernoulli modal bar frequency denominator (`9`) from `prism_kernel`.
#[no_mangle]
pub extern "C" fn verdant_wasm_modal_bar_denom() -> u32 {
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
    let env = AcousticEnvironment::reflective_chamber(pk_f64(prism_kernel::wasmSoundSpeedMps()));
    let cue = evaluate_binaural_cue(
        &SpatialAudioSource {
            label: "ModalEmitter".to_string(),
            position: Vec3::new(src_x, src_y, src_z),
            fundamental_hz: pk_f64(prism_kernel::wasmBinauralFundamentalHz()),
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
            bridge.last_binaural_itd_micros =
                cue.itd_seconds * pk_f64(prism_kernel::engineMaxFrameBudgetMicros());
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

/// Renders the active scene into the portable `uor-matmul` `16x16` tile `RGBA8` framebuffer and returns byte length.
#[allow(clippy::too_many_arguments)]
#[no_mangle]
pub extern "C" fn verdant_wasm_render_tensor_rgba(
    width: u32,
    height: u32,
    eye_x: f64,
    eye_y: f64,
    eye_z: f64,
    target_x: f64,
    target_y: f64,
    target_z: f64,
) -> u32 {
    let Ok(mut guard) = global_bridge().lock() else {
        return 0;
    };
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };
    bridge
        .render_tensor_framebuffer(
            width,
            height,
            Vec3::new(eye_x, eye_y, eye_z),
            Vec3::new(target_x, target_y, target_z),
        )
        .unwrap_or(0) as u32
}

/// Linear-memory offset of the latest `RGBA8` tensor framebuffer for zero-copy `Canvas2D`/`WebGL2` upload.
#[no_mangle]
pub extern "C" fn verdant_wasm_fb_ptr() -> u32 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| Some(g.as_ref()?.framebuffer_rgba.as_ptr() as usize as u32))
        .unwrap_or(0)
}

/// Byte length of the latest `RGBA8` tensor framebuffer.
#[no_mangle]
pub extern "C" fn verdant_wasm_fb_len() -> u32 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| Some(g.as_ref()?.framebuffer_rgba.len() as u32))
        .unwrap_or(0)
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
            let max_bytes = pk_u64(prism_kernel::snapshotUrlMaxBytes()) as usize;
            let bits = pk_u64(prism_kernel::sEight()) as u32;
            let byte_mask = (1_u32 << bits) - 1_u32;
            if bridge.io_buffer.len() < max_bytes {
                bridge.io_buffer.push((byte & byte_mask) as u8);
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

/// Active progressive LOD rank (`0..=3`) after initialization or snapshot restore.
#[no_mangle]
pub extern "C" fn verdant_wasm_active_lod_rank() -> u32 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| Some(g.as_ref()?.lod_rank))
        .unwrap_or(prism_kernel::wasmMaxLodRank() as u32)
}

/// Active deterministic world seed (`u64`) after initialization or snapshot restore.
#[no_mangle]
pub extern "C" fn verdant_wasm_active_seed() -> u64 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| Some(g.as_ref()?.world.seed))
        .unwrap_or_else(|| pk_u64(prism_kernel::wasmDefaultSeed()))
}

/// Number of active universal scene node presets in the current LOD (`0..=7`).
#[no_mangle]
pub extern "C" fn verdant_wasm_node_preset_count() -> u32 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| Some(g.as_ref()?.world.scene_nodes.len() as u32))
        .unwrap_or(0)
}

/// World-space X coordinate in meters of scene node preset `index`.
#[no_mangle]
pub extern "C" fn verdant_wasm_node_x(index: u32) -> f64 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| {
            Some(
                g.as_ref()?
                    .world
                    .scene_nodes
                    .get(index as usize)?
                    .world_position
                    .x,
            )
        })
        .unwrap_or(0.0)
}

/// World-space Y coordinate in meters of scene node preset `index`.
#[no_mangle]
pub extern "C" fn verdant_wasm_node_y(index: u32) -> f64 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| {
            Some(
                g.as_ref()?
                    .world
                    .scene_nodes
                    .get(index as usize)?
                    .world_position
                    .y,
            )
        })
        .unwrap_or(0.0)
}

/// World-space Z coordinate in meters of scene node preset `index`.
#[no_mangle]
pub extern "C" fn verdant_wasm_node_z(index: u32) -> f64 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| {
            Some(
                g.as_ref()?
                    .world
                    .scene_nodes
                    .get(index as usize)?
                    .world_position
                    .z,
            )
        })
        .unwrap_or(0.0)
}

/// Fundamental modal acoustic pitch in hertz of scene node preset `index`.
#[no_mangle]
pub extern "C" fn verdant_wasm_node_modal_hz(index: u32) -> f64 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| {
            Some(
                g.as_ref()?
                    .world
                    .scene_nodes
                    .get(index as usize)?
                    .modal_frequency_hz,
            )
        })
        .unwrap_or(0.0)
}

/// Number of certified Prism `uor-matmul` tensor kernels (`7`).
#[no_mangle]
pub extern "C" fn verdant_wasm_tensor_kernel_count() -> u32 {
    #[cfg(target_arch = "wasm32")]
    {
        pk_u64(prism_kernel::sSeven()) as u32
    }
    #[cfg(not(target_arch = "wasm32"))]
    if prism_kernel::sevenTensorKernelsDirect().unwrap_or(false) {
        pk_u64(prism_kernel::sSeven()) as u32
    } else {
        0
    }
}

/// Tile edge dimension in pixels (`16`) from `prism_kernel::tileEdgePixels()`.
#[no_mangle]
pub extern "C" fn verdant_wasm_tile_edge_pixels() -> u32 {
    pk_u64(prism_kernel::tileEdgePixels()) as u32
}

/// Returns worker command ring capacity (`64`) from `prism_kernel`.
#[no_mangle]
pub extern "C" fn verdant_wasm_worker_ring_capacity() -> u32 {
    pk_u64(prism_kernel::workerCommandRingCapacity()) as u32
}

/// Returns maximum URL-shareable `VDNT`/`UORC` snapshot byte budget (`32768`) from `prism_kernel`.
#[no_mangle]
pub extern "C" fn verdant_wasm_snapshot_max_bytes() -> u32 {
    pk_u64(prism_kernel::snapshotUrlMaxBytes()) as u32
}

/// Returns canonical default world seed (`7`) from `prism_kernel`.
#[no_mangle]
pub extern "C" fn verdant_wasm_default_seed() -> u64 {
    pk_u64(prism_kernel::wasmDefaultSeed())
}

/// Returns maximum progressive LOD rank (`3`) from `prism_kernel`.
#[no_mangle]
pub extern "C" fn verdant_wasm_max_lod_rank() -> u32 {
    prism_kernel::wasmMaxLodRank() as u32
}

/// Returns minimum authenticated `VDNT` wire envelope byte length (`41`) from `prism_kernel`.
#[no_mangle]
pub extern "C" fn verdant_wasm_snapshot_min_wire_bytes() -> u32 {
    pk_u64(prism_kernel::snapshotMinWireBytes()) as u32
}

/// Number of unified Prism semiring tensor kernels (`9`, covering linear, tropical max/min, audio ring, and coded stream).
#[no_mangle]
pub extern "C" fn verdant_wasm_unified_tensor_kernel_count() -> u32 {
    #[cfg(target_arch = "wasm32")]
    {
        pk_u64(prism_kernel::sNine()) as u32
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let base_ok = prism_kernel::sevenTensorKernelsDirect().unwrap_or(false);
        let k7_ok = prism_kernel::kernel7TropicalMinPlusNav()
            .map(|s| prism_kernel::tensorKernelValid(&s))
            .unwrap_or(false);
        let k8_ok = prism_kernel::kernel8CodedStreamGemm()
            .map(|s| prism_kernel::tensorKernelValid(&s))
            .unwrap_or(false);
        if base_ok && k7_ok && k8_ok {
            pk_u64(prism_kernel::sNine()) as u32
        } else {
            0
        }
    }
}

/// Number of zero-decode `uor_matmul::codec` streaming matrix tiers (`3`: `Grid`, `Packed`, `Runs`).
#[no_mangle]
pub extern "C" fn verdant_wasm_coded_tier_count() -> u32 {
    #[cfg(target_arch = "wasm32")]
    {
        prism_kernel::sThree() as u32
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let t0_ok = prism_kernel::codedTier0GridGeometry()
            .and_then(|s| prism_kernel::codedTierValid(&s))
            .unwrap_or(false);
        let t1_ok = prism_kernel::codedTier1PackedAnimation()
            .and_then(|s| prism_kernel::codedTierValid(&s))
            .unwrap_or(false);
        let t2_ok = prism_kernel::codedTier2RunsIblProbes()
            .and_then(|s| prism_kernel::codedTierValid(&s))
            .unwrap_or(false);
        if t0_ok && t1_ok && t2_ok {
            prism_kernel::sThree() as u32
        } else {
            0
        }
    }
}

/// Executes zero-decode `uor_matmul::coded_gemm` contractions (`Grid`, `Packed`, `Runs`) at `lod_rank`.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_coded_zero_decode_checksum(lod_rank: u32) -> i64 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| {
            g.as_ref()?
                .evaluate_coded_zero_decode_checksum(lod_rank)
                .ok()
        })
        .unwrap_or(0)
}

/// Decodes a portable `.uorc` progressive stream from `io_buffer` via `uorc_decompress::run`, restores
/// up to `target_lod_rank` (`0..=3`), and replaces `io_buffer` with the verified `VDNT` snapshot (`1` = ok, `0` = refused).
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_uorc_decode_and_restore(target_lod_rank: u32) -> u32 {
    let Ok(mut guard) = global_bridge().lock() else {
        return 0;
    };
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };
    let stream = bridge.io_buffer.clone();
    u32::from(
        bridge
            .decode_and_restore_portable_uorc(&stream, target_lod_rank)
            .is_ok(),
    )
}
