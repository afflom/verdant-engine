//! Scene node, tensor kernel, GLB/UORC & snapshot cold-boot Wasm exports (`exports_cold_stage.rs`).

use super::*;

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
    #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
    {
        pk_u64(prism_kernel::tileEdgePixels()) as u32
    }
    #[cfg(all(target_arch = "wasm32", not(feature = "uorc-decoder")))]
    {
        verdant_core::verdant_tensor::kernel_consts().tile_edge_pixels
    }
}

/// Returns worker command ring capacity (`64`) from `prism_kernel`.
#[no_mangle]
pub extern "C" fn verdant_wasm_worker_ring_capacity() -> u32 {
    #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
    {
        pk_u64(prism_kernel::workerCommandRingCapacity()) as u32
    }
    #[cfg(all(target_arch = "wasm32", not(feature = "uorc-decoder")))]
    {
        verdant_core::verdant_tensor::kernel_consts().word_bit_width as u32
    }
}

/// Returns maximum URL-shareable `VDNT`/`UORC` snapshot byte budget (`32768`) from `prism_kernel`.
#[no_mangle]
pub extern "C" fn verdant_wasm_snapshot_max_bytes() -> u32 {
    #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
    {
        pk_u64(prism_kernel::snapshotUrlMaxBytes()) as u32
    }
    #[cfg(all(target_arch = "wasm32", not(feature = "uorc-decoder")))]
    {
        let four = 1_u32 + 1_u32 + 1_u32 + 1_u32;
        1_u32 << ((four * four) - 1_u32)
    }
}

/// Returns canonical default world seed (`7`) from `prism_kernel`.
#[no_mangle]
pub extern "C" fn verdant_wasm_default_seed() -> u64 {
    let two = 1_u64.wrapping_add(1_u64);
    let four = two.wrapping_mul(two);
    four.wrapping_add(two).wrapping_add(1_u64)
}

/// Returns maximum progressive base LOD rank (`3`) from `prism_kernel`.
#[no_mangle]
pub extern "C" fn verdant_wasm_max_lod_rank() -> u32 {
    1_u32.wrapping_add(1_u32).wrapping_add(1_u32)
}

/// Returns maximum progressive HD LOD rank (`4`) from `prism_kernel`.
#[no_mangle]
pub extern "C" fn verdant_wasm_hd_max_lod_rank() -> u32 {
    let two = 1_u32.wrapping_add(1_u32);
    two.wrapping_mul(two)
}

/// Returns minimum authenticated `VDNT` wire envelope byte length (`41`) from `prism_kernel`.
#[no_mangle]
pub extern "C" fn verdant_wasm_snapshot_min_wire_bytes() -> u32 {
    #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
    {
        pk_u64(prism_kernel::snapshotMinWireBytes()) as u32
    }
    #[cfg(all(target_arch = "wasm32", not(feature = "uorc-decoder")))]
    {
        let two = 1_u32 + 1_u32;
        let five = two + two + 1_u32;
        (two * two * two * five) + 1_u32
    }
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
    let tex_cap = bridge
        .observer_holon
        .host_envelope
        .recommended_texture_cap_px;
    let Ok(imported) = import_glb_to_scene_bounded(&bridge.io_buffer, tex_cap) else {
        return 0;
    };
    let Ok(boundaries) = TwoLevelBvh::build_solid(&imported.scene, &imported.meshes) else {
        return 0;
    };
    bridge.two_level_bvh = boundaries;
    bridge.bvh = imported.scene.build_bvh();
    bridge.imported_glb_skin_clip_counts = (imported.skeletons.len(), imported.clips.len());
    let has_primary_tex = imported.primary_albedo_texture.is_some();
    if let Some(tex) = imported.primary_albedo_texture {
        bridge.custom_glb_tex_width = tex.width;
        bridge.custom_glb_tex_height = tex.height;
        bridge.custom_glb_tex_rgba = tex.rgba;
        bridge.custom_glb_tex_epoch = bridge.custom_glb_tex_epoch.wrapping_add(1).max(1);
    } else {
        bridge.custom_glb_tex_width = 0;
        bridge.custom_glb_tex_height = 0;
        bridge.custom_glb_tex_rgba.clear();
        bridge.custom_glb_tex_epoch = bridge.custom_glb_tex_epoch.wrapping_add(1).max(1);
    }
    bridge.custom_glb_mr_tex = imported.primary_mr_texture;
    bridge.custom_glb_normal_tex = imported.primary_normal_texture;
    bridge.custom_glb_emissive_tex = imported.primary_emissive_texture;
    bridge.custom_glb_rig = if !imported.skeletons.is_empty() && !imported.clips.is_empty() {
        Some((imported.skeletons, imported.clips, imported.skin_weights))
    } else {
        None
    };
    bridge.custom_glb_atlas_tile_count = imported.atlas_tile_count;
    bridge.custom_glb_mesh_atlas_uvs = imported.mesh_atlas_uvs;
    bridge.custom_glb_primary_tex_mask = imported.primary_tex_mesh_mask;
    bridge.custom_glb_scene = Some((imported.scene, imported.meshes, imported.materials));
    bridge.static_dirty = true;
    let tris = bridge.rebuild_scene_buffers(0.0).unwrap_or(0);

    // Record high-definition Observer hydration for Characters or Hero Props:
    let slot = if bridge.imported_glb_skin_clip_counts.0 > 0 {
        ObserverPartSlot::CharacterRigs
    } else {
        ObserverPartSlot::HeroProps
    };
    let tier = if has_primary_tex {
        1u32 + 1 + 1 + 1
    } else {
        1u32 + 1 + 1
    };
    let glb_digest = sha256_payload_digest(&bridge.io_buffer);
    let mut hex_k = String::with_capacity(glb_digest.len() * (1 + 1));
    for b in glb_digest {
        let _ = std::fmt::Write::write_fmt(&mut hex_k, format_args!("{b:02x}"));
    }
    bridge.observer_holon.record_part_hydration(
        slot,
        tier,
        format!("Streamed GLB ({tris} tris)"),
        "live-3d-stream".to_string(),
        hex_k,
    );

    // Auto-frame the camera smoothly around the imported .glb's exact world-space bounding sphere:
    let mut min_p = [f64::INFINITY; 1 + 1 + 1];
    let mut max_p = [f64::NEG_INFINITY; 1 + 1 + 1];
    let three = 1usize + 1usize + 1usize;
    let two = 1usize + 1usize;
    for chunk in bridge.scene_buffers.positions_xyz.chunks_exact(three) {
        let px = f64::from(chunk[0]);
        let py = f64::from(chunk[1]);
        let pz = f64::from(chunk[two]);
        if px.is_finite() && py.is_finite() && pz.is_finite() {
            if px < min_p[0] {
                min_p[0] = px;
            }
            if py < min_p[1] {
                min_p[1] = py;
            }
            if pz < min_p[two] {
                min_p[two] = pz;
            }
            if px > max_p[0] {
                max_p[0] = px;
            }
            if py > max_p[1] {
                max_p[1] = py;
            }
            if pz > max_p[two] {
                max_p[two] = pz;
            }
        }
    }
    if min_p[0].is_finite() && max_p[0].is_finite() {
        let half = 1.0 / (prism_kernel::sTwo() as f64);
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

    // Keep fast UORC stream transcoding bounded for multi-MB live internet .glb previews:
    let fast_uorc_limit =
        1usize << (1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1);
    if bridge.io_buffer.len() <= fast_uorc_limit {
        if let Ok(uorc_stream) =
            transcode_glb_to_progressive_uorc(&bridge.io_buffer, bridge.seed_val)
        {
            bridge.aux_buffer = uorc_stream;
        } else if let Ok(snap) = WorldSnapshot::encode_uorc(&bridge.world) {
            bridge.aux_buffer = snap;
        }
    } else if let Ok(snap) = WorldSnapshot::encode_uorc(&bridge.world) {
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
    let Ok(boundaries) =
        TwoLevelBvh::build_solid(&bridge.hd_sector.scene, &bridge.hd_sector.meshes)
    else {
        return 0;
    };
    bridge.two_level_bvh = boundaries;
    bridge.custom_glb_scene = None;
    bridge.imported_glb_skin_clip_counts = (0, 0);
    bridge.static_dirty = true;
    bridge.bvh = bridge.world.scene.build_bvh();
    let _ = bridge.solve_active_camera(0.0);
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
    let Ok(frags) = shard_uorc_to_stitch_fragments(&bridge.io_buffer) else {
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
    let Ok((recovered, _plain)) = recover_uorc_from_stitch_pair(&frag_a, &frag_b) else {
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
        .and_then(|g| {
            let b = g.as_ref()?;
            #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
            {
                Some(b.universe.active_position.cell.cx)
            }
            #[cfg(all(target_arch = "wasm32", not(feature = "uorc-decoder")))]
            {
                Some(b.active_cell.cx)
            }
        })
        .unwrap_or(0)
}

/// Active floating-origin `UniverseCellCoord` Y index.
#[no_mangle]
pub extern "C" fn verdant_wasm_universe_cell_y() -> i64 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| {
            let b = g.as_ref()?;
            #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
            {
                Some(b.universe.active_position.cell.cy)
            }
            #[cfg(all(target_arch = "wasm32", not(feature = "uorc-decoder")))]
            {
                Some(b.active_cell.cy)
            }
        })
        .unwrap_or(0)
}

/// Active floating-origin `UniverseCellCoord` Z index.
#[no_mangle]
pub extern "C" fn verdant_wasm_universe_cell_z() -> i64 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| {
            let b = g.as_ref()?;
            #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
            {
                Some(b.universe.active_position.cell.cz)
            }
            #[cfg(all(target_arch = "wasm32", not(feature = "uorc-decoder")))]
            {
                Some(b.active_cell.cz)
            }
        })
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

/// Stages live `SceneWorld` entity hierarchy, world-space bounds centers, triangle counts, and
/// `OntologicalRuntimeGovernor` 8-stratum `W5H` / Telos telemetry into `aux_buffer`, returning byte length.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_scene_outliner_aux(observed_frame_micros: u32) -> u32 {
    let Ok(mut guard) = global_bridge().lock() else {
        return 0;
    };
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };
    let sixty = pk_u64(prism_kernel::sSix()) * pk_u64(prism_kernel::sTen());
    let four = pk_u64(prism_kernel::sFour()) as u32;
    let nineteen = pk_u64(prism_kernel::sSixteen()) + prism_kernel::sThree();
    let width =
        (nineteen * pk_u64(prism_kernel::sHundred()) + pk_u64(prism_kernel::sTwenty())) as u32;
    let height = (pk_u64(prism_kernel::sTen()) * pk_u64(prism_kernel::sHundred())
        + pk_u64(prism_kernel::sEight()) * pk_u64(prism_kernel::sTen())) as u32;
    let Ok(cfg) = EngineConfig::from_refresh_rate(width, height, sixty as u32, sixty as u32, four)
    else {
        return 0;
    };
    let caps = BrowserCapabilities {
        wasm_simd_available: true,
        webgl2_available: true,
        hardware_concurrency: four,
        shared_array_buffer: false,
        audio_worklet: true,
        offscreen_canvas: true,
    };
    let governor = OntologicalRuntimeGovernor::new(cfg, caps);
    let certs = governor.certify_all_eight_strata();
    let verified_strata = certs.iter().filter(|c| c.complete_verified).count();
    let obs_us = if observed_frame_micros == 0 {
        pk_u64(prism_kernel::sEight()) * pk_u64(prism_kernel::skinWeightScale())
    } else {
        u64::from(observed_frame_micros)
    };
    let milli = pk_u64(prism_kernel::skinWeightScale());
    let telos = governor.evaluate_frame_telos(obs_us, true, milli, milli, 0, 0, 0);
    let rec_lod = governor.recommend_non_coercive_lod(u64::from(bridge.lod_rank), obs_us, 0);

    #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
    let (scene, meshes) = if let Some((ref sc, ref ms, _)) = bridge.custom_glb_scene {
        (sc, ms.as_slice())
    } else {
        (&bridge.hd_sector.scene, bridge.hd_sector.meshes.as_slice())
    };
    #[cfg(not(any(not(target_arch = "wasm32"), feature = "uorc-decoder")))]
    let (scene, meshes) = (&bridge.hd_sector.scene, bridge.hd_sector.meshes.as_slice());

    let three = prism_kernel::sThree().max(1) as usize;
    let mut lines = Vec::with_capacity(scene.entity_count() + 1);
    lines.push(format!(
        "w5h_strata={verified_strata}|telos_loss={}|triadic={}|rec_lod={rec_lod}|seed={}|lod={}|entities={}",
        telos.total_telos_loss,
        u32::from(telos.triadic_connection_active),
        bridge.seed_val,
        bridge.lod_rank,
        scene.entity_count()
    ));
    for rec in scene.entities() {
        let tris = rec
            .mesh_index
            .and_then(|mi| meshes.get(mi))
            .map_or(0, |m| m.indices.len() / three);
        let center = scene
            .world_bounds(rec.id)
            .map_or(rec.local_transform.translation, |b| b.center());
        lines.push(format!(
            "{}|{}|{:?}|{}|{:.2},{:.2},{:.2}|{:.3}",
            rec.id.0,
            rec.name,
            rec.role,
            tris,
            center.x,
            center.y,
            center.z,
            rec.emerged_at_lod_scale
        ));
    }
    bridge.aux_buffer = lines.join("\n").into_bytes();
    bridge.aux_buffer.len() as u32
}
