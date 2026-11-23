//! WasmRuntimeBridge simulation, instancing, camera, snapshot & UORC codec methods (`bridge_sim.rs`).

use super::*;

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

/// Retained glTF skeletal rigs, animation clips, and per-mesh LBS weights for live playback.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
pub type ImportedGlbRigBundle = (Vec<Skeleton>, Vec<SkeletalClip>, Vec<Vec<SkinWeights4>>);

/// One interactive 3D object authored by a creator inside Verdant Studio.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[derive(Clone, Debug)]
pub struct StudioAuthoredObject {
    /// Monotonic object identifier.
    pub id: u32,
    /// Primitive geometry shape.
    pub kind: AuthorPrimitiveKind,
    /// World position `(x, y, z)` in meters.
    pub pos: [f32; 1 + 1 + 1],
    /// Yaw rotation in milliradians.
    pub yaw_milli: i32,
    /// Non-uniform scale `(sx, sy, sz)`.
    pub scale: [f32; 1 + 1 + 1],
    /// Material-encoded RGB color (`r` integer part selects matte `0`, metallic `2`, emissive `3`).
    pub color: [f32; 1 + 1 + 1],
}

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
    #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
    pub io_buffer: Vec<u8>,
    /// Auxiliary output buffer for non-destructive Stitch shard, recovery, and UI reports.
    #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
    pub aux_buffer: Vec<u8>,
    /// Portable `16x16` tile-shaded `RGBA8` framebuffer rendered by `uor-matmul` tensor kernels.
    #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
    pub framebuffer_rgba: Vec<u8>,
    /// Reusable CSR software tile-binning arena (`0` steady-state inner `Vec` allocations).
    #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
    pub tile_scratch: TileRasterScratch,
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
    #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
    pub universe: UniverseManager,
    /// Active floating-origin universe cell coordinate on lean cold-boot `wasm32`.
    #[cfg(all(target_arch = "wasm32", not(feature = "uorc-decoder")))]
    pub active_cell: UniverseCellCoord,
    /// Conservation-verified Chronicle automaton (`kernel14ConservationAutomaton`).
    #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
    pub story: StoryAutomaton,
    /// Cached static `positions_xyz` float count for sub-millisecond dynamic buffer appends.
    pub(super) static_pos_len: usize,
    /// Cached static `indices` count for sub-millisecond dynamic buffer appends.
    pub(super) static_idx_len: usize,
    /// Cached static `tile_triangles` count for sub-millisecond dynamic buffer appends.
    pub(super) static_tri_len: usize,
    /// Whether static HD sector render buffers must be repacked before appending dynamic actors.
    pub(super) static_dirty: bool,
    /// Monotonic epoch incremented only when static world geometry changes (`STATIC_DRAW` guard).
    pub static_epoch: u32,
    /// Zero-based dynamic actor vertex/index buffer (`< 8 KiB/frame` `STREAM_DRAW` upload).
    pub dynamic_buffers: PackedSceneRenderBuffers,
    /// Hardware-instanced batch buffer (`gl.drawElementsInstanced`) for arbitrary crowd/lattice workloads.
    #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
    pub instanced_batch: InstancedBatchBuffer,
    /// Last Explorer position used to elide distant stationary NPC Foot IK solves.
    #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
    pub(super) last_npc_ik_player_pos: Vec3,
    /// Embedded zero-decode `uor_matmul::codec` language and intent router.
    #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
    pub slm_director: CodedSlmDirector,
    /// Denial-wins capability grant matrix over browser container APIs (`FK-07`).
    #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
    pub grants: ContainerGrantMatrix,
    /// Optional user-imported `.glb` scene overriding the procedural world meshes.
    #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
    pub custom_glb_scene: Option<(SceneWorld, Vec<ProceduralMesh>, Vec<PbrMaterial>)>,
    /// True when Verdant Studio is open on a clean Blank Studio Canvas (hides demo props).
    #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
    pub blank_studio_mode: bool,
    /// Creator-authored 3D scene objects placed in Verdant Studio.
    #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
    pub studio_objects: Vec<StudioAuthoredObject>,
    /// Next monotonic identifier for newly placed Studio objects.
    #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
    pub next_studio_obj_id: u32,
    /// Per-vertex packed UV coordinates (`2 * vertex_count` floats; `[-2.0, -2.0]` = untextured fallback).
    #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
    pub custom_glb_uvs_xy: Vec<f32>,
    /// Host-bounded primary `baseColorTexture` RGBA8 pixels for native WebGL2 texture upload.
    #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
    pub custom_glb_tex_rgba: Vec<u8>,
    /// Primary GPU albedo texture width in pixels (`0` if untextured).
    #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
    pub custom_glb_tex_width: u32,
    /// Primary GPU albedo texture height in pixels (`0` if untextured).
    #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
    pub custom_glb_tex_height: u32,
    /// Monotonic epoch bumped whenever `custom_glb_tex_rgba` changes.
    #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
    pub custom_glb_tex_epoch: u32,
    /// Per-mesh flag indicating whether the mesh samples `custom_glb_tex_rgba`.
    #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
    pub custom_glb_primary_tex_mask: Vec<bool>,
    /// Optional host-bounded `metallicRoughnessTexture` (`G=roughness, B=metallic`) from active `.glb`.
    #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
    pub custom_glb_mr_tex: Option<DecodedRgbaTexture>,
    /// Optional host-bounded tangent-space `normalTexture` from active `.glb`.
    #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
    pub custom_glb_normal_tex: Option<DecodedRgbaTexture>,
    /// Optional host-bounded `emissiveTexture` from active `.glb`.
    #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
    pub custom_glb_emissive_tex: Option<DecodedRgbaTexture>,
    /// Optional imported glTF skeletal rigs, keyframe clips, and per-mesh LBS weights for live playback.
    #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
    pub custom_glb_rig: Option<ImportedGlbRigBundle>,
    /// Per-mesh atlas-remapped UVs (`[-2.0, -2.0]` sentinel for untextured primitives).
    #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
    pub custom_glb_mesh_atlas_uvs: Vec<Vec<[f32; 1 + 1]>>,
    /// Number of distinct material textures packed into the active `.glb` PBR atlas (`1..=16`).
    #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
    pub custom_glb_atlas_tile_count: usize,
    /// Deterministic 64-triangle Morton-sorted GPU meshlet partition covering the active static scene.
    #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
    pub uhd_meshlet_graph: MeshletGraph,
    /// Static epoch for which `uhd_meshlet_graph` was compiled (prevents redundant rebuilds).
    #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
    pub uhd_meshlet_epoch: u32,
    /// Scratch buffer holding surviving indices after per-frame UHD frustum/cone/SSE culling.
    #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
    pub uhd_visible_indices: Vec<u32>,
    /// Most recent UHD meshlet visibility & culling telemetry.
    #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
    pub uhd_last_visibility: Option<UhdVisibilityTelemetry>,
    /// Zero-allocation 64-frame P50/P95/P99 stability oracle & hysteresis DRS governor.
    #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
    pub uhd_frame_oracle: FrameBudgetOracle,
    /// Closed-loop ISO/IEC 25010:2023 + ITU-R BT.2020/2100 dynamic workload scaling governor.
    #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
    pub workload_governor: DynamicWorkloadGovernor,
    /// Strictly host-bounded virtualized tensor memory arena (`resident_bytes <= budget_bytes`).
    #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
    pub tensor_arena: PagedTensorArena,
    /// Retained skin and animation clip counts from the active user-imported `.glb` bundle.
    #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
    pub imported_glb_skin_clip_counts: (usize, usize),
    /// Second-order Observer Prism holon hosting organs, stakeholders, catalogs, NPC patrols, and grounded language.
    #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
    pub observer_holon: ObserverPrismHolon,
    /// Whether live A* `NavMesh` NPC corridor locomotion is active.
    #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
    pub npc_patrol_active: bool,
}

impl WasmRuntimeBridge {
    /// Populates `self.instanced_batch` with `instance_count` procedural octahedral crystal/lattice
    /// monuments arranged across the Conservatory basin via `SoaSceneTable` + `WordBitset` (`0` clears).
    #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
    pub fn configure_instanced_workload(&mut self, instance_count: u32) -> usize {
        if instance_count == 0 {
            self.instanced_batch.clear_instances();
            return 0;
        }
        let max_inst = (1_u32
            << ((1_u32 + 1_u32 + 1_u32 + 1_u32) * (1_u32 + 1_u32 + 1_u32 + 1_u32)))
            .saturating_sub(1);
        let count = instance_count.min(max_inst) as usize;
        if self.instanced_batch.mesh_indices.is_empty() {
            let half = 1.0_f32 / ((1 + 1) as f32);
            let q = half * half;
            let positions = [
                [0.0_f32, 1.0, 0.0],
                [q, half, 0.0],
                [0.0, half, q],
                [-q, half, 0.0],
                [0.0, half, -q],
                [0.0, 0.0, 0.0],
            ];
            let normals = [
                [0.0_f32, 1.0, 0.0],
                [1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0],
                [-1.0, 0.0, 0.0],
                [0.0, 0.0, -1.0],
                [0.0, -1.0, 0.0],
            ];
            let mut mp = Vec::with_capacity(positions.len() * (1 + 1 + 1));
            let mut mn = Vec::with_capacity(normals.len() * (1 + 1 + 1));
            let mut mc = Vec::with_capacity(positions.len() * (1 + 1 + 1));
            let crystal_rgb = GpuSurfaceClass::EmissiveGlass.encode([1.0_f32, 1.0, 1.0]);
            for (p, n) in positions.iter().zip(normals.iter()) {
                mp.extend_from_slice(p);
                mn.extend_from_slice(n);
                mc.extend_from_slice(&crystal_rgb);
            }
            let two = (1 + 1) as u32;
            let three = two + 1;
            let four = two * two;
            let five = four + 1;
            self.instanced_batch.mesh_positions_xyz = mp;
            self.instanced_batch.mesh_normals_xyz = mn;
            self.instanced_batch.mesh_colors_rgb = mc;
            self.instanced_batch.mesh_indices = vec![
                0, 1, two, 0, two, three, 0, three, four, 0, four, 1, five, two, 1, five, three,
                two, five, four, three, five, 1, four,
            ];
        }

        self.instanced_batch.instance_rows_f32.clear();
        self.instanced_batch
            .instance_rows_f32
            .reserve(count * ((1 + 1 + 1 + 1) * (1 + 1 + 1 + 1)));

        let two = (1 + 1) as f64;
        let three = two + 1.0;
        let five = two + three;
        let eight = two * two * two;
        let half = 1.0 / two;
        let golden_angle = std::f64::consts::TAU * (1.0 - two / (1.0 + five.sqrt()));

        for i in 0..count {
            let fi = (i + 1) as f64;
            let r = eight + (fi / (count.max(1) as f64)).sqrt() * (eight * three);
            let theta = fi * golden_angle;
            let x = r * theta.cos();
            let z = r * theta.sin();
            let y = hd_terrain_height(self.seed_val, &self.terrain_params, x, z);
            let s = (half + (theta.sin().abs() * half)) as f32;
            let cy = (theta.cos() as f32) * s;
            let sy = (theta.sin() as f32) * s;
            let cr = (half as f32) + ((theta.cos() * half * half) as f32);
            let cg = (1.0 - half * half) as f32;
            let cb = 1.0_f32;
            self.instanced_batch.instance_rows_f32.extend_from_slice(&[
                cy,
                0.0,
                sy,
                cr,
                0.0,
                s * (1.0 + (half as f32)),
                0.0,
                cg,
                -sy,
                0.0,
                cy,
                cb,
                x as f32,
                y as f32,
                z as f32,
                1.0,
            ]);
        }
        self.instanced_batch.epoch = self.instanced_batch.epoch.wrapping_add(1).max(1);
        self.instanced_batch.effective_triangle_count()
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
        #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
        let solved = if self.custom_glb_scene.is_some() {
            let empty_bvh = TwoLevelBvh::build_solid(&SceneWorld::new(), &[])
                .unwrap_or_else(|_| self.two_level_bvh.clone());
            let five = (prism_kernel::sTwo() + prism_kernel::sThree()) as f64;
            let floor_y = anchor.y - five * five;
            self.camera_rig
                .solve_frame(dt_seconds, anchor, &empty_bvh, &|_, _| floor_y)
                .map_err(|e| WasmBridgeError::InvalidSnapshotEnvelope {
                    byte_len: 0,
                    reason: bridge_reason(e),
                })?
        } else {
            self.camera_rig
                .solve_frame(dt_seconds, anchor, &self.two_level_bvh, &|x, z| {
                    hd_terrain_height(seed_copy, &relief_copy, x, z)
                })
                .map_err(|e| WasmBridgeError::InvalidSnapshotEnvelope {
                    byte_len: 0,
                    reason: bridge_reason(e),
                })?
        };
        #[cfg(all(target_arch = "wasm32", not(feature = "uorc-decoder")))]
        let solved = self
            .camera_rig
            .solve_frame(dt_seconds, anchor, &self.two_level_bvh, &|x, z| {
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
        #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
        let cap = prism_kernel::workerCommandRingCapacity()
            .expect("verified Prism workerCommandRingCapacity") as usize;
        #[cfg(all(target_arch = "wasm32", not(feature = "uorc-decoder")))]
        let cap = verdant_core::verdant_tensor::kernel_consts().word_bit_width;
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
            if self.custom_glb_scene.is_none() {
                self.bvh = fresh.scene.build_bvh();
            }
            self.world = fresh;
        }

        #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
        let hd_sector = if self.npc_patrol_active {
            let active_slug = self.observer_holon.active_domain.spec.slug.clone();
            if self
                .observer_holon
                .switch_domain_pack(&active_slug, Some(u64::from(rank)))
                .is_ok()
            {
                self.walkable_polygons = self.observer_holon.active_navmesh.polygons.len();
                self.npcs
                    .clone_from(&self.observer_holon.active_domain.characters);
                self.observer_holon.active_domain.sector.clone()
            } else {
                synthesize_hd_citadel_sector(
                    self.seed_val,
                    &self.terrain_params,
                    self.env_params.water_level_meters,
                    u64::from(rank),
                )
            }
        } else {
            synthesize_hd_citadel_sector(
                self.seed_val,
                &self.terrain_params,
                self.env_params.water_level_meters,
                u64::from(rank),
            )
        };
        #[cfg(all(target_arch = "wasm32", not(feature = "uorc-decoder")))]
        let hd_sector = synthesize_hd_citadel_sector(
            self.seed_val,
            &self.terrain_params,
            self.env_params.water_level_meters,
            u64::from(rank),
        );
        // Imported triangles and their transforms are independent of procedural LOD.
        // For dense visual tiers (LOD3/LOD4), keep an already-built metric solid proxy BVH
        // so scaling up to UHD geometry never incurs a synchronous 150K-triangle BVH build hitch.
        #[cfg(all(target_arch = "wasm32", not(feature = "uorc-decoder")))]
        let rebuild_boundaries = rank <= (1 + 1) || self.two_level_bvh.instance_count() == 0;
        #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
        let rebuild_boundaries = self.custom_glb_scene.is_none()
            && (rank <= (1 + 1)
                || self.two_level_bvh.instance_count() == 0
                || self.two_level_bvh.total_surface_triangles()
                    < (1usize << (1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1)));
        if rebuild_boundaries {
            self.two_level_bvh = TwoLevelBvh::build_solid(&hd_sector.scene, &hd_sector.meshes)
                .map_err(|e| WasmBridgeError::InvalidSnapshotEnvelope {
                    byte_len: 0,
                    reason: bridge_reason(e),
                })?;
        }
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
                    &self.two_level_bvh,
                    &|x, z| hd_terrain_height(seed_copy, &relief_copy, x, z),
                );
                self.explorer = rebuilt_exp;
            }
        }

        self.hd_sector = hd_sector;
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
            .map_err(|e| WasmBridgeError::InvalidSnapshotEnvelope {
                byte_len: 0,
                reason: bridge_reason(e),
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

            #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
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

            #[cfg(all(target_arch = "wasm32", not(feature = "uorc-decoder")))]
            if let Ok(next_u) = UniversePosition::new_canonical(self.active_cell, cap_pos) {
                self.active_cell = next_u.cell;
                capsule.foot_position = next_u.local;
                cap_pos = next_u.local;
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
                &|x, z| hd_terrain_height(seed_copy, &relief_copy, x, z),
            );

            let player_head = self.explorer.head_anchor_world();
            if self.npc_patrol_active {
                self.observer_holon.step_npc_patrols(
                    clamped_dt,
                    walk_yaw_rad,
                    player_head,
                    &self.two_level_bvh,
                );
                self.npcs
                    .clone_from(&self.observer_holon.active_domain.characters);
            } else {
                let half = 1.0 / two;
                let gaze_thresh_sq = (half * half * half) * (half * half * half);
                let player_shifted =
                    (cap_pos - self.last_npc_ik_player_pos).length_squared() > gaze_thresh_sq;
                if player_shifted {
                    self.last_npc_ik_player_pos = cap_pos;
                    let max_npc_gaze_sq = (two * two * (two + 1.0)) * (two * two * (two + 1.0));
                    for npc in &mut self.npcs {
                        let p = npc.foot_position;
                        if (p - cap_pos).length_squared() <= max_npc_gaze_sq {
                            let _ = npc.step_and_solve_ik(
                                clamped_dt,
                                p,
                                Vec3::ZERO,
                                true,
                                npc.body_yaw_radians,
                                Some(player_head),
                                &self.two_level_bvh,
                                &|x, z| hd_terrain_height(seed_copy, &relief_copy, x, z),
                            );
                        }
                    }
                }
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
        if self.scene_buffers.indices.is_empty() {
            let _ = self.rebuild_scene_buffers(0.0);
        }
        self.scene_buffers.ensure_tile_triangles_built();
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

        let sun = Vec3::new(1.0 / two_f, 1.0, 1.0 / four);
        core::mem::swap(&mut self.tile_scratch.rgba8, &mut self.framebuffer_rgba);
        let res = render_triangles_tile_framebuffer_with_scratch(
            aligned_w,
            aligned_h,
            &vp,
            sun,
            u64::from(self.lod_rank),
            &self.scene_buffers.tile_triangles,
            &mut self.tile_scratch,
        );
        core::mem::swap(&mut self.tile_scratch.rgba8, &mut self.framebuffer_rgba);
        res.map_err(|e| WasmBridgeError::InvalidSnapshotEnvelope {
            byte_len: 0,
            reason: bridge_reason(e),
        })?;
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

    /// Encodes the active `StandardSceneState` into a SHA-256-sealed `UORC` snapshot archive (`b"UORC"`)
    /// verified by `prism_kernel::wasmBridgeEnvelopeValid`.
    #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
    pub fn encode_uorc_snapshot(&self) -> Result<Vec<u8>, WasmBridgeError> {
        let bytes = WorldSnapshot::encode_uorc(&self.world).map_err(|e| {
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
                reason: "UORC snapshot exceeds Prism URL bridge envelope".to_string(),
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
        self.custom_glb_scene = None;
        let _ = self.solve_active_camera(0.0);
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
        self.custom_glb_scene = None;
        let _ = self.solve_active_camera(0.0);
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
        let prev_seed = self.seed_val;
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
        let two_u64 = 1u64 + 1u64;
        if prev_seed != restored.seed
            || hd_rank <= two_u64
            || self.two_level_bvh.instance_count() == 0
        {
            let bvh_sector = if hd_rank > two_u64 {
                synthesize_hd_citadel_sector(
                    restored.seed,
                    &restored.params.terrain,
                    restored.params.environment.water_level_meters,
                    two_u64,
                )
            } else {
                self.hd_sector.clone()
            };
            if let Ok(tl) = TwoLevelBvh::build_solid(&bvh_sector.scene, &bvh_sector.meshes) {
                self.two_level_bvh = tl;
            }
        }
        self.world = restored;
        self.custom_glb_scene = None;
        let _ = self.solve_active_camera(0.0);
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
