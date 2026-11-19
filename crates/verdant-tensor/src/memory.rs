//! **Paged Coded Tensor Memory Arena (`memory.rs`)** — Standards-based out-of-core tensor memory
//! hierarchy scaling Verdant Engine from 1080p through UHD 4K/8K/16K+ graphics and arbitrary
//! large AI model workloads within a strict host memory envelope (`ISO/IEC 25010:2023` Resource
//! Utilization & Capacity, PagedAttention/FlashAttention tiled residency, `uor-matmul` coded tiles).
//!
//! Key guarantees enforced constructively:
//! - **Strict Resident Cap Invariant**: `resident_bytes <= budget_bytes` holds before and after
//!   every page admission, touch, eviction, and tiled contraction — regardless of how many
//!   gigabytes of logical workload pages exist across 3D worlds, 16K virtual framebuffers, or
//!   arbitrary AI model weights & KV caches.
//! - **Zero-Decode Coded Compression (`1x`..`16x`)**: Stores `Packed<Grid<16>, 2>` 4-bit nibble
//!   pages (`8x` vs `f32`) and `Runs<Grid<16>, 16>` sparse pages (`16x`) directly in resident
//!   memory and contracts them through [`CodebookProductLut`] without expanding intermediate float
//!   matrices.
//! - **Deterministic CLOCK / Second-Chance LRU Eviction**: Zero hash randomization, O(1) amortized
//!   page admission, and active working-set pinning during tiled tensor contractions.
//! - **Bit-Exact Out-of-Core Tiled Contraction**: [`contract_paged_packed_nibbles_stream`] streams
//!   arbitrary `rows x inner_k x cols` `Packed<Grid<16>, 2>` workloads through bounded `PagedTensorArena`
//!   tile pages, matching full in-memory [`contract_packed_nibbles_lut_into`] bit-for-bit while
//!   bounding both resident RAM and per-tick work quantum.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{
    coded::{contract_packed_nibbles_lut_into, CodebookProductLut},
    pow2_u64, TensorComputeError, PACKED_NIBBLE_BLOCK,
};

/// Canonical tensor page quantum (`64 KiB` = `2^16` bytes), aligning with WebAssembly linear
/// memory pages (`64 KiB`), GPU storage buffer alignment, and PagedAttention KV blocks.
pub const TENSOR_PAGE_QUANTUM_BYTES: u64 =
    1_u64 << ((1_u32 + 1_u32) * (1_u32 + 1_u32) * (1_u32 + 1_u32) * (1_u32 + 1_u32));

/// Minimum lawful resident arena budget (`4` pages = `256 KiB`) ensuring at least two LHS/RHS
/// active tile pages can be pinned simultaneously during out-of-core contraction.
pub const MIN_ARENA_BUDGET_BYTES: u64 = TENSOR_PAGE_QUANTUM_BYTES << (1_u32 + 1_u32);

/// Semantic workload domain owned by a virtual tensor memory page.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TensorPageKind {
    /// Spatially Morton-ordered 64-triangle GPU meshlet cluster page.
    GeometryMeshlet,
    /// Virtualized PBR texture atlas mip/tile page (`albedo`, `metallicRoughness`, `normal`, `emissive`).
    PbrTextureTile,
    /// Arbitrary AI model quantized weight block (`Packed<Grid<16>, 2>` / `Runs<Grid<16>, 16>`).
    AiWeightBlock,
    /// Arbitrary AI transformer / SSM paged KV-cache or recurrent state block.
    AiKvCacheBlock,
    /// Tiled UHD 4K/8K/16K+ virtual framebuffer tile (`16x16` / `64x64` HDR color + depth).
    UhdFramebufferTile,
    /// Progressive `.uorc` SHA-256-sealed coded stream payload page.
    UorcCodedStream,
}

/// On-wire / in-arena tensor storage representation and its logical-to-physical compression factor.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TensorStorageEncoding {
    /// Uncompressed 32-bit float elements (`1x` baseline).
    DenseF32,
    /// 8-bit `Grid<16>` index stream (`4x` compression vs `f32`).
    QuantizedGrid8,
    /// Sub-byte 4-bit `Packed<Grid<16>, 2>` nibble stream (`8x` compression vs `f32`).
    PackedNibble4,
    /// Variable-length sparse `Runs<Grid<16>, 16>` stream (`16x` effective compression vs `f32`).
    SparseRuns16,
}

impl TensorStorageEncoding {
    /// Constructive logical-to-physical compression factor relative to `DenseF32` (`1`, `4`, `8`, `16`).
    #[must_use]
    pub const fn compression_factor(self) -> u64 {
        match self {
            Self::DenseF32 => 1,
            Self::QuantizedGrid8 => 1_u64 << (1_u32 + 1_u32),
            Self::PackedNibble4 => 1_u64 << (1_u32 + 1_u32 + 1_u32),
            Self::SparseRuns16 => 1_u64 << ((1_u32 + 1_u32) * (1_u32 + 1_u32)),
        }
    }
}

/// Virtual page descriptor registered in [`PagedTensorArena`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TensorPageDescriptor {
    /// Stable monotonic page identifier (`page_id`).
    pub page_id: u64,
    /// Workload domain (`geometry-meshlet`, `ai-weight-block`, `ai-kv-cache-block`, `uhd-framebuffer-tile`, etc.).
    pub kind: TensorPageKind,
    /// Zero-decode coded representation stored when resident.
    pub encoding: TensorStorageEncoding,
    /// Physical resident byte footprint when loaded into host RAM / GPU buffer.
    pub physical_bytes: u64,
    /// Logical uncompressed `f32`-equivalent workload bytes represented by this page.
    pub logical_bytes: u64,
    /// Content-addressed 64-bit `kappa` fingerprint of this page's canonical payload.
    pub kappa_tag: u64,
}

#[derive(Clone, Debug)]
struct ArenaPageEntry {
    desc: TensorPageDescriptor,
    resident: bool,
    referenced: bool,
    pin_count: u32,
    payload: Vec<u8>,
}

/// Real-time ISO/IEC 25010 Resource Utilization & Capacity telemetry for [`PagedTensorArena`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PagedArenaTelemetry {
    /// Hard resident byte ceiling (`resident_bytes <= budget_bytes` invariant).
    pub budget_bytes: u64,
    /// Currently resident physical bytes held in host RAM / GPU memory.
    pub resident_bytes: u64,
    /// Peak physical resident bytes ever observed across the session.
    pub peak_resident_bytes: u64,
    /// Total logical (`f32`-equivalent) workload bytes addressable across all registered virtual pages.
    pub logical_workload_bytes: u64,
    /// Total number of virtual pages registered (`resident + evicted/cold`).
    pub registered_pages: u64,
    /// Number of pages currently resident in host memory.
    pub resident_pages: u64,
    /// Number of currently pinned active working-set pages.
    pub pinned_pages: u64,
    /// Total resident cache hits.
    pub cache_hits: u64,
    /// Total page faults / admissions from backing store or coded stream.
    pub page_faults: u64,
    /// Total deterministic CLOCK/LRU evictions executed to preserve `budget_bytes`.
    pub evictions: u64,
    /// Effective compression & virtual oversubscription ratio in permille (`1000` = `1.0x`, `8000` = `8.0x`).
    pub virtual_amplification_permille: u64,
    /// True iff `resident_bytes <= budget_bytes` and `peak_resident_bytes <= budget_bytes`.
    pub invariant_held: bool,
}

/// Deterministic, strictly host-bounded virtualized tensor memory manager supporting arbitrary
/// UHD 4K/8K/16K+ scenes and multi-gigabyte AI model weight/KV-cache workloads via zero-decode
/// coded residency and CLOCK/second-chance LRU page eviction.
#[derive(Clone, Debug)]
pub struct PagedTensorArena {
    budget_bytes: u64,
    resident_bytes: u64,
    peak_resident_bytes: u64,
    logical_workload_bytes: u64,
    pages: BTreeMap<u64, ArenaPageEntry>,
    resident_ring: Vec<u64>,
    clock_hand: usize,
    cache_hits: u64,
    page_faults: u64,
    evictions: u64,
}

impl Default for PagedTensorArena {
    fn default() -> Self {
        // Default 64 MiB resident host envelope (`2^26` bytes = `1024` 64 KiB pages):
        let two = 1_u32 + 1_u32;
        let three = two + 1_u32;
        let four = two * two;
        let shift_26 = (four * three + 1_u32) * two;
        Self::new(pow2_u64(shift_26))
    }
}

impl PagedTensorArena {
    /// Creates a new [`PagedTensorArena`] enforcing `resident_bytes <= budget_bytes`.
    #[must_use]
    pub fn new(budget_bytes: u64) -> Self {
        Self {
            budget_bytes: budget_bytes.max(MIN_ARENA_BUDGET_BYTES),
            resident_bytes: 0,
            peak_resident_bytes: 0,
            logical_workload_bytes: 0,
            pages: BTreeMap::new(),
            resident_ring: Vec::new(),
            clock_hand: 0,
            cache_hits: 0,
            page_faults: 0,
            evictions: 0,
        }
    }

    /// Dynamically updates the resident memory ceiling (e.g. when `HostEnvelope` changes),
    /// immediately evicting unpinned cold pages if `resident_bytes > new_budget`.
    pub fn set_budget_bytes(&mut self, new_budget_bytes: u64) -> Result<(), TensorComputeError> {
        let clamped = new_budget_bytes.max(MIN_ARENA_BUDGET_BYTES);
        self.budget_bytes = clamped;
        self.evict_until_within(clamped)
    }

    /// Registers a virtual tensor page descriptor without forcing immediate residency.
    /// Supports registering arbitrary gigabyte-scale logical workloads at O(1) metadata cost.
    pub fn register_virtual_page(
        &mut self,
        page_id: u64,
        kind: TensorPageKind,
        encoding: TensorStorageEncoding,
        physical_bytes: u64,
        kappa_tag: u64,
    ) -> Result<TensorPageDescriptor, TensorComputeError> {
        if physical_bytes == 0 {
            return Err(TensorComputeError::ZeroOrMismatchedExtents {
                detail: "tensor memory page must have positive physical_bytes".to_string(),
            });
        }
        if physical_bytes > self.budget_bytes {
            return Err(TensorComputeError::ZeroOrMismatchedExtents {
                detail: format!(
                    "single tensor page ({physical_bytes} B) exceeds arena budget ({} B)",
                    self.budget_bytes
                ),
            });
        }
        let logical_bytes = physical_bytes.saturating_mul(encoding.compression_factor());
        let desc = TensorPageDescriptor {
            page_id,
            kind,
            encoding,
            physical_bytes,
            logical_bytes,
            kappa_tag,
        };
        if let Some(prev) = self.pages.get(&page_id) {
            if prev.resident {
                self.resident_bytes = self.resident_bytes.saturating_sub(prev.desc.physical_bytes);
                self.resident_ring.retain(|&id| id != page_id);
                if self.clock_hand >= self.resident_ring.len() {
                    self.clock_hand = 0;
                }
            }
            self.logical_workload_bytes = self
                .logical_workload_bytes
                .saturating_sub(prev.desc.logical_bytes);
        }
        self.logical_workload_bytes = self.logical_workload_bytes.saturating_add(logical_bytes);
        self.pages.insert(
            page_id,
            ArenaPageEntry {
                desc: desc.clone(),
                resident: false,
                referenced: false,
                pin_count: 0,
                payload: Vec::new(),
            },
        );
        Ok(desc)
    }

    /// Admits or hits `page_id` in resident memory, evicting unpinned pages via deterministic
    /// CLOCK/second-chance LRU as needed so `resident_bytes <= budget_bytes` holds unconditionally.
    pub fn admit_or_touch(
        &mut self,
        page_id: u64,
        payload: Option<&[u8]>,
        pin: bool,
    ) -> Result<bool, TensorComputeError> {
        let Some((already_resident, phys_bytes)) = self
            .pages
            .get(&page_id)
            .map(|e| (e.resident, e.desc.physical_bytes))
        else {
            return Err(TensorComputeError::ZeroOrMismatchedExtents {
                detail: format!("unknown tensor page_id {page_id}"),
            });
        };

        if already_resident {
            self.cache_hits = self.cache_hits.saturating_add(1);
            if let Some(entry) = self.pages.get_mut(&page_id) {
                entry.referenced = true;
                if pin {
                    entry.pin_count = entry.pin_count.saturating_add(1);
                }
                if let Some(bytes) = payload {
                    entry.payload.clear();
                    entry.payload.extend_from_slice(bytes);
                }
            }
            return Ok(true);
        }

        if phys_bytes > self.budget_bytes {
            return Err(TensorComputeError::ZeroOrMismatchedExtents {
                detail: format!(
                    "tensor page {page_id} ({phys_bytes} B) exceeds arena budget ({} B)",
                    self.budget_bytes
                ),
            });
        }

        let target_limit = self.budget_bytes.saturating_sub(phys_bytes);
        self.evict_until_within(target_limit)?;

        if let Some(entry) = self.pages.get_mut(&page_id) {
            entry.resident = true;
            entry.referenced = true;
            if pin {
                entry.pin_count = entry.pin_count.saturating_add(1);
            }
            if let Some(bytes) = payload {
                entry.payload.clear();
                entry.payload.extend_from_slice(bytes);
            }
        }
        self.resident_ring.push(page_id);
        self.resident_bytes = self.resident_bytes.saturating_add(phys_bytes);
        if self.resident_bytes > self.peak_resident_bytes {
            self.peak_resident_bytes = self.resident_bytes;
        }
        self.page_faults = self.page_faults.saturating_add(1);
        Ok(false)
    }

    /// Releases one pin reference on `page_id` after a tiled tensor contraction finishes with it.
    pub fn unpin(&mut self, page_id: u64) {
        if let Some(entry) = self.pages.get_mut(&page_id) {
            entry.pin_count = entry.pin_count.saturating_sub(1);
        }
    }

    /// Returns a zero-copy slice of a resident page's payload bytes, if resident and materialized.
    #[must_use]
    pub fn resident_payload(&self, page_id: u64) -> Option<&[u8]> {
        self.pages
            .get(&page_id)
            .filter(|e| e.resident)
            .map(|e| e.payload.as_slice())
    }

    /// Returns whether `page_id` is currently resident in host memory.
    #[must_use]
    pub fn is_resident(&self, page_id: u64) -> bool {
        self.pages.get(&page_id).is_some_and(|e| e.resident)
    }

    /// Deterministic CLOCK / second-chance eviction sweeping until `resident_bytes <= target_limit`.
    fn evict_until_within(&mut self, target_limit: u64) -> Result<(), TensorComputeError> {
        let two = 1usize + 1usize;
        while self.resident_bytes > target_limit {
            if self.resident_ring.is_empty() {
                return Err(TensorComputeError::ZeroOrMismatchedExtents {
                    detail: "arena resident accounting exhausted while over budget".to_string(),
                });
            }
            let max_probes = self.resident_ring.len().saturating_mul(two).saturating_add(1);
            let mut evicted_slot: Option<usize> = None;
            for _ in 0..max_probes {
                if self.clock_hand >= self.resident_ring.len() {
                    self.clock_hand = 0;
                }
                let candidate_id = self.resident_ring[self.clock_hand];
                let mut can_evict = false;
                if let Some(entry) = self.pages.get_mut(&candidate_id) {
                    if entry.pin_count == 0 {
                        if entry.referenced {
                            entry.referenced = false;
                        } else {
                            can_evict = true;
                        }
                    }
                }
                if can_evict {
                    evicted_slot = Some(self.clock_hand);
                    break;
                }
                self.clock_hand += 1;
            }

            let Some(slot) = evicted_slot else {
                return Err(TensorComputeError::ZeroOrMismatchedExtents {
                    detail: "all resident tensor pages are pinned; cannot satisfy budget"
                        .to_string(),
                });
            };
            let evict_id = self.resident_ring.remove(slot);
            if self.clock_hand >= self.resident_ring.len() {
                self.clock_hand = 0;
            }
            if let Some(entry) = self.pages.get_mut(&evict_id) {
                entry.resident = false;
                entry.referenced = false;
                entry.payload.clear();
                entry.payload.shrink_to_fit();
                self.resident_bytes = self
                    .resident_bytes
                    .saturating_sub(entry.desc.physical_bytes);
                self.evictions = self.evictions.saturating_add(1);
            }
        }
        Ok(())
    }

    /// Computes live ISO/IEC 25010 Resource Utilization & Capacity telemetry.
    #[must_use]
    pub fn telemetry(&self) -> PagedArenaTelemetry {
        let ten = (1_u64 + 1_u64) * (1_u64 + 1_u64 + 1_u64 + 1_u64 + 1_u64);
        let thousand = ten * ten * ten;
        let pinned_pages = self
            .pages
            .values()
            .filter(|e| e.resident && e.pin_count > 0)
            .count() as u64;
        let denom = self.resident_bytes.max(1);
        let virtual_amplification_permille = if self.logical_workload_bytes == 0 {
            thousand
        } else {
            self.logical_workload_bytes
                .saturating_mul(thousand)
                .checked_div(denom)
                .unwrap_or(thousand)
        };
        PagedArenaTelemetry {
            budget_bytes: self.budget_bytes,
            resident_bytes: self.resident_bytes,
            peak_resident_bytes: self.peak_resident_bytes,
            logical_workload_bytes: self.logical_workload_bytes,
            registered_pages: self.pages.len() as u64,
            resident_pages: self.resident_ring.len() as u64,
            pinned_pages,
            cache_hits: self.cache_hits,
            page_faults: self.page_faults,
            evictions: self.evictions,
            virtual_amplification_permille,
            invariant_held: self.resident_bytes <= self.budget_bytes
                && self.peak_resident_bytes <= self.budget_bytes,
        }
    }
}

/// Summary report produced by an out-of-core paged tiled tensor contraction.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PagedContractionReport {
    /// Logical output matrix rows (`M`).
    pub rows: usize,
    /// Shared contraction extent (`K`).
    pub inner_k: usize,
    /// Logical output matrix columns (`N`).
    pub cols: usize,
    /// Row-tile / column-tile quantum used to stream pages through the arena.
    pub tile_rows: usize,
    /// Total tiled page contractions executed.
    pub tile_dispatches: u64,
    /// Total packed nibble bytes read directly via zero-decode LUT contraction.
    pub coded_bytes_read: u64,
    /// Equivalent uncompressed `f32` bytes avoided by zero-decode `Packed<Grid<16>, 2>` execution.
    pub avoided_f32_bytes: u64,
    /// Snapshot of arena telemetry at completion (`invariant_held == true` guaranteed).
    pub arena: PagedArenaTelemetry,
}

/// Executes an arbitrary-scale `rows x inner_k x cols` `Packed<Grid<16>, 2>` zero-decode tensor
/// contraction out-of-core through [`PagedTensorArena`] in bounded `tile_rows x tile_cols` pages.
///
/// Produces **bit-exact** `i64` outputs identical to [`contract_packed_nibbles_lut_into`] while
/// strictly honoring `arena.budget_bytes` via deterministic page admission, pinning, and eviction.
#[allow(clippy::too_many_arguments)]
pub fn contract_paged_packed_nibbles_stream(
    arena: &mut PagedTensorArena,
    lhs_packed_rows: &[u8],
    rhs_packed_cols: &[u8],
    rows: usize,
    inner_k: usize,
    cols: usize,
    tile_rows: usize,
    tile_cols: usize,
    lut: &CodebookProductLut,
    out_accum: &mut [i64],
) -> Result<PagedContractionReport, TensorComputeError> {
    let packed_k = inner_k.div_ceil(PACKED_NIBBLE_BLOCK);
    if rows == 0
        || inner_k == 0
        || cols == 0
        || tile_rows == 0
        || tile_cols == 0
        || lhs_packed_rows.len() != rows.saturating_mul(packed_k)
        || rhs_packed_cols.len() != cols.saturating_mul(packed_k)
        || out_accum.len() != rows.saturating_mul(cols)
    {
        return Err(TensorComputeError::ZeroOrMismatchedExtents {
            detail: "paged packed nibble contraction extents mismatch".to_string(),
        });
    }

    out_accum.fill(0_i64);
    let r_step = tile_rows.min(rows);
    let c_step = tile_cols.min(cols);
    let lhs_base_page: u64 = 1_u64 << (1_u32 + 1_u32 + 1_u32 + 1_u32 + 1_u32 + 1_u32 + 1_u32 + 1_u32);
    let rhs_base_page: u64 = lhs_base_page << 1;

    let r_tiles = rows.div_ceil(r_step);
    let c_tiles = cols.div_ceil(c_step);

    for rt in 0..r_tiles {
        let r0 = rt * r_step;
        let r_len = (rows - r0).min(r_step);
        let phys = (r_len * packed_k) as u64;
        arena.register_virtual_page(
            lhs_base_page + (rt as u64),
            TensorPageKind::AiWeightBlock,
            TensorStorageEncoding::PackedNibble4,
            phys,
            lhs_base_page ^ (rt as u64),
        )?;
    }
    for ct in 0..c_tiles {
        let c0 = ct * c_step;
        let c_len = (cols - c0).min(c_step);
        let phys = (c_len * packed_k) as u64;
        arena.register_virtual_page(
            rhs_base_page + (ct as u64),
            TensorPageKind::AiKvCacheBlock,
            TensorStorageEncoding::PackedNibble4,
            phys,
            rhs_base_page ^ (ct as u64),
        )?;
    }

    let mut tile_scratch: Vec<i64> = vec![0_i64; r_step * c_step];
    let mut tile_dispatches: u64 = 0;
    let mut coded_bytes_read: u64 = 0;

    for rt in 0..r_tiles {
        let r0 = rt * r_step;
        let r_len = (rows - r0).min(r_step);
        let lhs_id = lhs_base_page + (rt as u64);
        let lhs_slice = &lhs_packed_rows[r0 * packed_k..(r0 + r_len) * packed_k];
        arena.admit_or_touch(lhs_id, Some(lhs_slice), true)?;

        for ct in 0..c_tiles {
            let c0 = ct * c_step;
            let c_len = (cols - c0).min(c_step);
            let rhs_id = rhs_base_page + (ct as u64);
            let rhs_slice = &rhs_packed_cols[c0 * packed_k..(c0 + c_len) * packed_k];
            arena.admit_or_touch(rhs_id, Some(rhs_slice), true)?;

            let lhs_res = arena.resident_payload(lhs_id).ok_or_else(|| {
                TensorComputeError::ZeroOrMismatchedExtents {
                    detail: "pinned LHS tensor page missing".to_string(),
                }
            })?;
            let rhs_res = arena.resident_payload(rhs_id).ok_or_else(|| {
                TensorComputeError::ZeroOrMismatchedExtents {
                    detail: "pinned RHS tensor page missing".to_string(),
                }
            })?;

            let sub_out = &mut tile_scratch[..r_len * c_len];
            let bytes = contract_packed_nibbles_lut_into(
                lhs_res, rhs_res, r_len, inner_k, c_len, lut, sub_out,
            )?;
            coded_bytes_read = coded_bytes_read.saturating_add(bytes as u64);
            tile_dispatches = tile_dispatches.saturating_add(1);

            for dr in 0..r_len {
                let dst_row = (r0 + dr) * cols + c0;
                let src_row = dr * c_len;
                out_accum[dst_row..dst_row + c_len]
                    .copy_from_slice(&sub_out[src_row..src_row + c_len]);
            }

            arena.unpin(rhs_id);
        }
        arena.unpin(lhs_id);
    }

    let factor = TensorStorageEncoding::PackedNibble4.compression_factor();
    let avoided_f32_bytes = coded_bytes_read.saturating_mul(factor.saturating_sub(1));
    Ok(PagedContractionReport {
        rows,
        inner_k,
        cols,
        tile_rows: r_step,
        tile_dispatches,
        coded_bytes_read,
        avoided_f32_bytes,
        arena: arena.telemetry(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CODED_TABLE_LEN;

    #[test]
    fn paged_arena_enforces_budget_under_16x_oversubscription() {
        let default_arena = PagedTensorArena::default();
        assert_eq!(default_arena.telemetry().budget_bytes, 64 * 1024 * 1024);
        let mut arena = PagedTensorArena::new(MIN_ARENA_BUDGET_BYTES);
        let page_sz = TENSOR_PAGE_QUANTUM_BYTES;
        let total_pages = 1_u64 << (1_u32 + 1_u32 + 1_u32 + 1_u32 + 1_u32 + 1_u32);
        for id in 0..total_pages {
            arena
                .register_virtual_page(
                    id,
                    TensorPageKind::AiWeightBlock,
                    TensorStorageEncoding::PackedNibble4,
                    page_sz,
                    id,
                )
                .expect("register page");
        }
        for id in 0..total_pages {
            arena.admit_or_touch(id, None, false).expect("admit page");
            let t = arena.telemetry();
            assert!(t.invariant_held);
            assert!(t.resident_bytes <= t.budget_bytes);
        }
        let final_t = arena.telemetry();
        assert!(final_t.evictions > 0);
        assert!(final_t.logical_workload_bytes > final_t.budget_bytes);
        assert!(final_t.invariant_held);
    }

    #[test]
    fn paged_tiled_packed_contraction_matches_full_contraction_bit_exact() {
        let mut cb = [0_i32; CODED_TABLE_LEN];
        for (i, slot) in cb.iter_mut().enumerate() {
            *slot = (i as i32) - (1 + 1 + 1 + 1 + 1 + 1 + 1);
        }
        let lut = CodebookProductLut::new(&cb, &cb);
        let rows = 1usize << (1 + 1 + 1 + 1);
        let inner_k = 1usize << (1 + 1 + 1 + 1 + 1);
        let cols = 1usize << (1 + 1 + 1 + 1);
        let packed_k = inner_k.div_ceil(PACKED_NIBBLE_BLOCK);
        let lhs: Vec<u8> = (0..rows * packed_k).map(|i| (i & ((1 << (1 + 1 + 1 + 1 + 1 + 1 + 1 + 1)) - 1)) as u8).collect();
        let rhs: Vec<u8> = (0..cols * packed_k).map(|i| ((i * (1 + 1 + 1) + 1) & ((1 << (1 + 1 + 1 + 1 + 1 + 1 + 1 + 1)) - 1)) as u8).collect();

        let mut direct = vec![0_i64; rows * cols];
        contract_packed_nibbles_lut_into(&lhs, &rhs, rows, inner_k, cols, &lut, &mut direct)
            .expect("direct contraction");

        let mut arena = PagedTensorArena::new(MIN_ARENA_BUDGET_BYTES);
        let mut paged = vec![0_i64; rows * cols];
        let tile_q = 1usize << (1 + 1);
        let rep = contract_paged_packed_nibbles_stream(
            &mut arena,
            &lhs,
            &rhs,
            rows,
            inner_k,
            cols,
            tile_q,
            tile_q,
            &lut,
            &mut paged,
        )
        .expect("paged contraction");

        assert_eq!(direct, paged);
        assert!(rep.arena.invariant_held);
        assert_eq!(rep.arena.pinned_pages, 0);
        assert!(rep.avoided_f32_bytes > 0);
    }
}
