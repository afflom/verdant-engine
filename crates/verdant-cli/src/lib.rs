//! Command-line asset validator, progressive `UORC` + `.glb` + `.vdnt` scene packer, and headless
//! 60fps frame-budget benchmark for Verdant Engine (`verdant-cli`).

#![forbid(unsafe_code)]

use std::fmt;
use std::path::Path;
use std::time::Instant;

use verdant_core::collision::{gjk_intersect_and_manifold, ConvexHullCollider};
use verdant_core::geometry::canonical_scene_mesh_library;
use verdant_core::gltf_io::{export_scene_to_glb, validate_glb_bytes};
use verdant_core::math::Vec3;
use verdant_core::navmesh::NavMesh;
use verdant_core::physics::{
    CapsuleController, KinematicSplineBody, PhysicsWorld, PhysicsWorldConfig,
};
use verdant_core::prism_kernel;
use verdant_core::render::{
    canonical_pbr_material_library, compute_csm_splits, validate_builtin_engine_shaders,
    validate_wgsl_shader, RenderFrameGraph,
};
use verdant_core::runtime::WorldSnapshot;
use verdant_core::streaming::{
    pack_progressive_uorc_stream, verify_progressive_uorc_stream, StandardSceneState,
};

/// Refusal returned when a `verdant` CLI subcommand receives invalid arguments, unreadable paths,
/// or a non-conformant asset/shader/snapshot/UORC payload.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CliCommandError {
    /// Unknown or missing subcommand or required positional argument.
    InvalidUsage {
        /// Diagnostic message describing expected subcommand syntax.
        usage: String,
    },
    /// Filesystem read or write failed for a user-supplied path.
    IoFailure {
        /// Path that failed.
        path: String,
        /// Underlying OS diagnostic.
        reason: String,
    },
    /// Asset, WGSL shader, LOD rank, `UORC` stream, or `VDNT` snapshot failed engine validation.
    ValidationRejected {
        /// Subcommand that rejected the input.
        command: String,
        /// Engine refusal diagnostic.
        diagnostic: String,
    },
}

impl fmt::Display for CliCommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidUsage { usage } => write!(f, "invalid CLI usage: {usage}"),
            Self::IoFailure { path, reason } => write!(f, "I/O failure on `{path}`: {reason}"),
            Self::ValidationRejected {
                command,
                diagnostic,
            } => write!(f, "{command} rejected input: {diagnostic}"),
        }
    }
}

impl std::error::Error for CliCommandError {}

/// Usage summary printed on `help` or invalid command invocation.
pub const CLI_USAGE: &str = "\
verdant <subcommand> [options]
  validate-glb <file.glb>                     Verify binary Khronos glTF 2.0 container
  validate-wgsl <file.wgsl> [entry...]        Verify W3C WGSL shader module via naga
  pack-scene <out.glb> [seed] [lod_rank]      Pack progressive .uorc + .glb + .vdnt scene bundle
  verify-uorc <file.uorc>                     Verify UORC-SHA256-CHAIN-v1 multi-LOD stream
  snapshot-verify <file.vdnt>                 Verify VDNT + SHA-256 snapshot and Prism envelope
  benchmark [frames]                          Run headless 60fps physics + BVH + GJK + NavMesh benchmark";

fn pk(command: &str, res: Result<u64, prism_kernel::ComputeError>) -> Result<u64, CliCommandError> {
    res.map_err(|e| CliCommandError::ValidationRejected {
        command: command.to_string(),
        diagnostic: format!("prism_kernel compute error: {e:?}"),
    })
}

/// Executes a `verdant` CLI command over `args` (excluding `argv[0]`) and returns formatted output.
pub fn run_cli(args: &[String]) -> Result<String, CliCommandError> {
    let Some(cmd) = args.first().map(String::as_str) else {
        return Err(CliCommandError::InvalidUsage {
            usage: CLI_USAGE.to_string(),
        });
    };

    match cmd {
        "help" | "--help" | "-h" => Ok(CLI_USAGE.to_string()),
        "validate-glb" => {
            let Some(path) = args.get(1) else {
                return Err(CliCommandError::InvalidUsage {
                    usage: "verdant validate-glb <file.glb>".to_string(),
                });
            };
            let bytes = std::fs::read(path).map_err(|e| CliCommandError::IoFailure {
                path: path.clone(),
                reason: e.to_string(),
            })?;
            let summary =
                validate_glb_bytes(&bytes).map_err(|e| CliCommandError::ValidationRejected {
                    command: "validate-glb".to_string(),
                    diagnostic: e.to_string(),
                })?;
            Ok(format!(
                "GLB OK: path={path} bytes={} nodes={} meshes={} materials={} buffer_bytes={}",
                bytes.len(),
                summary.node_count,
                summary.mesh_count,
                summary.material_count,
                summary.total_buffer_bytes
            ))
        }
        "validate-wgsl" => {
            let Some(path) = args.get(1) else {
                return Err(CliCommandError::InvalidUsage {
                    usage: "verdant validate-wgsl <file.wgsl> [entry...]".to_string(),
                });
            };
            let src = std::fs::read_to_string(path).map_err(|e| CliCommandError::IoFailure {
                path: path.clone(),
                reason: e.to_string(),
            })?;
            let skip_args = prism_kernel::sTwo() as usize;
            let req_entries: Vec<&str> = args.iter().skip(skip_args).map(String::as_str).collect();
            let label = Path::new(path)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("shader.wgsl");
            let verified = validate_wgsl_shader(label, &src, &req_entries).map_err(|e| {
                CliCommandError::ValidationRejected {
                    command: "validate-wgsl".to_string(),
                    diagnostic: e.to_string(),
                }
            })?;
            Ok(format!(
                "WGSL OK: label={} entries={} globals={} expressions={}",
                verified.label,
                verified.entry_points.join(","),
                verified.global_binding_count,
                verified.expression_count
            ))
        }
        "pack-scene" => {
            let Some(out_glb) = args.get(1) else {
                return Err(CliCommandError::InvalidUsage {
                    usage: "verdant pack-scene <out.glb> [seed] [lod_rank]".to_string(),
                });
            };
            let seed_arg_idx = prism_kernel::sTwo() as usize;
            let lod_arg_idx = prism_kernel::sThree() as usize;
            let seed = match args.get(seed_arg_idx) {
                Some(raw) => raw
                    .parse::<u64>()
                    .map_err(|_| CliCommandError::InvalidUsage {
                        usage: format!("invalid u64 seed `{raw}`"),
                    })?,
                None => pk("pack-scene", prism_kernel::wasmDefaultSeed())?,
            };
            let lod_rank = match args.get(lod_arg_idx) {
                Some(raw) => raw
                    .parse::<u64>()
                    .map_err(|_| CliCommandError::InvalidUsage {
                        usage: format!("invalid u64 lod_rank `{raw}`"),
                    })?,
                None => prism_kernel::wasmMaxLodRank(),
            };
            let mut world = StandardSceneState::synthesize_at_lod(seed, lod_rank).map_err(|e| {
                CliCommandError::ValidationRejected {
                    command: "pack-scene".to_string(),
                    diagnostic: e.to_string(),
                }
            })?;
            let meshes = canonical_scene_mesh_library(world.seed);
            let materials = canonical_pbr_material_library();
            let entities: Vec<_> = world.scene.entities().map(|e| e.id).collect();
            for (idx, entity_id) in entities.into_iter().take(meshes.len()).enumerate() {
                world
                    .scene
                    .attach_renderable(entity_id, idx, idx % materials.len());
            }
            let glb_bytes =
                export_scene_to_glb(&world.scene, &meshes, &materials).map_err(|e| {
                    CliCommandError::ValidationRejected {
                        command: "pack-scene".to_string(),
                        diagnostic: e.to_string(),
                    }
                })?;
            let snap_bytes =
                WorldSnapshot::encode(&world).map_err(|e| CliCommandError::ValidationRejected {
                    command: "pack-scene".to_string(),
                    diagnostic: e.to_string(),
                })?;
            let uorc_bytes = pack_progressive_uorc_stream(seed).map_err(|e| {
                CliCommandError::ValidationRejected {
                    command: "pack-scene".to_string(),
                    diagnostic: e.to_string(),
                }
            })?;
            std::fs::write(out_glb, &glb_bytes).map_err(|e| CliCommandError::IoFailure {
                path: out_glb.clone(),
                reason: e.to_string(),
            })?;
            let out_vdnt = format!("{out_glb}.vdnt");
            std::fs::write(&out_vdnt, &snap_bytes).map_err(|e| CliCommandError::IoFailure {
                path: out_vdnt.clone(),
                reason: e.to_string(),
            })?;
            let out_uorc = format!("{out_glb}.uorc");
            std::fs::write(&out_uorc, &uorc_bytes).map_err(|e| CliCommandError::IoFailure {
                path: out_uorc.clone(),
                reason: e.to_string(),
            })?;
            Ok(format!(
                "PACKED OK: glb={out_glb} ({}B) vdnt={out_vdnt} ({}B) uorc={out_uorc} ({}B) lod={} nodes={}",
                glb_bytes.len(),
                snap_bytes.len(),
                uorc_bytes.len(),
                world.active_lod.rank(),
                world.scene_nodes.len()
            ))
        }
        "verify-uorc" => {
            let Some(path) = args.get(1) else {
                return Err(CliCommandError::InvalidUsage {
                    usage: "verdant verify-uorc <file.uorc>".to_string(),
                });
            };
            let stream = std::fs::read(path).map_err(|e| CliCommandError::IoFailure {
                path: path.clone(),
                reason: e.to_string(),
            })?;
            let summary = verify_progressive_uorc_stream(&stream).map_err(|e| {
                CliCommandError::ValidationRejected {
                    command: "verify-uorc".to_string(),
                    diagnostic: e.to_string(),
                }
            })?;
            Ok(format!(
                "UORC OK: path={path} compressed_bytes={} decoded_bytes={} frames={} lod_objects={} max_lod={}",
                summary.compressed_bytes,
                summary.decoded_total_bytes,
                summary.archive_frame_count,
                summary.lod_object_count,
                summary.max_lod_rank
            ))
        }
        "snapshot-verify" => {
            let Some(path) = args.get(1) else {
                return Err(CliCommandError::InvalidUsage {
                    usage: "verdant snapshot-verify <file.vdnt>".to_string(),
                });
            };
            let wire = std::fs::read(path).map_err(|e| CliCommandError::IoFailure {
                path: path.clone(),
                reason: e.to_string(),
            })?;
            let quantum = prism_kernel::audioWorkletQuantumFrames().map_err(|e| {
                CliCommandError::ValidationRejected {
                    command: "snapshot-verify".to_string(),
                    diagnostic: format!("{e:?}"),
                }
            })?;
            let channels = prism_kernel::audioStereoChannels();
            let url_safe =
                prism_kernel::wasmBridgeEnvelopeValid(quantum, channels, wire.len() as u64)
                    .unwrap_or(false);
            let world =
                WorldSnapshot::decode(&wire).map_err(|e| CliCommandError::ValidationRejected {
                    command: "snapshot-verify".to_string(),
                    diagnostic: e.to_string(),
                })?;
            Ok(format!(
                "VDNT OK: path={path} bytes={} seed={} lod={} scale={:.3} nodes={} url_bridge_ready={}",
                wire.len(),
                world.seed,
                world.active_lod.rank(),
                world.resolution_scale,
                world.scene_nodes.len(),
                url_safe
            ))
        }
        "benchmark" => {
            let two = prism_kernel::sTwo();
            let four = pk("benchmark", prism_kernel::sFour())?;
            let five = pk("benchmark", prism_kernel::sFive())?;
            let seven = pk("benchmark", prism_kernel::wasmDefaultSeed())?;
            let eight = pk("benchmark", prism_kernel::sEight())?;
            let twelve = pk("benchmark", prism_kernel::sTwelve())?;
            let sixty = pk("benchmark", prism_kernel::sSixty())?;
            let default_frames =
                pk("benchmark", prism_kernel::cliDefaultBenchmarkFrames())? as usize;
            let max_frames = pk("benchmark", prism_kernel::cliMaxBenchmarkFrames())? as usize;
            let frames = match args.get(1) {
                Some(raw) => raw
                    .parse::<usize>()
                    .map_err(|_| CliCommandError::InvalidUsage {
                        usage: format!("invalid frame count `{raw}`"),
                    })?,
                None => default_frames,
            };
            if frames == 0 || frames > max_frames {
                return Err(CliCommandError::InvalidUsage {
                    usage: format!("benchmark frame count must be in 1..={max_frames}"),
                });
            }
            let shaders = validate_builtin_engine_shaders().map_err(|e| {
                CliCommandError::ValidationRejected {
                    command: "benchmark".to_string(),
                    diagnostic: e.to_string(),
                }
            })?;
            let fg = RenderFrameGraph::canonical_pbr_shadow_pipeline();
            let pass_count = fg
                .validate()
                .map_err(|e| CliCommandError::ValidationRejected {
                    command: "benchmark".to_string(),
                    diagnostic: e.to_string(),
                })?;
            let near_m = 1.0 / (four as f64);
            let far_m = pk("benchmark", prism_kernel::wasmFrustumFarMeters())? as f64;
            let csm = compute_csm_splits(near_m, far_m).map_err(|e| {
                CliCommandError::ValidationRejected {
                    command: "benchmark".to_string(),
                    diagnostic: e.to_string(),
                }
            })?;

            let world = StandardSceneState::synthesize_at_lod(seven, prism_kernel::wasmMaxLodRank())
                .map_err(|e| CliCommandError::ValidationRejected {
                    command: "benchmark".to_string(),
                    diagnostic: e.to_string(),
                })?;
            let meshes = canonical_scene_mesh_library(world.seed);
            let nav = NavMesh::from_mesh(&meshes[0]).map_err(|e| {
                CliCommandError::ValidationRejected {
                    command: "benchmark".to_string(),
                    diagnostic: e.to_string(),
                }
            })?;
            let half_unit = 1.0 / (two as f64);
            let hull_a = ConvexHullCollider::from_box_half_extents(Vec3::new(1.0, 1.0, 1.0))
                .expect("valid hull a");
            let hull_b =
                ConvexHullCollider::from_box_half_extents(Vec3::new(half_unit, 1.0, half_unit))
                    .expect("valid hull b");
            let mut physics = PhysicsWorld::new(PhysicsWorldConfig {
                terrain_seed: world.seed,
                ..Default::default()
            });
            physics.player_capsule = Some(CapsuleController::new(Vec3::new(
                0.0,
                two as f64,
                eight as f64,
            )));
            let mover_speed = ((twelve + 1) as f64) / (five as f64);
            physics.kinematic_mover = Some(KinematicSplineBody::new(
                world.mover_spline_waypoints.clone(),
                mover_speed,
            ));

            let dt = 1.0 / (sixty as f64);
            let t0 = Instant::now();
            let mut contacts = 0usize;
            for f in 0..frames {
                physics.step(dt, Vec3::new(0.0, 0.0, -1.0)).map_err(|e| {
                    CliCommandError::ValidationRejected {
                        command: "benchmark".to_string(),
                        diagnostic: e.to_string(),
                    }
                })?;
                let p = physics
                    .player_capsule
                    .as_ref()
                    .map_or(Vec3::ZERO, |c| c.foot_position);
                if let Ok(m) = gjk_intersect_and_manifold(&hull_a, Vec3::ZERO, &hull_b, p) {
                    if m.intersecting {
                        contacts += 1;
                    }
                }
                if f == 0 && nav.polygons.len() >= (two as usize) {
                    let goal = nav.polygons[0].neighbors.first().copied().unwrap_or(0);
                    let _ = nav.find_path_astar(0, goal);
                }
            }
            let elapsed_us = t0.elapsed().as_micros();
            let mean_us = elapsed_us / (frames as u128);
            Ok(format!(
                "BENCHMARK OK: frames={frames} mean_frame_us={mean_us} shaders={} passes={pass_count} csm_cascades={} walkable_polys={} gjk_contacts={contacts}",
                shaders.len(),
                csm.len(),
                nav.polygons.len()
            ))
        }
        other => Err(CliCommandError::InvalidUsage {
            usage: format!("unknown subcommand `{other}`\n{CLI_USAGE}"),
        }),
    }
}
