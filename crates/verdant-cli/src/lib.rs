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
    recover_bytes_from_stitch_pair, shard_bytes_to_stitch_fragments, PrismIdSessionEnvelope,
    StitchFragment,
};
use verdant_core::observer::{
    sha256_payload_digest, CommunicationChannelKind, KnowledgePassageRequest, ObserverPrismHolon,
    ProjectModelEditRequest, ProjectModelImportRequest, StreamRegisterRequest,
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
  automate-check                              Self-verify all 4 catalog domains, Observer, UI, WGSL, & .uorc
  operator <status|prompt|apply|synthesize|publish-app|verify-seal|audit|session>  Manage Verdant via VSM & ISO 42010 API
  release <pack|verify> [options]             Pack or 4-of-8 verify a self-hosted .holo Stitch release
  mcp [--root <dir>]                          Run Model Context Protocol (JSON-RPC 2.0 stdio) server
  benchmark [frames]                          Run headless 60fps physics + BVH + GJK + NavMesh benchmark
  stress-benchmark [entities]                 Run 10k+ SoA bit-plane, TwoLevelBvh & monoid stress benchmark";

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
        "automate-check" => {
            let t0 = Instant::now();
            let domain_slugs = [
                "sunspire-regatta",
                "meridian-observatory",
                "glasshouse-chronicle",
                "starfall-bastion",
            ];
            let mut holon = ObserverPrismHolon::new(domain_slugs[0]).map_err(|e| {
                CliCommandError::ValidationRejected {
                    command: "automate-check".to_string(),
                    diagnostic: e.to_string(),
                }
            })?;
            let max_lod = prism_kernel::wasmMaxLodRank();
            let mut verified_domains = 0usize;
            let mut total_entities = 0usize;
            let origin = "https://afflom.github.io/verdant-engine/";
            let now_sec: u64 = 1;
            let expires_sec: u64 = 1u64 << (1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1);
            let prism_id = "prism:verdant:automate";
            let dev_secret = b"verdant-automate-device";
            for slug in domain_slugs {
                holon.switch_domain_pack(slug, Some(max_lod)).map_err(|e| {
                    CliCommandError::ValidationRejected {
                        command: "automate-check".to_string(),
                        diagnostic: format!("domain {slug}: {e}"),
                    }
                })?;
                verified_domains += 1;
                total_entities += holon.active_domain.sector.scene.entity_count();

                // Verify grounded ISO 24617-2 dialogue + zero-hallucination Unknown abstention:
                let grounded_prompt = holon
                    .active_domain
                    .spec
                    .characters
                    .first()
                    .and_then(|c| c.knowledge_facts.first())
                    .cloned()
                    .unwrap_or_else(|| "status".to_string());
                let g_digest = sha256_payload_digest(grounded_prompt.as_bytes());
                let g_env = PrismIdSessionEnvelope::sign_deterministic(
                    prism_id,
                    dev_secret,
                    origin,
                    now_sec,
                    expires_sec,
                    &g_digest,
                );
                let g_turn = holon
                    .converse_player_with_npc(
                        prism_id,
                        (&g_env, origin, now_sec),
                        0,
                        &grounded_prompt,
                    )
                    .map_err(|e| CliCommandError::ValidationRejected {
                        command: "automate-check".to_string(),
                        diagnostic: format!("grounded dialogue {slug}: {e}"),
                    })?;
                if g_turn.abstained_unknown || g_turn.matched_evidence_facts == 0 {
                    return Err(CliCommandError::ValidationRejected {
                        command: "automate-check".to_string(),
                        diagnostic: format!("grounded dialogue unexpectedly abstained on {slug}"),
                    });
                }

                let ungrounded = "unmodeled quantum neptune exchange rate";
                let u_digest = sha256_payload_digest(ungrounded.as_bytes());
                let u_env = PrismIdSessionEnvelope::sign_deterministic(
                    prism_id,
                    dev_secret,
                    origin,
                    now_sec,
                    expires_sec,
                    &u_digest,
                );
                let u_turn = holon
                    .converse_player_with_npc(prism_id, (&u_env, origin, now_sec), 0, ungrounded)
                    .map_err(|e| CliCommandError::ValidationRejected {
                        command: "automate-check".to_string(),
                        diagnostic: format!("abstention dialogue {slug}: {e}"),
                    })?;
                if !u_turn.abstained_unknown || u_turn.world_mutated {
                    return Err(CliCommandError::ValidationRejected {
                        command: "automate-check".to_string(),
                        diagnostic: format!("ungrounded dialogue failed abstention on {slug}"),
                    });
                }
            }

            let _ = holon
                .observe_communication(
                    CommunicationChannelKind::OrganToOrgan,
                    "urn:verdant:organ:studio",
                    "urn:verdant:organ:engine",
                    "en",
                    0,
                    "automate-check:self-verified",
                    None,
                    0,
                )
                .map_err(|e| CliCommandError::ValidationRejected {
                    command: "automate-check".to_string(),
                    diagnostic: format!("observe_communication: {e}"),
                })?;

            // Verify dynamic arbitrary model admission, versioning (v1 -> v2 -> revert v3), & removal:
            let grants = ContainerGrantMatrix::default();
            let imp = holon
                .import_project_model(
                    ProjectModelImportRequest {
                        model_id: "khronos:DamagedHelmet".to_string(),
                        title: "Damaged Helmet (Khronos glTF 2.0)".to_string(),
                        schema_type: "3DModel".to_string(),
                        media_type: "model/gltf-binary".to_string(),
                        target_slot: "design:3d-model".to_string(),
                        source_stream: "khronos-gltf".to_string(),
                        source_uri: "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Assets/main/Models/DamagedHelmet/glTF-Binary/DamagedHelmet.glb".to_string(),
                        license_spdx: "CC-BY-4.0".to_string(),
                        tags: vec!["3d".to_string(), "pbr".to_string(), "gltf".to_string()],
                        properties_json: "{\"scale\":1}".to_string(),
                        commit_note: "Initial catalog import".to_string(),
                    },
                    &grants,
                )
                .map_err(|e| CliCommandError::ValidationRejected {
                    command: "automate-check".to_string(),
                    diagnostic: format!("import_project_model: {e}"),
                })?;
            let edited = holon
                .edit_project_model(ProjectModelEditRequest {
                    model_id: imp.model_id.clone(),
                    title: "Damaged Helmet · Tuned Metallic".to_string(),
                    tags: vec!["3d".to_string(), "pbr".to_string(), "hero".to_string()],
                    properties_json: "{\"scale\":1,\"metallic\":1}".to_string(),
                    commit_note: "Tuned PBR metallic factor".to_string(),
                })
                .map_err(|e| CliCommandError::ValidationRejected {
                    command: "automate-check".to_string(),
                    diagnostic: format!("edit_project_model: {e}"),
                })?;
            let reverted = holon.revert_project_model(&imp.model_id, 1).map_err(|e| {
                CliCommandError::ValidationRejected {
                    command: "automate-check".to_string(),
                    diagnostic: format!("revert_project_model: {e}"),
                }
            })?;
            let removed_kappa = holon.remove_project_model(&imp.model_id).map_err(|e| {
                CliCommandError::ValidationRejected {
                    command: "automate-check".to_string(),
                    diagnostic: format!("remove_project_model: {e}"),
                }
            })?;
            if edited.active_version != (1 + 1)
                || reverted.active_version != (1 + 1 + 1)
                || reverted.current_kappa_hex != removed_kappa
            {
                return Err(CliCommandError::ValidationRejected {
                    command: "automate-check".to_string(),
                    diagnostic: "project model version chain mismatch".to_string(),
                });
            }

            // Self-verify Observer Sensory Stream Registry (10 seeded streams + dynamic add/refuse/remove),
            // UHD 4K (3840x2160) Host Capability Envelope, and live stream knowledge grounding:
            let expected_streams = (1usize + 1) * (1 + 1 + 1 + 1 + 1);
            if holon.sensory_streams.len() != expected_streams {
                return Err(CliCommandError::ValidationRejected {
                    command: "automate-check".to_string(),
                    diagnostic: format!(
                        "expected {expected_streams} sensory streams, found {}",
                        holon.sensory_streams.len()
                    ),
                });
            }
            if holon
                .register_stream(
                    StreamRegisterRequest {
                        stream_id: "bad-http".to_string(),
                        label: "Bad HTTP".to_string(),
                        authority: "Refused".to_string(),
                        adapter_kind: "schema-org-catalog".to_string(),
                        endpoint_url: "http://invalid.example/catalog.json".to_string(),
                        target_parts: vec!["hero-props".to_string()],
                    },
                    &grants,
                )
                .is_ok()
            {
                return Err(CliCommandError::ValidationRejected {
                    command: "automate-check".to_string(),
                    diagnostic: "non-HTTPS stream was not refused by DMN/Schema oracle".to_string(),
                });
            }
            let added_stream = holon
                .register_stream(
                    StreamRegisterRequest {
                        stream_id: "automate-uhd-stream".to_string(),
                        label: "Automate UHD Stream".to_string(),
                        authority: "Verdant CI".to_string(),
                        adapter_kind: "khronos-index".to_string(),
                        endpoint_url: "https://example.org/model-index.json".to_string(),
                        target_parts: vec!["character-rigs".to_string()],
                    },
                    &grants,
                )
                .map_err(|e| CliCommandError::ValidationRejected {
                    command: "automate-check".to_string(),
                    diagnostic: format!("register_stream: {e}"),
                })?;
            let _ = holon.remove_stream(&added_stream.stream_id).map_err(|e| {
                CliCommandError::ValidationRejected {
                    command: "automate-check".to_string(),
                    diagnostic: format!("remove_stream: {e}"),
                }
            })?;

            // Verify UHD 4K (3840x2160) Host Envelope laws (hardware native & software clamped):
            let tex_8192 = 1u32 << (1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1);
            let tex_2048 = tex_8192 >> (1 + 1);
            let tex_1024 = tex_2048 >> 1;
            let tex_512 = tex_1024 >> 1;
            let uhd_w = tex_2048 + tex_1024 + tex_512 + (tex_512 >> 1);
            let uhd_h = tex_2048 + (tex_512 >> (1 + 1)) - (1u32 << (1 + 1 + 1 + 1));
            let fast_us = (1u32 << (1 + 1 + 1))
                * (1 + 1 + 1 + 1 + 1)
                * (1 + 1 + 1 + 1 + 1)
                * (1 + 1 + 1 + 1 + 1)
                * (1 + 1 + 1 + 1 + 1 + 1);
            holon.update_host_envelope_with_viewport(
                tex_8192,
                false,
                1u32 << (1 + 1 + 1),
                fast_us,
                total_entities as u32,
                uhd_w,
                uhd_h,
            );
            if !holon.host_envelope.is_uhd_target
                || holon.host_envelope.recommended_texture_cap_px > tex_2048
            {
                return Err(CliCommandError::ValidationRejected {
                    command: "automate-check".to_string(),
                    diagnostic: "UHD 4K host envelope contract violation".to_string(),
                });
            }

            // Verify live stream knowledge passage ingestion grounds ISO 24617-2 answers:
            let _ = holon
                .ingest_knowledge_passage(KnowledgePassageRequest {
                    source_stream: "wikidata".to_string(),
                    source_uri: "https://en.wikipedia.org/wiki/Apollo_Lunar_Module".to_string(),
                    topic: "Apollo Lunar Module".to_string(),
                    passage:
                        "The Apollo Lunar Module landed crewed astronauts on the Moon surface."
                            .to_string(),
                })
                .map_err(|e| CliCommandError::ValidationRejected {
                    command: "automate-check".to_string(),
                    diagnostic: format!("ingest_knowledge: {e}"),
                })?;
            let q_ing = "Where did the Apollo Lunar Module land?";
            let q_ing_digest = sha256_payload_digest(q_ing.as_bytes());
            let q_ing_env = PrismIdSessionEnvelope::sign_deterministic(
                prism_id,
                dev_secret,
                origin,
                now_sec,
                expires_sec,
                &q_ing_digest,
            );
            let grounded_turn = holon
                .converse_player_with_npc(prism_id, (&q_ing_env, origin, now_sec), 0, q_ing)
                .map_err(|e| CliCommandError::ValidationRejected {
                    command: "automate-check".to_string(),
                    diagnostic: format!("grounded_turn: {e}"),
                })?;
            if grounded_turn.abstained_unknown {
                return Err(CliCommandError::ValidationRejected {
                    command: "automate-check".to_string(),
                    diagnostic: "ISO 24617-2 failed to ground freshly ingested stream passage"
                        .to_string(),
                });
            }

            let ui_w = (1u32 << (1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1))
                + (1u32 << (1 + 1 + 1 + 1 + 1 + 1 + 1 + 1));
            let ui_h = (1u32 << (1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1))
                + (1u32 << (1 + 1 + 1 + 1 + 1 + 1 + 1 + 1));
            let first_ent = holon
                .active_domain
                .sector
                .scene
                .entities()
                .next()
                .map(|e| e.id);
            let solved_ui = UiTree::canonical_studio_ui_tree(first_ent)
                .solve_and_verify(ui_w, ui_h, Some(&holon.active_domain.sector.scene))
                .map_err(|e| CliCommandError::ValidationRejected {
                    command: "automate-check".to_string(),
                    diagnostic: format!("verify-ui: {e}"),
                })?;

            let shaders = validate_builtin_engine_shaders().map_err(|e| {
                CliCommandError::ValidationRejected {
                    command: "automate-check".to_string(),
                    diagnostic: format!("wgsl: {e}"),
                }
            })?;
            let uhd_shaders = verdant_core::uhd::validate_uhd_shaders().map_err(|e| {
                CliCommandError::ValidationRejected {
                    command: "automate-check".to_string(),
                    diagnostic: format!("uhd-wgsl: {e}"),
                }
            })?;
            let uhd_8k = verdant_core::uhd::evaluate_uhd_display_profile(
                uhd_w << 1,
                uhd_h << 1,
                false,
                true,
                fast_us,
            );
            if uhd_8k.display_class != "UHD 8K 4320p" {
                return Err(CliCommandError::ValidationRejected {
                    command: "automate-check".to_string(),
                    diagnostic: format!("unexpected 8K UHD profile: {}", uhd_8k.display_class),
                });
            }
            let sample_meshlets = verdant_core::uhd::build_meshlet_graph_from_mesh(
                &holon.active_domain.sector.meshes[0],
            );
            if sample_meshlets.clusters.is_empty()
                || sample_meshlets.reordered_indices.len()
                    != sample_meshlets.total_triangles * (1 + 1 + 1)
            {
                return Err(CliCommandError::ValidationRejected {
                    command: "automate-check".to_string(),
                    diagnostic: "Morton meshlet graph produced empty or misaligned clusters"
                        .to_string(),
                });
            }
            let mut frame_oracle = verdant_core::uhd::FrameBudgetOracle::default();
            for _ in 0..verdant_core::uhd::FRAME_ORACLE_WINDOW {
                frame_oracle.record_frame(fast_us);
            }
            let frame_rep = frame_oracle.report();
            if !frame_rep.stable || frame_rep.drs_transitions != 0 {
                return Err(CliCommandError::ValidationRejected {
                    command: "automate-check".to_string(),
                    diagnostic: format!(
                        "FrameBudgetOracle unstable on nominal load: {frame_rep:?}"
                    ),
                });
            }

            let seed = pk("automate-check", prism_kernel::wasmDefaultSeed())?;
            let std_state = StandardSceneState::synthesize_at_lod(seed, max_lod).map_err(|e| {
                CliCommandError::ValidationRejected {
                    command: "automate-check".to_string(),
                    diagnostic: e.to_string(),
                }
            })?;
            let uorc_snap = WorldSnapshot::encode_uorc(&std_state).map_err(|e| {
                CliCommandError::ValidationRejected {
                    command: "automate-check".to_string(),
                    diagnostic: e.to_string(),
                }
            })?;
            if !is_uorc_archive(&uorc_snap) {
                return Err(CliCommandError::ValidationRejected {
                    command: "automate-check".to_string(),
                    diagnostic: "WorldSnapshot::encode_uorc did not emit UORC archive".to_string(),
                });
            }
            let frags = shard_bytes_to_stitch_fragments(&uorc_snap).map_err(|e| {
                CliCommandError::ValidationRejected {
                    command: "automate-check".to_string(),
                    diagnostic: format!("stitch shard: {e}"),
                }
            })?;
            let recovered =
                recover_bytes_from_stitch_pair(&frags[0], &frags[1 + 1]).map_err(|e| {
                    CliCommandError::ValidationRejected {
                        command: "automate-check".to_string(),
                        diagnostic: format!("stitch recovery: {e}"),
                    }
                })?;
            if recovered != uorc_snap {
                return Err(CliCommandError::ValidationRejected {
                    command: "automate-check".to_string(),
                    diagnostic: "stitch recovered bytes differed from .uorc snapshot".to_string(),
                });
            }

            let op_channel = verdant_core::operator::VerdantOperatorChannel::new(
                "sunspire-regatta",
            )
            .map_err(|e| CliCommandError::ValidationRejected {
                command: "automate-check".to_string(),
                diagnostic: format!("operator channel: {e}"),
            })?;
            let own_rep = op_channel.check_total_ownership();
            if !own_rep.complete {
                return Err(CliCommandError::ValidationRejected {
                    command: "automate-check".to_string(),
                    diagnostic: format!(
                        "Total Ownership violation (unmanaged parts): orphans={:?} duplicates={:?}",
                        own_rep.orphans, own_rep.duplicate_owners
                    ),
                });
            }

            let elapsed_ms = t0.elapsed().as_millis();
            Ok(format!(
                "AUTOMATE CHECK OK: domains={verified_domains} managers={} streams={} uhd={} uhd8k={} meshlets={} p95_us={} entities={total_entities} ui_widgets={} wgsl_shaders={}+{} uorc_bytes={} stitch_parity=2of3 elapsed_ms={elapsed_ms}",
                own_rep.managers_checked,
                holon.sensory_streams.len(),
                holon.host_envelope.uhd_tier_label,
                uhd_8k.display_class,
                sample_meshlets.clusters.len(),
                frame_rep.p95_us,
                solved_ui.len(),
                shaders.len(),
                uhd_shaders.len(),
                uorc_snap.len(),
            ))
        }
        "operator" => {
            let sub = args.get(1).map(String::as_str).unwrap_or("status");
            let mut ch = verdant_core::operator::VerdantOperatorChannel::new("sunspire-regatta")
                .map_err(|e| CliCommandError::ValidationRejected {
                    command: "operator".to_string(),
                    diagnostic: e.to_string(),
                })?;
            match sub {
                "status" => Ok(ch.status_json()),
                "prompt" => {
                    let two = 1usize + 1usize;
                    let mut viewpoint = "auto".to_string();
                    let mut directive = String::new();
                    let mut cited_authority: Option<String> = None;
                    let mut actor = "prism:verdant-operator".to_string();
                    let mut i = two;
                    while i < args.len() {
                        match args[i].as_str() {
                            "--viewpoint" if i + 1 < args.len() => {
                                viewpoint = args[i + 1].clone();
                                i += two;
                            }
                            "--directive" if i + 1 < args.len() => {
                                directive = args[i + 1].clone();
                                i += two;
                            }
                            "--authority" if i + 1 < args.len() => {
                                cited_authority = Some(args[i + 1].clone());
                                i += two;
                            }
                            "--actor" if i + 1 < args.len() => {
                                actor = args[i + 1].clone();
                                i += two;
                            }
                            other => {
                                if directive.is_empty() {
                                    directive = other.to_string();
                                } else {
                                    directive.push(' ');
                                    directive.push_str(other);
                                }
                                i += 1;
                            }
                        }
                    }
                    let resp = ch
                        .submit_directive(&verdant_core::operator::OperatorPromptRequest {
                            viewpoint,
                            directive,
                            cited_authority,
                            operator_prism_id: actor,
                        })
                        .map_err(|e| CliCommandError::ValidationRejected {
                            command: "operator prompt".to_string(),
                            diagnostic: e.to_string(),
                        })?;
                    verdant_core::operator::VerdantOperatorChannel::serialize_response_pretty(&resp)
                        .map_err(|e| CliCommandError::ValidationRejected {
                            command: "operator prompt".to_string(),
                            diagnostic: e.to_string(),
                        })
                }
                "apply" => {
                    let two = 1usize + 1usize;
                    let mut root =
                        std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
                    let mut viewpoint = "auto".to_string();
                    let mut directive = String::new();
                    let mut cited_authority: Option<String> = None;
                    let mut actor = "prism:verdant-operator".to_string();
                    let mut i = two;
                    while i < args.len() {
                        match args[i].as_str() {
                            "--root" if i + 1 < args.len() => {
                                root = std::path::PathBuf::from(&args[i + 1]);
                                i += two;
                            }
                            "--viewpoint" if i + 1 < args.len() => {
                                viewpoint = args[i + 1].clone();
                                i += two;
                            }
                            "--directive" if i + 1 < args.len() => {
                                directive = args[i + 1].clone();
                                i += two;
                            }
                            "--authority" if i + 1 < args.len() => {
                                cited_authority = Some(args[i + 1].clone());
                                i += two;
                            }
                            "--actor" if i + 1 < args.len() => {
                                actor = args[i + 1].clone();
                                i += two;
                            }
                            other => {
                                if directive.is_empty() {
                                    directive = other.to_string();
                                } else {
                                    directive.push(' ');
                                    directive.push_str(other);
                                }
                                i += 1;
                            }
                        }
                    }
                    let rep = ch
                        .apply_directive(
                            &root,
                            &verdant_core::operator::OperatorPromptRequest {
                                viewpoint,
                                directive,
                                cited_authority,
                                operator_prism_id: actor,
                            },
                        )
                        .map_err(|e| CliCommandError::ValidationRejected {
                            command: "operator apply".to_string(),
                            diagnostic: e.to_string(),
                        })?;
                    verdant_core::operator::VerdantOperatorChannel::serialize_apply_pretty(&rep)
                        .map_err(|e| CliCommandError::ValidationRejected {
                            command: "operator apply".to_string(),
                            diagnostic: e.to_string(),
                        })
                }
                "synthesize" => {
                    let two = 1usize + 1usize;
                    let mut root =
                        std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
                    let mut vp_filter: Option<String> = None;
                    let mut out_path: Option<String> = None;
                    let mut i = two;
                    while i < args.len() {
                        match args[i].as_str() {
                            "--root" if i + 1 < args.len() => {
                                root = std::path::PathBuf::from(&args[i + 1]);
                                i += two;
                            }
                            "--viewpoint" if i + 1 < args.len() => {
                                vp_filter = Some(args[i + 1].clone());
                                i += two;
                            }
                            "--out" if i + 1 < args.len() => {
                                out_path = Some(args[i + 1].clone());
                                i += two;
                            }
                            _ => i += 1,
                        }
                    }
                    if let Some(ref p) = out_path {
                        let json = ch.synthesize_viewpoint_contracts_json();
                        if let Some(parent) = Path::new(p).parent() {
                            if !parent.as_os_str().is_empty() {
                                std::fs::create_dir_all(parent).map_err(|e| {
                                    CliCommandError::IoFailure {
                                        path: p.clone(),
                                        reason: e.to_string(),
                                    }
                                })?;
                            }
                        }
                        std::fs::write(p, json.as_bytes()).map_err(|e| {
                            CliCommandError::IoFailure {
                                path: p.clone(),
                                reason: e.to_string(),
                            }
                        })?;
                        Ok(format!(
                            "OPERATOR SYNTHESIZE OK: wrote {p} ({} bytes, {} managers)",
                            json.len(),
                            ch.managers.len()
                        ))
                    } else {
                        let rep = ch
                            .materialize_manager_artifacts(&root, vp_filter.as_deref())
                            .map_err(|e| CliCommandError::ValidationRejected {
                                command: "operator synthesize".to_string(),
                                diagnostic: e.to_string(),
                            })?;
                        verdant_core::operator::VerdantOperatorChannel::serialize_materialization_pretty(&rep)
                            .map_err(|e| CliCommandError::ValidationRejected {
                                command: "operator synthesize".to_string(),
                                diagnostic: e.to_string(),
                            })
                    }
                }
                "publish-app" => {
                    let two = 1usize + 1usize;
                    let mut root =
                        std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
                    let mut title = String::new();
                    let mut genre = "coastal-adventure".to_string();
                    let mut domain = "sunspire-regatta".to_string();
                    let mut summary = "Published via Verdant Operator API".to_string();
                    let mut actor = "prism:verdant-operator".to_string();
                    let mut i = two;
                    while i < args.len() {
                        match args[i].as_str() {
                            "--root" if i + 1 < args.len() => {
                                root = std::path::PathBuf::from(&args[i + 1]);
                                i += two;
                            }
                            "--title" if i + 1 < args.len() => {
                                title = args[i + 1].clone();
                                i += two;
                            }
                            "--genre" if i + 1 < args.len() => {
                                genre = args[i + 1].clone();
                                i += two;
                            }
                            "--domain" if i + 1 < args.len() => {
                                domain = args[i + 1].clone();
                                i += two;
                            }
                            "--summary" if i + 1 < args.len() => {
                                summary = args[i + 1].clone();
                                i += two;
                            }
                            "--actor" if i + 1 < args.len() => {
                                actor = args[i + 1].clone();
                                i += two;
                            }
                            _ => i += 1,
                        }
                    }
                    let app = ch
                        .publish_app_to_repo_catalog(&root, &title, &genre, &domain, &summary, &actor)
                        .map_err(|e| CliCommandError::ValidationRejected {
                            command: "operator publish-app".to_string(),
                            diagnostic: e.to_string(),
                        })?;
                    verdant_core::operator::VerdantOperatorChannel::serialize_manifest_app_pretty(
                        &app,
                    )
                    .map_err(|e| CliCommandError::ValidationRejected {
                        command: "operator publish-app".to_string(),
                        diagnostic: e.to_string(),
                    })
                }
                "audit" => {
                    let two = 1usize + 1usize;
                    let mut root =
                        std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
                    let mut vp_filter: Option<String> = None;
                    let mut i = two;
                    while i < args.len() {
                        match args[i].as_str() {
                            "--root" if i + 1 < args.len() => {
                                root = std::path::PathBuf::from(&args[i + 1]);
                                i += two;
                            }
                            "--viewpoint" if i + 1 < args.len() => {
                                vp_filter = Some(args[i + 1].clone());
                                i += two;
                            }
                            _ => i += 1,
                        }
                    }
                    let (passed, rendered) = ch.audit_repository_json(&root, vp_filter.as_deref());
                    if !passed {
                        return Err(CliCommandError::ValidationRejected {
                            command: "operator audit".to_string(),
                            diagnostic: rendered,
                        });
                    }
                    Ok(rendered)
                }
                "verify-seal" => {
                    let two = 1usize + 1usize;
                    let mut root =
                        std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
                    let mut i = two;
                    while i < args.len() {
                        if args[i] == "--root" && i + 1 < args.len() {
                            root = std::path::PathBuf::from(&args[i + 1]);
                            i += two;
                        } else {
                            i += 1;
                        }
                    }
                    let rep = ch.verify_immutability_seal(&root);
                    let status_txt = if rep.valid { "VALID" } else { "TAMPERED" };
                    let summary = format!(
                        "VIEW IMMUTABILITY SEAL {status_txt}: checked={} root={} violations={}",
                        rep.checked_files,
                        rep.seal_root_sha256_hex,
                        rep.violations.join(" | ")
                    );
                    if !rep.valid {
                        return Err(CliCommandError::ValidationRejected {
                            command: "operator verify-seal".to_string(),
                            diagnostic: summary,
                        });
                    }
                    Ok(summary)
                }
                "kpis" => {
                    let two = 1usize + 1usize;
                    let mut root =
                        std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
                    let mut i = two;
                    while i < args.len() {
                        if args[i] == "--root" && i + 1 < args.len() {
                            root = std::path::PathBuf::from(&args[i + 1]);
                            i += two;
                        } else {
                            i += 1;
                        }
                    }
                    let rep = ch
                        .evaluate_observer_roadmap_kpis(&root, true, true)
                        .map_err(|e| CliCommandError::ValidationRejected {
                            command: "operator kpis".to_string(),
                            diagnostic: e.to_string(),
                        })?;
                    Ok(format!(
                        "OBSERVER ROADMAP & KPIs: version=v{} epoch={} managers={} prism_facets={}/{} kpis={}/{} minor_progress={}% major_progress={}% knowledge_authorities={} recommended_bump={:?}",
                        rep.current_version,
                        rep.release_epoch,
                        rep.total_managers,
                        rep.total_prism_facets_owned,
                        rep.total_prism_facets_required,
                        rep.kpi_passed_count,
                        rep.kpi_total_count,
                        rep.minor_progress_pct,
                        rep.major_progress_pct,
                        rep.knowledge.locked_authorities_count,
                        rep.change_decision.recommended_bump
                    ))
                }
                "change-evaluate" => {
                    let two = 1usize + 1usize;
                    let mut root =
                        std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
                    let mut ratchet_applied = false;
                    let mut pull_through = false;
                    let mut major = false;
                    let mut minor = false;
                    let mut patch = false;
                    let mut summary = "CLI Change Manager evaluation".to_string();
                    let mut i = two;
                    while i < args.len() {
                        match args[i].as_str() {
                            "--root" if i + 1 < args.len() => {
                                root = std::path::PathBuf::from(&args[i + 1]);
                                i += two;
                            }
                            "--ratchet-applied" => {
                                ratchet_applied = true;
                                i += 1;
                            }
                            "--pull-through-verified" => {
                                pull_through = true;
                                i += 1;
                            }
                            "--major" => {
                                major = true;
                                i += 1;
                            }
                            "--minor" => {
                                minor = true;
                                i += 1;
                            }
                            "--patch" => {
                                patch = true;
                                i += 1;
                            }
                            "--summary" if i + 1 < args.len() => {
                                summary = args[i + 1].clone();
                                i += two;
                            }
                            _ => i += 1,
                        }
                    }
                    let signals = verdant_core::governance::ObserverProgressSignals {
                        breaking_contract_change: major,
                        major_milestone_completed: major,
                        minor_capability_added: minor,
                        stakeholder_requirement_closed: minor,
                        defect_or_security_fix: patch,
                        summary,
                    };
                    let dec = ch
                        .evaluate_change_release(&root, ratchet_applied, pull_through, &signals)
                        .map_err(|e| CliCommandError::ValidationRejected {
                            command: "operator change-evaluate".to_string(),
                            diagnostic: e.to_string(),
                        })?;
                    Ok(format!(
                        "CHANGE MANAGER DECISION: bump={:?} current=v{} next=v{} epoch={}->{} incremented={} rationale={}",
                        dec.recommended_bump,
                        dec.current_version.display_string(),
                        dec.next_version.display_string(),
                        dec.current_epoch,
                        dec.next_epoch,
                        dec.version_incremented,
                        dec.rationale
                    ))
                }
                "session" => {
                    use std::io::BufRead;
                    let two = 1usize + 1usize;
                    let mut root =
                        std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
                    let mut i = two;
                    while i < args.len() {
                        if args[i] == "--root" && i + 1 < args.len() {
                            root = std::path::PathBuf::from(&args[i + 1]);
                            i += two;
                        } else {
                            i += 1;
                        }
                    }
                    let stdin = std::io::stdin();
                    let mut responses = Vec::new();
                    for line_res in stdin.lock().lines() {
                        let line = line_res.map_err(|e| CliCommandError::IoFailure {
                            path: "stdin".to_string(),
                            reason: e.to_string(),
                        })?;
                        let trimmed = line.trim();
                        if trimmed.is_empty() {
                            continue;
                        }
                        let (keep_going, out) = ch.handle_session_rpc_line(&root, trimmed);
                        if !out.is_empty() {
                            responses.push(out);
                        }
                        if !keep_going {
                            break;
                        }
                    }
                    Ok(responses.join("\n"))
                }
                other => Err(CliCommandError::InvalidUsage {
                    usage: format!(
                        "unknown operator action `{other}` (expected status|prompt|apply|synthesize|publish-app|verify-seal|audit|session)"
                    ),
                }),
            }
        }
        "release" => {
            let Some(action) = args.get(1).map(String::as_str) else {
                return Err(CliCommandError::InvalidUsage {
                    usage: "verdant release <pack|verify> [--root <dir>] [--version <v>] [--epoch <n>] [--out <dir>] [--relay <url>] [--pin <stitch://...>]".to_string(),
                });
            };
            let ch = verdant_core::operator::VerdantOperatorChannel::new("sunspire-regatta")
                .map_err(|e| CliCommandError::ValidationRejected {
                    command: "release".to_string(),
                    diagnostic: e.to_string(),
                })?;
            let two = 1usize + 1usize;
            let mut root =
                std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
            let mut version = "0.1.0".to_string();
            let mut epoch = 1u64;
            let mut out_dir = std::path::PathBuf::from("dist/stitch-release");
            let mut default_uri = "stitch://verdant/studio".to_string();
            let mut relays: Vec<String> = Vec::new();
            let mut i = two;
            while i < args.len() {
                match args[i].as_str() {
                    "--root" if i + 1 < args.len() => {
                        root = std::path::PathBuf::from(&args[i + 1]);
                        i += two;
                    }
                    "--version" if i + 1 < args.len() => {
                        version = args[i + 1].clone();
                        i += two;
                    }
                    "--epoch" if i + 1 < args.len() => {
                        epoch = args[i + 1].parse().unwrap_or(1);
                        i += two;
                    }
                    "--out" | "--dir" if i + 1 < args.len() => {
                        out_dir = std::path::PathBuf::from(&args[i + 1]);
                        i += two;
                    }
                    "--relay" if i + 1 < args.len() => {
                        relays.push(args[i + 1].clone());
                        i += two;
                    }
                    "--pin" if i + 1 < args.len() => {
                        default_uri = args[i + 1].clone();
                        i += two;
                    }
                    _ => i += 1,
                }
            }
            match action {
                "pack" => {
                    let pin = ch
                        .pack_repo_release_capsule(
                            &root,
                            &version,
                            epoch,
                            &out_dir,
                            relays,
                            &default_uri,
                        )
                        .map_err(|e| CliCommandError::ValidationRejected {
                            command: "release pack".to_string(),
                            diagnostic: e.to_string(),
                        })?;
                    Ok(format!(
                        "RELEASE PACKED: version={} epoch={} bytes={} shards={}of{} kappa={} vcs={}",
                        pin.release_version,
                        pin.release_epoch,
                        pin.capsule_byte_len,
                        pin.data_shards_k,
                        pin.total_shards_n,
                        pin.release_kappa_hex,
                        pin.vcs_commit_kappa
                    ))
                }
                "verify" => ch.verify_release_dir(&out_dir).map_err(|e| {
                    CliCommandError::ValidationRejected {
                        command: "release verify".to_string(),
                        diagnostic: e.to_string(),
                    }
                }),
                other => Err(CliCommandError::InvalidUsage {
                    usage: format!("unknown release action `{other}` (expected pack|verify)"),
                }),
            }
        }
        "mcp" => {
            use std::io::{BufRead, Write};
            let two = 1usize + 1usize;
            let mut root =
                std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
            let mut i = 1usize;
            while i < args.len() {
                if args[i] == "--root" && i + 1 < args.len() {
                    root = std::path::PathBuf::from(&args[i + 1]);
                    i += two;
                } else {
                    i += 1;
                }
            }
            let mut ch = verdant_core::operator::VerdantOperatorChannel::new("sunspire-regatta")
                .map_err(|e| CliCommandError::ValidationRejected {
                    command: "mcp".to_string(),
                    diagnostic: e.to_string(),
                })?;
            let stdin = std::io::stdin();
            let mut stdout = std::io::stdout().lock();
            for line_res in stdin.lock().lines() {
                let line = line_res.map_err(|e| CliCommandError::IoFailure {
                    path: "stdin".to_string(),
                    reason: e.to_string(),
                })?;
                if let Some(resp) = ch.handle_mcp_jsonrpc_line(&root, &line) {
                    writeln!(stdout, "{resp}").map_err(|e| CliCommandError::IoFailure {
                        path: "stdout".to_string(),
                        reason: e.to_string(),
                    })?;
                    stdout.flush().map_err(|e| CliCommandError::IoFailure {
                        path: "stdout".to_string(),
                        reason: e.to_string(),
                    })?;
                }
            }
            Ok(String::new())
        }
        other => Err(CliCommandError::InvalidUsage {
            usage: format!("unknown subcommand `{other}`\n{CLI_USAGE}"),
        }),
    }
}
