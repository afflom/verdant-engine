//! Lifted self-automation pipeline, ratchet-cycle report types, and constructive stage
//! predicates (`lifecycle_automate.rs`).

use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;

use serde::{Deserialize, Serialize};
use verdant_tensor::scalars::{
    automate_should_build_wasm, automate_should_run_rust_gates, automate_stage_passed, ten_u32,
    two_u32,
};

use super::super::{AutonomousRatchetLiftOptions, NativeBuildOp, VerdantOperatorChannel};
use super::LifecycleGateOptions;
use crate::digest::sha256_hex;
use crate::error::ObserverModelError;
use crate::governance::ObserverProgressSignals;
use crate::observer::CommunicationChannelKind;

impl VerdantOperatorChannel {
    /// Executes Verdant's full lifted self-automation & multi-gate verification pipeline
    /// (`verdant operator automate` / MCP `verdant_automate`) directly from inside Verdant:
    /// kappa-memoized Wasm build, in-process ratchet self-lift, site & RFC 2789 Cargo Sparse
    /// Registry assembly, in-process `lifecycle-gate`, and live contract + browser gates.
    pub fn execute_automate_pipeline(
        &mut self,
        repo_root: &Path,
        opts: &AutomateOptions,
    ) -> Result<(AutomateReport, String), ObserverModelError> {
        let cache_dir = repo_root.join(".verdant-cache");
        let _ = std::fs::create_dir_all(&cache_dir);
        let mut stages: Vec<RatchetCycleStageRecord> = Vec::new();
        let prefix_len = ten_u32().wrapping_add(two_u32()) as usize;

        let run_step = |stages: &mut Vec<RatchetCycleStageRecord>,
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
                                "automate stage `{stage_id}` failed: {tail}\n{stderr_s}"
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
                        detail: format!("automate stage `{stage_id}` failed: {detail}"),
                    })
                }
            }
        };

        // 1. Kappa-memoized Wasm build:
        let mut wasm_inputs = vec![repo_root.join("Cargo.toml"), repo_root.join("Cargo.lock")];
        let mut walk_stack = vec![
            repo_root.join("crates/verdant-core/src"),
            repo_root.join("crates/verdant-tensor/src"),
            repo_root.join("crates/verdant-wasm/src"),
            repo_root.join("catalogs"),
        ];
        while let Some(dir) = walk_stack.pop() {
            if let Ok(rd) = std::fs::read_dir(&dir) {
                for ent in rd.flatten() {
                    let p = ent.path();
                    if p.is_dir() {
                        walk_stack.push(p);
                    } else if p.is_file() {
                        wasm_inputs.push(p);
                    }
                }
            }
        }
        wasm_inputs.sort();
        let mut hash_buf = Vec::new();
        for f in &wasm_inputs {
            if let Ok(bytes) = std::fs::read(f) {
                let rel = f
                    .strip_prefix(repo_root)
                    .unwrap_or(f)
                    .to_string_lossy()
                    .replace('\\', "/");
                hash_buf.extend_from_slice(rel.as_bytes());
                hash_buf.push(0);
                hash_buf.extend_from_slice(&bytes);
                hash_buf.push(0);
            }
        }
        let wasm_kappa = sha256_hex(&hash_buf);
        let wasm_cache_path = cache_dir.join("wasm-inputs.sha256");
        let prev_kappa = std::fs::read_to_string(&wasm_cache_path)
            .unwrap_or_default()
            .trim()
            .to_string();
        let wasm_present = repo_root.join("site/verdant-wasm.wasm").is_file()
            && repo_root.join("site/verdant-uorc.wasm").is_file();
        let cache_hit = wasm_kappa == prev_kappa && wasm_present;
        if automate_should_build_wasm(opts.skip_rust, opts.force_wasm, cache_hit) {
            let t_wasm = Instant::now();
            let (rep, _) = self.execute_native_build_op(
                repo_root,
                &NativeBuildOp::SyncWasm {
                    check_only: false,
                    repack_stream: false,
                },
            )?;
            stages.push(RatchetCycleStageRecord {
                stage_id: "sync-prism-wasm".to_string(),
                status: "pass".to_string(),
                duration_ms: t_wasm.elapsed().as_millis() as u64,
                detail: rep.summary,
            });
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

        // 2. In-process ratchet self-lift (non-mutating frame-pin/tighten):
        let run_rust = automate_should_run_rust_gates(opts.skip_rust);
        if run_rust {
            let t_lift = Instant::now();
            let lift_opts = AutonomousRatchetLiftOptions {
                write_repo_artifacts: true,
                update_frame_pin: false,
                tighten_ratchet: false,
                stitch_relays: Vec::new(),
                advance_signals: None,
            };
            let lift_rep = self.execute_autonomous_ratchet_lift(
                repo_root,
                &repo_root.join("dist/stitch-release"),
                &lift_opts,
            )?;
            stages.push(RatchetCycleStageRecord {
                stage_id: "in-process-ratchet-lift".to_string(),
                status: "pass".to_string(),
                duration_ms: t_lift.elapsed().as_millis() as u64,
                detail: format!(
                    "version=v{} epoch={} lifted_modules={}",
                    lift_rep.version, lift_rep.release_epoch, lift_rep.lifted_module_count
                ),
            });
        }

        // 3. Site assembly, native RFC 2789 Cargo Sparse Registry packaging, and in-process lifecycle-gate:
        run_step(
            &mut stages,
            "build-site",
            "node",
            &["scripts/build-site.mjs"],
        )?;
        {
            let t_pkg = Instant::now();
            let (pkg_rep, _) = self.execute_native_build_op(
                repo_root,
                &NativeBuildOp::PackageRustCrates {
                    out_dir: repo_root.join("site/crates"),
                    dl_template:
                        "https://afflom.github.io/verdant-engine/crates/crates/{crate}-{version}.crate"
                            .to_string(),
                },
            )?;
            stages.push(RatchetCycleStageRecord {
                stage_id: "package-rust-crates".to_string(),
                status: "pass".to_string(),
                duration_ms: t_pkg.elapsed().as_millis() as u64,
                detail: pkg_rep.summary,
            });
        }
        if run_rust {
            run_step(
                &mut stages,
                "verdant-automate-check",
                "cargo",
                &[
                    "run",
                    "--locked",
                    "-q",
                    "-p",
                    "verdant-cli",
                    "--",
                    "automate-check",
                ],
            )?;
            let t_gate = Instant::now();
            let (gate_rep, _) = self.execute_lifecycle_gate(
                repo_root,
                &LifecycleGateOptions {
                    run_workspace_tests: false,
                    run_browser_gates: false,
                    expected_tag: None,
                    write_report: false,
                },
            )?;
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
                    "{}/{} checks passed · lifted={}",
                    gate_rep.passed_count,
                    gate_rep.total_count,
                    gate_rep.lifted_coverage.total_lifted_modules
                ),
            });
            if !ok {
                return Err(ObserverModelError::CommunicationEnvelopeRejected {
                    channel: CommunicationChannelKind::OrganToOrgan.slug().to_string(),
                    detail: "automate in-process lifecycle-gate failed".to_string(),
                });
            }
        }

        // 4. Contract, surface, weight (FR-03), UHD/VCS, sparse registry, tag & Stitch gates:
        run_step(
            &mut stages,
            "prism-contracts-test",
            "npx",
            &["--yes", "tsx", "--test", "prism-contracts.test.ts"],
        )?;
        run_step(
            &mut stages,
            "verify-live-surfaces",
            "node",
            &["scripts/verify-live-surfaces.mjs", "site"],
        )?;
        run_step(
            &mut stages,
            "content-check",
            "node",
            &["scripts/content/check.mjs", "site"],
        )?;
        run_step(
            &mut stages,
            "weight-fr03",
            "node",
            &["bootstrap/weight.mjs", "site"],
        )?;
        run_step(
            &mut stages,
            "verify-uhd-workload",
            "node",
            &["scripts/verify-uhd-workload.mjs", "site"],
        )?;
        {
            let t_reg = Instant::now();
            let (reg_rep, _) = self.execute_native_build_op(
                repo_root,
                &NativeBuildOp::VerifySparseRegistry {
                    crates_root: repo_root.join("site/crates"),
                },
            )?;
            stages.push(RatchetCycleStageRecord {
                stage_id: "verify-sparse-registry".to_string(),
                status: "pass".to_string(),
                duration_ms: t_reg.elapsed().as_millis() as u64,
                detail: reg_rep.summary,
            });
        }
        run_step(
            &mut stages,
            "qualify-tag-offline",
            "node",
            &["scripts/release/qualify-tag.mjs", "--offline"],
        )?;
        if run_rust {
            run_step(
                &mut stages,
                "verify-stitch-release",
                "node",
                &["scripts/verify-stitch-release.mjs"],
            )?;
        }
        let _ = std::fs::remove_file(repo_root.join("site/content-report.json"));
        let _ = std::fs::remove_file(repo_root.join("site/content-report.md"));

        let total_stages = stages.len() as u32;
        let passed_stages = stages.iter().filter(|s| s.status == "pass").count() as u32;
        let skipped_stages = stages.iter().filter(|s| s.status == "skip").count() as u32;
        let all_passed = automate_stage_passed(passed_stages, skipped_stages, total_stages);
        let mut report = AutomateReport {
            spec: "verdant/automate-report/1".to_string(),
            all_passed,
            passed_stages,
            skipped_stages,
            total_stages,
            stages,
            report_kappa_hex: String::new(),
        };
        let preimage = serde_json::to_vec(&report).unwrap_or_default();
        report.report_kappa_hex = sha256_hex(&preimage);
        let pretty = serde_json::to_string_pretty(&report).map_err(|e| {
            ObserverModelError::CommunicationEnvelopeRejected {
                channel: CommunicationChannelKind::OrganToOrgan.slug().to_string(),
                detail: format!("serialize automate-report: {e}"),
            }
        })?;
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

        // 3. Kappa-memoized LexLean 0.4.5 `lexlean lock && lexlean fmt && lexlean lock`:
        let mut app_files = vec![
            repo_root.join("app/lexlean.toml"),
            repo_root.join("template.lock"),
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
            let host_lexlean_ok = std::process::Command::new("lexlean")
                .arg("--version")
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false);
            if host_lexlean_ok {
                let app_wd = repo_root.join("app").to_string_lossy().to_string();
                run_stage_cmd(
                    &mut stages,
                    "lexlean-fmt-lock",
                    "bash",
                    &[
                        "-c",
                        &format!("cd {app_wd:?} && lexlean lock && lexlean fmt && lexlean lock"),
                    ],
                )?;
            } else {
                let img = "ghcr.io/uor-foundation/lexlean@sha256:d391bfedd4eb97dc25c218628f94cd081436589c348797ebf13c47ab13d4ef30";
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
                    "--entrypoint".to_string(),
                    "sh".to_string(),
                    img.to_string(),
                    "-c".to_string(),
                    "lexlean lock && lexlean fmt && lexlean lock".to_string(),
                ]);
                let d_arg_refs: Vec<&str> = d_args.iter().map(String::as_str).collect();
                run_stage_cmd(&mut stages, "lexlean-fmt-lock", "docker", &d_arg_refs)?;
            }
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
            let t_wasm = Instant::now();
            let (rep, _) = self.execute_native_build_op(
                repo_root,
                &NativeBuildOp::SyncWasm {
                    check_only: false,
                    repack_stream: false,
                },
            )?;
            stages.push(RatchetCycleStageRecord {
                stage_id: "sync-prism-wasm".to_string(),
                status: "pass".to_string(),
                duration_ms: t_wasm.elapsed().as_millis() as u64,
                detail: rep.summary,
            });
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
                write_report: false,
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

/// Configuration options for Verdant's lifted self-automation pipeline (`verdant operator automate`).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutomateOptions {
    /// Skip Rust/Wasm/Stitch stages and execute only site/JS verification gates.
    pub skip_rust: bool,
    /// Force Wasm rebuild even on a `.verdant-cache` kappa hit.
    pub force_wasm: bool,
}

/// Cryptographically committed report returned by `VerdantOperatorChannel::execute_automate_pipeline`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutomateReport {
    /// Fixed format identifier (`verdant/automate-report/1`).
    pub spec: String,
    /// True iff every automation stage passed or hit a verified kappa cache.
    pub all_passed: bool,
    /// Count of freshly executed passing stages.
    pub passed_stages: u32,
    /// Count of kappa-memoized skipped stages.
    pub skipped_stages: u32,
    /// Total stages in the automation run.
    pub total_stages: u32,
    /// Ordered per-stage execution records.
    pub stages: Vec<RatchetCycleStageRecord>,
    /// Content-addressed SHA-256 hex commitment over this report.
    pub report_kappa_hex: String,
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
    use super::super::*;

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
