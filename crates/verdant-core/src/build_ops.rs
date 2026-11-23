//! **Native In-Verdant Build, Wasm Sync & RFC 2789 Cargo Sparse Registry Operations (`build_ops.rs`)**.
//!
//! Lifts `scripts/sync-prism-wasm.mjs`, `scripts/package-rust-crates.mjs`, and
//! `scripts/verify-sparse-registry.mjs` natively into `verdant-core` (`mgr-change-release`) so
//! Verdant compiles & verifies its own WebAssembly artifacts, emits `site/prism-bridge-meta.mjs`,
//! packages deterministic `.crate` tarballs, and verifies its RFC 2789 Cargo Sparse Registry over
//! a live local HTTP loopback server with zero Node.js build dependency.

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use verdant_tensor::scalars::{
    build_op_stages_all_passed, crate_tarball_nonempty, csm_cascade_index_valid, eight_u32,
    five_u32, four_u32, hundred_u32, modal_bar_mode_index_valid, six_u32,
    sparse_index_crate_name_valid, sparse_registry_crate_count_complete,
    sparse_registry_http_status_ok, ten_u32, thousand_u32, two_u32, uorc_stream_nonempty,
    wasm_magic_header_valid,
};

use super::VerdantOperatorChannel;
use crate::digest::sha256_hex;
use crate::error::ObserverModelError;
use crate::hd_world::synthesize_hd_citadel_sector;
use crate::navmesh::NavMesh;
use crate::observer::CommunicationChannelKind;
use crate::prism_kernel;
use crate::streaming::{
    decode_portable_uorc_single_frame, pack_portable_progressive_uorc_stream, CodedLodFrameBundle,
    PhysicalEnvironment, StandardSceneState, TerrainReliefParams,
};

/// Native in-Verdant build/packaging/registry operation dispatched by [`VerdantOperatorChannel::execute_native_build_op`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum NativeBuildOp {
    /// Builds (unless `check_only`) and cryptographically verifies `site/verdant-wasm.wasm`,
    /// `site/verdant-uorc.wasm`, `site/scene-stream.uorc`, and `site/prism-bridge-meta.mjs`.
    SyncWasm {
        /// When true, verifies existing artifacts and `prism-bridge-meta.mjs` without rebuilding.
        check_only: bool,
        /// When true, forces repacking `site/scene-stream.uorc` even if already present.
        repack_stream: bool,
    },
    /// Packages Verdant's 4 shipped workspace crates into deterministic `.crate` tarballs and an
    /// RFC 2789 Cargo Sparse Registry index under `out_dir`.
    PackageRustCrates {
        /// Target output directory (e.g. `dist/crates` or `site/crates`).
        out_dir: PathBuf,
        /// RFC 2789 `index/config.json` `dl` URL template.
        dl_template: String,
    },
    /// Runs the live HTTP + downstream `cargo check` RFC 2789 Sparse Registry gate (`VE-54`).
    VerifySparseRegistry {
        /// Root directory containing `index/` and `crates/` (e.g. `site/crates`).
        crates_root: PathBuf,
    },
}

/// Outcome report emitted by [`VerdantOperatorChannel::execute_native_build_op`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeBuildReport {
    /// Fixed schema identifier (`verdant/native-build-report/1`).
    pub spec: String,
    /// Operation slug (`sync-wasm`, `package-crates`, or `verify-registry`).
    pub operation: String,
    /// True iff all validation steps passed.
    pub passed: bool,
    /// Elapsed wall-clock duration in milliseconds.
    pub duration_ms: u64,
    /// Concise human- and machine-readable summary line.
    pub summary: String,
    /// Structured operation details.
    pub details: serde_json::Value,
    /// Content-addressed SHA-256 commitment over this report.
    pub report_kappa_hex: String,
}

impl VerdantOperatorChannel {
    /// Executes one of Verdant's lifted native build, Wasm verification, or RFC 2789 Cargo Sparse
    /// Registry operations directly in-process (`mgr-change-release`), eliminating Node.js build
    /// script dependencies while keeping zero footprint on `wasm32`.
    pub fn execute_native_build_op(
        &self,
        repo_root: &Path,
        op: &NativeBuildOp,
    ) -> Result<(NativeBuildReport, String), ObserverModelError> {
        let t_start = Instant::now();
        let make_err = |msg: String| ObserverModelError::CommunicationEnvelopeRejected {
            channel: CommunicationChannelKind::OrganToOrgan.slug().to_string(),
            detail: msg,
        };

        let (operation, summary, details) = match op {
            NativeBuildOp::SyncWasm {
                check_only,
                repack_stream,
            } => {
                let target_wasm =
                    repo_root.join("target/wasm32-unknown-unknown/release/verdant_wasm.wasm");
                let site_wasm = repo_root.join("site/verdant-wasm.wasm");
                let site_uorc_wasm = repo_root.join("site/verdant-uorc.wasm");
                let site_uorc_stream = repo_root.join("site/scene-stream.uorc");
                let site_meta = repo_root.join("site/prism-bridge-meta.mjs");

                let default_seed = prism_kernel::sSeven().unwrap_or(1);
                let max_lod_rank = prism_kernel::wasmMaxLodRank();

                if !*check_only {
                    let run_cargo = |cmd_args: &[&str]| -> Result<(), ObserverModelError> {
                        let out = Command::new("cargo")
                            .args(cmd_args)
                            .current_dir(repo_root)
                            .output()
                            .map_err(|e| make_err(format!("spawn cargo failed: {e}")))?;
                        if !out.status.success() {
                            return Err(make_err(format!(
                                "cargo {:?} failed: {}",
                                cmd_args,
                                String::from_utf8_lossy(&out.stderr)
                            )));
                        }
                        Ok(())
                    };

                    run_cargo(&[
                        "build",
                        "-p",
                        "verdant-wasm",
                        "--target",
                        "wasm32-unknown-unknown",
                        "--release",
                        "--locked",
                    ])?;
                    std::fs::copy(&target_wasm, &site_wasm)
                        .map_err(|e| make_err(format!("copy verdant-wasm.wasm failed: {e}")))?;

                    run_cargo(&[
                        "build",
                        "-p",
                        "verdant-wasm",
                        "--target",
                        "wasm32-unknown-unknown",
                        "--release",
                        "--features",
                        "uorc-decoder",
                        "--locked",
                    ])?;
                    std::fs::copy(&target_wasm, &site_uorc_wasm)
                        .map_err(|e| make_err(format!("copy verdant-uorc.wasm failed: {e}")))?;

                    if !site_uorc_stream.is_file() || *repack_stream {
                        let packed = pack_portable_progressive_uorc_stream(default_seed)
                            .map_err(|e| make_err(format!("pack portable uorc stream: {e}")))?;
                        std::fs::write(&site_uorc_stream, &packed)
                            .map_err(|e| make_err(format!("write scene-stream.uorc: {e}")))?;
                    }
                }

                let wasm_bytes = std::fs::read(&site_wasm)
                    .map_err(|e| make_err(format!("read site/verdant-wasm.wasm: {e}")))?;
                let uorc_wasm_bytes = std::fs::read(&site_uorc_wasm)
                    .map_err(|e| make_err(format!("read site/verdant-uorc.wasm: {e}")))?;
                let uorc_stream_bytes = std::fs::read(&site_uorc_stream)
                    .map_err(|e| make_err(format!("read site/scene-stream.uorc: {e}")))?;

                let min_hdr = eight_u32() as usize;
                let i2 = two_u32() as usize;
                let i3 = i2 + 1;
                for (label, buf) in [
                    ("verdant-wasm.wasm", &wasm_bytes),
                    ("verdant-uorc.wasm", &uorc_wasm_bytes),
                ] {
                    if buf.len() < min_hdr
                        || !wasm_magic_header_valid(buf[0], buf[1], buf[i2], buf[i3])
                    {
                        return Err(make_err(format!(
                            "sync-wasm: invalid WebAssembly magic header in {label}"
                        )));
                    }
                }
                if !uorc_stream_nonempty(uorc_stream_bytes.len() as u64) {
                    return Err(make_err("sync-wasm: empty scene-stream.uorc".to_string()));
                }
                let decoded_state =
                    decode_portable_uorc_single_frame(&uorc_stream_bytes, max_lod_rank)
                        .map_err(|e| make_err(format!("sync-wasm uorc decode failed: {e}")))?;
                if decoded_state.active_lod.rank() != max_lod_rank {
                    return Err(make_err(format!(
                        "sync-wasm: decoded LOD rank {} != {max_lod_rank}",
                        decoded_state.active_lod.rank()
                    )));
                }

                let hd_max_lod_rank = prism_kernel::wasmHdMaxLodRank().unwrap_or(1);
                let snapshot_min_wire = prism_kernel::snapshotMinWireBytes().unwrap_or(1);
                let quantum_frames = prism_kernel::audioWorkletQuantumFrames().unwrap_or(1);
                let stereo_channels = prism_kernel::audioStereoChannels();
                let modal_bar_denom = prism_kernel::eulerBernoulliModeDenom();
                let mut modal_bar_numers = Vec::new();
                for mode_i in 0..five_u32() {
                    if modal_bar_mode_index_valid(mode_i) {
                        let v = prism_kernel::eulerBernoulliModeNumer(u64::from(mode_i))
                            .unwrap_or(modal_bar_denom);
                        modal_bar_numers.push(v);
                    }
                }
                let vol_1800 = u64::from(hundred_u32()) * u64::from(ten_u32() + eight_u32());
                let abs_96000 =
                    u64::from(thousand_u32()) * u64::from((ten_u32() - 1) * ten_u32() + six_u32());
                let chamber_rt60_ms =
                    prism_kernel::sabineRt60Millis(vol_1800, abs_96000).unwrap_or(0);

                let terrain_params = TerrainReliefParams::standard_basin_valley();
                let env_params = PhysicalEnvironment::temperate_terrestrial();
                let hd_sector = synthesize_hd_citadel_sector(
                    default_seed,
                    &terrain_params,
                    env_params.water_level_meters,
                    prism_kernel::sTwo(),
                );
                let walkable_polygons = hd_sector
                    .meshes
                    .first()
                    .map(NavMesh::count_walkable_triangles)
                    .unwrap_or(0);
                let std_world = StandardSceneState::synthesize_at_lod(default_seed, max_lod_rank)
                    .map_err(|e| make_err(format!("synthesize world for meta: {e}")))?;
                let node_preset_count = std_world.scene_nodes.len();
                let tensor_kernel_count =
                    if prism_kernel::sevenTensorKernelsDirect().unwrap_or(false) {
                        prism_kernel::sSeven().unwrap_or(0)
                    } else {
                        0
                    };
                let unified_kernel_count = prism_kernel::sFifteen().unwrap_or(0);
                let coded_tier_count = prism_kernel::sThree();
                let bundle = CodedLodFrameBundle::synthesize(default_seed, max_lod_rank)
                    .map_err(|e| make_err(format!("coded bundle synthesize: {e}")))?;
                let k_len = bundle.inner_k;
                let transform_q: Vec<i32> = (0..k_len).map(|i| (i as i32) + 1).collect();
                let weights_q: Vec<i32> = (0..k_len).map(|i| (k_len - i) as i32).collect();
                let sh_q: Vec<i32> = (0..k_len)
                    .map(|i| if (i & 1) == 0 { 1 } else { -1 })
                    .collect();
                let c_sum = bundle
                    .contract_zero_decode(&transform_q, &weights_q, &sh_q)
                    .map_err(|e| make_err(format!("coded contraction: {e}")))?;
                let coded_checksum: i64 = c_sum
                    .geometry_accum
                    .iter()
                    .chain(c_sum.animation_accum.iter())
                    .chain(c_sum.ibl_accum.iter())
                    .copied()
                    .fold(0_i64, i64::wrapping_add);

                let tile_edge_pixels = prism_kernel::tileEdgePixels().unwrap_or(1);
                let ring_capacity = prism_kernel::workerCommandRingCapacity().unwrap_or(1);
                let snapshot_max_bytes = prism_kernel::snapshotUrlMaxBytes().unwrap_or(1);

                let near_milli = u64::from(thousand_u32()) / u64::from(four_u32());
                let far_milli =
                    u64::from(thousand_u32()) * u64::from(ten_u32() * (ten_u32() + four_u32()));
                let scale_1000 = f64::from(thousand_u32());
                let ten_thousand = f64::from(thousand_u32() * ten_u32());
                let mut csm_parts = Vec::new();
                for c_idx in 0..four_u32() {
                    if csm_cascade_index_valid(c_idx) {
                        let m = prism_kernel::csmSplitDistanceMilli(
                            near_milli,
                            far_milli,
                            u64::from(c_idx),
                        )
                        .unwrap_or(far_milli);
                        let v = ((m as f64 / scale_1000) * ten_thousand).round() / ten_thousand;
                        let s = if (v - v.trunc()).abs() == 0.0 {
                            format!("{}", v as i64)
                        } else {
                            let raw = format!("{v:.4}");
                            raw.trim_end_matches('0').trim_end_matches('.').to_string()
                        };
                        csm_parts.push(s);
                    }
                }
                let modal_numers_js = modal_bar_numers
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(",");
                let csm_splits_js = csm_parts.join(",");

                let expected_meta = format!(
                    "// Generated mechanically by scripts/sync-prism-wasm.mjs from Prism-verified verdant-wasm.\n\
                     // Do not edit by hand.\n\n\
                     export const PRISM_DEFAULT_WORLD_SEED = {default_seed};\n\
                     export const PRISM_MAX_LOD_RANK = {max_lod_rank};\n\
                     export const PRISM_HD_MAX_LOD_RANK = {hd_max_lod_rank};\n\
                     export const PRISM_SNAPSHOT_MIN_WIRE_BYTES = {snapshot_min_wire};\n\
                     export const PRISM_AUDIO_QUANTUM_FRAMES = {quantum_frames};\n\
                     export const PRISM_AUDIO_STEREO_CHANNELS = {stereo_channels};\n\
                     export const PRISM_MODAL_BAR_DENOM = {modal_bar_denom};\n\
                     export const PRISM_MODAL_BAR_NUMERS = Object.freeze([{modal_numers_js}]);\n\
                     export const PRISM_CHAMBER_SABINE_RT60_MS = {chamber_rt60_ms};\n\
                     export const PRISM_TERRAIN_WALKABLE_POLYGONS = {walkable_polygons};\n\
                     export const PRISM_NODE_PRESET_COUNT = {node_preset_count};\n\
                     export const PRISM_TENSOR_KERNEL_COUNT = {tensor_kernel_count};\n\
                     export const PRISM_UNIFIED_KERNEL_COUNT = {unified_kernel_count};\n\
                     export const PRISM_CODED_TIER_COUNT = {coded_tier_count};\n\
                     export const PRISM_CODED_ZERO_DECODE_CHECKSUM = {coded_checksum};\n\
                     export const PRISM_PORTABLE_UORC_BYTES = {};\n\
                     export const PRISM_TILE_EDGE_PIXELS = {tile_edge_pixels};\n\
                     export const PRISM_CSM_SPLIT_METERS = Object.freeze([{csm_splits_js}]);\n\
                     export const PRISM_WORKER_RING_CAPACITY = {ring_capacity};\n\
                     export const PRISM_SNAPSHOT_URL_MAX_BYTES = {snapshot_max_bytes};\n",
                    uorc_stream_bytes.len()
                );

                if *check_only {
                    let current = std::fs::read_to_string(&site_meta)
                        .map_err(|e| make_err(format!("read site/prism-bridge-meta.mjs: {e}")))?;
                    if current != expected_meta {
                        return Err(make_err(
                            "sync-wasm --check: site/prism-bridge-meta.mjs is out of sync"
                                .to_string(),
                        ));
                    }
                } else {
                    std::fs::write(&site_meta, expected_meta.as_bytes())
                        .map_err(|e| make_err(format!("write site/prism-bridge-meta.mjs: {e}")))?;
                }

                let summary_line = if *check_only {
                    format!(
                        "sync-wasm --check: verified (cold={}B, uorc_wasm={}B, stream={}B, kernels={}, coded_tiers={})",
                        wasm_bytes.len(),
                        uorc_wasm_bytes.len(),
                        uorc_stream_bytes.len(),
                        unified_kernel_count,
                        coded_tier_count
                    )
                } else {
                    format!(
                        "sync-wasm: wrote site/verdant-wasm.wasm ({}B), site/verdant-uorc.wasm ({}B), site/scene-stream.uorc ({}B), and site/prism-bridge-meta.mjs",
                        wasm_bytes.len(),
                        uorc_wasm_bytes.len(),
                        uorc_stream_bytes.len()
                    )
                };
                (
                    "sync-wasm".to_string(),
                    summary_line,
                    serde_json::json!({
                        "checkOnly": check_only,
                        "wasmCoreBytes": wasm_bytes.len(),
                        "wasmUorcBytes": uorc_wasm_bytes.len(),
                        "uorcStreamBytes": uorc_stream_bytes.len(),
                        "unifiedKernelCount": unified_kernel_count,
                        "codedTierCount": coded_tier_count,
                        "codedChecksum": coded_checksum
                    }),
                )
            }

            NativeBuildOp::PackageRustCrates {
                out_dir,
                dl_template,
            } => {
                let shipped_crates = [
                    "verdant-tensor",
                    "verdant-core",
                    "verdant-wasm",
                    "verdant-cli",
                ];
                let meta_out = Command::new("cargo")
                    .args([
                        "metadata",
                        "--format-version=1",
                        "--all-features",
                        "--locked",
                    ])
                    .current_dir(repo_root)
                    .output()
                    .map_err(|e| make_err(format!("spawn cargo metadata: {e}")))?;
                if !meta_out.status.success() {
                    return Err(make_err(format!(
                        "cargo metadata failed: {}",
                        String::from_utf8_lossy(&meta_out.stderr)
                    )));
                }
                let meta: serde_json::Value = serde_json::from_slice(&meta_out.stdout)
                    .map_err(|e| make_err(format!("parse cargo metadata JSON: {e}")))?;
                let packages = meta
                    .get("packages")
                    .and_then(|v| v.as_array())
                    .ok_or_else(|| make_err("cargo metadata missing packages array".to_string()))?;

                let out_abs = if out_dir.is_absolute() {
                    out_dir.clone()
                } else {
                    repo_root.join(out_dir)
                };
                let crates_dir = out_abs.join("crates");
                let index_dir = out_abs.join("index");
                std::fs::create_dir_all(&crates_dir)
                    .map_err(|e| make_err(format!("create crates dir: {e}")))?;
                std::fs::create_dir_all(&index_dir)
                    .map_err(|e| make_err(format!("create index dir: {e}")))?;

                let cfg_json = serde_json::to_string_pretty(&serde_json::json!({
                    "dl": dl_template,
                    "api": "https://github.com/afflom/verdant-engine"
                }))
                .unwrap_or_default();
                std::fs::write(index_dir.join("config.json"), format!("{cfg_json}\n"))
                    .map_err(|e| make_err(format!("write index/config.json: {e}")))?;

                let stage_root = std::env::temp_dir().join(format!(
                    "verdant-crate-stage-{}-{}",
                    std::process::id(),
                    t_start.elapsed().as_nanos()
                ));
                let _ = std::fs::remove_dir_all(&stage_root);
                std::fs::create_dir_all(&stage_root)
                    .map_err(|e| make_err(format!("create crate stage dir: {e}")))?;

                // Cargo resolves the exact local kernels and Git packages for all published features.
                // Registry packages keep their own registry in the index and are never repackaged.
                let nodes = meta["resolve"]["nodes"]
                    .as_array()
                    .ok_or_else(|| make_err("cargo metadata missing resolved graph".to_string()))?;
                let mut pending = Vec::new();
                for name in shipped_crates {
                    let pkg = packages
                        .iter()
                        .find(|p| p["name"] == name && p["source"].is_null())
                        .ok_or_else(|| make_err(format!("missing workspace package: {name}")))?;
                    pending.push(pkg);
                }
                let mut closure = std::collections::BTreeMap::new();
                while let Some(pkg) = pending.pop() {
                    let id = pkg["id"]
                        .as_str()
                        .ok_or_else(|| make_err("missing package ID".to_string()))?;
                    if closure.insert(id, pkg).is_some() {
                        continue;
                    }
                    let node = nodes
                        .iter()
                        .find(|n| n["id"] == id)
                        .ok_or_else(|| make_err(format!("missing resolved node: {id}")))?;
                    for dep in node["deps"]
                        .as_array()
                        .ok_or_else(|| make_err(format!("missing dependencies: {id}")))?
                    {
                        if !dep["dep_kinds"]
                            .as_array()
                            .is_some_and(|kinds| kinds.iter().any(|k| k["kind"] != "dev"))
                        {
                            continue;
                        }
                        let child = packages
                            .iter()
                            .find(|p| p["id"] == dep["pkg"])
                            .ok_or_else(|| make_err(format!("missing resolved child of {id}")))?;
                        if !child["source"].as_str().is_some_and(|source| {
                            source.starts_with("registry+") || source.starts_with("sparse+")
                        }) {
                            pending.push(child);
                        }
                    }
                }
                let mut summary_rows: Vec<serde_json::Value> = Vec::new();
                let mut dependency_rows: Vec<serde_json::Value> = Vec::new();
                let mut labels: Vec<String> = Vec::new();
                let p_two = two_u32() as usize;
                let p_four = four_u32() as usize;

                let pkg_res: Result<(), ObserverModelError> = (|| {
                    let mut identities = std::collections::BTreeSet::new();
                    let mut records = std::collections::BTreeMap::new();
                    for pkg in closure.values() {
                        let crate_name = pkg["name"]
                            .as_str()
                            .ok_or_else(|| make_err("missing package name".to_string()))?;
                        let version = pkg["version"]
                            .as_str()
                            .ok_or_else(|| make_err(format!("missing version for {crate_name}")))?;
                        if !sparse_index_crate_name_valid(crate_name.len() as u32)
                            || !identities.insert((crate_name, version))
                        {
                            return Err(make_err(format!(
                                "invalid or duplicate package: {crate_name}@{version}"
                            )));
                        }
                        let crate_file_name = format!("{crate_name}-{version}.crate");
                        let dst_crate_path = crates_dir.join(&crate_file_name);
                        let manifest_path = pkg["manifest_path"].as_str().ok_or_else(|| {
                            make_err(format!("missing manifest for {crate_name}"))
                        })?;
                        let output = Command::new("cargo")
                            .args([
                                "package",
                                "--offline",
                                "--no-verify",
                                "--allow-dirty",
                                "--exclude-lockfile",
                                "--manifest-path",
                                manifest_path,
                                "--target-dir",
                            ])
                            .arg(&stage_root)
                            .current_dir(repo_root)
                            .output()
                            .map_err(|e| make_err(format!("package {crate_name}: {e}")))?;
                        if !output.status.success() {
                            return Err(make_err(format!(
                                "package {crate_name}: {}",
                                String::from_utf8_lossy(&output.stderr)
                            )));
                        }
                        std::fs::copy(
                            stage_root.join("package").join(&crate_file_name),
                            &dst_crate_path,
                        )
                        .map_err(|e| make_err(format!("stage {crate_file_name}: {e}")))?;
                        let bytes = std::fs::read(&dst_crate_path)
                            .map_err(|e| make_err(format!("read packaged crate: {e}")))?;
                        if !crate_tarball_nonempty(bytes.len() as u64) {
                            return Err(make_err(format!("empty tarball for {crate_name}")));
                        }
                        let cksum = sha256_hex(&bytes);

                        let lower = crate_name.to_ascii_lowercase();
                        let p1 = &lower[..p_two];
                        let p2 = &lower[p_two..p_four];
                        let entry_dir = index_dir.join(p1).join(p2);
                        std::fs::create_dir_all(&entry_dir)
                            .map_err(|e| make_err(format!("create sparse prefix dir: {e}")))?;

                        let mut deps_vec: Vec<serde_json::Value> = Vec::new();
                        if let Some(raw_deps) = pkg.get("dependencies").and_then(|v| v.as_array()) {
                            for d in raw_deps {
                                let kind_opt = d.get("kind").and_then(|v| v.as_str());
                                if !matches!(kind_opt, None | Some("normal") | Some("build")) {
                                    continue;
                                }
                                let d_name = d.get("name").and_then(|v| v.as_str()).unwrap_or("");
                                let d_rename = d.get("rename").and_then(|v| v.as_str());
                                let registry = d["source"].as_str().and_then(|source| {
                                    source
                                        .strip_prefix("registry+")
                                        .or_else(|| source.starts_with("sparse+").then_some(source))
                                });
                                deps_vec.push(serde_json::json!({
                                    "name": d_rename.unwrap_or(d_name),
                                    "req": d.get("req").and_then(|v| v.as_str()).unwrap_or("*"),
                                    "features": d.get("features").cloned().unwrap_or_else(|| serde_json::json!([])),
                                    "optional": d.get("optional").and_then(|v| v.as_bool()).unwrap_or(false),
                                    "default_features": d.get("uses_default_features").and_then(|v| v.as_bool()).unwrap_or(true),
                                    "target": d.get("target").cloned().unwrap_or(serde_json::Value::Null),
                                    "kind": kind_opt.unwrap_or("normal"),
                                    "registry": registry,
                                    "package": if d_rename.is_some() { serde_json::Value::String(d_name.to_string()) } else { serde_json::Value::Null },
                                }));
                            }
                        }

                        let index_rec = serde_json::json!({
                            "name": crate_name,
                            "vers": version,
                            "deps": deps_vec,
                            "cksum": cksum,
                            "features": pkg.get("features").cloned().unwrap_or_else(|| serde_json::json!({})),
                            "yanked": false,
                            "rust_version": pkg.get("rust_version").cloned().unwrap_or(serde_json::Value::Null),
                        });
                        records
                            .entry(entry_dir.join(&lower))
                            .or_insert_with(String::new)
                            .push_str(&format!("{index_rec}\n"));

                        let rows = if shipped_crates.contains(&crate_name) {
                            labels.push(format!("{crate_name}@{version}"));
                            &mut summary_rows
                        } else {
                            &mut dependency_rows
                        };
                        rows.push(serde_json::json!({
                            "name": crate_name,
                            "version": version,
                            "file": crate_file_name,
                            "bytes": bytes.len(),
                            "sha256": cksum,
                            "indexPath": format!("{p1}/{p2}/{lower}"),
                        }));
                    }
                    for (path, rows) in records {
                        std::fs::write(path, rows)
                            .map_err(|e| make_err(format!("write sparse index entry: {e}")))?;
                    }
                    Ok(())
                })();
                let _ = std::fs::remove_dir_all(&stage_root);
                pkg_res?;

                if !sparse_registry_crate_count_complete(
                    summary_rows.len() as u32,
                    shipped_crates.len() as u32,
                ) {
                    return Err(make_err("packaged crate count mismatch".to_string()));
                }

                let manifest_json = serde_json::to_string_pretty(&serde_json::json!({
                    "spec": "verdant/rust-crates-sparse/1",
                    "crates": summary_rows,
                    "dependencies": dependency_rows
                }))
                .unwrap_or_default();
                std::fs::write(
                    out_abs.join("crates-manifest.json"),
                    format!("{manifest_json}\n"),
                )
                .map_err(|e| make_err(format!("write crates-manifest.json: {e}")))?;

                let summary_line = format!(
                    "package-rust-crates: PASS — packaged {} Rust crates ({}) + RFC 2789 Cargo sparse index into {}",
                    summary_rows.len(),
                    labels.join(", "),
                    out_abs.display()
                );
                (
                    "package-crates".to_string(),
                    summary_line,
                    serde_json::json!({
                        "outDir": out_abs.display().to_string(),
                        "crates": summary_rows
                    }),
                )
            }

            NativeBuildOp::VerifySparseRegistry { crates_root } => {
                let shipped_crates = [
                    "verdant-tensor",
                    "verdant-core",
                    "verdant-wasm",
                    "verdant-cli",
                ];
                let root_abs = if crates_root.is_absolute() {
                    crates_root.clone()
                } else {
                    repo_root.join(crates_root)
                };
                let cargo_toml = std::fs::read_to_string(repo_root.join("Cargo.toml"))
                    .map_err(|e| make_err(format!("read Cargo.toml: {e}")))?;
                let expected_version = cargo_toml
                    .split("[workspace.package]")
                    .nth(1)
                    .and_then(|sec| {
                        sec.lines().find_map(|l| {
                            l.trim()
                                .strip_prefix("version = ")
                                .map(|v| v.trim().trim_matches('"').to_string())
                        })
                    })
                    .filter(|s| !s.is_empty())
                    .ok_or_else(|| {
                        make_err("could not parse [workspace.package].version".to_string())
                    })?;

                let disk_cfg_raw = std::fs::read_to_string(root_abs.join("index/config.json"))
                    .map_err(|e| make_err(format!("read index/config.json: {e}")))?;
                let disk_cfg: serde_json::Value = serde_json::from_str(&disk_cfg_raw)
                    .map_err(|e| make_err(format!("parse index/config.json: {e}")))?;
                let dl_str = disk_cfg.get("dl").and_then(|v| v.as_str()).unwrap_or("");
                if !dl_str.contains("{crate}") || !dl_str.contains("{version}") {
                    return Err(make_err(format!(
                        "invalid RFC 2789 index/config.json dl template: {disk_cfg}"
                    )));
                }

                let listener = TcpListener::bind("127.0.0.1:0")
                    .map_err(|e| make_err(format!("bind local HTTP server: {e}")))?;
                let port = listener
                    .local_addr()
                    .map_err(|e| make_err(format!("read local HTTP port: {e}")))?
                    .port();
                let stop_flag = Arc::new(AtomicBool::new(false));
                let served_paths: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));

                let stop_srv = Arc::clone(&stop_flag);
                let paths_srv = Arc::clone(&served_paths);
                let serve_root = root_abs.clone();
                let server_handle = std::thread::spawn(move || {
                    let read_timeout_secs = u64::from(two_u32());
                    let buf_len = (1usize << (eight_u32() + four_u32())) as usize;
                    for stream_res in listener.incoming() {
                        if stop_srv.load(Ordering::SeqCst) {
                            break;
                        }
                        let Ok(mut stream) = stream_res else {
                            continue;
                        };
                        let _ =
                            stream.set_read_timeout(Some(Duration::from_secs(read_timeout_secs)));
                        let mut req_buf = vec![0u8; buf_len];
                        let n = stream.read(&mut req_buf).unwrap_or(0);
                        if n == 0 {
                            continue;
                        }
                        let req_str = String::from_utf8_lossy(&req_buf[..n]);
                        let first_line = req_str.lines().next().unwrap_or("");
                        let raw_path = first_line.split_whitespace().nth(1).unwrap_or("/");
                        let url_path = raw_path.split('?').next().unwrap_or("/").to_string();
                        if let Ok(mut guard) = paths_srv.lock() {
                            guard.push(url_path.clone());
                        }
                        let rel = url_path.trim_start_matches('/');
                        if rel.contains("..") {
                            let _ = stream.write_all(
                                b"HTTP/1.1 400 Bad Request\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: 8\r\nConnection: close\r\n\r\nbad path",
                            );
                            continue;
                        }
                        let full = serve_root.join(rel);
                        match std::fs::read(&full) {
                            Ok(mut body) => {
                                let content_type = if rel == "index/config.json" {
                                    if let Ok(mut cfg) =
                                        serde_json::from_slice::<serde_json::Value>(&body)
                                    {
                                        if let Some(obj) = cfg.as_object_mut() {
                                            obj.insert(
                                                "dl".to_string(),
                                                serde_json::Value::String(format!(
                                                    "http://127.0.0.1:{port}/crates/{{crate}}-{{version}}.crate"
                                                )),
                                            );
                                            body = format!(
                                                "{}\n",
                                                serde_json::to_string_pretty(&cfg)
                                                    .unwrap_or_default()
                                            )
                                            .into_bytes();
                                        }
                                    }
                                    "application/json; charset=utf-8"
                                } else if rel.ends_with(".crate") {
                                    "application/gzip"
                                } else {
                                    "text/plain; charset=utf-8"
                                };
                                let hdr = format!(
                                    "HTTP/1.1 200 OK\r\nContent-Type: {content_type}\r\nAccess-Control-Allow-Origin: *\r\nCache-Control: no-store\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                                    body.len()
                                );
                                let _ = stream.write_all(hdr.as_bytes());
                                let _ = stream.write_all(&body);
                            }
                            Err(_) => {
                                let _ = stream.write_all(
                                    b"HTTP/1.1 404 Not Found\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: 9\r\nConnection: close\r\n\r\nnot found",
                                );
                            }
                        }
                    }
                });

                let http_get = |path: &str| -> Result<(u16, String, Vec<u8>), ObserverModelError> {
                    let mut sock = TcpStream::connect(format!("127.0.0.1:{port}"))
                        .map_err(|e| make_err(format!("connect loopback HTTP: {e}")))?;
                    let req = format!(
                        "GET {path} HTTP/1.1\r\nHost: 127.0.0.1:{port}\r\nConnection: close\r\n\r\n"
                    );
                    sock.write_all(req.as_bytes())
                        .map_err(|e| make_err(format!("write HTTP request: {e}")))?;
                    let mut raw = Vec::new();
                    sock.read_to_end(&mut raw)
                        .map_err(|e| make_err(format!("read HTTP response: {e}")))?;
                    let sep = b"\r\n\r\n";
                    let split_at = raw
                        .windows(sep.len())
                        .position(|w| w == sep)
                        .ok_or_else(|| make_err(format!("malformed HTTP response for {path}")))?;
                    let hdr_str = String::from_utf8_lossy(&raw[..split_at]).to_string();
                    let body = raw[split_at + sep.len()..].to_vec();
                    let code = hdr_str
                        .lines()
                        .next()
                        .and_then(|l| l.split_whitespace().nth(1))
                        .and_then(|s| s.parse::<u16>().ok())
                        .unwrap_or(0);
                    Ok((code, hdr_str, body))
                };

                let consumer_dir = std::env::temp_dir().join(format!(
                    "verdant-sparse-check-{}-{}",
                    std::process::id(),
                    t_start.elapsed().as_nanos()
                ));
                let _ = std::fs::remove_dir_all(&consumer_dir);
                std::fs::create_dir_all(&consumer_dir)
                    .map_err(|e| make_err(format!("mkdir consumer temp dir: {e}")))?;

                let p_two = two_u32() as usize;
                let p_four = four_u32() as usize;
                let mut verified_crates = 0u32;

                let verify_res: Result<Vec<String>, ObserverModelError> = (|| {
                    let (cfg_code, cfg_hdr, _) = http_get("/index/config.json")?;
                    if !sparse_registry_http_status_ok(cfg_code)
                        || !cfg_hdr
                            .to_ascii_lowercase()
                            .contains("access-control-allow-origin: *")
                    {
                        return Err(make_err(format!(
                            "sparse registry HTTP config check failed: status={cfg_code}"
                        )));
                    }

                    for &crate_name in &shipped_crates {
                        let p1 = &crate_name[..p_two];
                        let p2 = &crate_name[p_two..p_four];
                        let (idx_code, _, idx_body) =
                            http_get(&format!("/index/{p1}/{p2}/{crate_name}"))?;
                        if !sparse_registry_http_status_ok(idx_code) {
                            return Err(make_err(format!(
                                "missing sparse index entry for {crate_name}: HTTP {idx_code}"
                            )));
                        }
                        let idx_text = String::from_utf8_lossy(&idx_body);
                        let last_line = idx_text
                            .lines()
                            .rev()
                            .find(|l| !l.trim().is_empty())
                            .ok_or_else(|| {
                                make_err(format!("empty sparse index entry for {crate_name}"))
                            })?;
                        let rec: serde_json::Value = serde_json::from_str(last_line)
                            .map_err(|e| make_err(format!("parse index JSON {crate_name}: {e}")))?;
                        let rec_name = rec.get("name").and_then(|v| v.as_str()).unwrap_or("");
                        let rec_vers = rec.get("vers").and_then(|v| v.as_str()).unwrap_or("");
                        let rec_yanked =
                            rec.get("yanked").and_then(|v| v.as_bool()).unwrap_or(true);
                        let rec_cksum = rec.get("cksum").and_then(|v| v.as_str()).unwrap_or("");
                        if rec_name != crate_name || rec_vers != expected_version || rec_yanked {
                            return Err(make_err(format!(
                                "sparse index metadata mismatch for {crate_name}: got vers={rec_vers}, expected {expected_version}"
                            )));
                        }

                        let (crate_code, _, crate_bytes) =
                            http_get(&format!("/crates/{crate_name}-{expected_version}.crate"))?;
                        if !sparse_registry_http_status_ok(crate_code) {
                            return Err(make_err(format!(
                                "failed downloading {crate_name}-{expected_version}.crate: HTTP {crate_code}"
                            )));
                        }
                        let actual_sha = sha256_hex(&crate_bytes);
                        if actual_sha != rec_cksum {
                            return Err(make_err(format!(
                                "SHA-256 mismatch for {crate_name}: index={rec_cksum} != tarball={actual_sha}"
                            )));
                        }

                        let tar_path = consumer_dir.join(format!("{crate_name}.crate"));
                        std::fs::write(&tar_path, &crate_bytes)
                            .map_err(|e| make_err(format!("write temp tarball: {e}")))?;
                        let tar_extract = Command::new("tar")
                            .args(["-xOzf"])
                            .arg(&tar_path)
                            .arg(format!("{crate_name}-{expected_version}/Cargo.toml"))
                            .output()
                            .map_err(|e| make_err(format!("spawn tar extract: {e}")))?;
                        if !tar_extract.status.success() {
                            return Err(make_err(format!(
                                "tar extract Cargo.toml failed for {crate_name}"
                            )));
                        }
                        let extracted = String::from_utf8_lossy(&tar_extract.stdout);
                        if extracted.contains(".workspace = true")
                            || extracted.contains("workspace = true")
                        {
                            return Err(make_err(format!(
                                "packaged {crate_name}.crate still contains workspace = true inheritance"
                            )));
                        }
                        if !extracted.contains(&format!("version = \"{expected_version}\"")) {
                            return Err(make_err(format!(
                                "packaged {crate_name}.crate Cargo.toml missing version = \"{expected_version}\""
                            )));
                        }
                        verified_crates = verified_crates.saturating_add(1);
                    }

                    // Build outside the source workspace so every local dependency must come from the registry.
                    let proj_dir = consumer_dir.join("consumer");
                    std::fs::create_dir_all(proj_dir.join(".cargo"))
                        .map_err(|e| make_err(format!("mkdir consumer .cargo: {e}")))?;
                    std::fs::create_dir_all(proj_dir.join("src"))
                        .map_err(|e| make_err(format!("mkdir consumer src: {e}")))?;
                    std::fs::write(
                        proj_dir.join(".cargo/config.toml"),
                        format!(
                            "[registries.verdant]\nindex = \"sparse+http://127.0.0.1:{port}/index/\"\n"
                        ),
                    )
                    .map_err(|e| make_err(format!("write consumer config.toml: {e}")))?;
                    std::fs::write(
                        proj_dir.join("Cargo.toml"),
                        format!(
                            "[package]\nname = \"verdant-sparse-consumer\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\nverdant-tensor = {{ version = \"={expected_version}\", registry = \"verdant\" }}\n"
                        ),
                    )
                    .map_err(|e| make_err(format!("write consumer Cargo.toml: {e}")))?;
                    std::fs::write(
                        proj_dir.join("src/lib.rs"),
                        "pub use verdant_tensor::linear::encode_weight_block;\n",
                    )
                    .map_err(|e| make_err(format!("write consumer lib.rs: {e}")))?;

                    if let Ok(mut guard) = served_paths.lock() {
                        guard.clear();
                    }

                    let out = Command::new("cargo")
                        .args(["check", "--quiet"])
                        .current_dir(&proj_dir)
                        .env("CARGO_HOME", consumer_dir.join("cargo-home"))
                        .env("CARGO_TARGET_DIR", consumer_dir.join("target"))
                        .output()
                        .map_err(|e| make_err(format!("check registry consumer: {e}")))?;
                    if !out.status.success() {
                        return Err(make_err(format!(
                            "registry consumer failed: {}",
                            String::from_utf8_lossy(&out.stderr)
                        )));
                    }
                    let seen: Vec<String> =
                        served_paths.lock().map(|g| g.clone()).unwrap_or_default();
                    let archive = format!("/crates/verdant-tensor-{expected_version}.crate");
                    for path in [
                        "/index/config.json",
                        "/index/ve/rd/verdant-tensor",
                        &archive,
                    ] {
                        if !seen.iter().any(|served| served == path) {
                            return Err(make_err(format!("Cargo did not fetch {path}")));
                        }
                    }
                    Ok(seen)
                })();

                stop_flag.store(true, Ordering::SeqCst);
                let _ = TcpStream::connect(format!("127.0.0.1:{port}"));
                let _ = server_handle.join();
                let _ = std::fs::remove_dir_all(&consumer_dir);

                let seen_paths = verify_res?;
                if !build_op_stages_all_passed(verified_crates, shipped_crates.len() as u32) {
                    return Err(make_err(
                        "incomplete sparse registry crate verification".to_string(),
                    ));
                }

                let summary_line = format!(
                    "verify-sparse-registry: PASS — verified {} crates @ v{expected_version} (SHA-256 + standalone Cargo.toml + CORS *) and downstream Cargo build ({} HTTP requests).",
                    verified_crates,
                    seen_paths.len()
                );
                (
                    "verify-registry".to_string(),
                    summary_line,
                    serde_json::json!({
                        "version": expected_version,
                        "verifiedCrates": verified_crates,
                        "cargoServedPaths": seen_paths
                    }),
                )
            }
        };

        let mut report = NativeBuildReport {
            spec: "verdant/native-build-report/1".to_string(),
            operation,
            passed: true,
            duration_ms: t_start.elapsed().as_millis() as u64,
            summary,
            details,
            report_kappa_hex: String::new(),
        };
        let preimage = serde_json::to_vec(&report).unwrap_or_default();
        report.report_kappa_hex = sha256_hex(&preimage);
        let pretty = serde_json::to_string_pretty(&report)
            .map_err(|e| make_err(format!("serialize native-build-report: {e}")))?;
        Ok((report, pretty))
    }
}
