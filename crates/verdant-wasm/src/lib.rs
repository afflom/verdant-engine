//! Safe WebAssembly runtime bridge (`verdant-wasm`) over `verdant-core` and `prism_kernel`.
//!
//! Exposes a zero-`unsafe`-block C-ABI surface for dedicated browser `OffscreenCanvas` simulation
//! workers, zero-copy world-space mesh buffers, `uor-matmul` `16x16` software tile rasterization,
//! progressive `UORC` multi-LOD scene streaming, formal 2D/3D UI compliance, second-order
//! cybernetic LIF/SLM direction, `afflom/stitch` 2-of-3 parity custody, `AudioWorklet` modal bar
//! synthesis, and `#vdnt=` SHA-256 scene snapshot sharing, alongside a typed Rust
//! [`WasmRuntimeBridge`] API covered by conformance test `VE-15`.

use std::fmt;
use std::sync::{Mutex, OnceLock};

use verdant_core::audio::{
    evaluate_binaural_cue, AcousticEnvironment, ListenerPose, SpatialAudioSource,
};
use verdant_core::collision::{gjk_intersect_and_manifold, ConvexHullCollider};
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
use verdant_core::cybernetics::{CodedSlmDirector, ContainerGrantMatrix, ContainerOrgan};
use verdant_core::cybernetics::{
    CyberneticObserverLoop, PlayerObserverSample, WorldAttentionDirective,
};
use verdant_core::ecs::BoundingVolumeHierarchy;
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
use verdant_core::ecs::SceneWorld;
use verdant_core::geometry::generate_terrain_patch_mesh;
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
use verdant_core::geometry::{canonical_scene_mesh_library, ProceduralMesh};
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
use verdant_core::gltf_io::{export_scene_to_glb, import_glb_to_scene};
use verdant_core::math::{Frustum, Mat4, Vec3};
use verdant_core::navmesh::NavMesh;
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
use verdant_core::network::{
    recover_bytes_from_stitch_pair, shard_bytes_to_stitch_fragments, StitchFragment,
};
use verdant_core::physics::{
    CapsuleController, JointKind, KinematicSplineBody, PhysicsWorld, PhysicsWorldConfig, RigidBody,
    RigidJointConstraint,
};
use verdant_core::prism_kernel;
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
use verdant_core::render::{
    canonical_pbr_material_library, pack_world_meshes_to_buffers, PbrMaterial,
};
use verdant_core::render::{compute_csm_splits, PackedSceneRenderBuffers};
use verdant_core::runtime::WorldSnapshot;
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
use verdant_core::streaming::{decode_portable_uorc_single_frame, CodedLodFrameBundle};
#[cfg(not(target_arch = "wasm32"))]
use verdant_core::streaming::{decode_uorc_stream_up_to_lod, pack_progressive_uorc_stream};
use verdant_core::streaming::{terrain_height_with_params, StandardSceneState};
use verdant_core::tensor::render_triangles_tile_framebuffer;
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
use verdant_core::ui::UiTree;

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

/// Live simulation, tensor rasterization, culling, UI, cybernetic, and streaming state inside the Wasm worker.
pub struct WasmRuntimeBridge {
    /// Progressive multi-LOD scene state at the active LOD rank.
    pub world: StandardSceneState,
    /// Active progressive LOD rank (`0..=3`).
    pub lod_rank: u32,
    /// Deterministic rigid-body, XPBD joint, vehicle, capsule, and spline mover physics world.
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
    /// Byte staging buffer for safe Wasm host<->guest snapshot, `.glb`, and `.uorc` exchange.
    pub io_buffer: Vec<u8>,
    /// Auxiliary output buffer for non-destructive Stitch shard, recovery, and UI reports.
    pub aux_buffer: Vec<u8>,
    /// Portable `16x16` tile-shaded `RGBA8` framebuffer rendered by `uor-matmul` tensor kernels.
    pub framebuffer_rgba: Vec<u8>,
    /// Zero-copy world-space vertex, normal, color, index, and `16x16` tile triangle buffers.
    pub scene_buffers: PackedSceneRenderBuffers,
    /// Second-order neuromorphic LIF/SSM observer recurrence coupling player and world attention.
    pub observer_loop: CyberneticObserverLoop,
    /// Most recent `WorldAttentionDirective` emitted by `observer_loop`.
    pub last_directive: Option<WorldAttentionDirective>,
    /// Embedded zero-decode `uor_matmul::codec` language and intent router.
    #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
    pub slm_director: CodedSlmDirector,
    /// Denial-wins capability grant matrix over browser container APIs (`FK-07`).
    #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
    pub grants: ContainerGrantMatrix,
    /// Optional user-imported `.glb` scene overriding the procedural world meshes.
    #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
    pub custom_glb_scene: Option<(SceneWorld, Vec<ProceduralMesh>, Vec<PbrMaterial>)>,
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

        let slot_count = (pk_u64(prism_kernel::sSeven()) as usize).max(1);
        let observer_loop = CyberneticObserverLoop::new(slot_count).map_err(|e| {
            WasmBridgeError::InvalidSnapshotEnvelope {
                byte_len: 0,
                reason: bridge_reason(e),
            }
        })?;

        let mut bridge = Self {
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
            aux_buffer: Vec::new(),
            framebuffer_rgba: Vec::new(),
            scene_buffers: PackedSceneRenderBuffers::default(),
            observer_loop,
            last_directive: None,
            #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
            slm_director: CodedSlmDirector::new(),
            #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
            grants: ContainerGrantMatrix::default(),
            #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
            custom_glb_scene: None,
        };
        let _ = bridge.rebuild_scene_buffers(0.0);
        Ok(bridge)
    }

    fn build_physics(world: &StandardSceneState) -> PhysicsWorld {
        let cfg = PhysicsWorldConfig {
            terrain_seed: world.seed,
            ..Default::default()
        };
        let two = prism_kernel::sTwo() as f64;
        let three = prism_kernel::sThree() as f64;
        let four = pk_f64(prism_kernel::sFour());
        let five = pk_f64(prism_kernel::sFive());
        let nine = pk_f64(prism_kernel::sNine());
        let ten = pk_f64(prism_kernel::sTen());
        let twelve = pk_f64(prism_kernel::sTwelve());
        let hundred = pk_f64(prism_kernel::sHundred());
        let mut physics = PhysicsWorld::new(cfg);

        if let Ok(buoy) = RigidBody::new(
            "FluidBuoyBody",
            Vec3::new(
                pk_f64(prism_kernel::sFiftyFive()) / ten,
                two / ten,
                pk_f64(prism_kernel::sEightyFive()) / ten,
            ),
            twelve,
            (pk_f64(prism_kernel::sForty()) + five) / hundred,
            pk_f64(prism_kernel::sThirtyFive()) / hundred,
        ) {
            physics.bodies.push(buoy);
        }

        // Articulated XPBD hinge pendulum pair at ArticulatedRigBay (`nodePreset3`: `-6.5, y, 9.0`):
        let rig_x = -(pk_f64(prism_kernel::sSixty()) + five) / ten;
        let rig_z = nine;
        let rig_y = terrain_height_with_params(world.seed, &world.params.terrain, rig_x, rig_z);
        if let (Ok(anchor), Ok(bob)) = (
            RigidBody::new(
                "XpbdRigAnchor",
                Vec3::new(rig_x, rig_y + three, rig_z),
                hundred * hundred * ten,
                three / ten,
                two / ten,
            ),
            RigidBody::new(
                "XpbdRigPendulumBob",
                Vec3::new(rig_x + (twelve / ten), rig_y + two, rig_z),
                five,
                three / ten,
                two / ten,
            ),
        ) {
            let anchor_idx = physics.bodies.len();
            physics.bodies.push(anchor);
            let bob_idx = physics.bodies.len();
            physics.bodies.push(bob);
            physics.rigid_joints.push(RigidJointConstraint {
                label: "ArticulatedBayHinge".to_string(),
                body_a: anchor_idx,
                body_b: bob_idx,
                kind: JointKind::Hinge,
                axis_world: Vec3::new(0.0, 0.0, 1.0),
                target_distance_meters: (ten + four) / ten,
                compliance_permille: 1,
            });
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

    /// Rebuilds `self.scene_buffers` directly from Rust `SceneWorld` hierarchy contractions,
    /// `canonical_scene_mesh_library` (or imported `.glb`), live XPBD bodies, 4-wheel suspension
    /// vehicle, neuromorphic Observer Core gimbal, and diegetic `UiTree` bounds.
    pub fn rebuild_scene_buffers(&mut self, sim_time_sec: f64) -> Result<usize, WasmBridgeError> {
        #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
        if let Some((ref glb_scene, ref glb_meshes, ref glb_mats)) = self.custom_glb_scene {
            let packed =
                pack_world_meshes_to_buffers(glb_scene, glb_meshes, glb_mats).map_err(|e| {
                    WasmBridgeError::InvalidSnapshotEnvelope {
                        byte_len: 0,
                        reason: bridge_reason(e),
                    }
                })?;
            let tris = packed.triangle_count();
            self.scene_buffers = packed;
            return Ok(tris);
        }

        let two = prism_kernel::sTwo() as f64;
        let three = prism_kernel::sThree() as f64;
        let four = pk_f64(prism_kernel::sFour());
        let six = pk_f64(prism_kernel::sSix());
        let eight = pk_f64(prism_kernel::sEight());
        let ten = pk_f64(prism_kernel::sTen());
        let twelve = pk_f64(prism_kernel::sTwelve());
        let sixteen = pk_f64(prism_kernel::sSixteen());
        let half = 1.0 / two;
        let quarter = half * half;

        let i2_u32 = prism_kernel::sTwo() as u32;
        let grid_segs = if self.lod_rank == 0 {
            twelve as usize
        } else if self.lod_rank == 1 {
            (sixteen + two) as usize
        } else if self.lod_rank == i2_u32 {
            (sixteen + eight) as usize
        } else {
            pk_u64(prism_kernel::sThirtyTwo()) as usize
        };

        #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
        let mut packed = {
            let mut meshes = canonical_scene_mesh_library(self.world.seed);
            if !meshes.is_empty() {
                meshes[0] =
                    generate_terrain_patch_mesh(self.world.seed, sixteen * three, grid_segs.max(1));
            }
            let materials = canonical_pbr_material_library();
            pack_world_meshes_to_buffers(&self.world.scene, &meshes, &materials).map_err(|e| {
                WasmBridgeError::InvalidSnapshotEnvelope {
                    byte_len: 0,
                    reason: bridge_reason(e),
                }
            })?
        };

        #[cfg(all(target_arch = "wasm32", not(feature = "uorc-decoder")))]
        let mut packed = {
            let i2 = prism_kernel::sTwo() as usize;
            let terrain =
                generate_terrain_patch_mesh(self.world.seed, sixteen * three, grid_segs.max(1));
            let mut buf = PackedSceneRenderBuffers::default();
            for ((pos, nrm), col) in terrain
                .positions
                .iter()
                .zip(terrain.normals.iter())
                .zip(terrain.colors.iter())
            {
                buf.positions_xyz.extend_from_slice(pos);
                buf.normals_xyz.extend_from_slice(nrm);
                buf.colors_rgb.extend_from_slice(&[col[0], col[1], col[i2]]);
            }
            let third = 1.0_f32 / (prism_kernel::sThree() as f32);
            for tri in terrain
                .indices
                .chunks_exact(prism_kernel::sThree() as usize)
            {
                buf.indices.extend_from_slice(tri);
                let pa = (tri[0] as usize) * (prism_kernel::sThree() as usize);
                let pb = (tri[1] as usize) * (prism_kernel::sThree() as usize);
                let pc = (tri[i2] as usize) * (prism_kernel::sThree() as usize);
                buf.tile_triangles
                    .push(verdant_core::tensor::RasterTriangle {
                        positions: [
                            [
                                buf.positions_xyz[pa],
                                buf.positions_xyz[pa + 1],
                                buf.positions_xyz[pa + i2],
                            ],
                            [
                                buf.positions_xyz[pb],
                                buf.positions_xyz[pb + 1],
                                buf.positions_xyz[pb + i2],
                            ],
                            [
                                buf.positions_xyz[pc],
                                buf.positions_xyz[pc + 1],
                                buf.positions_xyz[pc + i2],
                            ],
                        ],
                        normal: [
                            (buf.normals_xyz[pa] + buf.normals_xyz[pb] + buf.normals_xyz[pc])
                                * third,
                            (buf.normals_xyz[pa + 1]
                                + buf.normals_xyz[pb + 1]
                                + buf.normals_xyz[pc + 1])
                                * third,
                            (buf.normals_xyz[pa + i2]
                                + buf.normals_xyz[pb + i2]
                                + buf.normals_xyz[pc + i2])
                                * third,
                        ],
                        base_color: [
                            (buf.colors_rgb[pa] + buf.colors_rgb[pb] + buf.colors_rgb[pc]) * third,
                            (buf.colors_rgb[pa + 1]
                                + buf.colors_rgb[pb + 1]
                                + buf.colors_rgb[pc + 1])
                                * third,
                            (buf.colors_rgb[pa + i2]
                                + buf.colors_rgb[pb + i2]
                                + buf.colors_rgb[pc + i2])
                                * third,
                        ],
                    });
            }
            // Fluid basin surface + SceneWorld entities (dielectrics, foliage, and node presets):
            let bx = self.world.params.terrain.basin_center_x;
            let bz = self.world.params.terrain.basin_center_z;
            let wy = self.world.params.environment.water_level_meters;
            let f_span = eight;
            buf.append_quad(
                Vec3::new(bx - f_span, wy, bz + f_span),
                Vec3::new(bx + f_span, wy, bz + f_span),
                Vec3::new(bx + f_span, wy, bz - f_span),
                Vec3::new(bx - f_span, wy, bz - f_span),
                [
                    (quarter * half) as f32,
                    (half) as f32,
                    (half + quarter * half) as f32,
                ],
            );
            for ent in self
                .world
                .scene
                .entities()
                .skip(prism_kernel::sThree() as usize)
            {
                let tr = ent.local_transform.translation;
                let raw_span = ent.local_bounds.max - ent.local_bounds.min;
                let ext = Vec3::new(raw_span.x * half, raw_span.y * half, raw_span.z * half);
                let tint = match ent.mesh_index.unwrap_or(0) {
                    x if x == i2 => [
                        quarter as f32,
                        (1.0 - quarter * half) as f32,
                        (1.0 - quarter * quarter) as f32,
                    ],
                    x if x == (prism_kernel::sThree() as usize) => [
                        quarter as f32,
                        (half + quarter * half) as f32,
                        (quarter + quarter * quarter) as f32,
                    ],
                    _ => [
                        (1.0 - quarter * half) as f32,
                        (1.0 - quarter * half) as f32,
                        (1.0 - quarter) as f32,
                    ],
                };
                buf.append_oriented_box(
                    Vec3::new(tr.x, tr.y + ext.y, tr.z),
                    Vec3::new(ext.x.min(1.0), ext.y.max(half), ext.z.min(1.0)),
                    tr.x * quarter,
                    0.0,
                    tint,
                );
            }
            buf
        };

        let y0 = terrain_height_with_params(self.world.seed, &self.world.params.terrain, 0.0, 0.0);
        let stone = [
            (1.0 - quarter * half) as f32,
            (1.0 - quarter * half) as f32,
            (1.0 - quarter) as f32,
        ];
        let gold = [
            (1.0 - quarter * half) as f32,
            (half + quarter * half) as f32,
            (quarter + quarter * half) as f32,
        ];
        let cyan = [
            quarter as f32,
            (1.0 - quarter * half) as f32,
            (1.0 - quarter * quarter) as f32,
        ];
        let slate = [
            quarter as f32,
            (quarter + quarter * quarter) as f32,
            (quarter + quarter * half) as f32,
        ];

        if self.lod_rank >= 1 {
            // Central Peristyle Pavilion plinth + ring beams + Neuromorphic Observer Core gimbal:
            packed.append_oriented_box(
                Vec3::new(0.0, y0 + quarter * half, 0.0),
                Vec3::new(four + half, quarter * half, four + half),
                0.0,
                0.0,
                stone,
            );
            let ring_cols = eight as usize;
            let ring_r = three + half + quarter;
            for i in 0..ring_cols {
                let a = (i as f64) * std::f64::consts::TAU / (ring_cols as f64);
                let cx = ring_r * a.cos();
                let cz = ring_r * a.sin();
                packed.append_oriented_box(
                    Vec3::new(cx, y0 + two, cz),
                    Vec3::new(quarter, two - quarter, quarter),
                    -a,
                    0.0,
                    stone,
                );
                packed.append_oriented_box(
                    Vec3::new(cx, y0 + four - quarter * half, cz),
                    Vec3::new(half, quarter * half, one_and_half(half)),
                    -a,
                    0.0,
                    stone,
                );
            }

            // Neuromorphic Observer Core: dual-axis gimbal orienting toward the player via LIF/SSM directive:
            let milli = pk_f64(prism_kernel::skinWeightScale());
            let obs_yaw = self
                .last_directive
                .as_ref()
                .map_or(sim_time_sec * half, |d| {
                    (d.observer_gimbal_yaw_milli as f64) / milli
                });
            let obs_core_col = if self
                .last_directive
                .as_ref()
                .is_some_and(|d| d.spike_mask != 0)
            {
                cyan
            } else {
                gold
            };
            let core_center = Vec3::new(0.0, y0 + two + half, 0.0);
            packed.append_oriented_box(
                core_center,
                Vec3::new(1.0 + quarter, quarter * quarter, 1.0 + quarter),
                obs_yaw,
                quarter,
                gold,
            );
            packed.append_oriented_box(
                core_center,
                Vec3::new(half, half, half),
                -obs_yaw,
                sim_time_sec * half,
                obs_core_col,
            );
        }

        if self.lod_rank >= i2_u32 {
            // Live XPBD Articulated Hinge Rig (`bodies[1]` anchor & `bodies[2]` pendulum bob):
            let i2_idx = prism_kernel::sTwo() as usize;
            if let (Some(anchor), Some(bob)) =
                (self.physics.bodies.get(1), self.physics.bodies.get(i2_idx))
            {
                packed.append_oriented_box(
                    anchor.position,
                    Vec3::new(1.0 + half, quarter * half, quarter),
                    0.0,
                    0.0,
                    slate,
                );
                let mid = Vec3::new(
                    (anchor.position.x + bob.position.x) * half,
                    (anchor.position.y + bob.position.y) * half,
                    (anchor.position.z + bob.position.z) * half,
                );
                let link_vec = bob.position - anchor.position;
                let link_len = (link_vec.length() * half).max(quarter);
                let swing_angle = link_vec.x.atan2((-link_vec.y).max(quarter * quarter));
                packed.append_oriented_box(
                    mid,
                    Vec3::new(quarter * quarter, link_len, quarter * quarter),
                    0.0,
                    swing_angle,
                    stone,
                );
                packed.append_oriented_box(
                    bob.position,
                    Vec3::new(quarter + quarter * half, quarter + quarter * half, quarter),
                    sim_time_sec,
                    swing_angle,
                    cyan,
                );
            }

            // Live 4-Wheel Suspension Vehicle along the autonomous spline / vehicle state:
            let (vx, vy, vz, vyaw) = if let Some(mover) = &self.physics.kinematic_mover {
                let scale = (sixteen + half) / (sixteen + ten);
                let mx = mover.position.x * scale;
                let mz = mover.position.z * scale;
                let my =
                    terrain_height_with_params(self.world.seed, &self.world.params.terrain, mx, mz)
                        + half;
                (mx, my, mz, mover.heading.x.atan2(mover.heading.z))
            } else if let Some(veh) = self.physics.vehicles.first() {
                (
                    veh.chassis_position.x,
                    veh.chassis_position.y,
                    veh.chassis_position.z,
                    veh.yaw_radians,
                )
            } else {
                (sixteen, 1.0, 0.0, 0.0)
            };
            packed.append_oriented_box(
                Vec3::new(vx, vy, vz),
                Vec3::new(half, quarter * half, 1.0 - quarter * half),
                vyaw,
                0.0,
                gold,
            );
            packed.append_oriented_box(
                Vec3::new(vx, vy + quarter, vz),
                Vec3::new(quarter + quarter * quarter, quarter * half, half),
                vyaw,
                0.0,
                cyan,
            );
            let cy = vyaw.cos();
            let sy = vyaw.sin();
            let wx_off = half + quarter * quarter;
            let wz_off = half + quarter * quarter;
            for (sx, sz) in [
                (-wx_off, -wz_off),
                (wx_off, -wz_off),
                (-wx_off, wz_off),
                (wx_off, wz_off),
            ] {
                let px = vx + sx * cy - sz * sy;
                let pz = vz + sx * sy + sz * cy;
                let gy =
                    terrain_height_with_params(self.world.seed, &self.world.params.terrain, px, pz)
                        + quarter;
                packed.append_oriented_box(
                    Vec3::new(px, gy, pz),
                    Vec3::new(quarter * half, quarter, quarter),
                    vyaw,
                    0.0,
                    slate,
                );
            }

            // Diegetic 3D UI Compliance Card anchored beside the central pavilion:
            packed.append_oriented_box(
                Vec3::new(six, y0 + two + half, -two),
                Vec3::new(1.0 + quarter, half + quarter, quarter * quarter),
                -quarter,
                0.0,
                cyan,
            );
        }

        let tris = packed.triangle_count();
        self.scene_buffers = packed;
        Ok(tris)
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
        let _ = self.rebuild_scene_buffers(0.0);
        Ok(())
    }

    /// Advances physics, XPBD joints, 4-wheel suspension vehicle, capsule movement, spline mover, and GJK contact.
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

    /// Renders the active Rust-owned 3D scene into `self.framebuffer_rgba` via Prism + `uor-matmul`
    /// `16x16` tile rasterization (`kernel3`).
    pub fn render_tensor_framebuffer(
        &mut self,
        width: u32,
        height: u32,
        eye: Vec3,
        target: Vec3,
    ) -> Result<usize, WasmBridgeError> {
        if self.scene_buffers.tile_triangles.is_empty() {
            let _ = self.rebuild_scene_buffers(0.0);
        }
        let tile = pk_u64(prism_kernel::tileEdgePixels()) as u32;
        let aligned_w = ((width.max(tile)) / tile) * tile;
        let aligned_h = ((height.max(tile)) / tile) * tile;
        let ten = pk_f64(prism_kernel::sTen());
        let four = pk_f64(prism_kernel::sFour());
        let two_f = prism_kernel::sTwo() as f64;
        let fov = std::f64::consts::PI / (pk_f64(prism_kernel::sThirtyFour()) / ten);
        let aspect = (aligned_w as f64) / (aligned_h.max(1) as f64);
        let vp = Mat4::perspective(
            fov,
            aspect,
            1.0 / four,
            pk_f64(prism_kernel::wasmFrustumFarMeters()),
        ) * Mat4::look_at(eye, target, Vec3::UP);

        // Keep interactive software tile rasterization responsive on mobile budgets by preserving
        // dynamic tail actors and striding dense static meshes when triangle count exceeds 1,000:
        let max_tris = pk_u64(prism_kernel::skinWeightScale()) as usize;
        let all_tris = &self.scene_buffers.tile_triangles;
        let sun = Vec3::new(1.0 / two_f, 1.0, 1.0 / four);
        let fb = if all_tris.len() <= max_tris {
            render_triangles_tile_framebuffer(
                aligned_w,
                aligned_h,
                &vp,
                sun,
                u64::from(self.lod_rank),
                all_tris,
            )
        } else {
            let tail_keep = (pk_u64(prism_kernel::sHundred()) * prism_kernel::sThree()) as usize;
            let tail_start = all_tris.len().saturating_sub(tail_keep);
            let head_budget = max_tris.saturating_sub(all_tris.len() - tail_start).max(1);
            let stride = (tail_start / head_budget).max(1);
            let mut sampled = Vec::with_capacity(max_tris);
            let mut idx = 0usize;
            while idx < tail_start && sampled.len() < head_budget {
                sampled.push(all_tris[idx]);
                idx += stride;
            }
            sampled.extend_from_slice(&all_tris[tail_start..]);
            render_triangles_tile_framebuffer(
                aligned_w,
                aligned_h,
                &vp,
                sun,
                u64::from(self.lod_rank),
                &sampled,
            )
        }
        .map_err(|e| WasmBridgeError::InvalidSnapshotEnvelope {
            byte_len: 0,
            reason: bridge_reason(e),
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
                reason: bridge_reason(e),
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
        let _ = self.rebuild_scene_buffers(0.0);
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
        let _ = self.rebuild_scene_buffers(0.0);
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
        self.custom_glb_scene = None;
        let _ = self.rebuild_scene_buffers(0.0);
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

fn one_and_half(half: f64) -> f64 {
    1.0 + half
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

/// Rebuilds the zero-copy world-space scene mesh buffers at `sim_time_sec` and returns triangle count.
#[no_mangle]
pub extern "C" fn verdant_wasm_scene_rebuild_buffers(sim_time_sec: f64) -> u32 {
    let Ok(mut guard) = global_bridge().lock() else {
        return 0;
    };
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };
    bridge.rebuild_scene_buffers(sim_time_sec).unwrap_or(0) as u32
}

/// Linear-memory byte offset of world-space `f32` vertex positions (`3 * vert_count` floats).
#[no_mangle]
pub extern "C" fn verdant_wasm_scene_pos_ptr() -> u32 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| Some(g.as_ref()?.scene_buffers.positions_xyz.as_ptr() as usize as u32))
        .unwrap_or(0)
}

/// Linear-memory byte offset of world-space `f32` vertex normals (`3 * vert_count` floats).
#[no_mangle]
pub extern "C" fn verdant_wasm_scene_norm_ptr() -> u32 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| Some(g.as_ref()?.scene_buffers.normals_xyz.as_ptr() as usize as u32))
        .unwrap_or(0)
}

/// Linear-memory byte offset of world-space `f32` vertex colors (`3 * vert_count` floats).
#[no_mangle]
pub extern "C" fn verdant_wasm_scene_col_ptr() -> u32 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| Some(g.as_ref()?.scene_buffers.colors_rgb.as_ptr() as usize as u32))
        .unwrap_or(0)
}

/// Linear-memory byte offset of `u32` triangle indices (`idx_count` integers).
#[no_mangle]
pub extern "C" fn verdant_wasm_scene_idx_ptr() -> u32 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| Some(g.as_ref()?.scene_buffers.indices.as_ptr() as usize as u32))
        .unwrap_or(0)
}

/// Total world-space vertex count in `scene_buffers`.
#[no_mangle]
pub extern "C" fn verdant_wasm_scene_vert_count() -> u32 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| Some(g.as_ref()?.scene_buffers.vertex_count() as u32))
        .unwrap_or(0)
}

/// Total `u32` triangle index count (`3 * triangle_count`) in `scene_buffers`.
#[no_mangle]
pub extern "C" fn verdant_wasm_scene_idx_count() -> u32 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| Some(g.as_ref()?.scene_buffers.indices.len() as u32))
        .unwrap_or(0)
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

/// Advances the second-order cybernetic LIF/SSM observer recurrence (`kernel10`) by one frame.
#[no_mangle]
pub extern "C" fn verdant_wasm_step_observer(
    gazed_slot: u32,
    dwell_ms: u32,
    ang_speed_milli: u32,
    loco_speed_milli: u32,
    player_yaw_milli: i32,
) -> u32 {
    let Ok(mut guard) = global_bridge().lock() else {
        return 0;
    };
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };
    let frame_dt_ms = (pk_u64(prism_kernel::sSixteen())) as u32;
    let cadence = (pk_u64(prism_kernel::sFive()) * pk_u64(prism_kernel::sHundred())) as u32;
    let sample = PlayerObserverSample {
        gazed_entity_slot: gazed_slot as usize,
        gaze_dwell_millis: dwell_ms,
        camera_angular_speed_milli: ang_speed_milli,
        locomotion_speed_milli: loco_speed_milli,
        cadence_focus_milli: cadence,
        audio_alignment_milli: (bridge.last_binaural_gain.abs()
            * pk_f64(prism_kernel::skinWeightScale())) as i32,
        ui_focus_slot: 0,
        frame_dt_millis: frame_dt_ms,
    };
    match bridge
        .observer_loop
        .step_observation(sample, i64::from(player_yaw_milli))
    {
        Ok(dir) => {
            bridge.last_directive = Some(dir);
            1
        }
        Err(_) => 0,
    }
}

/// Returns the entity slot currently attended by the world's neuromorphic observer.
#[no_mangle]
pub extern "C" fn verdant_wasm_observer_attended_slot() -> u32 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| Some(g.as_ref()?.last_directive.as_ref()?.attended_entity_slot as u32))
        .unwrap_or(0)
}

/// Returns the foveated LOD rank (`0..=3`) recommended by the neuromorphic observer.
#[no_mangle]
pub extern "C" fn verdant_wasm_observer_foveated_lod() -> u32 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| Some(g.as_ref()?.last_directive.as_ref()?.foveated_lod_rank))
        .unwrap_or(prism_kernel::wasmMaxLodRank() as u32)
}

/// Returns the world Observer Core gimbal yaw in milli-radians (`-player_yaw_milli` mutual gaze).
#[no_mangle]
pub extern "C" fn verdant_wasm_observer_gimbal_yaw_milli() -> i32 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| {
            Some(
                g.as_ref()?
                    .last_directive
                    .as_ref()?
                    .observer_gimbal_yaw_milli as i32,
            )
        })
        .unwrap_or(0)
}

/// Returns the 8-neuron LIF spike raster bitmask emitted on the latest observer tick.
#[no_mangle]
pub extern "C" fn verdant_wasm_observer_spike_mask() -> u32 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| Some(g.as_ref()?.last_directive.as_ref()?.spike_mask as u32))
        .unwrap_or(0)
}

/// Returns the salience score in milli-units for entity `slot`.
#[no_mangle]
pub extern "C" fn verdant_wasm_observer_salience_milli(slot: u32) -> i32 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| {
            g.as_ref()?
                .last_directive
                .as_ref()?
                .entity_salience_milli
                .get(slot as usize)
                .copied()
                .map(|v| v as i32)
        })
        .unwrap_or(0)
}

/// Routes a UTF-8 natural-language prompt in `io_buffer` through `CodedSlmDirector` (`kernel11`),
/// writing `"<slug>|<score_milli>|<raw_logit>|<summary>"` into `aux_buffer` and returning byte length.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_route_prompt_io() -> u32 {
    let Ok(mut guard) = global_bridge().lock() else {
        return 0;
    };
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };
    let Ok(prompt) = std::str::from_utf8(&bridge.io_buffer) else {
        return 0;
    };
    match bridge
        .slm_director
        .route_prompt(prompt, None, &bridge.grants)
    {
        Ok(prop) => {
            let formatted = format!(
                "{}|{}|{}|{}",
                prop.intent.slug(),
                prop.score_milli,
                prop.raw_logit,
                prop.summary
            );
            bridge.aux_buffer = formatted.into_bytes();
            bridge.aux_buffer.len() as u32
        }
        Err(_) => 0,
    }
}

/// Updates the denial-wins `ContainerGrantMatrix` (`organ_idx` in `0..11`; `mode`: `0` = default,
/// `1` = grant, `2` = deny) and returns `1` if the organ is currently allowed, else `0`.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_set_organ_grant(organ_idx: u32, mode: u32) -> u32 {
    let Ok(mut guard) = global_bridge().lock() else {
        return 0;
    };
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };
    let organs = [
        ContainerOrgan::WebAssemblyCore,
        ContainerOrgan::OffscreenCanvasViewport,
        ContainerOrgan::ServiceWorkerCache,
        ContainerOrgan::AudioWorkletSynth,
        ContainerOrgan::OpfsAssetCache,
        ContainerOrgan::GamepadController,
        ContainerOrgan::DeviceOrientationSensor,
        ContainerOrgan::WebCryptoPrismId,
        ContainerOrgan::BroadcastChannelTabs,
        ContainerOrgan::WebRtcStitchMesh,
        ContainerOrgan::WebNnAccelerator,
    ];
    let Some(&organ) = organs.get(organ_idx as usize) else {
        return 0;
    };
    let deny_mode = prism_kernel::sTwo() as u32;
    if mode == 0 {
        bridge.grants = ContainerGrantMatrix::default();
    } else if mode == 1 {
        bridge.grants.grant(organ);
    } else if mode == deny_mode {
        bridge.grants.deny(organ);
    }
    u32::from(bridge.grants.is_allowed(organ))
}

/// Solves and verifies the formal 2D/3D `UiTree` (`kernel9`) at `(width, height)`, writing solved
/// widget rect diagnostics into `aux_buffer` and returning the verified widget count (`0` on refusal).
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_verify_ui(width: u32, height: u32) -> u32 {
    let Ok(mut guard) = global_bridge().lock() else {
        return 0;
    };
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };
    let first_eid = bridge.world.scene.entities().next().map(|e| e.id);
    let tree = UiTree::canonical_studio_ui_tree(first_eid);
    match tree.solve_and_verify(width, height, Some(&bridge.world.scene)) {
        Ok(solved) => {
            let diegetic = UiTree::build_diegetic_bvh(&solved);
            let mut lines = Vec::with_capacity(solved.len() + 1);
            lines.push(format!(
                "viewport={width}x{height}|widgets={}|diegetic_bvh={}",
                solved.len(),
                diegetic.len()
            ));
            for r in &solved {
                let w = &tree.widgets[r.widget_index];
                lines.push(format!(
                    "{}:{},{},{}x{},contrast={}",
                    w.label, r.x_px, r.y_px, r.width_px, r.height_px, w.contrast_milli
                ));
            }
            bridge.aux_buffer = lines.join("\n").into_bytes();
            solved.len() as u32
        }
        Err(err) => {
            bridge.aux_buffer = bridge_reason(err).into_bytes();
            0
        }
    }
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

/// Resizes `io_buffer` up to `32 MiB` (`snapshotUrlMaxBytes * skinWeightScale`) for bulk host->guest
/// `.glb` / `.uorc` / prompt staging and returns its linear-memory byte offset (`0` on refusal).
#[no_mangle]
pub extern "C" fn verdant_wasm_io_resize(byte_len: u32) -> u32 {
    let Ok(mut guard) = global_bridge().lock() else {
        return 0;
    };
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };
    let max_bulk = (pk_u64(prism_kernel::snapshotUrlMaxBytes())
        * pk_u64(prism_kernel::skinWeightScale())) as usize;
    let target = byte_len as usize;
    if target > max_bulk {
        return 0;
    }
    bridge.io_buffer.resize(target, 0);
    bridge.io_buffer.as_mut_ptr() as usize as u32
}

/// Linear-memory byte offset of `io_buffer`.
#[no_mangle]
pub extern "C" fn verdant_wasm_io_ptr() -> u32 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| Some(g.as_ref()?.io_buffer.as_ptr() as usize as u32))
        .unwrap_or(0)
}

/// Linear-memory byte offset of `aux_buffer`.
#[no_mangle]
pub extern "C" fn verdant_wasm_aux_ptr() -> u32 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| Some(g.as_ref()?.aux_buffer.as_ptr() as usize as u32))
        .unwrap_or(0)
}

/// Current byte length of `aux_buffer`.
#[no_mangle]
pub extern "C" fn verdant_wasm_aux_len() -> u32 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| Some(g.as_ref()?.aux_buffer.len() as u32))
        .unwrap_or(0)
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

/// Number of base Prism `uor-matmul` tensor kernels (`7`).
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

/// Number of unified Prism semiring tensor kernels (`12`, covering linear, tropical max/min,
/// audio ring, coded stream, tropical UI layout, LIF/SSM observer, and `coded_gemm` SLM).
#[no_mangle]
pub extern "C" fn verdant_wasm_unified_tensor_kernel_count() -> u32 {
    #[cfg(target_arch = "wasm32")]
    {
        pk_u64(prism_kernel::sTwelve()) as u32
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
        let k9_ok = prism_kernel::kernel9TropicalUiLayout()
            .map(|s| prism_kernel::tensorKernelValid(&s))
            .unwrap_or(false);
        let k10_ok = prism_kernel::kernel10NeuromorphicSalienceSsm()
            .map(|s| prism_kernel::tensorKernelValid(&s))
            .unwrap_or(false);
        let k11_ok = prism_kernel::kernel11CodedSlmReasoning()
            .map(|s| prism_kernel::tensorKernelValid(&s))
            .unwrap_or(false);
        if base_ok && k7_ok && k8_ok && k9_ok && k10_ok && k11_ok {
            pk_u64(prism_kernel::sTwelve()) as u32
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

/// Imports a binary Khronos `.glb` scene from `io_buffer` into `WasmRuntimeBridge`, packs its
/// world-space render buffers for immediate WebGL2 / `16x16` tile rendering, writes an authenticated
/// `VDNT` snapshot into `aux_buffer`, and returns the packed triangle count (`0` on refusal).
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_glb_import_io() -> u32 {
    let Ok(mut guard) = global_bridge().lock() else {
        return 0;
    };
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };
    let Ok(imported) = import_glb_to_scene(&bridge.io_buffer) else {
        return 0;
    };
    bridge.bvh = imported.scene.build_bvh();
    bridge.custom_glb_scene = Some((imported.scene, imported.meshes, imported.materials));
    let tris = bridge.rebuild_scene_buffers(0.0).unwrap_or(0);
    if let Ok(snap) = WorldSnapshot::encode(&bridge.world) {
        bridge.aux_buffer = snap;
    }
    tris as u32
}

/// Exports the active Rust `SceneWorld` (either user-imported `.glb` or canonical scene) to binary
/// Khronos glTF 2.0 (`.glb`) in `aux_buffer`, returning byte length (`0` on refusal).
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_glb_export_aux() -> u32 {
    let Ok(mut guard) = global_bridge().lock() else {
        return 0;
    };
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };
    let exported = if let Some((ref sc, ref ms, ref mats)) = bridge.custom_glb_scene {
        export_scene_to_glb(sc, ms, mats)
    } else {
        let meshes = canonical_scene_mesh_library(bridge.world.seed);
        let mats = canonical_pbr_material_library();
        export_scene_to_glb(&bridge.world.scene, &meshes, &mats)
    };
    match exported {
        Ok(bytes) => {
            bridge.aux_buffer = bytes;
            bridge.aux_buffer.len() as u32
        }
        Err(_) => 0,
    }
}

/// Clears any user-imported `.glb` scene override and restores the procedural `StandardSceneState` meshes.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_clear_imported_glb() -> u32 {
    let Ok(mut guard) = global_bridge().lock() else {
        return 0;
    };
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };
    bridge.custom_glb_scene = None;
    bridge.bvh = bridge.world.scene.build_bvh();
    bridge.rebuild_scene_buffers(0.0).unwrap_or(0) as u32
}

/// Shards `io_buffer` into 3 `afflom/stitch` `(A, B, A ⊕ B)` parity fragments, writing `STCH` binary
/// wire bytes for `shard_index` (`0..=2`) into `aux_buffer` and returning `aux_buffer.len()`.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_stitch_shard_io(shard_index: u32) -> u32 {
    let Ok(mut guard) = global_bridge().lock() else {
        return 0;
    };
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };
    let Ok(frags) = shard_bytes_to_stitch_fragments(&bridge.io_buffer) else {
        return 0;
    };
    let Some(frag) = frags.get(shard_index as usize) else {
        return 0;
    };
    bridge.aux_buffer = frag.encode_wire();
    bridge.aux_buffer.len() as u32
}

/// Recovers original bytes from two concatenated `STCH` wire fragments in `io_buffer` split at
/// `split_offset`, writing the verified recovered bytes into `aux_buffer` and returning byte length.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_stitch_recover_io(split_offset: u32) -> u32 {
    let Ok(mut guard) = global_bridge().lock() else {
        return 0;
    };
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };
    let split = split_offset as usize;
    if split == 0 || split >= bridge.io_buffer.len() {
        return 0;
    }
    let Ok(frag_a) = StitchFragment::decode_wire(&bridge.io_buffer[..split]) else {
        return 0;
    };
    let Ok(frag_b) = StitchFragment::decode_wire(&bridge.io_buffer[split..]) else {
        return 0;
    };
    let Ok(recovered) = recover_bytes_from_stitch_pair(&frag_a, &frag_b) else {
        return 0;
    };
    bridge.aux_buffer = recovered;
    bridge.aux_buffer.len() as u32
}
