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
