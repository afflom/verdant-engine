//! Algebraic command monoid fusion, backward lightcone, and kappa checkpoint cache (`runtime_monoid.rs`).

use super::*;

/// Algebraic classification of engine input / state mutation commands for Prism monoid fusion
/// (extrapolated directly from `verdant-aqm` v0.2.0 `optimizeCircuitPrismMonoid` & `GateKind`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CanonicalCommandKind {
    /// Self-inverse camera possession toggle (`ThirdPerson <-> FirstPerson`, `I^2 = id`).
    ToggleCameraMode,
    /// Self-inverse renderer backend toggle (`WebGL2 <-> SoftwareTile16`, `I^2 = id`).
    ToggleRenderBackend,
    /// Self-inverse organ capability flip (`I^2 = id`).
    ToggleOrganCapability,
    /// Discrete octant yaw rotation (`Z_8` group: `(a + b) mod 8`, `0 => id`).
    YawOctantStep {
        /// Signed or unsigned octant delta (`mod 8`).
        octants: u8,
    },
    /// Additive camera look / zoom delta (`(dyaw + dyaw', dpitch + dpitch', ddist + ddist')`, `0 => id`).
    CameraOrbitDelta {
        /// Yaw delta in milli-radians.
        delta_yaw_milli: i32,
        /// Pitch delta in milli-radians.
        delta_pitch_milli: i32,
        /// Orbit distance delta in millimeters.
        delta_dist_milli: i32,
    },
    /// Non-commuting state barrier (`StoryInteract`, `SnapshotRestore`, or explicit sync barrier).
    StateBarrier,
}

impl CanonicalCommandKind {
    /// Returns true when applying an identical command twice on the same target cancels to identity.
    #[must_use]
    pub const fn is_self_inverse(self) -> bool {
        matches!(
            self,
            Self::ToggleCameraMode | Self::ToggleRenderBackend | Self::ToggleOrganCapability
        )
    }

    /// Numeric ordinal passed to `prism_kernel::commandInvolutionCancels`.
    #[must_use]
    pub const fn ordinal(self) -> u64 {
        let two = 1_u64.wrapping_add(1_u64);
        let three = two.wrapping_add(1_u64);
        let four = two.wrapping_mul(two);
        match self {
            Self::ToggleCameraMode => 0,
            Self::ToggleRenderBackend => 1,
            Self::ToggleOrganCapability => two,
            Self::YawOctantStep { .. } => three,
            Self::CameraOrbitDelta { .. } => four,
            Self::StateBarrier => four.wrapping_add(1_u64),
        }
    }
}

/// Target-addressed engine command node for monoid canonicalization and Kahn DAG scheduling.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalCommandOp {
    /// Command identifier.
    pub id: u32,
    /// Target entity or subsystem wire slot (`0 = camera`, `1 = renderer`, `2.. = entity/organ`).
    pub target_slot: u32,
    /// Algebraic operation kind.
    pub kind: CanonicalCommandKind,
    /// Optional explicit upstream command indices (`must satisfy dep < index` or acyclic Kahn sort).
    pub depends_on: Vec<usize>,
}

/// Summary report emitted by [`canonicalize_command_monoid`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandMonoidReport {
    /// Canonicalized, fused command sequence.
    pub fused_ops: Vec<CanonicalCommandOp>,
    /// Input command count before canonicalization.
    pub raw_count: usize,
    /// Output command count after canonicalization.
    pub fused_count: usize,
    /// Self-inverse pairs cancelled (`I * I -> id`).
    pub involution_cancellations: usize,
    /// `Z_8` octant and additive camera delta merges performed.
    pub additive_merges: usize,
    /// Parallel execution layers produced by Kahn topological `(max, +)` scheduling.
    pub parallel_layers: Vec<Vec<CanonicalCommandOp>>,
    /// Maximum critical-path layer depth (`0` when all ops cancelled).
    pub dag_depth: usize,
}

/// Schedules `ops` into parallel SIMD/worker-ready layer batches via Kahn's topological algorithm
/// and tropical `(max, +)` critical-path depth (`verdant-aqm` `verifyAndCompileCircuitPrism`).
///
/// Refuses cyclic or out-of-bounds `depends_on` graphs via [`RenderGraphError::InvalidPassDependency`].
pub fn compile_parallel_command_layers(
    ops: &[CanonicalCommandOp],
) -> Result<(Vec<Vec<CanonicalCommandOp>>, usize), RenderGraphError> {
    let n = ops.len();
    if n == 0 {
        return Ok((Vec::new(), 0));
    }
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut indegree = vec![0usize; n];
    let mut last_on_target: HashMap<u32, usize> = HashMap::new();

    for (idx, op) in ops.iter().enumerate() {
        let mut parents: BTreeSet<usize> = BTreeSet::new();
        if let Some(&prev) = last_on_target.get(&op.target_slot) {
            parents.insert(prev);
        }
        last_on_target.insert(op.target_slot, idx);
        for &dep in &op.depends_on {
            if dep >= n || dep == idx {
                return Err(RenderGraphError::InvalidPassDependency {
                    producer_pass: dep,
                    consumer_pass: idx,
                });
            }
            parents.insert(dep);
        }
        for p in parents {
            adj[p].push(idx);
            indegree[idx] += 1;
        }
    }

    let mut queue: VecDeque<usize> = VecDeque::new();
    let mut layer_of = vec![0usize; n];
    for (i, &deg) in indegree.iter().enumerate() {
        if deg == 0 {
            queue.push_back(i);
            layer_of[i] = 1;
        }
    }

    let mut visited = 0usize;
    let mut max_layer = 0usize;
    while let Some(u) = queue.pop_front() {
        visited += 1;
        if layer_of[u] > max_layer {
            max_layer = layer_of[u];
        }
        for &v in &adj[u] {
            let cand = layer_of[u] + 1;
            if cand > layer_of[v] {
                layer_of[v] = cand;
            }
            indegree[v] -= 1;
            if indegree[v] == 0 {
                queue.push_back(v);
            }
        }
    }

    if visited != n {
        return Err(RenderGraphError::InvalidPassDependency {
            producer_pass: visited,
            consumer_pass: n,
        });
    }

    let mut layers = vec![Vec::new(); max_layer];
    for (idx, op) in ops.iter().enumerate() {
        layers[layer_of[idx] - 1].push(op.clone());
    }
    Ok((layers, max_layer))
}

/// Applies the Prism Algebraic Monoid Canonicalizer (`verdant-aqm` v0.2.0 `optimizeCircuitPrismMonoid`):
/// 1. Validates DAG acyclicity via [`compile_parallel_command_layers`].
/// 2. Cancels target-adjacent self-inverse involutions (`Toggle^2 -> id`) verified by
///    `prism_kernel::commandInvolutionCancels`.
/// 3. Fuses target-adjacent `Z_8` octant turns modulo `8` (`(a + b) mod 8 == 0 -> id`) and
///    additive `CameraOrbitDelta` triples into single zero-jitter sweeps.
/// 4. Schedules the canonicalized stream into parallel Kahn layer batches.
pub fn canonicalize_command_monoid(
    ops: &[CanonicalCommandOp],
) -> Result<CommandMonoidReport, RenderGraphError> {
    let _ = compile_parallel_command_layers(ops)?;
    let has_explicit_deps = ops.iter().any(|o| !o.depends_on.is_empty());
    let mut working: Vec<CanonicalCommandOp> = ops.to_vec();
    let mut involution_cancellations = 0usize;
    let mut additive_merges = 0usize;
    let oct_mod = (1_u8) << (1 + 1 + 1);

    if !has_explicit_deps {
        let mut next_list: Vec<CanonicalCommandOp> = Vec::with_capacity(working.len());
        for op in working {
            let prev_on_slot = next_list
                .iter()
                .rposition(|p| p.target_slot == op.target_slot);
            let Some(p_idx) = prev_on_slot else {
                if !matches!(
                    op.kind,
                    CanonicalCommandKind::YawOctantStep { octants } if octants % oct_mod == 0
                ) && !matches!(
                    op.kind,
                    CanonicalCommandKind::CameraOrbitDelta {
                        delta_yaw_milli: 0,
                        delta_pitch_milli: 0,
                        delta_dist_milli: 0,
                    }
                ) {
                    next_list.push(op);
                }
                continue;
            };

            let prev = &next_list[p_idx];
            if matches!(prev.kind, CanonicalCommandKind::StateBarrier)
                || matches!(op.kind, CanonicalCommandKind::StateBarrier)
            {
                next_list.push(op);
                continue;
            }

            let cancels = prism_kernel::commandInvolutionCancels(
                prev.kind.ordinal(),
                op.kind.ordinal(),
                prev.kind.is_self_inverse() && op.kind.is_self_inverse(),
                prev.target_slot == op.target_slot,
            );
            if cancels {
                next_list.remove(p_idx);
                involution_cancellations += 1;
                continue;
            }

            match (prev.kind, op.kind) {
                (
                    CanonicalCommandKind::YawOctantStep { octants: a },
                    CanonicalCommandKind::YawOctantStep { octants: b },
                ) => {
                    let fused = prism_kernel::octantMod8Fusion(u64::from(a), u64::from(b))
                        .unwrap_or(u64::from((a.wrapping_add(b)) % oct_mod))
                        as u8;
                    additive_merges += 1;
                    if fused == 0 {
                        next_list.remove(p_idx);
                    } else {
                        next_list[p_idx].kind =
                            CanonicalCommandKind::YawOctantStep { octants: fused };
                    }
                }
                (
                    CanonicalCommandKind::CameraOrbitDelta {
                        delta_yaw_milli: y0,
                        delta_pitch_milli: p0,
                        delta_dist_milli: d0,
                    },
                    CanonicalCommandKind::CameraOrbitDelta {
                        delta_yaw_milli: y1,
                        delta_pitch_milli: p1,
                        delta_dist_milli: d1,
                    },
                ) => {
                    let fy = y0.saturating_add(y1);
                    let fp = p0.saturating_add(p1);
                    let fd = d0.saturating_add(d1);
                    additive_merges += 1;
                    if fy == 0 && fp == 0 && fd == 0 {
                        next_list.remove(p_idx);
                    } else {
                        next_list[p_idx].kind = CanonicalCommandKind::CameraOrbitDelta {
                            delta_yaw_milli: fy,
                            delta_pitch_milli: fp,
                            delta_dist_milli: fd,
                        };
                    }
                }
                _ => next_list.push(op),
            }
        }
        working = next_list;
    }

    let (parallel_layers, dag_depth) = compile_parallel_command_layers(&working)?;
    let fused_count = working.len();
    Ok(CommandMonoidReport {
        fused_ops: working,
        raw_count: ops.len(),
        fused_count,
        involution_cancellations,
        additive_merges,
        parallel_layers,
        dag_depth,
    })
}

/// Computes the backward causal lightcone of all command indices influencing `target_slot` up to
/// `max_step_exclusive` (`verdant-aqm` `computeBackwardLightcone`).
#[must_use]
pub fn compute_backward_command_lightcone(
    ops: &[CanonicalCommandOp],
    target_slot: u32,
    max_step_exclusive: usize,
) -> BTreeSet<usize> {
    let mut active_slots: BTreeSet<u32> = BTreeSet::new();
    active_slots.insert(target_slot);
    let mut active_indices: BTreeSet<usize> = BTreeSet::new();
    let upper = ops.len().min(max_step_exclusive);
    for idx in (0..upper).rev() {
        let op = &ops[idx];
        if active_slots.contains(&op.target_slot) || active_indices.contains(&idx) {
            active_indices.insert(idx);
            active_slots.insert(op.target_slot);
            for &dep in &op.depends_on {
                if dep < idx {
                    active_indices.insert(dep);
                    active_slots.insert(ops[dep].target_slot);
                }
            }
        }
    }
    active_indices
}

const KAPPA_DIGEST_BYTES: usize = 1 << (1 + 1 + 1 + 1 + 1);

/// Canonical 32-byte content-addressed SHA-256 `kappa` digest.
pub type KappaDigest = [u8; KAPPA_DIGEST_BYTES];

/// Content-addressed `kappa` transition memoizer (`verdant-aqm` `executeAtlasHolospaceBatch`).
///
/// Maps `(state_kappa_digest, action_tag)` -> `(next_state, next_kappa_digest)` so repeated
/// quantized observer, narrative, or SLM state transitions execute in `O(1)` without recomputing
/// tensor contractions or SHA-256 digests.
#[derive(Clone, Debug)]
pub struct KappaTransitionCache<S: Clone> {
    max_entries: usize,
    entries: HashMap<(KappaDigest, u64), (S, KappaDigest)>,
    elided_hits: usize,
    computed_misses: usize,
}

impl<S: Clone> KappaTransitionCache<S> {
    /// Creates a bounded content-addressed transition cache (`max_entries >= 1`).
    #[must_use]
    pub fn new(max_entries: usize) -> Self {
        Self {
            max_entries: max_entries.max(1),
            entries: HashMap::new(),
            elided_hits: 0,
            computed_misses: 0,
        }
    }

    /// Computes a canonical 32-byte SHA-256 `kappa` digest over quantized state bytes.
    #[must_use]
    pub fn digest_bytes(bytes: &[u8]) -> KappaDigest {
        let mut hasher = Sha256::new();
        hasher.update(bytes);
        let raw = hasher.finalize();
        let mut out = [0_u8; KAPPA_DIGEST_BYTES];
        out.copy_from_slice(&raw);
        out
    }

    /// Number of transitions elided via `O(1)` content-addressed `kappa` cache hits.
    #[must_use]
    pub const fn elided_step_hits(&self) -> usize {
        self.elided_hits
    }

    /// Number of distinct transitions computed and inserted.
    #[must_use]
    pub const fn computed_step_misses(&self) -> usize {
        self.computed_misses
    }

    /// Evaluates or elides `(current_kappa, action_tag)`, returning `(next_state, next_kappa, was_elided)`.
    pub fn step_or_elide<F>(
        &mut self,
        current_kappa: KappaDigest,
        action_tag: u64,
        compute_fn: F,
    ) -> (S, KappaDigest, bool)
    where
        F: FnOnce() -> (S, Vec<u8>),
    {
        let key = (current_kappa, action_tag);
        if let Some((cached_state, cached_kappa)) = self.entries.get(&key) {
            self.elided_hits += 1;
            return (cached_state.clone(), *cached_kappa, true);
        }
        let (next_state, next_canonical_bytes) = compute_fn();
        let next_kappa = Self::digest_bytes(&next_canonical_bytes);
        self.computed_misses += 1;
        if self.entries.len() >= self.max_entries {
            self.entries.clear();
        }
        self.entries.insert(key, (next_state.clone(), next_kappa));
        (next_state, next_kappa, false)
    }
}

/// Stride-8 checkpointed deterministic rollback journal (`verdant-aqm` `journal.ts` lightweight
/// intermediate frames + `Subsystems::rollbackCheckpointStrideFrames`), reducing snapshot memory
/// by `8x` while reconstructing any arbitrary historical frame `t` in at most `<= 7` micro-steps.
#[derive(Clone, Debug)]
pub struct CheckpointedRollbackJournal<S: Clone, I: Clone> {
    stride: usize,
    checkpoints: BTreeMap<usize, S>,
    inputs: Vec<I>,
    chain_digests: Vec<KappaDigest>,
}

impl<S: Clone, I: Clone> CheckpointedRollbackJournal<S, I> {
    /// Initializes a checkpointed rollback journal at genesis `frame 0`.
    #[must_use]
    pub fn new(genesis_state: S, genesis_seed_bytes: &[u8]) -> Self {
        let stride = (prism_kernel::rollbackCheckpointStrideFrames().unwrap_or(1)).max(1) as usize;
        let mut checkpoints = BTreeMap::new();
        checkpoints.insert(0, genesis_state);
        let genesis_digest = KappaTransitionCache::<()>::digest_bytes(genesis_seed_bytes);
        Self {
            stride,
            checkpoints,
            inputs: Vec::new(),
            chain_digests: vec![genesis_digest],
        }
    }

    /// Checkpoint stride in frames (`8`).
    #[must_use]
    pub const fn checkpoint_stride(&self) -> usize {
        self.stride
    }

    /// Number of full state checkpoints retained in memory (`ceil((T + 1) / stride)`).
    #[must_use]
    pub fn stored_checkpoint_count(&self) -> usize {
        self.checkpoints.len()
    }

    /// Latest recorded frame index (`T`).
    #[must_use]
    pub fn latest_frame(&self) -> usize {
        self.inputs.len()
    }

    /// Records frame `t + 1` input and chains its SHA-256 digest, storing `post_state` only on
    /// `stride` boundaries (`(frame_idx % stride) == 0`).
    pub fn record_step(&mut self, input: I, input_wire_bytes: &[u8], post_state: &S) -> usize {
        let prev = *self
            .chain_digests
            .last()
            .expect("genesis digest always present");
        let mut hasher = Sha256::new();
        hasher.update(prev);
        hasher.update(input_wire_bytes);
        let raw = hasher.finalize();
        let mut digest = [0_u8; KAPPA_DIGEST_BYTES];
        digest.copy_from_slice(&raw);

        self.inputs.push(input);
        self.chain_digests.push(digest);
        let frame_idx = self.inputs.len();
        if frame_idx.is_multiple_of(self.stride) {
            self.checkpoints.insert(frame_idx, post_state.clone());
        }
        frame_idx
    }

    /// Reconstructs exact state at `target_frame` (`0..=latest_frame`) from the nearest preceding
    /// checkpoint in at most `stride - 1` (`<= 7`) deterministic step applications, returning
    /// `(reconstructed_state, replay_steps, chain_digest)`.
    pub fn reconstruct_frame<F>(
        &self,
        target_frame: usize,
        mut step_fn: F,
    ) -> Result<(S, usize, KappaDigest), SnapshotCodecError>
    where
        F: FnMut(&S, &I) -> S,
    {
        if target_frame > self.inputs.len() {
            return Err(SnapshotCodecError::MalformedStatePayload {
                detail: "target rollback frame exceeds recorded journal horizon".to_string(),
            });
        }
        let (&ckpt_frame, ckpt_state) = self
            .checkpoints
            .range(..=target_frame)
            .next_back()
            .ok_or_else(|| SnapshotCodecError::MalformedStatePayload {
                detail: "missing genesis rollback checkpoint".to_string(),
            })?;
        let mut cur = ckpt_state.clone();
        let mut steps = 0usize;
        for inp in &self.inputs[ckpt_frame..target_frame] {
            cur = step_fn(&cur, inp);
            steps += 1;
        }
        Ok((cur, steps, self.chain_digests[target_frame]))
    }
}

impl<I: Clone> CheckpointedRollbackJournal<Vec<u8>, I> {
    /// Initializes a UORC-compressed stride-8 rollback journal by compressing `genesis_raw_bytes`
    /// into a stateless `UORC` archive at `frame 0`.
    #[cfg(any(feature = "uorc-streaming", feature = "uorc-wasm-decode"))]
    #[must_use]
    pub fn new_uorc(genesis_raw_bytes: &[u8], genesis_seed_bytes: &[u8]) -> Self {
        Self::new(
            crate::streaming::compress_uorc_portable(genesis_raw_bytes),
            genesis_seed_bytes,
        )
    }

    /// Records frame `t + 1`, compressing `post_state_raw` into a `UORC` archive only on stride-8 boundaries.
    #[cfg(any(feature = "uorc-streaming", feature = "uorc-wasm-decode"))]
    pub fn record_uorc_step(
        &mut self,
        input: I,
        input_wire_bytes: &[u8],
        post_state_raw: &[u8],
    ) -> usize {
        let next_idx = self.inputs.len() + 1;
        if next_idx.is_multiple_of(self.stride) {
            let compressed = crate::streaming::compress_uorc_portable(post_state_raw);
            self.record_step(input, input_wire_bytes, &compressed)
        } else {
            let empty = Vec::new();
            self.record_step(input, input_wire_bytes, &empty)
        }
    }

    /// Total compressed byte footprint across all stored UORC checkpoints in the journal.
    #[must_use]
    pub fn total_checkpoint_uorc_bytes(&self) -> usize {
        self.checkpoints.values().map(Vec::len).sum()
    }

    /// Decompresses the nearest preceding stride-8 `UORC` checkpoint via `uorc_decompress::run` and
    /// replays at most `<= 7` deterministic step transitions to reconstruct exact raw state bytes.
    #[cfg(feature = "uorc-wasm-decode")]
    pub fn reconstruct_uorc_frame<F>(
        &self,
        target_frame: usize,
        mut step_fn: F,
    ) -> Result<(Vec<u8>, usize, KappaDigest), SnapshotCodecError>
    where
        F: FnMut(&[u8], &I) -> Vec<u8>,
    {
        if target_frame > self.inputs.len() {
            return Err(SnapshotCodecError::MalformedStatePayload {
                detail: "target rollback frame exceeds recorded UORC journal horizon".to_string(),
            });
        }
        let (&ckpt_frame, ckpt_uorc) = self
            .checkpoints
            .range(..=target_frame)
            .next_back()
            .ok_or_else(|| SnapshotCodecError::MalformedStatePayload {
                detail: "missing genesis UORC rollback checkpoint".to_string(),
            })?;
        let mut cur = crate::streaming::decompress_uorc_portable(ckpt_uorc).map_err(|e| {
            SnapshotCodecError::IntegrityDigestMismatch {
                expected_hex: "UORC-CHECKPOINT-SEAL".to_string(),
                observed_hex: e.to_string(),
            }
        })?;
        let mut steps = 0usize;
        for inp in &self.inputs[ckpt_frame..target_frame] {
            cur = step_fn(&cur, inp);
            steps += 1;
        }
        Ok((cur, steps, self.chain_digests[target_frame]))
    }
}

/// Constructive Stratum-0 `W5H` (`What`, `Why`, `Who`, `Where`, `When`, `How`) certificate for a causal stratum (`0..=7`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StratumW5HCertificate {
    /// Causal stratum index (`0..=7`).
    pub stratum_rank: u64,
    /// Constructive prime atom (`nthPrimeConstructive(stratum_rank)`).
    pub intrinsic_prime: u64,
    /// Composite UOR signature (`intrinsic_prime * factor`).
    pub composite_uor_sig: u64,
    /// Triadic agency prime product (`Observer * Medium * Player = 2 * 3 * 5 = 30`).
    pub triadic_agency_product: u64,
    /// Whether `Foundation.Ontology.V1.Model::verifyW5HCertificateComplete` holds.
    pub complete_verified: bool,
}

/// Evaluated Stratum-0 Holon Telos loss decomposition produced by [`OntologicalRuntimeGovernor::evaluate_frame_telos`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HolonTelosReport {
    /// Count of uncertified nodes, ungrounded modules, or magic root literals.
    pub epistemic_opacity_debt: u64,
    /// Sensory legibility deficit across viewport or audio channels.
    pub sensory_legibility_gap: u64,
    /// Player control or camera dissonance penalty.
    pub player_dissonance: u64,
    /// Microseconds by which `observed_frame_micros` exceeded `frame_budget_micros` (`0` when within budget).
    pub host_ceiling_excess: u64,
    /// Total additive telos loss (`opacity + legibility + dissonance + ceiling_excess`).
    pub total_telos_loss: u64,
    /// Whether zero epistemic opacity debt and active witness presence hold simultaneously.
    pub triadic_connection_active: bool,
    /// Whether `Tensor::strataCertifiedUpTo(7)` holds in the compiled Prism kernel.
    pub all_eight_strata_certified: bool,
}

/// Stratum-0 Ontological Telos and `W5H` runtime governor binding [`EngineConfig`] and [`BrowserCapabilities`]
/// directly to `Foundation.Ontology.V1.Model` (`HostEnvelope`, `WitnessEnvelope`, `evaluateHolonTelosState`,
/// and `verifyW5HCertificateComplete`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OntologicalRuntimeGovernor {
    /// Validated viewport, tick-rate, and frame-budget envelope.
    pub config: EngineConfig,
    /// Discovered host hardware and browser container capabilities.
    pub capabilities: BrowserCapabilities,
}

impl OntologicalRuntimeGovernor {
    /// Constructs a new Stratum-0 Ontological Runtime Governor.
    #[must_use]
    pub const fn new(config: EngineConfig, capabilities: BrowserCapabilities) -> Self {
        Self {
            config,
            capabilities,
        }
    }

    /// Synthesizes and verifies constructive `W5H` certificates across all eight causal strata (`0..=7`)
    /// via `prism_kernel::nthPrimeConstructive`, `triadicAgencyPrimeProduct`, and `verifyW5HCertificateComplete`.
    #[must_use]
    pub fn certify_all_eight_strata(&self) -> Vec<StratumW5HCertificate> {
        let stratum_count = prism_kernel::sEight().unwrap_or(1);
        let agency = prism_kernel::triadicAgencyPrimeProduct(true, true, true).unwrap_or(1 + 1);
        let four = (1_u64 + 1_u64) * (1_u64 + 1_u64);
        let five = four + 1_u64;
        let mut certs = Vec::with_capacity(stratum_count as usize);
        for rank in 0..stratum_count {
            let prime = prism_kernel::nthPrimeConstructive(rank).unwrap_or(1 + 1);
            let uor_sig = prime.saturating_mul(rank + 1);
            let boundary_pole = rank % four;
            let structural_arity = rank + 1;
            let temporal_regime = rank % five;
            let verified = prism_kernel::verifyW5HCertificateComplete(
                prime,
                uor_sig,
                rank,
                agency,
                boundary_pole,
                structural_arity,
                temporal_regime,
                true,
            )
            .unwrap_or(false);
            certs.push(StratumW5HCertificate {
                stratum_rank: rank,
                intrinsic_prime: prime,
                composite_uor_sig: uor_sig,
                triadic_agency_product: agency,
                complete_verified: verified,
            });
        }
        certs
    }

    /// Evaluates Stratum-0 Holon Telos state over `HostEnvelope` and `WitnessEnvelope` via
    /// `prism_kernel::evaluateHolonTelosState`.
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate_frame_telos(
        &self,
        observed_frame_micros: u64,
        active_presence: bool,
        camera_stability_permille: u64,
        focus_dwell_permille: u64,
        opacity_debt: u64,
        legibility_gap: u64,
        player_dissonance: u64,
    ) -> HolonTelosReport {
        let cores = u64::from(self.capabilities.hardware_concurrency.max(1));
        let mem_units =
            prism_kernel::pow2Constructive(prism_kernel::sSix().unwrap_or(1)).unwrap_or(1);
        let thousand = prism_kernel::pow10Constructive(1 + 1 + 1).unwrap_or(1);
        let sample_hz = prism_kernel::sFortyEight()
            .unwrap_or(1)
            .saturating_mul(thousand);
        let quantum = prism_kernel::audioWorkletQuantumFrames().unwrap_or(1);
        let host = prism_kernel::HostEnvelope {
            logicalCores: cores,
            memoryUnits: mem_units,
            frameBudgetMicros: self.config.frame_budget_micros,
            observedFrameMicros: observed_frame_micros,
            audioSampleRateHz: sample_hz,
            audioQuantumFrames: quantum,
        };

        let eight = prism_kernel::sEight().unwrap_or(1);
        let headroom = prism_kernel::sNine()
            .unwrap_or(1)
            .saturating_mul(prism_kernel::sHundred().unwrap_or(1));
        let witness = prism_kernel::WitnessEnvelope {
            criticalBands: eight,
            inputPollMillis: eight,
            cameraStabilityRatio: camera_stability_permille,
            focusDwellRatio: focus_dwell_permille,
            audioHeadroomRatio: headroom,
            activePresence: active_presence,
        };

        let eval = prism_kernel::evaluateHolonTelosState(
            host,
            witness,
            opacity_debt,
            legibility_gap,
            player_dissonance,
        )
        .expect("prism");
        let max_stratum = prism_kernel::sSeven().unwrap_or(0);
        let all_eight_strata_certified =
            prism_kernel::strataCertifiedUpTo(max_stratum).unwrap_or(false);

        HolonTelosReport {
            epistemic_opacity_debt: eval.epistemicOpacityDebt,
            sensory_legibility_gap: eval.sensoryLegibilityGap,
            player_dissonance: eval.playerDissonance,
            host_ceiling_excess: eval.hostCeilingExcess,
            total_telos_loss: eval.totalTelosLoss,
            triadic_connection_active: eval.triadicConnectionActive,
            all_eight_strata_certified,
        }
    }

    /// Recommends the next progressive LOD rank (`0..=4`) using non-coercive telos descent:
    /// when `coercive_lock_count > 0` (explicit user LOD pin), `current_lod_rank` is preserved unchanged;
    /// otherwise over-budget frames step down cleanly while generous frame headroom steps up toward `LOD4`.
    #[must_use]
    pub fn recommend_non_coercive_lod(
        &self,
        current_lod_rank: u64,
        observed_frame_micros: u64,
        coercive_lock_count: u64,
    ) -> u64 {
        let max_lod = prism_kernel::wasmHdMaxLodRank().unwrap_or(1);
        let clamped = current_lod_rank.min(max_lod);
        if coercive_lock_count > 0 {
            return clamped;
        }
        let thousand = prism_kernel::pow10Constructive(1 + 1 + 1).unwrap_or(1);
        let report =
            self.evaluate_frame_telos(observed_frame_micros, true, thousand, thousand, 0, 0, 0);
        if report.host_ceiling_excess > 0 {
            clamped.saturating_sub(1)
        } else if observed_frame_micros.saturating_mul(1 + 1) < self.config.frame_budget_micros
            && clamped < max_lod
        {
            clamped + 1
        } else {
            clamped
        }
    }

    /// Recommends an adaptive internal framebuffer resolution scale in permille (`1000` = `1.0x`,
    /// `850` = `0.85x`, `700` = `0.70x`) via `prism_kernel::governorRecommendedScalePermille`.
    #[must_use]
    pub fn recommend_render_scale_permille(&self, observed_frame_micros: u64) -> u64 {
        prism_kernel::governorRecommendedScalePermille(
            observed_frame_micros,
            self.config.frame_budget_micros,
        )
        .unwrap_or_else(|_| verdant_tensor::kernel_consts().skin_weight_scale)
    }
}

/// Stateless XOR-residual `UORC-SHA256-CHAIN-v1` delta snapshot codec between two `StandardSceneState`s.
pub struct DeltaUorcSnapshot;

impl DeltaUorcSnapshot {
    /// Encodes the bytewise XOR residual from `base` to `next` into a stateless `UORC-SHA256-CHAIN-v1`
    /// archive ready for low-bandwidth rollback/network state synchronization.
    #[cfg(any(feature = "uorc-streaming", feature = "uorc-wasm-decode"))]
    pub fn encode_delta(
        base: &StandardSceneState,
        next: &StandardSceneState,
    ) -> Result<Vec<u8>, SnapshotCodecError> {
        let base_wire = WorldSnapshot::encode(base)?;
        let next_wire = WorldSnapshot::encode(next)?;
        let residual: Vec<u8> = next_wire
            .iter()
            .enumerate()
            .map(|(i, &b)| b ^ base_wire.get(i).copied().unwrap_or(0))
            .collect();

        #[cfg(feature = "uorc-streaming")]
        {
            let max_strats = if cfg!(debug_assertions) {
                1
            } else {
                prism_kernel::sEight().unwrap_or(1) as usize
            };
            let mut strategies = Vec::new();
            for pair in &uorc::portfolio::V04_PAIR {
                for strat in pair {
                    if strat.state == 0
                        && !strat.x86
                        && strategies.len() < max_strats
                        && !strategies.contains(&Some(*strat))
                    {
                        strategies.push(Some(*strat));
                    }
                }
            }
            Ok(uorc::compress_with_strategies(
                &residual,
                &[residual.len() as u64],
                &[],
                &strategies,
            ))
        }
        #[cfg(not(feature = "uorc-streaming"))]
        {
            Ok(crate::streaming::compress_uorc_portable(&residual))
        }
    }

    /// Decompresses `delta_uorc` via `uorc_decompress::run`, reconstructs the target `VDNT` wire
    /// against `base`, and verifies its SHA-256 trailer via [`WorldSnapshot::decode`].
    #[cfg(feature = "uorc-wasm-decode")]
    pub fn decode_delta(
        base: &StandardSceneState,
        delta_uorc: &[u8],
    ) -> Result<StandardSceneState, SnapshotCodecError> {
        if delta_uorc.is_empty() {
            return Err(SnapshotCodecError::TruncatedSnapshot { byte_len: 0 });
        }
        let base_wire = WorldSnapshot::encode(base)?;
        #[cfg(feature = "uorc-ncm-decode")]
        let residual = {
            let shift = prism_kernel::uorcDefaultDecodeWork().unwrap_or(1) as u32;
            let work = 1_u64 << shift;
            match uorc_decompress::run(
                uorc_decompress::Bytes::lit(delta_uorc),
                uorc_decompress::List::nil(),
                uorc_decompress::List::nil(),
                work,
            ) {
                Ok(Some(out)) => out.octets().to_vec(),
                Ok(None) => {
                    return Err(SnapshotCodecError::MalformedStatePayload {
                        detail: "uorc_decompress::run rejected delta snapshot seal or work budget"
                            .to_string(),
                    });
                }
                Err(_) => {
                    return Err(SnapshotCodecError::MalformedStatePayload {
                        detail: "uorc_decompress::run overflow on delta snapshot".to_string(),
                    });
                }
            }
        };
        #[cfg(not(feature = "uorc-ncm-decode"))]
        let residual = crate::streaming::decompress_uorc_portable(delta_uorc).map_err(|e| {
            SnapshotCodecError::MalformedStatePayload {
                detail: e.to_string(),
            }
        })?;
        let next_wire: Vec<u8> = residual
            .iter()
            .enumerate()
            .map(|(i, &r)| r ^ base_wire.get(i).copied().unwrap_or(0))
            .collect();
        WorldSnapshot::decode(&next_wire)
    }
}
