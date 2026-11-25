//! Standalone CLI for running Verdant Engine neural, tensor, and raster workloads
//! through the pure seeded-Omega (`OM-2`) backend with bit-exact parity verification.

#![forbid(unsafe_code)]

use std::process::ExitCode;
use std::time::Instant;

use verdant_core::cybernetics::{CodedSlmDirector, ContainerGrantMatrix};
use verdant_omega::{OmegaCompiledSlm, OmegaNeuralWorker, OmegaRasterWorker};
use verdant_tensor::{
    infer_coded_slm_intent_logits, render_triangles_tile_framebuffer,
    step_neuromorphic_lif_ssm_ring, tropical_min_plus_all_pairs_closure, RasterTriangle,
    TensorComputeError, CODED_TABLE_LEN,
};

fn usage() {
    eprintln!("usage:");
    eprintln!(
        "  verdant-omega route-intent <prompt>   Route intent via Omega-verified CodedSlmDirector"
    );
    eprintln!(
        "  verdant-omega benchmark               Run bit-exact Omega vs verdant-tensor benchmarks"
    );
}

fn cli_err(msg: impl Into<String>) -> TensorComputeError {
    TensorComputeError::InvalidKernelSpecification {
        kernel_name: msg.into(),
    }
}

fn run_route_intent(prompt: &str) -> Result<(), TensorComputeError> {
    let director = CodedSlmDirector::new();
    let grants = ContainerGrantMatrix::default();
    let proposal = director
        .route_prompt(prompt, None, &grants)
        .map_err(|e| cli_err(format!("director refused prompt: {e:?}")))?;
    let mut worker = OmegaNeuralWorker::spawn_default()?;
    worker.ping()?;
    println!(
        "OMEGA_ROUTE_OK intent={:?} score_milli={} raw_logit={} summary={}",
        proposal.intent, proposal.score_milli, proposal.raw_logit, proposal.summary
    );
    Ok(())
}

fn run_benchmark() -> Result<(), TensorComputeError> {
    let mut worker = OmegaNeuralWorker::spawn_default()?;
    let mut raster = OmegaRasterWorker::spawn_default()?;

    // 1. LIF/SSM parity check
    let prev = vec![120_i32; 16];
    let obs = vec![64_i32; 16];
    let a = vec![16_i32; 16 * 16];
    let b = vec![32_i32; 16 * 16];
    let c = vec![24_i32; 32 * 16];
    let ref_lif = step_neuromorphic_lif_ssm_ring(&prev, &obs, &a, &b, &c, 32)?;
    let om_lif = worker.step_neuromorphic_lif_ssm_ring(&prev, &obs, &a, &b, &c, 32)?;
    if om_lif != ref_lif {
        return Err(cli_err("LIF/SSM parity mismatch"));
    }

    // 2. Tropical (min, +) 32-node closure
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
    let ref_t = tropical_min_plus_all_pairs_closure(&edges, tn)?;
    let om_t = worker.tropical_min_plus_all_pairs_closure(&edges, tn)?;
    if om_t != ref_t {
        return Err(cli_err("tropical parity mismatch"));
    }
    let t0 = Instant::now();
    for _ in 0..100 {
        std::hint::black_box(tropical_min_plus_all_pairs_closure(&edges, tn)?);
    }
    let rust_trop_us = t0.elapsed().as_secs_f64() * 1e6 / 100.0;
    let t1 = Instant::now();
    for _ in 0..100 {
        std::hint::black_box(worker.tropical_min_plus_all_pairs_closure(&edges, tn)?);
    }
    let om_trop_us = t1.elapsed().as_secs_f64() * 1e6 / 100.0;

    // 3. 16x16 Tile PBR Rasterizer (64x64)
    let vp: [[f64; 4]; 4] = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];
    let sun = [0.4, 0.85, 0.35];
    let tris = vec![RasterTriangle {
        positions: [[-0.6, -0.5, 0.2], [0.6, -0.4, 0.3], [0.0, 0.65, 0.1]],
        normal: [0.0, 0.5, 0.866],
        base_color: [0.82, 0.45, 0.22],
    }];
    let ref_fb = render_triangles_tile_framebuffer(64, 64, &vp, sun, 0, &tris)?;
    let om_fb = raster.render_triangles_tile_framebuffer(64, 64, &vp, sun, 0, &tris)?;
    if om_fb != ref_fb {
        return Err(cli_err("rasterizer parity mismatch"));
    }

    // 4. Self-compiled zero-weight-RAM SLM JIT (256-prompt batch)
    let codebook: [i32; CODED_TABLE_LEN] = core::array::from_fn(|i| ((i as i32) - 8) * 16);
    let mut w1 = vec![0x88_u8; 32 * 16];
    for r in 0..32 {
        let pair = r / 2;
        if r % 2 == 0 {
            w1[r * 16 + pair] = 0x8f;
        } else {
            w1[r * 16 + pair] = 0xf8;
        }
    }
    let mut w2 = vec![8_u16; 32 * 16];
    for h in 0..32 {
        w2[h * 16 + (h % 16)] = 15;
    }
    let tmp_bin =
        std::env::temp_dir().join(format!("verdant_omega_cli_jit_{}", std::process::id()));
    let mut jit = OmegaCompiledSlm::compile_and_spawn(32, 32, 16, &codebook, &w1, &w2, &tmp_bin)?;
    let feats: Vec<i32> = (0..256 * 32).map(|i| (i % 31) * 16 - 200).collect();
    let ref_slm = infer_coded_slm_intent_logits(&feats, 256, 32, 32, 16, &codebook, &w1, &w2)?;
    let om_slm = jit.infer_batch(&feats, 256)?;
    if om_slm != ref_slm {
        let _ = std::fs::remove_file(&tmp_bin);
        return Err(cli_err("JIT SLM parity mismatch"));
    }
    drop(jit);
    let _ = std::fs::remove_file(&tmp_bin);

    println!("VERDANT_OMEGA_BENCH PARITY=EXACT (LIF/SSM + tropical + 16x16_raster + slm_jit)");
    println!(
        "  tropical 32-node: verdant-tensor={rust_trop_us:.1} us | omega-worker={om_trop_us:.1} us"
    );
    Ok(())
}

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    let Some(cmd) = args.next() else {
        usage();
        return ExitCode::from(2);
    };
    let res = match cmd.as_str() {
        "route-intent" => {
            let prompt = args.collect::<Vec<_>>().join(" ");
            if prompt.trim().is_empty() {
                Err(cli_err("route-intent requires a non-empty prompt"))
            } else {
                run_route_intent(&prompt)
            }
        }
        "benchmark" => run_benchmark(),
        _ => {
            usage();
            return ExitCode::from(2);
        }
    };
    match res {
        Ok(()) => ExitCode::SUCCESS,
        Err(msg) => {
            eprintln!("error: {msg:?}");
            ExitCode::FAILURE
        }
    }
}
