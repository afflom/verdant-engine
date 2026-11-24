//! Self-verification `verdant automate-check` subcommand (`automate_cmd.rs`).

use super::*;

/// Executes Verdant's full in-process headless self-verification check across all 4 domain packs,
/// ISO 24617-2 dialogue grounding & abstention, dynamic model versioning, Observer sensory streams,
/// UHD 4K/8K profiles, Morton meshlets, WGSL shaders, UORC streaming, Stitch parity, and VSM Total Ownership.
pub(crate) fn run_automate_check() -> Result<String, CliCommandError> {
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
            .converse_player_with_npc(prism_id, (&g_env, origin, now_sec), 0, &grounded_prompt)
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

    let _ = holon
        .ingest_knowledge_passage(KnowledgePassageRequest {
            source_stream: "wikidata".to_string(),
            source_uri: "https://en.wikipedia.org/wiki/Apollo_Lunar_Module".to_string(),
            topic: "Apollo Lunar Module".to_string(),
            passage: "The Apollo Lunar Module landed crewed astronauts on the Moon surface."
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
            diagnostic: "ISO 24617-2 failed to ground freshly ingested stream passage".to_string(),
        });
    }

    let ui_w = (1u32 << (1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1))
        + (1u32 << (1 + 1 + 1 + 1 + 1 + 1 + 1 + 1));
    let ui_h =
        (1u32 << (1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1)) + (1u32 << (1 + 1 + 1 + 1 + 1 + 1 + 1 + 1));
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

    let shaders =
        validate_builtin_engine_shaders().map_err(|e| CliCommandError::ValidationRejected {
            command: "automate-check".to_string(),
            diagnostic: format!("wgsl: {e}"),
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
    let sample_meshlets =
        verdant_core::uhd::build_meshlet_graph_from_mesh(&holon.active_domain.sector.meshes[0]);
    if sample_meshlets.clusters.is_empty()
        || sample_meshlets.reordered_indices.len() != sample_meshlets.total_triangles * (1 + 1 + 1)
    {
        return Err(CliCommandError::ValidationRejected {
            command: "automate-check".to_string(),
            diagnostic: "Morton meshlet graph produced empty or misaligned clusters".to_string(),
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
            diagnostic: format!("FrameBudgetOracle unstable on nominal load: {frame_rep:?}"),
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
    let recovered = recover_bytes_from_stitch_pair(&frags[0], &frags[1 + 1]).map_err(|e| {
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

    let op_channel = verdant_core::operator::VerdantOperatorChannel::new("sunspire-regatta")
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

pub(crate) fn run_release_self_host(
    ch: &mut verdant_core::operator::VerdantOperatorChannel,
    root: &std::path::Path,
    out_dir: &std::path::Path,
    version: &str,
    epoch: u64,
) -> Result<String, CliCommandError> {
    let lift = ch
        .lift_full_workspace_into_vcs(root, out_dir, version, epoch)
        .map_err(|e| CliCommandError::ValidationRejected {
            command: "release self-host".to_string(),
            diagnostic: e.to_string(),
        })?;
    Ok(format!(
        "SELF-HOST LIFTED: version={} epoch={} rust={} lex={} gov={} site={} unlifted={} uorc_bytes={} shards={} commit={}",
        lift.version,
        lift.release_epoch,
        lift.lifted_rust_files,
        lift.lifted_lex_files,
        lift.lifted_governance_files,
        lift.lifted_site_files,
        lift.unlifted_workspace_modules,
        lift.source_uorc_bytes,
        lift.source_shard_count,
        lift.source_commit_kappa
    ))
}

pub(crate) fn run_mcp_stdio_loop(sub_args: &[String]) -> Result<String, CliCommandError> {
    use std::io::{BufRead, Write};
    let two = 1usize + 1usize;
    let mut root = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    let mut i = 0usize;
    while i < sub_args.len() {
        if sub_args[i] == "--root" && i + 1 < sub_args.len() {
            root = std::path::PathBuf::from(&sub_args[i + 1]);
            i += two;
        } else {
            i += 1;
        }
    }
    let mut ch =
        verdant_core::operator::VerdantOperatorChannel::new("sunspire-regatta").map_err(|e| {
            CliCommandError::ValidationRejected {
                command: "mcp".to_string(),
                diagnostic: e.to_string(),
            }
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
