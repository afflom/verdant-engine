//! Bit-exact parity and negative-control refusal tests for the pure-Omega neural worker.

use verdant_omega::{
    default_raster_binary_path, default_slm_jit_binary_path, default_worker_binary_path,
    verify_locked_omc, OmegaCompiledSlm, OmegaNeuralWorker, OmegaRasterWorker, OP_LIF_SSM, OP_PING,
    STATUS_BAD_MAGIC, STATUS_PAYLOAD_MISMATCH, STATUS_UNKNOWN_OP, WIRE_MAGIC,
};
use verdant_tensor::{
    contract_coded_pair_lut_into, contract_packed_nibbles_lut_into, infer_coded_slm_intent_logits,
    render_triangles_tile_framebuffer, step_neuromorphic_lif_ssm_ring,
    tropical_min_plus_all_pairs_closure, CodebookProductLut, RasterTriangle, TensorComputeError,
    CODED_TABLE_LEN,
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

fn require_worker() -> bool {
    std::env::var("VERDANT_OMEGA_REQUIRE_WORKER")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

fn worker_or_skip() -> Option<OmegaNeuralWorker> {
    let path = default_worker_binary_path();
    if !path.is_file() {
        assert!(
            !require_worker(),
            "VERDANT_OMEGA_REQUIRE_WORKER=1 but pure-Omega worker binary is missing at {}; run `sh om/build.sh`",
            path.display()
        );
        eprintln!(
            "SKIP: pure-Omega worker binary not built at {} (run `sh om/build.sh` to execute live Omega parity tests)",
            path.display()
        );
        return None;
    }
    Some(
        OmegaNeuralWorker::spawn(&path)
            .expect("pure-Omega worker binary present but failed to spawn or answer OP_PING"),
    )
}

#[test]
fn lif_ssm_bit_exact_parity_across_shapes_and_rollout() {
    let Some(mut worker) = worker_or_skip() else {
        return;
    };
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
    let Some(mut worker) = worker_or_skip() else {
        return;
    };
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
    let Some(mut worker) = worker_or_skip() else {
        return;
    };
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
    let Some(mut worker) = worker_or_skip() else {
        return;
    };

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
fn compensating_slice_lengths_are_refused_before_framing() {
    let Some(mut worker) = worker_or_skip() else {
        return;
    };
    let cb = [0_i32; CODED_TABLE_LEN];

    // Exact Codex PR #71 P1 repro: short `a=[]` compensated by long `b=[1024, 1024]`.
    assert!(matches!(
        worker.step_neuromorphic_lif_ssm_ring(&[1], &[1], &[], &[1024, 1024], &[1024], 1),
        Err(TensorComputeError::ZeroOrMismatchedExtents { .. })
    ));

    // Compensating slices in coded_gemm SLM (`w1` short, `w2` long).
    assert!(matches!(
        worker.infer_coded_slm_intent_logits(&[1, 2], 1, 2, 2, 1, &cb, &[], &[0, 0, 0]),
        Err(TensorComputeError::ZeroOrMismatchedExtents { .. })
    ));

    // Compensating slices in coded_pair_lut (`lhs` short, `rhs` long).
    assert!(matches!(
        worker.contract_coded_pair_lut(&[0], &[0, 0, 0], 1, 2, 1, &cb, &cb),
        Err(TensorComputeError::ZeroOrMismatchedExtents { .. })
    ));

    // Compensating slices in packed_nibbles_lut (`lhs` short, `rhs` long).
    assert!(matches!(
        worker.contract_packed_nibbles_lut(&[], &[0, 0], 1, 2, 1, &cb, &cb),
        Err(TensorComputeError::ZeroOrMismatchedExtents { .. })
    ));

    // Compensating slices in JIT compiler (`w1` short, `w2` long).
    let tmp = std::env::temp_dir().join("verdant_omega_never_created");
    assert!(matches!(
        OmegaCompiledSlm::compile_and_spawn(2, 2, 1, &cb, &[], &[0, 0, 0], &tmp),
        Err(TensorComputeError::ZeroOrMismatchedExtents { .. })
    ));
}

#[test]
fn unlocked_omc_binary_is_refused_by_r6_lock() {
    let fake_omc =
        std::env::temp_dir().join(format!("verdant_omega_fake_omc_{}", std::process::id()));
    std::fs::write(&fake_omc, b"not-the-locked-omc-binary").unwrap();
    let err = verify_locked_omc(&fake_omc).unwrap_err();
    assert!(matches!(
        err,
        TensorComputeError::InvalidKernelSpecification { ref kernel_name }
            if kernel_name.contains("omc_digest_mismatch")
    ));

    let cb = [0_i32; CODED_TABLE_LEN];
    let tmp_bin = std::env::temp_dir().join("verdant_omega_fake_out");
    let err2 = OmegaCompiledSlm::compile_and_spawn_with_omc(
        2,
        2,
        1,
        &cb,
        &[0, 0],
        &[0, 0],
        &tmp_bin,
        &fake_omc,
    )
    .err()
    .expect("unlocked omc must be refused");
    assert!(matches!(
        err2,
        TensorComputeError::InvalidKernelSpecification { ref kernel_name }
            if kernel_name.contains("omc_digest_mismatch")
    ));
    let _ = std::fs::remove_file(&fake_omc);
}

#[test]
fn bare_relative_output_path_spawns_cleanly() {
    let jit_path = default_slm_jit_binary_path();
    if !jit_path.is_file() {
        assert!(
            !require_worker(),
            "VERDANT_OMEGA_REQUIRE_WORKER=1 but verdant-slm-jit missing"
        );
        return;
    }
    let cb: [i32; CODED_TABLE_LEN] = core::array::from_fn(|i| ((i as i32) - 8) * 16);
    let bare_name = format!("verdant_omega_bare_rel_{}", std::process::id());
    let mut jit =
        OmegaCompiledSlm::compile_and_spawn(2, 2, 1, &cb, &[0x8f, 0xf8], &[15, 8], &bare_name)
            .expect("bare relative output filename must resolve and spawn cleanly");
    let out = jit.infer_batch(&[64, 64], 1).unwrap();
    let expected =
        infer_coded_slm_intent_logits(&[64, 64], 1, 2, 2, 1, &cb, &[0x8f, 0xf8], &[15, 8]).unwrap();
    assert_eq!(out, expected);
    let bin_path = jit.binary_path().to_path_buf();
    drop(jit);
    let _ = std::fs::remove_file(&bin_path);
}

#[test]
fn coded_lut_and_packed_nibbles_bit_exact_parity() {
    let Some(mut worker) = worker_or_skip() else {
        return;
    };
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
    let Some(mut worker) = worker_or_skip() else {
        return;
    };
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
    let jit_path = default_slm_jit_binary_path();
    if !jit_path.is_file() {
        assert!(
            !require_worker(),
            "VERDANT_OMEGA_REQUIRE_WORKER=1 but verdant-slm-jit binary is missing at {}; run `sh om/build.sh`",
            jit_path.display()
        );
        eprintln!(
            "SKIP: verdant-slm-jit binary not built at {} (run `sh om/build.sh` to execute live JIT parity tests)",
            jit_path.display()
        );
        return;
    }
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

#[test]
fn raster_tile_pbr_matches_verdant_tensor_bit_for_bit() {
    let raster_path = default_raster_binary_path();
    if !raster_path.is_file() {
        assert!(
            !require_worker(),
            "VERDANT_OMEGA_REQUIRE_WORKER=1 but verdant-raster binary is missing at {}; run `sh om/build.sh`",
            raster_path.display()
        );
        eprintln!(
            "SKIP: verdant-raster binary not built at {} (run `sh om/build.sh`)",
            raster_path.display()
        );
        return;
    }

    let mut worker = OmegaRasterWorker::spawn(&raster_path).expect("spawn verdant-raster");
    let vp: [[f64; 4]; 4] = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];
    let sun = [0.4, 0.85, 0.35];
    let tris = vec![
        RasterTriangle {
            positions: [[-0.6, -0.5, 0.2], [0.6, -0.4, 0.3], [0.0, 0.65, 0.1]],
            normal: [0.0, 0.5, 0.866],
            base_color: [0.82, 0.45, 0.22],
        },
        RasterTriangle {
            positions: [[-0.3, -0.2, 0.05], [0.75, -0.1, 0.4], [0.15, 0.5, 0.15]],
            normal: [0.25, 0.75, 0.6],
            base_color: [0.2, 0.7, 0.85],
        },
    ];

    for lod_rank in [0_u64, 2_u64, 4_u64] {
        let expected =
            render_triangles_tile_framebuffer(64, 64, &vp, sun, lod_rank, &tris).unwrap();
        let actual = worker
            .render_triangles_tile_framebuffer(64, 64, &vp, sun, lod_rank, &tris)
            .unwrap();
        assert_eq!(actual.tiles_shaded, expected.tiles_shaded);
        assert_eq!(actual.rasterized_triangles, expected.rasterized_triangles);
        assert_eq!(
            actual.rgba8, expected.rgba8,
            "RGBA8 drift at lod={lod_rank}"
        );
        assert_eq!(
            actual.depth, expected.depth,
            "depth drift at lod={lod_rank}"
        );
    }

    // Negative control: non-multiple-of-16 dimensions must be refused.
    let err = worker
        .render_triangles_tile_framebuffer(30, 32, &vp, sun, 0, &tris)
        .unwrap_err();
    assert!(matches!(
        err,
        TensorComputeError::InvalidTileDimensions { .. }
    ));
}
