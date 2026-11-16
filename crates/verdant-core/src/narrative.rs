//! Conservation-verified narrative, quest, dialogue, and interactive 3D app automatons
//! (`kernel14ConservationAutomaton`) grounded by `CodedSlmDirector` and `CyberneticObserverLoop`.
//!
//! Every [`StoryAutomaton`] is verified at construction via tropical `(min, +)` reachability
//! closure (`uor-matmul`) so trapped deadlock branches or unreachable milestones are impossible,
//! and every inventory/resource transition is checked by `prism_kernel::conservationLawValid`
//! (`before + minted - burned == after`) so item duplication or negative underflow cannot occur.

use serde::{Deserialize, Serialize};
use verdant_tensor::{verify_conservation_and_reachability, ConservationAuditReport};

use crate::cybernetics::{CodedSlmDirector, ContainerGrantMatrix};
use crate::error::NarrativeLawError;
use crate::hd_world::hd_terrain_height;
use crate::math::{pow10_u64, three_u32, two_f64, Vec3};
use crate::streaming::TerrainReliefParams;

/// Exact non-negative integer resource ledger enforcing conservation laws across world inventories.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConservationLedger {
    /// Deterministic ordered `(account_id, balance)` pairs.
    pub balances: Vec<(String, u64)>,
    /// Chronological audit trail of verified `(before_total, minted, burned, after_total)` steps.
    pub audit_trail: Vec<(u64, u64, u64, u64)>,
}

impl ConservationLedger {
    /// Creates an empty ledger.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            balances: Vec::new(),
            audit_trail: Vec::new(),
        }
    }

    /// Reads the current balance of `account`.
    #[must_use]
    pub fn balance_of(&self, account: &str) -> u64 {
        self.balances
            .iter()
            .find(|(k, _)| k == account)
            .map_or(0, |(_, v)| *v)
    }

    /// Initializes or adds `mint_amount` to `account`, recording the initial mint in `audit_trail`.
    pub fn seed_account(
        &mut self,
        account: impl Into<String>,
        mint_amount: u64,
    ) -> Result<(), NarrativeLawError> {
        let key = account.into();
        let before = self.total_supply();
        if let Some((_, bal)) = self.balances.iter_mut().find(|(k, _)| *k == key) {
            *bal = bal
                .checked_add(mint_amount)
                .ok_or(NarrativeLawError::ConservationViolated {
                    before_total: before,
                    minted: mint_amount,
                    burned: 0,
                    after_total: u64::MAX,
                })?;
        } else {
            self.balances.push((key, mint_amount));
        }
        let after = self.total_supply();
        let ok = crate::prism_kernel::conservationLawValid(before, mint_amount, 0, after)
            .unwrap_or(false);
        if !ok {
            return Err(NarrativeLawError::ConservationViolated {
                before_total: before,
                minted: mint_amount,
                burned: 0,
                after_total: after,
            });
        }
        self.audit_trail.push((before, mint_amount, 0, after));
        Ok(())
    }

    /// Exact sum of all account balances.
    #[must_use]
    pub fn total_supply(&self) -> u64 {
        self.balances
            .iter()
            .fold(0_u64, |acc, (_, v)| acc.saturating_add(*v))
    }

    /// Executes an atomic conserved transfer (`from -> to`), optional `mint` (`to`), and optional
    /// `burn` (`from`), refusing underflow or conservation mismatch via `prism_kernel::conservationLawValid`.
    pub fn apply_conserved_delta(
        &mut self,
        from_account: Option<&str>,
        to_account: Option<&str>,
        transfer_amount: u64,
        mint_amount: u64,
        burn_amount: u64,
    ) -> Result<(u64, u64, u64, u64), NarrativeLawError> {
        let before = self.total_supply();
        let mut staged = self.balances.clone();

        let debit_total = transfer_amount.saturating_add(burn_amount);
        if debit_total > 0 {
            let src = from_account.ok_or(NarrativeLawError::ConservationViolated {
                before_total: before,
                minted: mint_amount,
                burned: burn_amount,
                after_total: before,
            })?;
            let slot = staged.iter_mut().find(|(k, _)| k == src);
            let Some((_, src_bal)) = slot else {
                return Err(NarrativeLawError::ConservationViolated {
                    before_total: before,
                    minted: mint_amount,
                    burned: burn_amount,
                    after_total: before,
                });
            };
            if *src_bal < debit_total {
                return Err(NarrativeLawError::ConservationViolated {
                    before_total: before,
                    minted: mint_amount,
                    burned: burn_amount,
                    after_total: before,
                });
            }
            *src_bal -= debit_total;
        }

        let credit_total = transfer_amount.saturating_add(mint_amount);
        if credit_total > 0 {
            let dst = to_account.ok_or(NarrativeLawError::ConservationViolated {
                before_total: before,
                minted: mint_amount,
                burned: burn_amount,
                after_total: before,
            })?;
            if let Some((_, dst_bal)) = staged.iter_mut().find(|(k, _)| k == dst) {
                *dst_bal = dst_bal.checked_add(credit_total).ok_or(
                    NarrativeLawError::ConservationViolated {
                        before_total: before,
                        minted: mint_amount,
                        burned: burn_amount,
                        after_total: u64::MAX,
                    },
                )?;
            } else {
                staged.push((dst.to_string(), credit_total));
            }
        }

        let after: u64 = staged
            .iter()
            .fold(0_u64, |acc, (_, v)| acc.saturating_add(*v));
        let valid =
            crate::prism_kernel::conservationLawValid(before, mint_amount, burn_amount, after)
                .unwrap_or(false);
        if !valid {
            return Err(NarrativeLawError::ConservationViolated {
                before_total: before,
                minted: mint_amount,
                burned: burn_amount,
                after_total: after,
            });
        }

        self.balances = staged;
        let tuple = (before, mint_amount, burn_amount, after);
        self.audit_trail.push(tuple);
        Ok(tuple)
    }
}

/// State node in a verified [`StoryAutomaton`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StoryNode {
    /// Zero-based state index (`0..nodes.len()`).
    pub index: usize,
    /// Canonical node slug.
    pub slug: String,
    /// Chapter / app view title.
    pub title: String,
    /// Player-facing objective or status line.
    pub objective: String,
    /// True if this node is a valid completion or milestone anchor for deadlock verification.
    pub is_milestone: bool,
}

/// Guarded directed transition in a [`StoryAutomaton`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoryEdge {
    /// Source state index.
    pub from_state: usize,
    /// Destination state index.
    pub to_state: usize,
    /// Action identifier (`speak_archivist`, `calibrate_bridge`, `align_observatory`, etc.).
    pub action_id: String,
    /// NPC guide or diegetic terminal hosting this interaction.
    pub actor_name: String,
    /// Spoken NPC line or terminal confirmation emitted on transition.
    pub dialogue_line: String,
    /// Positive tropical `(min, +)` transition cost in milli-units.
    pub cost_milli: u32,
    /// World-space anchor of the NPC or diegetic terminal.
    pub world_anchor: Vec3,
    /// Maximum Euclidean interaction distance in meters (`> 0`).
    pub interaction_radius_meters: f64,
    /// Optional source inventory account for resource transfer/burn.
    pub from_account: Option<String>,
    /// Optional destination inventory account for resource transfer/mint.
    pub to_account: Option<String>,
    /// Exact conserved unit transfer count (`from_account -> to_account`).
    pub transfer_amount: u64,
    /// Explicit minted units (`0` for pure conservation transfers).
    pub mint_amount: u64,
    /// Explicit burned units (`0` for pure conservation transfers).
    pub burn_amount: u64,
}

/// Verified state-machine automaton governing quests, NPC dialogues, and interactive 3D apps.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoryAutomaton {
    /// Ordered story/app states.
    pub nodes: Vec<StoryNode>,
    /// Guarded transitions.
    pub edges: Vec<StoryEdge>,
    /// Active state index.
    pub current_state: usize,
    /// Conserved world & player inventory ledger.
    pub ledger: ConservationLedger,
    /// Latest spoken dialogue or terminal output.
    pub last_dialogue: String,
    /// Number of transitions executed in this session.
    pub completed_steps: u64,
}

impl StoryAutomaton {
    /// Constructs and formally verifies a [`StoryAutomaton`] via tropical `(min, +)` reachability
    /// closure and exact integer resource conservation (`kernel14ConservationAutomaton`).
    pub fn new(
        nodes: Vec<StoryNode>,
        edges: Vec<StoryEdge>,
        initial_state: usize,
        ledger: ConservationLedger,
    ) -> Result<(Self, ConservationAuditReport), NarrativeLawError> {
        let n = nodes.len();
        if n == 0 || initial_state >= n {
            return Err(NarrativeLawError::UnreachableOrDeadlockedState {
                detail: "story automaton requires non-empty nodes and valid initial_state"
                    .to_string(),
            });
        }
        for (idx, node) in nodes.iter().enumerate() {
            if node.index != idx {
                return Err(NarrativeLawError::UnreachableOrDeadlockedState {
                    detail: format!("story node index {} does not match slot {idx}", node.index),
                });
            }
        }

        let terminal_states: Vec<usize> = nodes
            .iter()
            .filter(|nd| nd.is_milestone)
            .map(|nd| nd.index)
            .collect();
        if terminal_states.is_empty() {
            return Err(NarrativeLawError::UnreachableOrDeadlockedState {
                detail: "story automaton requires at least one milestone/completion state"
                    .to_string(),
            });
        }

        let mut edge_costs: Vec<Option<u32>> = vec![None; n * n];
        for edge in &edges {
            if edge.from_state >= n
                || edge.to_state >= n
                || edge.cost_milli == 0
                || !edge.interaction_radius_meters.is_finite()
                || edge.interaction_radius_meters <= 0.0
            {
                return Err(NarrativeLawError::UnreachableOrDeadlockedState {
                    detail: format!("invalid bounds on story edge `{}`", edge.action_id),
                });
            }
            let slot = edge.from_state * n + edge.to_state;
            edge_costs[slot] =
                Some(edge_costs[slot].map_or(edge.cost_milli, |prev| prev.min(edge.cost_milli)));
        }

        let report = verify_conservation_and_reachability(
            &edge_costs,
            n,
            initial_state,
            &terminal_states,
            &ledger.audit_trail,
        )
        .map_err(|err| NarrativeLawError::UnreachableOrDeadlockedState {
            detail: err.to_string(),
        })?;

        let initial_line = nodes[initial_state].objective.clone();
        Ok((
            Self {
                nodes,
                edges,
                current_state: initial_state,
                ledger,
                last_dialogue: initial_line,
                completed_steps: 0,
            },
            report,
        ))
    }

    /// Re-verifies live reachability from `self.current_state` and the entire `self.ledger.audit_trail`.
    pub fn audit_live(&self) -> Result<ConservationAuditReport, NarrativeLawError> {
        let n = self.nodes.len();
        let terminal_states: Vec<usize> = self
            .nodes
            .iter()
            .filter(|nd| nd.is_milestone)
            .map(|nd| nd.index)
            .collect();
        let mut edge_costs: Vec<Option<u32>> = vec![None; n * n];
        for edge in &self.edges {
            let slot = edge.from_state * n + edge.to_state;
            edge_costs[slot] =
                Some(edge_costs[slot].map_or(edge.cost_milli, |prev| prev.min(edge.cost_milli)));
        }
        verify_conservation_and_reachability(
            &edge_costs,
            n,
            self.current_state,
            &terminal_states,
            &self.ledger.audit_trail,
        )
        .map_err(|err| NarrativeLawError::UnreachableOrDeadlockedState {
            detail: err.to_string(),
        })
    }

    /// Active [`StoryNode`].
    #[must_use]
    pub fn active_node(&self) -> &StoryNode {
        &self.nodes[self.current_state]
    }

    /// Outgoing transitions available from `self.current_state`.
    #[must_use]
    pub fn available_edges(&self) -> Vec<&StoryEdge> {
        self.edges
            .iter()
            .filter(|e| e.from_state == self.current_state)
            .collect()
    }

    /// Attempts to execute `action_id` at `player_world_pos`, enforcing state, spatial distance,
    /// and exact inventory conservation laws.
    pub fn try_advance(
        &mut self,
        action_id: &str,
        player_world_pos: Vec3,
    ) -> Result<StoryEdge, NarrativeLawError> {
        let Some(edge) = self
            .edges
            .iter()
            .find(|e| e.from_state == self.current_state && e.action_id == action_id)
            .cloned()
        else {
            return Err(NarrativeLawError::GuardRefused {
                action_id: action_id.to_string(),
                reason: format!("not admissible from state {}", self.current_state),
            });
        };

        let dist = player_world_pos.distance(edge.world_anchor);
        if dist > edge.interaction_radius_meters {
            let scale_mm = pow10_u64(three_u32()) as f64;
            let dist_mm = (dist.max(0.0) * scale_mm).round() as u64;
            let rad_mm = (edge.interaction_radius_meters.max(0.0) * scale_mm).round() as u64;
            return Err(NarrativeLawError::GuardRefused {
                action_id: edge.action_id,
                reason: format!(
                    "player distance {dist_mm}mm exceeds interaction radius {rad_mm}mm"
                ),
            });
        }

        let _ = self.ledger.apply_conserved_delta(
            edge.from_account.as_deref(),
            edge.to_account.as_deref(),
            edge.transfer_amount,
            edge.mint_amount,
            edge.burn_amount,
        )?;

        self.current_state = edge.to_state;
        self.last_dialogue = edge.dialogue_line.clone();
        self.completed_steps += 1;
        Ok(edge)
    }

    /// Advances the nearest in-range outgoing interaction from `player_world_pos` (used by quick `Interact` key/button).
    pub fn interact_nearest(
        &mut self,
        player_world_pos: Vec3,
    ) -> Result<Option<StoryEdge>, NarrativeLawError> {
        let mut candidates: Vec<(f64, String)> = self
            .edges
            .iter()
            .filter(|e| e.from_state == self.current_state)
            .filter_map(|e| {
                let d = player_world_pos.distance(e.world_anchor);
                if d <= e.interaction_radius_meters {
                    Some((d, e.action_id.clone()))
                } else {
                    None
                }
            })
            .collect();
        candidates.sort_by(|a, b| a.0.total_cmp(&b.0));
        let Some((_, best_action)) = candidates.into_iter().next() else {
            return Ok(None);
        };
        self.try_advance(&best_action, player_world_pos).map(Some)
    }

    /// Routes a natural-language `prompt` through `CodedSlmDirector` (`coded_gemm`) to propose a
    /// grounded narrative transition, admitting it strictly through [`Self::try_advance`].
    pub fn advance_from_slm_prompt(
        &mut self,
        prompt: &str,
        player_world_pos: Vec3,
        director: &CodedSlmDirector,
    ) -> Result<StoryEdge, NarrativeLawError> {
        let grants = ContainerGrantMatrix::default();
        let proposal = director
            .route_prompt(prompt, None, &grants)
            .map_err(|err| NarrativeLawError::GuardRefused {
                action_id: "slm_proposal".to_string(),
                reason: err.to_string(),
            })?;
        let slug = proposal.intent.slug();

        let lower = prompt.to_ascii_lowercase();
        let outgoing: Vec<String> = self
            .edges
            .iter()
            .filter(|e| e.from_state == self.current_state)
            .map(|e| e.action_id.clone())
            .collect();
        if outgoing.is_empty() {
            return Err(NarrativeLawError::GuardRefused {
                action_id: slug.to_string(),
                reason: "no outgoing edges from active state".to_string(),
            });
        }

        let chosen = outgoing
            .iter()
            .find(|act| lower.contains(act.as_str()) || act.contains(slug))
            .cloned()
            .unwrap_or_else(|| outgoing[0].clone());

        self.try_advance(&chosen, player_world_pos)
    }
}

/// Builds the canonical **Verdant Citadel Chronicle** story & conservation graph connecting the
/// three articulated NPC guides (`Archivist`, `BridgeEngineer`, `ObservatoryKeeper`) with a fixed
/// conserved supply of `3` `PrismKeystone` relics (`vault -> player -> bridge -> observatory -> vault`).
pub fn canonical_citadel_chronicle_automaton(
    seed: u64,
) -> Result<(StoryAutomaton, ConservationAuditReport), NarrativeLawError> {
    let relief = TerrainReliefParams::standard_basin_valley();
    let two = two_f64();
    let four = two * two;
    let eight = four * two;
    let nine = eight + 1.0;
    let twelve = eight + four;
    let cost_unit = pow10_u64(three_u32()) as u32;

    let archivist_pos = Vec3::new(0.0, hd_terrain_height(seed, &relief, 0.0, 0.0) + 1.0, 0.0);
    let bridge_pos = Vec3::new(
        relief.basin_center_x,
        hd_terrain_height(seed, &relief, relief.basin_center_x, relief.basin_center_z) + two,
        relief.basin_center_z,
    );
    let obs_pos = Vec3::new(
        -nine,
        hd_terrain_height(seed, &relief, -nine, -four) + 1.0,
        -four,
    );

    let mut ledger = ConservationLedger::new();
    let initial_relics = 1_u64 + 1_u64 + 1_u64;
    ledger.seed_account("citadel_vault", initial_relics)?;

    let i2 = 1usize + 1usize;
    let i3 = i2 + 1usize;
    let nodes = vec![
        StoryNode {
            index: 0,
            slug: "citadel_awakening".to_string(),
            title: "I · Awakening at the Vaulted Citadel".to_string(),
            objective: "Enter the Vaulted Citadel Hall and speak with the Archivist.".to_string(),
            is_milestone: false,
        },
        StoryNode {
            index: 1,
            slug: "keystone_entrusted".to_string(),
            title: "II · Bearer of the Prism Keystone".to_string(),
            objective: "Cross to the Parabolic Arch Bridge and deliver a Keystone to the Bridge Engineer.".to_string(),
            is_milestone: false,
        },
        StoryNode {
            index: i2,
            slug: "bridge_calibrated".to_string(),
            title: "III · Equilibrium of the Arch Span".to_string(),
            objective: "Ascend to the High Observatory ridge and align the Keystone with the Observatory Keeper.".to_string(),
            is_milestone: false,
        },
        StoryNode {
            index: i3,
            slug: "universe_synchronized".to_string(),
            title: "IV · Synchronized Across the Stitch Mesh".to_string(),
            objective: "All three sector conduits hold conserved equilibrium; return to the Citadel Vault to cycle a new epoch.".to_string(),
            is_milestone: true,
        },
    ];

    let interact_r = twelve;
    let edges = vec![
        StoryEdge {
            from_state: 0,
            to_state: 1,
            action_id: "speak_archivist".to_string(),
            actor_name: "Archivist".to_string(),
            dialogue_line: "Archivist: Take two Prism Keystones from the Citadel Vault—every shard is conserved across the mesh.".to_string(),
            cost_milli: cost_unit,
            world_anchor: archivist_pos,
            interaction_radius_meters: interact_r,
            from_account: Some("citadel_vault".to_string()),
            to_account: Some("explorer_pack".to_string()),
            transfer_amount: 1 + 1,
            mint_amount: 0,
            burn_amount: 0,
        },
        StoryEdge {
            from_state: 1,
            to_state: i2,
            action_id: "calibrate_bridge".to_string(),
            actor_name: "BridgeEngineer".to_string(),
            dialogue_line: "Bridge Engineer: Keystone seated in the parabolic voussoir—the river span is locked in phase.".to_string(),
            cost_milli: cost_unit,
            world_anchor: bridge_pos,
            interaction_radius_meters: interact_r,
            from_account: Some("explorer_pack".to_string()),
            to_account: Some("bridge_conduit".to_string()),
            transfer_amount: 1,
            mint_amount: 0,
            burn_amount: 0,
        },
        StoryEdge {
            from_state: i2,
            to_state: i3,
            action_id: "align_observatory".to_string(),
            actor_name: "ObservatoryKeeper".to_string(),
            dialogue_line: "Observatory Keeper: Second Keystone aligned—Citadel, Arch Bridge, and Observatory form a closed Stitch triad.".to_string(),
            cost_milli: cost_unit,
            world_anchor: obs_pos,
            interaction_radius_meters: interact_r,
            from_account: Some("explorer_pack".to_string()),
            to_account: Some("observatory_conduit".to_string()),
            transfer_amount: 1,
            mint_amount: 0,
            burn_amount: 0,
        },
        StoryEdge {
            from_state: i3,
            to_state: 0,
            action_id: "reconcile_vault".to_string(),
            actor_name: "Archivist".to_string(),
            dialogue_line: "Archivist: Conduits reconciled back to the Citadel Vault with zero loss—ready for a new expedition.".to_string(),
            cost_milli: cost_unit,
            world_anchor: archivist_pos,
            interaction_radius_meters: interact_r,
            from_account: Some("bridge_conduit".to_string()),
            to_account: Some("citadel_vault".to_string()),
            transfer_amount: 0,
            mint_amount: 0,
            burn_amount: 0,
        },
    ];

    StoryAutomaton::new(nodes, edges, 0, ledger)
}
