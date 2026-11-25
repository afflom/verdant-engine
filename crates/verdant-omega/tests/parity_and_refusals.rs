//! Bit-exact parity and negative-control refusal tests for the pure-Omega neural worker.

use verdant_omega::{
    OmegaCompiledSlm, OmegaNeuralWorker, OP_LIF_SSM, OP_PING, STATUS_BAD_MAGIC,
    STATUS_PAYLOAD_MISMATCH, STATUS_UNKNOWN_OP, WIRE_MAGIC,
};
use verdant_tensor::{
    contract_coded_pair_lut_into, contract_packed_nibbles_lut_into, infer_coded_slm_intent_logits,
    step_neuromorphic_lif_ssm_ring, tropical_min_plus_all_pairs_closure, CodebookProductLut,
    TensorComputeError, CODED_TABLE_LEN,
};

struct SplitMix64(u64);

impl SplitMix64 {
    fn new(seed: u64) -> Self {
        Self(seed)
    }

    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    fn next_i32_range(&mut self, half_span: i32) -> i32 {
        let span = i64::from(half_span) * 2 + 1;
        let raw = (self.next_u64() % (span as u64)) as i64;
        (raw - i64::from(half_span)) as i32
    }
}

fn worker_or_skip() -> OmegaNeuralWorker {
    OmegaNeuralWorker::spawn_default().expect(
        "pure-Omega worker binary missing; run `sh om/build.sh` before running verdant-omega tests",
    )
}

#[test]
fn lif_ssm_bit_exact_parity_across_shapes_and_rollout() {
    let mut worker = worker_or_skip();
    let mut rng = SplitMix64::new(0x1234_5678_9abc_def0);

    let shapes = [(1, 1, 1), (8, 6, 4), (16, 12, 24), (64, 16, 32)];
    for (state_dim, obs_dim, entity_slots) in shapes {
        let mut prev: Vec<i32> = (0..state_dim).map(|_| rng.next_i32_range(4096)).collect();
        let obs: Vec<i32> = (0..obs_dim).map(|_| rng.next_i32_range(4096)).collect();
        let a: Vec<i32> = (0..state_dim * state_dim)
            .map(|_| rng.next_i32_range(2048))
            .collect();
        let b: Vec<i32> = (0..state_dim * obs_dim)
            .map(|_| rng.next_i32_range(2048))
            .collect();
        let c: Vec<i32> = (0..entity_slots * state_dim)
            .map(|_| rng.next_i32_range(2048))
            .collect();

        for tick in 0..8 {
            let expected =
                step_neuromorphic_lif_ssm_ring(&prev, &obs, &a, &b, &c, entity_slots).unwrap();
            let actual = worker
                .step_neuromorphic_lif_ssm_ring(&prev, &obs, &a, &b, &c, entity_slots)
                .unwrap();
            assert_eq!(
                actual, expected,
                "LIF/SSM parity drift at shape=({state_dim},{obs_dim},{entity_slots}) tick={tick}"
            );

            prev = expected
                .next_membrane_milli
                .iter()
                .map(|&v| v.clamp(i64::from(i32::MIN) + 1, i64::from(i32::MAX)) as i32)
                .collect();
        }
    }
}

#[test]
fn lif_ssm_bit_exact_parity_at_i32_extremes_requiring_128bit_acc() {
    let mut worker = worker_or_skip();
    let entity_slots = 3;

    // Products of i32::MIN * i32::MIN exceed i64::MAX when summed across >= 2 elements.
    let prev = vec![i32::MIN, i32::MAX, i32::MIN, i32::MAX];
    let obs = vec![i32::MAX, i32::MIN, i32::MAX, i32::MIN];
    let a = vec![
        i32::MIN,
        i32::MIN,
        i32::MIN,
        i32::MIN, // huge positive 128-bit sum
        i32::MAX,
        i32::MIN,
        i32::MAX,
        i32::MIN, // huge negative 128-bit sum
        0,
        0,
        0,
        0,
        i32::MAX,
        i32::MAX,
        i32::MAX,
        i32::MAX,
    ];
    let b = vec![
        i32::MAX,
        i32::MIN,
        i32::MAX,
        i32::MIN,
        i32::MIN,
        i32::MAX,
        i32::MIN,
        i32::MAX,
        i32::MIN,
        i32::MIN,
        i32::MIN,
        i32::MIN,
        0,
        0,
        0,
        0,
    ];
    let c = vec![
        i32::MAX,
        i32::MAX,
        i32::MAX,
        i32::MAX,
        i32::MIN,
        i32::MIN,
        i32::MIN,
        i32::MIN,
        1024,
        -1024,
        2048,
        -2048,
    ];

    let expected = step_neuromorphic_lif_ssm_ring(&prev, &obs, &a, &b, &c, entity_slots).unwrap();
    let actual = worker
        .step_neuromorphic_lif_ssm_ring(&prev, &obs, &a, &b, &c, entity_slots)
        .unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn coded_slm_bit_exact_parity_across_batches_and_real_shapes() {
    let mut worker = worker_or_skip();
    let mut rng = SplitMix64::new(0xfeed_face_cafe_beef);

    let cases = [
        (1, 32, 32, 16), // Verdant CodedSlmDirector exact dimensions
        (4, 16, 12, 8),
        (8, 64, 48, 16),
        (2, 7, 2, 5),
    ];

    for (batch_rows, embed_dim, hidden_dim, intent_count) in cases {
        let codebook: [i32; CODED_TABLE_LEN] =
            core::array::from_fn(|i| ((i as i32) - 8) * 16 + (i as i32));
        let feats: Vec<i32> = (0..batch_rows * embed_dim)
            .map(|_| rng.next_i32_range(8192))
            .collect();
        let w1: Vec<u8> = (0..embed_dim * (hidden_dim / 2))
            .map(|_| (rng.next_u64() & 0xff) as u8)
            .collect();
        let w2: Vec<u16> = (0..hidden_dim * intent_count)
            .map(|_| (rng.next_u64() % (CODED_TABLE_LEN as u64)) as u16)
            .collect();

        let expected = infer_coded_slm_intent_logits(
            &feats,
            batch_rows,
            embed_dim,
            hidden_dim,
            intent_count,
            &codebook,
            &w1,
            &w2,
        )
        .unwrap();
        let actual = worker
            .infer_coded_slm_intent_logits(
                &feats,
                batch_rows,
                embed_dim,
                hidden_dim,
                intent_count,
                &codebook,
                &w1,
                &w2,
            )
            .unwrap();
        assert_eq!(
            actual, expected,
            "coded_gemm SLM parity drift at ({batch_rows},{embed_dim},{hidden_dim},{intent_count})"
        );
    }
}

#[test]
fn negative_controls_refuse_invalid_frames_and_keep_worker_alive() {
    let mut worker = worker_or_skip();

    // 1. Bad magic
    let (st, payload) = worker.send_raw_frame(0xdead_beef, OP_PING, &[]).unwrap();
    assert_eq!(st, STATUS_BAD_MAGIC);
    assert!(payload.is_empty());

    // 2. Unknown op
    let (st, _) = worker.send_raw_frame(WIRE_MAGIC, 999, &[]).unwrap();
    assert_eq!(st, STATUS_UNKNOWN_OP);

    // 3. Truncated LIF payload
    let (st, _) = worker
        .send_raw_frame(WIRE_MAGIC, OP_LIF_SSM, &[0_u8; 16])
        .unwrap();
    assert_eq!(st, STATUS_PAYLOAD_MISMATCH);

    // 4. Invalid LIF extents (state_dim = 65 > 64)
    let err = worker
        .step_neuromorphic_lif_ssm_ring(&[0; 65], &[0; 1], &[0; 65 * 65], &[0; 65], &[0; 65], 1)
        .unwrap_err();
    assert!(matches!(
        err,
        TensorComputeError::ZeroOrMismatchedExtents { ref detail } if detail == "omega_worker:invalid_extents"
    ));

    // 5. Invalid LIF params (decay_den = 0)
    let err = worker
        .step_neuromorphic_lif_ssm_ring_with_params(&[1], &[1], &[1], &[1], &[1], 1, 10, 7, 0, 100)
        .unwrap_err();
    assert!(matches!(
        err,
        TensorComputeError::InvalidKernelSpecification { ref kernel_name }
            if kernel_name == "omega_worker:refused_status_6"
    ));

    // 6. Odd hidden_dim in coded_gemm SLM
    let codebook = [0_i32; CODED_TABLE_LEN];
    let err = worker
        .infer_coded_slm_intent_logits(&[1, 2], 1, 2, 3, 2, &codebook, &[0, 0], &[0; 6])
        .unwrap_err();
    assert!(matches!(
        err,
        TensorComputeError::ZeroOrMismatchedExtents { ref detail } if detail == "omega_worker:invalid_extents"
    ));

    // 7. Out-of-range codebook index (>= 16) in w2_grid_codes
    let err = worker
        .infer_coded_slm_intent_logits(&[1, 2], 1, 2, 2, 2, &codebook, &[0, 0], &[0, 16, 0, 0])
        .unwrap_err();
    assert!(matches!(
        err,
        TensorComputeError::ZeroOrMismatchedExtents { ref detail }
            if detail == "omega_worker:invalid_codebook_index"
    ));

    // Worker must still be healthy after every refusal.
    worker.ping().unwrap();
}

#[test]
fn planted_defect_control_detects_single_bit_drift() {
    let mut worker = worker_or_skip();
    let codebook: [i32; CODED_TABLE_LEN] = core::array::from_fn(|i| ((i as i32) - 8) * 16);
    let feats = vec![128_i32; 32];
    let w1 = vec![0xf9_u8; 32 * 16];
    let w2 = vec![15_u16; 32 * 16];

    let expected =
        infer_coded_slm_intent_logits(&feats, 1, 32, 32, 16, &codebook, &w1, &w2).unwrap();
    let mut actual = worker
        .infer_coded_slm_intent_logits(&feats, 1, 32, 32, 16, &codebook, &w1, &w2)
        .unwrap();
    assert_eq!(actual, expected);

    // Plant a 1-bit defect and confirm parity check rejects it.
    actual[0] ^= 1;
    assert_ne!(actual, expected);
}

#[test]
fn coded_lut_and_packed_nibbles_bit_exact_parity() {
    let mut worker = worker_or_skip();
    let mut rng = SplitMix64::new(0x0bad_c0de_1234_5678);

    let lhs_cb: [i32; CODED_TABLE_LEN] = core::array::from_fn(|i| ((i as i32) - 8) * 19 + 3);
    let rhs_cb: [i32; CODED_TABLE_LEN] = core::array::from_fn(|i| ((i as i32) - 7) * 23 - 5);
    let lut = CodebookProductLut::new(&lhs_cb, &rhs_cb);

    for (rows, inner_k, cols) in [(1, 1, 1), (7, 13, 5), (16, 64, 16), (8, 33, 12)] {
        let lhs_codes: Vec<u8> = (0..rows * inner_k)
            .map(|_| (rng.next_u64() & 0xff) as u8)
            .collect();
        let rhs_codes: Vec<u8> = (0..inner_k * cols)
            .map(|_| (rng.next_u64() & 0xff) as u8)
            .collect();
        let mut expected_pair = vec![0_i64; rows * cols];
        contract_coded_pair_lut_into(
            &lhs_codes,
            &rhs_codes,
            rows,
            inner_k,
            cols,
            &lut,
            &mut expected_pair,
        )
        .unwrap();
        let actual_pair = worker
            .contract_coded_pair_lut(
                &lhs_codes, &rhs_codes, rows, inner_k, cols, &lhs_cb, &rhs_cb,
            )
            .unwrap();
        assert_eq!(actual_pair, expected_pair);

        let packed_k = inner_k.div_ceil(2);
        let lhs_packed: Vec<u8> = (0..rows * packed_k)
            .map(|_| (rng.next_u64() & 0xff) as u8)
            .collect();
        let rhs_packed: Vec<u8> = (0..cols * packed_k)
            .map(|_| (rng.next_u64() & 0xff) as u8)
            .collect();
        let mut expected_packed = vec![0_i64; rows * cols];
        contract_packed_nibbles_lut_into(
            &lhs_packed,
            &rhs_packed,
            rows,
            inner_k,
            cols,
            &lut,
            &mut expected_packed,
        )
        .unwrap();
        let actual_packed = worker
            .contract_packed_nibbles_lut(
                &lhs_packed,
                &rhs_packed,
                rows,
                inner_k,
                cols,
                &lhs_cb,
                &rhs_cb,
            )
            .unwrap();
        assert_eq!(actual_packed, expected_packed);
    }
}

#[test]
fn tropical_min_plus_all_pairs_closure_bit_exact_parity() {
    let mut worker = worker_or_skip();
    let mut rng = SplitMix64::new(0x7777_8888_9999_aaaa);

    for n in [1, 4, 12, 24] {
        let mut edges = Vec::with_capacity(n * n);
        for _ in 0..n * n {
            let roll = rng.next_u64() % 10;
            if roll < 5 {
                edges.push(None);
            } else if roll == 9 {
                edges.push(Some(u32::MAX));
            } else {
                edges.push(Some((rng.next_u64() % 50_000) as u32));
            }
        }
        let expected = tropical_min_plus_all_pairs_closure(&edges, n).unwrap();
        let actual = worker
            .tropical_min_plus_all_pairs_closure(&edges, n)
            .unwrap();
        assert_eq!(actual, expected, "tropical min-plus drift at n={n}");
    }

    // Negative control: node_count = 0 must be refused.
    let err = worker
        .tropical_min_plus_all_pairs_closure(&[], 0)
        .unwrap_err();
    assert!(matches!(
        err,
        TensorComputeError::ZeroOrMismatchedExtents { ref detail }
            if detail == "omega_worker:invalid_extents"
    ));
}

#[test]
fn partial_evaluating_omega_slm_jit_matches_verdant_tensor_bit_for_bit() {
    // Exact Verdant CodedSlmDirector model (32x32x16, 93.75% sparse zeros erased at compile time).
    let intent_count = CODED_TABLE_LEN;
    let embed_dim = CODED_TABLE_LEN * 2;
    let hidden_dim = CODED_TABLE_LEN * 2;
    let codebook: [i32; CODED_TABLE_LEN] = core::array::from_fn(|i| ((i as i32) - 8) * 16);
    let zero_code = 8_u8;
    let pos_strong = 15_u8;
    let pos_mild = 10_u8;

    let half_cols = hidden_dim / 2;
    let mut w1 = vec![0_u8; embed_dim * half_cols];
    for r in 0..embed_dim {
        for pair in 0..half_cols {
            let c0 = pair * 2;
            let c1 = c0 + 1;
            let k0 = if r == c0 {
                pos_strong
            } else if (r % intent_count) == (c0 % intent_count) {
                pos_mild
            } else {
                zero_code
            };
            let k1 = if r == c1 {
                pos_strong
            } else if (r % intent_count) == (c1 % intent_count) {
                pos_mild
            } else {
                zero_code
            };
            w1[r * half_cols + pair] = k0 | (k1 << 4);
        }
    }
    let mut w2 = vec![u16::from(zero_code); hidden_dim * intent_count];
    for h in 0..hidden_dim {
        for k in 0..intent_count {
            if (h % intent_count) == k {
                w2[h * intent_count + k] = u16::from(pos_strong);
            }
        }
    }

    let tmp_bin =
        std::env::temp_dir().join(format!("verdant_omega_jit_test_{}", std::process::id()));
    let mut jit = OmegaCompiledSlm::compile_and_spawn(
        embed_dim,
        hidden_dim,
        intent_count,
        &codebook,
        &w1,
        &w2,
        &tmp_bin,
    )
    .expect("pure-Omega SLM JIT synthesis + omc compile failed");

    let mut rng = SplitMix64::new(0x51a7_c0de_cafe_1234);
    for batch_rows in [1, 8, 64] {
        let feats: Vec<i32> = (0..batch_rows * embed_dim)
            .map(|_| rng.next_i32_range(8192))
            .collect();
        let expected = infer_coded_slm_intent_logits(
            &feats,
            batch_rows,
            embed_dim,
            hidden_dim,
            intent_count,
            &codebook,
            &w1,
            &w2,
        )
        .unwrap();
        let actual = jit.infer_batch(&feats, batch_rows).unwrap();
        assert_eq!(actual, expected, "JIT SLM drift at batch_rows={batch_rows}");
    }

    drop(jit);
    let _ = std::fs::remove_file(&tmp_bin);
}
