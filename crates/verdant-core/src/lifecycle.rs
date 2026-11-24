//! **In-Verdant Self-Verification, Test Execution & Release-Gating Lifecycle (`lifecycle.rs`)**.
//!
//! Lifts Verdant's entire validation, test execution, and release-qualification lifecycle directly
//! INTO Verdant (`mgr-verification-observatory` + `mgr-change-release`) so Verdant invokes its
//! own tests from within, audits every lifted lifecycle category (`rust`, `verification-test`,
//! `lexlean`, `feature-suite`, `model-governance`, `viewpoint`), and gates its own releases
//! cryptographically without relying on external scripts.

use std::path::Path;
use std::process::Command;
use std::time::Instant;

use serde::{Deserialize, Serialize};
use verdant_tensor::scalars::{hundred_u32, thousand_u32, two_u32};

use super::VerdantOperatorChannel;
use crate::digest::sha256_hex;
use crate::error::ObserverModelError;
use crate::observer::CommunicationChannelKind;
use crate::vcs::MAX_BLANK_FRAME_BYTES;

/// Configuration options for running Verdant's in-process + self-invoked lifecycle gate.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LifecycleGateOptions {
    /// When true, Verdant spawns its own workspace validation & test commands (`cargo fmt`,
    /// `xtask validate`, `prism-fidelity --check`, `cargo clippy`, `repo-conformance --test bdd`,
    /// and unit/fidelity test suites). Set false only for fast in-process audits (e.g. inside tests).
    pub run_workspace_tests: bool,
    /// When true, Verdant also executes its headless Chromium browser E2E gates
    /// (`verify-stitch-release.mjs` and `verify-uhd-workload.mjs`).
    pub run_browser_gates: bool,
    /// Optional SemVer release tag (`vX.Y.Z`) to qualify against `Cargo.toml`, `release-ledger.json`,
    /// and `self-host-ratchet-manifest.toml`.
    pub expected_tag: Option<String>,
    /// When true, writes `site/generated/lifecycle-gate-report.json` owned by `mgr-verification-observatory`.
    pub write_report: bool,
}

/// Outcome of a single check executed inside Verdant's lifted lifecycle gate.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LifecycleCheckRecord {
    /// Deterministic check identifier (e.g., `total-ownership`, `conformance-bdd`, `release-version-sync`).
    pub check_id: String,
    /// Accountable VSM Manager owning this check (`mgr-*`).
    pub owning_manager_id: String,
    /// Execution category (`in-process`, `workspace-test`, `browser-e2e`).
    pub category: String,
    /// True iff this check passed cleanly.
    pub passed: bool,
    /// Elapsed wall-clock milliseconds.
    pub duration_ms: u64,
    /// Concise human- and machine-readable verdict detail.
    pub detail: String,
}

/// Counts of lifted lifecycle artifacts verified inside `self-host-ratchet-manifest.toml`.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiftedLifecycleCoverage {
    /// Lifted shipped Rust source modules (`crates/verdant-{tensor,core,wasm,cli}/src/**/*.rs`).
    pub rust_modules: u32,
    /// Lifted verification, fidelity, model, xtask & conformance test modules.
    pub verification_test_modules: u32,
    /// Lifted constructive Prism LexLean strata modules (`app/src/**/*.lex.tex`).
    pub lexlean_modules: u32,
    /// Lifted BDD Gherkin feature suites (`features/suites/**/*.feature`).
    pub feature_suite_modules: u32,
    /// Lifted formal model & governance authority files (`model/*`, `catalogs/**/*.json`).
    pub model_governance_modules: u32,
    /// Lifted lifecycle automation, build & verification scripts (`scripts/**/*.mjs`).
    pub automation_script_modules: u32,
    /// Lifted UI Viewpoint & immutable Blank Frame assets.
    pub viewpoint_modules: u32,
    /// Total lifted lifecycle modules tracked in `self-host-ratchet-manifest.toml`.
    pub total_lifted_modules: u32,
    /// True iff every single lifecycle category is non-empty and lifted into Verdant.
    pub all_categories_lifted: bool,
}

/// Cryptographically committed self-verification & release-gating report produced by Verdant
/// (`site/generated/lifecycle-gate-report.json`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LifecycleGateReport {
    /// Fixed format identifier (`verdant/lifecycle-gate-report/1`).
    pub spec: String,
    /// Active release version synchronized across `Cargo.toml`, ledger, and manifest.
    pub release_version: String,
    /// Active monotonic release epoch.
    pub release_epoch: u64,
    /// Execution mode (`in-process`, `full`, or `full-with-browser`).
    pub mode: String,
    /// True iff every executed check passed.
    pub all_passed: bool,
    /// True iff Verdant's internal release gate certifies this revision for release/tag cutting.
    pub release_qualified: bool,
    /// Number of checks passed.
    pub passed_count: u32,
    /// Total number of checks executed.
    pub total_count: u32,
    /// Constructive pass rate in permille (`0..=1000`).
    pub pass_rate_permille: u32,
    /// Coverage counts across all 6 lifted lifecycle artifact categories.
    pub lifted_coverage: LiftedLifecycleCoverage,
    /// Ordered per-check execution records.
    pub checks: Vec<LifecycleCheckRecord>,
    /// Content-addressed SHA-256 hex digest over this report.
    pub report_kappa_hex: String,
}

/// Pure MIR-lifted verdict: true iff at least one check ran and every check passed.
#[must_use]
pub const fn lifecycle_all_checks_passed(passed_count: u32, total_count: u32) -> bool {
    total_count > 0 && passed_count == total_count
}

/// Pure MIR-lifted verdict: true iff all internal Observer invariants and executed checks pass.
#[must_use]
pub const fn lifecycle_release_qualified(
    all_passed: bool,
    version_synced: bool,
    seal_intact: bool,
    ownership_total: bool,
    blank_frame_ok: bool,
) -> bool {
    all_passed && version_synced && seal_intact && ownership_total && blank_frame_ok
}

/// Pure MIR-lifted constructive pass-rate indicator in permille (`0..=1000`) without division.
#[must_use]
pub const fn lifecycle_pass_rate_permille(passed_count: u32, total_count: u32) -> u32 {
    let thousand = thousand_u32();
    let hundred = hundred_u32();
    let two_hundred = hundred.wrapping_add(hundred);
    if total_count == 0 || passed_count == 0 {
        0
    } else if passed_count >= total_count {
        thousand
    } else if passed_count.wrapping_mul(two_u32()) >= total_count {
        thousand.saturating_sub(two_hundred)
    } else {
        two_hundred
    }
}

/// Pure MIR-lifted SemVer triple equality predicate.
#[must_use]
pub const fn lifecycle_version_tuple_matches(
    a_maj: u32,
    a_min: u32,
    a_pat: u32,
    b_maj: u32,
    b_min: u32,
    b_pat: u32,
) -> bool {
    a_maj == b_maj && a_min == b_min && a_pat == b_pat
}

/// Pure MIR-lifted predicate confirming all 6 core Verdant lifecycle artifact categories are lifted.
#[must_use]
pub const fn lifecycle_lifted_categories_complete(
    rust_n: u32,
    test_n: u32,
    lex_n: u32,
    feature_n: u32,
    model_n: u32,
    view_n: u32,
) -> bool {
    rust_n > 0 && test_n > 0 && lex_n > 0 && feature_n > 0 && model_n > 0 && view_n > 0
}

/// Pure MIR-lifted predicate confirming all 7 Verdant lifecycle artifact categories (including
/// automation scripts under `scripts/**/*.mjs`) are lifted into Verdant.
#[must_use]
pub const fn lifecycle_lifted_categories_complete_v7(
    rust_n: u32,
    test_n: u32,
    lex_n: u32,
    feature_n: u32,
    model_n: u32,
    script_n: u32,
    view_n: u32,
) -> bool {
    lifecycle_lifted_categories_complete(rust_n, test_n, lex_n, feature_n, model_n, view_n)
        && script_n > 0
}

/// Pure MIR-lifted total count across all 7 lifted lifecycle categories.
#[must_use]
pub const fn lifecycle_manifest_total_modules_v7(
    rust_n: u32,
    test_n: u32,
    lex_n: u32,
    feature_n: u32,
    model_n: u32,
    script_n: u32,
    view_n: u32,
) -> u32 {
    rust_n
        .saturating_add(test_n)
        .saturating_add(lex_n)
        .saturating_add(feature_n)
        .saturating_add(model_n)
        .saturating_add(script_n)
        .saturating_add(view_n)
}

/// Pure MIR-lifted gate refusing any release advancement without qualification, ratchet, and 0 unlifted.
#[must_use]
pub const fn lifecycle_gate_can_advance_release(
    qualified: bool,
    ratchet_applied: bool,
    unlifted_count: u32,
) -> bool {
    qualified && ratchet_applied && unlifted_count == 0
}

impl VerdantOperatorChannel {
    /// Executes Verdant's lifted internal verification, self-invoked test suites, and release
    /// qualification gate directly from within Verdant (`mgr-verification-observatory` &
    /// `mgr-change-release`), returning `(LifecycleGateReport, pretty_json)`.
    pub fn execute_lifecycle_gate(
        &self,
        repo_root: &Path,
        opts: &LifecycleGateOptions,
    ) -> Result<(LifecycleGateReport, String), ObserverModelError> {
        let ledger = self.load_or_init_release_ledger(repo_root)?;
        let ledger_ver = ledger.current_version.display_string();
        let mut checks: Vec<LifecycleCheckRecord> = Vec::new();

        // 1. In-process Total Ownership check (13 Managers, 0 orphans):
        let t0 = Instant::now();
        let ownership = self.check_total_ownership();
        let ownership_ok = ownership.complete;
        checks.push(LifecycleCheckRecord {
            check_id: "total-ownership".to_string(),
            owning_manager_id: "mgr-observer-executive".to_string(),
            category: "in-process".to_string(),
            passed: ownership_ok,
            duration_ms: t0.elapsed().as_millis() as u64,
            detail: format!(
                "{} managers, {} orphans",
                ownership.managers_checked,
                ownership.orphans.len()
            ),
        });

        // 2. In-process View Immutability Seal check:
        let t1 = Instant::now();
        let seal = self.verify_immutability_seal(repo_root);
        let seal_ok = seal.valid;
        checks.push(LifecycleCheckRecord {
            check_id: "immutability-seal".to_string(),
            owning_manager_id: "mgr-observer-executive".to_string(),
            category: "in-process".to_string(),
            passed: seal_ok,
            duration_ms: t1.elapsed().as_millis() as u64,
            detail: format!(
                "valid={} entries={} violations={}",
                seal.valid,
                seal.checked_files,
                seal.violations.len()
            ),
        });

        // 3. In-process Immutable Blank Frame <= 30 KiB invariant:
        let t2 = Instant::now();
        let (blank_ok, blank_detail) = match Self::audit_blank_frame_invariants(repo_root) {
            Ok(bytes) => (
                bytes <= MAX_BLANK_FRAME_BYTES,
                format!("{bytes} B <= {MAX_BLANK_FRAME_BYTES} B"),
            ),
            Err(e) => (false, e.to_string()),
        };
        checks.push(LifecycleCheckRecord {
            check_id: "blank-frame-invariants".to_string(),
            owning_manager_id: "mgr-change-release".to_string(),
            category: "in-process".to_string(),
            passed: blank_ok,
            duration_ms: t2.elapsed().as_millis() as u64,
            detail: blank_detail,
        });

        // 4. In-process Release Version & Tag Qualification Sync (Cargo.toml == wasm == ledger == manifest == tag):
        let t3 = Instant::now();
        let cargo_raw = std::fs::read_to_string(repo_root.join("Cargo.toml")).unwrap_or_default();
        let cargo_ver = cargo_raw
            .split("[workspace.package]")
            .nth(1)
            .and_then(|sec| {
                sec.lines().find_map(|l| {
                    let tr = l.trim();
                    tr.strip_prefix("version = ")
                        .map(|v| v.trim().trim_matches('"').to_string())
                })
            })
            .unwrap_or_default();
        let wasm_raw = std::fs::read_to_string(repo_root.join("crates/verdant-wasm/Cargo.toml"))
            .unwrap_or_default();
        let wasm_synced = wasm_raw.contains(&format!("version = \"{cargo_ver}\""));
        let manifest_raw = std::fs::read_to_string(
            repo_root.join("site/generated/self-host-ratchet-manifest.toml"),
        )
        .unwrap_or_default();
        let manifest_ver = manifest_raw
            .lines()
            .find_map(|l| {
                l.trim()
                    .strip_prefix("version = ")
                    .map(|v| v.trim().trim_matches('"').to_string())
            })
            .unwrap_or_default();
        let parse_semver = |s: &str| -> Option<(u32, u32, u32)> {
            let mut it = s.trim().trim_start_matches('v').split('.');
            let maj = it.next()?.parse::<u32>().ok()?;
            let min = it.next()?.parse::<u32>().ok()?;
            let pat = it.next()?.parse::<u32>().ok()?;
            Some((maj, min, pat))
        };
        let cargo_t = parse_semver(&cargo_ver);
        let ledger_t = Some((
            ledger.current_version.major as u32,
            ledger.current_version.minor as u32,
            ledger.current_version.patch as u32,
        ));
        let manifest_t = parse_semver(&manifest_ver);
        let tag_matches = match opts.expected_tag.as_deref() {
            None => true,
            Some(tag) => {
                let clean = tag.trim().trim_start_matches("refs/tags/");
                clean == format!("v{cargo_ver}")
            }
        };
        let version_synced = match (cargo_t, ledger_t, manifest_t) {
            (Some((c1, c2, c3)), Some((l1, l2, l3)), Some((m1, m2, m3))) => {
                lifecycle_version_tuple_matches(c1, c2, c3, l1, l2, l3)
                    && lifecycle_version_tuple_matches(c1, c2, c3, m1, m2, m3)
                    && wasm_synced
                    && tag_matches
            }
            _ => false,
        };
        checks.push(LifecycleCheckRecord {
            check_id: "release-version-sync".to_string(),
            owning_manager_id: "mgr-change-release".to_string(),
            category: "in-process".to_string(),
            passed: version_synced,
            duration_ms: t3.elapsed().as_millis() as u64,
            detail: format!(
                "cargo=v{cargo_ver} ledger=v{ledger_ver} manifest=v{manifest_ver} wasm_synced={wasm_synced} tag_ok={tag_matches}"
            ),
        });

        // 5. In-process Formal Fidelity (0 residue) + uorust Frontier check:
        let t4 = Instant::now();
        let fidelity_raw =
            std::fs::read_to_string(repo_root.join("fidelity.toml")).unwrap_or_default();
        let fidelity_open = fidelity_raw.matches("[[residue]]").count();
        let frontier_raw =
            std::fs::read_to_string(repo_root.join("frontier.toml")).unwrap_or_default();
        let frontier_ok = frontier_raw.contains("spec = \"uorust/register/1\"")
            && frontier_raw.contains("lifted = ");
        let formal_ok = fidelity_open == 0 && frontier_ok;
        checks.push(LifecycleCheckRecord {
            check_id: "formal-fidelity-and-frontier".to_string(),
            owning_manager_id: "mgr-knowledge-provenance".to_string(),
            category: "in-process".to_string(),
            passed: formal_ok,
            duration_ms: t4.elapsed().as_millis() as u64,
            detail: format!("fidelity_open={fidelity_open} frontier_valid={frontier_ok}"),
        });

        // 6. In-process Full Lifted Lifecycle Coverage across all 7 categories:
        let t5 = Instant::now();
        let mut rust_n = 0u32;
        let mut test_n = 0u32;
        let mut lex_n = 0u32;
        let mut feat_n = 0u32;
        let mut model_n = 0u32;
        let mut script_n = 0u32;
        let mut view_n = 0u32;
        for line in manifest_raw.lines() {
            let tr = line.trim();
            if tr == "category = \"rust\"" {
                rust_n = rust_n.saturating_add(1);
            } else if tr == "category = \"verification-test\"" {
                test_n = test_n.saturating_add(1);
            } else if tr == "category = \"lexlean\"" {
                lex_n = lex_n.saturating_add(1);
            } else if tr == "category = \"feature-suite\"" {
                feat_n = feat_n.saturating_add(1);
            } else if tr == "category = \"model-governance\"" {
                model_n = model_n.saturating_add(1);
            } else if tr == "category = \"automation-script\"" {
                script_n = script_n.saturating_add(1);
            } else if tr == "category = \"viewpoint\"" {
                view_n = view_n.saturating_add(1);
            }
        }
        let all_cats = lifecycle_lifted_categories_complete_v7(
            rust_n, test_n, lex_n, feat_n, model_n, script_n, view_n,
        );
        let total_lifted = lifecycle_manifest_total_modules_v7(
            rust_n, test_n, lex_n, feat_n, model_n, script_n, view_n,
        );
        let lifted_coverage = LiftedLifecycleCoverage {
            rust_modules: rust_n,
            verification_test_modules: test_n,
            lexlean_modules: lex_n,
            feature_suite_modules: feat_n,
            model_governance_modules: model_n,
            automation_script_modules: script_n,
            viewpoint_modules: view_n,
            total_lifted_modules: total_lifted,
            all_categories_lifted: all_cats,
        };
        checks.push(LifecycleCheckRecord {
            check_id: "lifted-lifecycle-coverage".to_string(),
            owning_manager_id: "mgr-change-release".to_string(),
            category: "in-process".to_string(),
            passed: all_cats,
            duration_ms: t5.elapsed().as_millis() as u64,
            detail: format!(
                "total={total_lifted} (rust={rust_n}, test={test_n}, lex={lex_n}, feature={feat_n}, model={model_n}, script={script_n}, view={view_n})"
            ),
        });

        // 7. Self-invoked workspace validations & tests called directly from within Verdant:
        let run_cmd = |checks: &mut Vec<LifecycleCheckRecord>,
                       check_id: &str,
                       mgr: &str,
                       category: &str,
                       program: &str,
                       cmd_args: &[&str]| {
            let ts = Instant::now();
            match Command::new(program)
                .args(cmd_args)
                .current_dir(repo_root)
                .output()
            {
                Ok(out) => {
                    let ok = out.status.success();
                    let snippet = if ok {
                        String::from_utf8_lossy(&out.stdout)
                            .lines()
                            .rev()
                            .find(|l| !l.trim().is_empty())
                            .unwrap_or("OK")
                            .trim()
                            .to_string()
                    } else {
                        let stderr = String::from_utf8_lossy(&out.stderr);
                        let stdout = String::from_utf8_lossy(&out.stdout);
                        stderr
                            .lines()
                            .chain(stdout.lines())
                            .rev()
                            .find(|l| !l.trim().is_empty())
                            .unwrap_or("command failed")
                            .trim()
                            .to_string()
                    };
                    checks.push(LifecycleCheckRecord {
                        check_id: check_id.to_string(),
                        owning_manager_id: mgr.to_string(),
                        category: category.to_string(),
                        passed: ok,
                        duration_ms: ts.elapsed().as_millis() as u64,
                        detail: snippet,
                    });
                }
                Err(e) => {
                    checks.push(LifecycleCheckRecord {
                        check_id: check_id.to_string(),
                        owning_manager_id: mgr.to_string(),
                        category: category.to_string(),
                        passed: false,
                        duration_ms: ts.elapsed().as_millis() as u64,
                        detail: format!("spawn {program} failed: {e}"),
                    });
                }
            }
        };

        if opts.run_workspace_tests {
            run_cmd(
                &mut checks,
                "cargo-fmt-check",
                "mgr-verification-observatory",
                "workspace-test",
                "just",
                &["fmt-check"],
            );
            if std::env::var_os("PRISMPM_SDK_DIGEST").is_some() {
                run_cmd(
                    &mut checks,
                    "xtask-validate",
                    "mgr-architecture-design",
                    "workspace-test",
                    "cargo",
                    &["run", "-q", "--locked", "-p", "xtask", "--", "validate"],
                );
            } else {
                run_cmd(
                    &mut checks,
                    "xtask-check-model",
                    "mgr-architecture-design",
                    "workspace-test",
                    "cargo",
                    &["run", "-q", "--locked", "-p", "xtask", "--", "check-model"],
                );
                run_cmd(
                    &mut checks,
                    "xtask-audit-limits",
                    "mgr-architecture-design",
                    "workspace-test",
                    "cargo",
                    &["run", "-q", "--locked", "-p", "xtask", "--", "audit-limits"],
                );
                run_cmd(
                    &mut checks,
                    "xtask-audit-deferral",
                    "mgr-architecture-design",
                    "workspace-test",
                    "cargo",
                    &[
                        "run",
                        "-q",
                        "--locked",
                        "-p",
                        "xtask",
                        "--",
                        "audit-deferral",
                    ],
                );
            }
            run_cmd(
                &mut checks,
                "prism-fidelity-check",
                "mgr-verification-observatory",
                "workspace-test",
                "cargo",
                &[
                    "run",
                    "-q",
                    "--locked",
                    "-p",
                    "prism-fidelity",
                    "--",
                    "--project",
                    "app",
                    "--repo",
                    ".",
                    "--register",
                    "fidelity.toml",
                    "--ratchet",
                    "model/parametric-ratchet.toml",
                    "--cite",
                    "model/provenance.toml",
                    "app/src/Provenance.lex.tex",
                    "--measures",
                    "model/measures.toml",
                    "--lean",
                    "app/src/Fidelity.lex.tex",
                    "--check",
                ],
            );
            run_cmd(
                &mut checks,
                "workspace-clippy",
                "mgr-verification-observatory",
                "workspace-test",
                "cargo",
                &[
                    "clippy",
                    "--workspace",
                    "--all-targets",
                    "--locked",
                    "--",
                    "-D",
                    "warnings",
                ],
            );
            run_cmd(
                &mut checks,
                "conformance-bdd",
                "mgr-verification-observatory",
                "workspace-test",
                "cargo",
                &[
                    "test",
                    "--locked",
                    "-p",
                    "repo-conformance",
                    "--test",
                    "bdd",
                ],
            );
            run_cmd(
                &mut checks,
                "conformance-engine",
                "mgr-verification-observatory",
                "workspace-test",
                "cargo",
                &[
                    "test",
                    "--locked",
                    "-p",
                    "repo-conformance",
                    "--test",
                    "engine",
                ],
            );
            run_cmd(
                &mut checks,
                "unit-and-fidelity-tests",
                "mgr-verification-observatory",
                "workspace-test",
                "cargo",
                &[
                    "test",
                    "--locked",
                    "-p",
                    "prism-fidelity",
                    "-p",
                    "verdant-tensor",
                    "-p",
                    "verdant-core",
                    "--lib",
                ],
            );
        }

        if opts.run_browser_gates {
            run_cmd(
                &mut checks,
                "browser-stitch-pull-through",
                "mgr-change-release",
                "browser-e2e",
                "node",
                &["scripts/verify-stitch-release.mjs"],
            );
            run_cmd(
                &mut checks,
                "browser-uhd-and-vcs",
                "mgr-studio-alm",
                "browser-e2e",
                "node",
                &["scripts/verify-uhd-workload.mjs", "site"],
            );
        }

        let total_count = checks.len() as u32;
        let passed_count = checks.iter().filter(|c| c.passed).count() as u32;
        let all_passed = lifecycle_all_checks_passed(passed_count, total_count);
        let release_qualified = lifecycle_release_qualified(
            all_passed,
            version_synced,
            seal_ok,
            ownership_ok,
            blank_ok,
        );
        let pass_rate_permille = lifecycle_pass_rate_permille(passed_count, total_count);
        let mode = if opts.run_browser_gates {
            "full-with-browser".to_string()
        } else if opts.run_workspace_tests {
            "full".to_string()
        } else {
            "in-process".to_string()
        };

        let mut report = LifecycleGateReport {
            spec: "verdant/lifecycle-gate-report/1".to_string(),
            release_version: ledger_ver,
            release_epoch: ledger.release_epoch,
            mode,
            all_passed,
            release_qualified,
            passed_count,
            total_count,
            pass_rate_permille,
            lifted_coverage,
            checks,
            report_kappa_hex: String::new(),
        };
        let preimage = serde_json::to_vec(&report).unwrap_or_default();
        report.report_kappa_hex = sha256_hex(&preimage);

        let pretty = serde_json::to_string_pretty(&report).map_err(|e| {
            ObserverModelError::CommunicationEnvelopeRejected {
                channel: CommunicationChannelKind::OrganToOrgan.slug().to_string(),
                detail: format!("serialize lifecycle-gate-report: {e}"),
            }
        })?;
        if opts.write_report {
            let _ = Self::write_manager_file(
                repo_root,
                "mgr-verification-observatory",
                "site/generated/lifecycle-gate-report.json",
                &pretty,
            )?;
        }
        Ok((report, pretty))
    }
}

#[path = "lifecycle_automate.rs"]
mod lifecycle_automate;
pub use lifecycle_automate::*;
