//! Head-to-head `--release` benchmark comparing `verdant-tensor`'s generic `coded_gemm` SLM
//! against Omega's self-compiled, zero-weight-RAM, algebraically zero-erased native executable
//! (`om/verdant_slm_jit.om` -> `omc` -> native Mach-O/ELF).

use std::time::Instant;

use verdant_omega::OmegaCompiledSlm;
use verdant_tensor::{infer_coded_slm_intent_logits, CODED_TABLE_LEN};

fn main() {
    // Exact Verdant CodedSlmDirector model (32x32x16, 1536 weights, 1440 zeros = 93.75% sparse).
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
        std::env::temp_dir().join(format!("verdant_omega_jit_bench_{}", std::process::id()));
    let mut jit = OmegaCompiledSlm::compile_and_spawn(
        embed_dim,
        hidden_dim,
        intent_count,
        &codebook,
        &w1,
        &w2,
        &tmp_bin,
    )
    .expect("compile_and_spawn failed");

    println!(
        "OMEGA SELF-COMPILED SLM: om_src={} B (synth {:.2} ms) -> native_bin={} B (omc {:.2} ms)",
        jit.synthesized_om_bytes(),
        jit.synth_micros() as f64 / 1e3,
        jit.compiled_binary_bytes(),
        jit.compile_micros() as f64 / 1e3,
    );

    let batch_rows = 256;
    let feats: Vec<i32> = (0..batch_rows * embed_dim)
        .map(|i| ((i as i32) % 41) * 17 - 300)
        .collect();

    let ref_out = infer_coded_slm_intent_logits(
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
    let jit_out = jit.infer_batch(&feats, batch_rows).unwrap();
    assert_eq!(jit_out, ref_out, "PARITY MISMATCH");

    let iters = 400_u32;
    let t0 = Instant::now();
    for _ in 0..iters {
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
    let rust_us_batch = t0.elapsed().as_secs_f64() * 1e6 / f64::from(iters);

    let t1 = Instant::now();
    for _ in 0..iters {
        let out = jit.infer_batch(&feats, batch_rows).unwrap();
        std::hint::black_box(out);
    }
    let jit_us_batch = t1.elapsed().as_secs_f64() * 1e6 / f64::from(iters);

    let rust_ns_prompt = rust_us_batch * 1000.0 / (batch_rows as f64);
    let jit_ns_prompt = jit_us_batch * 1000.0 / (batch_rows as f64);
    println!(
        "PARITY=EXACT (256 prompts/batch x {iters} iters = {} inferences)",
        u64::from(iters) * (batch_rows as u64)
    );
    println!(
        "verdant-tensor (--release LLVM): {rust_us_batch:.1} us/batch ({rust_ns_prompt:.0} ns/prompt)"
    );
    println!(
        "omega-slm-jit  (zero-weight-RAM): {jit_us_batch:.1} us/batch ({jit_ns_prompt:.0} ns/prompt) => {:.2}x FASTER",
        rust_us_batch / jit_us_batch
    );

    drop(jit);
    let _ = std::fs::remove_file(&tmp_bin);
}
