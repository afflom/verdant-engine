//! Stable transport of certified binary32 row-order keys.

use crate::TensorComputeError;

const DIGITS: usize = std::mem::size_of::<u32>();
const BUCKETS: usize = u8::MAX as usize + 1;

/// Reusable stable ranking of equally sized binary32 rows.
pub struct RowRankingF32 {
    width: usize,
    descending: bool,
    count: usize,
    passes: usize,
    keys: Vec<[u8; DIGITS]>,
    order: Vec<u32>,
    scratch: Vec<u32>,
}

impl RowRankingF32 {
    fn invalid() -> TensorComputeError {
        TensorComputeError::ZeroOrMismatchedExtents {
            detail: "binary32 rank request exceeds row or index storage".into(),
        }
    }

    /// Prepare all temporary storage once for a run of rows with this shape.
    pub fn new(width: usize, count: usize, descending: bool) -> Result<Self, TensorComputeError> {
        use verdant_row_ranking::{run, Adt0 as Request};
        let passes = run(Request::C0(width as u64, count as u64))
            .map_err(|_| Self::invalid())?
            .ok_or_else(Self::invalid)?;
        let mut keys = Vec::new();
        let mut order = Vec::new();
        let mut scratch = Vec::new();
        let slots = if count == 0 { 0 } else { width };
        keys.try_reserve_exact(slots).map_err(|_| Self::invalid())?;
        order
            .try_reserve_exact(slots)
            .map_err(|_| Self::invalid())?;
        scratch
            .try_reserve_exact(slots)
            .map_err(|_| Self::invalid())?;
        keys.resize(slots, [0; DIGITS]);
        order.resize(slots, 0);
        scratch.resize(slots, 0);
        Ok(Self {
            width,
            descending,
            count,
            passes: passes as usize,
            keys,
            order,
            scratch,
        })
    }

    /// Return source indices in value order. Ties preserve lane order and NaNs sort last.
    pub fn select(&mut self, row: &[f32]) -> Result<&[u32], TensorComputeError> {
        use verdant_row_ranking::{run, Adt0 as Request};
        if row.len() != self.width {
            return Err(Self::invalid());
        }
        if self.count == 0 {
            return Ok(&[]);
        }
        let mut counts = [[0_usize; BUCKETS]; DIGITS];
        for (index, value) in row.iter().enumerate() {
            let key = run(Request::C1(self.descending, value.to_bits().into()))
                .map_err(|_| Self::invalid())?
                .ok_or_else(Self::invalid)?;
            let bytes = (key as u32).to_le_bytes();
            for (histogram, byte) in counts.iter_mut().zip(bytes) {
                histogram[byte as usize] += 1;
            }
            self.keys[index] = bytes;
            self.order[index] = index as u32;
        }
        // Stable byte passes carry original lane order through equal certified keys.
        for (digit, mut offsets) in counts.into_iter().take(self.passes).enumerate() {
            if offsets[self.keys[0][digit] as usize] == self.width {
                continue;
            }
            let mut start = 0;
            for offset in &mut offsets {
                let length = *offset;
                *offset = start;
                start += length;
            }
            for &index in &self.order {
                let next = &mut offsets[self.keys[index as usize][digit] as usize];
                self.scratch[*next] = index;
                *next += 1;
            }
            std::mem::swap(&mut self.order, &mut self.scratch);
        }
        Ok(&self.order[..self.count])
    }
}
