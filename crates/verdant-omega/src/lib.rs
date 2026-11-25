//! Safe Rust client for the pure seeded-Omega (`OM-2`) native neural worker.
//!
//! Communicates with `om/bin/verdant-neural` over a persistent little-endian binary `stdin`/`stdout`
//! pipe, preserving `#![forbid(unsafe_code)]` across the entire `verdant-engine` workspace while
//! delegating `kernel10` (`step_neuromorphic_lif_ssm_ring`) and `kernel11`
//! (`infer_coded_slm_intent_logits`) to a compiled pure-Omega native executable.
//!
//! All public methods return [`verdant_tensor::TensorComputeError`] (registered in `model/ids.toml`
//! under `R5`) so the Omega backend matches `verdant-tensor`'s exact refusal contract.

#![forbid(unsafe_code)]

use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

use verdant_tensor::{prism_kernel, NeuromorphicStepOutput, TensorComputeError, CODED_TABLE_LEN};

/// Wire protocol magic (`"VOM1"` little-endian).
pub const WIRE_MAGIC: u32 = 0x564f_4d31;
/// Wire protocol magic for the pure-Omega partial-evaluating SLM compiler (`"VOMJ"`).
pub const JIT_MAGIC: u32 = 0x564f_4d4a;
/// Wire operation code for `step_neuromorphic_lif_ssm_ring` (`kernel10`).
pub const OP_LIF_SSM: u32 = 1;
/// Wire operation code for `infer_coded_slm_intent_logits` (`kernel11`).
pub const OP_CODED_SLM: u32 = 2;
/// Wire operation code for a zero-payload liveness ping.
pub const OP_PING: u32 = 3;
/// Wire operation code for `contract_coded_pair_lut_into` (`16x16` codebook LUT contraction).
pub const OP_CODED_PAIR_LUT: u32 = 4;
/// Wire operation code for `contract_packed_nibbles_lut_into` (4-bit packed nibble LUT contraction).
pub const OP_PACKED_NIBBLES_LUT: u32 = 5;
/// Wire operation code for `tropical_min_plus_all_pairs_closure` (NavMesh/Chronicle min-plus closure).
pub const OP_TROPICAL_MIN_PLUS: u32 = 6;

/// Worker reply status: success.
pub const STATUS_OK: u32 = 0;
/// Worker reply status: invalid frame magic.
pub const STATUS_BAD_MAGIC: u32 = 1;
/// Worker reply status: unknown operation code.
pub const STATUS_UNKNOWN_OP: u32 = 2;
/// Worker reply status: zero or mismatched matrix extents.
pub const STATUS_INVALID_EXTENTS: u32 = 3;
/// Worker reply status: codebook index out of range (`>= 16`).
pub const STATUS_INVALID_CODEBOOK: u32 = 4;
/// Worker reply status: payload byte length does not match header dimensions.
pub const STATUS_PAYLOAD_MISMATCH: u32 = 5;
/// Worker reply status: invalid kernel parameter (`decay_den`, `threshold`, or shift).
pub const STATUS_INVALID_PARAMS: u32 = 6;

const HEADER_BYTES: usize = 12;
const LIF_SCALE_SHIFT: u32 = 10;
const SLM_RELU_SHIFT: u32 = 6;

fn io_err(err: io::Error) -> TensorComputeError {
    TensorComputeError::InvalidKernelSpecification {
        kernel_name: format!("omega_worker_io:{err}"),
    }
}

fn status_err(status: u32) -> TensorComputeError {
    match status {
        STATUS_INVALID_EXTENTS => TensorComputeError::ZeroOrMismatchedExtents {
            detail: "omega_worker:invalid_extents".to_string(),
        },
        STATUS_INVALID_CODEBOOK => TensorComputeError::ZeroOrMismatchedExtents {
            detail: "omega_worker:invalid_codebook_index".to_string(),
        },
        STATUS_PAYLOAD_MISMATCH => TensorComputeError::ZeroOrMismatchedExtents {
            detail: "omega_worker:payload_mismatch".to_string(),
        },
        _ => TensorComputeError::InvalidKernelSpecification {
            kernel_name: format!("omega_worker:refused_status_{status}"),
        },
    }
}

/// Resolves the default compiled pure-Omega neural worker binary path.
///
/// Checks `VERDANT_OMEGA_WORKER` first, then `<workspace_root>/om/bin/verdant-neural`.
#[must_use]
pub fn default_worker_binary_path() -> PathBuf {
    if let Ok(path) = std::env::var("VERDANT_OMEGA_WORKER") {
        return PathBuf::from(path);
    }
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("om/bin/verdant-neural")
}

/// Resolves the default pure-Omega partial-evaluating SLM compiler (`verdant-slm-jit`).
///
/// Checks `VERDANT_OMEGA_SLM_JIT` first, then `<workspace_root>/om/bin/verdant-slm-jit`.
#[must_use]
pub fn default_slm_jit_binary_path() -> PathBuf {
    if let Ok(path) = std::env::var("VERDANT_OMEGA_SLM_JIT") {
        return PathBuf::from(path);
    }
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("om/bin/verdant-slm-jit")
}

/// Resolves the native `omc` compiler binary path (`OMC`, `~/.local/bin/omc`, or Spark `~/canon/omega-mv/seed/out/omc`).
#[must_use]
pub fn default_omc_binary_path() -> PathBuf {
    if let Ok(path) = std::env::var("OMC") {
        return PathBuf::from(path);
    }
    if let Ok(home) = std::env::var("HOME") {
        let mac = Path::new(&home).join(".local/bin/omc");
        if mac.is_file() {
            return mac;
        }
        let spark = Path::new(&home).join("canon/omega-mv/seed/out/omc");
        if spark.is_file() {
            return spark;
        }
    }
    PathBuf::from("omc")
}

/// Persistent handle to the compiled pure-Omega (`OM-2`) neural worker process.
pub struct OmegaNeuralWorker {
    child: Child,
    stdin: Option<ChildStdin>,
    stdout: ChildStdout,
}

impl OmegaNeuralWorker {
    /// Spawns the pure-Omega neural worker at `binary_path` and verifies it with an `OP_PING`.
    pub fn spawn(binary_path: impl AsRef<Path>) -> Result<Self, TensorComputeError> {
        let mut child = Command::new(binary_path.as_ref())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .map_err(io_err)?;
        let stdin = child
            .stdin
            .take()
            .ok_or_else(|| io_err(io::Error::other("failed to capture omega worker stdin")))?;
        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| io_err(io::Error::other("failed to capture omega worker stdout")))?;
        let mut worker = Self {
            child,
            stdin: Some(stdin),
            stdout,
        };
        worker.ping()?;
        Ok(worker)
    }

    /// Spawns the worker using [`default_worker_binary_path`].
    pub fn spawn_default() -> Result<Self, TensorComputeError> {
        Self::spawn(default_worker_binary_path())
    }

    /// Sends a raw framed message to the worker and returns `(status, reply_payload)`.
    pub fn send_raw_frame(
        &mut self,
        magic: u32,
        op: u32,
        payload: &[u8],
    ) -> Result<(u32, Vec<u8>), TensorComputeError> {
        let plen = u32::try_from(payload.len()).map_err(|_| {
            TensorComputeError::ZeroOrMismatchedExtents {
                detail: "omega_worker:payload_exceeds_u32".to_string(),
            }
        })?;
        let mut hdr = [0_u8; HEADER_BYTES];
        hdr[0..4].copy_from_slice(&magic.to_le_bytes());
        hdr[4..8].copy_from_slice(&op.to_le_bytes());
        hdr[8..12].copy_from_slice(&plen.to_le_bytes());

        let stdin = self.stdin.as_mut().ok_or_else(|| {
            io_err(io::Error::new(
                io::ErrorKind::BrokenPipe,
                "worker stdin closed",
            ))
        })?;
        stdin.write_all(&hdr).map_err(io_err)?;
        if !payload.is_empty() {
            stdin.write_all(payload).map_err(io_err)?;
        }
        stdin.flush().map_err(io_err)?;

        let mut reply_hdr = [0_u8; HEADER_BYTES];
        self.stdout.read_exact(&mut reply_hdr).map_err(io_err)?;
        let reply_magic =
            u32::from_le_bytes([reply_hdr[0], reply_hdr[1], reply_hdr[2], reply_hdr[3]]);
        if reply_magic != WIRE_MAGIC {
            return Err(TensorComputeError::InvalidKernelSpecification {
                kernel_name: format!("omega_worker:bad_reply_magic_0x{reply_magic:08x}"),
            });
        }
        let status = u32::from_le_bytes([reply_hdr[4], reply_hdr[5], reply_hdr[6], reply_hdr[7]]);
        let reply_len =
            u32::from_le_bytes([reply_hdr[8], reply_hdr[9], reply_hdr[10], reply_hdr[11]]) as usize;

        let mut reply_payload = vec![0_u8; reply_len];
        if reply_len > 0 {
            self.stdout.read_exact(&mut reply_payload).map_err(io_err)?;
        }
        Ok((status, reply_payload))
    }

    /// Sends an `OP_PING` liveness probe to the persistent worker.
    pub fn ping(&mut self) -> Result<(), TensorComputeError> {
        let (status, payload) = self.send_raw_frame(WIRE_MAGIC, OP_PING, &[])?;
        if status != STATUS_OK {
            return Err(status_err(status));
        }
        if !payload.is_empty() {
            return Err(TensorComputeError::ZeroOrMismatchedExtents {
                detail: format!("omega_worker:ping_reply_len_{}", payload.len()),
            });
        }
        Ok(())
    }

    /// Executes `kernel10` (`step_neuromorphic_lif_ssm_ring`) on the pure-Omega worker using
    /// Prism-proved decay and threshold constants from [`verdant_tensor::prism_kernel`].
    pub fn step_neuromorphic_lif_ssm_ring(
        &mut self,
        prev_membrane_milli: &[i32],
        observer_drive_milli: &[i32],
        a_recurrent_q: &[i32],
        b_observer_q: &[i32],
        c_salience_q: &[i32],
        entity_slots: usize,
    ) -> Result<NeuromorphicStepOutput, TensorComputeError> {
        let decay_num = prism_kernel::lifDecayNumer().unwrap_or(1) as i64;
        let decay_den = prism_kernel::lifDecayDenom().unwrap_or(1).max(1) as i64;
        let threshold = prism_kernel::lifSpikeThresholdMilli().unwrap_or(1).max(1) as i64;
        self.step_neuromorphic_lif_ssm_ring_with_params(
            prev_membrane_milli,
            observer_drive_milli,
            a_recurrent_q,
            b_observer_q,
            c_salience_q,
            entity_slots,
            LIF_SCALE_SHIFT,
            decay_num,
            decay_den,
            threshold,
        )
    }

    /// Executes `kernel10` with explicit Prism decay/threshold/shift parameters.
    #[allow(clippy::too_many_arguments)]
    pub fn step_neuromorphic_lif_ssm_ring_with_params(
        &mut self,
        prev_membrane_milli: &[i32],
        observer_drive_milli: &[i32],
        a_recurrent_q: &[i32],
        b_observer_q: &[i32],
        c_salience_q: &[i32],
        entity_slots: usize,
        scale_shift: u32,
        decay_num: i64,
        decay_den: i64,
        threshold: i64,
    ) -> Result<NeuromorphicStepOutput, TensorComputeError> {
        let state_dim = u32::try_from(prev_membrane_milli.len()).map_err(|_| {
            TensorComputeError::ZeroOrMismatchedExtents {
                detail: "omega_worker:state_dim_exceeds_u32".to_string(),
            }
        })?;
        let obs_dim = u32::try_from(observer_drive_milli.len()).map_err(|_| {
            TensorComputeError::ZeroOrMismatchedExtents {
                detail: "omega_worker:obs_dim_exceeds_u32".to_string(),
            }
        })?;
        let eslots = u32::try_from(entity_slots).map_err(|_| {
            TensorComputeError::ZeroOrMismatchedExtents {
                detail: "omega_worker:entity_slots_exceeds_u32".to_string(),
            }
        })?;

        let total_i32s = prev_membrane_milli
            .len()
            .saturating_add(observer_drive_milli.len())
            .saturating_add(a_recurrent_q.len())
            .saturating_add(b_observer_q.len())
            .saturating_add(c_salience_q.len());
        let mut payload = Vec::with_capacity(40 + total_i32s * 4);
        payload.extend_from_slice(&state_dim.to_le_bytes());
        payload.extend_from_slice(&obs_dim.to_le_bytes());
        payload.extend_from_slice(&eslots.to_le_bytes());
        payload.extend_from_slice(&scale_shift.to_le_bytes());
        payload.extend_from_slice(&decay_num.to_le_bytes());
        payload.extend_from_slice(&decay_den.to_le_bytes());
        payload.extend_from_slice(&threshold.to_le_bytes());

        for &v in prev_membrane_milli {
            payload.extend_from_slice(&v.to_le_bytes());
        }
        for &v in observer_drive_milli {
            payload.extend_from_slice(&v.to_le_bytes());
        }
        for &v in a_recurrent_q {
            payload.extend_from_slice(&v.to_le_bytes());
        }
        for &v in b_observer_q {
            payload.extend_from_slice(&v.to_le_bytes());
        }
        for &v in c_salience_q {
            payload.extend_from_slice(&v.to_le_bytes());
        }

        let (status, reply) = self.send_raw_frame(WIRE_MAGIC, OP_LIF_SSM, &payload)?;
        if status != STATUS_OK {
            return Err(status_err(status));
        }

        let expected = 8 + (prev_membrane_milli.len() + entity_slots) * 8;
        if reply.len() != expected {
            return Err(TensorComputeError::ZeroOrMismatchedExtents {
                detail: format!(
                    "omega_worker:lif_reply_len_mismatch expected={expected} actual={}",
                    reply.len()
                ),
            });
        }

        let spike_mask = u64::from_le_bytes([
            reply[0], reply[1], reply[2], reply[3], reply[4], reply[5], reply[6], reply[7],
        ]);
        let mut next_membrane_milli = Vec::with_capacity(prev_membrane_milli.len());
        let mut off = 8;
        for _ in 0..prev_membrane_milli.len() {
            let v = i64::from_le_bytes([
                reply[off],
                reply[off + 1],
                reply[off + 2],
                reply[off + 3],
                reply[off + 4],
                reply[off + 5],
                reply[off + 6],
                reply[off + 7],
            ]);
            next_membrane_milli.push(v);
            off += 8;
        }
        let mut entity_salience_milli = Vec::with_capacity(entity_slots);
        for _ in 0..entity_slots {
            let v = i64::from_le_bytes([
                reply[off],
                reply[off + 1],
                reply[off + 2],
                reply[off + 3],
                reply[off + 4],
                reply[off + 5],
                reply[off + 6],
                reply[off + 7],
            ]);
            entity_salience_milli.push(v);
            off += 8;
        }

        Ok(NeuromorphicStepOutput {
            next_membrane_milli,
            spike_mask,
            entity_salience_milli,
        })
    }

    /// Executes `kernel11` (`infer_coded_slm_intent_logits`) on the pure-Omega worker.
    #[allow(clippy::too_many_arguments)]
    pub fn infer_coded_slm_intent_logits(
        &mut self,
        feature_activations_q: &[i32],
        batch_rows: usize,
        embed_dim: usize,
        hidden_dim: usize,
        intent_count: usize,
        codebook: &[i32; CODED_TABLE_LEN],
        w1_packed_nibbles: &[u8],
        w2_grid_codes: &[u16],
    ) -> Result<Vec<i64>, TensorComputeError> {
        let brows =
            u32::try_from(batch_rows).map_err(|_| TensorComputeError::ZeroOrMismatchedExtents {
                detail: "omega_worker:batch_rows_exceeds_u32".to_string(),
            })?;
        let edim =
            u32::try_from(embed_dim).map_err(|_| TensorComputeError::ZeroOrMismatchedExtents {
                detail: "omega_worker:embed_dim_exceeds_u32".to_string(),
            })?;
        let hdim =
            u32::try_from(hidden_dim).map_err(|_| TensorComputeError::ZeroOrMismatchedExtents {
                detail: "omega_worker:hidden_dim_exceeds_u32".to_string(),
            })?;
        let icnt = u32::try_from(intent_count).map_err(|_| {
            TensorComputeError::ZeroOrMismatchedExtents {
                detail: "omega_worker:intent_count_exceeds_u32".to_string(),
            }
        })?;

        let cap = 24
            + CODED_TABLE_LEN * 4
            + feature_activations_q.len() * 4
            + w1_packed_nibbles.len()
            + w2_grid_codes.len() * 2;
        let mut payload = Vec::with_capacity(cap);
        payload.extend_from_slice(&brows.to_le_bytes());
        payload.extend_from_slice(&edim.to_le_bytes());
        payload.extend_from_slice(&hdim.to_le_bytes());
        payload.extend_from_slice(&icnt.to_le_bytes());
        payload.extend_from_slice(&SLM_RELU_SHIFT.to_le_bytes());
        payload.extend_from_slice(&0_u32.to_le_bytes());

        for &entry in codebook {
            payload.extend_from_slice(&entry.to_le_bytes());
        }
        for &v in feature_activations_q {
            payload.extend_from_slice(&v.to_le_bytes());
        }
        payload.extend_from_slice(w1_packed_nibbles);
        for &code in w2_grid_codes {
            payload.extend_from_slice(&code.to_le_bytes());
        }

        let (status, reply) = self.send_raw_frame(WIRE_MAGIC, OP_CODED_SLM, &payload)?;
        if status != STATUS_OK {
            return Err(status_err(status));
        }

        let out_count = batch_rows.saturating_mul(intent_count);
        let expected = out_count.saturating_mul(8);
        if reply.len() != expected {
            return Err(TensorComputeError::ZeroOrMismatchedExtents {
                detail: format!(
                    "omega_worker:slm_reply_len_mismatch expected={expected} actual={}",
                    reply.len()
                ),
            });
        }

        let mut logits = Vec::with_capacity(out_count);
        for chunk in reply.chunks_exact(8) {
            logits.push(i64::from_le_bytes([
                chunk[0], chunk[1], chunk[2], chunk[3], chunk[4], chunk[5], chunk[6], chunk[7],
            ]));
        }
        Ok(logits)
    }

    /// Executes `contract_coded_pair_lut_into` (`16x16` codebook product LUT) on the pure-Omega worker.
    #[allow(clippy::too_many_arguments)]
    pub fn contract_coded_pair_lut(
        &mut self,
        lhs_codes: &[u8],
        rhs_codes: &[u8],
        rows: usize,
        inner_k: usize,
        cols: usize,
        lhs_codebook: &[i32; CODED_TABLE_LEN],
        rhs_codebook: &[i32; CODED_TABLE_LEN],
    ) -> Result<Vec<i64>, TensorComputeError> {
        let r = u32::try_from(rows).map_err(|_| TensorComputeError::ZeroOrMismatchedExtents {
            detail: "omega_worker:rows_exceeds_u32".to_string(),
        })?;
        let k =
            u32::try_from(inner_k).map_err(|_| TensorComputeError::ZeroOrMismatchedExtents {
                detail: "omega_worker:inner_k_exceeds_u32".to_string(),
            })?;
        let c = u32::try_from(cols).map_err(|_| TensorComputeError::ZeroOrMismatchedExtents {
            detail: "omega_worker:cols_exceeds_u32".to_string(),
        })?;

        let mut payload = Vec::with_capacity(144 + lhs_codes.len() + rhs_codes.len());
        payload.extend_from_slice(&r.to_le_bytes());
        payload.extend_from_slice(&k.to_le_bytes());
        payload.extend_from_slice(&c.to_le_bytes());
        payload.extend_from_slice(&0_u32.to_le_bytes());
        for &v in lhs_codebook {
            payload.extend_from_slice(&v.to_le_bytes());
        }
        for &v in rhs_codebook {
            payload.extend_from_slice(&v.to_le_bytes());
        }
        payload.extend_from_slice(lhs_codes);
        payload.extend_from_slice(rhs_codes);

        let (status, reply) = self.send_raw_frame(WIRE_MAGIC, OP_CODED_PAIR_LUT, &payload)?;
        if status != STATUS_OK {
            return Err(status_err(status));
        }

        let out_count = rows.saturating_mul(cols);
        let expected = out_count.saturating_mul(8);
        if reply.len() != expected {
            return Err(TensorComputeError::ZeroOrMismatchedExtents {
                detail: format!(
                    "omega_worker:pair_lut_reply_len expected={expected} actual={}",
                    reply.len()
                ),
            });
        }
        Ok(reply
            .chunks_exact(8)
            .map(|ch| i64::from_le_bytes([ch[0], ch[1], ch[2], ch[3], ch[4], ch[5], ch[6], ch[7]]))
            .collect())
    }

    /// Executes `contract_packed_nibbles_lut_into` (4-bit packed nibble LUT) on the pure-Omega worker.
    #[allow(clippy::too_many_arguments)]
    pub fn contract_packed_nibbles_lut(
        &mut self,
        lhs_packed_rows: &[u8],
        rhs_packed_cols: &[u8],
        rows: usize,
        inner_k: usize,
        cols: usize,
        lhs_codebook: &[i32; CODED_TABLE_LEN],
        rhs_codebook: &[i32; CODED_TABLE_LEN],
    ) -> Result<Vec<i64>, TensorComputeError> {
        let r = u32::try_from(rows).map_err(|_| TensorComputeError::ZeroOrMismatchedExtents {
            detail: "omega_worker:rows_exceeds_u32".to_string(),
        })?;
        let k =
            u32::try_from(inner_k).map_err(|_| TensorComputeError::ZeroOrMismatchedExtents {
                detail: "omega_worker:inner_k_exceeds_u32".to_string(),
            })?;
        let c = u32::try_from(cols).map_err(|_| TensorComputeError::ZeroOrMismatchedExtents {
            detail: "omega_worker:cols_exceeds_u32".to_string(),
        })?;

        let mut payload = Vec::with_capacity(144 + lhs_packed_rows.len() + rhs_packed_cols.len());
        payload.extend_from_slice(&r.to_le_bytes());
        payload.extend_from_slice(&k.to_le_bytes());
        payload.extend_from_slice(&c.to_le_bytes());
        payload.extend_from_slice(&0_u32.to_le_bytes());
        for &v in lhs_codebook {
            payload.extend_from_slice(&v.to_le_bytes());
        }
        for &v in rhs_codebook {
            payload.extend_from_slice(&v.to_le_bytes());
        }
        payload.extend_from_slice(lhs_packed_rows);
        payload.extend_from_slice(rhs_packed_cols);

        let (status, reply) = self.send_raw_frame(WIRE_MAGIC, OP_PACKED_NIBBLES_LUT, &payload)?;
        if status != STATUS_OK {
            return Err(status_err(status));
        }

        let out_count = rows.saturating_mul(cols);
        let expected = out_count.saturating_mul(8);
        if reply.len() != expected {
            return Err(TensorComputeError::ZeroOrMismatchedExtents {
                detail: format!(
                    "omega_worker:packed_lut_reply_len expected={expected} actual={}",
                    reply.len()
                ),
            });
        }
        Ok(reply
            .chunks_exact(8)
            .map(|ch| i64::from_le_bytes([ch[0], ch[1], ch[2], ch[3], ch[4], ch[5], ch[6], ch[7]]))
            .collect())
    }

    /// Executes `tropical_min_plus_all_pairs_closure` on the pure-Omega worker.
    pub fn tropical_min_plus_all_pairs_closure(
        &mut self,
        edge_costs_milli: &[Option<u32>],
        node_count: usize,
    ) -> Result<Vec<Option<u64>>, TensorComputeError> {
        let n =
            u32::try_from(node_count).map_err(|_| TensorComputeError::ZeroOrMismatchedExtents {
                detail: "omega_worker:node_count_exceeds_u32".to_string(),
            })?;
        let mut payload = Vec::with_capacity(8 + edge_costs_milli.len() * 8);
        payload.extend_from_slice(&n.to_le_bytes());
        payload.extend_from_slice(&0_u32.to_le_bytes());
        for &entry in edge_costs_milli {
            let encoded = match entry {
                Some(cost) => u64::from(cost),
                None => u64::MAX,
            };
            payload.extend_from_slice(&encoded.to_le_bytes());
        }

        let (status, reply) = self.send_raw_frame(WIRE_MAGIC, OP_TROPICAL_MIN_PLUS, &payload)?;
        if status != STATUS_OK {
            return Err(status_err(status));
        }

        let out_count = node_count.saturating_mul(node_count);
        let expected = out_count.saturating_mul(8);
        if reply.len() != expected {
            return Err(TensorComputeError::ZeroOrMismatchedExtents {
                detail: format!(
                    "omega_worker:tropical_reply_len expected={expected} actual={}",
                    reply.len()
                ),
            });
        }
        Ok(reply
            .chunks_exact(8)
            .map(|ch| {
                let raw =
                    u64::from_le_bytes([ch[0], ch[1], ch[2], ch[3], ch[4], ch[5], ch[6], ch[7]]);
                if raw == u64::MAX {
                    None
                } else {
                    Some(raw)
                }
            })
            .collect())
    }
}

impl Drop for OmegaNeuralWorker {
    fn drop(&mut self) {
        self.stdin.take();
        let _ = self.child.wait();
    }
}

/// Metadata and handle for a live, zero-weight-RAM native SLM executable synthesized by
/// `om/bin/verdant-slm-jit` and compiled on the fly by `omc` (zero LLVM, zero `rustc`).
pub struct OmegaCompiledSlm {
    child: Child,
    stdin: Option<ChildStdin>,
    stdout: ChildStdout,
    binary_path: PathBuf,
    embed_dim: usize,
    intent_count: usize,
    synthesized_om_bytes: usize,
    compiled_binary_bytes: usize,
    synth_micros: u64,
    compile_micros: u64,
}

impl OmegaCompiledSlm {
    /// Synthesizes a zero-weight-RAM pure-Omega `.om` program from live `CodedSlmDirector`
    /// weights (erasing every zero weight and baking non-zero weights into constant instructions),
    /// compiles it natively with `omc`, and launches the resulting executable.
    #[allow(clippy::too_many_arguments)]
    pub fn compile_and_spawn(
        embed_dim: usize,
        hidden_dim: usize,
        intent_count: usize,
        codebook: &[i32; CODED_TABLE_LEN],
        w1_packed_nibbles: &[u8],
        w2_grid_codes: &[u16],
        output_binary_path: impl AsRef<Path>,
    ) -> Result<Self, TensorComputeError> {
        let edim =
            u32::try_from(embed_dim).map_err(|_| TensorComputeError::ZeroOrMismatchedExtents {
                detail: "omega_slm_jit:embed_dim_exceeds_u32".to_string(),
            })?;
        let hdim =
            u32::try_from(hidden_dim).map_err(|_| TensorComputeError::ZeroOrMismatchedExtents {
                detail: "omega_slm_jit:hidden_dim_exceeds_u32".to_string(),
            })?;
        let icnt = u32::try_from(intent_count).map_err(|_| {
            TensorComputeError::ZeroOrMismatchedExtents {
                detail: "omega_slm_jit:intent_count_exceeds_u32".to_string(),
            }
        })?;
        let flags: u32 = if cfg!(all(target_os = "macos", target_arch = "aarch64")) {
            1
        } else {
            0
        };

        let mut model_bytes =
            Vec::with_capacity(88 + w1_packed_nibbles.len() + w2_grid_codes.len() * 2);
        model_bytes.extend_from_slice(&JIT_MAGIC.to_le_bytes());
        model_bytes.extend_from_slice(&edim.to_le_bytes());
        model_bytes.extend_from_slice(&hdim.to_le_bytes());
        model_bytes.extend_from_slice(&icnt.to_le_bytes());
        model_bytes.extend_from_slice(&SLM_RELU_SHIFT.to_le_bytes());
        model_bytes.extend_from_slice(&flags.to_le_bytes());
        for &v in codebook {
            model_bytes.extend_from_slice(&v.to_le_bytes());
        }
        model_bytes.extend_from_slice(w1_packed_nibbles);
        for &c in w2_grid_codes {
            model_bytes.extend_from_slice(&c.to_le_bytes());
        }

        // Stage 1: Pure-Omega partial evaluator synthesizes specialized .om source.
        let t0 = std::time::Instant::now();
        let mut jit_child = Command::new(default_slm_jit_binary_path())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(io_err)?;
        if let Some(mut stdin) = jit_child.stdin.take() {
            stdin.write_all(&model_bytes).map_err(io_err)?;
        }
        let jit_out = jit_child.wait_with_output().map_err(io_err)?;
        if !jit_out.status.success() {
            return Err(TensorComputeError::ZeroOrMismatchedExtents {
                detail: format!(
                    "omega_slm_jit:synthesis_refused_exit_{:?}",
                    jit_out.status.code()
                ),
            });
        }
        let synth_micros = t0.elapsed().as_micros() as u64;
        let om_src = jit_out.stdout;

        // Stage 2: Native omc compiles synthesized .om directly to Mach-O / ELF machine code.
        let t1 = std::time::Instant::now();
        let mut omc_child = Command::new(default_omc_binary_path())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(io_err)?;
        if let Some(mut stdin) = omc_child.stdin.take() {
            stdin.write_all(&om_src).map_err(io_err)?;
        }
        let omc_out = omc_child.wait_with_output().map_err(io_err)?;
        if !omc_out.status.success() {
            return Err(TensorComputeError::InvalidKernelSpecification {
                kernel_name: format!(
                    "omega_slm_jit:omc_failed_{:?}:{}",
                    omc_out.status.code(),
                    String::from_utf8_lossy(&omc_out.stderr).trim()
                ),
            });
        }
        let compile_micros = t1.elapsed().as_micros() as u64;
        let bin_bytes = omc_out.stdout;

        let bin_path = output_binary_path.as_ref().to_path_buf();
        std::fs::write(&bin_path, &bin_bytes).map_err(io_err)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = std::fs::metadata(&bin_path).map_err(io_err)?.permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&bin_path, perms).map_err(io_err)?;
        }
        #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
        {
            let sign_status = Command::new("codesign")
                .args(["-s", "-", "-f"])
                .arg(&bin_path)
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()
                .map_err(io_err)?;
            if !sign_status.success() {
                return Err(TensorComputeError::InvalidKernelSpecification {
                    kernel_name: "omega_slm_jit:codesign_failed".to_string(),
                });
            }
        }

        let mut child = Command::new(&bin_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .map_err(io_err)?;
        let stdin = child
            .stdin
            .take()
            .ok_or_else(|| io_err(io::Error::other("failed to capture compiled slm stdin")))?;
        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| io_err(io::Error::other("failed to capture compiled slm stdout")))?;

        Ok(Self {
            child,
            stdin: Some(stdin),
            stdout,
            binary_path: bin_path,
            embed_dim,
            intent_count,
            synthesized_om_bytes: om_src.len(),
            compiled_binary_bytes: bin_bytes.len(),
            synth_micros,
            compile_micros,
        })
    }

    /// Size in bytes of the pure-Omega `.om` source synthesized by `verdant-slm-jit`.
    #[must_use]
    pub const fn synthesized_om_bytes(&self) -> usize {
        self.synthesized_om_bytes
    }

    /// Size in bytes of the native executable emitted by `omc`.
    #[must_use]
    pub const fn compiled_binary_bytes(&self) -> usize {
        self.compiled_binary_bytes
    }

    /// Elapsed microseconds spent in `verdant-slm-jit` partial evaluation.
    #[must_use]
    pub const fn synth_micros(&self) -> u64 {
        self.synth_micros
    }

    /// Elapsed microseconds spent in native `omc` compilation.
    #[must_use]
    pub const fn compile_micros(&self) -> u64 {
        self.compile_micros
    }

    /// Path to the synthesized native executable on disk.
    #[must_use]
    pub fn binary_path(&self) -> &Path {
        &self.binary_path
    }

    /// Runs a batch of `batch_rows` feature vectors (`batch_rows * embed_dim` `i32`s) through the
    /// weight-baked native executable and returns exact `i64` logits (`batch_rows * intent_count`).
    pub fn infer_batch(
        &mut self,
        feature_activations_q: &[i32],
        batch_rows: usize,
    ) -> Result<Vec<i64>, TensorComputeError> {
        if batch_rows == 0
            || feature_activations_q.len() != batch_rows.saturating_mul(self.embed_dim)
        {
            return Err(TensorComputeError::ZeroOrMismatchedExtents {
                detail: "omega_slm_jit:batch_feature_len_mismatch".to_string(),
            });
        }
        let brows =
            u32::try_from(batch_rows).map_err(|_| TensorComputeError::ZeroOrMismatchedExtents {
                detail: "omega_slm_jit:batch_rows_exceeds_u32".to_string(),
            })?;
        let plen = u32::try_from(feature_activations_q.len() * 4).map_err(|_| {
            TensorComputeError::ZeroOrMismatchedExtents {
                detail: "omega_slm_jit:payload_exceeds_u32".to_string(),
            }
        })?;

        let mut hdr = [0_u8; HEADER_BYTES];
        hdr[0..4].copy_from_slice(&WIRE_MAGIC.to_le_bytes());
        hdr[4..8].copy_from_slice(&brows.to_le_bytes());
        hdr[8..12].copy_from_slice(&plen.to_le_bytes());

        let mut payload = Vec::with_capacity(plen as usize);
        for &v in feature_activations_q {
            payload.extend_from_slice(&v.to_le_bytes());
        }

        let stdin = self.stdin.as_mut().ok_or_else(|| {
            io_err(io::Error::new(
                io::ErrorKind::BrokenPipe,
                "compiled slm stdin closed",
            ))
        })?;
        stdin.write_all(&hdr).map_err(io_err)?;
        stdin.write_all(&payload).map_err(io_err)?;
        stdin.flush().map_err(io_err)?;

        let mut reply_hdr = [0_u8; HEADER_BYTES];
        self.stdout.read_exact(&mut reply_hdr).map_err(io_err)?;
        let reply_magic =
            u32::from_le_bytes([reply_hdr[0], reply_hdr[1], reply_hdr[2], reply_hdr[3]]);
        if reply_magic != WIRE_MAGIC {
            return Err(TensorComputeError::InvalidKernelSpecification {
                kernel_name: format!("omega_slm_jit:bad_reply_magic_0x{reply_magic:08x}"),
            });
        }
        let status = u32::from_le_bytes([reply_hdr[4], reply_hdr[5], reply_hdr[6], reply_hdr[7]]);
        if status != STATUS_OK {
            return Err(status_err(status));
        }
        let reply_len =
            u32::from_le_bytes([reply_hdr[8], reply_hdr[9], reply_hdr[10], reply_hdr[11]]) as usize;
        let expected = batch_rows
            .saturating_mul(self.intent_count)
            .saturating_mul(8);
        if reply_len != expected {
            return Err(TensorComputeError::ZeroOrMismatchedExtents {
                detail: format!(
                    "omega_slm_jit:reply_len_mismatch expected={expected} actual={reply_len}"
                ),
            });
        }
        let mut reply = vec![0_u8; reply_len];
        self.stdout.read_exact(&mut reply).map_err(io_err)?;
        Ok(reply
            .chunks_exact(8)
            .map(|ch| i64::from_le_bytes([ch[0], ch[1], ch[2], ch[3], ch[4], ch[5], ch[6], ch[7]]))
            .collect())
    }
}

impl Drop for OmegaCompiledSlm {
    fn drop(&mut self) {
        self.stdin.take();
        let _ = self.child.wait();
    }
}
