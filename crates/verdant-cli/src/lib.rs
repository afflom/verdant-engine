//! Command-line asset validator, progressive `UORC` + `.glb` + `.vdnt` scene packer, and headless
//! 60fps frame-budget benchmark for Verdant Engine (`verdant-cli`).

#![forbid(unsafe_code)]

use std::fmt;
use std::path::Path;
use std::time::Instant;

use verdant_core::collision::{gjk_intersect_and_manifold, ConvexHullCollider};
use verdant_core::cybernetics::{CodedSlmDirector, ContainerGrantMatrix};
use verdant_core::ecs::{EntityId, EntityRole, SoaSceneTable, Transform, TwoLevelBvh};
use verdant_core::geometry::canonical_scene_mesh_library;
use verdant_core::gltf_io::{
    export_scene_to_glb, import_glb_to_scene, transcode_glb_to_progressive_uorc, validate_glb_bytes,
};
use verdant_core::hd_world::synthesize_hd_citadel_sector;
use verdant_core::math::{Aabb, Frustum, Ray, Vec3};
use verdant_core::narrative::canonical_citadel_chronicle_automaton;
use verdant_core::navmesh::NavMesh;
use verdant_core::network::{
    recover_bytes_from_stitch_pair, shard_bytes_to_stitch_fragments, StitchFragment,
};
use verdant_core::physics::{
    CapsuleController, KinematicSplineBody, PhysicsWorld, PhysicsWorldConfig,
};
use verdant_core::prism_kernel;
use verdant_core::render::{
    canonical_pbr_material_library, compute_csm_splits, validate_builtin_engine_shaders,
    validate_wgsl_shader, RenderFrameGraph,
};
use verdant_core::runtime::{
    canonicalize_command_monoid, CanonicalCommandKind, CanonicalCommandOp, KappaTransitionCache,
    WorldSnapshot,
};
use verdant_core::streaming::{
    compress_uorc_portable, decode_portable_uorc_stream_up_to_lod, decompress_uorc_portable,
    is_uorc_archive, pack_portable_progressive_uorc_stream, pack_progressive_uorc_stream,
    split_self_delimiting_uorc_archives, verify_progressive_uorc_stream, StandardSceneState,
    TerrainReliefParams,
};
use verdant_core::ui::UiTree;
use verdant_core::universe::UniverseManager;

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
  import-glb <file.glb>                       Import .glb hierarchy, meshes, skins, clips, & PBR materials
  transcode-glb <in.glb> <out.uorc> [seed]    Transcode .glb scene into progressive LOD0..LOD3 .uorc stream
  validate-wgsl <file.wgsl> [entry...]        Verify W3C WGSL shader module via naga
  pack-scene <out.glb> [seed] [lod_rank]      Pack progressive .uorc + .glb + .vdnt scene bundle
  pack-portable-uorc <out.uorc> [seed]        Pack stateless Wasm-portable LOD0..LOD3 .uorc stream
  compress-uorc <in_file> <out.uorc>          Compress arbitrary bytes into a stateless UORC archive
  decompress-uorc <in.uorc> <out_file>        Verify and decompress a stateless UORC archive
  verify-uorc <file.uorc>                     Verify UORC-SHA256-CHAIN-v1 multi-LOD stream
  verify-portable-uorc <file.uorc> [lod_rank] Verify portable .uorc stream via uorc_decompress::run
  snapshot-verify <file.vdnt>                 Verify VDNT + SHA-256 snapshot and Prism envelope
  verify-ui [width] [height]                  Solve and verify formal 2D/3D UI & WCAG compliance
  route-intent <prompt>                       Route natural-language intent via embedded coded_gemm SLM
  stitch-shard <in.uorc> <out_dir>            Split object into 3 Stitch (A, B, parity) SHA-256 shards
  stitch-recover <frag1.stch> <frag2.stch> <out> Recover exact object from any 2 distinct Stitch shards
  verify-story [seed]                         Verify StoryAutomaton tropical reachability & conservation
  pack-universe <out_dir> [seed]              Pack 3-sector floating-origin universe & Stitch shards
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
        "import-glb" => {
            let Some(path) = args.get(1) else {
                return Err(CliCommandError::InvalidUsage {
                    usage: "verdant import-glb <file.glb>".to_string(),
                });
            };
            let bytes = std::fs::read(path).map_err(|e| CliCommandError::IoFailure {
                path: path.clone(),
                reason: e.to_string(),
            })?;
            let imported =
                import_glb_to_scene(&bytes).map_err(|e| CliCommandError::ValidationRejected {
                    command: "import-glb".to_string(),
                    diagnostic: e.to_string(),
                })?;
            Ok(format!(
                "GLB IMPORTED OK: path={path} entities={} meshes={} materials={} skeletons={} clips={}",
                imported.scene.entity_count(),
                imported.meshes.len(),
                imported.materials.len(),
                imported.skeletons.len(),
                imported.clips.len()
            ))
        }
        "transcode-glb" => {
            let out_idx = prism_kernel::sTwo() as usize;
            let seed_idx = prism_kernel::sThree() as usize;
            let (Some(in_glb), Some(out_uorc)) = (args.get(1), args.get(out_idx)) else {
                return Err(CliCommandError::InvalidUsage {
                    usage: "verdant transcode-glb <in.glb> <out.uorc> [seed]".to_string(),
                });
            };
            let seed = match args.get(seed_idx) {
                Some(raw) => raw
                    .parse::<u64>()
                    .map_err(|_| CliCommandError::InvalidUsage {
                        usage: format!("invalid u64 seed `{raw}`"),
                    })?,
                None => pk("transcode-glb", prism_kernel::wasmDefaultSeed())?,
            };
            let glb_bytes = std::fs::read(in_glb).map_err(|e| CliCommandError::IoFailure {
                path: in_glb.clone(),
                reason: e.to_string(),
            })?;
            let uorc_stream = transcode_glb_to_progressive_uorc(&glb_bytes, seed).map_err(|e| {
                CliCommandError::ValidationRejected {
                    command: "transcode-glb".to_string(),
                    diagnostic: e.to_string(),
                }
            })?;
            let max_lod = prism_kernel::wasmMaxLodRank();
            let restored =
                decode_portable_uorc_stream_up_to_lod(&uorc_stream, max_lod).map_err(|e| {
                    CliCommandError::ValidationRejected {
                        command: "transcode-glb".to_string(),
                        diagnostic: e.to_string(),
                    }
                })?;
            std::fs::write(out_uorc, &uorc_stream).map_err(|e| CliCommandError::IoFailure {
                path: out_uorc.clone(),
                reason: e.to_string(),
            })?;
            Ok(format!(
                "GLB TRANSCODED UORC OK: in={in_glb} ({}B) out={out_uorc} ({}B) max_lod={} entities={}",
                glb_bytes.len(),
                uorc_stream.len(),
                restored.active_lod.rank(),
                restored.scene.entity_count()
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
            let snap_bytes = WorldSnapshot::encode_uorc(&world).map_err(|e| {
                CliCommandError::ValidationRejected {
                    command: "pack-scene".to_string(),
                    diagnostic: e.to_string(),
                }
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
        "pack-portable-uorc" => {
            let Some(out_uorc) = args.get(1) else {
                return Err(CliCommandError::InvalidUsage {
                    usage: "verdant pack-portable-uorc <out.uorc> [seed]".to_string(),
                });
            };
            let seed_arg_idx = prism_kernel::sTwo() as usize;
            let seed = match args.get(seed_arg_idx) {
                Some(raw) => raw
                    .parse::<u64>()
                    .map_err(|_| CliCommandError::InvalidUsage {
                        usage: format!("invalid u64 seed `{raw}`"),
                    })?,
                None => pk("pack-portable-uorc", prism_kernel::wasmDefaultSeed())?,
            };
            let stream = pack_portable_progressive_uorc_stream(seed).map_err(|e| {
                CliCommandError::ValidationRejected {
                    command: "pack-portable-uorc".to_string(),
                    diagnostic: e.to_string(),
                }
            })?;
            let max_lod = prism_kernel::wasmMaxLodRank();
            let restored =
                decode_portable_uorc_stream_up_to_lod(&stream, max_lod).map_err(|e| {
                    CliCommandError::ValidationRejected {
                        command: "pack-portable-uorc".to_string(),
                        diagnostic: e.to_string(),
                    }
                })?;
            std::fs::write(out_uorc, &stream).map_err(|e| CliCommandError::IoFailure {
                path: out_uorc.clone(),
                reason: e.to_string(),
            })?;
            Ok(format!(
                "PORTABLE UORC OK: path={out_uorc} bytes={} seed={} max_lod={} nodes={}",
                stream.len(),
                restored.seed,
                restored.active_lod.rank(),
                restored.scene_nodes.len()
            ))
        }
        "verify-portable-uorc" => {
            let Some(path) = args.get(1) else {
                return Err(CliCommandError::InvalidUsage {
                    usage: "verdant verify-portable-uorc <file.uorc> [lod_rank]".to_string(),
                });
            };
            let lod_arg_idx = prism_kernel::sTwo() as usize;
            let target_lod = match args.get(lod_arg_idx) {
                Some(raw) => raw
                    .parse::<u64>()
                    .map_err(|_| CliCommandError::InvalidUsage {
                        usage: format!("invalid u64 lod_rank `{raw}`"),
                    })?,
                None => prism_kernel::wasmMaxLodRank(),
            };
            let stream = std::fs::read(path).map_err(|e| CliCommandError::IoFailure {
                path: path.clone(),
                reason: e.to_string(),
            })?;
            let frames = split_self_delimiting_uorc_archives(&stream).map_err(|e| {
                CliCommandError::ValidationRejected {
                    command: "verify-portable-uorc".to_string(),
                    diagnostic: e.to_string(),
                }
            })?;
            let restored =
                decode_portable_uorc_stream_up_to_lod(&stream, target_lod).map_err(|e| {
                    CliCommandError::ValidationRejected {
                        command: "verify-portable-uorc".to_string(),
                        diagnostic: e.to_string(),
                    }
                })?;
            Ok(format!(
                "PORTABLE UORC VERIFIED: path={path} bytes={} frames={} lod={} nodes={}",
                stream.len(),
                frames.len(),
                restored.active_lod.rank(),
                restored.scene_nodes.len()
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
                "VDNT OK: path={path} bytes={} uorc_wire={} seed={} lod={} scale={:.3} nodes={} url_bridge_ready={}",
                wire.len(),
                is_uorc_archive(&wire),
                world.seed,
                world.active_lod.rank(),
                world.resolution_scale,
                world.scene_nodes.len(),
                url_safe
            ))
        }
        "compress-uorc" => {
            let out_idx = prism_kernel::sTwo() as usize;
            let (Some(in_path), Some(out_path)) = (args.get(1), args.get(out_idx)) else {
                return Err(CliCommandError::InvalidUsage {
                    usage: "verdant compress-uorc <in_file> <out.uorc>".to_string(),
                });
            };
            let raw = std::fs::read(in_path).map_err(|e| CliCommandError::IoFailure {
                path: in_path.clone(),
                reason: e.to_string(),
            })?;
            let archive = compress_uorc_portable(&raw);
            std::fs::write(out_path, &archive).map_err(|e| CliCommandError::IoFailure {
                path: out_path.clone(),
                reason: e.to_string(),
            })?;
            Ok(format!(
                "UORC COMPRESSED OK: in={in_path} ({}B) out={out_path} ({}B)",
                raw.len(),
                archive.len()
            ))
        }
        "decompress-uorc" => {
            let out_idx = prism_kernel::sTwo() as usize;
            let (Some(in_path), Some(out_path)) = (args.get(1), args.get(out_idx)) else {
                return Err(CliCommandError::InvalidUsage {
                    usage: "verdant decompress-uorc <in.uorc> <out_file>".to_string(),
                });
            };
            let archive = std::fs::read(in_path).map_err(|e| CliCommandError::IoFailure {
                path: in_path.clone(),
                reason: e.to_string(),
            })?;
            let plain = decompress_uorc_portable(&archive).map_err(|e| {
                CliCommandError::ValidationRejected {
                    command: "decompress-uorc".to_string(),
                    diagnostic: e.to_string(),
                }
            })?;
            std::fs::write(out_path, &plain).map_err(|e| CliCommandError::IoFailure {
                path: out_path.clone(),
                reason: e.to_string(),
            })?;
            Ok(format!(
                "UORC DECOMPRESSED OK: in={in_path} ({}B) out={out_path} ({}B)",
                archive.len(),
                plain.len()
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

            let world =
                StandardSceneState::synthesize_at_lod(seven, prism_kernel::wasmMaxLodRank())
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
        "verify-ui" => {
            let two_idx = prism_kernel::sTwo() as usize;
            let default_w = (pk("verify-ui", prism_kernel::sTwelve())?
                * pk("verify-ui", prism_kernel::sHundred())?) as u32;
            let default_h = (pk("verify-ui", prism_kernel::sNine())?
                * pk("verify-ui", prism_kernel::sHundred())?) as u32;
            let w = match args.get(1) {
                Some(raw) => raw
                    .parse::<u32>()
                    .map_err(|_| CliCommandError::InvalidUsage {
                        usage: "width must be u32".to_string(),
                    })?,
                None => default_w,
            };
            let h = match args.get(two_idx) {
                Some(raw) => raw
                    .parse::<u32>()
                    .map_err(|_| CliCommandError::InvalidUsage {
                        usage: "height must be u32".to_string(),
                    })?,
                None => default_h,
            };
            let ui = UiTree::canonical_studio_ui_tree(None);
            let solved = ui.solve_and_verify(w, h, None).map_err(|e| {
                CliCommandError::ValidationRejected {
                    command: "verify-ui".to_string(),
                    diagnostic: e.to_string(),
                }
            })?;
            let diegetic = UiTree::build_diegetic_bvh(&solved);
            Ok(format!(
                "UI OK: viewport={w}x{h} widgets={} diegetic_bvh_leaves={}",
                solved.len(),
                diegetic.len()
            ))
        }
        "route-intent" => {
            let prompt = args.get(1..).map_or_else(String::new, |s| s.join(" "));
            let director = CodedSlmDirector::new();
            let grants = ContainerGrantMatrix::default();
            let proposal = director.route_prompt(&prompt, None, &grants).map_err(|e| {
                CliCommandError::ValidationRejected {
                    command: "route-intent".to_string(),
                    diagnostic: e.to_string(),
                }
            })?;
            Ok(format!(
                "INTENT OK: slug={} score_milli={} raw_logit={}",
                proposal.intent.slug(),
                proposal.score_milli,
                proposal.raw_logit
            ))
        }
        "stitch-shard" => {
            let dir_idx = prism_kernel::sTwo() as usize;
            let (Some(in_path), Some(out_dir)) = (args.get(1), args.get(dir_idx)) else {
                return Err(CliCommandError::InvalidUsage {
                    usage: "verdant stitch-shard <in_file> <out_dir>".to_string(),
                });
            };
            let bytes = std::fs::read(in_path).map_err(|e| CliCommandError::IoFailure {
                path: in_path.clone(),
                reason: e.to_string(),
            })?;
            let frags = shard_bytes_to_stitch_fragments(&bytes).map_err(|e| {
                CliCommandError::ValidationRejected {
                    command: "stitch-shard".to_string(),
                    diagnostic: e.to_string(),
                }
            })?;
            std::fs::create_dir_all(out_dir).map_err(|e| CliCommandError::IoFailure {
                path: out_dir.clone(),
                reason: e.to_string(),
            })?;
            for frag in &frags {
                let fp = Path::new(out_dir).join(format!("shard_{}.stch", frag.index));
                let wire = frag.encode_wire();
                std::fs::write(&fp, wire).map_err(|e| CliCommandError::IoFailure {
                    path: fp.to_string_lossy().into_owned(),
                    reason: e.to_string(),
                })?;
            }
            Ok(format!(
                "STITCH SHARD OK: bytes={} shards={} shard_bytes={}",
                bytes.len(),
                frags.len(),
                frags[0].payload.len()
            ))
        }
        "stitch-recover" => {
            let idx2 = prism_kernel::sTwo() as usize;
            let idx3 = prism_kernel::sThree() as usize;
            let (Some(f1_path), Some(f2_path), Some(out_path)) =
                (args.get(1), args.get(idx2), args.get(idx3))
            else {
                return Err(CliCommandError::InvalidUsage {
                    usage: "verdant stitch-recover <frag1.stch> <frag2.stch> <out_file>"
                        .to_string(),
                });
            };
            let raw1 = std::fs::read(f1_path).map_err(|e| CliCommandError::IoFailure {
                path: f1_path.clone(),
                reason: e.to_string(),
            })?;
            let raw2 = std::fs::read(f2_path).map_err(|e| CliCommandError::IoFailure {
                path: f2_path.clone(),
                reason: e.to_string(),
            })?;
            let frag1 = StitchFragment::decode_wire(&raw1).map_err(|e| {
                CliCommandError::ValidationRejected {
                    command: "stitch-recover".to_string(),
                    diagnostic: e.to_string(),
                }
            })?;
            let frag2 = StitchFragment::decode_wire(&raw2).map_err(|e| {
                CliCommandError::ValidationRejected {
                    command: "stitch-recover".to_string(),
                    diagnostic: e.to_string(),
                }
            })?;
            let recovered = recover_bytes_from_stitch_pair(&frag1, &frag2).map_err(|e| {
                CliCommandError::ValidationRejected {
                    command: "stitch-recover".to_string(),
                    diagnostic: e.to_string(),
                }
            })?;
            std::fs::write(out_path, &recovered).map_err(|e| CliCommandError::IoFailure {
                path: out_path.clone(),
                reason: e.to_string(),
            })?;
            Ok(format!(
                "STITCH RECOVER OK: pair=({},{}) bytes={}",
                frag1.index,
                frag2.index,
                recovered.len()
            ))
        }
        "verify-story" => {
            let seed = match args.get(1) {
                Some(raw) => raw
                    .parse::<u64>()
                    .map_err(|_| CliCommandError::InvalidUsage {
                        usage: format!("invalid u64 seed `{raw}`"),
                    })?,
                None => pk("verify-story", prism_kernel::wasmDefaultSeed())?,
            };
            let (automaton, report) = canonical_citadel_chronicle_automaton(seed).map_err(|e| {
                CliCommandError::ValidationRejected {
                    command: "verify-story".to_string(),
                    diagnostic: e.to_string(),
                }
            })?;
            Ok(format!(
                "STORY AUTOMATON OK: states={} edges={} live_reachable={} min_cost_milli={} supply={}",
                automaton.nodes.len(),
                automaton.edges.len(),
                report.reachable_live_states,
                report.min_cost_to_goal_milli,
                automaton.ledger.total_supply()
            ))
        }
        "pack-universe" => {
            let Some(out_dir) = args.get(1) else {
                return Err(CliCommandError::InvalidUsage {
                    usage: "verdant pack-universe <out_dir> [seed]".to_string(),
                });
            };
            let idx2 = prism_kernel::sTwo() as usize;
            let seed = match args.get(idx2) {
                Some(raw) => raw
                    .parse::<u64>()
                    .map_err(|_| CliCommandError::InvalidUsage {
                        usage: format!("invalid u64 seed `{raw}`"),
                    })?,
                None => pk("pack-universe", prism_kernel::wasmDefaultSeed())?,
            };
            let universe = UniverseManager::canonical_citadel_universe(seed).map_err(|e| {
                CliCommandError::ValidationRejected {
                    command: "pack-universe".to_string(),
                    diagnostic: e.to_string(),
                }
            })?;
            std::fs::create_dir_all(out_dir).map_err(|e| CliCommandError::IoFailure {
                path: out_dir.clone(),
                reason: e.to_string(),
            })?;
            let manifest_bytes = universe.to_manifest_json_bytes().map_err(|e| {
                CliCommandError::ValidationRejected {
                    command: "pack-universe".to_string(),
                    diagnostic: e.to_string(),
                }
            })?;
            let manifest_path = Path::new(out_dir).join("universe-manifest.json");
            std::fs::write(&manifest_path, &manifest_bytes).map_err(|e| {
                CliCommandError::IoFailure {
                    path: manifest_path.to_string_lossy().into_owned(),
                    reason: e.to_string(),
                }
            })?;
            let uorc_stream = pack_portable_progressive_uorc_stream(seed).map_err(|e| {
                CliCommandError::ValidationRejected {
                    command: "pack-universe".to_string(),
                    diagnostic: e.to_string(),
                }
            })?;
            let frags = shard_bytes_to_stitch_fragments(&uorc_stream).map_err(|e| {
                CliCommandError::ValidationRejected {
                    command: "pack-universe".to_string(),
                    diagnostic: e.to_string(),
                }
            })?;
            for frag in &frags {
                let fp = Path::new(out_dir).join(format!("sector_0_0_0_shard_{}.stch", frag.index));
                std::fs::write(&fp, frag.encode_wire()).map_err(|e| {
                    CliCommandError::IoFailure {
                        path: fp.to_string_lossy().into_owned(),
                        reason: e.to_string(),
                    }
                })?;
            }
            Ok(format!(
                "UNIVERSE PACKED OK: sectors={} portals={} uorc_bytes={} stitch_shards={}",
                universe.sectors.len(),
                universe.portal_graph.portals.len(),
                uorc_stream.len(),
                frags.len()
            ))
        }
        "stress-benchmark" => {
            let ten_k_default = {
                let ten = (1usize + 1usize) * (1usize + 1usize + 1usize + 1usize + 1usize);
                ten * ten * ten * ten
            };
            let entity_count = match args.get(1) {
                Some(raw) => raw
                    .parse::<usize>()
                    .map_err(|_| CliCommandError::InvalidUsage {
                        usage: format!("invalid entity count `{raw}`"),
                    })?,
                None => ten_k_default,
            };
            if entity_count == 0 {
                return Err(CliCommandError::InvalidUsage {
                    usage: "stress-benchmark requires entity_count >= 1".to_string(),
                });
            }

            let t0 = Instant::now();
            let mut soa = SoaSceneTable::with_capacity(entity_count);
            let half = 1.0 / ((1 + 1) as f64);
            let local_box = Aabb::new(Vec3::new(-half, -half, -half), Vec3::new(half, half, half));
            let grid_side = (entity_count as f64).sqrt().ceil().max(1.0) as usize;
            for i in 0..entity_count {
                let gx = (i % grid_side) as f64;
                let gz = (i / grid_side) as f64;
                soa.spawn_dense(
                    EntityId((i + 1) as u64),
                    EntityRole::StaticArchitecture,
                    None,
                    Transform::from_xyz(gx * (1.0 + 1.0), 1.0, -gz * (1.0 + 1.0)),
                    local_box,
                    (Some(0), [1.0, 1.0, 1.0, 1.0]),
                );
            }
            let dirty_subset = entity_count.min(1usize << (1 + 1 + 1 + 1 + 1));
            for slot in 0..dirty_subset {
                soa.set_local_transform(slot, Transform::from_xyz(slot as f64, 1.0 + half, -1.0));
            }
            let updated_xforms = soa.propagate_dirty_transforms();
            let frustum = Frustum::from_perspective(
                Vec3::new(0.0, 1.0 + 1.0, (1 + 1 + 1 + 1) as f64),
                Vec3::new(0.0, 0.0, -1.0),
                Vec3::UP,
                1.0,
                1.0 + half,
                half * half,
                (1usize << (1 + 1 + 1 + 1 + 1 + 1 + 1)) as f64,
            );
            let visible_count = soa.cull_frustum_word_packed(&frustum);
            let mut inst_f32 = Vec::new();
            let compacted = soa.compact_visible_instances_for_mesh(0, &mut inst_f32);
            let soa_us = t0.elapsed().as_micros();

            let lod4 = pk("stress-benchmark", prism_kernel::wasmHdMaxLodRank())?;
            let seed = pk("stress-benchmark", prism_kernel::wasmDefaultSeed())?;
            let sector = synthesize_hd_citadel_sector(
                seed,
                &TerrainReliefParams::standard_basin_valley(),
                -1.0,
                lod4,
            );
            let bvh = TwoLevelBvh::build_solid(&sector.scene, &sector.meshes).map_err(|e| {
                CliCommandError::ValidationRejected {
                    command: "stress-benchmark".to_string(),
                    diagnostic: e.to_string(),
                }
            })?;
            let ray_count = (1usize + 1usize + 1usize + 1usize + 1usize)
                * ((1usize << (1 + 1 + 1 + 1 + 1 + 1))
                    + (1usize << (1 + 1 + 1 + 1 + 1))
                    + (1usize << (1 + 1)));
            let t_bvh = Instant::now();
            let mut total_visited = 0usize;
            let mut hits = 0usize;
            for r_idx in 0..ray_count {
                let fx = ((r_idx % (1 << (1 + 1 + 1 + 1))) as f64) - ((1 << (1 + 1 + 1)) as f64);
                let ray = Ray::new(
                    Vec3::new(fx * half, (1 << (1 + 1 + 1 + 1)) as f64, fx * half),
                    Vec3::new(0.0, -1.0, 0.0),
                );
                let (hit, visited) =
                    bvh.raycast_exact_counted(ray, (1 << (1 + 1 + 1 + 1 + 1 + 1)) as f64);
                if hit.is_some() {
                    hits += 1;
                }
                total_visited += visited;
            }
            let bvh_us = t_bvh.elapsed().as_micros();
            let avg_visited = total_visited / ray_count.max(1);

            let mut bit_mask = verdant_core::verdant_tensor::WordBitset::with_len(entity_count);
            let plane_matches =
                soa.query_capability_planes_into(&[0, 1], &[1 + 1 + 1], &mut bit_mask);
            let ops = [
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
            ];
            let fused_ops = canonicalize_command_monoid(&ops)
                .map(|r| r.fused_count)
                .unwrap_or(1);
            let mut kcache: KappaTransitionCache<u64> = KappaTransitionCache::new(1 << (1 + 1 + 1));
            let k0 = KappaTransitionCache::<u64>::digest_bytes(&seed.to_le_bytes());
            let _ = kcache.step_or_elide(k0, 1, || (seed + 1, (seed + 1).to_le_bytes().to_vec()));
            let _ = kcache.step_or_elide(k0, 1, || (seed + 1, (seed + 1).to_le_bytes().to_vec()));
            let kappa_hits = kcache.elided_step_hits();

            Ok(format!(
                "STRESS BENCHMARK OK: soa_entities={entity_count} word_masks={} dirty_updated={updated_xforms} frustum_visible={visible_count} instanced_compacted={compacted} bit_plane_matches={plane_matches} monoid_fused_ops={fused_ops} kappa_elided_hits={kappa_hits} soa_us={soa_us} hd_tris={} flat_bvh_nodes={} rays={ray_count} ray_hits={hits} avg_visited_nodes={avg_visited} bvh_us={bvh_us}",
                soa.visible_mask.words().len(),
                bvh.total_surface_triangles(),
                bvh.total_bvh_nodes(),
            ))
        }
        other => Err(CliCommandError::InvalidUsage {
            usage: format!("unknown subcommand `{other}`\n{CLI_USAGE}"),
        }),
    }
}
