//! `verdant operator ...` CLI subcommand dispatcher (`operator_cmd.rs`), extracted from `lib.rs`.

use super::*;

pub(crate) fn run_operator_command(args: &[String]) -> Result<String, CliCommandError> {
    let sub = args.get(1).map(String::as_str).unwrap_or("status");
    let mut ch =
        verdant_core::operator::VerdantOperatorChannel::new("sunspire-regatta").map_err(|e| {
            CliCommandError::ValidationRejected {
                command: "operator".to_string(),
                diagnostic: e.to_string(),
            }
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
        "lifecycle-gate" => {
            let two = 1usize + 1usize;
            let mut root =
                std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
            let mut run_workspace_tests = false;
            let mut run_browser_gates = false;
            let mut write_report = false;
            let mut expected_tag: Option<String> = None;
            let mut i = two;
            while i < args.len() {
                match args[i].as_str() {
                    "--root" if i + 1 < args.len() => {
                        root = std::path::PathBuf::from(&args[i + 1]);
                        i += two;
                    }
                    "--tag" if i + 1 < args.len() => {
                        expected_tag = Some(args[i + 1].clone());
                        i += two;
                    }
                    "--full" => {
                        run_workspace_tests = true;
                        i += 1;
                    }
                    "--with-browser" => {
                        run_workspace_tests = true;
                        run_browser_gates = true;
                        i += 1;
                    }
                    "--in-process" => {
                        run_workspace_tests = false;
                        run_browser_gates = false;
                        i += 1;
                    }
                    "--write" => {
                        write_report = true;
                        i += 1;
                    }
                    _ => i += 1,
                }
            }
            let opts = verdant_core::operator::LifecycleGateOptions {
                run_workspace_tests,
                run_browser_gates,
                expected_tag,
                write_report,
            };
            let (rep, json) = ch.execute_lifecycle_gate(&root, &opts).map_err(|e| {
                CliCommandError::ValidationRejected {
                    command: "operator lifecycle-gate".to_string(),
                    diagnostic: e.to_string(),
                }
            })?;
            if !rep.release_qualified {
                return Err(CliCommandError::ValidationRejected {
                    command: "operator lifecycle-gate".to_string(),
                    diagnostic: format!(
                        "lifecycle gate REFUSED release ({}/{} passed):\n{json}",
                        rep.passed_count, rep.total_count
                    ),
                });
            }
            Ok(json)
        }
        "refine-plan" => {
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
            let (_agenda, json) =
                ch.synthesize_refinement_agenda(&root).map_err(|e| {
                    CliCommandError::ValidationRejected {
                        command: "operator refine-plan".to_string(),
                        diagnostic: e.to_string(),
                    }
                })?;
            Ok(json)
        }
        "sync-wasm" => {
            let two = 1usize + 1usize;
            let mut root =
                std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
            let mut check_only = false;
            let mut repack_stream = false;
            let mut i = two;
            while i < args.len() {
                match args[i].as_str() {
                    "--root" if i + 1 < args.len() => {
                        root = std::path::PathBuf::from(&args[i + 1]);
                        i += two;
                    }
                    "--check" => {
                        check_only = true;
                        i += 1;
                    }
                    "--repack" => {
                        repack_stream = true;
                        i += 1;
                    }
                    _ => i += 1,
                }
            }
            let op = verdant_core::operator::NativeBuildOp::SyncWasm {
                check_only,
                repack_stream,
            };
            let (rep, _) = ch.execute_native_build_op(&root, &op).map_err(|e| {
                CliCommandError::ValidationRejected {
                    command: "operator sync-wasm".to_string(),
                    diagnostic: e.to_string(),
                }
            })?;
            Ok(rep.summary)
        }
        "package-crates" => {
            let two = 1usize + 1usize;
            let mut root =
                std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
            let mut out_dir = std::path::PathBuf::from("dist/crates");
            let mut dl_template = "https://github.com/afflom/verdant-engine/releases/download/v{version}/{crate}-{version}.crate".to_string();
            let mut i = two;
            while i < args.len() {
                match args[i].as_str() {
                    "--root" if i + 1 < args.len() => {
                        root = std::path::PathBuf::from(&args[i + 1]);
                        i += two;
                    }
                    "--out" if i + 1 < args.len() => {
                        out_dir = std::path::PathBuf::from(&args[i + 1]);
                        i += two;
                    }
                    "--dl" if i + 1 < args.len() => {
                        dl_template = args[i + 1].clone();
                        i += two;
                    }
                    _ => i += 1,
                }
            }
            let op = verdant_core::operator::NativeBuildOp::PackageRustCrates {
                out_dir,
                dl_template,
            };
            let (rep, _) = ch.execute_native_build_op(&root, &op).map_err(|e| {
                CliCommandError::ValidationRejected {
                    command: "operator package-crates".to_string(),
                    diagnostic: e.to_string(),
                }
            })?;
            Ok(rep.summary)
        }
        "verify-registry" => {
            let two = 1usize + 1usize;
            let mut root =
                std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
            let mut crates_root = std::path::PathBuf::from("site/crates");
            let mut i = two;
            while i < args.len() {
                match args[i].as_str() {
                    "--root" if i + 1 < args.len() => {
                        root = std::path::PathBuf::from(&args[i + 1]);
                        i += two;
                    }
                    "--dir" if i + 1 < args.len() => {
                        crates_root = std::path::PathBuf::from(&args[i + 1]);
                        i += two;
                    }
                    _ => i += 1,
                }
            }
            let op = verdant_core::operator::NativeBuildOp::VerifySparseRegistry { crates_root };
            let (rep, _) = ch.execute_native_build_op(&root, &op).map_err(|e| {
                CliCommandError::ValidationRejected {
                    command: "operator verify-registry".to_string(),
                    diagnostic: e.to_string(),
                }
            })?;
            Ok(rep.summary)
        }
        "automate" => {
            let two = 1usize + 1usize;
            let mut root =
                std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
            let mut skip_rust = false;
            let mut force_wasm = false;
            let mut i = two;
            while i < args.len() {
                match args[i].as_str() {
                    "--root" if i + 1 < args.len() => {
                        root = std::path::PathBuf::from(&args[i + 1]);
                        i += two;
                    }
                    "--skip-rust" => {
                        skip_rust = true;
                        i += 1;
                    }
                    "--force-wasm" => {
                        force_wasm = true;
                        i += 1;
                    }
                    _ => i += 1,
                }
            }
            let opts = verdant_core::operator::AutomateOptions {
                skip_rust,
                force_wasm,
            };
            let (_rep, json) = ch.execute_automate_pipeline(&root, &opts).map_err(|e| {
                CliCommandError::ValidationRejected {
                    command: "operator automate".to_string(),
                    diagnostic: e.to_string(),
                }
            })?;
            Ok(json)
        }
        "ratchet-cycle" => {
            let two = 1usize + 1usize;
            let mut root =
                std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
            let mut force = false;
            let mut tighten = true;
            let mut run_sdk_docker = true;
            let mut run_lifecycle_gate = false;
            let mut run_browser_gates = false;
            let mut adv_major = false;
            let mut adv_minor = false;
            let mut adv_patch = false;
            let mut summary = "Autonomous Verdant lifted ratchet-cycle".to_string();
            let mut i = two;
            while i < args.len() {
                match args[i].as_str() {
                    "--root" if i + 1 < args.len() => {
                        root = std::path::PathBuf::from(&args[i + 1]);
                        i += two;
                    }
                    "--force" => {
                        force = true;
                        i += 1;
                    }
                    "--no-tighten" => {
                        tighten = false;
                        i += 1;
                    }
                    "--no-docker" => {
                        run_sdk_docker = false;
                        i += 1;
                    }
                    "--with-gate" => {
                        run_lifecycle_gate = true;
                        i += 1;
                    }
                    "--with-browser" => {
                        run_lifecycle_gate = true;
                        run_browser_gates = true;
                        i += 1;
                    }
                    "--advance-major" => {
                        adv_major = true;
                        i += 1;
                    }
                    "--advance-minor" => {
                        adv_minor = true;
                        i += 1;
                    }
                    "--advance-patch" => {
                        adv_patch = true;
                        i += 1;
                    }
                    "--summary" if i + 1 < args.len() => {
                        summary = args[i + 1].clone();
                        i += two;
                    }
                    _ => i += 1,
                }
            }
            let advance_signals = if adv_major || adv_minor || adv_patch {
                Some(verdant_core::governance::ObserverProgressSignals {
                    breaking_contract_change: adv_major,
                    major_milestone_completed: adv_major,
                    minor_capability_added: adv_minor,
                    stakeholder_requirement_closed: adv_minor,
                    defect_or_security_fix: adv_patch,
                    summary,
                })
            } else {
                None
            };
            let opts = verdant_core::operator::RatchetCycleOptions {
                force,
                tighten,
                run_sdk_docker,
                run_lifecycle_gate,
                run_browser_gates,
                advance_signals,
            };
            let (_rep, json) = ch.execute_ratchet_cycle(&root, &opts).map_err(|e| {
                CliCommandError::ValidationRejected {
                    command: "operator ratchet-cycle".to_string(),
                    diagnostic: e.to_string(),
                }
            })?;
            Ok(json)
        }
        "ratchet-lift" => {
            let two = 1usize + 1usize;
            let mut root =
                std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
            let mut out_dir = std::path::PathBuf::from("dist/stitch-release");
            let mut write_repo_artifacts = true;
            let mut update_frame_pin = true;
            let mut tighten_ratchet = true;
            let mut stitch_relays: Vec<String> = Vec::new();
            let mut adv_major = false;
            let mut adv_minor = false;
            let mut adv_patch = false;
            let mut summary = "Autonomous Verdant ratchet self-lift".to_string();
            let mut i = two;
            while i < args.len() {
                match args[i].as_str() {
                    "--root" if i + 1 < args.len() => {
                        root = std::path::PathBuf::from(&args[i + 1]);
                        i += two;
                    }
                    "--out" if i + 1 < args.len() => {
                        out_dir = std::path::PathBuf::from(&args[i + 1]);
                        i += two;
                    }
                    "--relay" if i + 1 < args.len() => {
                        stitch_relays.push(args[i + 1].clone());
                        i += two;
                    }
                    "--no-update-frame-pin" => {
                        update_frame_pin = false;
                        i += 1;
                    }
                    "--no-tighten" => {
                        tighten_ratchet = false;
                        i += 1;
                    }
                    "--isolated" => {
                        write_repo_artifacts = false;
                        update_frame_pin = false;
                        tighten_ratchet = false;
                        i += 1;
                    }
                    "--advance-major" => {
                        adv_major = true;
                        i += 1;
                    }
                    "--advance-minor" => {
                        adv_minor = true;
                        i += 1;
                    }
                    "--advance-patch" => {
                        adv_patch = true;
                        i += 1;
                    }
                    "--summary" if i + 1 < args.len() => {
                        summary = args[i + 1].clone();
                        i += two;
                    }
                    _ => i += 1,
                }
            }
            let advance_signals = if adv_major || adv_minor || adv_patch {
                Some(verdant_core::governance::ObserverProgressSignals {
                    breaking_contract_change: adv_major,
                    major_milestone_completed: adv_major,
                    minor_capability_added: adv_minor,
                    stakeholder_requirement_closed: adv_minor,
                    defect_or_security_fix: adv_patch,
                    summary,
                })
            } else {
                None
            };
            let opts = verdant_core::operator::AutonomousRatchetLiftOptions {
                write_repo_artifacts,
                update_frame_pin,
                tighten_ratchet,
                stitch_relays,
                advance_signals,
            };
            let rep = ch
                .execute_autonomous_ratchet_lift(&root, &out_dir, &opts)
                .map_err(|e| CliCommandError::ValidationRejected {
                    command: "operator ratchet-lift".to_string(),
                    diagnostic: e.to_string(),
                })?;
            Ok(format!(
                "AUTONOMOUS RATCHET LIFT OK: version=v{} epoch={} incremented={} lifted_modules={} unlifted_ws={} unlifted_ui={} blank_frame_bytes={} frame_pin_updated={} tightened={} capsule={} manifest={}",
                rep.version,
                rep.release_epoch,
                rep.version_incremented,
                rep.lifted_module_count,
                rep.unlifted_workspace_modules,
                rep.unlifted_ui_files,
                rep.blank_frame_bytes,
                rep.frame_pin_updated,
                rep.tightened_ratchet_fields.len(),
                rep.release_capsule_sha256,
                rep.manifest_path
            ))
        }
        "authority" => {
            let two = 1usize + 1usize;
            let mut root =
                std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
            let mut ledger_path = ".verdant/authority-ledger.json".to_string();
            let mut init_owner: Option<String> = None;
            let mut owner_secret = String::new();
            let mut repo_name = "verdant-engine".to_string();
            let mut cmd_json = r#"{"cmd":"verifyChain"}"#.to_string();
            let mut i = two;
            while i < args.len() {
                match args[i].as_str() {
                    "--root" if i + 1 < args.len() => {
                        root = std::path::PathBuf::from(&args[i + 1]);
                        i += two;
                    }
                    "--ledger" if i + 1 < args.len() => {
                        ledger_path = args[i + 1].clone();
                        i += two;
                    }
                    "--init-owner" if i + 1 < args.len() => {
                        init_owner = Some(args[i + 1].clone());
                        i += two;
                    }
                    "--owner-secret" if i + 1 < args.len() => {
                        owner_secret = args[i + 1].clone();
                        i += two;
                    }
                    "--repo-name" if i + 1 < args.len() => {
                        repo_name = args[i + 1].clone();
                        i += two;
                    }
                    "--cmd-json" if i + 1 < args.len() => {
                        cmd_json = args[i + 1].clone();
                        i += two;
                    }
                    _ => i += 1,
                }
            }
            let esc = |s: &str| s.replace('\\', "\\\\").replace('"', "\\\"");
            let rpc = if let Some(owner) = init_owner {
                format!(
                    r#"{{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{{"name":"verdant_authority_exec","arguments":{{"ledgerPath":"{}","initRepoName":"{}","initOwnerPrismId":"{}","initOwnerSecret":"{}"}}}}}}"#,
                    esc(&ledger_path),
                    esc(&repo_name),
                    esc(&owner),
                    esc(&owner_secret)
                )
            } else {
                format!(
                    r#"{{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{{"name":"verdant_authority_exec","arguments":{{"ledgerPath":"{}","command":{}}}}}}}"#,
                    esc(&ledger_path),
                    cmd_json
                )
            };
            let out = ch.handle_mcp_jsonrpc_line(&root, &rpc).ok_or_else(|| {
                CliCommandError::ValidationRejected {
                    command: "operator authority".to_string(),
                    diagnostic: "empty authority MCP response".to_string(),
                }
            })?;
            if out.contains(r#""isError":true"#) {
                return Err(CliCommandError::ValidationRejected {
                    command: "operator authority".to_string(),
                    diagnostic: out,
                });
            }
            Ok(out)
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
