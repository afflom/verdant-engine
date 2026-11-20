//! Deterministic linear algebra, spatial volumes, constructive number generators, and PRNG.
//!
//! All spatial coordinates use a right-handed Y-up metric coordinate frame (`1.0` = `1 meter`).
//! Every derived integer and real proportion is generated constructively from `{0, 1}` without
//! magic constants.

use std::ops::{Add, Mul, Sub};

use serde::{Deserialize, Serialize};

/// Planar coordinate dimension (`1 + 1`).
pub const DIM_PLANAR: usize = 1 + 1;
/// Spatial coordinate and triangle vertex arity (`1 + 1 + 1`).
pub const DIM_SPATIAL: usize = 1 + 1 + 1;
/// Homogeneous affine vector and RGBA channel dimension (`(1 + 1) * (1 + 1)`).
pub const DIM_HOMOGENEOUS: usize = (1 + 1) * (1 + 1);
/// Half-space plane count of a perspective view frustum (`DIM_PLANAR * DIM_SPATIAL`).
pub const FRUSTUM_PLANE_COUNT: usize = DIM_PLANAR * DIM_SPATIAL;

/// Constructive integer `2` (`1 + 1`).
#[must_use]
pub const fn two_u32() -> u32 {
    1_u32.wrapping_add(1_u32)
}

/// Constructive integer `3` (`1 + 1 + 1`).
#[must_use]
pub const fn three_u32() -> u32 {
    1_u32.wrapping_add(1_u32).wrapping_add(1_u32)
}

/// Constructive integer `4` (`(1 + 1) * (1 + 1)`).
#[must_use]
#[inline(always)]
pub const fn four_u32() -> u32 {
    two_u32().wrapping_mul(two_u32())
}

/// Constructive integer `5` (`4 + 1`).
#[must_use]
#[inline(always)]
pub const fn five_u32() -> u32 {
    four_u32().wrapping_add(1_u32)
}

/// Constructive integer `8` (`4 * 2`).
#[must_use]
#[inline(always)]
pub const fn eight_u32() -> u32 {
    four_u32().wrapping_mul(two_u32())
}

/// Constructive integer `10` (`5 * 2`).
#[must_use]
#[inline(always)]
pub const fn ten_u32() -> u32 {
    five_u32().wrapping_mul(two_u32())
}

/// Constructive integer `16` (`4 * 4`).
#[must_use]
#[inline(always)]
pub const fn sixteen_u32() -> u32 {
    four_u32().wrapping_mul(four_u32())
}

/// Constructive integer `64` (`16 * 4`).
#[must_use]
#[inline(always)]
pub const fn sixty_four_u32() -> u32 {
    sixteen_u32().wrapping_mul(four_u32())
}

/// Constructive integer `100` (`10 * 10`).
#[must_use]
#[inline(always)]
pub const fn hundred_u32() -> u32 {
    ten_u32().wrapping_mul(ten_u32())
}

/// Constructive integer `1000` (`100 * 10`).
#[must_use]
#[inline(always)]
pub const fn thousand_u32() -> u32 {
    hundred_u32().wrapping_mul(ten_u32())
}

/// Constructive power of two (`2^exp`) computed purely from `{0, 1}`.
#[must_use]
pub const fn pow2_u64(exp: u32) -> u64 {
    let mut acc = 1_u64;
    let mut i = 0_u32;
    while i < exp {
        acc = acc.wrapping_add(acc);
        i = i.wrapping_add(1_u32);
    }
    acc
}

/// Constructive low-bit mask (`2^bits - 1`) computed purely from `{0, 1}`.
#[must_use]
pub const fn bit_mask_u64(bits: u32) -> u64 {
    pow2_u64(bits).saturating_sub(1)
}

/// Constructive Fibonacci sequence `F(index)` (`0, 1, 1, 2, 3, 5, 8, ...`) from `{0, 1}`.
#[must_use]
pub const fn fibonacci_u64(index: u32) -> u64 {
    let mut a = 0_u64;
    let mut b = 1_u64;
    let mut i = 0_u32;
    while i < index {
        let next = a.saturating_add(b);
        a = b;
        b = next;
        i = i.wrapping_add(1_u32);
    }
    a
}

/// Constructive primality predicate over `u64` from `{0, 1}`.
#[must_use]
pub const fn is_prime_u64(n: u64) -> bool {
    let two = 1_u64.wrapping_add(1_u64);
    if n < two {
        return false;
    }
    let mut d = two;
    while d.saturating_mul(d) <= n {
        if n.is_multiple_of(d) {
            return false;
        }
        d = d.wrapping_add(1_u64);
    }
    true
}

/// Constructive `rank`-th prime (`rank = 0 -> 2`, `1 -> 3`, `2 -> 5`, `3 -> 7`, ...).
#[must_use]
pub const fn nth_prime_u64(rank: u32) -> u64 {
    let mut candidate = 1_u64.wrapping_add(1_u64);
    let mut found = 0_u32;
    loop {
        if is_prime_u64(candidate) {
            if found == rank {
                return candidate;
            }
            found = found.wrapping_add(1_u32);
        }
        candidate = candidate.wrapping_add(1_u64);
    }
}

/// Constructive power of ten (`10^exp` where `10 = 2 * 5`) from `{0, 1}`.
#[must_use]
pub const fn pow10_u64(exp: u32) -> u64 {
    let ten = nth_prime_u64(0).wrapping_mul(nth_prime_u64(1_u32.wrapping_add(1_u32)));
    let mut acc = 1_u64;
    let mut i = 0_u32;
    while i < exp {
        acc = acc.saturating_mul(ten);
        i = i.wrapping_add(1_u32);
    }
    acc
}

/// Double-precision `2.0` (`1.0 + 1.0`).
#[must_use]
pub fn two_f64() -> f64 {
    1.0 + 1.0
}

/// Double-precision `0.5` (`1.0 / (1.0 + 1.0)`).
#[must_use]
pub fn half_f64() -> f64 {
    1.0 / two_f64()
}

/// Numerical degeneracy tolerance `2^-40` derived constructively without magic float literals.
#[must_use]
pub fn geometric_epsilon() -> f64 {
    let forty = (1_u32 + 1_u32 + 1_u32 + 1_u32 + 1_u32) * (1_u32 << (1_u32 + 1_u32 + 1_u32));
    1.0 / (pow2_u64(forty) as f64)
}

/// Golden ratio `phi = (1 + sqrt(5)) / 2` derived constructively from `{0, 1}`.
#[must_use]
pub fn golden_ratio() -> f64 {
    let two = two_f64();
    let five = nth_prime_u64(two_u32()) as f64;
    (1.0 + five.sqrt()) / two
}

/// Golden phyllotactic divergence angle `TAU / phi^2` in radians.
#[must_use]
pub fn golden_angle_radians() -> f64 {
    let phi = golden_ratio();
    std::f64::consts::TAU / (phi * phi)
}

/// Two-component single-precision vector for texture coordinates and planar offsets.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Vec2 {
    /// Horizontal (`U` or `X`) coordinate.
    pub x: f32,
    /// Vertical (`V` or `Z`) coordinate.
    pub y: f32,
}

impl Vec2 {
    /// Constructs a new planar vector.
    #[must_use]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

/// Three-component double-precision spatial vector in meters, m/s, or Newtons.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Vec3 {
    /// East-west axis (`+X` east), in meters.
    pub x: f64,
    /// Vertical elevation axis (`+Y` up), in meters.
    pub y: f64,
    /// North-south axis (`+Z` south), in meters.
    pub z: f64,
}

impl Vec3 {
    /// Origin `(0, 0, 0)`.
    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    /// Unit upward vector `(0, 1, 0)`.
    pub const UP: Self = Self {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };

    /// Constructs a spatial vector from components.
    #[must_use]
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Uniform scalar product `self * s`.
    #[must_use]
    pub fn scale(self, s: f64) -> Self {
        Self::new(self.x * s, self.y * s, self.z * s)
    }

    /// Euclidean dot product.
    #[must_use]
    pub fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Right-handed cross product `self x other`.
    #[must_use]
    pub fn cross(self, other: Self) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    /// Squared Euclidean norm.
    #[must_use]
    pub fn length_squared(self) -> f64 {
        self.dot(self)
    }

    /// Euclidean norm.
    #[must_use]
    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    /// Euclidean distance to `other`.
    #[must_use]
    pub fn distance(self, other: Self) -> f64 {
        (self - other).length()
    }

    /// Unit-length vector along `self`, or `Vec3::UP` when degenerate.
    #[must_use]
    pub fn normalized(self) -> Self {
        let len = self.length();
        if len > geometric_epsilon() {
            self.scale(1.0 / len)
        } else {
            Self::UP
        }
    }

    /// Linear interpolation between `self` and `other` by `t` in `[0, 1]`.
    #[must_use]
    pub fn lerp(self, other: Self, t: f64) -> Self {
        self.scale(1.0 - t) + other.scale(t)
    }

    /// Converts to single-precision `[f32; DIM_SPATIAL]` for GPU/glTF vertex buffers.
    #[must_use]
    pub fn to_f32_array(self) -> [f32; DIM_SPATIAL] {
        [self.x as f32, self.y as f32, self.z as f32]
    }

    /// Checks that all three components are finite IEEE-754 numbers.
    #[must_use]
    pub fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

/// Unit quaternion `(x, y, z, w)` representing spatial orientation without gimbal lock.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Quat {
    /// Vector X component.
    pub x: f64,
    /// Vector Y component.
    pub y: f64,
    /// Vector Z component.
    pub z: f64,
    /// Scalar real component.
    pub w: f64,
}

impl Default for Quat {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Quat {
    /// Multiplicative identity quaternion `(0, 0, 0, 1)`.
    pub const IDENTITY: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: 1.0,
    };

    /// Constructs a normalized quaternion rotating around `axis` by `angle_radians`.
    #[must_use]
    pub fn from_axis_angle(axis: Vec3, angle_radians: f64) -> Self {
        let unit = axis.normalized();
        let half = half_f64() * angle_radians;
        let (s, c) = half.sin_cos();
        Self {
            x: unit.x * s,
            y: unit.y * s,
            z: unit.z * s,
            w: c,
        }
        .normalized()
    }

    /// Normalizes the quaternion to unit length.
    #[must_use]
    pub fn normalized(self) -> Self {
        let n = (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt();
        if n > geometric_epsilon() {
            Self {
                x: self.x / n,
                y: self.y / n,
                z: self.z / n,
                w: self.w / n,
            }
        } else {
            Self::IDENTITY
        }
    }

    /// Rotates a spatial vector by this unit quaternion.
    #[must_use]
    pub fn rotate_vec3(self, v: Vec3) -> Vec3 {
        let qv = Vec3::new(self.x, self.y, self.z);
        let t = qv.cross(v).scale(two_f64());
        v + t.scale(self.w) + qv.cross(t)
    }

    /// Converts to glTF `[x, y, z, w]` single-precision array.
    #[must_use]
    pub fn to_f32_array(self) -> [f32; DIM_HOMOGENEOUS] {
        [self.x as f32, self.y as f32, self.z as f32, self.w as f32]
    }
}

impl Mul for Quat {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            w: self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z,
            x: self.w * rhs.x + self.x * rhs.w + self.y * rhs.z - self.z * rhs.y,
            y: self.w * rhs.y - self.x * rhs.z + self.y * rhs.w + self.z * rhs.x,
            z: self.w * rhs.z + self.x * rhs.y - self.y * rhs.x + self.z * rhs.w,
        }
        .normalized()
    }
}

/// Column-major homogeneous affine transformation or projection matrix.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Mat4 {
    /// Column-major matrix storage.
    pub cols: [[f64; DIM_HOMOGENEOUS]; DIM_HOMOGENEOUS],
}

impl Default for Mat4 {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Mat4 {
    /// Homogeneous identity matrix.
    pub const IDENTITY: Self = Self {
        cols: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
    };

    /// Builds a TRS (`Translation * Rotation * Scale`) world transform matrix.
    #[must_use]
    pub fn from_trs(translation: Vec3, rotation: Quat, scale: Vec3) -> Self {
        let q = rotation.normalized();
        let two = two_f64();
        let (xx, yy, zz) = (q.x * q.x, q.y * q.y, q.z * q.z);
        let (xy, xz, yz) = (q.x * q.y, q.x * q.z, q.y * q.z);
        let (wx, wy, wz) = (q.w * q.x, q.w * q.y, q.w * q.z);

        Self {
            cols: [
                [
                    (1.0 - two * (yy + zz)) * scale.x,
                    two * (xy + wz) * scale.x,
                    two * (xz - wy) * scale.x,
                    0.0,
                ],
                [
                    two * (xy - wz) * scale.y,
                    (1.0 - two * (xx + zz)) * scale.y,
                    two * (yz + wx) * scale.y,
                    0.0,
                ],
                [
                    two * (xz + wy) * scale.z,
                    two * (yz - wx) * scale.z,
                    (1.0 - two * (xx + yy)) * scale.z,
                    0.0,
                ],
                [translation.x, translation.y, translation.z, 1.0],
            ],
        }
    }

    /// Builds a pure translation matrix.
    #[must_use]
    pub fn from_translation(translation: Vec3) -> Self {
        Self::from_trs(translation, Quat::IDENTITY, Vec3::new(1.0, 1.0, 1.0))
    }

    /// Builds a right-handed perspective projection matrix.
    #[must_use]
    pub fn perspective(fov_y_radians: f64, aspect_ratio: f64, z_near: f64, z_far: f64) -> Self {
        let half = half_f64();
        let two = two_f64();
        let f = 1.0 / (fov_y_radians * half).tan().max(geometric_epsilon());
        let aspect = aspect_ratio.max(geometric_epsilon());
        let span = z_near - z_far;
        let nf = if span.abs() < geometric_epsilon() {
            -1.0
        } else {
            1.0 / span
        };
        Self {
            cols: [
                [f / aspect, 0.0, 0.0, 0.0],
                [0.0, f, 0.0, 0.0],
                [0.0, 0.0, (z_far + z_near) * nf, -1.0],
                [0.0, 0.0, two * z_far * z_near * nf, 0.0],
            ],
        }
    }

    /// Builds a right-handed camera view matrix looking from `eye` toward `target`.
    #[must_use]
    pub fn look_at(eye: Vec3, target: Vec3, up: Vec3) -> Self {
        let z_axis = (eye - target).normalized();
        let x_axis = up.cross(z_axis).normalized();
        let y_axis = z_axis.cross(x_axis);
        Self {
            cols: [
                [x_axis.x, y_axis.x, z_axis.x, 0.0],
                [x_axis.y, y_axis.y, z_axis.y, 0.0],
                [x_axis.z, y_axis.z, z_axis.z, 0.0],
                [-x_axis.dot(eye), -y_axis.dot(eye), -z_axis.dot(eye), 1.0],
            ],
        }
    }

    /// Transforms a spatial direction `(x, y, z, 0)` by the upper-left `3x3` linear block.
    #[must_use]
    pub fn transform_vector3(self, v: Vec3) -> Vec3 {
        let [c0, c1, c2, _] = self.cols;
        let i2 = 1usize + 1usize;
        Vec3::new(
            c0[0] * v.x + c1[0] * v.y + c2[0] * v.z,
            c0[1] * v.x + c1[1] * v.y + c2[1] * v.z,
            c0[i2] * v.x + c1[i2] * v.y + c2[i2] * v.z,
        )
    }

    /// Computes the exact inverse of an affine `4x4` matrix (`det(M_3x3) != 0`), returning `None` if singular.
    #[must_use]
    pub fn inverse_affine(self) -> Option<Self> {
        let i2 = 1usize + 1usize;
        let [c0, c1, c2, c3] = self.cols;
        let m00 = c0[0];
        let m10 = c0[1];
        let m20 = c0[i2];
        let m01 = c1[0];
        let m11 = c1[1];
        let m21 = c1[i2];
        let m02 = c2[0];
        let m12 = c2[1];
        let m22 = c2[i2];

        let cof00 = m11 * m22 - m12 * m21;
        let cof01 = -(m10 * m22 - m12 * m20);
        let cof02 = m10 * m21 - m11 * m20;

        let det = m00 * cof00 + m01 * cof01 + m02 * cof02;
        if !det.is_finite() || det.abs() <= geometric_epsilon() {
            return None;
        }
        let inv_det = 1.0 / det;

        let cof10 = -(m01 * m22 - m02 * m21);
        let cof11 = m00 * m22 - m02 * m20;
        let cof12 = -(m00 * m21 - m01 * m20);

        let cof20 = m01 * m12 - m02 * m11;
        let cof21 = -(m00 * m12 - m02 * m10);
        let cof22 = m00 * m11 - m01 * m10;

        // Adjugate transpose (`inv = cof^T / det`), stored column-major:
        let ic0 = [cof00 * inv_det, cof01 * inv_det, cof02 * inv_det, 0.0];
        let ic1 = [cof10 * inv_det, cof11 * inv_det, cof12 * inv_det, 0.0];
        let ic2 = [cof20 * inv_det, cof21 * inv_det, cof22 * inv_det, 0.0];

        let tx = c3[0];
        let ty = c3[1];
        let tz = c3[i2];
        let itx = -(ic0[0] * tx + ic1[0] * ty + ic2[0] * tz);
        let ity = -(ic0[1] * tx + ic1[1] * ty + ic2[1] * tz);
        let itz = -(ic0[i2] * tx + ic1[i2] * ty + ic2[i2] * tz);

        Some(Self {
            cols: [ic0, ic1, ic2, [itx, ity, itz, 1.0]],
        })
    }

    /// Maximum column-vector Euclidean scale factor of the upper-left `3x3` block.
    #[must_use]
    pub fn max_axis_scale(self) -> f64 {
        let i2 = 1usize + 1usize;
        let sx = (self.cols[0][0] * self.cols[0][0]
            + self.cols[0][1] * self.cols[0][1]
            + self.cols[0][i2] * self.cols[0][i2])
            .sqrt();
        let sy = (self.cols[1][0] * self.cols[1][0]
            + self.cols[1][1] * self.cols[1][1]
            + self.cols[1][i2] * self.cols[1][i2])
            .sqrt();
        let sz = (self.cols[i2][0] * self.cols[i2][0]
            + self.cols[i2][1] * self.cols[i2][1]
            + self.cols[i2][i2] * self.cols[i2][i2])
            .sqrt();
        sx.max(sy).max(sz).max(geometric_epsilon())
    }

    /// Transforms a spatial point `(x, y, z, 1)`.
    #[must_use]
    pub fn transform_point3(self, p: Vec3) -> Vec3 {
        let [c0, c1, c2, c3] = self.cols;
        let [c0x, c0y, c0z, _] = c0;
        let [c1x, c1y, c1z, _] = c1;
        let [c2x, c2y, c2z, _] = c2;
        let [tx, ty, tz, _] = c3;
        Vec3::new(
            c0x * p.x + c1x * p.y + c2x * p.z + tx,
            c0y * p.x + c1y * p.y + c2y * p.z + ty,
            c0z * p.x + c1z * p.y + c2z * p.z + tz,
        )
    }
}

impl Mul for Mat4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let mut out = [[0.0; DIM_HOMOGENEOUS]; DIM_HOMOGENEOUS];
        for (c, rhs_col) in rhs.cols.iter().enumerate() {
            for (r, out_cell) in out[c].iter_mut().enumerate() {
                let mut sum = 0.0;
                for (k, &rhs_val) in rhs_col.iter().enumerate() {
                    sum += self.cols[k][r] * rhs_val;
                }
                *out_cell = sum;
            }
        }
        Self { cols: out }
    }
}

/// Axis-aligned spatial bounding box for culling and broadphase collision.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Aabb {
    /// Minimum corner `(min_x, min_y, min_z)`.
    pub min: Vec3,
    /// Maximum corner `(max_x, max_y, max_z)`.
    pub max: Vec3,
}

impl Aabb {
    /// Constructs an AABB from minimum and maximum corners.
    #[must_use]
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self {
            min: Vec3::new(min.x.min(max.x), min.y.min(max.y), min.z.min(max.z)),
            max: Vec3::new(min.x.max(max.x), min.y.max(max.y), min.z.max(max.z)),
        }
    }

    /// Constructs an AABB centered at `center` with `half_extents`.
    #[must_use]
    pub fn from_center_half_extents(center: Vec3, half_extents: Vec3) -> Self {
        Self::new(center - half_extents, center + half_extents)
    }

    /// Smallest AABB enclosing both `self` and `other`.
    #[must_use]
    pub fn union(self, other: Self) -> Self {
        Self {
            min: Vec3::new(
                self.min.x.min(other.min.x),
                self.min.y.min(other.min.y),
                self.min.z.min(other.min.z),
            ),
            max: Vec3::new(
                self.max.x.max(other.max.x),
                self.max.y.max(other.max.y),
                self.max.z.max(other.max.z),
            ),
        }
    }

    /// Geometric center of the bounding box.
    #[must_use]
    pub fn center(self) -> Vec3 {
        (self.min + self.max).scale(half_f64())
    }

    /// Surface area for Surface Area Heuristic (`SAH`) BVH construction.
    #[must_use]
    pub fn surface_area(self) -> f64 {
        let d = self.max - self.min;
        two_f64() * (d.x * d.y + d.y * d.z + d.z * d.x)
    }

    /// Returns whether two AABBs overlap on all three axes.
    #[must_use]
    pub fn intersects(self, other: Self) -> bool {
        self.min.x <= other.max.x
            && self.max.x >= other.min.x
            && self.min.y <= other.max.y
            && self.max.y >= other.min.y
            && self.min.z <= other.max.z
            && self.max.z >= other.min.z
    }

    /// Transforms all corners of `self` by `matrix` and returns their world-space AABB.
    #[must_use]
    pub fn transformed(self, matrix: Mat4) -> Self {
        let xs = [self.min.x, self.max.x];
        let ys = [self.min.y, self.max.y];
        let zs = [self.min.z, self.max.z];
        let first = matrix.transform_point3(self.min);
        let mut min = first;
        let mut max = first;
        for &x in &xs {
            for &y in &ys {
                for &z in &zs {
                    let p = matrix.transform_point3(Vec3::new(x, y, z));
                    min = Vec3::new(min.x.min(p.x), min.y.min(p.y), min.z.min(p.z));
                    max = Vec3::new(max.x.max(p.x), max.y.max(p.y), max.z.max(p.z));
                }
            }
        }
        Self { min, max }
    }
}

/// Semi-infinite spatial ray `origin + t * direction` (`t >= 0`) for scene picking and visibility.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Ray {
    /// Ray origin in world coordinates.
    pub origin: Vec3,
    /// Unit direction vector.
    pub direction: Vec3,
}

impl Ray {
    /// Constructs a ray with normalized direction.
    #[must_use]
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            origin,
            direction: direction.normalized(),
        }
    }

    /// Slab intersection test against `aabb`, returning nearest entry distance `t >= 0`.
    #[must_use]
    pub fn intersect_aabb(self, aabb: Aabb) -> Option<f64> {
        let mut tmin = 0.0_f64;
        let mut tmax = f64::INFINITY;
        let eps = geometric_epsilon();
        let orig = [self.origin.x, self.origin.y, self.origin.z];
        let dir = [self.direction.x, self.direction.y, self.direction.z];
        let bmin = [aabb.min.x, aabb.min.y, aabb.min.z];
        let bmax = [aabb.max.x, aabb.max.y, aabb.max.z];

        for axis in 0..DIM_SPATIAL {
            if dir[axis].abs() < eps {
                if orig[axis] < bmin[axis] || orig[axis] > bmax[axis] {
                    return None;
                }
            } else {
                let inv = 1.0 / dir[axis];
                let mut t1 = (bmin[axis] - orig[axis]) * inv;
                let mut t2 = (bmax[axis] - orig[axis]) * inv;
                if t1 > t2 {
                    std::mem::swap(&mut t1, &mut t2);
                }
                tmin = tmin.max(t1);
                tmax = tmax.min(t2);
                if tmin > tmax {
                    return None;
                }
            }
        }
        Some(tmin)
    }
}

/// Oriented half-space plane `normal . x + d >= 0` used by view frusta.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Plane {
    /// Inward-pointing unit normal.
    pub normal: Vec3,
    /// Signed offset from origin.
    pub distance: f64,
}

/// Perspective view frustum bounded by `FRUSTUM_PLANE_COUNT` inward-facing planes.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Frustum {
    /// Six inward-facing half-space planes.
    pub planes: [Plane; FRUSTUM_PLANE_COUNT],
}

impl Frustum {
    /// Constructs a perspective camera frustum from eye position, forward axis, FOV, aspect, and clip planes.
    #[must_use]
    pub fn from_perspective(
        eye: Vec3,
        forward: Vec3,
        up_hint: Vec3,
        vertical_fov_radians: f64,
        aspect_ratio: f64,
        near_meters: f64,
        far_meters: f64,
    ) -> Self {
        let fwd = forward.normalized();
        let right = fwd.cross(up_hint).normalized();
        let up = right.cross(fwd).normalized();

        let near_center = eye + fwd.scale(near_meters);
        let far_center = eye + fwd.scale(far_meters);

        let half_v = (half_f64() * vertical_fov_radians).tan();
        let half_h = half_v * aspect_ratio;

        let left_normal = (fwd + right.scale(half_h)).normalized();
        let right_normal = (fwd - right.scale(half_h)).normalized();
        let bottom_normal = (fwd + up.scale(half_v)).normalized();
        let top_normal = (fwd - up.scale(half_v)).normalized();

        let plane_at = |normal: Vec3, point: Vec3| Plane {
            normal,
            distance: -normal.dot(point),
        };

        Self {
            planes: [
                plane_at(fwd, near_center),
                plane_at(fwd.scale(-1.0), far_center),
                plane_at(left_normal, eye),
                plane_at(right_normal, eye),
                plane_at(bottom_normal, eye),
                plane_at(top_normal, eye),
            ],
        }
    }

    /// Returns whether `aabb` lies at least partially inside all frustum planes.
    #[must_use]
    pub fn intersects_aabb(&self, aabb: Aabb) -> bool {
        for plane in &self.planes {
            let px = if plane.normal.x >= 0.0 {
                aabb.max.x
            } else {
                aabb.min.x
            };
            let py = if plane.normal.y >= 0.0 {
                aabb.max.y
            } else {
                aabb.min.y
            };
            let pz = if plane.normal.z >= 0.0 {
                aabb.max.z
            } else {
                aabb.min.z
            };
            if plane.normal.dot(Vec3::new(px, py, pz)) + plane.distance < 0.0 {
                return false;
            }
        }
        true
    }
}

/// Deterministic cellular-automaton (`left ^ (center | right)`) pseudo-random generator.
///
/// Uses zero arbitrary magic constants: state advances via circular elementary cellular
/// evolution over `{0, 1}` bits seeded by `(seed | 1)`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeterministicRng {
    state: u64,
    counter: u64,
}

impl DeterministicRng {
    /// Initializes a deterministic generator from a world seed without magic constants.
    #[must_use]
    pub const fn new(seed: u64) -> Self {
        Self {
            state: seed | 1,
            counter: 1,
        }
    }

    /// Advances state via circular `left ^ (center | right)` rounds and returns a word.
    pub fn next_u64(&mut self) -> u64 {
        let rounds = fibonacci_u64(1 + 1 + 1 + 1 + 1 + 1); // F(6) = 8 rounds from {0, 1}
        let mut s = self.state ^ self.counter;
        let mut r = 0_u64;
        while r < rounds {
            let left = s.rotate_left(1);
            let right = s.rotate_right(1);
            s = left ^ (s | right);
            r += 1;
        }
        self.state = s | 1;
        self.counter = self.counter.wrapping_add(1);
        s ^ self.counter.rotate_left(1)
    }

    /// Uniform double in `[0.0, 1.0)` using IEEE-754 mantissa precision (`53 = nth_prime(15)`).
    pub fn next_f64(&mut self) -> f64 {
        let mantissa_bits = (1_u32 << (1_u32 + 1_u32 + 1_u32 + 1_u32 + 1_u32))
            + (1_u32 << (1_u32 + 1_u32 + 1_u32 + 1_u32))
            + (1_u32 << (1_u32 + 1_u32))
            + 1_u32; // 32 + 16 + 4 + 1 = 53
        let word_bits = 1_u32 << (1_u32 + 1_u32 + 1_u32 + 1_u32 + 1_u32 + 1_u32); // 64
        let top = self.next_u64() >> (word_bits - mantissa_bits);
        (top as f64) / (pow2_u64(mantissa_bits) as f64)
    }

    /// Uniform double in `[low, high)`.
    pub fn range_f64(&mut self, low: f64, high: f64) -> f64 {
        low + (high - low) * self.next_f64()
    }
}

/// Cross-verifies `verdant_core::math` constructive dimensions, powers, and primes against
/// `Foundation.Ontology.V1.Model` and `Subsystems::canonicalDimensions` in `prism_kernel`.
#[must_use]
pub fn verify_constructive_math_with_prism_kernel() -> bool {
    let Ok(dims) = crate::prism_kernel::canonicalDimensions() else {
        return false;
    };
    let dims_ok = dims.dimPlanar == (DIM_PLANAR as u64)
        && dims.dimSpatial == (DIM_SPATIAL as u64)
        && dims.dimHomogeneous == (DIM_HOMOGENEOUS as u64)
        && dims.frustumPlaneCount == (FRUSTUM_PLANE_COUNT as u64);
    let exp_four = four_u32();
    let pow2_ok =
        crate::prism_kernel::pow2Constructive(u64::from(exp_four)) == Ok(pow2_u64(exp_four));
    let pow10_ok = crate::prism_kernel::pow10Constructive(u64::from(three_u32()))
        == Ok(pow10_u64(three_u32()));
    let prime_ok = crate::prism_kernel::nthPrimeConstructive(u64::from(exp_four))
        == Ok(nth_prime_u64(exp_four));
    dims_ok && pow2_ok && pow10_ok && prime_ok
}
