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

#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
use verdant_core::animation::{skin_mesh_linear_blend, SkeletalClip, Skeleton, SkinWeights4};
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
use verdant_core::ecs::{BoundingVolumeHierarchy, SceneWorld, Transform};
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
use verdant_core::geometry::{generate_faceted_prism_mesh, ProceduralMesh};
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
use verdant_core::gltf_io::{
    export_scene_to_glb, import_glb_to_scene, import_glb_to_scene_bounded,
    pack_multi_primitive_glb, transcode_glb_to_progressive_uorc, DecodedRgbaTexture,
};
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
use verdant_core::hd_world::{
    generate_studio_primitive_mesh, synthesize_hd_world_sector_for_kind, AuthorPrimitiveKind,
    WorldArchitectureKind,
};
use verdant_core::hd_world::{hd_terrain_height, synthesize_hd_citadel_sector, HdSectorBundle};
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
use verdant_core::math::Mat4;
use verdant_core::math::{Frustum, Vec3};
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
use verdant_core::narrative::{canonical_citadel_chronicle_automaton, StoryAutomaton};
use verdant_core::navmesh::NavMesh;
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
use verdant_core::network::{
    recover_uorc_from_stitch_pair, shard_uorc_to_stitch_fragments, PrismIdSessionEnvelope,
    StitchFragment,
};
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
use verdant_core::observer::{
    sha256_payload_digest, CommunicationChannelKind, ObserverPartSlot, ObserverPrismHolon,
};
use verdant_core::physics::{
    CapsuleController, CharacterMoveIntent, KinematicSplineBody, PhysicsWorld, PhysicsWorldConfig,
};
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
use verdant_core::physics::{JointKind, RigidBody, RigidJointConstraint};
use verdant_core::prism_kernel;
use verdant_core::render::{
    canonical_pbr_material_library, pack_world_meshes_to_buffers_opts, GpuSurfaceClass,
    PackedSceneRenderBuffers,
};
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
use verdant_core::render::{InstancedBatchBuffer, PbrMaterial};
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
use verdant_core::runtime::{
    canonicalize_command_monoid, BrowserCapabilities, CanonicalCommandKind, CanonicalCommandOp,
    CheckpointedRollbackJournal, EngineConfig, KappaTransitionCache, OntologicalRuntimeGovernor,
    WorldSnapshot,
};
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
use verdant_core::streaming::terrain_height_with_params;
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
use verdant_core::streaming::{
    decode_portable_uorc_single_frame, is_uorc_archive, CodedLodFrameBundle, StandardSceneState,
};
#[cfg(not(target_arch = "wasm32"))]
use verdant_core::streaming::{decode_uorc_stream_up_to_lod, pack_progressive_uorc_stream};
use verdant_core::streaming::{PhysicalEnvironment, TerrainReliefParams};
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
use verdant_core::tensor::{
    contract_packed_nibbles_lut_into, contract_paged_packed_nibbles_stream,
    render_triangles_tile_framebuffer_with_scratch, CodebookProductLut, PagedTensorArena,
    TensorPageKind, TensorStorageEncoding, TileRasterScratch, CODED_TABLE_LEN,
    MIN_ARENA_BUDGET_BYTES, PACKED_NIBBLE_BLOCK, TENSOR_PAGE_QUANTUM_BYTES,
};
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
use verdant_core::uhd::{
    build_meshlet_graph, evaluate_lod_pyramid_telemetry, evaluate_meshlet_visibility,
    evaluate_uhd_display_profile, DynamicWorkloadGovernor, FrameBudgetOracle, MeshletGraph,
    UhdVisibilityTelemetry,
};
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
use verdant_core::ui::UiTree;
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
use verdant_core::universe::UniverseManager;
use verdant_core::universe::{UniverseCellCoord, UniversePosition};

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
                &|x, z| hd_terrain_height(clean_seed, &terrain_params, x, z),
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
                    &|x, z| hd_terrain_height(clean_seed, &terrain_params, x, z),
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

        #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
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
            #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
            io_buffer: Vec::with_capacity(
                pk_u64(prism_kernel::wasmIoInitialCapacityBytes()) as usize
            ),
            #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
            aux_buffer: Vec::new(),
            #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
            framebuffer_rgba: Vec::new(),
            #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
            tile_scratch: TileRasterScratch::new(),
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
            #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
            universe,
            #[cfg(all(target_arch = "wasm32", not(feature = "uorc-decoder")))]
            active_cell: UniverseCellCoord::ORIGIN,
            #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
            story,
            static_pos_len: 0,
            static_idx_len: 0,
            static_tri_len: 0,
            static_dirty: true,
            static_epoch: 0,
            dynamic_buffers: PackedSceneRenderBuffers::default(),
            #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
            instanced_batch: InstancedBatchBuffer::default(),
            #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
            last_npc_ik_player_pos: Vec3::new(f64::INFINITY, 0.0, 0.0),
            #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
            slm_director: CodedSlmDirector::new(),
            #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
            grants: ContainerGrantMatrix::default(),
            #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
            custom_glb_scene: None,
            #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
            blank_studio_mode: false,
            #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
            studio_objects: Vec::new(),
            #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
            next_studio_obj_id: 1,
            #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
            custom_glb_uvs_xy: Vec::new(),
            #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
            custom_glb_tex_rgba: Vec::new(),
            #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
            custom_glb_tex_width: 0,
            #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
            custom_glb_tex_height: 0,
            #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
            custom_glb_tex_epoch: 0,
            #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
            custom_glb_primary_tex_mask: Vec::new(),
            #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
            custom_glb_mr_tex: None,
            #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
            custom_glb_normal_tex: None,
            #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
            custom_glb_emissive_tex: None,
            #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
            custom_glb_rig: None,
            #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
            custom_glb_mesh_atlas_uvs: Vec::new(),
            #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
            custom_glb_atlas_tile_count: 0,
            #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
            uhd_meshlet_graph: MeshletGraph::default(),
            #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
            uhd_meshlet_epoch: 0,
            #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
            uhd_visible_indices: Vec::new(),
            #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
            uhd_last_visibility: None,
            #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
            uhd_frame_oracle: FrameBudgetOracle::default(),
            #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
            workload_governor: DynamicWorkloadGovernor::default(),
            #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
            tensor_arena: PagedTensorArena::default(),
            #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
            imported_glb_skin_clip_counts: (0, 0),
            #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
            observer_holon: ObserverPrismHolon::new("sunspire-regatta").map_err(|e| {
                WasmBridgeError::InvalidSnapshotEnvelope {
                    byte_len: 0,
                    reason: bridge_reason(e),
                }
            })?,
            #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
            npc_patrol_active: false,
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
        let ten = pk_f64(prism_kernel::sTen());
        let mut physics = PhysicsWorld::new(cfg);

        #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
        {
            let two = prism_kernel::sTwo() as f64;
            let three = prism_kernel::sThree() as f64;
            let four = pk_f64(prism_kernel::sFour());
            let five = pk_f64(prism_kernel::sFive());
            let nine = pk_f64(prism_kernel::sNine());
            let twelve = pk_f64(prism_kernel::sTwelve());
            let hundred = pk_f64(prism_kernel::sHundred());

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

    /// Rebuilds dynamic actor buffers without repacking static world geometry when `!self.static_dirty`.
    pub fn rebuild_dynamic_buffers_only(
        &mut self,
        sim_time_sec: f64,
    ) -> Result<usize, WasmBridgeError> {
        self.rebuild_scene_buffers(sim_time_sec)
    }

    /// Rebuilds `self.scene_buffers` from the cached static `HdSectorBundle` (`VaultedCitadelHall`,
    /// `ParabolicStoneArchBridge`, sculpted terrain, canopy trees) plus dynamic skinned characters
    /// (`Explorer` + 3 `NPC`s), Observer Core gimbal, XPBD hinge rig, 4-wheel vehicle, and diegetic card.
    pub fn rebuild_scene_buffers(&mut self, sim_time_sec: f64) -> Result<usize, WasmBridgeError> {
        #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
        if let Some((ref glb_scene, ref glb_meshes, ref glb_mats)) = self.custom_glb_scene {
            let has_anim = self
                .custom_glb_rig
                .as_ref()
                .is_some_and(|(skels, clips, weights)| {
                    !skels.is_empty()
                        && !clips.is_empty()
                        && clips[0].duration_millis > 0
                        && weights.iter().any(|w| !w.is_empty())
                });
            if self.static_dirty || self.static_idx_len == 0 || (has_anim && sim_time_sec > 0.0) {
                let animated_meshes: Option<Vec<ProceduralMesh>> = if has_anim && sim_time_sec > 0.0
                {
                    if let Some((ref skels, ref clips, ref skin_weights)) = self.custom_glb_rig {
                        let skel = &skels[0];
                        let clip = &clips[0];
                        let ms_scale = (1u64 << (1 + 1 + 1))
                            * (1 + 1 + 1 + 1 + 1)
                            * (1 + 1 + 1 + 1 + 1)
                            * (1 + 1 + 1 + 1 + 1);
                        let elapsed_ms = ((sim_time_sec.max(0.0) * (ms_scale as f64)).round()
                            as u64)
                            % clip.duration_millis.max(1);
                        if let Ok(pose) = clip.sample(skel, elapsed_ms) {
                            let mut out_m = Vec::with_capacity(glb_meshes.len());
                            for (mi, mesh) in glb_meshes.iter().enumerate() {
                                if let Some(w) = skin_weights.get(mi) {
                                    if w.len() == mesh.positions.len() {
                                        if let Ok(deformed) =
                                            skin_mesh_linear_blend(mesh, skel, &pose, w)
                                        {
                                            out_m.push(deformed);
                                            continue;
                                        }
                                    }
                                }
                                out_m.push(mesh.clone());
                            }
                            Some(out_m)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                };
                let meshes_ref = animated_meshes.as_deref().unwrap_or(glb_meshes);
                let packed =
                    pack_world_meshes_to_buffers_opts(glb_scene, meshes_ref, glb_mats, false)
                        .map_err(|e| WasmBridgeError::InvalidSnapshotEnvelope {
                            byte_len: 0,
                            reason: bridge_reason(e),
                        })?;
                self.static_pos_len = packed.positions_xyz.len();
                self.static_idx_len = packed.indices.len();
                self.static_tri_len = 0;
                self.scene_buffers = packed;
                self.custom_glb_uvs_xy.clear();
                let uv_sentinel = -(1.0_f32 + 1.0_f32);
                let has_tex = (!self.custom_glb_tex_rgba.is_empty()
                    && self.custom_glb_tex_width > 0
                    && self.custom_glb_tex_height > 0)
                    || self.custom_glb_atlas_tile_count > 0;
                for entity in glb_scene.entities() {
                    let Some(mesh_idx) = entity.mesh_index else {
                        continue;
                    };
                    let Some(mesh) = meshes_ref.get(mesh_idx) else {
                        continue;
                    };
                    let atlas_uvs = self.custom_glb_mesh_atlas_uvs.get(mesh_idx);
                    let uses_tex = has_tex
                        && self
                            .custom_glb_primary_tex_mask
                            .get(mesh_idx)
                            .copied()
                            .unwrap_or(false);
                    for vi in 0..mesh.positions.len() {
                        if uses_tex {
                            let uv = atlas_uvs
                                .and_then(|v| v.get(vi).copied())
                                .or_else(|| mesh.uvs.get(vi).copied())
                                .unwrap_or([0.0, 0.0]);
                            self.custom_glb_uvs_xy.push(uv[0]);
                            self.custom_glb_uvs_xy.push(uv[1]);
                        } else {
                            self.custom_glb_uvs_xy.push(uv_sentinel);
                            self.custom_glb_uvs_xy.push(uv_sentinel);
                        }
                    }
                }
                self.uhd_meshlet_graph = build_meshlet_graph(
                    &self.scene_buffers.positions_xyz,
                    &self.scene_buffers.indices,
                );
                self.dynamic_buffers.positions_xyz.clear();
                self.dynamic_buffers.normals_xyz.clear();
                self.dynamic_buffers.colors_rgb.clear();
                self.dynamic_buffers.indices.clear();
                self.dynamic_buffers.tile_triangles.clear();
                self.static_epoch = self.static_epoch.wrapping_add(1).max(1);
                self.uhd_meshlet_epoch = self.static_epoch;
                self.static_dirty = false;
            }
            return Ok(self.scene_buffers.triangle_count());
        }

        if self.static_dirty || self.static_idx_len == 0 {
            #[allow(unused_mut)]
            let mut materials = canonical_pbr_material_library();
            #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
            if self.npc_patrol_active {
                materials = self
                    .observer_holon
                    .active_domain
                    .hd_materials
                    .iter()
                    .map(verdant_core::render::HdPbrDescriptor::to_pbr_material)
                    .collect();
            }
            #[allow(unused_mut)]
            let mut base = pack_world_meshes_to_buffers_opts(
                &self.hd_sector.scene,
                &self.hd_sector.meshes,
                &materials,
                false,
            )
            .map_err(|e| WasmBridgeError::InvalidSnapshotEnvelope {
                byte_len: 0,
                reason: bridge_reason(e),
            })?;
            #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
            if !self.studio_objects.is_empty() {
                let idx_two = 1usize + 1usize;
                let milli_denom = (1u64 << (1 + 1 + 1))
                    * (1 + 1 + 1 + 1 + 1)
                    * (1 + 1 + 1 + 1 + 1)
                    * (1 + 1 + 1 + 1 + 1);
                let milli_f = milli_denom as f64;
                for obj in &self.studio_objects {
                    let mesh = generate_studio_primitive_mesh(
                        obj.kind,
                        f64::from(obj.scale[0]),
                        f64::from(obj.scale[1]),
                        f64::from(obj.scale[idx_two]),
                        [obj.color[0], obj.color[1], obj.color[idx_two], 1.0],
                    );
                    let yaw_rad = f64::from(obj.yaw_milli) / milli_f;
                    let (sy, cy) = yaw_rad.sin_cos();
                    let sy_f = sy as f32;
                    let cy_f = cy as f32;
                    let base_v = (base.positions_xyz.len() / (1 + 1 + 1)) as u32;
                    for (vi, p) in mesh.positions.iter().enumerate() {
                        let rx = p[0] * cy_f + p[idx_two] * sy_f;
                        let rz = -p[0] * sy_f + p[idx_two] * cy_f;
                        base.positions_xyz.push(rx + obj.pos[0]);
                        base.positions_xyz.push(p[1] + obj.pos[1]);
                        base.positions_xyz.push(rz + obj.pos[idx_two]);
                        let n = mesh.normals.get(vi).copied().unwrap_or([0.0, 1.0, 0.0]);
                        let rnx = n[0] * cy_f + n[idx_two] * sy_f;
                        let rnz = -n[0] * sy_f + n[idx_two] * cy_f;
                        base.normals_xyz.push(rnx);
                        base.normals_xyz.push(n[1]);
                        base.normals_xyz.push(rnz);
                        let c = mesh.colors.get(vi).copied().unwrap_or([
                            obj.color[0],
                            obj.color[1],
                            obj.color[idx_two],
                            1.0,
                        ]);
                        base.colors_rgb.push(c[0]);
                        base.colors_rgb.push(c[1]);
                        base.colors_rgb.push(c[idx_two]);
                    }
                    for &ix in &mesh.indices {
                        base.indices.push(base_v + ix);
                    }
                }
            }
            self.static_pos_len = base.positions_xyz.len();
            self.static_idx_len = base.indices.len();
            self.static_tri_len = 0;
            self.scene_buffers = base;
            self.static_epoch = self.static_epoch.wrapping_add(1).max(1);
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

        self.dynamic_buffers.positions_xyz.clear();
        self.dynamic_buffers.normals_xyz.clear();
        self.dynamic_buffers.colors_rgb.clear();
        self.dynamic_buffers.indices.clear();
        self.dynamic_buffers.tile_triangles.clear();

        #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
        if self.blank_studio_mode {
            return Ok(self.scene_buffers.triangle_count());
        }

        let kc = verdant_core::verdant_tensor::kernel_consts();
        let two = (1 + 1) as f64;
        let three = (1 + 1 + 1) as f64;
        let six = two * three;
        let eight = two * two * two;
        let ten = two * (two + three);
        let sixteen = two * eight;
        let half = 1.0 / two;
        let quarter = half * half;
        let i2_u32 = (1 + 1) as u32;

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
            let milli = kc.skin_weight_scale_f64;
            let obs_yaw = self
                .last_directive
                .as_ref()
                .map_or(sim_time_sec * half, |d| {
                    (d.observer_gimbal_yaw_milli as f64) / milli
                });
            let spiking = self
                .last_directive
                .as_ref()
                .is_some_and(|d| d.spike_mask != 0);
            let (obs_core_col, obs_core_class) = if spiking {
                (cyan, GpuSurfaceClass::EmissiveGlass)
            } else {
                (gold, GpuSurfaceClass::Metal)
            };
            let core_center = Vec3::new(0.0, y0 + three + quarter, 0.0);
            self.dynamic_buffers.surface_class = GpuSurfaceClass::Metal;
            self.dynamic_buffers.append_oriented_box_gpu_only(
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
            self.dynamic_buffers.surface_class = obs_core_class;
            self.dynamic_buffers.append_oriented_box_gpu_only(
                core_center,
                Vec3::new(quarter, quarter, quarter),
                -obs_yaw,
                sim_time_sec * half,
                obs_core_col,
            );
        }

        if self.lod_rank >= i2_u32 {
            // Live XPBD Articulated Hinge Rig (`bodies[1]` anchor & `bodies[2]` pendulum bob):
            let i2_idx = 1usize + 1usize;
            if let (Some(anchor), Some(bob)) =
                (self.physics.bodies.get(1), self.physics.bodies.get(i2_idx))
            {
                self.dynamic_buffers.surface_class = GpuSurfaceClass::OpaqueDielectric;
                self.dynamic_buffers.append_oriented_box_gpu_only(
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
                self.dynamic_buffers.surface_class = GpuSurfaceClass::OpaqueDielectric;
                self.dynamic_buffers.append_oriented_box_gpu_only(
                    mid,
                    Vec3::new(quarter * quarter, link_len, quarter * quarter),
                    0.0,
                    swing_angle,
                    stone,
                );
                self.dynamic_buffers.surface_class = GpuSurfaceClass::EmissiveGlass;
                self.dynamic_buffers.append_oriented_box_gpu_only(
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
            self.dynamic_buffers.surface_class = GpuSurfaceClass::Metal;
            self.dynamic_buffers.append_oriented_box_gpu_only(
                Vec3::new(vx, vy, vz),
                Vec3::new(half, quarter * half, 1.0 - quarter * half),
                vyaw,
                0.0,
                gold,
            );
            self.dynamic_buffers.surface_class = GpuSurfaceClass::EmissiveGlass;
            self.dynamic_buffers.append_oriented_box_gpu_only(
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
                self.dynamic_buffers.surface_class = GpuSurfaceClass::OpaqueDielectric;
                self.dynamic_buffers.append_oriented_box_gpu_only(
                    Vec3::new(px, gy, pz),
                    Vec3::new(quarter * half, quarter, quarter),
                    vyaw,
                    0.0,
                    slate,
                );
            }

            // Diegetic 3D UI Compliance Card anchored on the Citadel south terrace:
            let card_y = hd_terrain_height(self.seed_val, &self.terrain_params, six, eight) + two;
            self.dynamic_buffers.surface_class = GpuSurfaceClass::EmissiveGlass;
            self.dynamic_buffers.append_oriented_box_gpu_only(
                Vec3::new(six, card_y, eight),
                Vec3::new(1.0 + quarter, half + quarter, quarter * quarter),
                -quarter,
                0.0,
                cyan,
            );
        }

        self.dynamic_buffers.surface_class = GpuSurfaceClass::OpaqueDielectric;

        let hide_head = self.camera_rig.mode == CameraMode::FirstPersonEye
            && self.camera_anchor_override.is_none();

        #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
        {
            let _ = self.explorer.append_skinned_to_buffers_opts(
                &mut self.dynamic_buffers,
                hide_head,
                false,
            );
            if self.lod_rank >= 1 {
                for npc in &self.npcs {
                    let _ =
                        npc.append_skinned_to_buffers_opts(&mut self.dynamic_buffers, false, false);
                }
            }
        }

        #[cfg(all(target_arch = "wasm32", not(feature = "uorc-decoder")))]
        {
            append_cold_boot_humanoid(
                &mut self.dynamic_buffers,
                self.explorer_foot,
                self.explorer_yaw,
                stone,
                cyan,
                hide_head,
            );
            if self.lod_rank >= 1 {
                let five = two + three;
                let podium_y = y0 + three / five;
                append_cold_boot_humanoid(
                    &mut self.dynamic_buffers,
                    Vec3::new(1.0 + half, podium_y, -1.0),
                    0.0,
                    gold,
                    cyan,
                    false,
                );
                let four = two * two;
                let bx = self.terrain_params.basin_center_x - four - two;
                let bz = self.terrain_params.basin_center_z;
                let by = hd_terrain_height(self.seed_val, &self.terrain_params, bx, bz);
                append_cold_boot_humanoid(
                    &mut self.dynamic_buffers,
                    Vec3::new(bx, by, bz),
                    half,
                    slate,
                    cyan,
                    false,
                );
            }
        }

        // Mirror `dynamic_buffers` into the tail of `scene_buffers` with offset indices so both
        // split-VAO (`STATIC_DRAW` + `STREAM_DRAW`) and unified-buffer readers remain 100% consistent:
        let base_static_verts = (self.static_pos_len / (1 + 1 + 1)) as u32;
        self.scene_buffers
            .positions_xyz
            .extend_from_slice(&self.dynamic_buffers.positions_xyz);
        self.scene_buffers
            .normals_xyz
            .extend_from_slice(&self.dynamic_buffers.normals_xyz);
        self.scene_buffers
            .colors_rgb
            .extend_from_slice(&self.dynamic_buffers.colors_rgb);
        self.scene_buffers
            .indices
            .reserve(self.dynamic_buffers.indices.len());
        for &idx in &self.dynamic_buffers.indices {
            self.scene_buffers.indices.push(base_static_verts + idx);
        }

        #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
        let inst_tris = self.instanced_batch.effective_triangle_count();
        #[cfg(all(target_arch = "wasm32", not(feature = "uorc-decoder")))]
        let inst_tris = 0;

        Ok(self.scene_buffers.triangle_count() + inst_tris)
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
    let two = (1 + 1) as f64;
    let five = two + two + 1.0;
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
        buffers.append_oriented_box_gpu_only(
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
    buffers.append_oriented_box_gpu_only(
        Vec3::new(foot.x, foot.y + 1.0 + tenth, foot.z),
        Vec3::new(fifth, fifth + tenth, tenth + tenth * half),
        yaw,
        0.0,
        suit_rgb,
    );
    for side in [-1.0_f64, 1.0_f64] {
        let ax = cy * (side * (fifth + tenth));
        let az = -sy * (side * (fifth + tenth));
        buffers.append_oriented_box_gpu_only(
            Vec3::new(foot.x + ax, foot.y + 1.0 + tenth * half, foot.z + az),
            Vec3::new(tenth * half, fifth + tenth * half, tenth * half),
            yaw,
            0.0,
            suit_rgb,
        );
    }
    if !hide_head {
        buffers.append_oriented_box_gpu_only(
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
        buffers.surface_class = GpuSurfaceClass::EmissiveGlass;
        buffers.append_oriented_box_gpu_only(
            Vec3::new(foot.x + vx, foot.y + 1.0 + half + tenth * half, foot.z + vz),
            Vec3::new(tenth, tenth * half, tenth / five),
            yaw,
            0.0,
            visor_rgb,
        );
        buffers.surface_class = GpuSurfaceClass::OpaqueDielectric;
    }
}

fn global_bridge() -> &'static Mutex<Option<WasmRuntimeBridge>> {
    static STATE: OnceLock<Mutex<Option<WasmRuntimeBridge>>> = OnceLock::new();
    STATE.get_or_init(|| Mutex::new(None))
}

mod bridge_sim;
pub use bridge_sim::*;
mod exports_cold;
mod exports_cold_stage;
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
mod exports_uorc;
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
mod exports_uorc_studio;

pub use exports_cold::*;
pub use exports_cold_stage::*;
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
pub use exports_uorc::*;
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
pub use exports_uorc_studio::*;
