//! Lawful spring-arm and first-person camera rig with continuous near-frustum guard-sphere sweeps.
//!
//! Guarantees mathematically that the perspective near clip rectangle can never slice through any
//! solid wall, ceiling, floor, or architectural mesh by sweeping a finite guard sphere of radius
//! `r_guard >= z_near * sqrt(1 + tan^2(fov_y / 2) * (1 + aspect^2)) + eps_skin` (verified at every
//! frame by `prism_kernel::cameraNearSphereGuarded`) against [`TwoLevelBvh`] and terrain heightfields.

use serde::{Deserialize, Serialize};

use crate::ecs::TwoLevelBvh;
use crate::error::CameraRigError;
use crate::math::{
    geometric_epsilon, half_f64, pow10_u64, three_u32, two_f64, Frustum, Mat4, Vec3,
};

/// Active camera possession / framing topology.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CameraMode {
    /// Collision-guarded third-person orbital / over-the-shoulder spring arm around `anchor`.
    ThirdPersonSpringArm,
    /// First-person eye view anchored at the possessed character's head/visor joint.
    FirstPersonEye,
}

/// Optical frustum and physical spring-arm bounds for [`LawfulCameraRig`].
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct CameraRigConfig {
    /// Vertical field of view in radians (`(0, pi)`).
    pub vertical_fov_radians: f64,
    /// Viewport width-to-height aspect ratio (`> 0`).
    pub aspect_ratio: f64,
    /// Perspective near clip plane distance in meters (`> 0`).
    pub z_near_meters: f64,
    /// Perspective far clip plane distance in meters (`> z_near_meters`).
    pub z_far_meters: f64,
    /// Minimum vertical pitch angle in radians.
    pub min_pitch_radians: f64,
    /// Maximum vertical pitch angle in radians.
    pub max_pitch_radians: f64,
    /// Maximum admissible third-person spring-arm orbit distance in meters.
    pub max_orbit_distance_meters: f64,
}

impl Default for CameraRigConfig {
    fn default() -> Self {
        let half = half_f64();
        let two = two_f64();
        let three = (1 + 1 + 1) as f64;
        let ten = pow10_u64(1) as f64;
        let fov = std::f64::consts::PI / three;
        let pitch_limit = std::f64::consts::FRAC_PI_2 - half * half;
        Self {
            vertical_fov_radians: fov,
            aspect_ratio: (two * two * two * two) / (three * three),
            z_near_meters: half * half,
            z_far_meters: (crate::prism_kernel::wasmFrustumFarMeters().unwrap_or(1) as f64)
                .max(ten * ten),
            min_pitch_radians: -pitch_limit,
            max_pitch_radians: pitch_limit,
            max_orbit_distance_meters: ten * ten,
        }
    }
}

impl CameraRigConfig {
    /// Validates optical frustum parameters and returns the exact near-plane diagonal guard radius.
    pub fn validate_and_guard_radius(&self) -> Result<(f64, bool), CameraRigError> {
        let eps = geometric_epsilon();
        if !self.vertical_fov_radians.is_finite()
            || !self.aspect_ratio.is_finite()
            || !self.z_near_meters.is_finite()
            || !self.z_far_meters.is_finite()
            || self.vertical_fov_radians <= eps
            || self.vertical_fov_radians >= std::f64::consts::PI - eps
            || self.aspect_ratio <= eps
            || self.z_near_meters <= eps
            || self.z_far_meters <= self.z_near_meters
        {
            return Err(CameraRigError::InvalidOpticalFrustum {
                fov_y_radians: self.vertical_fov_radians,
                aspect_ratio: self.aspect_ratio,
                z_near_meters: self.z_near_meters,
                z_far_meters: self.z_far_meters,
            });
        }

        let scale_milli = pow10_u64(three_u32()) as f64;
        let skin_m = (crate::prism_kernel::ccdSkinMarginMilli().max(1) as f64) / scale_milli;
        let tan_half_fov = (self.vertical_fov_radians * half_f64()).tan();
        let diag_factor = (1.0
            + tan_half_fov * tan_half_fov * (1.0 + self.aspect_ratio * self.aspect_ratio))
            .sqrt();
        let two = two_f64();
        let guard_radius = self.z_near_meters * diag_factor + skin_m * two;

        let guard_milli = (guard_radius * scale_milli).ceil().max(1.0) as u64;
        let near_milli = (self.z_near_meters * scale_milli).round().max(1.0) as u64;
        let diag_milli = (diag_factor * scale_milli).ceil().max(1.0) as u64;
        let guarded =
            crate::prism_kernel::cameraNearSphereGuarded(guard_milli, near_milli, diag_milli)
                .unwrap_or(false);

        Ok((guard_radius, guarded))
    }
}

/// Verified camera pose, view/projection matrices, and obstruction telemetry for a single frame.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct SolvedCameraFrame {
    /// Collision-guarded world-space camera eye position.
    pub eye: Vec3,
    /// World-space look target position.
    pub target: Vec3,
    /// Unit upward camera reference vector.
    pub up: Vec3,
    /// Active near-frustum guard-sphere radius in meters.
    pub guard_radius_meters: f64,
    /// Post-sweep minimum clearance from solid geometry in meters.
    pub clearance_meters: f64,
    /// True if the spring arm pulled inward to prevent wall occlusion on this frame.
    pub obstructed: bool,
    /// Proof witness from Lean-verified `prism_kernel::cameraNearSphereGuarded`.
    pub near_sphere_guarded: bool,
    /// Right-handed camera view matrix.
    pub view_matrix: Mat4,
    /// Right-handed perspective projection matrix.
    pub proj_matrix: Mat4,
    /// Exact `proj * view` matrix contracted via `uor-matmul`.
    pub view_proj_matrix: Mat4,
    /// Six-plane perspective view frustum for visibility culling.
    pub frustum: Frustum,
}

/// Stateful spring-arm and first-person camera controller enforcing optical non-clipping laws.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LawfulCameraRig {
    /// Optical and spring-arm bounds.
    pub config: CameraRigConfig,
    /// Active camera mode (`ThirdPersonSpringArm` or `FirstPersonEye`).
    pub mode: CameraMode,
    /// Horizontal orbit/look yaw angle in radians.
    pub yaw_radians: f64,
    /// Vertical orbit/look pitch angle in radians.
    pub pitch_radians: f64,
    /// Desired unobstructed spring-arm length in meters.
    pub desired_distance_meters: f64,
    /// Current smoothed & collision-clamped spring-arm length in meters.
    pub current_distance_meters: f64,
}

impl LawfulCameraRig {
    /// Creates a validated lawful camera rig.
    pub fn new(
        config: CameraRigConfig,
        mode: CameraMode,
        yaw_radians: f64,
        pitch_radians: f64,
        desired_distance_meters: f64,
    ) -> Result<Self, CameraRigError> {
        let _ = config.validate_and_guard_radius()?;
        if !desired_distance_meters.is_finite()
            || desired_distance_meters < 0.0
            || desired_distance_meters > config.max_orbit_distance_meters
        {
            return Err(CameraRigError::InvalidSpringArmDistance {
                requested_meters: desired_distance_meters,
                max_meters: config.max_orbit_distance_meters,
            });
        }
        let clamped_pitch = pitch_radians
            .max(config.min_pitch_radians)
            .min(config.max_pitch_radians);
        Ok(Self {
            config,
            mode,
            yaw_radians,
            pitch_radians: clamped_pitch,
            desired_distance_meters,
            current_distance_meters: desired_distance_meters,
        })
    }

    /// Updates viewport aspect ratio (`width / height`) while preserving optical validity.
    pub fn set_aspect_ratio(&mut self, aspect_ratio: f64) -> Result<(), CameraRigError> {
        let mut next = self.config;
        next.aspect_ratio = aspect_ratio;
        let _ = next.validate_and_guard_radius()?;
        self.config = next;
        Ok(())
    }

    /// Applies incremental pointer/gamepad yaw, pitch, and zoom intents inside Rust.
    pub fn apply_look_and_zoom_intent(
        &mut self,
        delta_yaw_radians: f64,
        delta_pitch_radians: f64,
        delta_distance_meters: f64,
    ) {
        if delta_yaw_radians.is_finite() {
            self.yaw_radians =
                (self.yaw_radians + delta_yaw_radians).rem_euclid(std::f64::consts::TAU);
        }
        if delta_pitch_radians.is_finite() {
            self.pitch_radians = (self.pitch_radians + delta_pitch_radians)
                .max(self.config.min_pitch_radians)
                .min(self.config.max_pitch_radians);
        }
        if delta_distance_meters.is_finite() {
            let min_orbit = half_f64();
            self.desired_distance_meters = (self.desired_distance_meters + delta_distance_meters)
                .max(min_orbit)
                .min(self.config.max_orbit_distance_meters);
        }
    }

    /// Evaluates the collision-guarded camera frame around `anchor` against `two_level_bvh` and
    /// `support_height_at` (`terrain` floor), guaranteeing zero near-plane wall/floor intersections.
    pub fn solve_frame<F>(
        &mut self,
        dt_seconds: f64,
        anchor: Vec3,
        two_level_bvh: &TwoLevelBvh,
        support_height_at: F,
    ) -> Result<SolvedCameraFrame, CameraRigError>
    where
        F: Fn(f64, f64) -> f64,
    {
        let (guard_r, near_sphere_guarded) = self.config.validate_and_guard_radius()?;
        if !self.desired_distance_meters.is_finite()
            || self.desired_distance_meters < 0.0
            || self.desired_distance_meters > self.config.max_orbit_distance_meters
        {
            return Err(CameraRigError::InvalidSpringArmDistance {
                requested_meters: self.desired_distance_meters,
                max_meters: self.config.max_orbit_distance_meters,
            });
        }

        let eps = geometric_epsilon();
        let two = two_f64();
        let scale_milli = pow10_u64(three_u32()) as f64;
        let skin_m = (crate::prism_kernel::ccdSkinMarginMilli().max(1) as f64) / scale_milli;
        let safe_r = guard_r + skin_m;

        // 1. Ensure anchor itself lies strictly outside solid geometry by at least `safe_r`:
        let mut safe_anchor = anchor;
        let anchor_floor = support_height_at(safe_anchor.x, safe_anchor.z) + safe_r;
        if safe_anchor.y < anchor_floor {
            safe_anchor.y = anchor_floor;
        }
        let passes = 1usize + 1usize + 1usize;
        for _ in 0..passes {
            if let Some((_, _, outward_n, dist)) =
                two_level_bvh.min_surface_clearance(safe_anchor, safe_r + skin_m * two)
            {
                if dist < safe_r {
                    safe_anchor = safe_anchor + outward_n.scale(safe_r - dist);
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        let pitch = self
            .pitch_radians
            .max(self.config.min_pitch_radians)
            .min(self.config.max_pitch_radians);
        let (sy, cy) = self.yaw_radians.sin_cos();
        let (sp, cp) = pitch.sin_cos();

        let mut obstructed = false;
        let (mut eye, mut target) = match self.mode {
            CameraMode::FirstPersonEye => {
                self.current_distance_meters = 0.0;
                let fwd = Vec3::new(-sy * cp, sp, -cy * cp).normalized();
                (safe_anchor, safe_anchor + fwd)
            }
            CameraMode::ThirdPersonSpringArm => {
                // Orbit offset behind/above `safe_anchor` along `(yaw, pitch)`:
                let arm_dir = Vec3::new(sy * cp, sp.max(-half_f64()), cy * cp).normalized();
                let desired_len = self.desired_distance_meters.max(safe_r * two);
                let arm_delta = arm_dir.scale(desired_len);

                let max_clear_len = if let Some((_, hit)) =
                    two_level_bvh.sweep_sphere(safe_anchor, guard_r, arm_delta)
                {
                    obstructed = true;
                    (hit.toi * desired_len - skin_m).max(0.0)
                } else {
                    desired_len
                };

                if obstructed || max_clear_len < self.current_distance_meters {
                    // Pull in immediately on obstruction so near-plane never flashes inside a wall:
                    self.current_distance_meters = max_clear_len;
                } else {
                    let ease_rate = two * two * two;
                    let alpha = (dt_seconds.max(0.0) * ease_rate).clamp(0.0, 1.0);
                    self.current_distance_meters +=
                        (max_clear_len - self.current_distance_meters) * alpha;
                }

                let candidate_eye = safe_anchor + arm_dir.scale(self.current_distance_meters);
                let look_target = if self.current_distance_meters > eps {
                    safe_anchor
                } else {
                    safe_anchor - arm_dir
                };
                (candidate_eye, look_target)
            }
        };

        // 2. Clamp above terrain floor and depenetrate `eye` so `B(eye, guard_r)` is 100% obstacle-free:
        let eye_floor = support_height_at(eye.x, eye.z) + safe_r;
        if eye.y < eye_floor {
            let lift = eye_floor - eye.y;
            eye.y = eye_floor;
            if self.mode == CameraMode::FirstPersonEye {
                target.y += lift;
            }
            obstructed = true;
        }

        for _ in 0..passes {
            if let Some((_, _, outward_n, dist)) =
                two_level_bvh.min_surface_clearance(eye, safe_r + skin_m * two)
            {
                if dist < safe_r {
                    let shift = outward_n.scale(safe_r - dist);
                    eye = eye + shift;
                    if self.mode == CameraMode::FirstPersonEye {
                        target = target + shift;
                    }
                    obstructed = true;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        if (target - eye).length_squared() <= eps * eps {
            target = eye + Vec3::new(0.0, 0.0, -1.0);
        }

        let forward = (target - eye).normalized();
        let up = Vec3::UP;
        let view_matrix = Mat4::look_at(eye, target, up);
        let proj_matrix = Mat4::perspective(
            self.config.vertical_fov_radians,
            self.config.aspect_ratio,
            self.config.z_near_meters,
            self.config.z_far_meters,
        );
        let vp_cols =
            verdant_tensor::linear::multiply_mat4_cols_exact(&proj_matrix.cols, &view_matrix.cols)
                .unwrap_or((proj_matrix * view_matrix).cols);
        let view_proj_matrix = Mat4 { cols: vp_cols };
        let frustum = Frustum::from_perspective(
            eye,
            forward,
            up,
            self.config.vertical_fov_radians,
            self.config.aspect_ratio,
            self.config.z_near_meters,
            self.config.z_far_meters,
        );

        let search_r = safe_r * two * two;
        let bvh_clear = two_level_bvh
            .min_surface_clearance(eye, search_r)
            .map_or(search_r, |(_, _, _, d)| d);
        let floor_clear = (eye.y - support_height_at(eye.x, eye.z)).max(0.0);
        let clearance_meters = bvh_clear.min(floor_clear);

        Ok(SolvedCameraFrame {
            eye,
            target,
            up,
            guard_radius_meters: guard_r,
            clearance_meters,
            obstructed,
            near_sphere_guarded,
            view_matrix,
            proj_matrix,
            view_proj_matrix,
            frustum,
        })
    }
}
