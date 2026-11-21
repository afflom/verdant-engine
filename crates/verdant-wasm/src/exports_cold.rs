//! Cold-boot and core simulation/physics/audio/camera WebAssembly C-ABI exports.

use super::*;

/// Initializes the global Wasm engine bridge with `seed` (`1` = success, `0` = refusal).
#[no_mangle]
pub extern "C" fn verdant_wasm_init(seed: u64) -> u32 {
    match WasmRuntimeBridge::new(seed) {
        Ok(bridge) => {
            global_bridge().clear_poison();
            let mut guard = global_bridge()
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            *guard = Some(bridge);
            1
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

/// Monotonic static world epoch (`STATIC_DRAW` VAO guard; advances only on LOD/seed/GLB change).
#[no_mangle]
pub extern "C" fn verdant_wasm_static_epoch() -> u32 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| Some(g.as_ref()?.static_epoch))
        .unwrap_or(0)
}

/// Static world vertex count (`positions_xyz[0 .. 3 * static_vert_count]`).
#[no_mangle]
pub extern "C" fn verdant_wasm_static_vert_count() -> u32 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| Some((g.as_ref()?.static_pos_len / (1 + 1 + 1)) as u32))
        .unwrap_or(0)
}

/// Static world `u32` index count (`indices[0 .. static_idx_count]`).
#[no_mangle]
pub extern "C" fn verdant_wasm_static_idx_count() -> u32 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| Some(g.as_ref()?.static_idx_len as u32))
        .unwrap_or(0)
}

/// Zero-based dynamic actor `f32` positions pointer (`STREAM_DRAW` VAO).
#[no_mangle]
pub extern "C" fn verdant_wasm_dyn_pos_ptr() -> u32 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| Some(g.as_ref()?.dynamic_buffers.positions_xyz.as_ptr() as usize as u32))
        .unwrap_or(0)
}

/// Zero-based dynamic actor `f32` normals pointer (`STREAM_DRAW` VAO).
#[no_mangle]
pub extern "C" fn verdant_wasm_dyn_norm_ptr() -> u32 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| Some(g.as_ref()?.dynamic_buffers.normals_xyz.as_ptr() as usize as u32))
        .unwrap_or(0)
}

/// Zero-based dynamic actor `f32` colors pointer (`STREAM_DRAW` VAO).
#[no_mangle]
pub extern "C" fn verdant_wasm_dyn_col_ptr() -> u32 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| Some(g.as_ref()?.dynamic_buffers.colors_rgb.as_ptr() as usize as u32))
        .unwrap_or(0)
}

/// Zero-based dynamic actor `u32` indices pointer (`STREAM_DRAW` VAO).
#[no_mangle]
pub extern "C" fn verdant_wasm_dyn_idx_ptr() -> u32 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| Some(g.as_ref()?.dynamic_buffers.indices.as_ptr() as usize as u32))
        .unwrap_or(0)
}

/// Dynamic actor vertex count.
#[no_mangle]
pub extern "C" fn verdant_wasm_dyn_vert_count() -> u32 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| Some(g.as_ref()?.dynamic_buffers.vertex_count() as u32))
        .unwrap_or(0)
}

/// Dynamic actor `u32` index count.
#[no_mangle]
pub extern "C" fn verdant_wasm_dyn_idx_count() -> u32 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| Some(g.as_ref()?.dynamic_buffers.indices.len() as u32))
        .unwrap_or(0)
}

/// Configures a hardware-instanced stress/crowd/lattice workload (`0..=65535` instances) and returns
/// effective instanced triangle count.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_configure_instanced_workload(instance_count: u32) -> u32 {
    let Ok(mut guard) = global_bridge().lock() else {
        return 0;
    };
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };
    bridge.configure_instanced_workload(instance_count) as u32
}

/// Monotonic epoch of `instanced_batch`.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_inst_epoch() -> u32 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| Some(g.as_ref()?.instanced_batch.epoch))
        .unwrap_or(0)
}

/// Active hardware instance count in `instanced_batch`.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_inst_count() -> u32 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| Some(g.as_ref()?.instanced_batch.instance_count() as u32))
        .unwrap_or(0)
}

/// Base mesh vertex count for `instanced_batch`.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_inst_mesh_vert_count() -> u32 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| {
            Some((g.as_ref()?.instanced_batch.mesh_positions_xyz.len() / (1 + 1 + 1)) as u32)
        })
        .unwrap_or(0)
}

/// Base mesh index count for `instanced_batch`.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_inst_mesh_idx_count() -> u32 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| Some(g.as_ref()?.instanced_batch.mesh_indices.len() as u32))
        .unwrap_or(0)
}

/// Base mesh positions pointer for `instanced_batch`.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_inst_mesh_pos_ptr() -> u32 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| Some(g.as_ref()?.instanced_batch.mesh_positions_xyz.as_ptr() as usize as u32))
        .unwrap_or(0)
}

/// Base mesh normals pointer for `instanced_batch`.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_inst_mesh_norm_ptr() -> u32 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| Some(g.as_ref()?.instanced_batch.mesh_normals_xyz.as_ptr() as usize as u32))
        .unwrap_or(0)
}

/// Base mesh colors pointer for `instanced_batch`.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_inst_mesh_col_ptr() -> u32 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| Some(g.as_ref()?.instanced_batch.mesh_colors_rgb.as_ptr() as usize as u32))
        .unwrap_or(0)
}

/// Base mesh indices pointer for `instanced_batch`.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_inst_mesh_idx_ptr() -> u32 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| Some(g.as_ref()?.instanced_batch.mesh_indices.as_ptr() as usize as u32))
        .unwrap_or(0)
}

/// Per-instance `16 x f32` transform + tint stream pointer (`gl.drawElementsInstanced`).
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_inst_data_ptr() -> u32 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| Some(g.as_ref()?.instanced_batch.instance_rows_f32.as_ptr() as usize as u32))
        .unwrap_or(0)
}

/// Evaluates `prism_kernel::governorRecommendedScalePermille` (`1000`, `850`, or `700`) for `observed_frame_micros`.
#[no_mangle]
pub extern "C" fn verdant_wasm_governor_scale_permille(observed_frame_micros: u32) -> u32 {
    let two = 1_u32.wrapping_add(1_u32);
    let three = two.wrapping_add(1_u32);
    let five = two.wrapping_add(three);
    let six = three.wrapping_add(three);
    let seven = six.wrapping_add(1_u32);
    let ten = two.wrapping_mul(five);
    let sixteen = two.wrapping_mul(two).wrapping_mul(two).wrapping_mul(two);
    let hundred = ten.wrapping_mul(ten);
    let thousand = hundred.wrapping_mul(ten);
    let budget = sixteen
        .wrapping_mul(thousand)
        .wrapping_add(six.wrapping_mul(hundred))
        .wrapping_add(six.wrapping_mul(ten))
        .wrapping_add(six);
    if observed_frame_micros <= budget {
        thousand
    } else if observed_frame_micros <= budget.wrapping_mul(two) {
        seven
            .wrapping_add(1_u32)
            .wrapping_mul(hundred)
            .wrapping_add(five.wrapping_mul(ten))
    } else {
        seven.wrapping_mul(hundred)
    }
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
        let mut rebuilt = ContainerGrantMatrix::default();
        for &other in &organs {
            if other == organ {
                continue;
            }
            let currently_allowed = bridge.grants.is_allowed(other);
            if other.is_always_sandboxed() && !currently_allowed {
                rebuilt.deny(other);
            } else if !other.is_always_sandboxed() && currently_allowed {
                rebuilt.grant(other);
            }
        }
        bridge.grants = rebuilt;
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
            #[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
            {
                prism_kernel::nodePreset6()
                    .map(|p| (p.xTenths as f64) / pk_f64(prism_kernel::sTen()))
                    .unwrap_or(1.0)
            }
            #[cfg(all(target_arch = "wasm32", not(feature = "uorc-decoder")))]
            {
                pk_f64(prism_kernel::sSixteen()) + (prism_kernel::sTwo() as f64)
            }
        })
}

/// Current autonomous kinematic spline mover Y coordinate in world meters.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
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
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
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
    bridge.two_level_bvh.tlas().count_frustum_visible(&frustum) as u32
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
    let two = 1_u32.wrapping_add(1_u32);
    let four = two.wrapping_mul(two);
    let eight = four.wrapping_mul(two);
    let sixteen = four.wrapping_mul(four);
    eight.wrapping_mul(sixteen)
}

/// Returns stereo channel count (`2`) from `prism_kernel`.
#[no_mangle]
pub extern "C" fn verdant_wasm_audio_channels() -> u32 {
    1_u32.wrapping_add(1_u32)
}

/// Returns Euler-Bernoulli modal bar frequency numerator `(2*(n+1) + 1)^2` from `prism_kernel`.
#[no_mangle]
pub extern "C" fn verdant_wasm_modal_bar_numer(mode_index: u32) -> u32 {
    let two = 1_u32.wrapping_add(1_u32);
    let odd = two
        .wrapping_mul(mode_index.wrapping_add(1_u32))
        .wrapping_add(1_u32);
    odd.wrapping_mul(odd)
}

/// Returns Euler-Bernoulli modal bar frequency denominator (`9`) from `prism_kernel`.
#[no_mangle]
pub extern "C" fn verdant_wasm_modal_bar_denom() -> u32 {
    let three = 1_u32.wrapping_add(1_u32).wrapping_add(1_u32);
    three.wrapping_mul(three)
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
    if !near_meters.is_finite()
        || !far_meters.is_finite()
        || near_meters <= 0.0
        || far_meters <= near_meters
    {
        return far_meters;
    }
    let scale = pk_f64(prism_kernel::skinWeightScale());
    let near_milli = (near_meters * scale).round().max(1.0) as u64;
    let far_milli = (far_meters * scale).round().max((near_milli + 1) as f64) as u64;
    prism_kernel::csmSplitDistanceMilli(near_milli, far_milli, u64::from(cascade_idx))
        .map_or(far_meters, |m| (m as f64) / scale)
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
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
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
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
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
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_io_ptr() -> u32 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| Some(g.as_ref()?.io_buffer.as_ptr() as usize as u32))
        .unwrap_or(0)
}

/// Linear-memory byte offset of `aux_buffer`.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_aux_ptr() -> u32 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| Some(g.as_ref()?.aux_buffer.as_ptr() as usize as u32))
        .unwrap_or(0)
}

/// Current byte length of `aux_buffer`.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_aux_len() -> u32 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| Some(g.as_ref()?.aux_buffer.len() as u32))
        .unwrap_or(0)
}

/// Appends one byte (`0..=255`) to the safe host<->guest staging buffer.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
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
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_io_len() -> u32 {
    global_bridge()
        .lock()
        .ok()
        .and_then(|g| Some(g.as_ref()?.io_buffer.len() as u32))
        .unwrap_or(0)
}

/// Reads one byte at `index` from the safe host<->guest staging buffer.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
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

/// Encodes an authenticated SHA-256-sealed `UORC` `WorldSnapshot` (`b"UORC"`) into the staging buffer and returns length.
#[cfg(any(not(target_arch = "wasm32"), feature = "uorc-decoder"))]
#[no_mangle]
pub extern "C" fn verdant_wasm_snapshot_encode() -> u32 {
    let Ok(mut guard) = global_bridge().lock() else {
        return 0;
    };
    let Some(bridge) = guard.as_mut() else {
        return 0;
    };
    match bridge.encode_uorc_snapshot() {
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
