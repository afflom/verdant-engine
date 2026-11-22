//! Sovereign Stitch monotonic release pointers, zero-account reciprocal budget ledgers,
//! multi-origin relay failover plans, peer exchange sessions, and JSON boundary bridges
//! (`network_custody.rs`).

use serde::{Deserialize, Serialize};

use super::{
    cauchy_parity_coeff, gf256_mul, hmac_sha256, invert_gf256_matrix, sha256_digest,
    BoundaryKeyring, BoundaryVisibility, PrismBoundaryCipher, StitchBoundaryEnvelope,
    RELEASE_RS_DATA_SHARDS, RELEASE_RS_TOTAL_SHARDS, SHA256_BYTES,
};
use crate::digest::hex_encode;
use crate::error::StitchCustodyError;

/// Quorum-signed monotonic Stitch release or deep-link pointer (`Threat 14`: anti-rollback).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StitchMonotonicPointer {
    /// Canonical Stitch URI (e.g. `stitch://verdant/releases/stable` or `stitch://verdant/studio`).
    pub stitch_uri: String,
    /// Strictly increasing release epoch (`> persisted_epoch`).
    pub release_epoch: u64,
    /// Prior release `kappa` hex (`"genesis"` for epoch 1).
    pub prev_kappa_hex: String,
    /// Target `.holo` / `.uorc` content `kappa` hex.
    pub target_kappa_hex: String,
    /// Minimum distinct `Prism ID` signatures required (`thresholdMet`).
    pub min_quorum: u32,
    /// Map of signer `Prism ID` -> deterministic hex signature over canonical pointer preimage.
    pub quorum_signatures: std::collections::BTreeMap<String, String>,
}

impl StitchMonotonicPointer {
    fn preimage(
        stitch_uri: &str,
        release_epoch: u64,
        prev_kappa_hex: &str,
        target_kappa_hex: &str,
    ) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(b"stitch/pointer/v1:");
        buf.extend_from_slice(stitch_uri.as_bytes());
        buf.push(b'|');
        buf.extend_from_slice(&release_epoch.to_le_bytes());
        buf.push(b'|');
        buf.extend_from_slice(prev_kappa_hex.as_bytes());
        buf.push(b'|');
        buf.extend_from_slice(target_kappa_hex.as_bytes());
        buf
    }

    /// Signs a monotonic pointer update with one or more `Prism ID` quorum signers.
    pub fn sign_quorum(
        stitch_uri: &str,
        release_epoch: u64,
        prev_kappa_hex: &str,
        target_kappa_hex: &str,
        min_quorum: u32,
        signers: &[(&str, &[u8])],
    ) -> Result<Self, StitchCustodyError> {
        if !stitch_uri.starts_with("stitch://") || release_epoch == 0 || min_quorum == 0 {
            return Err(StitchCustodyError::InvalidJournalSignature {
                detail: "invalid stitch:// URI, epoch, or quorum".to_string(),
            });
        }
        let pre = Self::preimage(stitch_uri, release_epoch, prev_kappa_hex, target_kappa_hex);
        let mut quorum_signatures = std::collections::BTreeMap::new();
        for &(prism_id, secret) in signers {
            if prism_id.starts_with("prism:") && !secret.is_empty() {
                let sig = hex_encode(&hmac_sha256(secret, &pre));
                quorum_signatures.insert(prism_id.to_string(), sig);
            }
        }
        Ok(Self {
            stitch_uri: stitch_uri.to_string(),
            release_epoch,
            prev_kappa_hex: prev_kappa_hex.to_string(),
            target_kappa_hex: target_kappa_hex.to_string(),
            min_quorum,
            quorum_signatures,
        })
    }

    /// Verifies monotonic epoch progression (`> persisted_epoch`), `prev_kappa_hex` chaining, and
    /// `Stitch.PrismId.thresholdMet` quorum signatures against `trusted_quorum_secrets`.
    pub fn verify_advance(
        &self,
        persisted_epoch: u64,
        expected_prev_kappa: Option<&str>,
        trusted_quorum_secrets: &std::collections::BTreeMap<String, Vec<u8>>,
    ) -> Result<(), StitchCustodyError> {
        if self.release_epoch <= persisted_epoch {
            return Err(StitchCustodyError::InvalidJournalSignature {
                detail: format!(
                    "rollback refused: pointer epoch {} <= persisted epoch {persisted_epoch}",
                    self.release_epoch
                ),
            });
        }
        if let Some(prev) = expected_prev_kappa {
            if self.prev_kappa_hex != prev {
                return Err(StitchCustodyError::InvalidJournalSignature {
                    detail: "pointer prev_kappa_hex chain mismatch".to_string(),
                });
            }
        }
        let pre = Self::preimage(
            &self.stitch_uri,
            self.release_epoch,
            &self.prev_kappa_hex,
            &self.target_kappa_hex,
        );
        let mut valid_sigs = 0u32;
        for (prism_id, sig_hex) in &self.quorum_signatures {
            if let Some(secret) = trusted_quorum_secrets.get(prism_id) {
                let expected = hex_encode(&hmac_sha256(secret, &pre));
                if *sig_hex == expected {
                    valid_sigs = valid_sigs.saturating_add(1);
                }
            }
        }
        if valid_sigs < self.min_quorum {
            return Err(StitchCustodyError::InvalidJournalSignature {
                detail: format!(
                    "insufficient Prism ID quorum signatures: got {valid_sigs}, need {}",
                    self.min_quorum
                ),
            });
        }
        Ok(())
    }
}

/// Zero-account per-`Prism-ID` reciprocal storage & flood-control ledger (`Threat 16`,
/// enforcing `identity/stitch.lock` `Stitch.Budget.reserveAllowed` / `Relay.admissionAllowed`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StitchBudgetLedger {
    /// Maximum total bytes a single external `Prism ID` may reserve without reciprocal credit.
    pub per_identity_cap_bytes: u64,
    /// Hard total cache ceiling for unpinned alien peer shards (never evicts local owner data).
    pub alien_pool_cap_bytes: u64,
    /// Active reserved bytes per external `Prism ID`.
    pub identity_usage: std::collections::BTreeMap<String, u64>,
    /// FIFO/LRU queue of unpinned alien shards `(shard_key, owner_prism_id, bytes)`.
    pub alien_shards: Vec<(String, String, u64)>,
    /// Current resident alien bytes.
    pub alien_resident_bytes: u64,
    /// Total alien evictions performed to stay within `alien_pool_cap_bytes`.
    pub alien_evictions: u64,
}

impl StitchBudgetLedger {
    /// Creates a new zero-account budget ledger with hard per-identity and alien-pool ceilings.
    #[must_use]
    pub fn new(per_identity_cap_bytes: u64, alien_pool_cap_bytes: u64) -> Self {
        Self {
            per_identity_cap_bytes: per_identity_cap_bytes.max(1),
            alien_pool_cap_bytes: alien_pool_cap_bytes.max(1),
            identity_usage: std::collections::BTreeMap::new(),
            alien_shards: Vec::new(),
            alien_resident_bytes: 0,
            alien_evictions: 0,
        }
    }

    /// Admits a shard from `peer_prism_id` (zero account needed!) if within its per-identity cap,
    /// evicting oldest unpinned alien shards when needed so `alien_resident_bytes <= alien_pool_cap_bytes`.
    pub fn admit_shard(
        &mut self,
        peer_prism_id: &str,
        shard_key: &str,
        shard_bytes: u64,
    ) -> Result<(), StitchCustodyError> {
        if !peer_prism_id.starts_with("prism:") || shard_bytes == 0 {
            return Err(StitchCustodyError::InvalidJournalSignature {
                detail: "Stitch admission requires valid prism: identity and positive shard_bytes"
                    .to_string(),
            });
        }
        let used = self.identity_usage.get(peer_prism_id).copied().unwrap_or(0);
        if used.saturating_add(shard_bytes) > self.per_identity_cap_bytes
            || shard_bytes > self.alien_pool_cap_bytes
        {
            return Err(StitchCustodyError::StalePeerObservation { peer_id: 0 });
        }
        while self.alien_resident_bytes.saturating_add(shard_bytes) > self.alien_pool_cap_bytes {
            if self.alien_shards.is_empty() {
                break;
            }
            let (_, ev_owner, ev_bytes) = self.alien_shards.remove(0);
            self.alien_resident_bytes = self.alien_resident_bytes.saturating_sub(ev_bytes);
            if let Some(slot) = self.identity_usage.get_mut(&ev_owner) {
                *slot = slot.saturating_sub(ev_bytes);
            }
            self.alien_evictions = self.alien_evictions.saturating_add(1);
        }
        *self
            .identity_usage
            .entry(peer_prism_id.to_string())
            .or_insert(0) += shard_bytes;
        self.alien_shards.push((
            shard_key.to_string(),
            peer_prism_id.to_string(),
            shard_bytes,
        ));
        self.alien_resident_bytes = self.alien_resident_bytes.saturating_add(shard_bytes);
        Ok(())
    }
}

/// JSON bridge for zero-account Prism-ID boundary encryption + systematic k-of-n Reed-Solomon
/// Stitch sharding (`AUTH-STITCH-SELF-HOSTING`), shared across native Rust, MCP, and WebAssembly.
#[must_use]
pub fn stitch_boundary_seal_json(raw_json: &str) -> String {
    let Ok(req) = serde_json::from_str::<serde_json::Value>(raw_json) else {
        return serde_json::json!({ "ok": false, "error": "invalid JSON request" }).to_string();
    };
    let boundary_id = req
        .get("boundaryId")
        .and_then(|v| v.as_str())
        .unwrap_or("stitch://boundary/default");
    let owner = req
        .get("ownerPrismId")
        .and_then(|v| v.as_str())
        .unwrap_or("prism:anonymous-stakeholder");
    let visibility = match req
        .get("visibility")
        .and_then(|v| v.as_str())
        .unwrap_or("PrivateEncrypted")
    {
        "SelectiveShare" => BoundaryVisibility::SelectiveShare,
        "Public" => BoundaryVisibility::Public,
        _ => BoundaryVisibility::PrivateEncrypted,
    };
    let passphrase = req
        .get("secretPassphrase")
        .and_then(|v| v.as_str())
        .unwrap_or("prism-zero-account-local-root");
    let payload = req
        .get("payloadUtf8")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let k = req
        .get("dataShards")
        .and_then(|v| v.as_u64())
        .and_then(|v| u8::try_from(v).ok())
        .unwrap_or(RELEASE_RS_DATA_SHARDS);
    let n = req
        .get("totalShards")
        .and_then(|v| v.as_u64())
        .and_then(|v| u8::try_from(v).ok())
        .unwrap_or(RELEASE_RS_TOTAL_SHARDS);
    let res = BoundaryKeyring::new(boundary_id, owner, passphrase.as_bytes())
        .and_then(|keyring| {
            PrismBoundaryCipher::seal(
                &keyring,
                owner,
                visibility,
                payload.as_bytes(),
                boundary_id.as_bytes(),
            )
        })
        .and_then(|env| {
            let raw = serde_json::to_vec(&env).unwrap_or_default();
            let shards = ReedSolomonStitchCodec::shard_k_of_n(&raw, k, n)?;
            Ok((env, shards))
        });
    match res {
        Ok((env, shards)) => serde_json::json!({
            "ok": true,
            "boundaryId": env.boundary_id,
            "ownerPrismId": env.owner_prism_id,
            "visibility": env.visibility,
            "boundaryEpoch": env.boundary_epoch,
            "shards": shards,
        })
        .to_string(),
        Err(e) => serde_json::json!({ "ok": false, "error": e.to_string() }).to_string(),
    }
}

/// JSON bridge for zero-account Stitch k-of-n Reed-Solomon recovery + Encrypt-then-MAC boundary
/// decryption (`AUTH-STITCH-SELF-HOSTING`), shared across native Rust, MCP, and WebAssembly.
#[must_use]
pub fn stitch_boundary_recover_json(raw_json: &str) -> String {
    let Ok(req) = serde_json::from_str::<serde_json::Value>(raw_json) else {
        return serde_json::json!({ "ok": false, "error": "invalid JSON request" }).to_string();
    };
    let boundary_id = req
        .get("boundaryId")
        .and_then(|v| v.as_str())
        .unwrap_or("stitch://boundary/default");
    let owner = req
        .get("ownerPrismId")
        .and_then(|v| v.as_str())
        .unwrap_or("prism:anonymous-stakeholder");
    let passphrase = req
        .get("secretPassphrase")
        .and_then(|v| v.as_str())
        .unwrap_or("prism-zero-account-local-root");
    let shards: Vec<StitchRsShard> = match req
        .get("shards")
        .cloned()
        .and_then(|v| serde_json::from_value(v).ok())
    {
        Some(s) => s,
        None => {
            return serde_json::json!({ "ok": false, "error": "missing or invalid shards" })
                .to_string();
        }
    };
    let res = BoundaryKeyring::new(boundary_id, owner, passphrase.as_bytes()).and_then(|keyring| {
        let raw = ReedSolomonStitchCodec::recover_k_of_n(&shards)?;
        let env: StitchBoundaryEnvelope = serde_json::from_slice(&raw).map_err(|e| {
            StitchCustodyError::PayloadDigestMismatch {
                detail: e.to_string(),
            }
        })?;
        let plain = PrismBoundaryCipher::open(&keyring, owner, &env)?;
        Ok((env, plain))
    });
    match res {
        Ok((env, plain)) => serde_json::json!({
            "ok": true,
            "boundaryId": env.boundary_id,
            "ownerPrismId": env.owner_prism_id,
            "boundaryEpoch": env.boundary_epoch,
            "payloadUtf8": String::from_utf8_lossy(&plain),
        })
        .to_string(),
        Err(e) => serde_json::json!({ "ok": false, "error": e.to_string() }).to_string(),
    }
}

/// Multi-origin Stitch shard distribution & failover plan (`VE-48`, `AUTH-STITCH-SELF-HOSTING`).
/// Guarantees at least two independent HTTPS origins (or localhost test endpoints) so losing any
/// single host never interrupts Blank Frame hydration.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultiOriginRelayPlan {
    /// Ordered independent relay base URLs raced in parallel by the Blank Frame.
    pub relay_urls: Vec<String>,
    /// Canonical lowercase distinct host origins verified to exceed `min_independent_origins`.
    pub distinct_origins: Vec<String>,
    /// Minimum required independent origins (`>= 2`).
    pub min_independent_origins: usize,
}

impl MultiOriginRelayPlan {
    /// Validates and constructs a multi-origin relay plan, refusing single-origin downgrades,
    /// duplicate origins, or non-HTTPS/non-localhost unsafe URLs.
    pub fn build(
        relays: Vec<String>,
        min_independent_origins: usize,
    ) -> Result<Self, StitchCustodyError> {
        let floor = min_independent_origins.max(1usize + 1usize);
        let mut distinct = std::collections::BTreeSet::new();
        let mut clean = Vec::with_capacity(relays.len());
        for raw in relays {
            let trimmed = raw.trim().trim_end_matches('/').to_string();
            let is_https = trimmed.starts_with("https://");
            let is_local_http =
                trimmed.starts_with("http://127.0.0.1") || trimmed.starts_with("http://localhost");
            if !is_https && !is_local_http {
                return Err(StitchCustodyError::InvalidJournalSignature {
                    detail: format!("unsafe or non-HTTPS Stitch relay URL refused: `{trimmed}`"),
                });
            }
            let host = trimmed
                .split("://")
                .nth(1)
                .and_then(|rest| rest.split('/').next())
                .unwrap_or("")
                .to_ascii_lowercase();
            if host.is_empty() {
                return Err(StitchCustodyError::InvalidJournalSignature {
                    detail: format!("Stitch relay URL missing host: `{trimmed}`"),
                });
            }
            distinct.insert(host);
            clean.push(trimmed);
        }
        if distinct.len() < floor {
            return Err(StitchCustodyError::IncompatibleFragmentPair {
                left_index: distinct.len() as u8,
                right_index: floor as u8,
                detail: format!(
                    "multi-origin resilience requires >= {floor} independent origins (found {})",
                    distinct.len()
                ),
            });
        }
        Ok(Self {
            relay_urls: clean,
            distinct_origins: distinct.into_iter().collect(),
            min_independent_origins: floor,
        })
    }

    /// Selects surviving relay URLs after excluding any failed origin hostnames, failing closed
    /// only when zero relays remain.
    pub fn surviving_relays(
        &self,
        failed_hosts: &[&str],
    ) -> Result<Vec<String>, StitchCustodyError> {
        let mut alive = Vec::new();
        for url in &self.relay_urls {
            let host = url
                .split("://")
                .nth(1)
                .and_then(|rest| rest.split('/').next())
                .unwrap_or("")
                .to_ascii_lowercase();
            if !failed_hosts
                .iter()
                .any(|dead| dead.eq_ignore_ascii_case(&host))
            {
                alive.push(url.clone());
            }
        }
        if alive.is_empty() {
            return Err(StitchCustodyError::StalePeerObservation { peer_id: 0 });
        }
        Ok(alive)
    }
}

/// Zero-account browser-to-browser WebRTC DataChannel / BroadcastChannel reciprocal shard seeder
/// (`VE-48`), enforcing SHA-256 `STCH2` verification and [`StitchBudgetLedger`] credit ceilings.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StitchPeerExchangeSession {
    /// Local zero-account `Prism ID` (`prism:...`).
    pub local_prism_id: String,
    /// Reciprocal budget & alien-pool eviction ledger.
    pub ledger: StitchBudgetLedger,
    /// Verified binary `STCH2` shards cached locally keyed by `(manifest_root_hex:index)`.
    pub verified_shards: std::collections::BTreeMap<String, Vec<u8>>,
    /// Total verified shards served to peers.
    pub served_shards: u64,
    /// Total verified shards received from peers.
    pub received_shards: u64,
}

impl StitchPeerExchangeSession {
    /// Creates a reciprocal peer exchange session for `local_prism_id`.
    pub fn new(
        local_prism_id: &str,
        per_identity_cap_bytes: u64,
        alien_pool_cap_bytes: u64,
    ) -> Result<Self, StitchCustodyError> {
        if !local_prism_id.starts_with("prism:") {
            return Err(StitchCustodyError::InvalidJournalSignature {
                detail: "peer exchange requires valid zero-account prism: identity".to_string(),
            });
        }
        Ok(Self {
            local_prism_id: local_prism_id.to_string(),
            ledger: StitchBudgetLedger::new(per_identity_cap_bytes, alien_pool_cap_bytes),
            verified_shards: std::collections::BTreeMap::new(),
            served_shards: 0,
            received_shards: 0,
        })
    }

    /// Receives, SHA-256-verifies, and budget-admits a binary `STCH2` shard from `peer_prism_id`.
    pub fn receive_verified_peer_shard(
        &mut self,
        peer_prism_id: &str,
        stch2_bytes: &[u8],
    ) -> Result<StitchRsShard, StitchCustodyError> {
        let shard = StitchRsShard::from_binary_stch2(stch2_bytes)?;
        let key = format!("{}:{}", hex_encode(&shard.manifest_root), shard.index);
        self.ledger
            .admit_shard(peer_prism_id, &key, stch2_bytes.len() as u64)?;
        self.verified_shards.insert(key, stch2_bytes.to_vec());
        self.received_shards = self.received_shards.saturating_add(1);
        Ok(shard)
    }

    /// Serves a verified cached shard to `requester_prism_id`, enforcing per-identity budget
    /// reciprocity so flood peers cannot drain bandwidth without contributing.
    pub fn serve_shard_to_peer(
        &mut self,
        requester_prism_id: &str,
        shard_key: &str,
    ) -> Result<Vec<u8>, StitchCustodyError> {
        let Some(bytes) = self.verified_shards.get(shard_key).cloned() else {
            return Err(StitchCustodyError::PayloadDigestMismatch {
                detail: format!("requested Stitch shard `{shard_key}` not resident on peer"),
            });
        };
        self.ledger
            .admit_shard(requester_prism_id, shard_key, bytes.len() as u64)?;
        self.served_shards = self.served_shards.saturating_add(1);
        Ok(bytes)
    }
}

/// Self-verifying `k-of-n` systematic Reed-Solomon (`GF(2^8)`) Stitch shard (`STCH2`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StitchRsShard {
    /// Full `SHA-256` commitment of the original unfragmented payload.
    pub manifest_root: [u8; SHA256_BYTES],
    /// Exact original byte length.
    pub original_byte_len: usize,
    /// Minimum shards required for bit-exact reconstruction (`k >= 1`).
    pub data_shards_k: u8,
    /// Total shards produced (`n >= k`).
    pub total_shards_n: u8,
    /// Shard slot (`0..k` systematic data shards, `k..n` Cauchy Reed-Solomon parity shards).
    pub index: u8,
    /// Per-shard `SHA-256` commitment over `payload`.
    pub payload_root: [u8; SHA256_BYTES],
    /// Fixed-length shard payload (`ceil(original_byte_len / k)` bytes).
    pub payload: Vec<u8>,
}

/// 8-byte binary header magic for compact `STCH2` Reed-Solomon shard frames.
pub const STCH2_BINARY_MAGIC: &[u8; 1 << (1 + 1 + 1)] = b"STCH2_RS";

impl StitchRsShard {
    /// Encodes this shard into a compact zero-overhead binary `STCH2` frame:
    /// `[8B magic][1B k][1B n][1B index][1B 0][4B original_len LE][32B manifest_root][32B payload_root][payload]`.
    #[must_use]
    pub fn to_binary_stch2(&self) -> Vec<u8> {
        let meta_len = (1usize << (1 + 1 + 1))
            + (1usize + 1usize + 1usize + 1usize)
            + (1usize + 1usize + 1usize + 1usize)
            + SHA256_BYTES
            + SHA256_BYTES;
        let mut out = Vec::with_capacity(meta_len + self.payload.len());
        out.extend_from_slice(STCH2_BINARY_MAGIC);
        out.push(self.data_shards_k);
        out.push(self.total_shards_n);
        out.push(self.index);
        out.push(0);
        out.extend_from_slice(&(self.original_byte_len as u32).to_le_bytes());
        out.extend_from_slice(&self.manifest_root);
        out.extend_from_slice(&self.payload_root);
        out.extend_from_slice(&self.payload);
        out
    }

    /// Parses and SHA-256-verifies a compact binary `STCH2` Reed-Solomon shard frame.
    pub fn from_binary_stch2(bytes: &[u8]) -> Result<Self, StitchCustodyError> {
        let magic_len = STCH2_BINARY_MAGIC.len();
        let u32_len = 1usize + 1usize + 1usize + 1usize;
        let roots_start = magic_len + u32_len + u32_len;
        let payload_start = roots_start + SHA256_BYTES + SHA256_BYTES;
        if bytes.len() <= payload_start || &bytes[..magic_len] != STCH2_BINARY_MAGIC {
            return Err(StitchCustodyError::PayloadDigestMismatch {
                detail: "invalid STCH2 binary shard magic or truncated header".to_string(),
            });
        }
        let data_shards_k = bytes[magic_len];
        let total_shards_n = bytes[magic_len + 1];
        let index = bytes[magic_len + 1 + 1];
        let len_off = magic_len + u32_len;
        let mut len_buf = [0u8; 1 + 1 + 1 + 1];
        len_buf.copy_from_slice(&bytes[len_off..roots_start]);
        let original_byte_len = u32::from_le_bytes(len_buf) as usize;
        let mut manifest_root = [0u8; SHA256_BYTES];
        manifest_root.copy_from_slice(&bytes[roots_start..roots_start + SHA256_BYTES]);
        let mut payload_root = [0u8; SHA256_BYTES];
        payload_root.copy_from_slice(&bytes[roots_start + SHA256_BYTES..payload_start]);
        let payload = bytes[payload_start..].to_vec();
        if sha256_digest(&payload) != payload_root {
            return Err(StitchCustodyError::PayloadDigestMismatch {
                detail: format!("STCH2 shard {index} failed SHA-256 payload verification"),
            });
        }
        Ok(Self {
            manifest_root,
            original_byte_len,
            data_shards_k,
            total_shards_n,
            index,
            payload_root,
            payload,
        })
    }
}

/// Systematic `k-of-n` Reed-Solomon (`GF(2^8)`) Stitch codec supporting `4-of-8` archival custody
/// and autonomous parity regeneration (`Threat 17`).
pub struct ReedSolomonStitchCodec;

impl ReedSolomonStitchCodec {
    /// Encodes `payload` into `n` self-verifying systematic `k-of-n` shards (`1 <= k <= n <= 64`).
    pub fn shard_k_of_n(
        payload: &[u8],
        k: u8,
        n: u8,
    ) -> Result<Vec<StitchRsShard>, StitchCustodyError> {
        let max_n = 1u8 << (1 + 1 + 1 + 1 + 1 + 1);
        if payload.is_empty() || k == 0 || n < k || n > max_n {
            return Err(StitchCustodyError::IncompatibleFragmentPair {
                left_index: k,
                right_index: n,
                detail: "invalid k-of-n Reed-Solomon shard geometry or empty payload".to_string(),
            });
        }
        let manifest_root = sha256_digest(payload);
        let k_usize = usize::from(k);
        let n_usize = usize::from(n);
        let shard_len = payload.len().div_ceil(k_usize);
        let mut data_blocks = vec![vec![0u8; shard_len]; k_usize];
        for (i, blk) in data_blocks.iter_mut().enumerate() {
            let start = i * shard_len;
            if start < payload.len() {
                let end = (start + shard_len).min(payload.len());
                blk[..end - start].copy_from_slice(&payload[start..end]);
            }
        }
        let mut shards = Vec::with_capacity(n_usize);
        for (i, blk) in data_blocks.iter().enumerate() {
            let payload_root = sha256_digest(blk);
            shards.push(StitchRsShard {
                manifest_root,
                original_byte_len: payload.len(),
                data_shards_k: k,
                total_shards_n: n,
                index: i as u8,
                payload_root,
                payload: blk.clone(),
            });
        }
        for p in 0..(n_usize - k_usize) {
            let mut parity = vec![0u8; shard_len];
            for (c, data_blk) in data_blocks.iter().enumerate() {
                let coeff = cauchy_parity_coeff(p as u8, c as u8, k);
                for (dst, &src) in parity.iter_mut().zip(data_blk.iter()) {
                    *dst ^= gf256_mul(coeff, src);
                }
            }
            let payload_root = sha256_digest(&parity);
            shards.push(StitchRsShard {
                manifest_root,
                original_byte_len: payload.len(),
                data_shards_k: k,
                total_shards_n: n,
                index: (k_usize + p) as u8,
                payload_root,
                payload: parity,
            });
        }
        Ok(shards)
    }

    /// Reconstructs the exact original payload from ANY `k` distinct verified shards in `shards`.
    pub fn recover_k_of_n(shards: &[StitchRsShard]) -> Result<Vec<u8>, StitchCustodyError> {
        let Some(first) = shards.first() else {
            return Err(StitchCustodyError::IncompatibleFragmentPair {
                left_index: 0,
                right_index: 0,
                detail: "no Reed-Solomon shards supplied".to_string(),
            });
        };
        let k = first.data_shards_k;
        let n = first.total_shards_n;
        let k_usize = usize::from(k);
        if k == 0 || n < k || shards.len() < k_usize {
            return Err(StitchCustodyError::IncompatibleFragmentPair {
                left_index: shards.len() as u8,
                right_index: k,
                detail: "insufficient Reed-Solomon shards for k-of-n reconstruction".to_string(),
            });
        }
        let shard_len = first.payload.len();
        let mut selected: Vec<&StitchRsShard> = Vec::with_capacity(k_usize);
        let mut seen = std::collections::BTreeSet::new();
        for s in shards {
            if s.manifest_root != first.manifest_root
                || s.original_byte_len != first.original_byte_len
                || s.data_shards_k != k
                || s.total_shards_n != n
                || s.payload.len() != shard_len
                || s.index >= n
            {
                return Err(StitchCustodyError::IncompatibleFragmentPair {
                    left_index: first.index,
                    right_index: s.index,
                    detail: "mismatched Reed-Solomon shard metadata".to_string(),
                });
            }
            if sha256_digest(&s.payload) != s.payload_root {
                return Err(StitchCustodyError::PayloadDigestMismatch {
                    detail: format!("Reed-Solomon shard {} failed SHA-256 verification", s.index),
                });
            }
            if seen.insert(s.index) && selected.len() < k_usize {
                selected.push(s);
            }
        }
        if selected.len() < k_usize {
            return Err(StitchCustodyError::IncompatibleFragmentPair {
                left_index: selected.len() as u8,
                right_index: k,
                detail: "duplicate shard indices left fewer than k distinct shards".to_string(),
            });
        }
        let mut sub_mat = vec![vec![0u8; k_usize]; k_usize];
        for (r, s) in selected.iter().enumerate() {
            if s.index < k {
                sub_mat[r][usize::from(s.index)] = 1;
            } else {
                let p_row = s.index - k;
                for (c, cell) in sub_mat[r].iter_mut().enumerate().take(k_usize) {
                    *cell = cauchy_parity_coeff(p_row, c as u8, k);
                }
            }
        }
        let inv = invert_gf256_matrix(sub_mat)?;
        let mut out = Vec::with_capacity(k_usize * shard_len);
        for row_coeffs in &inv {
            let mut block = vec![0u8; shard_len];
            for (src_idx, &coeff) in row_coeffs.iter().enumerate() {
                if coeff != 0 {
                    for (dst, &src) in block.iter_mut().zip(selected[src_idx].payload.iter()) {
                        *dst ^= gf256_mul(coeff, src);
                    }
                }
            }
            out.extend_from_slice(&block);
        }
        out.truncate(first.original_byte_len);
        if sha256_digest(&out) != first.manifest_root {
            return Err(StitchCustodyError::PayloadDigestMismatch {
                detail: "reconstructed k-of-n payload failed manifest_root SHA-256 check"
                    .to_string(),
            });
        }
        Ok(out)
    }

    /// Autonomous S3* parity repair: reconstructs from `>= k` surviving shards and regenerates all `n`.
    pub fn repair_k_of_n(
        surviving_shards: &[StitchRsShard],
    ) -> Result<Vec<StitchRsShard>, StitchCustodyError> {
        let Some(first) = surviving_shards.first() else {
            return Err(StitchCustodyError::IncompatibleFragmentPair {
                left_index: 0,
                right_index: 0,
                detail: "cannot repair empty shard set".to_string(),
            });
        };
        let payload = Self::recover_k_of_n(surviving_shards)?;
        Self::shard_k_of_n(&payload, first.data_shards_k, first.total_shards_n)
    }
}
