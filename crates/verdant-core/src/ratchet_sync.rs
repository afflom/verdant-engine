//! Autonomous one-way ratchet measurement, live Rust source-tree binding for `frontier.toml`,
//! and monotonic auto-tightening for `model/parametric-ratchet.toml` (`VE-55`).
//!
//! Ensures the ratchet does its job automatically:
//! 1. Computes a deterministic SHA-256 over all shipped Rust source files (`frontier_rust_tree_sha256`)
//!    so `frontier.toml` can never go stale silently.
//! 2. Refreshes `frontier.toml` via `uorust` automatically when `uorust` is available and the Rust
//!    source tree changed.
//! 3. Measures live surface high-water marks (`verdant-wasm.wasm`, `verdant-uorc.wasm`, `.holo`
//!    release capsule, max hand-written Rust file lines, max JS file lines).
//! 4. Monotonically clicks floors UP and ceilings DOWN in `model/parametric-ratchet.toml` whenever
//!    live measurements beat current watermarks — and fails closed if any metric regresses!

#[cfg(not(target_arch = "wasm32"))]
use std::collections::BTreeMap;
#[cfg(not(target_arch = "wasm32"))]
use std::fmt::Write as _;
#[cfg(not(target_arch = "wasm32"))]
use std::path::{Path, PathBuf};
#[cfg(not(target_arch = "wasm32"))]
use std::process::Command;

#[cfg(not(target_arch = "wasm32"))]
use sha2::{Digest, Sha256};

#[cfg(not(target_arch = "wasm32"))]
use crate::error::ObserverModelError;
#[cfg(not(target_arch = "wasm32"))]
use crate::observer::CommunicationChannelKind;

macro_rules! ratchet_err {
    ($detail:expr) => {
        ObserverModelError::CommunicationEnvelopeRejected {
            channel: CommunicationChannelKind::OrganToOrgan.slug().to_string(),
            detail: $detail,
        }
    };
}

#[cfg(not(target_arch = "wasm32"))]
fn hex_encode_digest(bytes: &[u8]) -> String {
    let mut out = String::new();
    for b in bytes {
        let _ = write!(out, "{b:02x}");
    }
    out
}

/// Live measured ratchet watermarks across Rust sources, `frontier.toml`, Wasm binaries,
/// release capsules, and JS surfaces (`VE-55`).
#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LiveRatchetMetrics {
    /// Deterministic SHA-256 over all sorted shipped `.rs` source files.
    pub rust_tree_sha256: String,
    /// MIR-lifted function count parsed from `frontier.toml`.
    pub frontier_lifted: usize,
    /// Hand-written residue function count parsed from `frontier.toml`.
    pub frontier_residue: usize,
    /// Total uncertified (`residue + foreign + effect`) functions parsed from `frontier.toml`.
    pub frontier_uncertified: usize,
    /// Live byte size of `site/verdant-wasm.wasm`.
    pub wasm_core_bytes: usize,
    /// Live byte size of `site/verdant-uorc.wasm`.
    pub wasm_uorc_bytes: usize,
    /// Live byte size of the freshly packed `verdant-v<ver>.holo` capsule.
    pub release_capsule_bytes: usize,
    /// Maximum line count of any hand-written shipped `.rs` file (excluding generated `prism_kernel.rs`).
    pub max_handwritten_rust_file_lines: usize,
    /// Maximum line count of any shipped `.mjs`/`.js` file under `site/` (excluding `vendor/` and `crates/`).
    pub max_js_file_lines: usize,
}

#[cfg(not(target_arch = "wasm32"))]
fn walk_suffix(
    repo_root: &Path,
    dir: &Path,
    suffixes: &[&str],
    out: &mut BTreeMap<String, Vec<u8>>,
) {
    let Ok(rd) = std::fs::read_dir(dir) else {
        return;
    };
    let mut entries: Vec<_> = rd.filter_map(Result::ok).collect();
    entries.sort_by_key(|e| e.path());
    for entry in entries {
        let p = entry.path();
        if let Some(name) = p.file_name().and_then(|n| n.to_str()) {
            if name.starts_with('.') || name == "target" || name == "node_modules" {
                continue;
            }
        }
        if p.is_dir() {
            walk_suffix(repo_root, &p, suffixes, out);
        } else if p.is_file() {
            let Ok(rel) = p.strip_prefix(repo_root) else {
                continue;
            };
            let rel_str = rel.to_string_lossy().replace('\\', "/");
            if suffixes.iter().any(|s| rel_str.ends_with(s)) {
                if let Ok(bytes) = std::fs::read(&p) {
                    out.insert(rel_str, bytes);
                }
            }
        }
    }
}

/// Computes the canonical deterministic SHA-256 hex digest over every `.rs` file under `rust_roots`.
#[cfg(not(target_arch = "wasm32"))]
pub fn compute_rust_source_tree_sha256(
    repo_root: &Path,
    rust_roots: &[&str],
) -> Result<String, ObserverModelError> {
    let mut files = BTreeMap::new();
    for root in rust_roots {
        walk_suffix(repo_root, &repo_root.join(root), &[".rs"], &mut files);
    }
    if files.is_empty() {
        return Err(ratchet_err!(
            "no shipped Rust source files found for frontier tree binding".to_string()
        ));
    }
    let mut hasher = Sha256::new();
    for (rel, bytes) in &files {
        hasher.update(rel.as_bytes());
        hasher.update([0_u8]);
        hasher.update(bytes);
        hasher.update([0_u8]);
    }
    Ok(hex_encode_digest(&hasher.finalize()))
}

#[cfg(not(target_arch = "wasm32"))]
fn parse_toml_u64(raw: &str, key: &str) -> Option<u64> {
    let prefix = format!("{key} = ");
    for line in raw.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix(&prefix) {
            if let Ok(v) = rest.trim().parse::<u64>() {
                return Some(v);
            }
        }
    }
    None
}

#[cfg(not(target_arch = "wasm32"))]
fn parse_toml_string(raw: &str, key: &str) -> Option<String> {
    let prefix = format!("{key} = \"");
    for line in raw.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix(&prefix) {
            if let Some(val) = rest.strip_suffix('"') {
                return Some(val.to_string());
            }
        }
    }
    None
}

#[cfg(not(target_arch = "wasm32"))]
fn find_uorust_binary() -> Option<PathBuf> {
    if let Ok(env_bin) = std::env::var("VERDANT_UORUST_BIN") {
        let p = PathBuf::from(env_bin);
        if p.is_file() {
            return Some(p);
        }
    }
    let tmp_bin = PathBuf::from("/tmp/uorust-bin/uorust-x86_64-unknown-linux-gnu/uorust");
    if tmp_bin.is_file() {
        return Some(tmp_bin);
    }
    let probe = Command::new("uorust").arg("--version").output();
    if probe.is_ok_and(|o| o.status.success()) {
        return Some(PathBuf::from("uorust"));
    }
    None
}

/// Automatically invokes `uorust` when installed and the live Rust source tree differs from the
/// recorded `frontier_rust_tree_sha256` (or `frontier.toml` is missing).
#[cfg(not(target_arch = "wasm32"))]
pub fn maybe_refresh_uorust_frontier(
    repo_root: &Path,
    live_rust_tree_sha256: &str,
) -> Result<bool, ObserverModelError> {
    let cfg_raw = std::fs::read_to_string(repo_root.join("model/parametric-ratchet.toml"))
        .unwrap_or_default();
    let recorded_sha = parse_toml_string(&cfg_raw, "frontier_rust_tree_sha256").unwrap_or_default();
    let frontier_exists = repo_root.join("frontier.toml").is_file();
    if frontier_exists && recorded_sha == live_rust_tree_sha256 {
        return Ok(false);
    }
    let Some(uorust_bin) = find_uorust_binary() else {
        return Ok(false);
    };
    let tmp_out = std::env::temp_dir().join(format!("verdant-uorust-sync-{}", std::process::id()));
    let status = Command::new(&uorust_bin)
        .args([
            "project",
            "--out",
            &tmp_out.to_string_lossy(),
            "--register",
            "frontier.toml",
            ".",
        ])
        .current_dir(repo_root)
        .output()
        .map_err(|e| ratchet_err!(format!("failed launching uorust: {e}")))?;
    let _ = std::fs::remove_dir_all(&tmp_out);
    if !status.status.success() {
        return Err(ratchet_err!(format!(
            "uorust refresh failed: {}",
            String::from_utf8_lossy(&status.stderr)
        )));
    }
    Ok(true)
}

/// Measures all live ratchet metrics from the repository workspace and freshly packed capsule.
#[cfg(not(target_arch = "wasm32"))]
pub fn measure_live_ratchet_metrics(
    repo_root: &Path,
    release_capsule_bytes: usize,
) -> Result<LiveRatchetMetrics, ObserverModelError> {
    let rust_roots = [
        "crates/verdant-core/src",
        "crates/verdant-tensor/src",
        "crates/verdant-wasm/src",
        "crates/verdant-cli/src",
    ];
    let rust_tree_sha256 = compute_rust_source_tree_sha256(repo_root, &rust_roots)?;
    let _ = maybe_refresh_uorust_frontier(repo_root, &rust_tree_sha256)?;

    let frontier_raw = std::fs::read_to_string(repo_root.join("frontier.toml"))
        .map_err(|e| ratchet_err!(format!("read frontier.toml: {e}")))?;
    let frontier_lifted = parse_toml_u64(&frontier_raw, "lifted")
        .ok_or_else(|| ratchet_err!("frontier.toml missing lifted count".to_string()))?
        as usize;
    let frontier_residue = parse_toml_u64(&frontier_raw, "residue")
        .ok_or_else(|| ratchet_err!("frontier.toml missing residue count".to_string()))?
        as usize;
    let frontier_foreign = parse_toml_u64(&frontier_raw, "foreign").unwrap_or(0) as usize;
    let frontier_effect = parse_toml_u64(&frontier_raw, "effect").unwrap_or(0) as usize;
    let frontier_uncertified = frontier_residue
        .saturating_add(frontier_foreign)
        .saturating_add(frontier_effect);

    let wasm_core_bytes = std::fs::metadata(repo_root.join("site/verdant-wasm.wasm"))
        .map(|m| m.len() as usize)
        .unwrap_or(0);
    let wasm_uorc_bytes = std::fs::metadata(repo_root.join("site/verdant-uorc.wasm"))
        .map(|m| m.len() as usize)
        .unwrap_or(0);

    let mut rust_files = BTreeMap::new();
    for root in &rust_roots {
        walk_suffix(repo_root, &repo_root.join(root), &[".rs"], &mut rust_files);
    }
    let mut max_handwritten_rust_file_lines = 0usize;
    for (rel, bytes) in &rust_files {
        if rel == "crates/verdant-tensor/src/prism_kernel.rs" {
            continue;
        }
        if let Ok(text) = std::str::from_utf8(bytes) {
            let lines = text.lines().count();
            if lines > max_handwritten_rust_file_lines {
                max_handwritten_rust_file_lines = lines;
            }
        }
    }

    let mut js_files = BTreeMap::new();
    walk_suffix(
        repo_root,
        &repo_root.join("site"),
        &[".mjs", ".js"],
        &mut js_files,
    );
    let mut max_js_file_lines = 0usize;
    for (rel, bytes) in &js_files {
        if rel.starts_with("site/vendor/") || rel.starts_with("site/crates/") {
            continue;
        }
        if let Ok(text) = std::str::from_utf8(bytes) {
            let lines = text.lines().count();
            if lines > max_js_file_lines {
                max_js_file_lines = lines;
            }
        }
    }

    Ok(LiveRatchetMetrics {
        rust_tree_sha256,
        frontier_lifted,
        frontier_residue,
        frontier_uncertified,
        wasm_core_bytes,
        wasm_uorc_bytes,
        release_capsule_bytes,
        max_handwritten_rust_file_lines,
        max_js_file_lines,
    })
}

/// One-way monotonic auto-tightener for `model/parametric-ratchet.toml` (`VE-55`).
///
/// - **Floors** (`min_lifted_functions`) may ONLY stay flat or click UP; regressions fail closed.
/// - **Ceilings** (`max_residue_functions`, `max_uncertified_functions`, `max_wasm_core_bytes`,
///   `max_wasm_uorc_bytes`, `max_release_capsule_bytes`, `max_handwritten_rust_file_lines`,
///   `max_js_file_lines`) may ONLY stay flat or click DOWN; regressions fail closed.
/// - Updates `frontier_rust_tree_sha256` to bind `frontier.toml` to the exact live Rust source tree.
#[cfg(not(target_arch = "wasm32"))]
pub fn tighten_parametric_ratchet_config(
    repo_root: &Path,
    metrics: &LiveRatchetMetrics,
    write_back: bool,
) -> Result<Vec<String>, ObserverModelError> {
    let cfg_path = repo_root.join("model/parametric-ratchet.toml");
    let raw = std::fs::read_to_string(&cfg_path)
        .map_err(|e| ratchet_err!(format!("read model/parametric-ratchet.toml: {e}")))?;

    let mut tightened = Vec::new();
    let mut violations = Vec::new();
    let mut out_lines = Vec::new();

    let four_kib = (verdant_tensor::scalars::sixty_four_u64() as usize)
        .wrapping_mul(verdant_tensor::scalars::sixty_four_u64() as usize);
    let align_4k = |b: usize| -> usize {
        if b == 0 {
            0
        } else {
            b.div_ceil(four_kib).wrapping_mul(four_kib)
        }
    };

    let floors = [("min_lifted_functions", metrics.frontier_lifted)];
    let ceilings = [
        ("max_residue_functions", metrics.frontier_residue, false),
        (
            "max_uncertified_functions",
            metrics.frontier_uncertified,
            false,
        ),
        ("max_wasm_core_bytes", metrics.wasm_core_bytes, true),
        ("max_wasm_uorc_bytes", metrics.wasm_uorc_bytes, true),
        (
            "max_release_capsule_bytes",
            metrics.release_capsule_bytes,
            true,
        ),
        (
            "max_handwritten_rust_file_lines",
            metrics.max_handwritten_rust_file_lines,
            false,
        ),
        ("max_js_file_lines", metrics.max_js_file_lines, false),
    ];

    for line in raw.lines() {
        let trimmed = line.trim();
        let mut replaced = line.to_string();

        for (key, live) in floors {
            let pref = format!("{key} = ");
            if let Some(rest) = trimmed.strip_prefix(&pref) {
                if let Ok(old) = rest.trim().parse::<usize>() {
                    if live < old {
                        violations.push(format!("{key} regressed: live {live} < floor {old}"));
                    } else if live > old {
                        replaced = format!("{key} = {live}");
                        tightened.push(format!("{key}: {old} -> {live}"));
                    }
                }
            }
        }

        for (key, live, page_align) in ceilings {
            let pref = format!("{key} = ");
            if let Some(rest) = trimmed.strip_prefix(&pref) {
                if let Ok(old) = rest.trim().parse::<usize>() {
                    let target = if page_align { align_4k(live) } else { live };
                    if live > old {
                        violations.push(format!("{key} regressed: live {live} > ceiling {old}"));
                    } else if target > 0 && target < old {
                        replaced = format!("{key} = {target}");
                        tightened.push(format!("{key}: {old} -> {target}"));
                    }
                }
            }
        }

        if let Some(rest) = trimmed.strip_prefix("frontier_rust_tree_sha256 = \"") {
            if let Some(old_sha) = rest.strip_suffix('"') {
                if old_sha != metrics.rust_tree_sha256 {
                    replaced = format!(
                        "frontier_rust_tree_sha256 = \"{}\"",
                        metrics.rust_tree_sha256
                    );
                    tightened.push(format!(
                        "frontier_rust_tree_sha256 -> {}",
                        &metrics.rust_tree_sha256
                            [..verdant_tensor::scalars::sixteen_u64() as usize]
                    ));
                }
            }
        }
        out_lines.push(replaced);
    }

    if !violations.is_empty() {
        return Err(ratchet_err!(format!(
            "one-way ratchet refused regression: {}",
            violations.join("; ")
        )));
    }

    if write_back && !tightened.is_empty() {
        let mut next = out_lines.join("\n");
        next.push('\n');
        std::fs::write(&cfg_path, next.as_bytes()).map_err(|e| {
            ratchet_err!(format!(
                "write tightened model/parametric-ratchet.toml: {e}"
            ))
        })?;
    }

    Ok(tightened)
}
