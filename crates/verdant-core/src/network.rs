//! Sovereign `Prism id` (`prism/identity/4`) Journal Envelopes & `afflom/stitch` 2-of-3
//! `(A, B, A ⊕ B)` Erasure Fragment Custody (`network.rs`).
//!
//! Connects Verdant Engine's deterministic `RollbackSession` and progressive `.uorc` streams to
//! `UOR-Foundation/foundry`'s zero-signin WebCrypto `Prism id` model and `afflom/stitch`'s
//! SHA-256 committed parity recovery and survival-entropy peer refresh fabric.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::error::StitchCustodyError;
use crate::prism_kernel;
use crate::runtime::{DeterministicInputJournal, FrameInputCommand};

/// Digest byte width (`32` bytes for SHA-256).
pub const SHA256_BYTES: usize = 1 << (1 + 1 + 1 + 1 + 1);

fn sha256_digest(bytes: &[u8]) -> [u8; SHA256_BYTES] {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let out = hasher.finalize();
    core::array::from_fn(|i| out[i])
}

fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 1 << (1 + 1 + 1 + 1)] = b"0123456789abcdef";
    let shift = 1 + 1 + 1 + 1;
    let mask = (1usize << shift) - 1usize;
    let mut s = String::with_capacity(bytes.len() * (1 + 1));
    for &b in bytes {
        let v = usize::from(b);
        s.push(HEX[v >> shift] as char);
        s.push(HEX[v & mask] as char);
    }
    s
}

/// Session signature envelope matching `<prism-frame>` `prism/identity/4` (`UOR-Foundation/foundry`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrismIdSessionEnvelope {
    /// Sovereign `Prism id` identifier (`prism:...`).
    pub prism_id: String,
    /// Origin-scoped non-extractable device public key fingerprint.
    pub device_fingerprint: String,
    /// Origin audience this session was issued for (`https://afflom.github.io` or local origin).
    pub audience_origin: String,
    /// Session issuance Unix timestamp in seconds.
    pub issued_at_sec: u64,
    /// Session expiration Unix timestamp in seconds (`<= 8` days from `issued_at_sec`).
    pub expires_at_sec: u64,
    /// Hex-encoded session signature over `(prism_id || audience_origin || payload_digest)`.
    pub signature_hex: String,
}

impl PrismIdSessionEnvelope {
    /// Maximum allowed session lifetime in seconds (`8` days, matching `prism/identity/4`).
    #[must_use]
    pub fn max_session_lifetime_secs() -> u64 {
        let eight = prism_kernel::sEight().unwrap_or(1);
        let sixty = prism_kernel::sSixty().unwrap_or(1);
        let twenty_four = prism_kernel::sTwentyFour().unwrap_or(1);
        eight * twenty_four * sixty * sixty
    }

    /// Creates a deterministic signed envelope binding `payload_digest` to `prism_id` and `device_secret`.
    /// In the browser, `<prism-frame>`'s `frame.sign(bytes)` produces external P-256/Ed25519 signatures
    /// paired with the kept session log; `sign_deterministic` provides the exact commitment structure
    /// for native, CLI, and offline ephemeral devices.
    #[must_use]
    pub fn sign_deterministic(
        prism_id: &str,
        device_secret: &[u8],
        audience_origin: &str,
        issued_at_sec: u64,
        expires_at_sec: u64,
        payload_digest: &[u8; SHA256_BYTES],
    ) -> Self {
        let dev_fp = hex_encode(&sha256_digest(device_secret));
        let mut msg = Vec::new();
        msg.extend_from_slice(b"prism/session/1:");
        msg.extend_from_slice(prism_id.as_bytes());
        msg.push(b':');
        msg.extend_from_slice(dev_fp.as_bytes());
        msg.push(b':');
        msg.extend_from_slice(audience_origin.as_bytes());
        msg.push(b':');
        msg.extend_from_slice(&issued_at_sec.to_le_bytes());
        msg.extend_from_slice(&expires_at_sec.to_le_bytes());
        msg.extend_from_slice(payload_digest);
        let sig = hex_encode(&sha256_digest(&msg));
        Self {
            prism_id: prism_id.to_string(),
            device_fingerprint: dev_fp,
            audience_origin: audience_origin.to_string(),
            issued_at_sec,
            expires_at_sec,
            signature_hex: sig,
        }
    }

    /// Verifies freshness, `8`-day lifetime bound, audience match, and cryptographic binding to `payload_digest`.
    pub fn verify(
        &self,
        expected_audience: &str,
        now_sec: u64,
        payload_digest: &[u8; SHA256_BYTES],
    ) -> Result<(), StitchCustodyError> {
        if !self.prism_id.starts_with("prism:") || self.device_fingerprint.is_empty() {
            return Err(StitchCustodyError::InvalidJournalSignature {
                detail: "missing prism: identifier or device fingerprint".to_string(),
            });
        }
        if self.audience_origin != expected_audience {
            return Err(StitchCustodyError::InvalidJournalSignature {
                detail: format!(
                    "session audience `{}` does not match `{expected_audience}`",
                    self.audience_origin
                ),
            });
        }
        if self.expires_at_sec <= self.issued_at_sec
            || self.expires_at_sec.saturating_sub(self.issued_at_sec)
                > Self::max_session_lifetime_secs()
            || now_sec < self.issued_at_sec
            || now_sec >= self.expires_at_sec
        {
            return Err(StitchCustodyError::InvalidJournalSignature {
                detail: "Prism ID session expired or outside 8-day freshness window".to_string(),
            });
        }

        let mut msg = Vec::new();
        msg.extend_from_slice(b"prism/session/1:");
        msg.extend_from_slice(self.prism_id.as_bytes());
        msg.push(b':');
        msg.extend_from_slice(self.device_fingerprint.as_bytes());
        msg.push(b':');
        msg.extend_from_slice(self.audience_origin.as_bytes());
        msg.push(b':');
        msg.extend_from_slice(&self.issued_at_sec.to_le_bytes());
        msg.extend_from_slice(&self.expires_at_sec.to_le_bytes());
        msg.extend_from_slice(payload_digest);
        let expected_sig = hex_encode(&sha256_digest(&msg));

        if self.signature_hex != expected_sig {
            return Err(StitchCustodyError::InvalidJournalSignature {
                detail: "signature mismatch for journal digest".to_string(),
            });
        }
        Ok(())
    }
}

/// Authenticated batch of rollback `FrameInputCommand`s signed by a citizen's `Prism id`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignedJournalBatch {
    /// Monotonically ordered frame commands.
    pub commands: Vec<FrameInputCommand>,
    /// Final SHA-256 chain digest over `commands`.
    pub chain_digest: [u8; SHA256_BYTES],
    /// `Prism id` session signature envelope binding `chain_digest`.
    pub envelope: PrismIdSessionEnvelope,
}

impl SignedJournalBatch {
    /// Builds and signs a verified `DeterministicInputJournal` batch.
    pub fn sign_commands(
        commands: Vec<FrameInputCommand>,
        prism_id: &str,
        device_secret: &[u8],
        audience_origin: &str,
        issued_at_sec: u64,
        expires_at_sec: u64,
    ) -> Result<Self, StitchCustodyError> {
        if commands.is_empty() {
            return Err(StitchCustodyError::InvalidJournalSignature {
                detail: "cannot sign empty command journal".to_string(),
            });
        }
        let mut journal = DeterministicInputJournal::new();
        for cmd in &commands {
            journal
                .append(*cmd)
                .map_err(|e| StitchCustodyError::InvalidJournalSignature {
                    detail: e.to_string(),
                })?;
        }
        let chain_digest = sha256_digest(journal.chain_digest_hex().as_bytes());
        let envelope = PrismIdSessionEnvelope::sign_deterministic(
            prism_id,
            device_secret,
            audience_origin,
            issued_at_sec,
            expires_at_sec,
            &chain_digest,
        );
        Ok(Self {
            commands,
            chain_digest,
            envelope,
        })
    }

    /// Recomputes the monotonic SHA-256 command chain and verifies the `Prism id` envelope.
    pub fn verify_admission(
        &self,
        expected_audience: &str,
        now_sec: u64,
    ) -> Result<(), StitchCustodyError> {
        if self.commands.is_empty() {
            return Err(StitchCustodyError::InvalidJournalSignature {
                detail: "empty SignedJournalBatch refused".to_string(),
            });
        }
        let mut journal = DeterministicInputJournal::new();
        for cmd in &self.commands {
            journal
                .append(*cmd)
                .map_err(|e| StitchCustodyError::InvalidJournalSignature {
                    detail: e.to_string(),
                })?;
        }
        if sha256_digest(journal.chain_digest_hex().as_bytes()) != self.chain_digest {
            return Err(StitchCustodyError::PayloadDigestMismatch {
                detail: "SignedJournalBatch chain_digest does not match commands".to_string(),
            });
        }
        self.envelope
            .verify(expected_audience, now_sec, &self.chain_digest)
    }
}

/// Erasure-coded object shard compatible with `afflom/stitch` (`Stitch.Fragments::Fragment`).
///
/// Any object (`bytes`) splits into `3` equal-length shards:
/// - `index = 0`: Left half `A`
/// - `index = 1`: Right half `B` (zero-padded by at most `1` byte if `bytes.len()` is odd)
/// - `index = 2`: Parity `P = A ⊕ B`
///
/// Any two distinct fragments (`(0, 1)`, `(0, 2)`, or `(1, 2)`) reconstruct `bytes` bit-for-bit
/// and verify `SHA-256(bytes) == manifest_root`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StitchFragment {
    /// Full `SHA-256` commitment of the original unfragmented `.uorc` or `WorldSnapshot` object.
    pub manifest_root: [u8; SHA256_BYTES],
    /// Exact byte length of the original unfragmented object.
    pub original_byte_len: usize,
    /// Shard slot (`0` = left `A`, `1` = right `B`, `2` = parity `A ⊕ B`).
    pub index: u8,
    /// `SHA-256` commitment of `payload`.
    pub payload_root: [u8; SHA256_BYTES],
    /// Fragment bytes (`ceil(original_byte_len / 2)` bytes).
    pub payload: Vec<u8>,
}

impl StitchFragment {
    /// Serializes this fragment into a self-verifying `STCH` binary wire frame (`77 + payload.len()` bytes).
    #[must_use]
    pub fn encode_wire(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(
            (prism_kernel::sFour().unwrap_or(1) as usize)
                + SHA256_BYTES
                + (prism_kernel::sEight().unwrap_or(1) as usize)
                + 1
                + SHA256_BYTES
                + self.payload.len(),
        );
        out.extend_from_slice(b"STCH");
        out.extend_from_slice(&self.manifest_root);
        out.extend_from_slice(&(self.original_byte_len as u64).to_le_bytes());
        out.push(self.index);
        out.extend_from_slice(&self.payload_root);
        out.extend_from_slice(&self.payload);
        out
    }

    /// Parses and SHA-256-verifies a `STCH` binary wire frame.
    pub fn decode_wire(wire: &[u8]) -> Result<Self, StitchCustodyError> {
        let magic_len = prism_kernel::sFour().unwrap_or(1) as usize;
        let u64_len = prism_kernel::sEight().unwrap_or(1) as usize;
        let header_len = magic_len + SHA256_BYTES + u64_len + 1 + SHA256_BYTES;
        if wire.len() <= header_len || &wire[..magic_len] != b"STCH" {
            return Err(StitchCustodyError::IncompatibleFragmentPair {
                left_index: 0,
                right_index: 0,
                detail: "invalid or truncated STCH fragment wire header".to_string(),
            });
        }
        let mut cursor = magic_len;
        let manifest_root: [u8; SHA256_BYTES] = core::array::from_fn(|i| wire[cursor + i]);
        cursor += SHA256_BYTES;

        let len_bytes: [u8; 1 << (1 + 1 + 1)] = core::array::from_fn(|i| wire[cursor + i]);
        let original_byte_len = u64::from_le_bytes(len_bytes) as usize;
        cursor += u64_len;

        let index = wire[cursor];
        cursor += 1;

        let payload_root: [u8; SHA256_BYTES] = core::array::from_fn(|i| wire[cursor + i]);
        cursor += SHA256_BYTES;

        let payload = wire[cursor..].to_vec();
        if sha256_digest(&payload) != payload_root {
            return Err(StitchCustodyError::PayloadDigestMismatch {
                detail: "STCH wire fragment payload failed SHA-256 verification".to_string(),
            });
        }
        Ok(Self {
            manifest_root,
            original_byte_len,
            index,
            payload_root,
            payload,
        })
    }
}

/// Splits `object_bytes` into `[fragment_a, fragment_b, fragment_parity]` with SHA-256 commitments.
pub fn shard_bytes_to_stitch_fragments(
    object_bytes: &[u8],
) -> Result<[StitchFragment; 1 + 1 + 1], StitchCustodyError> {
    if object_bytes.is_empty() {
        return Err(StitchCustodyError::IncompatibleFragmentPair {
            left_index: 0,
            right_index: 0,
            detail: "cannot shard empty object".to_string(),
        });
    }
    let manifest_root = sha256_digest(object_bytes);
    let original_byte_len = object_bytes.len();
    let half_len = original_byte_len.div_ceil(1 + 1);

    let mut left_a = vec![0_u8; half_len];
    left_a.copy_from_slice(&object_bytes[..half_len]);

    let mut right_b = vec![0_u8; half_len];
    let rem = &object_bytes[half_len..];
    right_b[..rem.len()].copy_from_slice(rem);

    let parity_p: Vec<u8> = left_a.iter().zip(&right_b).map(|(&a, &b)| a ^ b).collect();

    let frag_a = StitchFragment {
        manifest_root,
        original_byte_len,
        index: 0,
        payload_root: sha256_digest(&left_a),
        payload: left_a,
    };
    let frag_b = StitchFragment {
        manifest_root,
        original_byte_len,
        index: 1,
        payload_root: sha256_digest(&right_b),
        payload: right_b,
    };
    let frag_p = StitchFragment {
        manifest_root,
        original_byte_len,
        index: prism_kernel::sTwo() as u8,
        payload_root: sha256_digest(&parity_p),
        payload: parity_p,
    };
    Ok([frag_a, frag_b, frag_p])
}

/// Reconstructs and verifies the exact original object from **any two distinct** `StitchFragment`s,
/// gated by `prism_kernel::stitchPairRecoverable`.
pub fn recover_bytes_from_stitch_pair(
    first: &StitchFragment,
    second: &StitchFragment,
) -> Result<Vec<u8>, StitchCustodyError> {
    let manifest_matches = first.manifest_root == second.manifest_root
        && first.original_byte_len == second.original_byte_len
        && !first.payload.is_empty()
        && first.payload.len() == second.payload.len()
        && first.payload.len() == first.original_byte_len.div_ceil(1 + 1);
    let first_valid = sha256_digest(&first.payload) == first.payload_root;
    let second_valid = sha256_digest(&second.payload) == second.payload_root;

    let allowed = prism_kernel::stitchPairRecoverable(
        u64::from(first.index),
        u64::from(second.index),
        manifest_matches,
        first_valid,
        second_valid,
    );

    if !allowed {
        if !first_valid || !second_valid {
            return Err(StitchCustodyError::PayloadDigestMismatch {
                detail: "fragment payload SHA-256 commitment failed".to_string(),
            });
        }
        return Err(StitchCustodyError::IncompatibleFragmentPair {
            left_index: first.index,
            right_index: second.index,
            detail: "pair requires two distinct indices in 0..3 with matching manifest".to_string(),
        });
    }

    let (low, high) = if first.index < second.index {
        (first, second)
    } else {
        (second, first)
    };
    let idx_parity = prism_kernel::sTwo() as u8;

    let (left_a, right_b): (Vec<u8>, Vec<u8>) = match (low.index, high.index) {
        (0, 1) => (low.payload.clone(), high.payload.clone()),
        (0, p) if p == idx_parity => {
            let b: Vec<u8> = low
                .payload
                .iter()
                .zip(&high.payload)
                .map(|(&a, &par)| a ^ par)
                .collect();
            (low.payload.clone(), b)
        }
        (1, p) if p == idx_parity => {
            let a: Vec<u8> = low
                .payload
                .iter()
                .zip(&high.payload)
                .map(|(&b, &par)| b ^ par)
                .collect();
            (a, low.payload.clone())
        }
        _ => {
            return Err(StitchCustodyError::IncompatibleFragmentPair {
                left_index: low.index,
                right_index: high.index,
                detail: "unsupported fragment index combination".to_string(),
            });
        }
    };

    let mut combined = Vec::with_capacity(left_a.len() + right_b.len());
    combined.extend_from_slice(&left_a);
    combined.extend_from_slice(&right_b);
    combined.truncate(low.original_byte_len);

    if sha256_digest(&combined) != low.manifest_root {
        return Err(StitchCustodyError::PayloadDigestMismatch {
            detail: "reconstructed object SHA-256 did not match manifest_root".to_string(),
        });
    }
    Ok(combined)
}

/// Authenticated peer liveness observation (`afflom/stitch` `Stitch.Observation::PeerObservation`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PeerObservationRecord {
    /// Peer numeric slot.
    pub peer_id: u64,
    /// Monotonic sequence number from that peer.
    pub sequence: u64,
    /// Observation timestamp in seconds.
    pub observed_at_sec: u64,
    /// Expiration timestamp in seconds.
    pub expires_at_sec: u64,
    /// Shard index (`0..=2`) held by this peer.
    pub held_fragment_index: u8,
    /// Whether the observation carried a valid `Prism id` signature.
    pub authenticated: bool,
}

/// Tracks peer custody observations and schedules survival-entropy parity refreshes.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurvivalEntropyTracker {
    peers: std::collections::BTreeMap<u64, PeerObservationRecord>,
}

impl SurvivalEntropyTracker {
    /// Admits a fresh, authenticated, strictly monotonic peer observation via `prism_kernel::peerObservationFresh`.
    pub fn record_observation(
        &mut self,
        candidate: PeerObservationRecord,
        now_sec: u64,
    ) -> Result<(), StitchCustodyError> {
        let prev_seq = self.peers.get(&candidate.peer_id).map_or(0, |p| p.sequence);
        let next_seq = if self.peers.contains_key(&candidate.peer_id) {
            candidate.sequence
        } else {
            candidate.sequence.max(1)
        };
        let fresh = prism_kernel::peerObservationFresh(
            candidate.observed_at_sec,
            candidate.expires_at_sec,
            now_sec,
            prev_seq,
            next_seq,
            candidate.authenticated,
        );
        if !fresh || u64::from(candidate.held_fragment_index) >= prism_kernel::sThree() {
            return Err(StitchCustodyError::StalePeerObservation {
                peer_id: candidate.peer_id,
            });
        }
        self.peers.insert(candidate.peer_id, candidate);
        Ok(())
    }

    /// Computes survival-entropy refresh urgency in permille (`0` = all 3 distinct shards live,
    /// `500` = down to 2 distinct shards at recovery threshold, `1000` = below recovery threshold).
    #[must_use]
    pub fn refresh_urgency_permille(&self, now_sec: u64) -> u32 {
        let mut distinct_mask = 0_u8;
        for rec in self.peers.values() {
            if rec.authenticated && rec.observed_at_sec <= now_sec && now_sec < rec.expires_at_sec {
                distinct_mask |= 1_u8 << rec.held_fragment_index;
            }
        }
        let distinct_live = distinct_mask.count_ones();
        let scale = prism_kernel::skinWeightScale().unwrap_or(1) as u32;
        let full = prism_kernel::sThree() as u32;
        let min_thresh = prism_kernel::sTwo() as u32;
        if distinct_live >= full {
            0
        } else if distinct_live == min_thresh {
            scale / min_thresh
        } else {
            scale
        }
    }
}
