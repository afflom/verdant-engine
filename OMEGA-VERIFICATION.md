# Omega Backend Verification (`VE-82`)

Project-owned verification evidence for the opt-in pure seeded-Omega (`OM-2`) native backend
(`crates/verdant-omega/` and `om/`), registered as open (measured) capability `VE-82` in
`model/ids.toml` and `features/suites/engine.feature`.

## Locked Compiler Dependency (`R6`)

Both `om/build.sh` and `verdant_omega::OmegaCompiledSlm` verify the SHA-256 digest of the
resolved `omc` compiler binary against `om/omc.lock` before compiling any native executable:

| Platform | Locked `omc` SHA-256 (`om/omc.lock`) |
| --- | --- |
| `darwin-arm64` | `68a590c37dfb1ec37a183f0032e02db1570db299266f574f65556edfc70feb9c` |
| `linux-aarch64` | `28fca13d44054e26f78c33d1753fd5c005d08e9144d95985c90bd75b60885072` |

Any compiler binary whose SHA-256 differs from `om/omc.lock` is refused (`om/build.sh` exits `3`;
`OmegaCompiledSlm::compile_and_spawn` returns `TensorComputeError::InvalidKernelSpecification`).

## Planted-Defect Gate Evidence (`AGENTS.md` Gate Quality)

- **Planted defect:** Perturbed `slm_dot_stage2` in `om/verdant_neural.om` from
  `ret bitcast long acc` to `ret add (bitcast long acc) long 1` (off-by-1 logit drift) and rebuilt
  `om/bin/verdant-neural-macos` (`bin_sha256=baf780aeea5914a636f5617d49f18ca146ec70f8b2c73d147f7c17200d6ffcc9`).
- **Failing command:**
  ```sh
  VERDANT_OMEGA_REQUIRE_WORKER=1 cargo test -p verdant-omega --test parity_and_refusals \
    coded_slm_bit_exact_parity_across_batches_and_real_shapes -- --nocapture
  ```
- **Observed failure (`exit 101`):**
  ```text
  thread 'coded_slm_bit_exact_parity_across_batches_and_real_shapes' panicked at crates/verdant-omega/tests/parity_and_refusals.rs:222:9:
  assertion `left == right` failed: coded_gemm SLM parity drift at (1,32,32,16)
    left: [9251883, -19114096, 373531, -7209404, 10122453, -7052443, -12170565, -4803275, 8209052, 6761468, -17315394, -1495313, 821464, -6811927, -9344468, 10399808]
   right: [9251882, -19114097, 373530, -7209405, 10122452, -7052444, -12170566, -4803276, 8209051, 6761467, -17315395, -1495314, 821463, -6811928, -9344469, 10399807]
  test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 10 filtered out
  ```
- **Restored result (`exit 0`):** After reverting `om/verdant_neural.om` and rebuilding
  (`bin_sha256=468e7f450631e5249d57b0c7b80f349467bf853d298b28e65ff8f2a55df5eb6a`), the exact same
  command passed (`test result: ok. 1 passed; 0 failed`).

## Cross-Platform & Silicon Parity Receipts

- **3-way cross-platform determinism (`280 B` 6-op stdout stream across macOS ARM64 Mach-O,
  DGX Spark Linux AArch64 ELF, and Linux x86-64 ELF under `qemu-x86_64`):**
  `sha256 = f8f2d252c66be752026d66476e446ec6cb8e15cf17383d3209f36adcb06960e3`
- **3-way identical synthesized zero-weight-RAM `.om` (`13,725 B`):**
  `sha256 = 170564f85e9e8a9b55aff366399a1d3d1b4e3bb6c0a7a9c4be4dd22e60576ec7`
- **DGX Spark NVIDIA GB10 (SM121) Zero-CUDA `coded_gemm` (`512×512×512`, `134,217,728` exact
  integer LUT MACs, `spark-window` `20261125T215713Z-a22a25`):**
  `PARITY=EXACT` (`mismatches=0/262144`), `cpu_us=176977`, `gpu_us=1327` (`133.4×` speedup).
