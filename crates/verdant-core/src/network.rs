//! Sovereign `Prism id` (`prism/identity/4`) Journal Envelopes & `afflom/stitch` 2-of-3
//! `(A, B, A ⊕ B)` Erasure Fragment Custody (`network.rs`).
//!
//! Connects Verdant Engine's deterministic `RollbackSession` and progressive `.uorc` streams to
//! `UOR-Foundation/foundry`'s zero-signin WebCrypto `Prism id` model and `afflom/stitch`'s
//! SHA-256 committed parity recovery and survival-entropy peer refresh fabric.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::digest::hex_encode;
use crate::error::StitchCustodyError;
use crate::observer::sha256_payload_digest as sha256_digest;
use crate::prism_kernel;
use crate::runtime::{DeterministicInputJournal, FrameInputCommand};

/// Digest byte width (`32` bytes for SHA-256).
pub const SHA256_BYTES: usize = 1 << (1 + 1 + 1 + 1 + 1);

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
        let eight = crate::math::eight_u32() as u64;
        let sixty = crate::math::sixty_four_u32().wrapping_sub(crate::math::four_u32()) as u64;
        let twenty_four = crate::math::eight_u32().wrapping_mul(crate::math::three_u32()) as u64;
        eight
            .wrapping_mul(twenty_four)
            .wrapping_mul(sixty)
            .wrapping_mul(sixty)
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

/// Enforces `UORC` wire framing before splitting into `3` `afflom/stitch` `(A, B, A ⊕ B)` parity
/// fragments: if `object_bytes` is not already a `b"UORC"` archive, compresses it via
/// [`crate::streaming::compress_uorc_portable`] first.
#[cfg(any(feature = "uorc-streaming", feature = "uorc-wasm-decode"))]
pub fn shard_uorc_to_stitch_fragments(
    object_bytes: &[u8],
) -> Result<[StitchFragment; 1 + 1 + 1], StitchCustodyError> {
    if object_bytes.is_empty() {
        return Err(StitchCustodyError::IncompatibleFragmentPair {
            left_index: 0,
            right_index: 0,
            detail: "cannot shard empty UORC payload".to_string(),
        });
    }
    if crate::streaming::is_uorc_archive(object_bytes) {
        shard_bytes_to_stitch_fragments(object_bytes)
    } else {
        let uorc_archive = crate::streaming::compress_uorc_portable(object_bytes);
        shard_bytes_to_stitch_fragments(&uorc_archive)
    }
}

/// Reconstructs a `UORC` archive from any two distinct `StitchFragment`s and verifies both the
/// outer `STCH` SHA-256 manifest root and the inner `UORC` SHA-256 chain seal via `uorc_decompress::run`.
#[cfg(feature = "uorc-wasm-decode")]
pub fn recover_uorc_from_stitch_pair(
    left: &StitchFragment,
    right: &StitchFragment,
) -> Result<(Vec<u8>, Vec<u8>), StitchCustodyError> {
    let uorc_bytes = recover_bytes_from_stitch_pair(left, right)?;
    let plain_bytes = crate::streaming::decompress_uorc_portable(&uorc_bytes).map_err(|e| {
        StitchCustodyError::PayloadDigestMismatch {
            detail: e.to_string(),
        }
    })?;
    Ok((uorc_bytes, plain_bytes))
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

// ---------------------------------------------------------------------------
// Zero-Account Sovereign Stitch CIA Primitives (AUTH-STITCH-SELF-HOSTING / VE-43):
//   1. PrismBoundaryCipher (RFC 2104 HMAC-SHA256 + SHA-256-CTR Encrypt-then-MAC AEAD + 256B padding)
//   2. Systematic k-of-n Cauchy Reed-Solomon over GF(2^8) (STCH2) + Autonomous Parity Repair
//   3. StitchMonotonicPointer (Anti-Rollback Quorum Release / Deep-Link Pointer)
//   4. StitchBudgetLedger (Zero-Account Reciprocal Quota & Sybil Flood Guard)
// ---------------------------------------------------------------------------

/// Fixed AEAD ciphertext padding block quantum (`256` bytes = `2^8`) mitigating payload size leakage.
pub const BOUNDARY_PADDING_BLOCK_BYTES: usize = 1 << (1 + 1 + 1 + 1 + 1 + 1 + 1 + 1);

/// Default systematic Reed-Solomon data shard count for Verdant Releases (`k = 4`).
pub const RELEASE_RS_DATA_SHARDS: u8 = 1 << (1 + 1);

/// Default systematic Reed-Solomon total shard count for Verdant Releases (`n = 8`, tolerating 4 lost hosts).
pub const RELEASE_RS_TOTAL_SHARDS: u8 = 1 << (1 + 1 + 1);

fn hmac_sha256(key: &[u8], msg: &[u8]) -> [u8; SHA256_BYTES] {
    let block_len = 1usize << (1 + 1 + 1 + 1 + 1 + 1);
    let ipad_byte =
        (1u8 << (1 + 1 + 1 + 1 + 1)) | (1u8 << (1 + 1 + 1 + 1)) | (1u8 << (1 + 1)) | (1u8 << 1);
    let opad_byte = (1u8 << (1 + 1 + 1 + 1 + 1 + 1))
        | (1u8 << (1 + 1 + 1 + 1))
        | (1u8 << (1 + 1 + 1))
        | (1u8 << (1 + 1));
    let mut k_block = [0u8; 1 << (1 + 1 + 1 + 1 + 1 + 1)];
    if key.len() > block_len {
        let kh = sha256_digest(key);
        k_block[..SHA256_BYTES].copy_from_slice(&kh);
    } else {
        k_block[..key.len()].copy_from_slice(key);
    }
    let mut inner_pad = [ipad_byte; 1 << (1 + 1 + 1 + 1 + 1 + 1)];
    let mut outer_pad = [opad_byte; 1 << (1 + 1 + 1 + 1 + 1 + 1)];
    for i in 0..block_len {
        inner_pad[i] ^= k_block[i];
        outer_pad[i] ^= k_block[i];
    }
    let mut inner_hasher = Sha256::new();
    inner_hasher.update(inner_pad);
    inner_hasher.update(msg);
    let inner_digest = inner_hasher.finalize();

    let mut outer_hasher = Sha256::new();
    outer_hasher.update(outer_pad);
    outer_hasher.update(inner_digest);
    let out = outer_hasher.finalize();
    core::array::from_fn(|i| out[i])
}

fn constant_time_eq_32(a: &[u8; SHA256_BYTES], b: &[u8; SHA256_BYTES]) -> bool {
    a.iter().zip(b.iter()).fold(
        0_u64,
        |acc, (&x, &y)| {
            if x == y {
                acc
            } else {
                acc.wrapping_add(1_u64)
            }
        },
    ) == 0_u64
}

/// Confidentiality visibility mode for any stakeholder or Observer content on Stitch.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BoundaryVisibility {
    /// Publicly verifiable content (Verdant Releases, Verdant Studio, public App Catalog).
    Public,
    /// Default stakeholder mode: AEAD-encrypted under owner's `Prism ID` boundary key before sharding.
    PrivateEncrypted,
    /// Selectively shared private boundary with wrapped per-collaborator keys & epoch revocation.
    SelectiveShare,
}

/// Encrypted, padded, AAD-authenticated Stitch boundary envelope (`Encrypt-then-MAC` AEAD).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StitchBoundaryEnvelope {
    /// Boundary identifier slug or content URI (`stitch://...`).
    pub boundary_id: String,
    /// Visibility class (`private-encrypted` by default for user content).
    pub visibility: BoundaryVisibility,
    /// Monotonic boundary key epoch (incremented on every collaborator revocation).
    pub boundary_epoch: u64,
    /// Owner's sovereign `Prism ID` (`prism:...`).
    pub owner_prism_id: String,
    /// Unpadded plaintext byte length (authenticated inside AAD).
    pub plaintext_len: usize,
    /// 32-byte deterministic per-message nonce.
    pub nonce: [u8; SHA256_BYTES],
    /// Fixed-block-padded ciphertext (`ceil((plaintext_len + 1) / 256) * 256` bytes).
    pub ciphertext: Vec<u8>,
    /// RFC 2104 `HMAC-SHA256` authentication tag over `(AAD || nonce || ciphertext)`.
    pub mac_tag: [u8; SHA256_BYTES],
}

/// Sovereign per-boundary key ring managing zero-account `Prism ID` encryption, selective sharing,
/// and forward-secret revocation epoch rotation (`Stitch.PrismId.revocationCounts`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BoundaryKeyring {
    /// Boundary identifier (`stitch://boundary/...`).
    pub boundary_id: String,
    /// Sovereign owner `Prism ID`.
    pub owner_prism_id: String,
    /// Current monotonic key epoch (`>= 1`).
    pub current_epoch: u64,
    /// Active 32-byte master boundary key for `current_epoch`.
    pub active_key: [u8; SHA256_BYTES],
    /// Active authorized collaborator `Prism ID`s.
    pub admitted_members: std::collections::BTreeSet<String>,
    /// Revoked `Prism ID`s denied access on `current_epoch` and future epochs.
    pub revoked_members: std::collections::BTreeSet<String>,
}

impl BoundaryKeyring {
    /// Creates a new zero-account private boundary keyring derived from the owner's `Prism ID` seed.
    pub fn new(
        boundary_id: &str,
        owner_prism_id: &str,
        owner_device_secret: &[u8],
    ) -> Result<Self, StitchCustodyError> {
        if boundary_id.trim().is_empty()
            || !owner_prism_id.starts_with("prism:")
            || owner_device_secret.is_empty()
        {
            return Err(StitchCustodyError::InvalidJournalSignature {
                detail: "BoundaryKeyring requires non-empty boundary_id, prism: owner, and secret"
                    .to_string(),
            });
        }
        let mut init = Vec::new();
        init.extend_from_slice(b"stitch/boundary-key/v1:");
        init.extend_from_slice(boundary_id.as_bytes());
        init.push(b':');
        init.extend_from_slice(owner_prism_id.as_bytes());
        init.push(b':');
        init.extend_from_slice(&1u64.to_le_bytes());
        let active_key = hmac_sha256(owner_device_secret, &init);
        let mut admitted_members = std::collections::BTreeSet::new();
        admitted_members.insert(owner_prism_id.to_string());
        Ok(Self {
            boundary_id: boundary_id.to_string(),
            owner_prism_id: owner_prism_id.to_string(),
            current_epoch: 1,
            active_key,
            admitted_members,
            revoked_members: std::collections::BTreeSet::new(),
        })
    }

    /// Grants selective-share access to another zero-account collaborator's `Prism ID`.
    pub fn admit_member(&mut self, member_prism_id: &str) -> Result<(), StitchCustodyError> {
        if !member_prism_id.starts_with("prism:") {
            return Err(StitchCustodyError::InvalidJournalSignature {
                detail: "collaborator identity must start with prism:".to_string(),
            });
        }
        self.revoked_members.remove(member_prism_id);
        self.admitted_members.insert(member_prism_id.to_string());
        Ok(())
    }

    /// Revokes `member_prism_id` and performs forward-secret key rotation to `current_epoch + 1`
    /// so the revoked peer cannot decrypt future revisions or replay stale-epoch writes.
    pub fn revoke_and_rotate(
        &mut self,
        member_prism_id: &str,
        rotation_entropy: &[u8],
    ) -> Result<u64, StitchCustodyError> {
        if member_prism_id == self.owner_prism_id {
            return Err(StitchCustodyError::InvalidJournalSignature {
                detail: "cannot revoke sovereign boundary owner".to_string(),
            });
        }
        self.admitted_members.remove(member_prism_id);
        self.revoked_members.insert(member_prism_id.to_string());
        let next_epoch = self.current_epoch.saturating_add(1);
        let mut pre = Vec::new();
        pre.extend_from_slice(b"stitch/boundary-rotate/v1:");
        pre.extend_from_slice(self.boundary_id.as_bytes());
        pre.push(b':');
        pre.extend_from_slice(&next_epoch.to_le_bytes());
        pre.push(b':');
        pre.extend_from_slice(member_prism_id.as_bytes());
        pre.push(b':');
        pre.extend_from_slice(rotation_entropy);
        self.active_key = hmac_sha256(&self.active_key, &pre);
        self.current_epoch = next_epoch;
        Ok(next_epoch)
    }

    /// Returns true iff `actor_prism_id` is currently admitted and unrevoked.
    #[must_use]
    pub fn is_authorized(&self, actor_prism_id: &str) -> bool {
        self.admitted_members.contains(actor_prism_id)
            && !self.revoked_members.contains(actor_prism_id)
    }
}

/// Standards-based `Encrypt-then-MAC` AEAD (`SHA-256-CTR` + `HMAC-SHA256` + 256B padding)
/// executing encrypt-before-shard boundary confidentiality across native Rust and `wasm32`.
pub struct PrismBoundaryCipher;

impl PrismBoundaryCipher {
    fn derive_subkeys(
        master_key: &[u8; SHA256_BYTES],
        epoch: u64,
    ) -> ([u8; SHA256_BYTES], [u8; SHA256_BYTES]) {
        let mut enc_ctx = Vec::new();
        enc_ctx.extend_from_slice(b"stitch/aead-enc/v1:");
        enc_ctx.extend_from_slice(&epoch.to_le_bytes());
        let mut mac_ctx = Vec::new();
        mac_ctx.extend_from_slice(b"stitch/aead-mac/v1:");
        mac_ctx.extend_from_slice(&epoch.to_le_bytes());
        (
            hmac_sha256(master_key, &enc_ctx),
            hmac_sha256(master_key, &mac_ctx),
        )
    }

    fn compute_mac(
        mac_key: &[u8; SHA256_BYTES],
        boundary_id: &str,
        owner_prism_id: &str,
        epoch: u64,
        plaintext_len: usize,
        nonce: &[u8; SHA256_BYTES],
        ciphertext: &[u8],
    ) -> [u8; SHA256_BYTES] {
        let mut msg = Vec::with_capacity(
            boundary_id.len()
                + owner_prism_id.len()
                + SHA256_BYTES
                + ciphertext.len()
                + (1 << (1 + 1 + 1 + 1 + 1)),
        );
        msg.extend_from_slice(b"stitch/aead-aad/v1:");
        msg.extend_from_slice(boundary_id.as_bytes());
        msg.push(0);
        msg.extend_from_slice(owner_prism_id.as_bytes());
        msg.push(0);
        msg.extend_from_slice(&epoch.to_le_bytes());
        msg.extend_from_slice(&(plaintext_len as u64).to_le_bytes());
        msg.extend_from_slice(nonce);
        msg.extend_from_slice(ciphertext);
        hmac_sha256(mac_key, &msg)
    }

    fn apply_ctr_stream(enc_key: &[u8; SHA256_BYTES], nonce: &[u8; SHA256_BYTES], data: &mut [u8]) {
        let mut counter = 0u64;
        let mut offset = 0usize;
        while offset < data.len() {
            let mut blk = Vec::with_capacity(SHA256_BYTES + (1 << (1 + 1 + 1)));
            blk.extend_from_slice(nonce);
            blk.extend_from_slice(&counter.to_le_bytes());
            let stream_block = hmac_sha256(enc_key, &blk);
            let take = (data.len() - offset).min(SHA256_BYTES);
            for i in 0..take {
                data[offset + i] ^= stream_block[i];
            }
            offset += take;
            counter = counter.saturating_add(1);
        }
    }

    /// Encrypts & authenticates `plaintext` into a 256-byte padded [`StitchBoundaryEnvelope`]
    /// BEFORE erasure sharding (`Threat 12`: raw shards leak zero plaintext).
    pub fn seal(
        keyring: &BoundaryKeyring,
        actor_prism_id: &str,
        visibility: BoundaryVisibility,
        plaintext: &[u8],
        nonce_seed: &[u8],
    ) -> Result<StitchBoundaryEnvelope, StitchCustodyError> {
        if !keyring.is_authorized(actor_prism_id) {
            return Err(StitchCustodyError::InvalidJournalSignature {
                detail: format!(
                    "actor `{actor_prism_id}` is not authorized for boundary `{}`",
                    keyring.boundary_id
                ),
            });
        }
        let (enc_key, mac_key) = Self::derive_subkeys(&keyring.active_key, keyring.current_epoch);
        let mut nonce_in = Vec::with_capacity(nonce_seed.len() + SHA256_BYTES + (1 << (1 + 1 + 1)));
        nonce_in.extend_from_slice(b"stitch/nonce/v1:");
        nonce_in.extend_from_slice(&keyring.current_epoch.to_le_bytes());
        nonce_in.extend_from_slice(nonce_seed);
        nonce_in.extend_from_slice(&sha256_digest(plaintext));
        let nonce = hmac_sha256(&enc_key, &nonce_in);

        let padded_len = plaintext
            .len()
            .saturating_add(1)
            .div_ceil(BOUNDARY_PADDING_BLOCK_BYTES)
            .saturating_mul(BOUNDARY_PADDING_BLOCK_BYTES);
        let mut ciphertext = vec![0u8; padded_len];
        ciphertext[..plaintext.len()].copy_from_slice(plaintext);
        // Domain-separated 0x80 padding delimiter:
        ciphertext[plaintext.len()] = 1u8 << (1 + 1 + 1 + 1 + 1 + 1 + 1);

        Self::apply_ctr_stream(&enc_key, &nonce, &mut ciphertext);
        let mac_tag = Self::compute_mac(
            &mac_key,
            &keyring.boundary_id,
            &keyring.owner_prism_id,
            keyring.current_epoch,
            plaintext.len(),
            &nonce,
            &ciphertext,
        );
        Ok(StitchBoundaryEnvelope {
            boundary_id: keyring.boundary_id.clone(),
            visibility,
            boundary_epoch: keyring.current_epoch,
            owner_prism_id: keyring.owner_prism_id.clone(),
            plaintext_len: plaintext.len(),
            nonce,
            ciphertext,
            mac_tag,
        })
    }

    /// Verifies authorization, epoch binding, and `HMAC-SHA256` MAC tag BEFORE decrypting.
    pub fn open(
        keyring: &BoundaryKeyring,
        actor_prism_id: &str,
        envelope: &StitchBoundaryEnvelope,
    ) -> Result<Vec<u8>, StitchCustodyError> {
        if !keyring.is_authorized(actor_prism_id) {
            return Err(StitchCustodyError::InvalidJournalSignature {
                detail: format!("revoked or unauthorized actor `{actor_prism_id}` refused"),
            });
        }
        if envelope.boundary_id != keyring.boundary_id
            || envelope.boundary_epoch != keyring.current_epoch
        {
            return Err(StitchCustodyError::InvalidJournalSignature {
                detail: "boundary_id or rotated boundary_epoch mismatch".to_string(),
            });
        }
        let (enc_key, mac_key) = Self::derive_subkeys(&keyring.active_key, keyring.current_epoch);
        let expected_mac = Self::compute_mac(
            &mac_key,
            &envelope.boundary_id,
            &envelope.owner_prism_id,
            envelope.boundary_epoch,
            envelope.plaintext_len,
            &envelope.nonce,
            &envelope.ciphertext,
        );
        if !constant_time_eq_32(&expected_mac, &envelope.mac_tag) {
            return Err(StitchCustodyError::PayloadDigestMismatch {
                detail: "StitchBoundaryEnvelope HMAC-SHA256 authentication failed".to_string(),
            });
        }
        if envelope.plaintext_len >= envelope.ciphertext.len() {
            return Err(StitchCustodyError::PayloadDigestMismatch {
                detail: "corrupted StitchBoundaryEnvelope plaintext_len".to_string(),
            });
        }
        let mut plain = envelope.ciphertext.clone();
        Self::apply_ctr_stream(&enc_key, &envelope.nonce, &mut plain);
        plain.truncate(envelope.plaintext_len);
        Ok(plain)
    }
}

// --- GF(2^8) Systematic k-of-n Cauchy Reed-Solomon Erasure Codec (`STCH2`) ---

const GF256_POLY: u16 = (1u16 << (1 + 1 + 1 + 1 + 1 + 1 + 1 + 1))
    | (1u16 << (1 + 1 + 1 + 1))
    | (1u16 << (1 + 1 + 1))
    | (1u16 << (1 + 1))
    | 1u16;

#[inline]
fn gf256_mul(mut a: u8, mut b: u8) -> u8 {
    let mut p = 0u8;
    let bits = 1usize << (1 + 1 + 1);
    let hi_bit = 1u8 << (1 + 1 + 1 + 1 + 1 + 1 + 1);
    let poly_low = (GF256_POLY & ((1u16 << bits) - 1)) as u8;
    for _ in 0..bits {
        if (b & 1) != 0 {
            p ^= a;
        }
        let carry = (a & hi_bit) != 0;
        a <<= 1;
        if carry {
            a ^= poly_low;
        }
        b >>= 1;
    }
    p
}

#[inline]
fn gf256_pow(mut base: u8, mut exp: u16) -> u8 {
    let mut acc = 1u8;
    while exp > 0 {
        if (exp & 1) != 0 {
            acc = gf256_mul(acc, base);
        }
        base = gf256_mul(base, base);
        exp >>= 1;
    }
    acc
}

#[inline]
fn gf256_inv(a: u8) -> u8 {
    if a == 0 {
        0
    } else {
        // By Fermat's little theorem in GF(2^8): a^-1 = a^(2^8 - 2) = a^254.
        let exp = (crate::math::pow2_u64(crate::math::eight_u32()) as u16)
            .wrapping_sub(1_u16.wrapping_add(1_u16));
        gf256_pow(a, exp)
    }
}

#[inline]
fn cauchy_parity_coeff(parity_row: u8, data_col: u8, k: u8) -> u8 {
    // Distinct elements: x_i = k + parity_row, y_j = data_col => x_i ^ y_j != 0 for all i, j.
    let x = k.wrapping_add(parity_row);
    gf256_inv(x ^ data_col)
}

fn invert_gf256_matrix(mut mat: Vec<Vec<u8>>) -> Result<Vec<Vec<u8>>, StitchCustodyError> {
    let k = mat.len();
    let mut inv = vec![vec![0u8; k]; k];
    for (i, row) in inv.iter_mut().enumerate() {
        row[i] = 1;
    }
    for col in 0..k {
        let mut pivot = col;
        while pivot < k && mat[pivot][col] == 0 {
            pivot += 1;
        }
        if pivot == k {
            return Err(StitchCustodyError::IncompatibleFragmentPair {
                left_index: 0,
                right_index: 0,
                detail: "singular GF(2^8) Reed-Solomon decoding matrix".to_string(),
            });
        }
        if pivot != col {
            mat.swap(col, pivot);
            inv.swap(col, pivot);
        }
        let inv_diag = gf256_inv(mat[col][col]);
        for j in 0..k {
            mat[col][j] = gf256_mul(mat[col][j], inv_diag);
            inv[col][j] = gf256_mul(inv[col][j], inv_diag);
        }
        for row in 0..k {
            if row == col {
                continue;
            }
            let factor = mat[row][col];
            if factor != 0 {
                for j in 0..k {
                    mat[row][j] ^= gf256_mul(factor, mat[col][j]);
                    inv[row][j] ^= gf256_mul(factor, inv[col][j]);
                }
            }
        }
    }
    Ok(inv)
}

#[path = "network_custody.rs"]
mod network_custody;
pub use network_custody::*;
