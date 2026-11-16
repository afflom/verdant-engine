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
use verdant_core::camera::{CameraMode, CameraRigConfig, LawfulCameraRig, SolvedCameraFrame};
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
use verdant_core::character::{ArticulatedCharacter, CharacterArchetype};
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
use verdant_core::cybernetics::{CodedSlmDirector, ContainerGrantMatrix, ContainerOrgan};
use verdant_core::cybernetics::{
    CyberneticObserverLoop, PlayerObserverSample, WorldAttentionDirective,
};
use verdant_core::ecs::TwoLevelBvh;
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
use verdant_core::ecs::{BoundingVolumeHierarchy, SceneWorld};
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
use verdant_core::geometry::ProceduralMesh;
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
use verdant_core::gltf_io::{export_scene_to_glb, import_glb_to_scene};
use verdant_core::hd_world::{hd_terrain_height, synthesize_hd_citadel_sector, HdSectorBundle};
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
use verdant_core::math::Mat4;
use verdant_core::math::{Frustum, Vec3};
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
use verdant_core::narrative::{canonical_citadel_chronicle_automaton, StoryAutomaton};
use verdant_core::navmesh::NavMesh;
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
use verdant_core::network::{
    recover_bytes_from_stitch_pair, shard_bytes_to_stitch_fragments, StitchFragment,
};
use verdant_core::physics::{
    CapsuleController, CharacterMoveIntent, JointKind, KinematicSplineBody, PhysicsWorld,
    PhysicsWorldConfig, RigidBody, RigidJointConstraint,
};
use verdant_core::prism_kernel;
use verdant_core::render::{
    canonical_pbr_material_library, pack_world_meshes_to_buffers, PackedSceneRenderBuffers,
};
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
use verdant_core::render::{compute_csm_splits, PbrMaterial};
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
use verdant_core::runtime::WorldSnapshot;
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
use verdant_core::streaming::{
    decode_portable_uorc_single_frame, CodedLodFrameBundle, StandardSceneState,
};
#[cfg(not(target_arch = "wasm32"))]
use verdant_core::streaming::{decode_uorc_stream_up_to_lod, pack_progressive_uorc_stream};
use verdant_core::streaming::{
    terrain_height_with_params, PhysicalEnvironment, TerrainReliefParams,
};
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
use verdant_core::tensor::render_triangles_tile_framebuffer;
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
use verdant_core::ui::UiTree;
use verdant_core::universe::{UniverseManager, UniversePosition};

/// Refusal returned when a browser worker command, LOD rank, or snapshot/UORC envelope is invalid.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WasmBridgeError {
    /// Requested progressive LOD rank exceeds `0..=4`.
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
                write!(f, "invalid progressive LOD rank {rank}; expected 0..=4")
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
    /// Deterministic world seed (`u64`).
    pub seed_val: u64,
    /// Terrain relief parameters.
    pub terrain_params: TerrainReliefParams,
    /// Planetary physical environment parameters.
    pub env_params: PhysicalEnvironment,
    /// Progressive multi-LOD scene state at the active LOD rank.
    #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
    pub world: StandardSceneState,
    /// Active progressive LOD rank (`0..=4`).
    pub lod_rank: u32,
    /// Deterministic rigid-body, XPBD joint, vehicle, capsule, and spline mover physics world.
    pub physics: PhysicsWorld,
    /// Prebuilt SAH BVH over the world scene graph.
    #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
    pub bvh: BoundingVolumeHierarchy,
    /// Walkable polygon count extracted from the primary terrain mesh.
    pub walkable_polygons: usize,
    /// Whether the player capsule is currently inside or contacting the central Citadel hall.
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
    #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
    pub framebuffer_rgba: Vec<u8>,
    /// Zero-copy world-space vertex, normal, color, index, and `16x16` tile triangle buffers.
    pub scene_buffers: PackedSceneRenderBuffers,
    /// Second-order neuromorphic LIF/SSM observer recurrence coupling player and world attention.
    pub observer_loop: CyberneticObserverLoop,
    /// Most recent `WorldAttentionDirective` emitted by `observer_loop`.
    pub last_directive: Option<WorldAttentionDirective>,
    /// High-definition sector bundle (`VaultedCitadelHall`, `ParabolicStoneArchBridge`, sculpted terrain, trees).
    pub hd_sector: HdSectorBundle,
    /// Exact two-level collision BVH (`Entity TLAS` + triangle `MeshBlas`) enforcing solid geometry laws.
    pub two_level_bvh: TwoLevelBvh,
    /// Lawful spring-arm and first-person camera rig with near-frustum guard-sphere sweeps.
    pub camera_rig: LawfulCameraRig,
    /// Latest solved lawful camera pose and matrices.
    pub last_camera_frame: SolvedCameraFrame,
    /// Optional landmark anchor override (`None` = possessed Explorer character).
    pub camera_anchor_override: Option<Vec3>,
    /// Current possessed Explorer foot position in world meters.
    pub explorer_foot: Vec3,
    /// Current possessed Explorer facing yaw in radians.
    pub explorer_yaw: f64,
    /// Possessed 22-joint Explorer character with LBS skinning and analytical Two-Bone Foot IK.
    #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
    pub explorer: ArticulatedCharacter,
    /// Articulated 22-joint NPC guides (`Archivist`, `BridgeEngineer`, `ObservatoryKeeper`).
    #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
    pub npcs: Vec<ArticulatedCharacter>,
    /// Floating-origin universe manager (`UniversePosition` + `PortalGraph`).
    pub universe: UniverseManager,
    /// Conservation-verified Chronicle automaton (`kernel14ConservationAutomaton`).
    #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
    pub story: StoryAutomaton,
    /// Cached static `positions_xyz` float count for sub-millisecond dynamic buffer appends.
    static_pos_len: usize,
    /// Cached static `indices` count for sub-millisecond dynamic buffer appends.
    static_idx_len: usize,
    /// Cached static `tile_triangles` count for sub-millisecond dynamic buffer appends.
    static_tri_len: usize,
    /// Whether static HD sector render buffers must be repacked before appending dynamic actors.
    static_dirty: bool,
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
    /// Initializes a deterministic runtime from `seed` (`LOD2` fast cold-boot on `wasm32`, `LOD3` on native/UORC).
    pub fn new(seed: u64) -> Result<Self, WasmBridgeError> {
        let clean_seed = seed.max(1);
        #[cfg(all(target_arch = "wasm32", not(feature = "uorc-decoder")))]
        let init_rank = prism_kernel::sTwo();
        #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
        let init_rank = prism_kernel::wasmMaxLodRank();

        let terrain_params = TerrainReliefParams::standard_basin_valley();
        let env_params = PhysicalEnvironment::temperate_terrestrial();

        #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
        let world = {
            let mut w = StandardSceneState::new_at_lod0(clean_seed).map_err(|e| {
                WasmBridgeError::InvalidSnapshotEnvelope {
                    byte_len: 0,
                    reason: bridge_reason(e),
                }
            })?;
            w.advance_to_lod(init_rank)
                .map_err(|e| WasmBridgeError::InvalidSnapshotEnvelope {
                    byte_len: 0,
                    reason: bridge_reason(e),
                })?;
            w
        };
        #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
        let bvh = world.scene.build_bvh();

        let physics = Self::build_physics_from_params(clean_seed, terrain_params, env_params);

        let slot_count = (pk_u64(prism_kernel::sSeven()) as usize).max(1);
        let observer_loop = CyberneticObserverLoop::new(slot_count).map_err(|e| {
            WasmBridgeError::InvalidSnapshotEnvelope {
                byte_len: 0,
                reason: bridge_reason(e),
            }
        })?;

        let hd_sector = synthesize_hd_citadel_sector(
            clean_seed,
            &terrain_params,
            env_params.water_level_meters,
            init_rank,
        );
        let walkable_polygons = hd_sector
            .meshes
            .first()
            .map(NavMesh::count_walkable_triangles)
            .unwrap_or(0);
        let two_level_bvh =
            TwoLevelBvh::build_solid(&hd_sector.scene, &hd_sector.meshes).map_err(|e| {
                WasmBridgeError::InvalidSnapshotEnvelope {
                    byte_len: 0,
                    reason: bridge_reason(e),
                }
            })?;

        let two = prism_kernel::sTwo() as f64;
        let half = 1.0 / two;
        let eleven_half = pk_f64(prism_kernel::sEleven()) + half;
        let exp_y = hd_terrain_height(clean_seed, &terrain_params, 0.0, eleven_half);
        let exp_start = Vec3::new(0.0, exp_y, eleven_half);

        #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
        let (explorer, npcs) = {
            let four = pk_f64(prism_kernel::sFour());
            let nine = pk_f64(prism_kernel::sNine());
            let mut exp = ArticulatedCharacter::new_humanoid(
                "Explorer",
                CharacterArchetype::Explorer,
                exp_start,
                init_rank,
            )
            .map_err(|e| WasmBridgeError::InvalidSnapshotEnvelope {
                byte_len: 0,
                reason: bridge_reason(e),
            })?;
            exp.body_yaw_radians = std::f64::consts::PI;

            let podium_y = hd_terrain_height(clean_seed, &terrain_params, 0.0, 0.0)
                + (prism_kernel::sThree() as f64) / pk_f64(prism_kernel::sFive());
            let archivist = ArticulatedCharacter::new_humanoid(
                "Archivist",
                CharacterArchetype::Archivist,
                Vec3::new(1.0 + half, podium_y, -1.0),
                1 + 1,
            )
            .map_err(|e| WasmBridgeError::InvalidSnapshotEnvelope {
                byte_len: 0,
                reason: bridge_reason(e),
            })?;

            let bx = terrain_params.basin_center_x - four - two;
            let bz = terrain_params.basin_center_z;
            let by = hd_terrain_height(clean_seed, &terrain_params, bx, bz);
            let engineer = ArticulatedCharacter::new_humanoid(
                "BridgeEngineer",
                CharacterArchetype::BridgeEngineer,
                Vec3::new(bx, by, bz),
                1 + 1,
            )
            .map_err(|e| WasmBridgeError::InvalidSnapshotEnvelope {
                byte_len: 0,
                reason: bridge_reason(e),
            })?;

            let oy = hd_terrain_height(clean_seed, &terrain_params, -nine, -four);
            let keeper = ArticulatedCharacter::new_humanoid(
                "ObservatoryKeeper",
                CharacterArchetype::ObservatoryKeeper,
                Vec3::new(-nine, oy, -four),
                1 + 1,
            )
            .map_err(|e| WasmBridgeError::InvalidSnapshotEnvelope {
                byte_len: 0,
                reason: bridge_reason(e),
            })?;
            let mut npc_list = vec![archivist, engineer, keeper];
            let _ = exp.step_and_solve_ik(
                half * half,
                exp_start,
                Vec3::ZERO,
                true,
                0.0,
                Some(Vec3::new(0.0, podium_y + two, 0.0)),
                &two_level_bvh,
                |x, z| hd_terrain_height(clean_seed, &terrain_params, x, z),
            );
            for npc in &mut npc_list {
                let p = npc.foot_position;
                let _ = npc.step_and_solve_ik(
                    half * half,
                    p,
                    Vec3::ZERO,
                    true,
                    0.0,
                    Some(exp_start + Vec3::new(0.0, 1.0 + half, 0.0)),
                    &two_level_bvh,
                    |x, z| hd_terrain_height(clean_seed, &terrain_params, x, z),
                );
            }
            (exp, npc_list)
        };

        let cam_cfg = CameraRigConfig::default();
        let init_yaw = half;
        let init_pitch = half * half;
        let init_dist = pk_f64(prism_kernel::sSixteen());
        let mut camera_rig = LawfulCameraRig::new(
            cam_cfg,
            CameraMode::ThirdPersonSpringArm,
            init_yaw,
            init_pitch,
            init_dist,
        )
        .map_err(|e| WasmBridgeError::InvalidSnapshotEnvelope {
            byte_len: 0,
            reason: bridge_reason(e),
        })?;
        let head_anchor = Vec3::new(exp_start.x, exp_start.y + 1.0 + half, exp_start.z);
        let last_camera_frame = camera_rig
            .solve_frame(half * half, head_anchor, &two_level_bvh, |x, z| {
                hd_terrain_height(clean_seed, &terrain_params, x, z)
            })
            .map_err(|e| WasmBridgeError::InvalidSnapshotEnvelope {
                byte_len: 0,
                reason: bridge_reason(e),
            })?;

        let universe = UniverseManager::canonical_citadel_universe(clean_seed).map_err(|e| {
            WasmBridgeError::InvalidSnapshotEnvelope {
                byte_len: 0,
                reason: bridge_reason(e),
            }
        })?;
        #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
        let (story, _) = canonical_citadel_chronicle_automaton(clean_seed).map_err(|e| {
            WasmBridgeError::InvalidSnapshotEnvelope {
                byte_len: 0,
                reason: bridge_reason(e),
            }
        })?;

        let mut bridge = Self {
            seed_val: clean_seed,
            terrain_params,
            env_params,
            #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
            world,
            lod_rank: init_rank as u32,
            physics,
            #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
            bvh,
            walkable_polygons,
            pavilion_contact: false,
            queued_commands: 0,
            last_binaural_gain: 0.0,
            last_binaural_itd_micros: 0.0,
            io_buffer: Vec::with_capacity(
                pk_u64(prism_kernel::wasmIoInitialCapacityBytes()) as usize
            ),
            aux_buffer: Vec::new(),
            #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
            framebuffer_rgba: Vec::new(),
            scene_buffers: PackedSceneRenderBuffers::default(),
            observer_loop,
            last_directive: None,
            hd_sector,
            two_level_bvh,
            camera_rig,
            last_camera_frame,
            camera_anchor_override: None,
            explorer_foot: exp_start,
            explorer_yaw: std::f64::consts::PI,
            #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
            explorer,
            #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
            npcs,
            universe,
            #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
            story,
            static_pos_len: 0,
            static_idx_len: 0,
            static_tri_len: 0,
            static_dirty: true,
            #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
            slm_director: CodedSlmDirector::new(),
            #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
            grants: ContainerGrantMatrix::default(),
            #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
            custom_glb_scene: None,
        };
        if let Some(cap) = &mut bridge.physics.player_capsule {
            cap.foot_position = exp_start;
        }
        let _ = bridge.rebuild_scene_buffers(0.0);
        Ok(bridge)
    }

    fn build_physics_from_params(
        seed: u64,
        terrain_relief: TerrainReliefParams,
        environment: PhysicalEnvironment,
    ) -> PhysicsWorld {
        let cfg = PhysicsWorldConfig {
            terrain_seed: seed,
            environment,
            terrain_relief,
            platform_deck: None,
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

        let rig_x = -(pk_f64(prism_kernel::sSixty()) + five) / ten;
        let rig_z = nine;
        let rig_y = terrain_height_with_params(seed, &terrain_relief, rig_x, rig_z);
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
        let wp_count = pk_u64(prism_kernel::sSixteen()) as usize;
        let r_spline = pk_f64(prism_kernel::sTwentySix());
        let mut waypoints = Vec::with_capacity(wp_count);
        for i in 0..wp_count {
            let theta = (i as f64) * std::f64::consts::TAU / (wp_count as f64);
            waypoints.push(Vec3::new(
                r_spline * theta.cos(),
                1.0,
                r_spline * theta.sin(),
            ));
        }
        physics.kinematic_mover = Some(KinematicSplineBody::new(waypoints, r_spline / ten));
        physics
    }

    /// Rebuilds `self.scene_buffers` from the cached static `HdSectorBundle` (`VaultedCitadelHall`,
    /// `ParabolicStoneArchBridge`, sculpted terrain, canopy trees) plus dynamic skinned characters
    /// (`Explorer` + 3 `NPC`s), Observer Core gimbal, XPBD hinge rig, 4-wheel vehicle, and diegetic card.
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
            self.static_dirty = true;
            return Ok(tris);
        }

        if self.static_dirty || self.static_idx_len == 0 {
            let materials = canonical_pbr_material_library();
            let base = pack_world_meshes_to_buffers(
                &self.hd_sector.scene,
                &self.hd_sector.meshes,
                &materials,
            )
            .map_err(|e| WasmBridgeError::InvalidSnapshotEnvelope {
                byte_len: 0,
                reason: bridge_reason(e),
            })?;
            self.static_pos_len = base.positions_xyz.len();
            self.static_idx_len = base.indices.len();
            self.static_tri_len = base.tile_triangles.len();
            self.scene_buffers = base;
            self.static_dirty = false;
        } else {
            self.scene_buffers
                .positions_xyz
                .truncate(self.static_pos_len);
            self.scene_buffers.normals_xyz.truncate(self.static_pos_len);
            self.scene_buffers.colors_rgb.truncate(self.static_pos_len);
            self.scene_buffers.indices.truncate(self.static_idx_len);
            self.scene_buffers
                .tile_triangles
                .truncate(self.static_tri_len);
        }

        let two = prism_kernel::sTwo() as f64;
        let three = prism_kernel::sThree() as f64;
        let six = pk_f64(prism_kernel::sSix());
        let eight = pk_f64(prism_kernel::sEight());
        let ten = pk_f64(prism_kernel::sTen());
        let sixteen = pk_f64(prism_kernel::sSixteen());
        let half = 1.0 / two;
        let quarter = half * half;
        let i2_u32 = prism_kernel::sTwo() as u32;

        let y0 = hd_terrain_height(self.seed_val, &self.terrain_params, 0.0, 0.0);
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
            // Sleek Neuromorphic Observer Core suspended inside the groin-vaulted Citadel dome:
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
            let core_center = Vec3::new(0.0, y0 + three + quarter, 0.0);
            self.scene_buffers.append_oriented_box(
                core_center,
                Vec3::new(
                    quarter + quarter * half,
                    quarter * quarter,
                    quarter + quarter * half,
                ),
                obs_yaw,
                quarter,
                gold,
            );
            self.scene_buffers.append_oriented_box(
                core_center,
                Vec3::new(quarter, quarter, quarter),
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
                self.scene_buffers.append_oriented_box(
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
                self.scene_buffers.append_oriented_box(
                    mid,
                    Vec3::new(quarter * quarter, link_len, quarter * quarter),
                    0.0,
                    swing_angle,
                    stone,
                );
                self.scene_buffers.append_oriented_box(
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
                let my = hd_terrain_height(self.seed_val, &self.terrain_params, mx, mz) + half;
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
            self.scene_buffers.append_oriented_box(
                Vec3::new(vx, vy, vz),
                Vec3::new(half, quarter * half, 1.0 - quarter * half),
                vyaw,
                0.0,
                gold,
            );
            self.scene_buffers.append_oriented_box(
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
                let gy = hd_terrain_height(self.seed_val, &self.terrain_params, px, pz) + quarter;
                self.scene_buffers.append_oriented_box(
                    Vec3::new(px, gy, pz),
                    Vec3::new(quarter * half, quarter, quarter),
                    vyaw,
                    0.0,
                    slate,
                );
            }

            // Diegetic 3D UI Compliance Card anchored on the Citadel south terrace:
            let card_y = hd_terrain_height(self.seed_val, &self.terrain_params, six, eight) + two;
            self.scene_buffers.append_oriented_box(
                Vec3::new(six, card_y, eight),
                Vec3::new(1.0 + quarter, half + quarter, quarter * quarter),
                -quarter,
                0.0,
                cyan,
            );
        }

        let hide_head = self.camera_rig.mode == CameraMode::FirstPersonEye
            && self.camera_anchor_override.is_none();

        #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
        {
            let _ = self
                .explorer
                .append_skinned_to_buffers(&mut self.scene_buffers, hide_head);
            if self.lod_rank >= 1 {
                for npc in &self.npcs {
                    let _ = npc.append_skinned_to_buffers(&mut self.scene_buffers, false);
                }
            }
        }

        #[cfg(all(target_arch = "wasm32", not(feature = "uorc-decoder")))]
        {
            append_cold_boot_humanoid(
                &mut self.scene_buffers,
                self.explorer_foot,
                self.explorer_yaw,
                stone,
                cyan,
                hide_head,
            );
            if self.lod_rank >= 1 {
                let podium_y = y0 + three / pk_f64(prism_kernel::sFive());
                append_cold_boot_humanoid(
                    &mut self.scene_buffers,
                    Vec3::new(1.0 + half, podium_y, -1.0),
                    0.0,
                    gold,
                    cyan,
                    false,
                );
                let four = pk_f64(prism_kernel::sFour());
                let bx = self.terrain_params.basin_center_x - four - two;
                let bz = self.terrain_params.basin_center_z;
                let by = hd_terrain_height(self.seed_val, &self.terrain_params, bx, bz);
                append_cold_boot_humanoid(
                    &mut self.scene_buffers,
                    Vec3::new(bx, by, bz),
                    half,
                    slate,
                    cyan,
                    false,
                );
            }
        }

        Ok(self.scene_buffers.triangle_count())
    }

    /// Solves the lawful spring-arm or first-person camera frame against `two_level_bvh` + terrain.
    pub fn solve_active_camera(
        &mut self,
        dt_seconds: f64,
    ) -> Result<SolvedCameraFrame, WasmBridgeError> {
        #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
        let default_head = self.explorer.head_anchor_world();
        #[cfg(all(target_arch = "wasm32", not(feature = "uorc-decoder")))]
        let default_head = {
            let half = 1.0 / (prism_kernel::sTwo() as f64);
            Vec3::new(
                self.explorer_foot.x,
                self.explorer_foot.y + 1.0 + half,
                self.explorer_foot.z,
            )
        };
        let anchor = self.camera_anchor_override.unwrap_or(default_head);
        let seed_copy = self.seed_val;
        let relief_copy = self.terrain_params;
        let solved = self
            .camera_rig
            .solve_frame(dt_seconds, anchor, &self.two_level_bvh, |x, z| {
                hd_terrain_height(seed_copy, &relief_copy, x, z)
            })
            .map_err(|e| WasmBridgeError::InvalidSnapshotEnvelope {
                byte_len: 0,
                reason: bridge_reason(e),
            })?;
        self.last_camera_frame = solved;
        Ok(solved)
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

    /// Rebuilds progressive scene state at `rank` (`0..=4`, including `LOD4` HD geometry) from `LOD0`.
    pub fn set_lod_rank(&mut self, rank: u32) -> Result<(), WasmBridgeError> {
        let max_hd_rank = pk_u64(prism_kernel::wasmHdMaxLodRank()) as u32;
        if rank > max_hd_rank {
            return Err(WasmBridgeError::InvalidLodRank { rank });
        }

        #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
        {
            let _env = prism_kernel::lodEnvelopeByRank(u64::from(rank))
                .map_err(|_| WasmBridgeError::InvalidLodRank { rank })?;
            let base_rank = u64::from(rank).min(prism_kernel::wasmMaxLodRank());
            let mut fresh = StandardSceneState::new_at_lod0(self.seed_val).map_err(|e| {
                WasmBridgeError::InvalidSnapshotEnvelope {
                    byte_len: 0,
                    reason: bridge_reason(e),
                }
            })?;
            fresh.advance_to_lod(base_rank).map_err(|e| {
                WasmBridgeError::InvalidSnapshotEnvelope {
                    byte_len: 0,
                    reason: bridge_reason(e),
                }
            })?;
            self.bvh = fresh.scene.build_bvh();
            self.world = fresh;
        }

        let hd_sector = synthesize_hd_citadel_sector(
            self.seed_val,
            &self.terrain_params,
            self.env_params.water_level_meters,
            u64::from(rank),
        );
        let two_level_bvh =
            TwoLevelBvh::build_solid(&hd_sector.scene, &hd_sector.meshes).map_err(|e| {
                WasmBridgeError::InvalidSnapshotEnvelope {
                    byte_len: 0,
                    reason: bridge_reason(e),
                }
            })?;

        #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
        {
            let prev_foot = self.explorer.foot_position;
            let prev_yaw = self.explorer.body_yaw_radians;
            let prev_phase = self.explorer.gait_phase_radians;
            if let Ok(mut rebuilt_exp) = ArticulatedCharacter::new_humanoid(
                "Explorer",
                CharacterArchetype::Explorer,
                prev_foot,
                u64::from(rank),
            ) {
                rebuilt_exp.body_yaw_radians = prev_yaw;
                rebuilt_exp.gait_phase_radians = prev_phase;
                let seed_copy = self.seed_val;
                let relief_copy = self.terrain_params;
                let half = 1.0 / (prism_kernel::sTwo() as f64);
                let _ = rebuilt_exp.step_and_solve_ik(
                    half * half,
                    prev_foot,
                    Vec3::ZERO,
                    true,
                    prev_yaw,
                    None,
                    &two_level_bvh,
                    |x, z| hd_terrain_height(seed_copy, &relief_copy, x, z),
                );
                self.explorer = rebuilt_exp;
            }
        }

        self.hd_sector = hd_sector;
        self.two_level_bvh = two_level_bvh;
        self.lod_rank = rank;
        self.static_dirty = true;
        let _ = self.solve_active_camera(0.0);
        let _ = self.rebuild_scene_buffers(0.0);
        Ok(())
    }

    /// Advances physics via `PhysicsWorld::step_lawful` (`TwoLevelBvh` swept CCD + crease projection),
    /// updates floating-origin `UniversePosition` + `PortalGraph`, steps 22-joint IK characters, and
    /// solves `LawfulCameraRig` (`cameraNearSphereGuarded`).
    pub fn step_frame_lawful(
        &mut self,
        dt_seconds: f64,
        move_forward: f64,
        move_strafe: f64,
        walk_yaw_rad: f64,
        speed_scale: f64,
        jump_requested: bool,
    ) -> Result<(), WasmBridgeError> {
        self.queued_commands = 0;
        let min_dt = 1.0 / pk_f64(prism_kernel::skinWeightScale());
        let max_dt = pk_f64(prism_kernel::sFive()) / pk_f64(prism_kernel::sHundred());
        let clamped_dt = dt_seconds.max(min_dt).min(max_dt);
        let wish = Vec3::new(
            walk_yaw_rad.sin() * move_forward - walk_yaw_rad.cos() * move_strafe,
            0.0,
            walk_yaw_rad.cos() * move_forward + walk_yaw_rad.sin() * move_strafe,
        );

        let prev_local = self
            .physics
            .player_capsule
            .as_ref()
            .map(|c| c.foot_position)
            .unwrap_or(self.explorer_foot);

        self.physics
            .step_lawful(
                clamped_dt,
                CharacterMoveIntent {
                    planar_wish_dir: wish,
                    speed_scale: speed_scale.max(0.0),
                    jump_requested,
                },
                &self.two_level_bvh,
            )
            .map_err(|_| WasmBridgeError::InvalidSnapshotEnvelope {
                byte_len: 0,
                reason: "lawful physics step out of bounds".to_string(),
            })?;

        let mut cap_pos = prev_local;
        let mut cap_vel = Vec3::ZERO;
        #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
        let mut cap_grounded = true;

        if let Some(capsule) = &mut self.physics.player_capsule {
            cap_pos = capsule.foot_position;
            cap_vel = capsule.velocity;
            #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
            {
                cap_grounded = capsule.grounded;
            }

            if let Ok(next_u) =
                UniversePosition::new_canonical(self.universe.active_position.cell, cap_pos)
            {
                if let Ok(Some((dest_u, dest_vel, portal))) =
                    self.universe.portal_graph.try_traverse(&next_u, cap_vel)
                {
                    let yaw_delta = portal.yaw_delta_radians;
                    self.universe.active_position = dest_u;
                    capsule.foot_position = dest_u.local;
                    capsule.velocity = dest_vel;
                    cap_pos = dest_u.local;
                    cap_vel = dest_vel;
                    self.camera_rig
                        .apply_look_and_zoom_intent(yaw_delta, 0.0, 0.0);
                    self.explorer_yaw =
                        (self.explorer_yaw + yaw_delta).rem_euclid(std::f64::consts::TAU);
                } else {
                    self.universe.active_position = next_u;
                }
            }

            let six = pk_f64(prism_kernel::sSix());
            self.pavilion_contact = (cap_pos.x * cap_pos.x + cap_pos.z * cap_pos.z) <= six * six;
        }

        self.explorer_foot = cap_pos;
        if cap_vel.x * cap_vel.x + cap_vel.z * cap_vel.z > (min_dt * min_dt) {
            self.explorer_yaw = cap_vel.x.atan2(cap_vel.z);
        }
        if self.camera_rig.mode == CameraMode::FirstPersonEye
            && self.camera_anchor_override.is_none()
        {
            self.explorer_yaw = (self.camera_rig.yaw_radians + std::f64::consts::PI)
                .rem_euclid(std::f64::consts::TAU);
        }

        #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
        {
            self.explorer.body_yaw_radians = self.explorer_yaw;
            let seed_copy = self.seed_val;
            let relief_copy = self.terrain_params;
            let two = prism_kernel::sTwo() as f64;
            let gaze_target = self
                .npcs
                .iter()
                .min_by(|a, b| {
                    let da = (a.foot_position - cap_pos).length_squared();
                    let db = (b.foot_position - cap_pos).length_squared();
                    da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
                })
                .and_then(|npc| {
                    let six = pk_f64(prism_kernel::sSix());
                    if (npc.foot_position - cap_pos).length_squared() <= six * six {
                        Some(npc.head_anchor_world())
                    } else {
                        None
                    }
                })
                .or_else(|| {
                    let y0 = hd_terrain_height(seed_copy, &relief_copy, 0.0, 0.0);
                    Some(Vec3::new(0.0, y0 + two + 1.0, 0.0))
                });

            let _ = self.explorer.step_and_solve_ik(
                clamped_dt,
                cap_pos,
                cap_vel,
                cap_grounded,
                walk_yaw_rad,
                gaze_target,
                &self.two_level_bvh,
                |x, z| hd_terrain_height(seed_copy, &relief_copy, x, z),
            );

            let player_head = self.explorer.head_anchor_world();
            for npc in &mut self.npcs {
                let p = npc.foot_position;
                let _ = npc.step_and_solve_ik(
                    clamped_dt,
                    p,
                    Vec3::ZERO,
                    true,
                    npc.body_yaw_radians,
                    Some(player_head),
                    &self.two_level_bvh,
                    |x, z| hd_terrain_height(seed_copy, &relief_copy, x, z),
                );
            }
        }

        let _ = self.solve_active_camera(clamped_dt)?;
        Ok(())
    }

    /// Advances physics, XPBD joints, 4-wheel suspension vehicle, capsule movement, spline mover, and GJK contact.
    #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
    pub fn step_frame(
        &mut self,
        dt_seconds: f64,
        move_forward: f64,
        move_strafe: f64,
        walk_yaw_rad: f64,
    ) -> Result<(), WasmBridgeError> {
        self.step_frame_lawful(
            dt_seconds,
            move_forward,
            move_strafe,
            walk_yaw_rad,
            1.0,
            false,
        )
    }

    /// Renders the active Rust-owned 3D scene into `self.framebuffer_rgba` via Prism + `uor-matmul`
    /// `16x16` tile rasterization (`kernel3`).
    #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
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
    #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
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
    #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
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
        let rank = restored.active_lod.rank();
        self.lod_rank = rank as u32;
        self.seed_val = restored.seed;
        self.terrain_params = restored.params.terrain;
        self.env_params = restored.params.environment;
        self.bvh = restored.scene.build_bvh();
        self.physics = Self::build_physics_from_params(
            restored.seed,
            restored.params.terrain,
            restored.params.environment,
        );
        if let Some(cap) = &mut self.physics.player_capsule {
            cap.foot_position = self.explorer.foot_position;
        }
        self.hd_sector = synthesize_hd_citadel_sector(
            restored.seed,
            &restored.params.terrain,
            restored.params.environment.water_level_meters,
            rank,
        );
        if let Ok(tl) = TwoLevelBvh::build_solid(&self.hd_sector.scene, &self.hd_sector.meshes) {
            self.two_level_bvh = tl;
        }
        self.world = restored;
        self.static_dirty = true;
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
        let rank = restored.active_lod.rank();
        self.lod_rank = rank as u32;
        self.seed_val = restored.seed;
        self.terrain_params = restored.params.terrain;
        self.env_params = restored.params.environment;
        self.bvh = restored.scene.build_bvh();
        self.physics = Self::build_physics_from_params(
            restored.seed,
            restored.params.terrain,
            restored.params.environment,
        );
        if let Some(cap) = &mut self.physics.player_capsule {
            cap.foot_position = self.explorer.foot_position;
        }
        self.hd_sector = synthesize_hd_citadel_sector(
            restored.seed,
            &restored.params.terrain,
            restored.params.environment.water_level_meters,
            rank,
        );
        if let Ok(tl) = TwoLevelBvh::build_solid(&self.hd_sector.scene, &self.hd_sector.meshes) {
            self.two_level_bvh = tl;
        }
        self.world = restored;
        self.static_dirty = true;
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
        let base_rank = u64::from(target_lod_rank).min(prism_kernel::wasmMaxLodRank());
        let restored = decode_portable_uorc_single_frame(stream_bytes, base_rank).map_err(|e| {
            WasmBridgeError::InvalidSnapshotEnvelope {
                byte_len: stream_bytes.len(),
                reason: bridge_reason(e),
            }
        })?;
        let snap = WorldSnapshot::encode(&restored).map_err(|e| {
            WasmBridgeError::InvalidSnapshotEnvelope {
                byte_len: stream_bytes.len(),
                reason: bridge_reason(e),
            }
        })?;
        let hd_rank = u64::from(target_lod_rank).min(pk_u64(prism_kernel::wasmHdMaxLodRank()));
        self.lod_rank = hd_rank as u32;
        self.seed_val = restored.seed;
        self.terrain_params = restored.params.terrain;
        self.env_params = restored.params.environment;
        self.bvh = restored.scene.build_bvh();
        self.physics = Self::build_physics_from_params(
            restored.seed,
            restored.params.terrain,
            restored.params.environment,
        );
        if let Some(cap) = &mut self.physics.player_capsule {
            cap.foot_position = self.explorer.foot_position;
        }
        self.hd_sector = synthesize_hd_citadel_sector(
            restored.seed,
            &restored.params.terrain,
            restored.params.environment.water_level_meters,
            hd_rank,
        );
        if let Ok(tl) = TwoLevelBvh::build_solid(&self.hd_sector.scene, &self.hd_sector.meshes) {
            self.two_level_bvh = tl;
        }
        self.world = restored;
        self.custom_glb_scene = None;
        self.static_dirty = true;
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
        let base_rank = u64::from(lod_rank).min(prism_kernel::wasmMaxLodRank());
        let bundle = CodedLodFrameBundle::synthesize(self.world.seed, base_rank).map_err(|e| {
            WasmBridgeError::InvalidSnapshotEnvelope {
                byte_len: 0,
                reason: bridge_reason(e),
            }
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

#[cfg(all(target_arch = "wasm32", not(feature = "uorc-decoder")))]
fn append_cold_boot_humanoid(
    buffers: &mut PackedSceneRenderBuffers,
    foot: Vec3,
    yaw: f64,
    suit_rgb: [f32; 1 + 1 + 1],
    visor_rgb: [f32; 1 + 1 + 1],
    hide_head: bool,
) {
    let two = prism_kernel::sTwo() as f64;
    let five = pk_f64(prism_kernel::sFive());
    let ten = two * five;
    let half = 1.0 / two;
    let fifth = 1.0 / five;
    let tenth = 1.0 / ten;
    let sy = yaw.sin();
    let cy = yaw.cos();
    let leg_off = tenth + tenth * half;
    for side in [-1.0_f64, 1.0_f64] {
        let lx = cy * (side * leg_off);
        let lz = -sy * (side * leg_off);
        buffers.append_oriented_box(
            Vec3::new(foot.x + lx, foot.y + fifth + fifth, foot.z + lz),
            Vec3::new(
                tenth * half + tenth / five,
                fifth + fifth,
                tenth * half + tenth / five,
            ),
            yaw,
            0.0,
            suit_rgb,
        );
    }
    buffers.append_oriented_box(
        Vec3::new(foot.x, foot.y + 1.0 + tenth, foot.z),
        Vec3::new(fifth, fifth + tenth, tenth + tenth * half),
        yaw,
        0.0,
        suit_rgb,
    );
    for side in [-1.0_f64, 1.0_f64] {
        let ax = cy * (side * (fifth + tenth));
        let az = -sy * (side * (fifth + tenth));
        buffers.append_oriented_box(
            Vec3::new(foot.x + ax, foot.y + 1.0 + tenth * half, foot.z + az),
            Vec3::new(tenth * half, fifth + tenth * half, tenth * half),
            yaw,
            0.0,
            suit_rgb,
        );
    }
    if !hide_head {
        buffers.append_oriented_box(
            Vec3::new(foot.x, foot.y + 1.0 + half + tenth * half, foot.z),
            Vec3::new(
                tenth + tenth * half,
                tenth + tenth * half,
                tenth + tenth * half,
            ),
            yaw,
            0.0,
            suit_rgb,
        );
        let vx = sy * (tenth + tenth * half);
        let vz = cy * (tenth + tenth * half);
        buffers.append_oriented_box(
            Vec3::new(foot.x + vx, foot.y + 1.0 + half + tenth * half, foot.z + vz),
            Vec3::new(tenth, tenth * half, tenth / five),
            yaw,
            0.0,
            visor_rgb,
        );
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
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
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
    hd_terrain_height(bridge.seed_val, &bridge.terrain_params, x, z) * amp
}

/// Returns `1` when the player hull is within contact of the central Citadel hall, else `0`.
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
    bridge.two_level_bvh.tlas().query_frustum(&frustum).len() as u32
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
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
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
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
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
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_fb_ptr() -> u32 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| Some(g.as_ref()?.framebuffer_rgba.as_ptr() as usize as u32))
        .unwrap_or(0)
}

/// Byte length of the latest `RGBA8` tensor framebuffer.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
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
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
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
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
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
        .and_then(|g| Some(g.as_ref()?.seed_val))
        .unwrap_or_else(|| pk_u64(prism_kernel::wasmDefaultSeed()))
}

/// Number of active universal scene node presets in the current LOD (`0..=7`).
#[no_mangle]
pub extern "C" fn verdant_wasm_node_preset_count() -> u32 {
    #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
    {
        global_bridge()
            .lock()
            .ok()
            .and_then(|g| Some(g.as_ref()?.world.scene_nodes.len() as u32))
            .unwrap_or(0)
    }
    #[cfg(all(target_arch = "wasm32", not(feature = "uorc-decoder")))]
    {
        pk_u64(prism_kernel::sSeven()) as u32
    }
}

/// World-space X coordinate in meters of scene node preset `index`.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
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
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
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
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
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
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
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

/// Returns maximum progressive base LOD rank (`3`) from `prism_kernel`.
#[no_mangle]
pub extern "C" fn verdant_wasm_max_lod_rank() -> u32 {
    prism_kernel::wasmMaxLodRank() as u32
}

/// Returns maximum progressive HD LOD rank (`4`) from `prism_kernel`.
#[no_mangle]
pub extern "C" fn verdant_wasm_hd_max_lod_rank() -> u32 {
    pk_u64(prism_kernel::wasmHdMaxLodRank()) as u32
}

/// Returns minimum authenticated `VDNT` wire envelope byte length (`41`) from `prism_kernel`.
#[no_mangle]
pub extern "C" fn verdant_wasm_snapshot_min_wire_bytes() -> u32 {
    pk_u64(prism_kernel::snapshotMinWireBytes()) as u32
}

/// Number of unified Prism semiring tensor kernels (`15`, covering linear, tropical max/min,
/// audio ring, coded stream, tropical UI layout, LIF/SSM observer, `coded_gemm` SLM,
/// tropical BLAS swept CCD, analytical Two-Bone IK, and conservation-verified story automata).
#[no_mangle]
pub extern "C" fn verdant_wasm_unified_tensor_kernel_count() -> u32 {
    #[cfg(target_arch = "wasm32")]
    {
        pk_u64(prism_kernel::sFifteen()) as u32
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
        let k12_ok = prism_kernel::kernel12TropicalBlasSweep()
            .map(|s| prism_kernel::tensorKernelValid(&s))
            .unwrap_or(false);
        let k13_ok = prism_kernel::kernel13TwoBoneIkContraction()
            .map(|s| prism_kernel::tensorKernelValid(&s))
            .unwrap_or(false);
        let k14_ok = prism_kernel::kernel14ConservationAutomaton()
            .map(|s| prism_kernel::tensorKernelValid(&s))
            .unwrap_or(false);
        if base_ok && k7_ok && k8_ok && k9_ok && k10_ok && k11_ok && k12_ok && k13_ok && k14_ok {
            pk_u64(prism_kernel::sFifteen()) as u32
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

/// Exports the active Rust `SceneWorld` (either user-imported `.glb` or HD Citadel sector) to binary
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
        let mats = canonical_pbr_material_library();
        export_scene_to_glb(&bridge.hd_sector.scene, &bridge.hd_sector.meshes, &mats)
    };
    match exported {
        Ok(bytes) => {
            bridge.aux_buffer = bytes;
            bridge.aux_buffer.len() as u32
        }
        Err(_) => 0,
    }
}

/// Clears any user-imported `.glb` scene override and restores the procedural `HdSectorBundle` meshes.
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
    bridge.static_dirty = true;
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

/// Advances lawful swept CCD physics, character Foot IK, floating-origin portals, and camera guard sphere.
#[no_mangle]
pub extern "C" fn verdant_wasm_step_lawful(
    dt_micros: u32,
    move_fwd_milli: i32,
    move_strafe_milli: i32,
    walk_yaw_milli: i32,
    speed_milli: u32,
    jump_flag: u32,
) -> u32 {
    let Ok(mut guard) = global_bridge().lock() else {
        return 0;
    };
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };
    if move_fwd_milli != 0 || move_strafe_milli != 0 || jump_flag != 0 {
        bridge.camera_anchor_override = None;
    }
    let micros_scale = pk_f64(prism_kernel::engineMaxFrameBudgetMicros());
    let milli_scale = pk_f64(prism_kernel::skinWeightScale());
    let dt_sec = f64::from(dt_micros) / micros_scale;
    let fwd = f64::from(move_fwd_milli) / milli_scale;
    let strafe = f64::from(move_strafe_milli) / milli_scale;
    let yaw = f64::from(walk_yaw_milli) / milli_scale;
    let speed = f64::from(speed_milli) / milli_scale;
    u32::from(
        bridge
            .step_frame_lawful(dt_sec, fwd, strafe, yaw, speed, jump_flag != 0)
            .is_ok(),
    )
}

/// Switches lawful camera possession mode (`0` = `ThirdPersonSpringArm`, `1` = `FirstPersonEye`).
#[no_mangle]
pub extern "C" fn verdant_wasm_camera_set_mode(mode_u32: u32) -> u32 {
    let Ok(mut guard) = global_bridge().lock() else {
        return 0;
    };
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };
    bridge.camera_anchor_override = None;
    bridge.camera_rig.mode = if mode_u32 == 1 {
        CameraMode::FirstPersonEye
    } else {
        CameraMode::ThirdPersonSpringArm
    };
    let _ = bridge.solve_active_camera(0.0);
    let _ = bridge.rebuild_scene_buffers(0.0);
    1
}

/// Returns active lawful camera mode (`0` = `ThirdPersonSpringArm`, `1` = `FirstPersonEye`).
#[no_mangle]
pub extern "C" fn verdant_wasm_camera_mode() -> u32 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| {
            Some(match g.as_ref()?.camera_rig.mode {
                CameraMode::ThirdPersonSpringArm => 0,
                CameraMode::FirstPersonEye => 1,
            })
        })
        .unwrap_or(0)
}

/// Sets or clears (`enable_override == 0`) a world-space landmark anchor for lawful camera orbiting.
#[no_mangle]
pub extern "C" fn verdant_wasm_camera_set_anchor(
    enable_override: u32,
    x: f64,
    y: f64,
    z: f64,
) -> u32 {
    let Ok(mut guard) = global_bridge().lock() else {
        return 0;
    };
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };
    if enable_override == 0 {
        bridge.camera_anchor_override = None;
    } else if x.is_finite() && y.is_finite() && z.is_finite() {
        bridge.camera_anchor_override = Some(Vec3::new(x, y, z));
        bridge.camera_rig.mode = CameraMode::ThirdPersonSpringArm;
    }
    u32::from(bridge.solve_active_camera(0.0).is_ok())
}

/// Applies incremental pointer/wheel look & zoom intent and solves the guarded camera frame inside Rust.
#[no_mangle]
pub extern "C" fn verdant_wasm_camera_intent(
    delta_yaw_milli: i32,
    delta_pitch_milli: i32,
    delta_dist_milli: i32,
    aspect_milli: u32,
    dt_micros: u32,
) -> u32 {
    let Ok(mut guard) = global_bridge().lock() else {
        return 0;
    };
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };
    let milli_scale = pk_f64(prism_kernel::skinWeightScale());
    let micros_scale = pk_f64(prism_kernel::engineMaxFrameBudgetMicros());
    if aspect_milli > 0 {
        let _ = bridge
            .camera_rig
            .set_aspect_ratio(f64::from(aspect_milli) / milli_scale);
    }
    bridge.camera_rig.apply_look_and_zoom_intent(
        f64::from(delta_yaw_milli) / milli_scale,
        f64::from(delta_pitch_milli) / milli_scale,
        f64::from(delta_dist_milli) / milli_scale,
    );
    let dt_sec = f64::from(dt_micros) / micros_scale;
    u32::from(bridge.solve_active_camera(dt_sec).is_ok())
}

/// Sets absolute camera orbit `(yaw, pitch, desired_distance)` intent and solves through `LawfulCameraRig`.
#[no_mangle]
pub extern "C" fn verdant_wasm_camera_set_pose_intent(
    yaw_milli: i32,
    pitch_milli: i32,
    dist_milli: u32,
    aspect_milli: u32,
) -> u32 {
    let Ok(mut guard) = global_bridge().lock() else {
        return 0;
    };
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };
    let milli_scale = pk_f64(prism_kernel::skinWeightScale());
    if aspect_milli > 0 {
        let _ = bridge
            .camera_rig
            .set_aspect_ratio(f64::from(aspect_milli) / milli_scale);
    }
    bridge.camera_rig.yaw_radians =
        (f64::from(yaw_milli) / milli_scale).rem_euclid(std::f64::consts::TAU);
    bridge.camera_rig.pitch_radians = (f64::from(pitch_milli) / milli_scale)
        .max(bridge.camera_rig.config.min_pitch_radians)
        .min(bridge.camera_rig.config.max_pitch_radians);
    let half = 1.0 / (prism_kernel::sTwo() as f64);
    let d = (f64::from(dist_milli) / milli_scale)
        .max(half)
        .min(bridge.camera_rig.config.max_orbit_distance_meters);
    bridge.camera_rig.desired_distance_meters = d;
    bridge.camera_rig.current_distance_meters = d;
    u32::from(bridge.solve_active_camera(0.0).is_ok())
}

/// Solved lawful camera eye X coordinate in world meters.
#[no_mangle]
pub extern "C" fn verdant_wasm_camera_eye_x() -> f64 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| Some(g.as_ref()?.last_camera_frame.eye.x))
        .unwrap_or(0.0)
}

/// Solved lawful camera eye Y coordinate in world meters.
#[no_mangle]
pub extern "C" fn verdant_wasm_camera_eye_y() -> f64 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| Some(g.as_ref()?.last_camera_frame.eye.y))
        .unwrap_or(1.0)
}

/// Solved lawful camera eye Z coordinate in world meters.
#[no_mangle]
pub extern "C" fn verdant_wasm_camera_eye_z() -> f64 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| Some(g.as_ref()?.last_camera_frame.eye.z))
        .unwrap_or(pk_f64(prism_kernel::sSixteen()))
}

/// Solved lawful camera look-target X coordinate in world meters.
#[no_mangle]
pub extern "C" fn verdant_wasm_camera_target_x() -> f64 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| Some(g.as_ref()?.last_camera_frame.target.x))
        .unwrap_or(0.0)
}

/// Solved lawful camera look-target Y coordinate in world meters.
#[no_mangle]
pub extern "C" fn verdant_wasm_camera_target_y() -> f64 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| Some(g.as_ref()?.last_camera_frame.target.y))
        .unwrap_or(1.0)
}

/// Solved lawful camera look-target Z coordinate in world meters.
#[no_mangle]
pub extern "C" fn verdant_wasm_camera_target_z() -> f64 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| Some(g.as_ref()?.last_camera_frame.target.z))
        .unwrap_or(0.0)
}

/// Active camera rig yaw angle in milli-radians.
#[no_mangle]
pub extern "C" fn verdant_wasm_camera_yaw_milli() -> i32 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| {
            let milli = pk_f64(prism_kernel::skinWeightScale());
            Some((g.as_ref()?.camera_rig.yaw_radians * milli).round() as i32)
        })
        .unwrap_or(0)
}

/// Returns `1` if the spring-arm was pulled forward by solid geometry on the latest frame, else `0`.
#[no_mangle]
pub extern "C" fn verdant_wasm_camera_obstructed() -> u32 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| Some(u32::from(g.as_ref()?.last_camera_frame.obstructed)))
        .unwrap_or(0)
}

/// Minimum solid surface clearance from the solved camera eye in millimeters.
#[no_mangle]
pub extern "C" fn verdant_wasm_camera_clearance_milli() -> u32 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| {
            let milli = pk_f64(prism_kernel::skinWeightScale());
            Some(
                (g.as_ref()?.last_camera_frame.clearance_meters * milli)
                    .max(0.0)
                    .round() as u32,
            )
        })
        .unwrap_or(0)
}

/// Active floating-origin `UniverseCellCoord` X index.
#[no_mangle]
pub extern "C" fn verdant_wasm_universe_cell_x() -> i64 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| Some(g.as_ref()?.universe.active_position.cell.cx))
        .unwrap_or(0)
}

/// Active floating-origin `UniverseCellCoord` Y index.
#[no_mangle]
pub extern "C" fn verdant_wasm_universe_cell_y() -> i64 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| Some(g.as_ref()?.universe.active_position.cell.cy))
        .unwrap_or(0)
}

/// Active floating-origin `UniverseCellCoord` Z index.
#[no_mangle]
pub extern "C" fn verdant_wasm_universe_cell_z() -> i64 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| Some(g.as_ref()?.universe.active_position.cell.cz))
        .unwrap_or(0)
}

/// Active Chronicle `StoryAutomaton` node index.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_story_state() -> u32 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| Some(g.as_ref()?.story.current_state as u32))
        .unwrap_or(0)
}

/// Total conserved item supply in the active Chronicle `ConservationLedger`.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_story_supply() -> u64 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| Some(g.as_ref()?.story.ledger.total_supply()))
        .unwrap_or(0)
}

/// Attempts to trigger the nearest spatial Chronicle transition around `(x, y, z)` (`1` = advanced, `0` = guard unmet).
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_story_interact_at(x: f64, y: f64, z: f64) -> u32 {
    let Ok(mut guard) = global_bridge().lock() else {
        return 0;
    };
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };
    u32::from(matches!(
        bridge.story.interact_nearest(Vec3::new(x, y, z)),
        Ok(Some(_))
    ))
}

/// Stages `"<state_idx>|<node_title>|<supply>|<objective>|<dialogue>"` into `aux_buffer` and returns byte length.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_story_status_aux() -> u32 {
    let Ok(mut guard) = global_bridge().lock() else {
        return 0;
    };
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };
    let idx = bridge.story.current_state;
    let supply = bridge.story.ledger.total_supply();
    let node = bridge.story.active_node();
    let formatted = format!(
        "{}|{}|{}|{}|{}",
        idx, node.title, supply, node.objective, bridge.story.last_dialogue
    );
    bridge.aux_buffer = formatted.into_bytes();
    bridge.aux_buffer.len() as u32
}
