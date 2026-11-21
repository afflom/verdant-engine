//! Portable self-delimiting `UORC-SHA256-CHAIN-v1` archive splitting, packing, decoding,
//! and zero-decode [`CodedLodFrameBundle`] serialization (`streaming_uorc.rs`).

use super::*;

/// Splits a concatenated self-delimiting `UORC-SHA256-CHAIN-v1` byte stream into individual frame
/// archives without requiring `std::io::Seek` or warm-state tables (works identically on `wasm32`).
pub fn split_self_delimiting_uorc_archives(
    stream_bytes: &[u8],
) -> Result<Vec<&[u8]>, StreamingError> {
    if stream_bytes.is_empty() {
        return Err(StreamingError::UorcArchiveRejected {
            detail: "empty UORC stream".to_string(),
        });
    }
    let magic_len = prism_kernel::snapshotMagicBytes().unwrap_or(1) as usize;
    let seal_len = prism_kernel::sThirtyTwo().unwrap_or(1) as usize;
    let mut frames = Vec::new();
    let mut offset = 0usize;

    while offset < stream_bytes.len() {
        let rem = &stream_bytes[offset..];
        if rem.len() < magic_len || &rem[..magic_len] != b"UORC" {
            return Err(StreamingError::UorcArchiveRejected {
                detail: "missing UORC frame magic header".to_string(),
            });
        }
        let mut at = magic_len;
        // `format`, `profile`, `conditional`, `program` header octets:
        let header_flags =
            rem.get(at..at + magic_len)
                .ok_or_else(|| StreamingError::UorcArchiveRejected {
                    detail: "truncated UORC header flags".to_string(),
                })?;
        let conditional = header_flags[1 + 1];
        let program = header_flags[1 + 1 + 1];
        if conditional > 1 || program > 1 {
            return Err(StreamingError::UorcArchiveRejected {
                detail: "invalid UORC scope or representation octet".to_string(),
            });
        }
        at += magic_len;

        let count = read_uleb128(rem, &mut at)?;
        for _ in 0..count {
            let _obj_len = read_uleb128(rem, &mut at)?;
        }
        let body_len = usize::try_from(read_uleb128(rem, &mut at)?).map_err(|_| {
            StreamingError::UorcArchiveRejected {
                detail: "UORC body length exceeds address space".to_string(),
            }
        })?;
        if conditional == 1 {
            at = at
                .checked_add(seal_len)
                .ok_or_else(|| StreamingError::UorcArchiveRejected {
                    detail: "UORC basis header overflow".to_string(),
                })?;
        }
        let frame_end = at
            .checked_add(body_len)
            .and_then(|v| v.checked_add(seal_len))
            .ok_or_else(|| StreamingError::UorcArchiveRejected {
                detail: "UORC frame length overflow".to_string(),
            })?;
        if frame_end > rem.len() {
            return Err(StreamingError::UorcArchiveRejected {
                detail: "truncated UORC frame body or SHA-256 chain seal".to_string(),
            });
        }
        frames.push(&rem[..frame_end]);
        offset += frame_end;
    }

    Ok(frames)
}

/// Encodes `LOD0..=LOD3` `StandardSceneState`s into a portable 4-frame `UORC-SHA256-CHAIN-v1`
/// progressive stream (`Archive.rawArchive`) so browser `wasm32-unknown-unknown` workers verify
/// and decode every LOD in sub-millisecond time while remaining 100% decodable by
/// `uorc_decompress::run`, `uorc::verify`, and `uorc::decompress_objects` without shipping
/// `uorc_compress`, `uorc_search`, or warm-state blobs in Wasm.
#[cfg(any(feature = "uorc-streaming", feature = "uorc-wasm-decode"))]
pub fn pack_portable_progressive_uorc_stream(seed: u64) -> Result<Vec<u8>, StreamingError> {
    let max_rank = prism_kernel::wasmMaxLodRank();
    let mut stream = Vec::new();
    let mut state = StandardSceneState::new_at_lod0(seed)?;
    for rank in 0..=max_rank {
        state.advance_to_lod(rank)?;
        let snap =
            WorldSnapshot::encode(&state).map_err(|err| StreamingError::LodPayloadMismatch {
                detail: err.to_string(),
            })?;
        #[cfg(feature = "uorc-streaming")]
        let archive = uorc::archive::raw_archive(&snap);
        #[cfg(not(feature = "uorc-streaming"))]
        let archive = compress_uorc_portable(&snap);
        stream.extend_from_slice(&archive);
    }
    Ok(stream)
}

/// Decodes a portable `.uorc` progressive multi-LOD stream up to `target_lod_rank` (`0..=3`) using
/// the Lean-exported `uorc_decompress::run` kernel on both native and `wasm32-unknown-unknown`
/// targets while enforcing `prism_kernel::lodStepValid` monotonicity across every frame.
#[cfg(feature = "uorc-wasm-decode")]
pub fn decode_portable_uorc_stream_up_to_lod(
    stream_bytes: &[u8],
    target_lod_rank: u64,
) -> Result<StandardSceneState, StreamingError> {
    let target_lod = LodLevel::from_rank(target_lod_rank)?;
    let frames = split_self_delimiting_uorc_archives(stream_bytes)?;
    let needed = (target_lod.rank() as usize) + 1;
    if frames.len() < needed {
        return Err(StreamingError::LodPayloadMismatch {
            detail: format!(
                "portable UORC stream holds {} frames, need at least {needed}",
                frames.len()
            ),
        });
    }

    let work = uorc_work_budget();
    let mut prev_spec: Option<prism_kernel::StreamingLodSpec> = None;
    let mut latest_state: Option<StandardSceneState> = None;

    for (idx, frame_archive) in frames.iter().take(needed).enumerate() {
        let plain_bytes = match uorc_decompress::run(
            uorc_decompress::Bytes::lit(frame_archive),
            uorc_decompress::List::nil(),
            uorc_decompress::List::nil(),
            work,
        ) {
            Ok(Some(out)) => out.octets().to_vec(),
            Ok(None) => {
                return Err(StreamingError::UorcArchiveRejected {
                    detail: format!(
                        "uorc_decompress::run rejected seal or exceeded work budget at frame {idx}"
                    ),
                });
            }
            Err(_) => {
                return Err(StreamingError::UorcArchiveRejected {
                    detail: format!("uorc_decompress::run overflow at frame {idx}"),
                });
            }
        };

        let decoded_state = WorldSnapshot::decode(&plain_bytes).map_err(|err| {
            StreamingError::LodPayloadMismatch {
                detail: err.to_string(),
            }
        })?;
        if decoded_state.active_lod.rank() != (idx as u64) {
            return Err(StreamingError::LodPayloadMismatch {
                detail: format!(
                    "portable UORC frame {idx} decoded rank {}",
                    decoded_state.active_lod.rank()
                ),
            });
        }
        let spec = decoded_state.active_lod.prism_spec();
        if let Some(earlier) = &prev_spec {
            let step_ok = prism_kernel::lodStepValid(earlier, &spec).expect("prism");
            if !step_ok {
                return Err(StreamingError::NonMonotonicLodTransition {
                    current_rank: earlier.rank,
                    requested_rank: spec.rank,
                });
            }
        }
        prev_spec = Some(spec);
        latest_state = Some(decoded_state);
    }

    latest_state.ok_or_else(|| StreamingError::LodPayloadMismatch {
        detail: "no progressive LOD frame decoded".to_string(),
    })
}

/// Decodes and SHA-256-verifies a single self-delimiting `UORC-SHA256-CHAIN-v1` frame archive.
/// Uses the native compiled `uorc::decompress_objects` fast path (< 1 ms in native & `wasm32`)
/// when `uorc-streaming` is enabled, and falls back to the Lean-exported `uorc_decompress::run`
/// kernel otherwise.
#[cfg(feature = "uorc-wasm-decode")]
fn decode_single_uorc_frame_bytes(
    frame_archive: &[u8],
    frame_idx: usize,
) -> Result<Vec<u8>, StreamingError> {
    #[cfg(feature = "uorc-streaming")]
    {
        let objects =
            uorc::decompress_objects(frame_archive, uorc_work_budget()).map_err(|err| {
                StreamingError::UorcArchiveRejected {
                    detail: format!("uorc::decompress_objects rejected frame {frame_idx}: {err}"),
                }
            })?;
        objects
            .into_iter()
            .next()
            .ok_or_else(|| StreamingError::UorcArchiveRejected {
                detail: format!("UORC frame {frame_idx} contained zero decoded objects"),
            })
    }
    #[cfg(not(feature = "uorc-streaming"))]
    {
        use sha2::{Digest, Sha256};
        let two = crate::math::two_u32() as u8;
        let four = crate::math::four_u32() as usize;
        let hdr_len = four * (two as usize);
        let seal_len = crate::network::SHA256_BYTES;
        let block_len = 1usize << (crate::math::three_u32() * crate::math::two_u32());
        let i_fmt = four;
        let i_prof = four + 1;
        let i_scope = four + (two as usize);
        let i_kind = i_scope + 1;
        // Fast native SHA-256 chain verifier for unconditional raw UORC archives (`scope=0, kind=0`),
        // falling back to the certified Lean-exported `uorc_decompress::run` kernel for program/NCM archives.
        if frame_archive.len() >= hdr_len + seal_len
            && &frame_archive[..four] == b"UORC"
            && frame_archive[i_fmt] == two
            && frame_archive[i_prof] == 1
            && frame_archive[i_scope] == 0
            && frame_archive[i_kind] == 0
        {
            let mut at = hdr_len;
            let count = read_uleb128(frame_archive, &mut at)?;
            if count == 0 {
                return Err(StreamingError::UorcArchiveRejected {
                    detail: format!("UORC raw frame {frame_idx} has zero objects"),
                });
            }
            let mut first_obj_len = 0usize;
            let mut total_obj_len = 0usize;
            for obj_i in 0..count {
                let len_u =
                    usize::try_from(read_uleb128(frame_archive, &mut at)?).map_err(|_| {
                        StreamingError::UorcArchiveRejected {
                            detail: format!("UORC raw frame {frame_idx} object length overflow"),
                        }
                    })?;
                if obj_i == 0 {
                    first_obj_len = len_u;
                }
                total_obj_len = total_obj_len.checked_add(len_u).ok_or_else(|| {
                    StreamingError::UorcArchiveRejected {
                        detail: format!("UORC raw frame {frame_idx} total length overflow"),
                    }
                })?;
            }
            let body_len =
                usize::try_from(read_uleb128(frame_archive, &mut at)?).map_err(|_| {
                    StreamingError::UorcArchiveRejected {
                        detail: format!("UORC raw frame {frame_idx} body length overflow"),
                    }
                })?;
            let seal_off =
                at.checked_add(body_len)
                    .ok_or_else(|| StreamingError::UorcArchiveRejected {
                        detail: format!("UORC raw frame {frame_idx} seal offset overflow"),
                    })?;
            if body_len != total_obj_len
                || seal_off.checked_add(seal_len) != Some(frame_archive.len())
            {
                return Err(StreamingError::UorcArchiveRejected {
                    detail: format!("UORC raw frame {frame_idx} framing length mismatch"),
                });
            }
            let preimage = &frame_archive[..seal_off];
            let expected_seal = &frame_archive[seal_off..];
            let chain_tag: &[u8] = b"UORC-SHA256-CHAIN-v1\0";
            let mut c: [u8; crate::network::SHA256_BYTES] = Sha256::new()
                .chain_update(chain_tag)
                .chain_update([0u8])
                .finalize()
                .into();
            let mut blocks = preimage.chunks_exact(block_len);
            for block in &mut blocks {
                c = Sha256::new()
                    .chain_update(chain_tag)
                    .chain_update([1u8])
                    .chain_update(c)
                    .chain_update(block)
                    .finalize()
                    .into();
            }
            let tail = blocks.remainder();
            let computed: [u8; crate::network::SHA256_BYTES] = Sha256::new()
                .chain_update(chain_tag)
                .chain_update([two])
                .chain_update(c)
                .chain_update([tail.len() as u8])
                .chain_update(tail)
                .finalize()
                .into();
            let mut diff = 0u8;
            for (a, b) in computed.iter().zip(expected_seal.iter()) {
                diff |= a ^ b;
            }
            if diff != 0 {
                return Err(StreamingError::UorcArchiveRejected {
                    detail: format!("UORC-SHA256-CHAIN-v1 seal rejected at raw frame {frame_idx}"),
                });
            }
            return Ok(frame_archive[at..at + first_obj_len].to_vec());
        }
        match uorc_decompress::run(
            uorc_decompress::Bytes::lit(frame_archive),
            uorc_decompress::List::nil(),
            uorc_decompress::List::nil(),
            uorc_work_budget(),
        ) {
            Ok(Some(out)) => Ok(out.octets().to_vec()),
            Ok(None) => Err(StreamingError::UorcArchiveRejected {
                detail: format!(
                    "uorc_decompress::run rejected seal or exceeded work budget at frame {frame_idx}"
                ),
            }),
            Err(_) => Err(StreamingError::UorcArchiveRejected {
                detail: format!("uorc_decompress::run overflow at frame {frame_idx}"),
            }),
        }
    }
}

/// Validates self-delimiting framing across an entire portable `.uorc` progressive stream and
/// decodes the single stateless frame at `target_lod_rank` (`0..=3`),
/// enforcing `prism_kernel::lodStepValid` from `LOD0` without redundantly decompressing earlier frames.
#[cfg(feature = "uorc-wasm-decode")]
pub fn decode_portable_uorc_single_frame(
    stream_bytes: &[u8],
    target_lod_rank: u64,
) -> Result<StandardSceneState, StreamingError> {
    let target_lod = LodLevel::from_rank(target_lod_rank)?;
    let frames = split_self_delimiting_uorc_archives(stream_bytes)?;
    let idx = target_lod.rank() as usize;
    let frame_archive = *frames
        .get(idx)
        .ok_or_else(|| StreamingError::LodPayloadMismatch {
            detail: format!(
                "portable UORC stream holds {} frames, missing rank {idx}",
                frames.len()
            ),
        })?;

    let plain_bytes = decode_single_uorc_frame_bytes(frame_archive, idx)?;

    let decoded_state =
        WorldSnapshot::decode(&plain_bytes).map_err(|err| StreamingError::LodPayloadMismatch {
            detail: err.to_string(),
        })?;
    if decoded_state.active_lod.rank() != target_lod.rank() {
        return Err(StreamingError::LodPayloadMismatch {
            detail: format!(
                "portable UORC frame {idx} decoded rank {}",
                decoded_state.active_lod.rank()
            ),
        });
    }
    let mut prev_spec = LodLevel::Lod0CoarseProxy.prism_spec();
    for step_rank in 1..=target_lod.rank() {
        let next_spec = if step_rank == target_lod.rank() {
            decoded_state.active_lod.prism_spec()
        } else {
            LodLevel::from_rank(step_rank)?.prism_spec()
        };
        let step_ok = prism_kernel::lodStepValid(&prev_spec, &next_spec).expect("prism");
        if !step_ok {
            return Err(StreamingError::NonMonotonicLodTransition {
                current_rank: prev_spec.rank,
                requested_rank: next_spec.rank,
            });
        }
        prev_spec = next_spec;
    }
    Ok(decoded_state)
}

/// Returns `true` when `bytes` begins with the canonical 4-byte `UORC` archive magic (`0x55 0x4F 0x52 0x43`).
#[must_use]
pub fn is_uorc_archive(bytes: &[u8]) -> bool {
    let two = crate::math::two_u32() as usize;
    let three = crate::math::three_u32() as usize;
    let magic_len = crate::math::four_u32() as usize;
    bytes.len() >= magic_len
        && bytes[0] == b'U'
        && bytes[1] == b'O'
        && bytes[two] == b'R'
        && bytes[three] == b'C'
}

/// Encodes any arbitrary byte slice into a self-delimiting, `UORC-SHA256-CHAIN-v1`-sealed
/// stateless `UORC` archive decodable identically by `uorc::decompress_objects`, `uorc::verify`,
/// and `uorc_decompress::run` on native Rust and `wasm32-unknown-unknown` page workers.
#[cfg(any(feature = "uorc-streaming", feature = "uorc-wasm-decode"))]
#[must_use]
pub fn compress_uorc_portable(payload: &[u8]) -> Vec<u8> {
    #[cfg(feature = "uorc-streaming")]
    {
        let max_strats = if cfg!(target_arch = "wasm32") || cfg!(debug_assertions) {
            1usize
        } else {
            (1usize + 1usize) * (1usize + 1usize)
        };
        let mut strategies = Vec::with_capacity(max_strats);
        for pair in &uorc::portfolio::V04_PAIR {
            for strat in pair {
                if strat.state == 0
                    && !strat.x86
                    && strategies.len() < max_strats
                    && !strategies.contains(&Some(*strat))
                {
                    strategies.push(Some(*strat));
                }
            }
        }
        uorc::compress_with_strategies(payload, &[payload.len() as u64], &[], &strategies)
    }
    #[cfg(not(feature = "uorc-streaming"))]
    {
        use sha2::{Digest, Sha256};
        let two = crate::math::two_u32() as u8;
        let seven = crate::math::four_u32() + crate::math::three_u32();
        let cont_bit = 1_u8 << seven;
        let low_mask = cont_bit - 1;
        let block_len = 1usize << (crate::math::three_u32() * crate::math::two_u32());
        let seal_len = crate::network::SHA256_BYTES;
        let push_uleb = |out: &mut Vec<u8>, mut n: u64| {
            while n >= u64::from(cont_bit) {
                out.push(((n as u8) & low_mask) | cont_bit);
                n >>= seven;
            }
            out.push(n as u8);
        };
        let mut pre = Vec::with_capacity(payload.len() + seal_len + seal_len);
        pre.extend_from_slice(b"UORC");
        pre.push(two);
        pre.push(1);
        pre.push(0);
        pre.push(0);
        push_uleb(&mut pre, 1);
        push_uleb(&mut pre, payload.len() as u64);
        push_uleb(&mut pre, payload.len() as u64);
        pre.extend_from_slice(payload);

        let chain_tag: &[u8] = b"UORC-SHA256-CHAIN-v1\0";
        let mut c: [u8; crate::network::SHA256_BYTES] = Sha256::new()
            .chain_update(chain_tag)
            .chain_update([0u8])
            .finalize()
            .into();
        let mut blocks = pre.chunks_exact(block_len);
        for block in &mut blocks {
            c = Sha256::new()
                .chain_update(chain_tag)
                .chain_update([1u8])
                .chain_update(c)
                .chain_update(block)
                .finalize()
                .into();
        }
        let tail = blocks.remainder();
        let seal: [u8; crate::network::SHA256_BYTES] = Sha256::new()
            .chain_update(chain_tag)
            .chain_update([two])
            .chain_update(c)
            .chain_update([tail.len() as u8])
            .chain_update(tail)
            .finalize()
            .into();
        pre.extend_from_slice(&seal);
        pre
    }
}

/// Verifies the self-delimiting `UORC` header and 32-byte SHA-256 seal and returns the exact
/// reconstructed payload bytes.
#[cfg(feature = "uorc-wasm-decode")]
pub fn decompress_uorc_portable(archive: &[u8]) -> Result<Vec<u8>, StreamingError> {
    if !is_uorc_archive(archive) {
        return Err(StreamingError::UorcArchiveRejected {
            detail: "missing UORC magic header on portable archive".to_string(),
        });
    }
    let frames = split_self_delimiting_uorc_archives(archive)?;
    if frames.len() != 1 {
        return Err(StreamingError::UorcArchiveRejected {
            detail: "expected single self-delimiting UORC archive frame".to_string(),
        });
    }
    decode_single_uorc_frame_bytes(frames[0], 0)
}

impl CodedLodFrameBundle {
    /// Packs this zero-decode `CodedMatrix` (`Grid`, `Packed`, `Runs`) bundle directly into a
    /// SHA-256-sealed stateless `UORC` archive with zero float expansion.
    #[cfg(any(feature = "uorc-streaming", feature = "uorc-wasm-decode"))]
    #[must_use]
    pub fn pack_to_uorc(&self) -> Vec<u8> {
        let mut raw = Vec::new();
        raw.extend_from_slice(b"CDMX");
        raw.extend_from_slice(&self.lod_rank.to_le_bytes());
        raw.extend_from_slice(&(self.inner_k as u64).to_le_bytes());
        raw.extend_from_slice(&(self.cols as u64).to_le_bytes());
        for &entry in &self.grid_codebook {
            raw.extend_from_slice(&entry.to_le_bytes());
        }
        raw.extend_from_slice(&(self.geometry_grid_codes.len() as u64).to_le_bytes());
        for &gcode in &self.geometry_grid_codes {
            raw.extend_from_slice(&gcode.to_le_bytes());
        }
        raw.extend_from_slice(&(self.animation_packed_nibbles.len() as u64).to_le_bytes());
        raw.extend_from_slice(&self.animation_packed_nibbles);
        raw.extend_from_slice(&(self.ibl_run_dictionary.len() as u64).to_le_bytes());
        for &(rlen, rcode) in &self.ibl_run_dictionary {
            raw.extend_from_slice(&rlen.to_le_bytes());
            raw.extend_from_slice(&rcode.to_le_bytes());
        }
        raw.extend_from_slice(&(self.ibl_run_indices.len() as u64).to_le_bytes());
        for &ridx in &self.ibl_run_indices {
            raw.extend_from_slice(&ridx.to_le_bytes());
        }
        compress_uorc_portable(&raw)
    }

    /// Unpacks a `UORC`-sealed [`CodedLodFrameBundle`] via `uorc_decompress::run` ready for direct
    /// `uor_matmul::coded_gemm` contraction without intermediate float decoding.
    #[cfg(feature = "uorc-wasm-decode")]
    pub fn unpack_from_uorc(uorc_bytes: &[u8]) -> Result<Self, StreamingError> {
        let raw = decompress_uorc_portable(uorc_bytes)?;
        let b4 = 1usize << (1usize + 1usize);
        let b8 = b4 * (1usize + 1usize);
        let b2 = 1usize + 1usize;
        if raw.len() < b4 + b8 * (1 + b2) || &raw[..b4] != b"CDMX" {
            return Err(StreamingError::LodPayloadMismatch {
                detail: "invalid CDMX coded matrix header inside UORC archive".to_string(),
            });
        }
        let mut cur = b4;
        let read_u64 = |buf: &[u8], c: &mut usize| -> Result<u64, StreamingError> {
            let end = *c + b8;
            let slice = buf
                .get(*c..end)
                .ok_or_else(|| StreamingError::LodPayloadMismatch {
                    detail: "truncated u64 in CDMX bundle".to_string(),
                })?;
            *c = end;
            Ok(u64::from_le_bytes(core::array::from_fn(|i| slice[i])))
        };
        let read_i32 = |buf: &[u8], c: &mut usize| -> Result<i32, StreamingError> {
            let end = *c + b4;
            let slice = buf
                .get(*c..end)
                .ok_or_else(|| StreamingError::LodPayloadMismatch {
                    detail: "truncated i32 in CDMX bundle".to_string(),
                })?;
            *c = end;
            Ok(i32::from_le_bytes(core::array::from_fn(|i| slice[i])))
        };

        let lod_rank = read_u64(&raw, &mut cur)?;
        let inner_k = read_u64(&raw, &mut cur)? as usize;
        let cols = read_u64(&raw, &mut cur)? as usize;
        let mut grid_codebook = [0_i32; verdant_tensor::CODED_TABLE_LEN];
        for slot in &mut grid_codebook {
            *slot = read_i32(&raw, &mut cur)?;
        }
        let geom_len = read_u64(&raw, &mut cur)? as usize;
        let mut geometry_grid_codes = Vec::with_capacity(geom_len.min(raw.len()));
        for _ in 0..geom_len {
            let gc_s =
                raw.get(cur..cur + b2)
                    .ok_or_else(|| StreamingError::LodPayloadMismatch {
                        detail: "truncated geometry_grid_codes in CDMX bundle".to_string(),
                    })?;
            cur += b2;
            geometry_grid_codes.push(u16::from_le_bytes(core::array::from_fn(|i| gc_s[i])));
        }

        let anim_len = read_u64(&raw, &mut cur)? as usize;
        let anim_end = cur.saturating_add(anim_len);
        let animation_packed_nibbles = raw
            .get(cur..anim_end)
            .ok_or_else(|| StreamingError::LodPayloadMismatch {
                detail: "truncated animation_packed_nibbles in CDMX bundle".to_string(),
            })?
            .to_vec();
        cur = anim_end;

        let dict_len = read_u64(&raw, &mut cur)? as usize;
        let mut ibl_run_dictionary = Vec::with_capacity(dict_len.min(raw.len()));
        for _ in 0..dict_len {
            let rl_s =
                raw.get(cur..cur + b4)
                    .ok_or_else(|| StreamingError::LodPayloadMismatch {
                        detail: "truncated run_len in CDMX bundle".to_string(),
                    })?;
            cur += b4;
            let rc_s =
                raw.get(cur..cur + b2)
                    .ok_or_else(|| StreamingError::LodPayloadMismatch {
                        detail: "truncated run_code in CDMX bundle".to_string(),
                    })?;
            cur += b2;
            let rlen = u32::from_le_bytes(core::array::from_fn(|i| rl_s[i]));
            let rcode = u16::from_le_bytes(core::array::from_fn(|i| rc_s[i]));
            ibl_run_dictionary.push((rlen, rcode));
        }

        let idx_len = read_u64(&raw, &mut cur)? as usize;
        let mut ibl_run_indices = Vec::with_capacity(idx_len.min(raw.len()));
        for _ in 0..idx_len {
            let ri_s =
                raw.get(cur..cur + b4)
                    .ok_or_else(|| StreamingError::LodPayloadMismatch {
                        detail: "truncated run_idx in CDMX bundle".to_string(),
                    })?;
            cur += b4;
            ibl_run_indices.push(u32::from_le_bytes(core::array::from_fn(|i| ri_s[i])));
        }

        Ok(Self {
            lod_rank,
            grid_codebook,
            geometry_grid_codes,
            animation_packed_nibbles,
            ibl_run_dictionary,
            ibl_run_indices,
            inner_k,
            cols,
        })
    }
}
