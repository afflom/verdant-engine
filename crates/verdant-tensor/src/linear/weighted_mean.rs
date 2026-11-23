//! Online softmax-weighted means of streams of binary32 rows.

use super::{evaluate_f32_scalar, exponential_f32, TensorComputeError};

fn failure() -> TensorComputeError {
    TensorComputeError::InvalidKernelSpecification {
        kernel_name: "OnlineWeightedMean".to_owned(),
    }
}

/// A running weighted sum and its normalization mass at the current maximum score.
pub struct WeightedMeanF32 {
    maximum: f32,
    mass: f32,
    sums: Vec<f32>,
    scratch: Vec<f32>,
}

impl WeightedMeanF32 {
    /// Start an empty accumulation of rows of the given width.
    pub fn new(width: usize) -> Self {
        Self {
            maximum: f32::NEG_INFINITY,
            mass: 0.0,
            sums: vec![0.0; width],
            scratch: vec![0.0; width],
        }
    }

    /// Include an observation. An absent row contributes only to normalization.
    ///
    /// # Errors
    /// Returns a typed error for a mismatched row or an unrepresentable word.
    /// A failed observation leaves the previous accumulated result intact.
    pub fn push(&mut self, score: f32, row: Option<&[f32]>) -> Result<(), TensorComputeError> {
        if row.is_some_and(|r| r.len() != self.sums.len()) {
            return Err(TensorComputeError::ZeroOrMismatchedExtents {
                detail: "weighted observation width differs from accumulator".to_string(),
            });
        }
        let bits = |v: f32| u64::from(v.to_bits());
        let float = |w: u64| u32::try_from(w).map(f32::from_bits).map_err(|_| failure());
        let verdant_weighted_pivot::Adt0::C0(raised, maximum, difference) =
            verdant_weighted_pivot::run(bits(self.maximum), bits(score)).map_err(|_| failure())?;
        let maximum = float(maximum)?;
        let weight = exponential_f32(float(difference)?)?;
        let combine = |prior, sample| {
            let word = verdant_weighted_blend::run(raised, bits(prior), bits(sample), bits(weight))
                .map_err(|_| failure())?;
            float(word)
        };
        let mass = combine(self.mass, 1.0)?;
        if let Some(row) = row {
            for ((next, &prior), &sample) in self.scratch.iter_mut().zip(&self.sums).zip(row) {
                *next = combine(prior, sample)?;
            }
            std::mem::swap(&mut self.sums, &mut self.scratch);
        } else if raised {
            for (next, &prior) in self.scratch.iter_mut().zip(&self.sums) {
                *next = evaluate_f32_scalar(crate::scalars::two_u64(), prior, weight)?;
            }
            std::mem::swap(&mut self.sums, &mut self.scratch);
        }
        self.maximum = maximum;
        self.mass = mass;
        Ok(())
    }

    /// Normalize the accumulated coordinates. An empty stream yields a zero row.
    ///
    /// # Errors
    /// Returns a typed error if a certified arithmetic operation exceeds its word width.
    pub fn finish(mut self) -> Result<Vec<f32>, TensorComputeError> {
        let inverse = verdant_weighted_reciprocal::run(
            u64::from(self.mass.to_bits()),
            u64::from(1.0_f32.to_bits()),
        )
        .map_err(|_| failure())?;
        let scale = u32::try_from(inverse)
            .map(f32::from_bits)
            .map_err(|_| failure())?;
        for value in &mut self.sums {
            *value = evaluate_f32_scalar(crate::scalars::two_u64(), *value, scale)?;
        }
        Ok(self.sums)
    }
}
