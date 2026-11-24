//! Canonical SHA-256 & lowercase hexadecimal helpers (`digest.rs`).
//!
//! Consolidates content-addressed digest formatting so `operator`, `vcs`, `standards`, `runtime`,
//! `network`, and `ratchet_sync` share a single zero-magic-constants (`FI-08`) implementation.

use sha2::{Digest, Sha256};

use crate::math::{four_u32, sixteen_u32, two_u32};

/// Encodes an arbitrary byte slice as lowercase hexadecimal.
#[must_use]
pub fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 1 << (1 + 1 + 1 + 1)] = b"0123456789abcdef";
    let shift = four_u32();
    let mask = sixteen_u32().wrapping_sub(1_u32) as usize;
    let mut out = String::with_capacity(bytes.len().saturating_mul(two_u32() as usize));
    for &b in bytes {
        let v = usize::from(b);
        out.push(HEX[(v >> shift) & mask] as char);
        out.push(HEX[v & mask] as char);
    }
    out
}

/// Computes the canonical lowercase SHA-256 hex digest of `bytes`.
#[must_use]
pub fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hex_encode(&hasher.finalize())
}

/// Computes a domain-separated SHA-256 hex digest over `domain_prefix || tag || ":" || bytes`.
#[must_use]
pub fn sha256_hex_tagged(domain_prefix: &str, tag: &str, bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(domain_prefix.as_bytes());
    hasher.update(tag.as_bytes());
    hasher.update(b":");
    hasher.update(bytes);
    hex_encode(&hasher.finalize())
}
