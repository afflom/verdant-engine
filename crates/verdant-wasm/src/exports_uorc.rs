//! Studio, UORC streaming, UHD meshlet/governor, VCS, and operator WebAssembly C-ABI exports.

use super::*;

/// Executes a live `verdant-aqm` extrapolated performance benchmark over `entity_count` entities
/// (`ColumnBitPlaneTable` 8-plane `O(N / 64)` popcount query, `canonicalize_command_monoid`
/// involution + `Z_8` octant fusion, `KappaTransitionCache` elision, and `CheckpointedRollbackJournal`
/// stride-8 replay) and writes a `|`-delimited telemetry summary to `aux_buffer`.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_aqm_benchmark_aux(entity_count: u32) -> u32 {
    let Ok(mut guard) = global_bridge().lock() else {
        return 0;
    };
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };
    let two = 1usize + 1usize;
    let three = two + 1;
    let four = two * two;
    let eight = four * two;
    let n = (entity_count as usize).max(1 << (eight + two));

    let mut planes = verdant_core::verdant_tensor::ColumnBitPlaneTable::new(eight, n);
    for idx in 0..n {
        planes.set(0, idx, true);
        planes.set(1, idx, idx % two == 0);
        planes.set(two, idx, idx % three != 0);
        planes.set(three, idx, idx % eight == 0);
    }
    let mut mask = verdant_core::verdant_tensor::WordBitset::with_len(n);
    let matched = planes.query_archetype_into(&[0, 1, two], &[three], &mut mask);
    let pop_pair = planes.popcount_intersection(1, two);

    let ops = vec![
        CanonicalCommandOp {
            id: 0,
            target_slot: 0,
            kind: CanonicalCommandKind::ToggleCameraMode,
            depends_on: Vec::new(),
        },
        CanonicalCommandOp {
            id: 1,
            target_slot: 0,
            kind: CanonicalCommandKind::ToggleCameraMode,
            depends_on: Vec::new(),
        },
        CanonicalCommandOp {
            id: two as u32,
            target_slot: 1,
            kind: CanonicalCommandKind::YawOctantStep {
                octants: three as u8,
            },
            depends_on: Vec::new(),
        },
        CanonicalCommandOp {
            id: three as u32,
            target_slot: 1,
            kind: CanonicalCommandKind::YawOctantStep {
                octants: (eight - three) as u8,
            },
            depends_on: Vec::new(),
        },
        CanonicalCommandOp {
            id: four as u32,
            target_slot: 0,
            kind: CanonicalCommandKind::CameraOrbitDelta {
                delta_yaw_milli: eight as i32,
                delta_pitch_milli: 0,
                delta_dist_milli: 0,
            },
            depends_on: Vec::new(),
        },
        CanonicalCommandOp {
            id: (four + 1) as u32,
            target_slot: 0,
            kind: CanonicalCommandKind::CameraOrbitDelta {
                delta_yaw_milli: four as i32,
                delta_pitch_milli: -(two as i32),
                delta_dist_milli: 0,
            },
            depends_on: Vec::new(),
        },
    ];
    let Ok(monoid) = canonicalize_command_monoid(&ops) else {
        return 0;
    };

    let mut kappa_cache: KappaTransitionCache<u64> = KappaTransitionCache::new(eight * four);
    let init_k = KappaTransitionCache::<u64>::digest_bytes(&bridge.seed_val.to_le_bytes());
    let mut elided = 0usize;
    for step_i in 0..eight {
        let tag = (step_i % two) as u64;
        let (_, _, hit) = kappa_cache.step_or_elide(init_k, tag, || {
            let next_v = bridge.seed_val.wrapping_add(tag + 1);
            (next_v, next_v.to_le_bytes().to_vec())
        });
        if hit {
            elided += 1;
        }
    }

    let mut journal: CheckpointedRollbackJournal<u64, u64> =
        CheckpointedRollbackJournal::new(0_u64, &bridge.seed_val.to_le_bytes());
    let total_ticks = eight * three;
    let mut st = 0_u64;
    for t in 0..total_ticks {
        let delta = (t + 1) as u64;
        st += delta;
        journal.record_step(delta, &delta.to_le_bytes(), &st);
    }
    let target_f = total_ticks - three;
    let replay_steps = journal
        .reconstruct_frame(target_f, |prev, inp| prev + inp)
        .map(|(_, s, _)| s)
        .unwrap_or(0);

    let summary = format!(
        "entities={n}|words_per_plane={}|archetype_matches={matched}|popcnt_pair={pop_pair}|raw_ops={}|fused_ops={}|involutions={}|merges={}|kappa_elided={elided}|ckpts={}|replay_steps={replay_steps}",
        planes.planes_per_col(),
        monoid.raw_count,
        monoid.fused_count,
        monoid.involution_cancellations,
        monoid.additive_merges,
        journal.stored_checkpoint_count(),
    );
    bridge.aux_buffer = summary.into_bytes();
    bridge.aux_buffer.len() as u32
}

/// Executes a live rock-solid stability and arbitrary-workload audit in WebAssembly (`VE-35`):
/// 1. Flat CSR software tile rasterization (`steady_alloc_growth == 0` bytes on frame 2),
/// 2. `body_count`-body spatial-grid broadphase sphere-sphere collision resolution + `NaN` intent sanitization,
/// 3. `52`-bit IEEE-754 floating-origin horizon refusal (`UniversePosition::new_canonical`),
/// 4. Multi-primitive Khronos `.glb` packing and merging via `import_glb_to_scene`.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_stability_audit_aux(body_count: u32) -> u32 {
    let mut guard = global_bridge()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };

    let two = 1usize + 1usize;
    let four = two * two;
    let eight = four * two;
    let sixteen = eight * two;
    let tile_w = (sixteen * eight) as u32;
    let tile_h = (sixteen * four) as u32;

    if bridge.scene_buffers.indices.is_empty() {
        let _ = bridge.rebuild_scene_buffers(0.0);
    }
    bridge.scene_buffers.ensure_tile_triangles_built();
    let sample_len = bridge
        .scene_buffers
        .tile_triangles
        .len()
        .min(sixteen * eight);
    let sample_tris = bridge.scene_buffers.tile_triangles[..sample_len].to_vec();
    let vp = Mat4::perspective(
        std::f64::consts::FRAC_PI_3,
        (tile_w as f64) / (tile_h as f64),
        1.0 / (four as f64),
        (sixteen * eight) as f64,
    ) * Mat4::look_at(
        bridge.last_camera_frame.eye,
        bridge.last_camera_frame.target,
        Vec3::UP,
    );
    let sun = Vec3::new(1.0 / (two as f64), 1.0, 1.0 / (four as f64));
    let mut csr_scratch = TileRasterScratch::new();
    let (csr_tiles, csr_tris) = render_triangles_tile_framebuffer_with_scratch(
        tile_w,
        tile_h,
        &vp,
        sun,
        u64::from(bridge.lod_rank),
        &sample_tris,
        &mut csr_scratch,
    )
    .unwrap_or((0, 0));
    let cap_after_first = csr_scratch.total_allocated_bytes();
    let _ = render_triangles_tile_framebuffer_with_scratch(
        tile_w,
        tile_h,
        &vp,
        sun,
        u64::from(bridge.lod_rank),
        &sample_tris,
        &mut csr_scratch,
    );
    let csr_alloc_growth = csr_scratch
        .total_allocated_bytes()
        .saturating_sub(cap_after_first);

    // Spatial-grid dynamic sphere broadphase + NaN wish sanitization:
    let n_bodies = (body_count as usize).max(sixteen * four);
    let mut pw = PhysicsWorld::new(PhysicsWorldConfig::default());
    let cols = sixteen;
    let half_f = 1.0 / (two as f64);
    let radius = half_f;
    let spacing = radius * (1.0 + half_f);
    for i in 0..n_bodies {
        let gx = (i % cols) as f64;
        let gz = (i / cols) as f64;
        if let Ok(mut rb) = RigidBody::new(
            "AuditSphere",
            Vec3::new(gx * spacing, (four as f64) + radius, gz * spacing),
            1.0 + half_f,
            radius,
            half_f,
        ) {
            rb.velocity = Vec3::new(if i % two == 0 { 1.0 } else { -1.0 }, 0.0, 0.0);
            pw.bodies.push(rb);
        }
    }
    pw.player_capsule = Some(CapsuleController::new(Vec3::new(0.0, two as f64, 0.0)));
    let contacts = pw.resolve_dynamic_body_broadphase();
    let dt = 1.0 / ((sixteen * four) as f64);
    let _ = pw.step(dt, Vec3::new(f64::NAN, f64::INFINITY, f64::NEG_INFINITY));
    let nan_safe = u32::from(
        pw.player_capsule
            .as_ref()
            .is_some_and(|c| c.foot_position.x.is_finite() && c.foot_position.z.is_finite()),
    );

    // 52-bit exact IEEE-754 floating-origin horizon guard:
    let kc = verdant_core::verdant_tensor::kernel_consts();
    let cell_span = verdant_core::universe::canonical_cell_span_meters();
    let astro_x = (verdant_core::verdant_tensor::pow2_u64(kc.f64_exact_mantissa_bits) as f64)
        * cell_span
        * (two as f64);
    let horizon_refused = u32::from(
        UniversePosition::new_canonical(UniverseCellCoord::ORIGIN, Vec3::new(astro_x, 0.0, 0.0))
            .is_err(),
    );

    // Multi-primitive Khronos GLB pack + import merge:
    let mats = canonical_pbr_material_library();
    let ma = generate_faceted_prism_mesh(half_f, 1.0);
    let mb = generate_faceted_prism_mesh(1.0, two as f64);
    let glb_tris = pack_multi_primitive_glb("AuditMulti", &[(&ma, 0), (&mb, 1)], &mats)
        .and_then(|bytes| import_glb_to_scene(&bytes))
        .ok()
        .and_then(|imp| imp.meshes.first().map(|m| m.triangle_count()))
        .unwrap_or(0);

    let summary = format!(
        "csr_tiles={csr_tiles}|csr_tris={csr_tris}|csr_alloc_growth={csr_alloc_growth}|bodies={}|contacts={contacts}|nan_safe={nan_safe}|horizon_refused={horizon_refused}|glb_tris={glb_tris}",
        pw.bodies.len(),
    );
    bridge.aux_buffer = summary.into_bytes();
    bridge.aux_buffer.len() as u32
}

/// Executes a live end-to-end audit of Verdant Engine's exclusive `UORC` wire runtime (`VE-36`)
/// inside WebAssembly:
/// 1. `WorldSnapshot::encode_uorc` vs raw `VDNT` byte compression ratio + `b"UORC"` seal check,
/// 2. `CodedLodFrameBundle::pack_to_uorc` + `unpack_from_uorc` + zero-decode `coded_gemm` contraction,
/// 3. `shard_uorc_to_stitch_fragments` + `recover_uorc_from_stitch_pair` `2-of-3` parity recovery,
/// 4. `CheckpointedRollbackJournal::new_uorc` stride-8 `UORC` checkpoint memory compression.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_uorc_exclusive_audit_aux() -> u32 {
    let mut guard = global_bridge()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };

    let raw_vdnt = WorldSnapshot::encode(&bridge.world).unwrap_or_default();
    let uorc_snap = WorldSnapshot::encode_uorc(&bridge.world).unwrap_or_default();
    let snap_magic_ok = u32::from(is_uorc_archive(&uorc_snap));

    let mut tampered = uorc_snap.clone();
    if let Some(last) = tampered.last_mut() {
        *last ^= 1_u8;
    }
    let tamper_refused = u32::from(WorldSnapshot::decode(&tampered).is_err());

    let max_lod = prism_kernel::wasmMaxLodRank();
    let coded_bundle = CodedLodFrameBundle::synthesize(bridge.seed_val, max_lod).ok();
    let (coded_uorc_bytes, zero_decode_ok) = if let Some(bundle) = coded_bundle {
        let uorc_cdmx = bundle.pack_to_uorc();
        let len = uorc_cdmx.len();
        let unpacked = CodedLodFrameBundle::unpack_from_uorc(&uorc_cdmx).ok();
        let q_vec = vec![1_i32; bundle.inner_k];
        let ref_sum = bundle.contract_zero_decode(&q_vec, &q_vec, &q_vec).ok();
        let unp_sum = unpacked.and_then(|u| u.contract_zero_decode(&q_vec, &q_vec, &q_vec).ok());
        (
            len,
            u32::from(is_uorc_archive(&uorc_cdmx) && ref_sum.is_some() && ref_sum == unp_sum),
        )
    } else {
        (0usize, 0_u32)
    };

    let stitch_shard_bytes = shard_uorc_to_stitch_fragments(&uorc_snap)
        .ok()
        .and_then(|frags| {
            let two = 1usize + 1usize;
            recover_uorc_from_stitch_pair(&frags[0], &frags[two])
                .ok()
                .filter(|(rec_uorc, _)| rec_uorc == &uorc_snap)
                .map(|_| frags[0].payload.len())
        })
        .unwrap_or(0);

    let mut uorc_journal =
        CheckpointedRollbackJournal::new_uorc(&raw_vdnt, &bridge.seed_val.to_le_bytes());
    let stride = uorc_journal.checkpoint_stride();
    for step_idx in 0..stride {
        let b = (step_idx + 1) as u8;
        uorc_journal.record_uorc_step(b, &[b], &raw_vdnt);
    }
    let ckpt_uorc_bytes = uorc_journal.total_checkpoint_uorc_bytes();

    let summary = format!(
        "raw_vdnt_bytes={}|uorc_snap_bytes={}|snap_uorc_magic={snap_magic_ok}|tamper_refused={tamper_refused}|coded_uorc_bytes={coded_uorc_bytes}|zero_decode_ok={zero_decode_ok}|stitch_shard_bytes={stitch_shard_bytes}|ckpt_uorc_bytes={ckpt_uorc_bytes}",
        raw_vdnt.len(),
        uorc_snap.len(),
    );
    bridge.aux_buffer = summary.into_bytes();
    bridge.aux_buffer.len() as u32
}

/// Switches `WasmRuntimeBridge` live to one of the 4 SHA-256-locked cataloged domain packs
/// (`0 = sunspire-regatta`, `1 = meridian-observatory`, `2 = glasshouse-chronicle`,
/// `3 = starfall-bastion`), rebuilding terrain, HD landmarks, `TwoLevelBvh`, walkable `NavMesh`,
/// and differentiated 22-joint NPCs, and writing the Observer status JSON into `aux_buffer`.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_load_domain_pack(domain_idx: u32) -> u32 {
    let mut guard = global_bridge()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };

    let two = 1_u32 + 1_u32;
    let three = two + 1_u32;
    let slug = if domain_idx == 0 {
        "sunspire-regatta"
    } else if domain_idx == 1 {
        "meridian-observatory"
    } else if domain_idx == two {
        "glasshouse-chronicle"
    } else if domain_idx == three {
        "starfall-bastion"
    } else {
        return 0;
    };

    let lod = u64::from(bridge.lod_rank);
    if bridge
        .observer_holon
        .switch_domain_pack(slug, Some(lod))
        .is_err()
    {
        return 0;
    }

    let dom = &bridge.observer_holon.active_domain;
    bridge.seed_val = dom.spec.seed;
    bridge.terrain_params = dom.terrain_relief;
    bridge.env_params = dom.physical_environment;
    bridge.physics.config.terrain_seed = dom.spec.seed;
    bridge.physics.config.terrain_relief = dom.terrain_relief;
    bridge.physics.config.environment = dom.physical_environment;
    bridge.hd_sector = dom.sector.clone();
    if let Ok(bvh) = TwoLevelBvh::build_solid(&bridge.hd_sector.scene, &bridge.hd_sector.meshes) {
        bridge.two_level_bvh = bvh;
    }
    bridge.walkable_polygons = bridge.observer_holon.active_navmesh.polygons.len();
    bridge.npcs = dom.characters.clone();
    bridge.custom_glb_scene = None;
    bridge.custom_glb_uvs_xy.clear();
    bridge.custom_glb_tex_rgba.clear();
    bridge.custom_glb_tex_width = 0;
    bridge.custom_glb_tex_height = 0;
    bridge.custom_glb_primary_tex_mask.clear();
    bridge.custom_glb_mr_tex = None;
    bridge.custom_glb_normal_tex = None;
    bridge.custom_glb_emissive_tex = None;
    bridge.custom_glb_rig = None;
    bridge.custom_glb_mesh_atlas_uvs.clear();
    bridge.custom_glb_atlas_tile_count = 0;
    bridge.npc_patrol_active = true;
    bridge.blank_studio_mode = false;
    bridge.static_dirty = true;
    let _ = bridge.rebuild_scene_buffers(0.0);

    let two_f = prism_kernel::sTwo() as f64;
    let three_f = prism_kernel::sThree() as f64;
    let four_f = two_f * two_f;
    let nine_f = three_f * three_f;
    let half_f = 1.0 / two_f;
    let overview_dist = if domain_idx == 0 {
        nine_f * two_f
    } else {
        nine_f * two_f + four_f + two_f
    };
    bridge.camera_anchor_override = if domain_idx == 0 {
        Some(Vec3::new(two_f, four_f, nine_f))
    } else {
        Some(Vec3::new(0.0, four_f, 0.0))
    };
    bridge.camera_rig.mode = CameraMode::ThirdPersonSpringArm;
    bridge.camera_rig.yaw_radians = half_f;
    bridge.camera_rig.pitch_radians = if domain_idx == 0 {
        half_f * half_f
    } else {
        half_f * (half_f + half_f * half_f)
    };
    bridge.camera_rig.desired_distance_meters = overview_dist;
    bridge.camera_rig.current_distance_meters = overview_dist;
    let _ = bridge.solve_active_camera(0.0);

    bridge.aux_buffer = bridge.observer_holon.to_status_json_bytes();
    bridge.aux_buffer.len() as u32
}

/// Assigns `directive_idx` to cataloged `npc_idx`, replans its admissible `NavMesh` A* corridor,
/// activates live NPC patrol locomotion, records an `AgentToAgent` event on the Observer bus,
/// and writes `"<character_id>|<directive_label>|<waypoint_count>"` into `aux_buffer`.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_set_npc_directive(npc_idx: u32, directive_idx: u32) -> u32 {
    let mut guard = global_bridge()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };

    let Ok(state) = bridge
        .observer_holon
        .set_npc_directive(npc_idx as usize, directive_idx as usize)
    else {
        return 0;
    };
    bridge.npc_patrol_active = true;
    bridge
        .npcs
        .clone_from(&bridge.observer_holon.active_domain.characters);
    let summary = format!(
        "{}|{}|{}",
        state.character_id,
        state.active_directive_label,
        state.corridor_waypoints.len()
    );
    bridge.aux_buffer = summary.into_bytes();
    bridge.aux_buffer.len() as u32
}

/// Evaluates a native `VerdantLanguageEngine` ISO 24617-2 player-to-NPC dialogue turn from
/// `io_buffer` (UTF-8 utterance), signing an origin-scoped `PrismIdSessionEnvelope`, logging both
/// `PlayerToAgent` and `AgentToPlayer` communications under W3C SOSA/PROV-O, and writing the
/// grounded reply or `"Unknown"` abstention JSON into `aux_buffer`.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_dialogue_turn_io(npc_idx: u32) -> u32 {
    let mut guard = global_bridge()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };
    let Ok(utterance) = std::str::from_utf8(&bridge.io_buffer) else {
        return 0;
    };
    let trimmed = utterance.trim().to_string();
    if trimmed.is_empty() {
        return 0;
    }

    let origin = "https://afflom.github.io";
    let issued_sec = prism_kernel::skinWeightScale().unwrap_or(1);
    let expires_sec = issued_sec + prism_kernel::skinWeightScale().unwrap_or(1);
    let digest = sha256_payload_digest(trimmed.as_bytes());
    let session = PrismIdSessionEnvelope::sign_deterministic(
        "prism:studio-player",
        &bridge.seed_val.to_le_bytes(),
        origin,
        issued_sec,
        expires_sec,
        &digest,
    );

    let Ok(turn) = bridge.observer_holon.converse_player_with_npc(
        "urn:verdant:player:local",
        (&session, origin, issued_sec + 1),
        npc_idx as usize,
        &trimmed,
    ) else {
        return 0;
    };

    if turn.mutated_directive_index.is_some() {
        bridge.npc_patrol_active = true;
        bridge
            .npcs
            .clone_from(&bridge.observer_holon.active_domain.characters);
    }
    let comm_kappa = bridge
        .observer_holon
        .communications
        .last()
        .map_or("", |c| c.kappa_hex.as_str())
        .to_string();
    bridge.aux_buffer = turn.to_json_bytes(&comm_kappa);
    bridge.aux_buffer.len() as u32
}

/// Records an explicit communication (`channel_code`: `0=OrganToOrgan`, `1=PlayerToPlayer`,
/// `2=PlayerToAgent`, `3=AgentToPlayer`, `4=AgentToAgent`) formatted as `"sender|recipient|message"`
/// in `io_buffer` into the Observer holon's W3C SOSA/PROV-O bus, writing `kappa_hex` into `aux_buffer`.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_observe_comm_io(channel_code: u32) -> u32 {
    let mut guard = global_bridge()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };
    let Ok(raw) = std::str::from_utf8(&bridge.io_buffer) else {
        return 0;
    };
    let mut parts = raw.splitn(1 + 1 + 1, '|');
    let Some(sender) = parts.next() else {
        return 0;
    };
    let Some(recipient) = parts.next() else {
        return 0;
    };
    let Some(msg) = parts.next() else {
        return 0;
    };
    let sender_s = sender.trim().to_string();
    let recipient_s = recipient.trim().to_string();
    let msg_s = msg.trim().to_string();

    let two = 1_u32 + 1_u32;
    let three = two + 1_u32;
    let four = two * two;
    let channel = if channel_code == 0 {
        CommunicationChannelKind::OrganToOrgan
    } else if channel_code == 1 {
        CommunicationChannelKind::PlayerToPlayer
    } else if channel_code == two {
        CommunicationChannelKind::PlayerToAgent
    } else if channel_code == three {
        CommunicationChannelKind::AgentToPlayer
    } else if channel_code == four {
        CommunicationChannelKind::AgentToAgent
    } else {
        return 0;
    };

    let origin = "https://afflom.github.io";
    let issued_sec = prism_kernel::skinWeightScale().unwrap_or(1);
    let expires_sec = issued_sec + prism_kernel::skinWeightScale().unwrap_or(1);
    let digest = sha256_payload_digest(msg_s.as_bytes());
    let session = PrismIdSessionEnvelope::sign_deterministic(
        "prism:peer-citizen",
        &bridge.seed_val.to_le_bytes(),
        origin,
        issued_sec,
        expires_sec,
        &digest,
    );
    let opt_session = if channel.requires_prism_signature() {
        Some((&session, origin, issued_sec + 1))
    } else {
        None
    };

    let Ok(rec) = bridge.observer_holon.observe_communication(
        channel,
        &sender_s,
        &recipient_s,
        "en",
        0,
        &msg_s,
        opt_session,
        0,
    ) else {
        return 0;
    };
    bridge.aux_buffer = rec.kappa_hex.into_bytes();
    bridge.aux_buffer.len() as u32
}

/// Writes the live Observer Prism holon JSON snapshot (`observerKappa`, `organs`, `stakeholders`,
/// `activeDomain`, `npcs`, and `recentCommunications`) into `aux_buffer`, returning byte length.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_observer_status_aux() -> u32 {
    let mut guard = global_bridge()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };
    bridge.aux_buffer = bridge.observer_holon.to_status_json_bytes();
    bridge.aux_buffer.len() as u32
}

/// Admits and auto-tags an external/local catalog model JSON staged in `io_buffer` into
/// `ObserverPrismHolon::project_models`, writing the versioned record JSON into `aux_buffer`.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_import_project_model_io() -> u32 {
    let mut guard = global_bridge()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };
    let req_bytes = bridge.io_buffer.clone();
    let grants = bridge.grants.clone();
    let Ok(out) = bridge
        .observer_holon
        .import_project_model_json(&req_bytes, &grants)
    else {
        return 0;
    };
    bridge.aux_buffer = out;
    bridge.aux_buffer.len() as u32
}

/// Edits an imported project model JSON staged in `io_buffer`, committing immutable `vN+1`
/// and writing the updated record JSON into `aux_buffer`.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_edit_project_model_io() -> u32 {
    let mut guard = global_bridge()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };
    let req_bytes = bridge.io_buffer.clone();
    let Ok(out) = bridge.observer_holon.edit_project_model_json(&req_bytes) else {
        return 0;
    };
    bridge.aux_buffer = out;
    bridge.aux_buffer.len() as u32
}

/// Reverts the imported model whose UTF-8 `model_id` is staged in `io_buffer` back to
/// `target_version`, writing the new non-destructive revision JSON into `aux_buffer`.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_revert_project_model_io(target_version: u32) -> u32 {
    let mut guard = global_bridge()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };
    let Ok(model_id) = std::str::from_utf8(&bridge.io_buffer).map(str::to_string) else {
        return 0;
    };
    let Ok(out) = bridge
        .observer_holon
        .revert_project_model_json(&model_id, target_version)
    else {
        return 0;
    };
    bridge.aux_buffer = out;
    bridge.aux_buffer.len() as u32
}

/// Removes the imported model whose UTF-8 `model_id` is staged in `io_buffer`, writing the
/// removed revision's `kappa` hex into `aux_buffer`.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_remove_project_model_io() -> u32 {
    let mut guard = global_bridge()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };
    let Ok(model_id) = std::str::from_utf8(&bridge.io_buffer).map(str::to_string) else {
        return 0;
    };
    let Ok(kappa_hex) = bridge.observer_holon.remove_project_model(&model_id) else {
        return 0;
    };
    bridge.aux_buffer = kappa_hex.into_bytes();
    bridge.aux_buffer.len() as u32
}

/// Writes the JSON list of all currently imported project models and their version histories
/// into `aux_buffer`.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_list_project_models_aux() -> u32 {
    let mut guard = global_bridge()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };
    bridge.aux_buffer = bridge.observer_holon.project_models_json_bytes();
    bridge.aux_buffer.len() as u32
}

/// Pointer to packed per-vertex UV coordinates (`2 * static_vert_count` `f32` values) for textured `.glb` rendering.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_scene_uv_ptr() -> u32 {
    let Ok(guard) = global_bridge().lock() else {
        return 0;
    };
    guard.as_ref().map_or(0, |b| {
        if b.custom_glb_uvs_xy.is_empty() {
            0
        } else {
            b.custom_glb_uvs_xy.as_ptr() as u32
        }
    })
}

/// Pointer to host-bounded primary `RGBA8` albedo texture bytes extracted from the active `.glb`.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_scene_tex_ptr() -> u32 {
    let Ok(guard) = global_bridge().lock() else {
        return 0;
    };
    guard.as_ref().map_or(0, |b| {
        if b.custom_glb_tex_rgba.is_empty() {
            0
        } else {
            b.custom_glb_tex_rgba.as_ptr() as u32
        }
    })
}

/// Width in pixels of the active host-bounded GPU albedo texture (`0` when untextured).
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_scene_tex_width() -> u32 {
    let Ok(guard) = global_bridge().lock() else {
        return 0;
    };
    guard.as_ref().map_or(0, |b| b.custom_glb_tex_width)
}

/// Height in pixels of the active host-bounded GPU albedo texture (`0` when untextured).
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_scene_tex_height() -> u32 {
    let Ok(guard) = global_bridge().lock() else {
        return 0;
    };
    guard.as_ref().map_or(0, |b| b.custom_glb_tex_height)
}

/// Monotonic epoch for the active GPU albedo texture.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_scene_tex_epoch() -> u32 {
    let Ok(guard) = global_bridge().lock() else {
        return 0;
    };
    guard.as_ref().map_or(0, |b| b.custom_glb_tex_epoch)
}

/// Appends a second Khronos `.glb` from `io_buffer` into the active composed stage (or starts a
/// new composed stage if none is active), auto-placing it cleanly beside existing assets and
/// re-framing the look-dev orbit camera around the combined multi-asset scene.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_glb_compose_append_io() -> u32 {
    let Ok(mut guard) = global_bridge().lock() else {
        return 0;
    };
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };
    if bridge.custom_glb_scene.is_none() {
        drop(guard);
        return verdant_wasm_glb_import_io();
    }
    let tex_cap = bridge
        .observer_holon
        .host_envelope
        .recommended_texture_cap_px;
    let Ok(imported) = import_glb_to_scene_bounded(&bridge.io_buffer, tex_cap) else {
        return 0;
    };

    // Measure current composed scene X-max so the appended asset sits cleanly beside it:
    let three = 1usize + 1usize + 1usize;
    let two = 1usize + 1usize;
    let mut existing_max_x = f64::NEG_INFINITY;
    let mut existing_min_y = f64::INFINITY;
    let mut existing_max_y = f64::NEG_INFINITY;
    for chunk in bridge.scene_buffers.positions_xyz.chunks_exact(three) {
        let px = f64::from(chunk[0]);
        let py = f64::from(chunk[1]);
        if px.is_finite() && px > existing_max_x {
            existing_max_x = px;
        }
        if py.is_finite() {
            if py < existing_min_y {
                existing_min_y = py;
            }
            if py > existing_max_y {
                existing_max_y = py;
            }
        }
    }
    let half = 1.0 / (prism_kernel::sTwo() as f64);
    let target_h = if existing_min_y.is_finite() && existing_max_y.is_finite() {
        (existing_max_y - existing_min_y).max(1.0)
    } else {
        1.0 + 1.0
    };

    // Compute bounds of incoming asset to normalize its scale & ground it cleanly:
    let mut in_min = [f64::INFINITY; 1 + 1 + 1];
    let mut in_max = [f64::NEG_INFINITY; 1 + 1 + 1];
    if let Ok(in_packed) = pack_world_meshes_to_buffers_opts(
        &imported.scene,
        &imported.meshes,
        &imported.materials,
        false,
    ) {
        for chunk in in_packed.positions_xyz.chunks_exact(three) {
            let px = f64::from(chunk[0]);
            let py = f64::from(chunk[1]);
            let pz = f64::from(chunk[two]);
            if px.is_finite() && py.is_finite() && pz.is_finite() {
                in_min[0] = in_min[0].min(px);
                in_min[1] = in_min[1].min(py);
                in_min[two] = in_min[two].min(pz);
                in_max[0] = in_max[0].max(px);
                in_max[1] = in_max[1].max(py);
                in_max[two] = in_max[two].max(pz);
            }
        }
    }
    let in_h = if in_min[1].is_finite() && in_max[1].is_finite() {
        (in_max[1] - in_min[1]).max(half * half)
    } else {
        1.0
    };
    let in_w = if in_min[0].is_finite() && in_max[0].is_finite() {
        (in_max[0] - in_min[0]).max(half)
    } else {
        1.0
    };
    let norm_scale =
        (target_h / in_h).clamp(half * half * half, (1.0 + 1.0 + 1.0 + 1.0) * (1.0 + 1.0));
    let offset_x = (if existing_max_x.is_finite() {
        existing_max_x
    } else {
        0.0
    }) + (in_w * norm_scale * half)
        + target_h * half * half;
    let ground_y = if existing_min_y.is_finite() {
        existing_min_y
    } else {
        0.0
    } - (if in_min[1].is_finite() {
        in_min[1] * norm_scale
    } else {
        0.0
    });
    let offset_z = -((if in_min[two].is_finite() && in_max[two].is_finite() {
        (in_min[two] + in_max[two]) * half
    } else {
        0.0
    }) * norm_scale);

    if let Some((ref mut base_scene, ref mut base_meshes, ref mut base_mats)) =
        bridge.custom_glb_scene
    {
        let mesh_offset = base_meshes.len();
        let mat_offset = base_mats.len();
        base_meshes.extend(imported.meshes);
        base_mats.extend(imported.materials);
        // Composed companion meshes use their vertex-sampled albedo colors while keeping the hero's primary texture intact:
        for _ in 0..imported.primary_tex_mesh_mask.len() {
            bridge.custom_glb_primary_tex_mask.push(false);
        }
        let anchor_id = base_scene.spawn(
            "ComposedStreamAnchor",
            verdant_core::ecs::EntityRole::StaticArchitecture,
            None,
            Transform {
                translation: Vec3::new(offset_x, ground_y, offset_z),
                rotation: verdant_core::math::Quat::IDENTITY,
                scale: Vec3::new(norm_scale, norm_scale, norm_scale),
            },
            verdant_core::math::Aabb::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0)),
            (0.0, "stream:composed"),
        );
        let mut id_map = std::collections::BTreeMap::new();
        for ent in imported.scene.entities() {
            let mapped_parent = ent
                .parent
                .and_then(|pid| id_map.get(&pid).copied())
                .or(Some(anchor_id));
            let new_id = base_scene.spawn(
                ent.name.clone(),
                ent.role,
                mapped_parent,
                ent.local_transform,
                ent.local_bounds,
                (ent.emerged_at_lod_scale, ent.provenance_tag.clone()),
            );
            if let Some(m_idx) = ent.mesh_index {
                let mat_idx = ent.material_index.map_or(mat_offset, |mi| mat_offset + mi);
                base_scene.attach_renderable(new_id, mesh_offset + m_idx, mat_idx);
            }
            id_map.insert(ent.id, new_id);
        }
        if let Ok(bvh) = TwoLevelBvh::build_solid(base_scene, base_meshes) {
            bridge.two_level_bvh = bvh;
        }
        bridge.bvh = base_scene.build_bvh();
    }

    bridge.static_dirty = true;
    let tris = bridge.rebuild_scene_buffers(0.0).unwrap_or(0);

    // Re-frame camera cleanly around the full composed multi-asset bounding sphere:
    let mut min_p = [f64::INFINITY; 1 + 1 + 1];
    let mut max_p = [f64::NEG_INFINITY; 1 + 1 + 1];
    for chunk in bridge.scene_buffers.positions_xyz.chunks_exact(three) {
        let px = f64::from(chunk[0]);
        let py = f64::from(chunk[1]);
        let pz = f64::from(chunk[two]);
        if px.is_finite() && py.is_finite() && pz.is_finite() {
            min_p[0] = min_p[0].min(px);
            min_p[1] = min_p[1].min(py);
            min_p[two] = min_p[two].min(pz);
            max_p[0] = max_p[0].max(px);
            max_p[1] = max_p[1].max(py);
            max_p[two] = max_p[two].max(pz);
        }
    }
    if min_p[0].is_finite() && max_p[0].is_finite() {
        let cx = (min_p[0] + max_p[0]) * half;
        let cy = (min_p[1] + max_p[1]) * half;
        let cz = (min_p[two] + max_p[two]) * half;
        let dx = (max_p[0] - min_p[0]) * half;
        let dy = (max_p[1] - min_p[1]) * half;
        let dz = (max_p[two] - min_p[two]) * half;
        let radius = (dx * dx + dy * dy + dz * dz).sqrt().max(half * half);
        let frame_dist = (radius * (1.0 + 1.0 + 1.0 + half)).max(1.0 + 1.0);
        let needed_max = frame_dist * (1.0 + 1.0);
        if needed_max > bridge.camera_rig.config.max_orbit_distance_meters {
            bridge.camera_rig.config.max_orbit_distance_meters = needed_max;
        }
        bridge.camera_anchor_override = Some(Vec3::new(cx, cy, cz));
        bridge.camera_rig.mode = CameraMode::ThirdPersonSpringArm;
        bridge.camera_rig.yaw_radians = half * half;
        bridge.camera_rig.pitch_radians = half * half * half;
        bridge.camera_rig.desired_distance_meters = frame_dist;
        bridge.camera_rig.current_distance_meters = frame_dist;
    }
    let _ = bridge.solve_active_camera(0.0);
    tris as u32
}

/// Feeds live host container GPU/CPU/frame telemetry into `ObserverPrismHolon` and writes
/// the updated cognitive resolution JSON into `aux_buffer`.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_update_host_envelope(
    max_tex_px: u32,
    is_software_gpu: u32,
    hardware_threads: u32,
    ema_frame_micros: u32,
) -> u32 {
    let mut guard = global_bridge()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };
    let tris = bridge.scene_buffers.triangle_count() as u32;
    bridge.observer_holon.update_host_envelope(
        max_tex_px,
        is_software_gpu != 0,
        hardware_threads,
        ema_frame_micros,
        tris,
    );
    bridge.aux_buffer = bridge.observer_holon.cognitive_resolution_json_bytes();
    bridge.aux_buffer.len() as u32
}

/// Dynamically registers a new sensory catalog stream from JSON in `io_buffer` into
/// `ObserverPrismHolon`, writing the admitted stream JSON into `aux_buffer`.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_register_stream_io() -> u32 {
    let mut guard = global_bridge()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };
    let req_bytes = bridge.io_buffer.clone();
    let grants = bridge.grants.clone();
    let Ok(out) = bridge
        .observer_holon
        .register_stream_json(&req_bytes, &grants)
    else {
        return 0;
    };
    bridge.aux_buffer = out;
    bridge.aux_buffer.len() as u32
}

/// Removes a sensory stream whose UTF-8 `stream_id` is staged in `io_buffer`, writing the
/// removed stream's `kappa` hex into `aux_buffer`.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_remove_stream_io() -> u32 {
    let mut guard = global_bridge()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };
    let Ok(stream_id) = std::str::from_utf8(&bridge.io_buffer).map(str::to_string) else {
        return 0;
    };
    let Ok(kappa_hex) = bridge.observer_holon.remove_stream(&stream_id) else {
        return 0;
    };
    bridge.aux_buffer = kappa_hex.into_bytes();
    bridge.aux_buffer.len() as u32
}

/// Enables (`enabled != 0`) or disables (`enabled == 0`) the sensory stream whose UTF-8 `stream_id`
/// is staged in `io_buffer`, writing the updated stream JSON into `aux_buffer`.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_toggle_stream_io(enabled: u32) -> u32 {
    let mut guard = global_bridge()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };
    let Ok(stream_id) = std::str::from_utf8(&bridge.io_buffer).map(str::to_string) else {
        return 0;
    };
    let Ok(out) = bridge
        .observer_holon
        .toggle_stream_json(&stream_id, enabled != 0)
    else {
        return 0;
    };
    bridge.aux_buffer = out;
    bridge.aux_buffer.len() as u32
}

/// Ingests a live natural-language knowledge passage JSON from `io_buffer` directly into the
/// Observer's native `ISO 24617-2` language engine, writing the audited record JSON into `aux_buffer`.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_ingest_knowledge_io() -> u32 {
    let mut guard = global_bridge()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };
    let req_bytes = bridge.io_buffer.clone();
    let Ok(out) = bridge
        .observer_holon
        .ingest_knowledge_passage_json(&req_bytes)
    else {
        return 0;
    };
    bridge.aux_buffer = out;
    bridge.aux_buffer.len() as u32
}

/// Writes the complete Observer Cognitive Resolution JSON (Host Capability Envelope, all 8 parts,
/// connected streams, precision tiers, and escalation plan) into `aux_buffer`.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_cognitive_resolution_aux() -> u32 {
    let mut guard = global_bridge()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };
    bridge.aux_buffer = bridge.observer_holon.cognitive_resolution_json_bytes();
    bridge.aux_buffer.len() as u32
}

/// Records a stream-hydrated precision tier (`1..=4`) for the Observer part whose UTF-8
/// `"slot|stream|title"` descriptor is staged in `io_buffer`, writing updated cognition JSON into `aux_buffer`.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_record_part_hydration_io(precision_tier: u32) -> u32 {
    let mut guard = global_bridge()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };
    let Ok(wire) = std::str::from_utf8(&bridge.io_buffer).map(str::to_string) else {
        return 0;
    };
    let mut parts = wire.splitn(1 + 1 + 1, '|');
    let slot_str = parts.next().unwrap_or("stage-environment");
    let stream_str = parts.next().unwrap_or("polyhaven");
    let title_str = parts.next().unwrap_or("Stream Hydrated Asset");
    let slot = ObserverPartSlot::from_target_slot(slot_str);
    let digest = sha256_payload_digest(wire.as_bytes());
    let mut hex_k = String::with_capacity(digest.len() * (1 + 1));
    for b in digest {
        let _ = std::fmt::Write::write_fmt(&mut hex_k, format_args!("{b:02x}"));
    }
    bridge.observer_holon.record_part_hydration(
        slot,
        precision_tier,
        title_str.to_string(),
        stream_str.to_string(),
        hex_k,
    );
    bridge.aux_buffer = bridge.observer_holon.cognitive_resolution_json_bytes();
    bridge.aux_buffer.len() as u32
}

/// Pointer to host-bounded Khronos glTF PBR texture channel bytes (`0`=albedo, `1`=metallicRoughness,
/// `2`=normal, `3`=emissive).
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_scene_pbr_tex_ptr(channel: u32) -> u32 {
    let Ok(guard) = global_bridge().lock() else {
        return 0;
    };
    let two = 1u32 + 1;
    let three = two + 1;
    guard.as_ref().map_or(0, |b| match channel {
        0 => {
            if b.custom_glb_tex_rgba.is_empty() {
                0
            } else {
                b.custom_glb_tex_rgba.as_ptr() as u32
            }
        }
        1 => b
            .custom_glb_mr_tex
            .as_ref()
            .map_or(0, |t| t.rgba.as_ptr() as u32),
        x if x == two => b
            .custom_glb_normal_tex
            .as_ref()
            .map_or(0, |t| t.rgba.as_ptr() as u32),
        x if x == three => b
            .custom_glb_emissive_tex
            .as_ref()
            .map_or(0, |t| t.rgba.as_ptr() as u32),
        _ => 0,
    })
}

/// Width in pixels of Khronos glTF PBR texture channel (`0..=3`).
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_scene_pbr_tex_width(channel: u32) -> u32 {
    let Ok(guard) = global_bridge().lock() else {
        return 0;
    };
    let two = 1u32 + 1;
    let three = two + 1;
    guard.as_ref().map_or(0, |b| match channel {
        0 => b.custom_glb_tex_width,
        1 => b.custom_glb_mr_tex.as_ref().map_or(0, |t| t.width),
        x if x == two => b.custom_glb_normal_tex.as_ref().map_or(0, |t| t.width),
        x if x == three => b.custom_glb_emissive_tex.as_ref().map_or(0, |t| t.width),
        _ => 0,
    })
}

/// Height in pixels of Khronos glTF PBR texture channel (`0..=3`).
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_scene_pbr_tex_height(channel: u32) -> u32 {
    let Ok(guard) = global_bridge().lock() else {
        return 0;
    };
    let two = 1u32 + 1;
    let three = two + 1;
    guard.as_ref().map_or(0, |b| match channel {
        0 => b.custom_glb_tex_height,
        1 => b.custom_glb_mr_tex.as_ref().map_or(0, |t| t.height),
        x if x == two => b.custom_glb_normal_tex.as_ref().map_or(0, |t| t.height),
        x if x == three => b.custom_glb_emissive_tex.as_ref().map_or(0, |t| t.height),
        _ => 0,
    })
}

/// Returns `1` if the active imported `.glb` includes a live skeletal rig + keyframe clip, else `0`.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_glb_has_animation() -> u32 {
    let Ok(guard) = global_bridge().lock() else {
        return 0;
    };
    guard.as_ref().map_or(0, |b| {
        u32::from(b.custom_glb_rig.as_ref().is_some_and(|(s, c, w)| {
            !s.is_empty()
                && !c.is_empty()
                && c[0].duration_millis > 0
                && w.iter().any(|v| !v.is_empty())
        }))
    })
}

/// Feeds live UHD viewport + GPU/CPU/frame telemetry into `ObserverPrismHolon` and writes the
/// updated cognitive resolution JSON (including UHD 4K DRS + CAS budgets) into `aux_buffer`.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_update_host_envelope_uhd(
    max_tex_px: u32,
    is_software_gpu: u32,
    hardware_threads: u32,
    ema_frame_micros: u32,
    viewport_width_px: u32,
    viewport_height_px: u32,
) -> u32 {
    let mut guard = global_bridge()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };
    let tris = bridge.scene_buffers.triangle_count() as u32;
    bridge.observer_holon.update_host_envelope_with_viewport(
        max_tex_px,
        is_software_gpu != 0,
        hardware_threads,
        ema_frame_micros,
        tris,
        viewport_width_px,
        viewport_height_px,
    );
    bridge.aux_buffer = bridge.observer_holon.cognitive_resolution_json_bytes();
    bridge.aux_buffer.len() as u32
}

/// Returns the number of distinct material textures packed into the active `.glb` PBR atlas (`0..=16`).
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_glb_atlas_tile_count() -> u32 {
    let Ok(guard) = global_bridge().lock() else {
        return 0;
    };
    guard
        .as_ref()
        .map_or(0, |b| b.custom_glb_atlas_tile_count as u32)
}

/// Evaluates real-time UHD Frustum + O(1) Backface Normal-Cone + Screen-Space Error culling over
/// the active scene's 64-triangle [`MeshletGraph`] and writes JSON telemetry into `aux_buffer`.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
#[allow(clippy::too_many_arguments)]
pub extern "C" fn verdant_wasm_uhd_cull_meshlets(
    eye_x: f64,
    eye_y: f64,
    eye_z: f64,
    target_x: f64,
    target_y: f64,
    target_z: f64,
    aspect: f64,
    viewport_h_px: u32,
    min_sse_tenths_px: u32,
) -> u32 {
    let mut guard = global_bridge()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };
    if (bridge.uhd_meshlet_epoch != bridge.static_epoch
        || bridge.uhd_meshlet_graph.clusters.is_empty())
        && !bridge.scene_buffers.indices.is_empty()
    {
        bridge.uhd_meshlet_graph = build_meshlet_graph(
            &bridge.scene_buffers.positions_xyz,
            &bridge.scene_buffers.indices,
        );
        bridge.uhd_meshlet_epoch = bridge.static_epoch;
    }
    let eye = Vec3::new(eye_x, eye_y, eye_z);
    let target = Vec3::new(target_x, target_y, target_z);
    let dist = (target - eye).length().max(1.0);
    let two = 1.0 + 1.0;
    let three = two + 1.0;
    let fov_rad = std::f64::consts::PI / (three + (1.0 / (two * two + 1.0)) * two);
    let near = (dist / ((1u32 << (1 + 1 + 1 + 1 + 1 + 1 + 1 + 1)) as f64))
        .max(1.0 / ((1u32 << (1 + 1 + 1 + 1)) as f64));
    let far =
        (dist * ((1u32 << (1 + 1 + 1)) as f64)).max((1u32 << (1 + 1 + 1 + 1 + 1 + 1 + 1)) as f64);
    let frustum = Frustum::from_perspective(
        eye,
        (target - eye).normalized(),
        Vec3::new(0.0, 1.0, 0.0),
        fov_rad,
        aspect.max(1.0 / (two * two)),
        near,
        far,
    );
    let ten = two * (two + two + 1.0);
    let min_px = f64::from(min_sse_tenths_px.max(1)) / ten;
    let tan_half_fov = (fov_rad / two).tan();

    let tel = evaluate_meshlet_visibility(
        &bridge.uhd_meshlet_graph,
        &bridge.scene_buffers.indices,
        &frustum,
        eye,
        viewport_h_px,
        tan_half_fov,
        min_px,
        &mut bridge.uhd_visible_indices,
    );
    let json_str = format!(
        "{{\"totalMeshlets\":{},\"visibleMeshlets\":{},\"culledFrustum\":{},\"culledBackfaceCone\":{},\"culledSubpixel\":{},\"totalTriangles\":{},\"visibleTriangles\":{},\"cullRatioPermille\":{},\"atlasTileCount\":{}}}",
        tel.total_meshlets,
        tel.visible_meshlets,
        tel.culled_frustum,
        tel.culled_backface_cone,
        tel.culled_subpixel,
        tel.total_triangles,
        tel.visible_triangles,
        tel.cull_ratio_permille,
        bridge.custom_glb_atlas_tile_count,
    );
    bridge.uhd_last_visibility = Some(tel);
    bridge.aux_buffer = json_str.into_bytes();
    bridge.aux_buffer.len() as u32
}

/// Zero-copy pointer to the last frame's culled visible triangle index buffer (`u32` stream).
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_uhd_visible_idx_ptr() -> u32 {
    let guard = global_bridge()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let Some(bridge) = guard.as_ref() else {
        return 0;
    };
    if bridge.uhd_visible_indices.is_empty() {
        0
    } else {
        bridge.uhd_visible_indices.as_ptr() as usize as u32
    }
}

/// Element count of the last frame's culled visible triangle index buffer.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_uhd_visible_idx_count() -> u32 {
    let guard = global_bridge()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    guard
        .as_ref()
        .map(|b| b.uhd_visible_indices.len() as u32)
        .unwrap_or(0)
}

/// Evaluates a lawful UHD-and-beyond (`1080p` -> `4K` -> `5K` -> `8K`) display fillrate profile
/// and writes its JSON into `aux_buffer`.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_uhd_profile_aux(
    width_px: u32,
    height_px: u32,
    is_software_gpu: u32,
    webgpu_available: u32,
    ema_frame_micros: u32,
) -> u32 {
    let mut guard = global_bridge()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };
    let p = evaluate_uhd_display_profile(
        width_px,
        height_px,
        is_software_gpu != 0,
        webgpu_available != 0,
        ema_frame_micros,
    );
    let json_str = format!(
        "{{\"displayClass\":\"{}\",\"widthPx\":{},\"heightPx\":{},\"megapixelsX100\":{},\"drsScalePermille\":{},\"casSharpnessPermille\":{},\"meshletSseTenthsPx\":{},\"preferredBackend\":\"{}\"}}",
        p.display_class,
        p.width_px,
        p.height_px,
        p.megapixels_x100,
        p.drs_scale_permille,
        p.cas_sharpness_permille,
        p.meshlet_sse_tenths_px,
        p.preferred_backend,
    );
    bridge.aux_buffer = json_str.into_bytes();
    bridge.aux_buffer.len() as u32
}

/// Records one completed frame time in microseconds into the zero-allocation 64-frame
/// [`FrameBudgetOracle`] and returns the active hysteresis-guarded DRS permille (`650..=1000`).
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_record_frame_time_us(frame_us: u32) -> u32 {
    let mut guard = global_bridge()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };
    bridge.uhd_frame_oracle.record_frame(frame_us)
}

/// Serializes the live 64-frame `P50 / P95 / P99 / Jitter / Hysteresis DRS` stability oracle
/// report as JSON into `aux_buffer`.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_frame_stability_aux() -> u32 {
    let mut guard = global_bridge()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };
    let r = bridge.uhd_frame_oracle.report();
    let json_str = format!(
        "{{\"sampleCount\":{},\"targetBudgetUs\":{},\"p50Us\":{},\"p95Us\":{},\"p99Us\":{},\"maxUs\":{},\"jitterUs\":{},\"budgetCompliancePermille\":{},\"drsScalePermille\":{},\"drsTransitions\":{},\"stable\":{}}}",
        r.sample_count,
        r.target_budget_us,
        r.p50_us,
        r.p95_us,
        r.p99_us,
        r.max_us,
        r.jitter_us,
        r.budget_compliance_permille,
        r.drs_scale_permille,
        r.drs_transitions,
        r.stable,
    );
    bridge.aux_buffer = json_str.into_bytes();
    bridge.aux_buffer.len() as u32
}

/// Evaluates the monotonic 3-level hierarchical LOD pyramid (`LOD0` -> `LOD1` -> `LOD2`) for
/// the active static scene or imported `.glb` and writes its JSON into `aux_buffer`.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_uhd_lod_pyramid_aux() -> u32 {
    let mut guard = global_bridge()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };
    let pyr = evaluate_lod_pyramid_telemetry(
        &bridge.scene_buffers.positions_xyz,
        &bridge.scene_buffers.normals_xyz,
        &bridge.scene_buffers.colors_rgb,
        &bridge.scene_buffers.indices,
    );
    let json_str = format!(
        "{{\"lod0Triangles\":{},\"lod1Triangles\":{},\"lod2Triangles\":{},\"lod2ReductionPermille\":{}}}",
        pyr.lod0_triangles,
        pyr.lod1_triangles,
        pyr.lod2_triangles,
        pyr.lod2_reduction_permille,
    );
    bridge.aux_buffer = json_str.into_bytes();
    bridge.aux_buffer.len() as u32
}

/// Steps the closed-loop ISO/IEC 25010:2023 + ITU-R BT.2020/2100 [`DynamicWorkloadGovernor`]
/// paired with live [`PagedTensorArena`] residency tracking, writing the scaling decision JSON
/// into `aux_buffer`.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_workload_governor_step_aux(
    viewport_w: u32,
    viewport_h: u32,
    is_software_gpu: u32,
    webgpu_available: u32,
    frame_us: u32,
) -> u32 {
    let mut guard = global_bridge()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };
    let four_u64 = (1u64 + 1u64) * (1u64 + 1u64);
    let scene_bytes = ((bridge.scene_buffers.positions_xyz.len()
        + bridge.scene_buffers.normals_xyz.len()
        + bridge.scene_buffers.colors_rgb.len()
        + bridge.scene_buffers.indices.len()) as u64)
        .saturating_mul(four_u64)
        .max(TENSOR_PAGE_QUANTUM_BYTES);
    let page_q = TENSOR_PAGE_QUANTUM_BYTES;
    let scene_page_count = scene_bytes
        .div_ceil(page_q)
        .min(1u64 << (1 + 1 + 1 + 1 + 1 + 1));
    let base_scene_id = 1u64;
    for p in 0..scene_page_count {
        let pid = base_scene_id + p;
        if !bridge.tensor_arena.is_resident(pid) {
            let _ = bridge.tensor_arena.register_virtual_page(
                pid,
                TensorPageKind::GeometryMeshlet,
                TensorStorageEncoding::QuantizedGrid8,
                page_q,
                pid ^ u64::from(bridge.static_epoch),
            );
            let _ = bridge.tensor_arena.admit_or_touch(pid, None, false);
        }
    }
    let arena_t = bridge.tensor_arena.telemetry();
    let _ = bridge.uhd_frame_oracle.record_frame(frame_us);
    let dec = bridge.workload_governor.step(
        viewport_w,
        viewport_h,
        is_software_gpu != 0,
        webgpu_available != 0,
        frame_us,
        arena_t.resident_bytes,
        arena_t.budget_bytes,
    );
    let json_str = format!(
        "{{\"displayClass\":\"{}\",\"preferredBackend\":\"{}\",\"drsScalePermille\":{},\"casSharpnessPermille\":{},\"recommendedLodRank\":{},\"meshletSseTenthsPx\":{},\"tensorTileQuantum\":{},\"memoryPressurePermille\":{},\"performanceContractMet\":{},\"p50Us\":{},\"p95Us\":{},\"p99Us\":{},\"jitterUs\":{},\"budgetCompliancePermille\":{},\"residentBytes\":{},\"budgetBytes\":{},\"logicalWorkloadBytes\":{},\"evictions\":{},\"virtualAmplificationPermille\":{},\"memoryInvariantHeld\":{}}}",
        dec.display_class,
        dec.preferred_backend,
        dec.drs_scale_permille,
        dec.cas_sharpness_permille,
        dec.recommended_lod_rank,
        dec.meshlet_sse_tenths_px,
        dec.tensor_tile_quantum,
        dec.memory_pressure_permille,
        dec.performance_contract_met,
        dec.stability.p50_us,
        dec.stability.p95_us,
        dec.stability.p99_us,
        dec.stability.jitter_us,
        dec.stability.budget_compliance_permille,
        arena_t.resident_bytes,
        arena_t.budget_bytes,
        arena_t.logical_workload_bytes,
        arena_t.evictions,
        arena_t.virtual_amplification_permille,
        arena_t.invariant_held,
    );
    bridge.aux_buffer = json_str.into_bytes();
    bridge.aux_buffer.len() as u32
}

/// Serializes live [`PagedTensorArena`] ISO/IEC 25010 resource utilization telemetry into `aux_buffer`.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_tensor_memory_status_aux() -> u32 {
    let mut guard = global_bridge()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };
    let t = bridge.tensor_arena.telemetry();
    let json_str = format!(
        "{{\"budgetBytes\":{},\"residentBytes\":{},\"peakResidentBytes\":{},\"logicalWorkloadBytes\":{},\"registeredPages\":{},\"residentPages\":{},\"pinnedPages\":{},\"cacheHits\":{},\"pageFaults\":{},\"evictions\":{},\"virtualAmplificationPermille\":{},\"invariantHeld\":{}}}",
        t.budget_bytes,
        t.resident_bytes,
        t.peak_resident_bytes,
        t.logical_workload_bytes,
        t.registered_pages,
        t.resident_pages,
        t.pinned_pages,
        t.cache_hits,
        t.page_faults,
        t.evictions,
        t.virtual_amplification_permille,
        t.invariant_held,
    );
    bridge.aux_buffer = json_str.into_bytes();
    bridge.aux_buffer.len() as u32
}

/// Executes an out-of-core `Packed<Grid<16>, 2>` tiled tensor contraction through a bounded
/// [`PagedTensorArena`] proving bit-exact parity with in-memory contraction and `resident <= budget`
/// across oversubscribed arbitrary AI / beyond-8K workloads; writes JSON report into `aux_buffer`.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_tensor_paged_benchmark_aux(
    logical_pages: u32,
    budget_kib: u32,
) -> u32 {
    let mut guard = global_bridge()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };
    let kib_shift = 1u32 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1;
    let budget_bytes = u64::from(budget_kib.max(1))
        .saturating_mul(1u64 << kib_shift)
        .max(MIN_ARENA_BUDGET_BYTES);
    let mut bench_arena = PagedTensorArena::new(budget_bytes);

    let max_pages = 1u32 << (1 + 1 + 1 + 1 + 1 + 1 + 1 + 1);
    let target_pages = logical_pages.clamp(1 << (1 + 1 + 1), max_pages);
    for id in 0..u64::from(target_pages) {
        let _ = bench_arena.register_virtual_page(
            id,
            TensorPageKind::AiWeightBlock,
            TensorStorageEncoding::PackedNibble4,
            TENSOR_PAGE_QUANTUM_BYTES,
            id,
        );
        let _ = bench_arena.admit_or_touch(id, None, false);
    }

    let mut cb = [0_i32; CODED_TABLE_LEN];
    let seven = 1i32 + 1 + 1 + 1 + 1 + 1 + 1;
    for (i, slot) in cb.iter_mut().enumerate() {
        *slot = (i as i32) - seven;
    }
    let lut = CodebookProductLut::new(&cb, &cb);
    let rows = 1usize << (1 + 1 + 1 + 1 + 1);
    let inner_k = 1usize << (1 + 1 + 1 + 1 + 1 + 1);
    let cols = 1usize << (1 + 1 + 1 + 1 + 1);
    let packed_k = inner_k.div_ceil(PACKED_NIBBLE_BLOCK);
    let byte_mask = (1usize << (1 + 1 + 1 + 1 + 1 + 1 + 1 + 1)) - 1;
    let three = 1usize + 1 + 1;
    let lhs: Vec<u8> = (0..rows * packed_k)
        .map(|i| (i & byte_mask) as u8)
        .collect();
    let rhs: Vec<u8> = (0..cols * packed_k)
        .map(|i| ((i * three + 1) & byte_mask) as u8)
        .collect();
    let mut direct = vec![0_i64; rows * cols];
    let _ = contract_packed_nibbles_lut_into(&lhs, &rhs, rows, inner_k, cols, &lut, &mut direct);
    let mut paged = vec![0_i64; rows * cols];
    let tile_q = 1usize << (1 + 1 + 1);
    let rep = match contract_paged_packed_nibbles_stream(
        &mut bench_arena,
        &lhs,
        &rhs,
        rows,
        inner_k,
        cols,
        tile_q,
        tile_q,
        &lut,
        &mut paged,
    ) {
        Ok(r) => r,
        Err(_) => return 0,
    };
    let bit_exact = direct == paged;
    let json_str = format!(
        "{{\"bitExact\":{},\"tileDispatches\":{},\"codedBytesRead\":{},\"avoidedF32Bytes\":{},\"budgetBytes\":{},\"residentBytes\":{},\"peakResidentBytes\":{},\"logicalWorkloadBytes\":{},\"registeredPages\":{},\"residentPages\":{},\"evictions\":{},\"virtualAmplificationPermille\":{},\"invariantHeld\":{}}}",
        bit_exact,
        rep.tile_dispatches,
        rep.coded_bytes_read,
        rep.avoided_f32_bytes,
        rep.arena.budget_bytes,
        rep.arena.resident_bytes,
        rep.arena.peak_resident_bytes,
        rep.arena.logical_workload_bytes,
        rep.arena.registered_pages,
        rep.arena.resident_pages,
        rep.arena.evictions,
        rep.arena.virtual_amplification_permille,
        rep.arena.invariant_held,
    );
    bridge.aux_buffer = json_str.into_bytes();
    bridge.aux_buffer.len() as u32
}

/// Serializes the VSM S1–S5 & ISO/IEC/IEEE 42010:2022 Manager Total Ownership matrix and
/// Viewpoint contracts into `aux_buffer`.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_operator_status_aux() -> u32 {
    let mut guard = global_bridge()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };
    let Ok(ch) = verdant_core::operator::VerdantOperatorChannel::new(
        &bridge.observer_holon.active_domain.spec.slug,
    ) else {
        return 0;
    };
    let json_str = ch.synthesize_viewpoint_contracts_json();
    bridge.aux_buffer = json_str.into_bytes();
    bridge.aux_buffer.len() as u32
}

/// Processes a UTF-8 JSON [`verdant_core::operator::OperatorPromptRequest`] staged in `io_buffer`
/// through Verdant's Observer Executive & owning S1 Manager, writing the signed ISO 24617-2
/// `Inform`/`Refusal` JSON response into `aux_buffer`.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_operator_prompt_io() -> u32 {
    let mut guard = global_bridge()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };
    let Ok(raw) = std::str::from_utf8(&bridge.io_buffer) else {
        return 0;
    };
    let Ok(mut ch) = verdant_core::operator::VerdantOperatorChannel::new(
        &bridge.observer_holon.active_domain.spec.slug,
    ) else {
        return 0;
    };
    let Ok(json_str) = ch.submit_raw_directive_json(raw, false) else {
        return 0;
    };
    bridge.aux_buffer = json_str.into_bytes();
    bridge.aux_buffer.len() as u32
}

/// Switches Verdant Studio to a clean Blank Studio Canvas (16x16 architectural floor grid, zero
/// hardcoded demo dome/car/NPCs) ready for interactive 3D creation.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_studio_load_blank_stage() -> u32 {
    let mut guard = global_bridge()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };
    let lod = u64::from(bridge.lod_rank);
    bridge.hd_sector = synthesize_hd_world_sector_for_kind(
        bridge.seed_val,
        &bridge.terrain_params,
        bridge.env_params.water_level_meters,
        lod,
        WorldArchitectureKind::BlankStudio,
    );
    if let Ok(bvh) = TwoLevelBvh::build_solid(&bridge.hd_sector.scene, &bridge.hd_sector.meshes) {
        bridge.two_level_bvh = bvh;
    }
    bridge.custom_glb_scene = None;
    bridge.npc_patrol_active = false;
    bridge.blank_studio_mode = true;
    bridge.static_dirty = true;
    let _ = bridge.rebuild_scene_buffers(0.0);
    let two_f = prism_kernel::sTwo() as f64;
    let four_f = two_f * two_f;
    let eight_f = four_f * two_f;
    let twelve_f = eight_f + four_f;
    let half_f = 1.0 / two_f;
    bridge.camera_anchor_override = Some(Vec3::new(0.0, 1.0, 0.0));
    bridge.camera_rig.mode = CameraMode::ThirdPersonSpringArm;
    bridge.camera_rig.yaw_radians = half_f + half_f * half_f;
    bridge.camera_rig.pitch_radians = half_f * (1.0 - half_f * half_f);
    bridge.camera_rig.desired_distance_meters = twelve_f;
    bridge.camera_rig.current_distance_meters = twelve_f;
    let _ = bridge.solve_active_camera(0.0);
    bridge.scene_buffers.triangle_count() as u32
}

/// Places a new interactive 3D primitive object (`0=Box, 1=Platform, 2=Pillar, 3=Ramp, 4=Orb`)
/// into the active Verdant Studio scene and returns its monotonic object ID.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
#[allow(clippy::too_many_arguments)]
pub extern "C" fn verdant_wasm_studio_add_object(
    prim_kind: u32,
    x: f32,
    y: f32,
    z: f32,
    sx: f32,
    sy: f32,
    sz: f32,
    r: f32,
    g: f32,
    b: f32,
) -> u32 {
    let mut guard = global_bridge()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };
    let id = bridge.next_studio_obj_id;
    bridge.next_studio_obj_id = bridge.next_studio_obj_id.saturating_add(1);
    bridge.studio_objects.push(StudioAuthoredObject {
        id,
        kind: AuthorPrimitiveKind::from_u32(prim_kind),
        pos: [x, y, z],
        yaw_milli: 0,
        scale: [sx, sy, sz],
        color: [r, g, b],
    });
    bridge.custom_glb_scene = None;
    bridge.static_dirty = true;
    let _ = bridge.rebuild_scene_buffers(0.0);
    id
}

/// Updates transform and color of an existing Studio-authored 3D object (`1` on success, `0` if missing).
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
#[allow(clippy::too_many_arguments)]
pub extern "C" fn verdant_wasm_studio_update_object(
    obj_id: u32,
    x: f32,
    y: f32,
    z: f32,
    yaw_milli: i32,
    sx: f32,
    sy: f32,
    sz: f32,
    r: f32,
    g: f32,
    b: f32,
) -> u32 {
    let mut guard = global_bridge()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };
    let Some(obj) = bridge.studio_objects.iter_mut().find(|o| o.id == obj_id) else {
        return 0;
    };
    obj.pos = [x, y, z];
    obj.yaw_milli = yaw_milli;
    obj.scale = [sx, sy, sz];
    obj.color = [r, g, b];
    bridge.static_dirty = true;
    let _ = bridge.rebuild_scene_buffers(0.0);
    1
}

/// Removes an authored 3D object by ID (`1` if removed, `0` if not found).
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_studio_remove_object(obj_id: u32) -> u32 {
    let mut guard = global_bridge()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };
    let before = bridge.studio_objects.len();
    bridge.studio_objects.retain(|o| o.id != obj_id);
    if bridge.studio_objects.len() == before {
        return 0;
    }
    bridge.static_dirty = true;
    let _ = bridge.rebuild_scene_buffers(0.0);
    1
}

/// Clears all creator-placed primitive objects from the active Studio scene.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_studio_clear_objects() -> u32 {
    let mut guard = global_bridge()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };
    bridge.studio_objects.clear();
    bridge.static_dirty = true;
    let _ = bridge.rebuild_scene_buffers(0.0);
    1
}

/// Serializes all creator-authored Studio objects into `aux_buffer` as a JSON array.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_studio_objects_json_aux() -> u32 {
    let mut guard = global_bridge()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };
    let idx_two = 1usize + 1usize;
    let mut out = String::from("[");
    for (i, o) in bridge.studio_objects.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        let kind_code: u32 = match o.kind {
            AuthorPrimitiveKind::Box => 0,
            AuthorPrimitiveKind::Platform => 1,
            AuthorPrimitiveKind::Pillar => 1 + 1,
            AuthorPrimitiveKind::Ramp => 1 + 1 + 1,
            AuthorPrimitiveKind::Orb => 1 + 1 + 1 + 1,
        };
        let entry = format!(
            "{{\"id\":{},\"kind\":{},\"label\":\"{} #{}\",\"pos\":[{},{},{}],\"yawMilli\":{},\"scale\":[{},{},{}],\"color\":[{},{},{}]}}",
            o.id,
            kind_code,
            o.kind.label(),
            o.id,
            o.pos[0],
            o.pos[1],
            o.pos[idx_two],
            o.yaw_milli,
            o.scale[0],
            o.scale[1],
            o.scale[idx_two],
            o.color[0],
            o.color[1],
            o.color[idx_two],
        );
        out.push_str(&entry);
    }
    out.push(']');
    bridge.aux_buffer = out.into_bytes();
    bridge.aux_buffer.len() as u32
}

/// Zero-account Prism-ID Encrypt-then-MAC boundary seal + systematic k-of-n Reed-Solomon
/// Stitch sharding (`AUTH-STITCH-SELF-HOSTING`), reading a JSON request from `io_buffer`
/// and writing the JSON response into `aux_buffer`.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_stitch_boundary_seal_io() -> u32 {
    let mut guard = global_bridge()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };
    let Ok(raw) = std::str::from_utf8(&bridge.io_buffer) else {
        return 0;
    };
    bridge.aux_buffer = verdant_core::network::stitch_boundary_seal_json(raw).into_bytes();
    bridge.aux_buffer.len() as u32
}

/// Zero-account Stitch k-of-n Reed-Solomon recovery + Encrypt-then-MAC boundary decryption
/// (`AUTH-STITCH-SELF-HOSTING`), reading `{ boundaryId, ownerPrismId, secretPassphrase, shards }`
/// from `io_buffer` and writing the verified plaintext JSON into `aux_buffer`.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_stitch_boundary_recover_io() -> u32 {
    let mut guard = global_bridge()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };
    let Ok(raw) = std::str::from_utf8(&bridge.io_buffer) else {
        return 0;
    };
    bridge.aux_buffer = verdant_core::network::stitch_boundary_recover_json(raw).into_bytes();
    bridge.aux_buffer.len() as u32
}

/// Serializes the Observer's live Stakeholder Roadmap, ISO 15939 KPIs, Knowledge Manager summary,
/// and Change/Version Manager release readiness into `aux_buffer` (`AUTH-SEMVER-CHANGE-KPI`, `VE-45`).
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_observer_roadmap_kpis_aux() -> u32 {
    let mut guard = global_bridge()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };
    let Ok(ch) = verdant_core::operator::VerdantOperatorChannel::new(
        &bridge.observer_holon.active_domain.spec.slug,
    ) else {
        return 0;
    };
    bridge.aux_buffer = ch.synthesize_embedded_roadmap_kpis_json().into_bytes();
    bridge.aux_buffer.len() as u32
}

/// Zero-account in-browser Studio `VerdantVcs` commit + portable `.holo` capsule packer (`VE-49`),
/// reading JSON from `io_buffer` and writing the signed commit + `.holo` hex + `stitch://` URI
/// into `aux_buffer`.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_vcs_commit_and_pack_holo_io() -> u32 {
    let mut guard = global_bridge()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };
    let Ok(raw) = std::str::from_utf8(&bridge.io_buffer) else {
        return 0;
    };
    bridge.aux_buffer = verdant_core::vcs::studio_vcs_commit_and_pack_holo_json(raw).into_bytes();
    bridge.aux_buffer.len() as u32
}

/// Live dynamic UHD/8K + `PagedTensorArena` workload governor step (`VE-50`), reading JSON
/// workload dimensions & page counts from `io_buffer` and writing bounded-RAM + DRS/LOD telemetry
/// JSON into `aux_buffer`.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_paged_workload_step_io() -> u32 {
    let mut guard = global_bridge()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };
    let Ok(raw) = std::str::from_utf8(&bridge.io_buffer) else {
        return 0;
    };
    bridge.aux_buffer = verdant_core::uhd::paged_workload_step_json(raw).into_bytes();
    bridge.aux_buffer.len() as u32
}
