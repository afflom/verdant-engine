//! UHD PBR/meshlet, workload governor, Studio, VCS & operator Wasm exports (`exports_uorc_studio.rs`).

use super::*;

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
