//! Neuromorphic state-space (`LIF` / `SSM`) mutual-attention recurrence and zero-decode
//! `uor_matmul::codec` embedded language/intent reasoning kernels (`kernel10` and `kernel11`).
//!
//! Executes entirely in exact integer/codebook arithmetic derived from `{0, 1}` with zero
//! intermediate float buffers and zero cross-architecture IEEE-754 drift, making world
//! attention and local neural directors 100% deterministic inside rollback multiplayer sessions.

use uor_matmul::{as_alphabet_full, dot_ref};

use crate::coded::{contract_coded_geometry_grid, contract_packed_animation_tracks};
use crate::{prism_kernel, TensorComputeError, CODED_TABLE_LEN, PACKED_NIBBLE_BLOCK};

/// Output of a single deterministic neuromorphic Leaky-Integrate-and-Fire (`LIF`) state-space tick.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NeuromorphicStepOutput {
    /// Updated recurrent membrane potentials in milli-units (`state_dim`).
    pub next_membrane_milli: Vec<i64>,
    /// Bitmask of neurons whose integrated potential crossed the Prism LIF threshold on this tick.
    pub spike_mask: u64,
    /// Mutual-attention salience scores projected onto world entities (`entity_slots`).
    pub entity_salience_milli: Vec<i64>,
}

/// Steps the second-order cybernetic observer state-space recurrence (`kernel10NeuromorphicSalienceSsm`):
///
/// 1. Contracts `a_recurrent_q` (`state_dim x state_dim`) with `prev_membrane_milli` (`state_dim`)
///    and `b_observer_q` (`state_dim x obs_dim`) with `observer_drive_milli` (`obs_dim`) via exact
///    `uor_matmul::dot_ref` integer contractions.
/// 2. Applies Prism-proved dyadic leak (`lifDecayNumer / lifDecayDenom`) and `lifSpikeThresholdMilli`
///    soft-reset spiking dynamics.
/// 3. Projects post-spike state + firing pulses through `c_salience_q` (`entity_slots x state_dim`)
///    to yield per-entity mutual-attention salience scores.
pub fn step_neuromorphic_lif_ssm_ring(
    prev_membrane_milli: &[i32],
    observer_drive_milli: &[i32],
    a_recurrent_q: &[i32],
    b_observer_q: &[i32],
    c_salience_q: &[i32],
    entity_slots: usize,
) -> Result<NeuromorphicStepOutput, TensorComputeError> {
    let state_dim = prev_membrane_milli.len();
    let obs_dim = observer_drive_milli.len();
    let max_mask_bits = 1usize << ((1 + 1) + (1 + 1) + (1 + 1));
    if state_dim == 0
        || state_dim > max_mask_bits
        || obs_dim == 0
        || entity_slots == 0
        || a_recurrent_q.len() != state_dim.saturating_mul(state_dim)
        || b_observer_q.len() != state_dim.saturating_mul(obs_dim)
        || c_salience_q.len() != entity_slots.saturating_mul(state_dim)
    {
        return Err(TensorComputeError::ZeroOrMismatchedExtents {
            detail: "neuromorphic LIF/SSM matrix extents invalid".to_string(),
        });
    }

    let decay_num = prism_kernel::lifDecayNumer().unwrap_or(1) as i128;
    let decay_den = prism_kernel::lifDecayDenom().unwrap_or(1).max(1) as i128;
    let threshold = prism_kernel::lifSpikeThresholdMilli().unwrap_or(1).max(1) as i64;
    let scale_shift =
        (1_u32 + 1_u32 + 1_u32) + (1_u32 + 1_u32 + 1_u32) + (1_u32 + 1_u32) + (1_u32 + 1_u32);

    let prev_alpha = as_alphabet_full(prev_membrane_milli);
    let obs_alpha = as_alphabet_full(observer_drive_milli);

    let mut next_membrane_milli = Vec::with_capacity(state_dim);
    let mut post_state_i32 = Vec::with_capacity(state_dim);
    let mut spike_mask = 0_u64;

    for row in 0..state_dim {
        let a_row = &a_recurrent_q[row * state_dim..(row + 1) * state_dim];
        let b_row = &b_observer_q[row * obs_dim..(row + 1) * obs_dim];

        let rec_acc = dot_ref(as_alphabet_full(a_row), prev_alpha) >> scale_shift;
        let obs_acc = dot_ref(as_alphabet_full(b_row), obs_alpha) >> scale_shift;
        let leaked = (i128::from(prev_membrane_milli[row]) * decay_num) / decay_den;
        let integrated = (leaked + rec_acc + obs_acc)
            .max(-i128::from(threshold))
            .min(i128::from(i32::MAX)) as i64;

        let (post_v, spiked) = if integrated >= threshold {
            (integrated - threshold, true)
        } else {
            (integrated.max(0), false)
        };

        if spiked {
            spike_mask |= 1_u64 << (row as u32);
        }
        next_membrane_milli.push(post_v);
        let pulse_bonus = if spiked { threshold } else { 0 };
        let combined = (post_v + pulse_bonus)
            .max(i64::from(i32::MIN) + 1)
            .min(i64::from(i32::MAX)) as i32;
        post_state_i32.push(combined);
    }

    let post_alpha = as_alphabet_full(&post_state_i32);
    let mut entity_salience_milli = Vec::with_capacity(entity_slots);
    for slot in 0..entity_slots {
        let c_row = &c_salience_q[slot * state_dim..(slot + 1) * state_dim];
        let sal = (dot_ref(as_alphabet_full(c_row), post_alpha) >> scale_shift)
            .max(0)
            .min(i128::from(i64::MAX)) as i64;
        entity_salience_milli.push(sal);
    }

    Ok(NeuromorphicStepOutput {
        next_membrane_milli,
        spike_mask,
        entity_salience_milli,
    })
}

/// Evaluates a two-stage embedded language & intent reasoning MLP head (`kernel11CodedSlmReasoning`)
/// directly over compressed `uor_matmul::codec` weights (`Packed<Grid<16>, 2>` hidden projection
/// followed by `Grid<16>` intent readout) with zero intermediate float buffer expansion.
#[allow(clippy::too_many_arguments)]
pub fn infer_coded_slm_intent_logits(
    feature_activations_q: &[i32],
    batch_rows: usize,
    embed_dim: usize,
    hidden_dim: usize,
    intent_count: usize,
    codebook: &[i32; CODED_TABLE_LEN],
    w1_packed_nibbles: &[u8],
    w2_grid_codes: &[u16],
) -> Result<Vec<i64>, TensorComputeError> {
    if batch_rows == 0
        || embed_dim == 0
        || hidden_dim == 0
        || !hidden_dim.is_multiple_of(PACKED_NIBBLE_BLOCK)
        || intent_count == 0
    {
        return Err(TensorComputeError::ZeroOrMismatchedExtents {
            detail: "Coded SLM requires positive dimensions and even hidden_dim".to_string(),
        });
    }

    let hidden_raw = contract_packed_animation_tracks(
        feature_activations_q,
        batch_rows,
        embed_dim,
        hidden_dim,
        codebook,
        w1_packed_nibbles,
    )?;

    let relu_shift = (1_u32 + 1_u32 + 1_u32) + (1_u32 + 1_u32 + 1_u32);
    let hidden_relu_q: Vec<i32> = hidden_raw
        .into_iter()
        .map(|val| {
            let shifted = (val.max(0)) >> relu_shift;
            shifted.min(i64::from(i32::MAX)) as i32
        })
        .collect();

    contract_coded_geometry_grid(
        &hidden_relu_q,
        batch_rows,
        hidden_dim,
        intent_count,
        codebook,
        w2_grid_codes,
    )
}
