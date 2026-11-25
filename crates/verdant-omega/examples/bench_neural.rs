//! Steady-state latency benchmark comparing the persistent pure-Omega neural worker
//! against `verdant-tensor`'s Rust implementation with bit-exact parity verification.

use std::time::Instant;

use verdant_omega::OmegaNeuralWorker;
use verdant_tensor::{
    contract_packed_nibbles_lut_into, infer_coded_slm_intent_logits,
    step_neuromorphic_lif_ssm_ring, tropical_min_plus_all_pairs_closure, CodebookProductLut,
    CODED_TABLE_LEN,
};

fn main() {
    let mut worker = OmegaNeuralWorker::spawn_default()
        .expect("build the Omega worker first with `sh om/build.sh`");

    // 1. kernel10: Q15 LIF/SSM (state_dim=16, obs_dim=16, entity_slots=32)
    let state_dim = 16;
    let obs_dim = 16;
    let entity_slots = 32;
    let prev: Vec<i32> = (0..state_dim).map(|i| (i as i32) * 37 - 200).collect();
    let obs: Vec<i32> = (0..obs_dim).map(|i| (i as i32) * 29 - 150).collect();
    let a: Vec<i32> = (0..state_dim * state_dim)
        .map(|i| ((i as i32) % 19) * 16 - 140)
        .collect();
    let b: Vec<i32> = (0..state_dim * obs_dim)
        .map(|i| ((i as i32) % 17) * 16 - 120)
        .collect();
    let c: Vec<i32> = (0..entity_slots * state_dim)
        .map(|i| ((i as i32) % 23) * 16 - 160)
        .collect();

    let ref_lif = step_neuromorphic_lif_ssm_ring(&prev, &obs, &a, &b, &c, entity_slots).unwrap();
    let om_lif = worker
        .step_neuromorphic_lif_ssm_ring(&prev, &obs, &a, &b, &c, entity_slots)
        .unwrap();
    assert_eq!(om_lif, ref_lif, "LIF/SSM parity mismatch before benchmark");

    let lif_iters = 2000_u32;
    let t0 = Instant::now();
    for _ in 0..lif_iters {
        let out = step_neuromorphic_lif_ssm_ring(&prev, &obs, &a, &b, &c, entity_slots).unwrap();
        std::hint::black_box(out);
    }
    let rust_lif_us = t0.elapsed().as_secs_f64() * 1e6 / f64::from(lif_iters);

    let t1 = Instant::now();
    for _ in 0..lif_iters {
        let out = worker
            .step_neuromorphic_lif_ssm_ring(&prev, &obs, &a, &b, &c, entity_slots)
            .unwrap();
        std::hint::black_box(out);
    }
    let om_lif_us = t1.elapsed().as_secs_f64() * 1e6 / f64::from(lif_iters);

    // 2. kernel11: coded_gemm SLM (batch=16, embed=64, hidden=64, intents=16)
    let batch_rows = 16;
    let embed_dim = 64;
    let hidden_dim = 64;
    let intent_count = 16;
    let codebook: [i32; CODED_TABLE_LEN] = core::array::from_fn(|i| ((i as i32) - 8) * 16);
    let feats: Vec<i32> = (0..batch_rows * embed_dim)
        .map(|i| ((i as i32) % 31) * 12 - 180)
        .collect();
    let w1: Vec<u8> = (0..embed_dim * (hidden_dim / 2))
        .map(|i| (((i * 7) & 0xf) | (((i * 13) & 0xf) << 4)) as u8)
        .collect();
    let w2: Vec<u16> = (0..hidden_dim * intent_count)
        .map(|i| (i % CODED_TABLE_LEN) as u16)
        .collect();

    let ref_slm = infer_coded_slm_intent_logits(
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
    let om_slm = worker
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
        om_slm, ref_slm,
        "coded_gemm SLM parity mismatch before benchmark"
    );

    let slm_iters = 500_u32;
    let t2 = Instant::now();
    for _ in 0..slm_iters {
        let out = infer_coded_slm_intent_logits(
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
        std::hint::black_box(out);
    }
    let rust_slm_us = t2.elapsed().as_secs_f64() * 1e6 / f64::from(slm_iters);

    let t3 = Instant::now();
    for _ in 0..slm_iters {
        let out = worker
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
        std::hint::black_box(out);
    }
    let om_slm_us = t3.elapsed().as_secs_f64() * 1e6 / f64::from(slm_iters);

    println!("PARITY=EXACT (LIF/SSM + coded_gemm SLM + packed_lut + tropical_min_plus)");
    println!(
        "kernel10 LIF/SSM (16x16, 32 slots, {lif_iters} iters): verdant-tensor={rust_lif_us:.1} us/call | omega-worker={om_lif_us:.1} us/call"
    );
    println!(
        "kernel11 coded_gemm SLM (16x64x64x16, {slm_iters} iters): verdant-tensor={rust_slm_us:.1} us/call | omega-worker={om_slm_us:.1} us/call"
    );

    // 3. Packed nibble LUT contraction (32x128x32)
    let (pr, pk_inner, pc) = (32, 128, 32);
    let pk_bytes = pk_inner / 2;
    let lhs_cb: [i32; CODED_TABLE_LEN] = core::array::from_fn(|i| ((i as i32) - 8) * 16);
    let rhs_cb: [i32; CODED_TABLE_LEN] = core::array::from_fn(|i| ((i as i32) - 8) * 12);
    let lut = CodebookProductLut::new(&lhs_cb, &rhs_cb);
    let lhs_p: Vec<u8> = (0..pr * pk_bytes)
        .map(|i| ((i * 37) & 0xff) as u8)
        .collect();
    let rhs_p: Vec<u8> = (0..pc * pk_bytes)
        .map(|i| ((i * 53) & 0xff) as u8)
        .collect();
    let mut ref_lut = vec![0_i64; pr * pc];
    contract_packed_nibbles_lut_into(&lhs_p, &rhs_p, pr, pk_inner, pc, &lut, &mut ref_lut).unwrap();
    let om_lut = worker
        .contract_packed_nibbles_lut(&lhs_p, &rhs_p, pr, pk_inner, pc, &lhs_cb, &rhs_cb)
        .unwrap();
    assert_eq!(om_lut, ref_lut, "packed LUT parity mismatch");
    let lut_iters = 500_u32;
    let t4 = Instant::now();
    for _ in 0..lut_iters {
        contract_packed_nibbles_lut_into(&lhs_p, &rhs_p, pr, pk_inner, pc, &lut, &mut ref_lut)
            .unwrap();
        std::hint::black_box(&ref_lut);
    }
    let rust_lut_us = t4.elapsed().as_secs_f64() * 1e6 / f64::from(lut_iters);
    let t5 = Instant::now();
    for _ in 0..lut_iters {
        let out = worker
            .contract_packed_nibbles_lut(&lhs_p, &rhs_p, pr, pk_inner, pc, &lhs_cb, &rhs_cb)
            .unwrap();
        std::hint::black_box(out);
    }
    let om_lut_us = t5.elapsed().as_secs_f64() * 1e6 / f64::from(lut_iters);
    println!(
        "coded packed_nibbles_lut (32x128x32, {lut_iters} iters): verdant-tensor={rust_lut_us:.1} us/call | omega-worker={om_lut_us:.1} us/call"
    );

    // 4. Tropical (min, +) all-pairs closure (32 nodes)
    let tn = 32;
    let edges: Vec<Option<u32>> = (0..tn * tn)
        .map(|i| {
            if i % 3 == 0 {
                Some(((i * 19) % 5000) as u32)
            } else {
                None
            }
        })
        .collect();
    let ref_trop = tropical_min_plus_all_pairs_closure(&edges, tn).unwrap();
    let om_trop = worker
        .tropical_min_plus_all_pairs_closure(&edges, tn)
        .unwrap();
    assert_eq!(om_trop, ref_trop, "tropical min-plus parity mismatch");
    let trop_iters = 200_u32;
    let t6 = Instant::now();
    for _ in 0..trop_iters {
        let out = tropical_min_plus_all_pairs_closure(&edges, tn).unwrap();
        std::hint::black_box(out);
    }
    let rust_trop_us = t6.elapsed().as_secs_f64() * 1e6 / f64::from(trop_iters);
    let t7 = Instant::now();
    for _ in 0..trop_iters {
        let out = worker
            .tropical_min_plus_all_pairs_closure(&edges, tn)
            .unwrap();
        std::hint::black_box(out);
    }
    let om_trop_us = t7.elapsed().as_secs_f64() * 1e6 / f64::from(trop_iters);
    println!(
        "tropical min-plus closure (32 nodes, {trop_iters} iters): verdant-tensor={rust_trop_us:.1} us/call | omega-worker={om_trop_us:.1} us/call"
    );
}
