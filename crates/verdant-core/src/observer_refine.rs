//! Generative Second-Order Self-Refinement Engine (`observer_refine.rs`).
//!
//! Enables Verdant's Observer Executive (`mgr-observer-executive`) and its 13 accountable
//! VSM S1–S5 Managers to inspect every organ, owned source file, `frontier.toml` proof blocker,
//! `fidelity.toml` obligation, Wasm/capsule byte budget, and Blank Frame weight, and generatively
//! synthesize a cryptographically committed [`ObserverRefinementAgenda`]
//! (`site/generated/observer-refinement-agenda.json`) that guides continuous self-refinement.

use std::path::Path;

use serde::{Deserialize, Serialize};
use verdant_tensor::scalars::{eight_u32, four_u32, hundred_u32, ten_u32, thousand_u32, two_u32};

use super::VerdantOperatorChannel;
use crate::digest::{sha256_hex, sha256_hex_tagged as digest_sha256_hex_tagged};
use crate::error::ObserverModelError;
use crate::observer::CommunicationChannelKind;
use crate::vcs::MAX_BLANK_FRAME_BYTES;

/// Urgency tier for an Observer-generated self-refinement proposal.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ObserverRefinementPriority {
    /// Safety, formal closure, or ratchet ceiling pressure requiring immediate action.
    Critical,
    /// High-leverage weight, modularity, or proof-frontier refinement.
    High,
    /// Steady architectural or UX parity improvement.
    Medium,
    /// Ongoing monotonic optimization when all thresholds already show strong headroom.
    Continuous,
}

/// Top unlifted MIR construct blocker extracted directly from `frontier.toml`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FrontierBlockerSummary {
    /// Blocking Rust construct description from `uorust`.
    pub construct: String,
    /// Classification (`form`, `call`, `type`, `arithmetic`, `statements`, `signature`, `width`).
    pub cause: String,
    /// Number of product functions stopped by this construct.
    pub functions: usize,
}

/// Per-Manager live modularity & ownership health snapshot across every part of the Observer.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObserverPartHealthSnapshot {
    /// Accountable VSM Manager ID (`mgr-*`).
    pub manager_id: String,
    /// Viable System Model tier (`VSM-S1`..`VSM-S5`, `VSM-S3*`).
    pub vsm_system: String,
    /// ISO/IEC/IEEE 42010:2022 viewpoint identifier.
    pub viewpoint_id: String,
    /// Count of existing workspace files owned by this Manager.
    pub owned_file_count: usize,
    /// Line count of the largest text source file owned by this Manager.
    pub max_file_lines: usize,
    /// Repo-relative path of the largest file owned by this Manager.
    pub heaviest_file: String,
    /// Constructive health score in permille (`0..=1000`).
    pub health_score_permille: u32,
}

/// Manager-assigned, authority-cited generative refinement proposal emitted by the Observer.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObserverRefinementProposal {
    /// Deterministic proposal identifier.
    pub proposal_id: String,
    /// Accountable Manager owning the targeted part (`mgr-*`).
    pub owning_manager_id: String,
    /// VSM system tier of the owning Manager.
    pub vsm_system: String,
    /// Targeted Observer organ or facet (`engine`, `studio`, `catalogs`, `demo`, `reference`, `observer`).
    pub target_organ: String,
    /// Cited authoritative standard (`AUTH-*`).
    pub cited_authority: String,
    /// Ratchet layer governed by this proposal.
    pub ratchet_layer: String,
    /// Deterministic priority ranking.
    pub priority: ObserverRefinementPriority,
    /// Live observed value.
    pub current_value: String,
    /// Recommended next monotonic watermark target.
    pub target_value: String,
    /// Concise rationale for this generative refinement step.
    pub summary: String,
    /// Actionable, Manager-scoped refinement steps.
    pub concrete_actions: Vec<String>,
    /// Content-addressed SHA-256 hex commitment over this proposal.
    pub kappa_hex: String,
}

/// Live cross-surface metrics observed during generative self-diagnosis.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObserverRefinementMetrics {
    /// Live `site/verdant-wasm.wasm` size in bytes.
    pub wasm_core_bytes: usize,
    /// Live `site/verdant-uorc.wasm` size in bytes.
    pub wasm_uorc_bytes: usize,
    /// Live pinned `.holo` release capsule size in bytes.
    pub release_capsule_bytes: usize,
    /// Live `site/frame/` Blank Frame footprint in bytes.
    pub blank_frame_bytes: usize,
    /// Maximum line count across all hand-written shipped `.rs` files.
    pub max_handwritten_rust_file_lines: usize,
    /// Maximum line count across all shipped `site/**/*.mjs|js` files.
    pub max_js_file_lines: usize,
    /// MIR-lifted function count from `frontier.toml`.
    pub frontier_lifted: usize,
    /// Total residue function count from `frontier.toml`.
    pub frontier_residue: usize,
    /// Total uncertified function count from `frontier.toml`.
    pub frontier_uncertified: usize,
    /// Open `[[residue]]` record count in `fidelity.toml` (`0` = 100% formal closure).
    pub fidelity_open_residue: usize,
}

/// Cryptographically sealed generative self-refinement agenda synthesized by the Observer
/// (`site/generated/observer-refinement-agenda.json`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObserverRefinementAgenda {
    /// Fixed format identifier (`verdant/observer-refinement-agenda/1`).
    pub spec: String,
    /// Active release version from `site/generated/release-ledger.json`.
    pub release_version: String,
    /// Active monotonic release epoch.
    pub release_epoch: u64,
    /// Observer holon root `kappa`.
    pub observer_kappa_hex: String,
    /// Aggregate generative self-refinement score in permille (`0..=1000`).
    pub overall_refinement_score_permille: u32,
    /// Live cross-surface metrics measured directly from the workspace.
    pub live_metrics: ObserverRefinementMetrics,
    /// Per-Manager modularity and ownership health across all 13 Managers.
    pub part_health: Vec<ObserverPartHealthSnapshot>,
    /// Top unlifted MIR construct blockers parsed from `frontier.toml`.
    pub top_frontier_blockers: Vec<FrontierBlockerSummary>,
    /// Prioritized, Manager-routed generative refinement proposals.
    pub proposals: Vec<ObserverRefinementProposal>,
    /// Content-addressed SHA-256 hex commitment over this agenda.
    pub agenda_kappa_hex: String,
}

/// Computes a Manager's constructive modularity health score in permille (`0..=1000`) from its
/// heaviest owned source file line count against `(loc_ideal, loc_ceiling)`.
#[must_use]
pub const fn part_health_score_permille(max_lines: u32, loc_ideal: u32, loc_ceiling: u32) -> u32 {
    let thousand = thousand_u32();
    let hundred = hundred_u32();
    let fifty = ten_u32().wrapping_mul(four_u32().wrapping_add(1_u32));
    let two_hundred = hundred.wrapping_add(hundred);
    if max_lines <= loc_ideal {
        thousand
    } else if max_lines >= loc_ceiling {
        thousand.saturating_sub(two_hundred.wrapping_add(fifty))
    } else {
        let gap = loc_ceiling.saturating_sub(loc_ideal);
        let over = max_lines.saturating_sub(loc_ideal);
        if over.wrapping_mul(two_u32()) <= gap {
            thousand.saturating_sub(hundred)
        } else {
            thousand.saturating_sub(two_hundred)
        }
    }
}

/// Computes the fidelity closure bonus in permille (`1000` at 0 open residue, decaying linearly).
#[must_use]
pub const fn fidelity_bonus_permille(open_residue: u32) -> u32 {
    thousand_u32().saturating_sub(open_residue)
}

/// Computes remaining Blank Frame byte headroom (`0` if over budget).
#[must_use]
pub const fn blank_frame_headroom_bytes(used_bytes: u32, max_bytes: u32) -> u32 {
    max_bytes.saturating_sub(used_bytes)
}

/// Computes aggregate Observer self-refinement score in permille across modularity, fidelity, and
/// Blank Frame compliance without division.
#[must_use]
pub const fn aggregate_refinement_score_permille(
    avg_part_health: u32,
    fidelity_open: u32,
    blank_ok: bool,
) -> u32 {
    let thousand = thousand_u32();
    let hundred = hundred_u32();
    let base = if avg_part_health > thousand {
        thousand
    } else {
        avg_part_health
    };
    let after_fidelity = base.saturating_sub(fidelity_open);
    if blank_ok {
        after_fidelity
    } else {
        after_fidelity.saturating_sub(hundred.wrapping_add(hundred))
    }
}

/// Classifies metric urgency against a target watermark.
#[must_use]
pub const fn refinement_priority_for_ceiling(
    current: u32,
    target: u32,
    high_when_over: bool,
) -> ObserverRefinementPriority {
    if current > target {
        if high_when_over {
            ObserverRefinementPriority::High
        } else {
            ObserverRefinementPriority::Medium
        }
    } else {
        ObserverRefinementPriority::Continuous
    }
}

impl VerdantOperatorChannel {
    /// Generatively observes every Manager, owned source file, `frontier.toml`, `fidelity.toml`,
    /// Wasm/capsule weight, and Blank Frame ceiling, producing a deterministic self-refinement
    /// agenda that guides the Observer to refine itself and all of its parts.
    pub fn synthesize_refinement_agenda(
        &self,
        repo_root: &Path,
    ) -> Result<(ObserverRefinementAgenda, String), ObserverModelError> {
        let ledger = self.load_or_init_release_ledger(repo_root)?;
        let pinned_capsule_bytes =
            std::fs::read_to_string(repo_root.join("site/frame/frame-pin.json"))
                .ok()
                .and_then(|raw| serde_json::from_str::<serde_json::Value>(&raw).ok())
                .and_then(|v| v.get("capsuleByteLen").and_then(|n| n.as_u64()))
                .unwrap_or(0) as usize;
        let live =
            crate::ratchet_sync::measure_live_ratchet_metrics(repo_root, pinned_capsule_bytes)?;
        let blank_frame_bytes = Self::audit_blank_frame_invariants(repo_root)?;

        let frontier_raw =
            std::fs::read_to_string(repo_root.join("frontier.toml")).map_err(|e| {
                ObserverModelError::CommunicationEnvelopeRejected {
                    channel: CommunicationChannelKind::OrganToOrgan.slug().to_string(),
                    detail: format!("read frontier.toml for refinement agenda: {e}"),
                }
            })?;
        let top_limit = four_u32().wrapping_add(1) as usize;
        let mut top_frontier_blockers = Vec::new();
        {
            let mut in_construct = false;
            let mut construct = String::new();
            let mut cause = String::new();
            let mut functions: Option<usize> = None;
            for raw_line in frontier_raw.lines() {
                let line = raw_line.trim();
                if line == "[[construct]]"
                    || line.starts_with("[[")
                    || (line.starts_with('[') && line.ends_with(']'))
                {
                    if !construct.is_empty() && !cause.is_empty() {
                        if let Some(cnt) = functions.take() {
                            top_frontier_blockers.push(FrontierBlockerSummary {
                                construct: core::mem::take(&mut construct),
                                cause: core::mem::take(&mut cause),
                                functions: cnt,
                            });
                        }
                    }
                    construct.clear();
                    cause.clear();
                    functions = None;
                    if top_frontier_blockers.len() >= top_limit {
                        break;
                    }
                    in_construct = line == "[[construct]]";
                    continue;
                }
                if !in_construct {
                    continue;
                }
                if let Some(rest) = line.strip_prefix("construct = ") {
                    construct = rest.trim().trim_matches('"').to_string();
                } else if let Some(rest) = line.strip_prefix("cause = ") {
                    cause = rest.trim().trim_matches('"').to_string();
                } else if let Some(rest) = line.strip_prefix("functions = ") {
                    functions = rest.trim().parse::<usize>().ok();
                }
            }
            if top_frontier_blockers.len() < top_limit && !construct.is_empty() && !cause.is_empty()
            {
                if let Some(cnt) = functions {
                    top_frontier_blockers.push(FrontierBlockerSummary {
                        construct,
                        cause,
                        functions: cnt,
                    });
                }
            }
        }

        let fidelity_raw =
            std::fs::read_to_string(repo_root.join("fidelity.toml")).unwrap_or_default();
        let fidelity_open_residue = fidelity_raw.matches("[[residue]]").count();

        let loc_ideal = thousand_u32().wrapping_add(hundred_u32().wrapping_mul(four_u32()));
        let loc_ceiling = thousand_u32()
            .wrapping_mul(two_u32())
            .wrapping_add(hundred_u32().wrapping_mul(two_u32()));

        let mut part_health = Vec::with_capacity(self.managers.len());
        let mut heaviest_rust_file = String::new();
        let mut heaviest_rust_lines = 0usize;
        let mut heaviest_js_file = String::new();
        let mut heaviest_js_lines = 0usize;
        let mut sum_part_health: u64 = 0;

        for mgr in &self.managers {
            let mut existing_count = 0usize;
            let mut max_lines = 0usize;
            let mut heaviest = String::new();
            for rel in &mgr.spec.owned_files {
                let p = repo_root.join(rel);
                if p.is_file() {
                    existing_count = existing_count.saturating_add(1);
                    let is_data_or_register = rel == "fidelity.toml"
                        || rel == "frontier.toml"
                        || rel.starts_with("site/generated/")
                        || rel.starts_with("model/")
                        || rel.starts_with("catalogs/");
                    if is_data_or_register {
                        continue;
                    }
                    let lines = std::fs::read_to_string(&p)
                        .map(|s| s.lines().count())
                        .unwrap_or(0);
                    if lines > max_lines {
                        max_lines = lines;
                        heaviest = rel.clone();
                    }
                    if rel.ends_with(".rs")
                        && !rel.ends_with("prism_kernel.rs")
                        && lines > heaviest_rust_lines
                    {
                        heaviest_rust_lines = lines;
                        heaviest_rust_file = rel.clone();
                    }
                    if rel.starts_with("site/")
                        && (rel.ends_with(".mjs") || rel.ends_with(".js"))
                        && lines > heaviest_js_lines
                    {
                        heaviest_js_lines = lines;
                        heaviest_js_file = rel.clone();
                    }
                }
            }
            for crate_name in &mgr.spec.owned_crates {
                let src_dir = repo_root.join("crates").join(crate_name).join("src");
                if let Ok(rd) = std::fs::read_dir(&src_dir) {
                    for ent in rd.flatten() {
                        let p = ent.path();
                        if !p.is_file() {
                            continue;
                        }
                        let rel = p
                            .strip_prefix(repo_root)
                            .unwrap_or(&p)
                            .to_string_lossy()
                            .replace('\\', "/");
                        if rel.ends_with(".rs") && !rel.ends_with("prism_kernel.rs") {
                            let lines = std::fs::read_to_string(&p)
                                .map(|s| s.lines().count())
                                .unwrap_or(0);
                            if lines > heaviest_rust_lines {
                                heaviest_rust_lines = lines;
                                heaviest_rust_file = rel;
                            }
                        }
                    }
                }
            }
            let score = part_health_score_permille(max_lines as u32, loc_ideal, loc_ceiling);
            sum_part_health = sum_part_health.saturating_add(u64::from(score));
            part_health.push(ObserverPartHealthSnapshot {
                manager_id: mgr.spec.manager_id.clone(),
                vsm_system: mgr.spec.vsm_system.clone(),
                viewpoint_id: mgr.spec.viewpoint_id.clone(),
                owned_file_count: existing_count,
                max_file_lines: max_lines,
                heaviest_file: heaviest,
                health_score_permille: score,
            });
        }

        let push_proposal = |proposals: &mut Vec<ObserverRefinementProposal>,
                             proposal_id: &str,
                             owning_manager_id: &str,
                             vsm_system: &str,
                             target_organ: &str,
                             cited_authority: &str,
                             ratchet_layer: &str,
                             priority: ObserverRefinementPriority,
                             current_value: String,
                             target_value: String,
                             summary: String,
                             concrete_actions: Vec<String>| {
            let sig_input = format!(
                "{proposal_id}|{owning_manager_id}|{ratchet_layer}|{current_value}|{target_value}|{summary}"
            );
            let kappa_hex = digest_sha256_hex_tagged(
                "verdant/operator/v1:",
                "observer-refine-proposal",
                sig_input.as_bytes(),
            );
            proposals.push(ObserverRefinementProposal {
                proposal_id: proposal_id.to_string(),
                owning_manager_id: owning_manager_id.to_string(),
                vsm_system: vsm_system.to_string(),
                target_organ: target_organ.to_string(),
                cited_authority: cited_authority.to_string(),
                ratchet_layer: ratchet_layer.to_string(),
                priority,
                current_value,
                target_value,
                summary,
                concrete_actions,
                kappa_hex,
            });
        };

        let mut proposals = Vec::new();
        if fidelity_open_residue > 0 {
            push_proposal(
                &mut proposals,
                "refine-fidelity-closure",
                "mgr-knowledge-provenance",
                "VSM-S4",
                "reference",
                "AUTH-W3C-SOSA-PROVO",
                "fidelity",
                ObserverRefinementPriority::Critical,
                format!("{fidelity_open_residue} open fidelity residue"),
                "0 open fidelity residue".to_string(),
                "Close every remaining open derived/meant/measured record in fidelity.toml."
                    .to_string(),
                vec![
                    "Ground un-cited model records in model/charter.md primary source spans."
                        .to_string(),
                    "Run npm run ratchet to re-cite Provenance.lex.tex and lock Fidelity.lex.tex."
                        .to_string(),
                ],
            );
        }

        let lift_step = four_u32().wrapping_mul(four_u32()).wrapping_add(four_u32()) as usize;
        let next_lift_target = live.frontier_lifted.saturating_add(lift_step);
        let top_blocker_desc = top_frontier_blockers
            .first()
            .map(|b| format!("{} ({} fns, cause={})", b.construct, b.functions, b.cause))
            .unwrap_or_else(|| "remaining arithmetic/width/statement residue".to_string());
        push_proposal(
            &mut proposals,
            "refine-uorust-frontier",
            "mgr-verification-observatory",
            "VSM-S3*",
            "engine",
            "AUTH-SEMVER-CHANGE-KPI",
            "uorust-frontier",
            ObserverRefinementPriority::High,
            format!(
                "{} lifted / {} residue ({} uncertified)",
                live.frontier_lifted, live.frontier_residue, live.frontier_uncertified
            ),
            format!(">= {next_lift_target} lifted functions"),
            format!("Expand constructive MIR proof frontier; top blocker: {top_blocker_desc}."),
            vec![
                "Replace remaining raw shift/overflow ops with lifted verdant_tensor::scalars and wrapping ops."
                    .to_string(),
                "Extract pure decision/calculation helpers out of IO/effectful wrappers so uorust lifts them."
                    .to_string(),
            ],
        );

        let nine_hundred = hundred_u32().wrapping_mul(eight_u32().wrapping_add(1));
        let js_target = (live.max_js_file_lines as u32)
            .saturating_sub(hundred_u32())
            .max(nine_hundred);
        let js_focus = if heaviest_js_file.is_empty() {
            "site/catalog-streams.mjs".to_string()
        } else {
            heaviest_js_file
        };
        push_proposal(
            &mut proposals,
            "refine-studio-js-modularity",
            "mgr-studio-alm",
            "VSM-S1",
            "studio",
            "AUTH-ISO-42010-VIEWPOINTS",
            "surface-loc-js",
            refinement_priority_for_ceiling(live.max_js_file_lines as u32, js_target, false),
            format!("{} lines max ({js_focus})", live.max_js_file_lines),
            format!("<= {js_target} lines max shipped JS file"),
            "Decompose heaviest shipped JS modules under site/ to auto-tighten max_js_file_lines."
                .to_string(),
            vec![
                format!("Split {js_focus} into focused cohesive ES modules."),
                "Register every extracted module in catalogs/governance/iso42010-viewpoints.json."
                    .to_string(),
            ],
        );

        let rust_target = (live.max_handwritten_rust_file_lines as u32)
            .saturating_sub(hundred_u32())
            .max(thousand_u32());
        let rust_focus = if heaviest_rust_file.is_empty() {
            "crates/verdant-core/src/streaming.rs".to_string()
        } else {
            heaviest_rust_file
        };
        push_proposal(
            &mut proposals,
            "refine-rust-modularity",
            "mgr-runtime-stage",
            "VSM-S1",
            "engine",
            "AUTH-ISO-42010-VIEWPOINTS",
            "surface-loc-rust",
            refinement_priority_for_ceiling(
                live.max_handwritten_rust_file_lines as u32,
                rust_target,
                false,
            ),
            format!(
                "{} lines max ({rust_focus})",
                live.max_handwritten_rust_file_lines
            ),
            format!("<= {rust_target} lines max hand-written Rust file"),
            "Keep every hand-written Rust module modular and auto-tighten max_handwritten_rust_file_lines."
                .to_string(),
            vec![
                format!("Extract cohesive submodules out of {rust_focus}."),
                "Register every extracted module in catalogs/governance/iso42010-viewpoints.json."
                    .to_string(),
            ],
        );

        let kibi = verdant_tensor::scalars::thirty_two_u32()
            .wrapping_mul(verdant_tensor::scalars::thirty_two_u32());
        let uorc_target = kibi.wrapping_mul(kibi).wrapping_mul(two_u32());
        let headroom =
            blank_frame_headroom_bytes(blank_frame_bytes as u32, MAX_BLANK_FRAME_BYTES as u32);
        push_proposal(
            &mut proposals,
            "refine-wasm-capsule-weight",
            "mgr-change-release",
            "VSM-S3",
            "engine",
            "AUTH-STITCH-SELF-HOSTING",
            "surface-wasm",
            refinement_priority_for_ceiling(live.wasm_uorc_bytes as u32, uorc_target, false),
            format!(
                "verdant-uorc.wasm {} B · .holo {} B · Blank Frame {} B ({headroom} B headroom)",
                live.wasm_uorc_bytes, live.release_capsule_bytes, blank_frame_bytes
            ),
            format!("verdant-uorc.wasm <= {uorc_target} B and gzipped .holo <= 1.0 MiB"),
            "Keep browser verdant-uorc.wasm lean (< 2.0 MiB) and preserve gzipped STCH2 shard efficiency for cold Blank Frame boots."
                .to_string(),
            vec![
                "Split heavy uorc_decompress and gltf-json paths into an on-demand lazy Wasm module so verdant-uorc.wasm drops below 2.0 MiB."
                    .to_string(),
                "Maintain Blank Frame strictly under 30720 B with DecompressionStream('gzip')."
                    .to_string(),
            ],
        );

        proposals.sort_by(|a, b| {
            a.priority
                .cmp(&b.priority)
                .then(a.proposal_id.cmp(&b.proposal_id))
        });

        let live_metrics = ObserverRefinementMetrics {
            wasm_core_bytes: live.wasm_core_bytes,
            wasm_uorc_bytes: live.wasm_uorc_bytes,
            release_capsule_bytes: live.release_capsule_bytes,
            blank_frame_bytes,
            max_handwritten_rust_file_lines: live.max_handwritten_rust_file_lines,
            max_js_file_lines: live.max_js_file_lines,
            frontier_lifted: live.frontier_lifted,
            frontier_residue: live.frontier_residue,
            frontier_uncertified: live.frontier_uncertified,
            fidelity_open_residue,
        };

        let avg_part_health = if part_health.is_empty() {
            thousand_u32()
        } else {
            (sum_part_health / (part_health.len() as u64)) as u32
        };
        let overall_refinement_score_permille = aggregate_refinement_score_permille(
            avg_part_health,
            fidelity_open_residue as u32,
            blank_frame_bytes <= MAX_BLANK_FRAME_BYTES,
        );

        let mut agenda = ObserverRefinementAgenda {
            spec: "verdant/observer-refinement-agenda/1".to_string(),
            release_version: ledger.current_version.display_string(),
            release_epoch: ledger.release_epoch,
            observer_kappa_hex: self.observer.observer_kappa_hex.clone(),
            overall_refinement_score_permille,
            live_metrics,
            part_health,
            top_frontier_blockers,
            proposals,
            agenda_kappa_hex: String::new(),
        };
        let preimage = serde_json::to_vec(&agenda).unwrap_or_default();
        agenda.agenda_kappa_hex = sha256_hex(&preimage);
        let pretty = serde_json::to_string_pretty(&agenda).map_err(|e| {
            ObserverModelError::CommunicationEnvelopeRejected {
                channel: CommunicationChannelKind::OrganToOrgan.slug().to_string(),
                detail: format!("serialize refinement agenda: {e}"),
            }
        })?;
        Ok((agenda, pretty))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn observer_synthesizes_deterministic_self_refinement_agenda_across_all_managers() {
        let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let ch = VerdantOperatorChannel::new("sunspire-regatta").expect("operator channel");
        let (a1, json1) = ch
            .synthesize_refinement_agenda(&repo_root)
            .expect("synthesize agenda 1");
        let (a2, json2) = ch
            .synthesize_refinement_agenda(&repo_root)
            .expect("synthesize agenda 2");
        assert_eq!(a1.agenda_kappa_hex, a2.agenda_kappa_hex);
        assert_eq!(json1, json2);
        assert_eq!(a1.spec, "verdant/observer-refinement-agenda/1");
        assert_eq!(a1.part_health.len(), ch.managers.len());
        assert!(!a1.proposals.is_empty());
        assert!(!a1.top_frontier_blockers.is_empty());
        assert_eq!(a1.live_metrics.fidelity_open_residue, 0);
    }
}
