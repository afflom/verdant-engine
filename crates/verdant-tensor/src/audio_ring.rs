//! Quantized `Q15` audio-ring contractions over `uor-matmul`.
//!
//! Hosts multi-emitter Woodworth stereo/multichannel panning, Toeplitz modal FIR impulse-response
//! convolution, and 4-channel orthogonal Hadamard Sabine/Eyring feedback delay network (`FDN`)
//! reverberation steps with bit-exact integer accumulation.

#[cfg(not(target_arch = "wasm32"))]
use uor_matmul::{slice, suggested_scratch, Shape};

use crate::{prism_kernel, TensorComputeError, DIM_HOMOGENEOUS};

/// Mixes `emitter_count` Q15 audio sources across `frame_count` audio samples into `channel_count`
/// output channels via exact integer `uor-matmul` contraction (`i16 -> i64`).
pub fn mix_spatial_audio_channels_exact(
    frame_count: usize,
    emitter_count: usize,
    channel_count: usize,
    emitter_frames_q15: &[i16],
    pan_gains_q15: &[i16],
) -> Result<Vec<i64>, TensorComputeError> {
    if frame_count == 0 || emitter_count == 0 || channel_count == 0 {
        return Err(TensorComputeError::ZeroOrMismatchedExtents {
            detail: "audio frame, emitter, and channel counts must all be positive".to_string(),
        });
    }
    if emitter_frames_q15.len() != frame_count.saturating_mul(emitter_count)
        || pan_gains_q15.len() != emitter_count.saturating_mul(channel_count)
    {
        return Err(TensorComputeError::ZeroOrMismatchedExtents {
            detail: "audio emitter or pan gain buffer length does not match declared shape"
                .to_string(),
        });
    }

    let mut out = vec![0_i64; frame_count * channel_count];

    #[cfg(not(target_arch = "wasm32"))]
    {
        let mut scratch = vec![
            0_i16;
            suggested_scratch(Shape {
                m: frame_count,
                k: emitter_count,
                n: channel_count,
            })
        ];
        slice::gemm(
            frame_count,
            emitter_count,
            channel_count,
            emitter_frames_q15,
            pan_gains_q15,
            &mut out,
            &mut scratch,
        )
        .map_err(|_| TensorComputeError::ZeroOrMismatchedExtents {
            detail: "audio integer gemm shape mismatch".to_string(),
        })?;
    }

    #[cfg(target_arch = "wasm32")]
    {
        let mut b_cols = vec![vec![0_i16; emitter_count]; channel_count];
        for e in 0..emitter_count {
            for ch in 0..channel_count {
                b_cols[ch][e] = pan_gains_q15[e * channel_count + ch];
            }
        }
        for f in 0..frame_count {
            let row = &emitter_frames_q15[f * emitter_count..(f + 1) * emitter_count];
            let a_alpha = uor_matmul::as_alphabet_full(row);
            for ch in 0..channel_count {
                let b_alpha = uor_matmul::as_alphabet_full(&b_cols[ch]);
                let wide = uor_matmul::dot_ref(a_alpha, b_alpha);
                out[f * channel_count + ch] =
                    wide.max(i128::from(i64::MIN)).min(i128::from(i64::MAX)) as i64;
            }
        }
    }

    Ok(out)
}

/// Convolves `dry_frames_q15` (`N` samples) with `modal_taps_q15` (`K` FIR impulse taps) by
/// forming the causal Toeplitz history matrix (`N x K`) and contracting via [`mix_spatial_audio_channels_exact`].
pub fn convolve_modal_fir_exact(
    dry_frames_q15: &[i16],
    modal_taps_q15: &[i16],
) -> Result<Vec<i64>, TensorComputeError> {
    if dry_frames_q15.is_empty() || modal_taps_q15.is_empty() {
        return Err(TensorComputeError::ZeroOrMismatchedExtents {
            detail: "FIR convolution requires non-empty dry signal and impulse taps".to_string(),
        });
    }
    let n = dry_frames_q15.len();
    let k = modal_taps_q15.len();
    let mut toeplitz = vec![0_i16; n * k];
    for frame in 0..n {
        for tap in 0..k {
            if frame >= tap {
                toeplitz[frame * k + tap] = dry_frames_q15[frame - tap];
            }
        }
    }
    mix_spatial_audio_channels_exact(n, k, 1, &toeplitz, modal_taps_q15)
}

/// Advances a 4-channel orthogonal Hadamard Sabine/Eyring feedback delay network (`FDN`) across
/// `delay_lines_q15` (`N x 4`) scaled by `decay_gains_q15` (`4` Q15 tank feedback coefficients)
/// via exact `uor-matmul` integer contraction.
pub fn step_sabine_fdn_reverb_ring(
    delay_lines_q15: &[[i16; DIM_HOMOGENEOUS]],
    decay_gains_q15: &[i16; DIM_HOMOGENEOUS],
) -> Result<Vec<[i64; DIM_HOMOGENEOUS]>, TensorComputeError> {
    if delay_lines_q15.is_empty() {
        return Err(TensorComputeError::ZeroOrMismatchedExtents {
            detail: "FDN reverb delay line batch must be non-empty".to_string(),
        });
    }
    // Normalized 4x4 Hadamard feedback matrix `(1/2) * H_4` scaled by `decay_gains_q15`:
    let half_div = prism_kernel::sTwo() as i16;
    let signs: [[i16; DIM_HOMOGENEOUS]; DIM_HOMOGENEOUS] =
        [[1, 1, 1, 1], [1, -1, 1, -1], [1, 1, -1, -1], [1, -1, -1, 1]];
    let mut fdn_matrix = [0_i16; DIM_HOMOGENEOUS * DIM_HOMOGENEOUS];
    for r in 0..DIM_HOMOGENEOUS {
        for c in 0..DIM_HOMOGENEOUS {
            fdn_matrix[r * DIM_HOMOGENEOUS + c] = (decay_gains_q15[r] / half_div) * signs[r][c];
        }
    }

    let frame_count = delay_lines_q15.len();
    let mut flat_in = Vec::with_capacity(frame_count * DIM_HOMOGENEOUS);
    for d in delay_lines_q15 {
        flat_in.extend_from_slice(d);
    }

    let mixed = mix_spatial_audio_channels_exact(
        frame_count,
        DIM_HOMOGENEOUS,
        DIM_HOMOGENEOUS,
        &flat_in,
        &fdn_matrix,
    )?;

    let i2 = 1usize + 1usize;
    let i3 = i2 + 1usize;
    let mut out = Vec::with_capacity(frame_count);
    for chunk in mixed.chunks_exact(DIM_HOMOGENEOUS) {
        out.push([chunk[0], chunk[1], chunk[i2], chunk[i3]]);
    }
    Ok(out)
}
