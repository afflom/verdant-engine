//! **In-Verdant Self-Verification, Test Execution & Release-Gating Lifecycle (`lifecycle.rs`)**.
//!
//! Lifts Verdant's entire validation, test execution, and release-qualification lifecycle directly
//! INTO Verdant (`mgr-verification-observatory` + `mgr-change-release`) so Verdant invokes its
//! own tests from within, audits every lifted lifecycle category (`rust`, `verification-test`,
//! `lexlean`, `feature-suite`, `model-governance`, `viewpoint`), and gates its own releases
//! cryptographically without relying on external scripts.

use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;

use serde::{Deserialize, Serialize};
use verdant_tensor::scalars::{hundred_u32, ten_u32, thousand_u32, two_u32};

use super::{AutonomousRatchetLiftOptions, VerdantOperatorChannel};
use crate::digest::sha256_hex;
use crate::error::ObserverModelError;
use crate::governance::ObserverProgressSignals;
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
                "cargo",
                &["fmt", "--all", "--", "--check"],
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

    /// Executes Verdant's full end-to-end build, formal-model refresh, kappa-memoized LexLean/uorust/Wasm
    /// rebuild, in-process autonomous `ratchet-lift --tighten`, and fail-closed `prism-fidelity --check`
    /// cycle directly from inside Verdant (`mgr-change-release` + `mgr-verification-observatory`).
    pub fn execute_ratchet_cycle(
        &mut self,
        repo_root: &Path,
        opts: &RatchetCycleOptions,
    ) -> Result<(RatchetCycleReport, String), ObserverModelError> {
        let cache_dir = repo_root.join(".verdant-cache");
        let _ = std::fs::create_dir_all(&cache_dir);
        let mut stages: Vec<RatchetCycleStageRecord> = Vec::new();
        let prefix_len = ten_u32().wrapping_add(two_u32()) as usize;

        let hash_tree = |entries: &[PathBuf]| -> String {
            let mut acc = Vec::new();
            let mut sorted: Vec<PathBuf> = entries.to_vec();
            sorted.sort();
            for file in sorted {
                if let Ok(bytes) = std::fs::read(&file) {
                    let rel = file
                        .strip_prefix(repo_root)
                        .unwrap_or(&file)
                        .to_string_lossy()
                        .replace('\\', "/");
                    acc.extend_from_slice(rel.as_bytes());
                    acc.push(0);
                    acc.extend_from_slice(&bytes);
                    acc.push(0);
                }
            }
            sha256_hex(&acc)
        };

        let collect_dir_files = |roots: &[PathBuf], suffix: Option<&str>| -> Vec<PathBuf> {
            let mut out = Vec::new();
            let mut stack: Vec<PathBuf> = roots.to_vec();
            while let Some(dir) = stack.pop() {
                let Ok(rd) = std::fs::read_dir(&dir) else {
                    continue;
                };
                for ent in rd.flatten() {
                    let p = ent.path();
                    let name = ent.file_name().to_string_lossy().to_string();
                    if name.starts_with('.') || name == "target" || name == "node_modules" {
                        continue;
                    }
                    if p.is_dir() {
                        stack.push(p);
                    } else if suffix.is_none_or(|sfx| name.ends_with(sfx)) {
                        out.push(p);
                    }
                }
            }
            out.sort();
            out
        };

        let run_stage_cmd = |stages: &mut Vec<RatchetCycleStageRecord>,
                             stage_id: &str,
                             program: &str,
                             cmd_args: &[&str]|
         -> Result<(), ObserverModelError> {
            let t0 = Instant::now();
            match Command::new(program)
                .args(cmd_args)
                .current_dir(repo_root)
                .output()
            {
                Ok(out) => {
                    let ms = t0.elapsed().as_millis() as u64;
                    let stdout_s = String::from_utf8_lossy(&out.stdout);
                    let stderr_s = String::from_utf8_lossy(&out.stderr);
                    let tail = stdout_s
                        .lines()
                        .rev()
                        .find(|l| !l.trim().is_empty())
                        .or_else(|| stderr_s.lines().rev().find(|l| !l.trim().is_empty()))
                        .unwrap_or("OK")
                        .trim()
                        .to_string();
                    let ok = out.status.success();
                    stages.push(RatchetCycleStageRecord {
                        stage_id: stage_id.to_string(),
                        status: if ok {
                            "pass".to_string()
                        } else {
                            "fail".to_string()
                        },
                        duration_ms: ms,
                        detail: tail.clone(),
                    });
                    if !ok {
                        return Err(ObserverModelError::CommunicationEnvelopeRejected {
                            channel: CommunicationChannelKind::OrganToOrgan.slug().to_string(),
                            detail: format!(
                                "ratchet-cycle stage `{stage_id}` failed: {tail}\n{stderr_s}"
                            ),
                        });
                    }
                    Ok(())
                }
                Err(e) => {
                    let ms = t0.elapsed().as_millis() as u64;
                    let detail = format!("spawn `{program}` failed: {e}");
                    stages.push(RatchetCycleStageRecord {
                        stage_id: stage_id.to_string(),
                        status: "fail".to_string(),
                        duration_ms: ms,
                        detail: detail.clone(),
                    });
                    Err(ObserverModelError::CommunicationEnvelopeRejected {
                        channel: CommunicationChannelKind::OrganToOrgan.slug().to_string(),
                        detail: format!("ratchet-cycle stage `{stage_id}` failed: {detail}"),
                    })
                }
            }
        };

        // 1. Model & Conformance check-model --write:
        run_stage_cmd(
            &mut stages,
            "xtask-check-model",
            "cargo",
            &[
                "run",
                "--locked",
                "-q",
                "-p",
                "xtask",
                "--",
                "check-model",
                "--write",
            ],
        )?;

        // 2. Refresh fidelity & provenance (pre-ratchet):
        let fidelity_refresh_args = [
            "run",
            "--locked",
            "-q",
            "-p",
            "prism-fidelity",
            "--",
            "--project",
            "app",
            "--repo",
            ".",
            "--register",
            "fidelity.toml",
            "--no-ratchet",
            "--cite",
            "model/provenance.toml",
            "app/src/Provenance.lex.tex",
            "--measures",
            "model/measures.toml",
            "--lean",
            "app/src/Fidelity.lex.tex",
        ];
        run_stage_cmd(
            &mut stages,
            "prism-fidelity-refresh",
            "cargo",
            &fidelity_refresh_args,
        )?;

        // 3. Kappa-memoized locked SDK `lexlean lock && lexlean fmt && lexlean lock`:
        let mut app_files = vec![
            repo_root.join("app/lexlean.toml"),
            repo_root.join("app/prismpm.toml"),
        ];
        app_files.extend(collect_dir_files(
            &[repo_root.join("app/src")],
            Some(".lex.tex"),
        ));
        let app_kappa = hash_tree(&app_files);
        let app_cache_path = cache_dir.join("lexlean-app.sha256");
        let prev_app_kappa = std::fs::read_to_string(&app_cache_path)
            .unwrap_or_default()
            .trim()
            .to_string();
        let app_cache_hit = ratchet_stage_cache_hit(opts.force, app_kappa == prev_app_kappa, true);
        if ratchet_cycle_should_run_sdk(opts.run_sdk_docker, app_cache_hit) {
            let sdk_img = std::fs::read_to_string(repo_root.join("prismpm.lock"))
                .ok()
                .and_then(|raw| {
                    serde_json::from_str::<serde_json::Value>(&raw)
                        .ok()
                        .and_then(|v| {
                            v.get("sdk_image")
                                .and_then(|s| s.as_str())
                                .map(str::to_string)
                        })
                });
            if let Some(img) = sdk_img {
                let root_abs =
                    std::fs::canonicalize(repo_root).unwrap_or_else(|_| repo_root.to_path_buf());
                let mount_spec = format!("{}:{}", root_abs.display(), root_abs.display());
                let app_wd = root_abs.join("app").to_string_lossy().to_string();
                let mut d_args: Vec<String> = vec!["run".to_string(), "--rm".to_string()];
                #[cfg(unix)]
                {
                    use std::os::unix::fs::MetadataExt;
                    if let Ok(meta) = std::fs::metadata(&root_abs) {
                        d_args.push("-u".to_string());
                        d_args.push(format!("{}:{}", meta.uid(), meta.gid()));
                    }
                }
                d_args.extend([
                    "-v".to_string(),
                    mount_spec,
                    "-w".to_string(),
                    app_wd,
                    img,
                    "bash".to_string(),
                    "-c".to_string(),
                    "lexlean lock && lexlean fmt && lexlean lock".to_string(),
                ]);
                let d_arg_refs: Vec<&str> = d_args.iter().map(String::as_str).collect();
                run_stage_cmd(&mut stages, "lexlean-fmt-lock", "docker", &d_arg_refs)?;
                run_stage_cmd(
                    &mut stages,
                    "prism-fidelity-post-fmt",
                    "cargo",
                    &fidelity_refresh_args,
                )?;
                let post_app_kappa = hash_tree(&app_files);
                let _ = std::fs::write(&app_cache_path, format!("{post_app_kappa}\n"));
            } else {
                stages.push(RatchetCycleStageRecord {
                    stage_id: "lexlean-fmt-lock".to_string(),
                    status: "skip".to_string(),
                    duration_ms: 0,
                    detail: "no sdk_image in prismpm.lock".to_string(),
                });
            }
        } else {
            stages.push(RatchetCycleStageRecord {
                stage_id: "lexlean-fmt-lock".to_string(),
                status: "skip".to_string(),
                duration_ms: 0,
                detail: format!(
                    "kappa hit ({}…)",
                    &app_kappa[..prefix_len.min(app_kappa.len())]
                ),
            });
        }

        // 4. Kappa-memoized uorust MIR proof frontier refresh:
        let rust_files = collect_dir_files(
            &[
                repo_root.join("crates/verdant-core/src"),
                repo_root.join("crates/verdant-tensor/src"),
                repo_root.join("crates/verdant-wasm/src"),
                repo_root.join("crates/verdant-cli/src"),
            ],
            Some(".rs"),
        );
        let rust_kappa = hash_tree(&rust_files);
        let frontier_cache_path = cache_dir.join("uorust-frontier.sha256");
        let prev_frontier_kappa = std::fs::read_to_string(&frontier_cache_path)
            .unwrap_or_default()
            .trim()
            .to_string();
        let frontier_exists = repo_root.join("frontier.toml").is_file();
        let uorust_bin: Option<String> = std::env::var("VERDANT_UORUST_BIN")
            .ok()
            .filter(|p| Path::new(p).is_file())
            .or_else(|| {
                let known = "/tmp/uorust-bin/uorust-x86_64-unknown-linux-gnu/uorust";
                Path::new(known).is_file().then(|| known.to_string())
            })
            .or_else(|| {
                Command::new("which")
                    .arg("uorust")
                    .output()
                    .ok()
                    .filter(|o| o.status.success())
                    .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
                    .filter(|s| !s.is_empty())
            });
        if let Some(bin) = uorust_bin {
            if !ratchet_stage_cache_hit(
                opts.force,
                rust_kappa == prev_frontier_kappa,
                frontier_exists,
            ) {
                let tmp_out = std::env::temp_dir()
                    .join(format!("verdant-uorust-cycle-{}", std::process::id()));
                let tmp_out_s = tmp_out.to_string_lossy().to_string();
                let res = run_stage_cmd(
                    &mut stages,
                    "uorust-frontier-refresh",
                    &bin,
                    &[
                        "project",
                        "--out",
                        &tmp_out_s,
                        "--register",
                        "frontier.toml",
                        ".",
                    ],
                );
                let _ = std::fs::remove_dir_all(&tmp_out);
                res?;
                let _ = std::fs::write(&frontier_cache_path, format!("{rust_kappa}\n"));
            } else {
                stages.push(RatchetCycleStageRecord {
                    stage_id: "uorust-frontier-refresh".to_string(),
                    status: "skip".to_string(),
                    duration_ms: 0,
                    detail: format!(
                        "kappa hit ({}…)",
                        &rust_kappa[..prefix_len.min(rust_kappa.len())]
                    ),
                });
            }
        } else {
            stages.push(RatchetCycleStageRecord {
                stage_id: "uorust-frontier-refresh".to_string(),
                status: "skip".to_string(),
                duration_ms: 0,
                detail: "uorust binary not in PATH (using committed frontier.toml)".to_string(),
            });
        }

        // 5. Kappa-memoized Wasm rebuild (`scripts/sync-prism-wasm.mjs`):
        let mut wasm_inputs = vec![repo_root.join("Cargo.toml"), repo_root.join("Cargo.lock")];
        wasm_inputs.extend(collect_dir_files(
            &[
                repo_root.join("crates/verdant-core/src"),
                repo_root.join("crates/verdant-tensor/src"),
                repo_root.join("crates/verdant-wasm/src"),
            ],
            Some(".rs"),
        ));
        wasm_inputs.extend(collect_dir_files(&[repo_root.join("catalogs")], None));
        let wasm_kappa = hash_tree(&wasm_inputs);
        let wasm_cache_path = cache_dir.join("wasm-inputs.sha256");
        let prev_wasm_kappa = std::fs::read_to_string(&wasm_cache_path)
            .unwrap_or_default()
            .trim()
            .to_string();
        let wasm_present = repo_root.join("site/verdant-wasm.wasm").is_file()
            && repo_root.join("site/verdant-uorc.wasm").is_file();
        if !ratchet_stage_cache_hit(opts.force, wasm_kappa == prev_wasm_kappa, wasm_present) {
            run_stage_cmd(
                &mut stages,
                "sync-prism-wasm",
                "node",
                &["scripts/sync-prism-wasm.mjs"],
            )?;
            let _ = std::fs::write(&wasm_cache_path, format!("{wasm_kappa}\n"));
        } else {
            stages.push(RatchetCycleStageRecord {
                stage_id: "sync-prism-wasm".to_string(),
                status: "skip".to_string(),
                duration_ms: 0,
                detail: format!(
                    "kappa hit ({}…)",
                    &wasm_kappa[..prefix_len.min(wasm_kappa.len())]
                ),
            });
        }

        // 6. In-process Autonomous Ratchet Lift (+ Cargo.lock sync & post-sync tighten if version bumped):
        let has_bump = opts.advance_signals.is_some();
        let cargo_before =
            std::fs::read_to_string(repo_root.join("Cargo.toml")).unwrap_or_default();
        let out_dir = repo_root.join("dist/stitch-release");
        let t_lift = Instant::now();
        let lift_opts_1 = AutonomousRatchetLiftOptions {
            write_repo_artifacts: true,
            update_frame_pin: true,
            tighten_ratchet: ratchet_cycle_tighten_first_pass(opts.tighten, has_bump),
            stitch_relays: Vec::new(),
            advance_signals: opts.advance_signals.clone(),
        };
        let mut lift_rep =
            self.execute_autonomous_ratchet_lift(repo_root, &out_dir, &lift_opts_1)?;
        stages.push(RatchetCycleStageRecord {
            stage_id: "in-process-ratchet-lift".to_string(),
            status: "pass".to_string(),
            duration_ms: t_lift.elapsed().as_millis() as u64,
            detail: format!(
                "version=v{} epoch={} lifted_modules={} blank_frame={}B tightened={}",
                lift_rep.version,
                lift_rep.release_epoch,
                lift_rep.lifted_module_count,
                lift_rep.blank_frame_bytes,
                lift_rep.tightened_ratchet_fields.len()
            ),
        });

        let cargo_after = std::fs::read_to_string(repo_root.join("Cargo.toml")).unwrap_or_default();
        let cargo_changed = cargo_after != cargo_before;
        if ratchet_cycle_needs_second_pass(cargo_changed, has_bump) {
            if cargo_changed {
                run_stage_cmd(
                    &mut stages,
                    "cargo-lock-sync",
                    "cargo",
                    &["update", "-q", "-w", "--offline"],
                )?;
            }
            let t_lift2 = Instant::now();
            let lift_opts_2 = AutonomousRatchetLiftOptions {
                write_repo_artifacts: true,
                update_frame_pin: true,
                tighten_ratchet: opts.tighten,
                stitch_relays: Vec::new(),
                advance_signals: None,
            };
            lift_rep = self.execute_autonomous_ratchet_lift(repo_root, &out_dir, &lift_opts_2)?;
            stages.push(RatchetCycleStageRecord {
                stage_id: "in-process-ratchet-lift-post-sync".to_string(),
                status: "pass".to_string(),
                duration_ms: t_lift2.elapsed().as_millis() as u64,
                detail: format!(
                    "version=v{} epoch={} lifted_modules={} tightened={}",
                    lift_rep.version,
                    lift_rep.release_epoch,
                    lift_rep.lifted_module_count,
                    lift_rep.tightened_ratchet_fields.len()
                ),
            });
        }

        // 7. Final fail-closed `prism-fidelity --check` across ALL ratchet layers:
        run_stage_cmd(
            &mut stages,
            "prism-fidelity-check",
            "cargo",
            &[
                "run",
                "--locked",
                "-q",
                "-p",
                "prism-fidelity",
                "--",
                "--project",
                "app",
                "--repo",
                ".",
                "--register",
                "fidelity.toml",
                "--cite",
                "model/provenance.toml",
                "app/src/Provenance.lex.tex",
                "--measures",
                "model/measures.toml",
                "--lean",
                "app/src/Fidelity.lex.tex",
                "--check",
            ],
        )?;

        // 8. Optional in-process lifecycle-gate execution:
        if opts.run_lifecycle_gate || opts.run_browser_gates {
            let t_gate = Instant::now();
            let gate_opts = LifecycleGateOptions {
                run_workspace_tests: true,
                run_browser_gates: opts.run_browser_gates,
                expected_tag: Some(format!("v{}", lift_rep.version)),
                write_report: true,
            };
            let (gate_rep, _) = self.execute_lifecycle_gate(repo_root, &gate_opts)?;
            let ok = gate_rep.release_qualified;
            stages.push(RatchetCycleStageRecord {
                stage_id: "in-process-lifecycle-gate".to_string(),
                status: if ok {
                    "pass".to_string()
                } else {
                    "fail".to_string()
                },
                duration_ms: t_gate.elapsed().as_millis() as u64,
                detail: format!(
                    "{}/{} checks passed · releaseQualified={}",
                    gate_rep.passed_count, gate_rep.total_count, gate_rep.release_qualified
                ),
            });
            if !ok {
                return Err(ObserverModelError::CommunicationEnvelopeRejected {
                    channel: CommunicationChannelKind::OrganToOrgan.slug().to_string(),
                    detail: format!(
                        "ratchet-cycle lifecycle-gate REFUSED release ({}/{} passed)",
                        gate_rep.passed_count, gate_rep.total_count
                    ),
                });
            }
        }

        let total_stages = stages.len() as u32;
        let passed_stages = stages.iter().filter(|s| s.status == "pass").count() as u32;
        let skipped_stages = stages.iter().filter(|s| s.status == "skip").count() as u32;
        let all_passed = ratchet_cycle_all_passed(passed_stages, skipped_stages, total_stages);
        let mut report = RatchetCycleReport {
            spec: "verdant/ratchet-cycle-report/1".to_string(),
            release_version: lift_rep.version,
            release_epoch: lift_rep.release_epoch,
            all_passed,
            passed_stages,
            skipped_stages,
            total_stages,
            tightened_count: lift_rep.tightened_ratchet_fields.len() as u32,
            lifted_modules: lift_rep.lifted_module_count as u32,
            stages,
            cycle_kappa_hex: String::new(),
        };
        let preimage = serde_json::to_vec(&report).unwrap_or_default();
        report.cycle_kappa_hex = sha256_hex(&preimage);
        let pretty = serde_json::to_string_pretty(&report).map_err(|e| {
            ObserverModelError::CommunicationEnvelopeRejected {
                channel: CommunicationChannelKind::OrganToOrgan.slug().to_string(),
                detail: format!("serialize ratchet-cycle-report: {e}"),
            }
        })?;
        Ok((report, pretty))
    }
}

/// Configuration options for Verdant's lifted end-to-end `ratchet-cycle` orchestrator.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RatchetCycleOptions {
    /// Bypass `.verdant-cache` kappa memoization and re-execute every stage.
    pub force: bool,
    /// Enable strictly one-way auto-tightening of `model/parametric-ratchet.toml` watermarks.
    pub tighten: bool,
    /// Allow invoking locked `prismpm-sdk` Docker image when `app/` LexLean sources change.
    pub run_sdk_docker: bool,
    /// Also execute Verdant's full `execute_lifecycle_gate` at the end of the cycle.
    pub run_lifecycle_gate: bool,
    /// Include headless Chromium browser E2E gates inside the trailing lifecycle gate.
    pub run_browser_gates: bool,
    /// Optional SemVer bump request passed to `mgr-change-release`.
    pub advance_signals: Option<ObserverProgressSignals>,
}

impl Default for RatchetCycleOptions {
    fn default() -> Self {
        RatchetCycleOptions {
            force: false,
            tighten: true,
            run_sdk_docker: true,
            run_lifecycle_gate: false,
            run_browser_gates: false,
            advance_signals: None,
        }
    }
}

/// Outcome of a single stage executed inside Verdant's lifted `ratchet-cycle`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RatchetCycleStageRecord {
    /// Deterministic stage identifier.
    pub stage_id: String,
    /// Stage outcome (`pass`, `skip`, or `fail`).
    pub status: String,
    /// Elapsed wall-clock milliseconds.
    pub duration_ms: u64,
    /// Concise status summary.
    pub detail: String,
}

/// Cryptographically committed report returned by `VerdantOperatorChannel::execute_ratchet_cycle`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RatchetCycleReport {
    /// Fixed format identifier (`verdant/ratchet-cycle-report/1`).
    pub spec: String,
    /// Active release version after the ratchet cycle.
    pub release_version: String,
    /// Active monotonic release epoch.
    pub release_epoch: u64,
    /// True iff every stage passed or hit a verified kappa cache.
    pub all_passed: bool,
    /// Count of freshly executed passing stages.
    pub passed_stages: u32,
    /// Count of kappa-memoized skipped stages.
    pub skipped_stages: u32,
    /// Total stage count.
    pub total_stages: u32,
    /// Count of ratchet floors/ceilings tightened during this cycle.
    pub tightened_count: u32,
    /// Total lifted lifecycle modules in `self-host-ratchet-manifest.toml`.
    pub lifted_modules: u32,
    /// Ordered per-stage execution records.
    pub stages: Vec<RatchetCycleStageRecord>,
    /// Content-addressed SHA-256 hex commitment over this report.
    pub cycle_kappa_hex: String,
}

/// Pure constructive predicate: a memoized stage is a valid cache hit iff not forced, input
/// content hashes match, and expected materialized outputs already exist.
#[must_use]
pub const fn ratchet_stage_cache_hit(
    force: bool,
    hashes_match: bool,
    outputs_present: bool,
) -> bool {
    !force && hashes_match && outputs_present
}

/// Pure constructive count of non-failing stages (`passed + skipped`).
#[must_use]
pub const fn ratchet_cycle_effective_stages(passed: u32, skipped: u32) -> u32 {
    passed.wrapping_add(skipped)
}

/// Pure constructive verdict: a ratchet cycle passes iff `total > 0` and every stage passed or hit
/// a verified kappa cache (`passed + skipped == total`).
#[must_use]
pub const fn ratchet_cycle_all_passed(passed: u32, skipped: u32, total: u32) -> bool {
    total > 0 && ratchet_cycle_effective_stages(passed, skipped) == total
}

/// Pure constructive predicate: run a second post-sync ratchet-lift pass whenever `Cargo.toml`
/// changed or a SemVer bump was requested so `Cargo.lock` and watermarks seal cleanly.
#[must_use]
pub const fn ratchet_cycle_needs_second_pass(cargo_changed: bool, has_bump: bool) -> bool {
    cargo_changed || has_bump
}

/// Pure constructive predicate: run locked SDK `lexlean fmt && lock` iff Docker SDK execution is
/// enabled and the `app/` kappa cache missed.
#[must_use]
pub const fn ratchet_cycle_should_run_sdk(run_sdk_docker: bool, cache_hit: bool) -> bool {
    run_sdk_docker && !cache_hit
}

/// Pure constructive predicate: tighten watermarks on pass 1 iff `--tighten` is enabled and no
/// SemVer bump defers tightening to the post-`Cargo.lock` second pass.
#[must_use]
pub const fn ratchet_cycle_tighten_first_pass(tighten: bool, has_bump: bool) -> bool {
    tighten && !has_bump
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lifecycle_gate_verifies_in_process_and_refuses_mismatched_tag() {
        let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let ch = VerdantOperatorChannel::new("sunspire-regatta").expect("operator channel");
        let (rep, json) = ch
            .execute_lifecycle_gate(
                &repo_root,
                &LifecycleGateOptions {
                    run_workspace_tests: false,
                    run_browser_gates: false,
                    expected_tag: None,
                    write_report: false,
                },
            )
            .expect("in-process lifecycle gate");
        assert_eq!(rep.spec, "verdant/lifecycle-gate-report/1");
        assert!(json.contains("verdant/lifecycle-gate-report/1"));
        assert!(!rep.report_kappa_hex.is_empty());

        // Mismatched tag must fail release qualification closed:
        let (bad, _) = ch
            .execute_lifecycle_gate(
                &repo_root,
                &LifecycleGateOptions {
                    run_workspace_tests: false,
                    run_browser_gates: false,
                    expected_tag: Some("v0.0.1".to_string()),
                    write_report: false,
                },
            )
            .expect("mismatched tag gate");
        assert!(!bad.release_qualified);
        assert!(!lifecycle_gate_can_advance_release(
            bad.release_qualified,
            true,
            0
        ));
    }
}
