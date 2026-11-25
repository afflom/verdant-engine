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
/// Wire operation code for `step_neuromorphic_lif_ssm_ring` (`kernel10`).
pub const OP_LIF_SSM: u32 = 1;
/// Wire operation code for `infer_coded_slm_intent_logits` (`kernel11`).
pub const OP_CODED_SLM: u32 = 2;
/// Wire operation code for a zero-payload liveness ping.
pub const OP_PING: u32 = 3;

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
}

impl Drop for OmegaNeuralWorker {
    fn drop(&mut self) {
        self.stdin.take();
        let _ = self.child.wait();
    }
}
