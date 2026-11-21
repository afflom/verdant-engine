//! Observed W3C SOSA/PROV-O communication channels, grounded Player<->NPC dialogue turns,
//! Observer holon status serialization, and cognition tests (`observer_comms.rs`).

use serde_json::json;

use super::{
    compute_kappa, sha256_payload_digest, CommunicationChannelKind, NpcDialogueTurnResult,
    ObservedCommunicationRecord, ObserverPrismHolon,
};
use crate::cybernetics::PlayerObserverSample;
use crate::error::ObserverModelError;
use crate::math::{four_u32, pow10_u64, two_u32};
use crate::network::PrismIdSessionEnvelope;
use crate::prism_kernel;

impl ObserverPrismHolon {
    /// Experiences, verifies, and records a communication across any of the Observer's 5 channels
    /// (`Organ<->Organ`, `Player<->Player`, `Player->Agent`, `Agent->Player`, `Agent<->Agent`).
    #[allow(clippy::too_many_arguments)]
    pub fn observe_communication(
        &mut self,
        channel: CommunicationChannelKind,
        sender_id: &str,
        recipient_id: &str,
        language_bcp47: &str,
        dialogue_act_code: u8,
        message_text: &str,
        prism_session: Option<(&PrismIdSessionEnvelope, &str, u64)>,
        attended_slot: usize,
    ) -> Result<ObservedCommunicationRecord, ObserverModelError> {
        let trimmed = message_text.trim();
        if trimmed.is_empty() || sender_id.trim().is_empty() || recipient_id.trim().is_empty() {
            return Err(ObserverModelError::CommunicationEnvelopeRejected {
                channel: channel.slug().to_string(),
                detail: "sender, recipient, and message text must be non-empty".to_string(),
            });
        }

        let msg_digest = sha256_payload_digest(trimmed.as_bytes());
        let prism_sig_valid = if let Some((env, expected_origin, now_sec)) = prism_session {
            env.verify(expected_origin, now_sec, &msg_digest).is_ok()
        } else {
            false
        };

        if channel.requires_prism_signature() && !prism_sig_valid {
            return Err(ObserverModelError::CommunicationEnvelopeRejected {
                channel: channel.slug().to_string(),
                detail:
                    "player communication channel requires a valid fresh PrismIdSessionEnvelope"
                        .to_string(),
            });
        }

        let seq = (self.communications.len() as u64) + 1;
        let action_iri = format!("urn:verdant:comm:{seq}");
        let sender_type = if channel == CommunicationChannelKind::OrganToOrgan {
            "SoftwareApplication"
        } else {
            "Person"
        };
        let recipient_type = if channel == CommunicationChannelKind::OrganToOrgan {
            "SoftwareApplication"
        } else {
            "Person"
        };
        let schema_action_jsonld = json!({
            "@context": "https://schema.org",
            "@graph": [
                { "@id": sender_id, "@type": sender_type, "name": sender_id },
                { "@id": recipient_id, "@type": recipient_type, "name": recipient_id },
                {
                    "@id": action_iri,
                    "@type": "CommunicateAction",
                    "name": format!("{}#{seq}", channel.slug()),
                    "agent": { "@id": sender_id },
                    "recipient": { "@id": recipient_id },
                    "inLanguage": language_bcp47,
                    "about": trimmed
                }
            ]
        });
        self.catalogs
            .schema_org
            .validate_schema_org_jsonld(&schema_action_jsonld)
            .map_err(|err| ObserverModelError::CommunicationEnvelopeRejected {
                channel: channel.slug().to_string(),
                detail: err.to_string(),
            })?;

        let prev_iri = if seq > 1 {
            format!("urn:verdant:comm:{}", seq - 1)
        } else {
            format!("urn:verdant:domain:{}", self.active_domain.spec.id)
        };
        let sanitized_lit: String = trimmed
            .chars()
            .filter(|&c| c != '"' && c != '\\' && c != '\n' && c != '\r')
            .collect();
        let sosa_provo_turtle = format!(
            "@prefix sosa: <http://www.w3.org/ns/sosa/> .\n\
             @prefix prov: <http://www.w3.org/ns/prov#> .\n\
             @prefix schema: <https://schema.org/> .\n\
             <{sender_id}> a prov:Agent ; schema:name \"{sender_id}\" .\n\
             <urn:verdant:activity:comm:{seq}> a prov:Activity ; prov:wasAssociatedWith <{sender_id}> .\n\
             <{prev_iri}> a prov:Entity ; prov:wasAttributedTo <{sender_id}> .\n\
             <{action_iri}> a prov:Entity ; prov:wasDerivedFrom <{prev_iri}> ; prov:wasGeneratedBy <urn:verdant:activity:comm:{seq}> .\n\
             <urn:verdant:obs:comm:{seq}> a sosa:Observation ; sosa:madeBySensor <urn:verdant:sensor:observer-holon> ; sosa:hasFeatureOfInterest <{recipient_id}> ; sosa:hasSimpleResult \"{sanitized_lit}\" .\n"
        );
        self.catalogs
            .sosa_provo
            .validate_sosa_provo_turtle(&sosa_provo_turtle)
            .map_err(|err| ObserverModelError::CommunicationEnvelopeRejected {
                channel: channel.slug().to_string(),
                detail: err.to_string(),
            })?;

        let kernel_env_ok =
            prism_kernel::observedChannelEnvelopeValid(channel.code(), prism_sig_valid, true, true)
                .unwrap_or(false);
        if !kernel_env_ok {
            return Err(ObserverModelError::CommunicationEnvelopeRejected {
                channel: channel.slug().to_string(),
                detail: "rejected by Observer::observedChannelEnvelopeValid".to_string(),
            });
        }

        // Excite the Observer's neuromorphic LIF salience loop so the Observer genuinely experiences the message:
        let hundred = pow10_u64(two_u32()) as u32;
        let sixteen = four_u32() * four_u32();
        let sample = PlayerObserverSample {
            gazed_entity_slot: attended_slot,
            gaze_dwell_millis: hundred * two_u32(),
            camera_angular_speed_milli: hundred,
            locomotion_speed_milli: 0,
            cadence_focus_milli: hundred * (four_u32() * two_u32()),
            audio_alignment_milli: hundred as i32,
            ui_focus_slot: u32::from(dialogue_act_code),
            frame_dt_millis: sixteen,
        };
        let spike_mask = self
            .attention_loop
            .step_observation(sample, 0)
            .map_or(0, |d| d.spike_mask);

        let (_, kappa_hex) = compute_kappa(&action_iri, sosa_provo_turtle.as_bytes());
        let rec = ObservedCommunicationRecord {
            sequence: seq,
            channel,
            sender_id: sender_id.to_string(),
            recipient_id: recipient_id.to_string(),
            language_bcp47: language_bcp47.to_string(),
            dialogue_act_code,
            message_text: trimmed.to_string(),
            prism_signed: prism_sig_valid,
            kappa_hex,
            schema_action_jsonld,
            sosa_provo_turtle,
            observer_spike_mask: spike_mask,
        };
        self.communications.push(rec.clone());
        Ok(rec)
    }

    /// Executes a complete observed `Player -> Agent` and `Agent -> Player` dialogue exchange:
    /// 1. Logs and verifies the signed `PlayerToAgent` communication,
    /// 2. Evaluates grounded Q&A or directive mutation via `VerdantLanguageEngine`,
    /// 3. Applies any directive update to `set_npc_directive` (replanning A* `NavMesh` patrol), and
    /// 4. Logs and verifies the resulting `AgentToPlayer` response (`inform`, `request`, or `"Unknown"` `abstain`).
    pub fn converse_player_with_npc(
        &mut self,
        player_id: &str,
        prism_session: (&PrismIdSessionEnvelope, &str, u64),
        npc_index: usize,
        utterance: &str,
    ) -> Result<NpcDialogueTurnResult, ObserverModelError> {
        let Some(ch_spec) = self.active_domain.spec.characters.get(npc_index) else {
            return Err(ObserverModelError::UnknownDomainOrCharacter {
                identifier: format!("npc:{npc_index}"),
            });
        };
        let npc_iri = format!("urn:verdant:character:{}", ch_spec.id);
        let (lang, p_act) = self
            .language_engine
            .classify_utterance(utterance, &self.catalogs);

        let _ = self.observe_communication(
            CommunicationChannelKind::PlayerToAgent,
            player_id,
            &npc_iri,
            &lang,
            p_act,
            utterance,
            Some(prism_session),
            npc_index,
        )?;

        let turn = self.language_engine.evaluate_npc_dialogue_turn(
            utterance,
            &self.active_domain,
            npc_index,
            &self.catalogs,
        )?;

        if let Some(dir_idx) = turn.mutated_directive_index {
            let _ = self.set_npc_directive(npc_index, dir_idx)?;
        }

        let _ = self.observe_communication(
            CommunicationChannelKind::AgentToPlayer,
            &npc_iri,
            player_id,
            &turn.detected_language,
            turn.response_act_code,
            &turn.reply_text,
            None,
            npc_index,
        )?;

        Ok(turn)
    }

    /// Serializes the Observer holon's live identity, organs, stakeholders, active domain, NPC
    /// directives, and recent observed communications into UTF-8 JSON for Studio and Catalogs.
    #[must_use]
    pub fn to_status_json_bytes(&self) -> Vec<u8> {
        let max_recent = (four_u32() * two_u32()) as usize;
        let comm_start = self.communications.len().saturating_sub(max_recent);
        let recent_comms: Vec<serde_json::Value> = self.communications[comm_start..]
            .iter()
            .map(|c| {
                json!({
                    "sequence": c.sequence,
                    "channel": c.channel.slug(),
                    "sender": c.sender_id,
                    "recipient": c.recipient_id,
                    "lang": c.language_bcp47,
                    "act": c.dialogue_act_code,
                    "text": c.message_text,
                    "prismSigned": c.prism_signed,
                    "kappa": c.kappa_hex,
                    "spikeMask": c.observer_spike_mask
                })
            })
            .collect();

        let organs_json: Vec<serde_json::Value> = self
            .organs
            .iter()
            .map(|o| {
                json!({
                    "slug": o.kind.slug(),
                    "index": o.index,
                    "basisPrime": o.basis_prime,
                    "hostsAllModels": o.hosts_all_models,
                    "kappa": o.kappa_hex
                })
            })
            .collect();

        let stakeholders_json: Vec<serde_json::Value> = self
            .stakeholders
            .iter()
            .map(|s| {
                json!({
                    "index": s.index,
                    "workspaceIndex": s.workspace_index,
                    "workspaceSlug": s.workspace_slug,
                    "basisPrime": s.basis_prime,
                    "organGrantCount": s.allowed_organs.len(),
                    "kappa": s.kappa_hex
                })
            })
            .collect();

        let npcs_json: Vec<serde_json::Value> = self
            .active_domain
            .spec
            .characters
            .iter()
            .enumerate()
            .map(|(idx, ch)| {
                let agent = self.npc_agents.get(idx);
                json!({
                    "index": idx,
                    "id": ch.id,
                    "name": ch.name,
                    "role": ch.role,
                    "rigId": ch.rig_id,
                    "directives": ch.directives,
                    "activeDirectiveIndex": agent.map_or(0, |a| a.active_directive_index),
                    "activeDirectiveLabel": agent.map_or("", |a| a.active_directive_label.as_str()),
                    "corridorWaypoints": agent.map_or(0, |a| a.corridor_waypoints.len()),
                    "distanceWalkedMeters": agent.map_or(0.0, |a| a.distance_walked_meters)
                })
            })
            .collect();

        let doc = json!({
            "observerKappa": self.observer_kappa_hex,
            "organBasisProduct": self.organ_basis_product,
            "emergencyHalted": self.emergency_halted,
            "activeDomain": {
                "id": self.active_domain.spec.id,
                "slug": self.active_domain.spec.slug,
                "title": self.active_domain.spec.title,
                "schemaOrgType": self.active_domain.spec.schema_type,
                "kappa": compute_kappa(&self.active_domain.spec.id, self.active_domain.spec.slug.as_bytes()).1,
                "seed": self.active_domain.spec.seed,
                "defaultLodRank": self.active_domain.spec.default_lod_rank,
                "cameraMode": self.active_domain.spec.camera_mode,
                "conservedSupply": self.active_domain.item_ledger.total_supply(),
                "navPolygons": self.active_navmesh.polygons.len()
            },
            "organs": organs_json,
            "roleCount": self.stakeholders.len(),
            "stakeholders": stakeholders_json,
            "npcs": npcs_json,
            "admittedModelCount": self.admitted_model_kappas.len(),
            "importedProjectModelCount": self.project_models.len(),
            "communicationCount": self.communications.len(),
            "recentCommunications": recent_comms
        });
        serde_json::to_vec(&doc).unwrap_or_default()
    }
}

#[cfg(test)]
mod observer_cognition_tests {
    use std::collections::BTreeSet;

    use super::super::*;
    use crate::cybernetics::ContainerGrantMatrix;
    use crate::network::PrismIdSessionEnvelope;

    #[test]
    fn observer_sensory_streams_have_unique_primes_kappas_and_dmn_lifecycle() {
        let mut holon = ObserverPrismHolon::new("sunspire-regatta").expect("observer init");
        assert_eq!(holon.sensory_streams.len(), 10);

        let primes: BTreeSet<u64> = holon
            .sensory_streams
            .iter()
            .map(|s| s.basis_prime)
            .collect();
        let kappas: BTreeSet<&str> = holon
            .sensory_streams
            .iter()
            .map(|s| s.kappa_hex.as_str())
            .collect();
        assert_eq!(
            primes.len(),
            10,
            "each sensory stream needs a unique basis prime"
        );
        assert_eq!(
            kappas.len(),
            10,
            "each sensory stream needs a unique SHA-256 kappa"
        );

        let grants = ContainerGrantMatrix::default();
        // Negative DMN / URI scheme validation test: non-HTTPS endpoint must be refused!
        let bad = holon.register_stream(
            StreamRegisterRequest {
                stream_id: "insecure-feed".to_string(),
                label: "Insecure HTTP Feed".to_string(),
                authority: "Untrusted".to_string(),
                adapter_kind: "schema-org-catalog".to_string(),
                endpoint_url: "http://insecure.example/feed.json".to_string(),
                target_parts: vec!["hero-props".to_string()],
            },
            &grants,
        );
        assert!(bad.is_err(), "non-https stream endpoint must be refused");

        // Positive admission + toggle + removal lifecycle:
        let added = holon
            .register_stream(
                StreamRegisterRequest {
                    stream_id: "test-uhd-stream".to_string(),
                    label: "Verified UHD glTF Stream".to_string(),
                    authority: "Open 3D Consortium".to_string(),
                    adapter_kind: "khronos-index".to_string(),
                    endpoint_url: "https://example.org/model-index.json".to_string(),
                    target_parts: vec!["character-rigs".to_string(), "hero-props".to_string()],
                },
                &grants,
            )
            .expect("valid https stream must be admitted");
        assert!(added.basis_prime > 47);
        assert_eq!(holon.sensory_streams.len(), 11);

        let paused = holon
            .toggle_stream("test-uhd-stream", false)
            .expect("toggle off");
        assert!(!paused.enabled);

        let removed_kappa = holon
            .remove_stream("test-uhd-stream")
            .expect("remove stream");
        assert_eq!(removed_kappa, added.kappa_hex);
        assert_eq!(holon.sensory_streams.len(), 10);
    }

    #[test]
    fn live_stream_knowledge_ingestion_grounds_iso_24617_2_and_preserves_abstention() {
        let mut holon = ObserverPrismHolon::new("sunspire-regatta").expect("observer init");
        let grants = ContainerGrantMatrix::default();

        let rec = holon
            .ingest_knowledge_passage(KnowledgePassageRequest {
                source_stream: "wikidata".to_string(),
                source_uri: "https://en.wikipedia.org/wiki/Apollo_Lunar_Module".to_string(),
                topic: "Apollo Lunar Module".to_string(),
                passage: "The Apollo Lunar Module was the lunar lander spacecraft flown between lunar orbit and the Moon surface during the Apollo program.".to_string(),
            })
            .expect("knowledge passage ingestion");
        assert_eq!(rec.kappa_hex.len(), 64);

        // Grounded query against freshly ingested stream knowledge must answer without hallucinating:
        let _ = grants;
        let q1 = "What was the Apollo Lunar Module spacecraft?";
        let d1 = sha256_payload_digest(q1.as_bytes());
        let env1 = PrismIdSessionEnvelope::sign_deterministic(
            "prism:test:observer",
            b"observer-test-secret",
            "https://afflom.github.io",
            100,
            200,
            &d1,
        );
        let turn = holon
            .converse_player_with_npc(
                "prism:test:observer",
                (&env1, "https://afflom.github.io", 100),
                0,
                q1,
            )
            .expect("grounded dialogue turn");
        assert!(
            !turn.abstained_unknown,
            "ingested stream passage should ground answer"
        );
        assert!(turn.reply_text.contains("Apollo Lunar Module"));

        // Ungrounded query must still abstain with Unknown (zero hallucination law):
        let q2 = "What is the xylophone price on Neptune?";
        let d2 = sha256_payload_digest(q2.as_bytes());
        let env2 = PrismIdSessionEnvelope::sign_deterministic(
            "prism:test:observer",
            b"observer-test-secret",
            "https://afflom.github.io",
            100,
            200,
            &d2,
        );
        let unknown = holon
            .converse_player_with_npc(
                "prism:test:observer",
                (&env2, "https://afflom.github.io", 100),
                0,
                q2,
            )
            .expect("abstention dialogue turn");
        assert!(unknown.abstained_unknown, "ungrounded query must abstain");
    }

    #[test]
    fn host_capability_envelope_enforces_uhd_4k_drs_and_software_clamping() {
        let mut env = HostCapabilityEnvelope::default_safe();

        // Hardware GPU at UHD 4K (3840x2160) with strong frame headroom (6ms):
        env.update_with_viewport(8192, false, 12, 6000, 120_000, 3840, 2160);
        assert!(env.is_uhd_target);
        assert_eq!(env.uhd_tier_label, "UHD 4K Native");
        assert_eq!(env.recommended_render_scale_permille, 1000);
        assert_eq!(env.recommended_shadow_map_px, 4096);
        assert_eq!(env.recommended_anisotropy, 8);
        assert_eq!(env.recommended_texture_cap_px, 2048);

        // Hardware GPU at UHD 4K under heavier load (11ms) -> engages DRS + CAS without exceeding host:
        env.update_with_viewport(8192, false, 8, 11000, 180_000, 3840, 2160);
        assert_eq!(env.uhd_tier_label, "UHD 4K DRS + CAS");
        assert!(env.recommended_render_scale_permille < 1000);
        assert!(env.recommended_render_scale_permille >= 650);

        // Constrained Software WebGL2 at UHD 4K -> strictly clamps texture/shadow/DRS/GLB cap:
        env.update_with_viewport(8192, true, 12, 14000, 50_000, 3840, 2160);
        assert_eq!(env.uhd_tier_label, "Host-Bounded Software");
        assert_eq!(env.recommended_texture_cap_px, 512);
        assert_eq!(env.recommended_shadow_map_px, 1024);
        assert_eq!(env.recommended_anisotropy, 1);
        assert_eq!(env.recommended_render_scale_permille, 650);
    }
}
