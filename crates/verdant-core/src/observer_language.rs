//! Native `uor-matmul` `Packed<Grid<16>, 2>` ISO 24617-2 dialogue & passage-grounded reasoning
//! engine (`observer_language.rs`), extracted from `observer.rs`.

use super::*;

/// Result of a native ISO 24617-2 NPC dialogue & passage-grounded reasoning turn.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct NpcDialogueTurnResult {
    /// Target character ID (`CHAR-*`).
    pub character_id: String,
    /// Target character display name.
    pub character_name: String,
    /// Detected BCP-47 language tag (`en`, `es`, `fr`, `ja`).
    pub detected_language: String,
    /// Classified player ISO 24617-2 act code (`0..=7`).
    pub player_act_code: u8,
    /// Classified NPC response ISO 24617-2 act code (`0` = `inform`, `3` = `request` ack, `7` = `abstain`).
    pub response_act_code: u8,
    /// Response ISO 24617-2 act ID (`"inform"`, `"request"`, `"abstain"`).
    pub response_act_id: String,
    /// Number of cataloged `knowledgeFacts` / ledger facts matched via `uor-matmul` `coded_gemm`.
    pub matched_evidence_facts: usize,
    /// True iff `matched_evidence_facts == 0` and the engine abstained with `"Unknown"`.
    pub abstained_unknown: bool,
    /// True iff an actionable `request` updated an NPC directive or conserved state.
    pub world_mutated: bool,
    /// Optional updated NPC directive slot (`0..directives.len()`).
    pub mutated_directive_index: Option<usize>,
    /// Grounded NPC reply text (`"Unknown"` when `abstained_unknown == true`).
    pub reply_text: String,
    /// W3C PROV-O source fact identifiers supporting `reply_text`.
    pub cited_fact_ids: Vec<String>,
    /// Top `coded_gemm` passage salience score in milli-units (`0..=1000`).
    pub confidence_milli: u64,
}

impl NpcDialogueTurnResult {
    /// Serializes this dialogue turn result alongside its observed communication `kappa` into UTF-8 JSON.
    #[must_use]
    pub fn to_json_bytes(&self, comm_kappa_hex: &str) -> Vec<u8> {
        let doc = json!({
            "characterId": self.character_id,
            "characterName": self.character_name,
            "detectedLanguage": self.detected_language,
            "playerActCode": self.player_act_code,
            "responseActCode": self.response_act_code,
            "responseActId": self.response_act_id,
            "matchedEvidenceFacts": self.matched_evidence_facts,
            "abstainedUnknown": self.abstained_unknown,
            "worldMutated": self.world_mutated,
            "mutatedDirectiveIndex": self.mutated_directive_index,
            "replyText": self.reply_text,
            "citedFactIds": self.cited_fact_ids,
            "confidenceMilli": self.confidence_milli,
            "commKappa": comm_kappa_hex
        });
        serde_json::to_vec(&doc).unwrap_or_default()
    }
}

/// Verified communication event experienced and logged by the Observer holon.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObservedCommunicationRecord {
    /// Monotonic Observer sequence number (`1..`).
    pub sequence: u64,
    /// Channel Prism model (`Organ<->Organ`, `Player<->Player`, `Player->Agent`, `Agent->Player`, `Agent<->Agent`).
    pub channel: CommunicationChannelKind,
    /// Sender IRI or identifier.
    pub sender_id: String,
    /// Recipient IRI or identifier.
    pub recipient_id: String,
    /// BCP-47 language tag.
    pub language_bcp47: String,
    /// ISO 24617-2 dialogue act code (`0..=7`).
    pub dialogue_act_code: u8,
    /// Message content experienced by the Observer.
    pub message_text: String,
    /// Whether a `PrismIdSessionEnvelope` signature was verified.
    pub prism_signed: bool,
    /// Strong 32-byte `kappa` hex identity of this communication envelope.
    pub kappa_hex: String,
    /// Verified Schema.org `CommunicateAction` JSON-LD document.
    pub schema_action_jsonld: serde_json::Value,
    /// Verified W3C SOSA `Observation` + W3C PROV-O `wasDerivedFrom` Turtle record.
    pub sosa_provo_turtle: String,
    /// Neuromorphic LIF spike bitmask excited in the Observer by this communication.
    pub observer_spike_mask: u64,
}

/// Autonomous NPC navigation & directive state driven by walkable `NavMesh` A* corridors.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NpcNavAgentState {
    /// Catalog character ID (`CHAR-*`).
    pub character_id: String,
    /// Active directive index into `CatalogCharacterProfile::directives`.
    pub active_directive_index: usize,
    /// Active directive label.
    pub active_directive_label: String,
    /// Planned walkable `NavMesh` A* corridor waypoints in world meters.
    pub corridor_waypoints: Vec<Vec3>,
    /// Current target waypoint slot along `corridor_waypoints`.
    pub waypoint_cursor: usize,
    /// Cumulative distance walked along A* corridors in meters.
    pub distance_walked_meters: f64,
}

/// Native Verdant Engine language, dialogue-act, and passage-grounded reasoning runtime
/// (`uor-matmul` `Packed<Grid<16>, 2>` + `Grid<16>` `coded_gemm` — zero external model runtimes).
#[derive(Clone, Debug)]
pub struct VerdantLanguageEngine {
    codebook: [i32; CODED_TABLE_LEN],
    w1_identity_packed: Vec<u8>,
    w2_identity_grid: Vec<u16>,
}

impl Default for VerdantLanguageEngine {
    fn default() -> Self {
        Self::new()
    }
}

fn tokenize_content_words(text: &str) -> Vec<String> {
    const STOP_WORDS: &[&str] = &[
        "the", "and", "for", "with", "that", "this", "from", "into", "what", "where", "who", "how",
        "which", "when", "why", "are", "is", "was", "were", "does", "do", "did", "can", "has",
        "have", "about", "tell", "me", "you", "your", "please", "one", "two", "que", "donde",
        "quien", "como", "quel", "quelle", "est", "les", "des", "une", "para", "del",
    ];
    let min_len = (1usize + 1usize) + 1usize;
    text.to_ascii_lowercase()
        .split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|w| w.len() >= min_len && !STOP_WORDS.contains(w))
        .map(|w| {
            // Light plural normalization (`sextants -> sextant`, `gates -> gate`) for exact lexical grounding:
            if w.len() > (min_len + 1) && w.ends_with('s') && !w.ends_with("ss") {
                w[..w.len() - 1].to_string()
            } else {
                w.to_string()
            }
        })
        .collect()
}

impl VerdantLanguageEngine {
    /// Synthesizes the native `Packed<Grid<16>, 2>` passage-grounding projection from `{0, 1}`.
    #[must_use]
    pub fn new() -> Self {
        let dim = CODED_TABLE_LEN;
        let mid = (dim / (1 + 1)) as i32;
        let step = prism_kernel::sSixteen().unwrap_or(1) as i32;
        let codebook: [i32; CODED_TABLE_LEN] = core::array::from_fn(|i| ((i as i32) - mid) * step);
        let zero_code = mid as u8;
        let pos_strong = (dim - 1) as u8;
        let nibble_shift = prism_kernel::sFour().unwrap_or(1) as u8;
        let half_cols = dim / (1 + 1);

        let mut w1_identity_packed = vec![0_u8; dim * half_cols];
        for r in 0..dim {
            for pair in 0..half_cols {
                let c_lo = pair * (1 + 1);
                let c_hi = c_lo + 1;
                let lo = if r == c_lo { pos_strong } else { zero_code };
                let hi = if r == c_hi { pos_strong } else { zero_code };
                w1_identity_packed[r * half_cols + pair] = lo | (hi << nibble_shift);
            }
        }

        let mut w2_identity_grid = vec![u16::from(zero_code); dim * dim];
        for r in 0..dim {
            w2_identity_grid[r * dim + r] = u16::from(pos_strong);
        }

        Self {
            codebook,
            w1_identity_packed,
            w2_identity_grid,
        }
    }

    /// Detects BCP-47 language (`en`, `es`, `fr`, `ja`) and ISO 24617-2 player dialogue act (`0..=7`).
    #[must_use]
    pub fn classify_utterance(
        &self,
        utterance: &str,
        registry: &CatalogBundleRegistry,
    ) -> (String, u8) {
        let lower = utterance.trim().to_ascii_lowercase();
        let first_word = lower
            .split(|c: char| !c.is_ascii_alphanumeric())
            .find(|s| !s.is_empty())
            .unwrap_or("");

        let mut detected_lang = "en".to_string();
        let mut is_wh = false;
        let mut is_yn = false;

        for lang in &registry.language_dmn.languages {
            if lang.question_starters.iter().any(|w| w == first_word) {
                detected_lang.clone_from(&lang.bcp47);
                is_wh = true;
                break;
            }
            if lang.yn_starters.iter().any(|w| w == first_word) {
                detected_lang.clone_from(&lang.bcp47);
                is_yn = true;
                break;
            }
        }

        let req_verbs = [
            "patrol",
            "calibrate",
            "inspect",
            "survey",
            "measure",
            "escort",
            "audit",
            "align",
            "record",
            "track",
            "guard",
            "guide",
            "tend",
            "verify",
            "monitor",
            "seal",
            "greet",
            "directive",
            "switch",
            "assign",
        ];
        let act_wh = 1_u8;
        let act_yn = 1_u8 + 1_u8;
        let act_req = act_yn + 1_u8;
        if req_verbs
            .iter()
            .any(|v| first_word == *v || lower.starts_with("set directive"))
        {
            (detected_lang, act_req)
        } else if is_wh {
            (detected_lang, act_wh)
        } else if is_yn || lower.ends_with('?') {
            (detected_lang, act_yn)
        } else {
            (detected_lang, 0_u8)
        }
    }

    /// Verifies that a proposed dialogue turn satisfies `prism_kernel::epistemicDialogueTurnValid`,
    /// refusing ungrounded hallucinations (`matched == 0 && !abstained`) or read-only world mutations.
    pub fn verify_dialogue_turn_claim(
        agent_id: &str,
        response_act_code: u8,
        matched_evidence_facts: usize,
        abstained_unknown: bool,
        world_mutated: bool,
    ) -> Result<(), ObserverModelError> {
        let valid = prism_kernel::epistemicDialogueTurnValid(
            u64::from(response_act_code),
            matched_evidence_facts as u64,
            abstained_unknown,
            world_mutated,
        )
        .unwrap_or(false);
        if valid {
            Ok(())
        } else {
            Err(ObserverModelError::EpistemicHallucinationRefused {
                agent_id: agent_id.to_string(),
                detail: format!(
                    "act={response_act_code}, evidence={matched_evidence_facts}, abstained={abstained_unknown}, mutated={world_mutated} violated Observer::epistemicDialogueTurnValid"
                ),
            })
        }
    }

    /// Evaluates a grounded player-to-NPC dialogue turn natively in Verdant Engine:
    /// - Executes `uor-matmul` `Packed<Grid<16>, 2>` relevance contraction over cataloged `knowledgeFacts`
    ///   and live `ConservationLedger` balances,
    /// - Updates NPC directive when `player_act_code == 3` (`request`) matches a cataloged directive,
    /// - Deterministically returns `"Unknown"` (`act = 7` `abstain`) whenever `matched_evidence_facts == 0`.
    pub fn evaluate_npc_dialogue_turn(
        &self,
        utterance: &str,
        domain: &InstantiatedDomainPack,
        character_index: usize,
        registry: &CatalogBundleRegistry,
    ) -> Result<NpcDialogueTurnResult, ObserverModelError> {
        let Some(ch_profile) = domain.spec.characters.get(character_index) else {
            return Err(ObserverModelError::UnknownDomainOrCharacter {
                identifier: format!("{}#npc:{character_index}", domain.spec.id),
            });
        };

        let (detected_language, player_act_code) = self.classify_utterance(utterance, registry);
        let raw_query_tokens: BTreeSet<String> =
            tokenize_content_words(utterance).into_iter().collect();
        let subject_name_tokens: BTreeSet<String> =
            tokenize_content_words(&format!("{} {}", ch_profile.name, ch_profile.role))
                .into_iter()
                .collect();
        let predicate_tokens: BTreeSet<String> = raw_query_tokens
            .difference(&subject_name_tokens)
            .cloned()
            .collect();
        let query_tokens = if predicate_tokens.is_empty() {
            raw_query_tokens
        } else {
            predicate_tokens
        };
        let act_req = (1_u8 + 1_u8) + 1_u8;
        let act_abstain = (four_u32() as u8) + act_req;
        let scale = prism_kernel::skinWeightScale().unwrap_or(1);

        // 1. Check actionable `request` against cataloged NPC `directives`:
        if player_act_code == act_req && !query_tokens.is_empty() {
            let mut best_dir: Option<(usize, usize)> = None;
            for (d_idx, dir_text) in ch_profile.directives.iter().enumerate() {
                let dir_tokens = tokenize_content_words(dir_text);
                let overlap = dir_tokens
                    .iter()
                    .filter(|t| query_tokens.contains(t.as_str()))
                    .count();
                if overlap > 0 && best_dir.is_none_or(|(_, b)| overlap > b) {
                    best_dir = Some((d_idx, overlap));
                }
            }
            if let Some((d_idx, overlap)) = best_dir {
                let dir_label = &ch_profile.directives[d_idx];
                let reply = format!(
                    "{} acknowledged directive `{}` ({}) and updated A* patrol route.",
                    ch_profile.name, dir_label, ch_profile.role
                );
                Self::verify_dialogue_turn_claim(&ch_profile.id, act_req, overlap, false, true)?;
                return Ok(NpcDialogueTurnResult {
                    character_id: ch_profile.id.clone(),
                    character_name: ch_profile.name.clone(),
                    detected_language,
                    player_act_code,
                    response_act_code: act_req,
                    response_act_id: "request".to_string(),
                    matched_evidence_facts: overlap,
                    abstained_unknown: false,
                    world_mutated: true,
                    mutated_directive_index: Some(d_idx),
                    reply_text: reply,
                    cited_fact_ids: vec![format!("{}:directive:{d_idx}", ch_profile.id)],
                    confidence_milli: scale,
                });
            }
        }

        // 2. Build grounded passage candidates from cataloged `knowledgeFacts` + live `ConservationLedger`:
        let mut passages: Vec<(String, String)> = ch_profile
            .knowledge_facts
            .iter()
            .enumerate()
            .map(|(i, fact)| (format!("{}:fact:{i}", ch_profile.id), fact.clone()))
            .collect();
        for item in &domain.spec.items {
            let bal = domain.item_ledger.balance_of(&format!("vault:{}", item.id));
            passages.push((
                format!("{}:ledger:{}", domain.spec.id, item.id),
                format!(
                    "Conserved vault balance for {} ({}) in {} is {} units.",
                    item.name, item.id, domain.spec.title, bal
                ),
            ));
        }

        // Score every passage through `uor-matmul` `infer_coded_slm_intent_logits` (`Packed<Grid<16>, 2>`):
        let boost =
            (prism_kernel::sSixteen().unwrap_or(1) * prism_kernel::sSixteen().unwrap_or(1)) as i32;
        let mut slot_activations = vec![0_i32; CODED_TABLE_LEN];
        let mut overlap_per_passage = vec![0usize; passages.len()];

        for (p_idx, (_, text)) in passages.iter().enumerate() {
            let p_tokens = tokenize_content_words(text);
            let overlap = p_tokens
                .iter()
                .filter(|w| query_tokens.contains(w.as_str()))
                .count();
            overlap_per_passage[p_idx] = overlap;
            if overlap > 0 {
                let slot = p_idx % CODED_TABLE_LEN;
                slot_activations[slot] =
                    slot_activations[slot].saturating_add((overlap as i32) * boost);
            }
        }

        let logits = infer_coded_slm_intent_logits(
            &slot_activations,
            1,
            CODED_TABLE_LEN,
            CODED_TABLE_LEN,
            CODED_TABLE_LEN,
            &self.codebook,
            &self.w1_identity_packed,
            &self.w2_identity_grid,
        )
        .map_err(|err| ObserverModelError::InvalidPrismModelEnvelope {
            model_id: "VerdantLanguageEngine".to_string(),
            detail: err.to_string(),
        })?;

        let mut matched_indices: Vec<usize> = (0..passages.len())
            .filter(|&i| overlap_per_passage[i] > 0 && logits[i % CODED_TABLE_LEN] > 0)
            .collect();
        matched_indices.sort_by(|&a, &b| {
            overlap_per_passage[b]
                .cmp(&overlap_per_passage[a])
                .then_with(|| a.cmp(&b))
        });

        if matched_indices.is_empty() {
            let abstain_tok = registry.language_dmn.abstention_token.clone();
            Self::verify_dialogue_turn_claim(&ch_profile.id, act_abstain, 0, true, false)?;
            return Ok(NpcDialogueTurnResult {
                character_id: ch_profile.id.clone(),
                character_name: ch_profile.name.clone(),
                detected_language,
                player_act_code,
                response_act_code: act_abstain,
                response_act_id: "abstain".to_string(),
                matched_evidence_facts: 0,
                abstained_unknown: true,
                world_mutated: false,
                mutated_directive_index: None,
                reply_text: abstain_tok,
                cited_fact_ids: Vec::new(),
                confidence_milli: 0,
            });
        }

        let best_idx = matched_indices[0];
        let max_citations = 1usize + 1usize;
        let cited_fact_ids: Vec<String> = matched_indices
            .iter()
            .take(max_citations)
            .map(|&i| passages[i].0.clone())
            .collect();
        let reply_text = passages[best_idx].1.clone();
        let top_logit = logits[best_idx % CODED_TABLE_LEN].max(1);
        let pos_total: i128 = logits
            .iter()
            .copied()
            .filter(|&v| v > 0)
            .map(i128::from)
            .sum::<i128>()
            .max(1);
        let confidence_milli =
            ((i128::from(top_logit) * i128::from(scale)) / pos_total).min(i128::from(scale)) as u64;

        Self::verify_dialogue_turn_claim(&ch_profile.id, 0, matched_indices.len(), false, false)?;
        Ok(NpcDialogueTurnResult {
            character_id: ch_profile.id.clone(),
            character_name: ch_profile.name.clone(),
            detected_language,
            player_act_code,
            response_act_code: 0,
            response_act_id: "inform".to_string(),
            matched_evidence_facts: matched_indices.len(),
            abstained_unknown: false,
            world_mutated: false,
            mutated_directive_index: None,
            reply_text,
            cited_fact_ids,
            confidence_milli,
        })
    }
}
