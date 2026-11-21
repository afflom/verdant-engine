//! Continuous swept-capsule collide-and-slide character controller (`physics_controller.rs`).

use super::*;

/// Player or autonomous character movement intent submitted to [`PhysicsWorld::step_lawful`].
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct CharacterMoveIntent {
    /// Desired world-space planar movement direction (`x, 0, z`).
    pub planar_wish_dir: Vec3,
    /// Planar speed multiplier (`1.0` standard walk, `> 1.0` sprint).
    pub speed_scale: f64,
    /// Whether a jump impulse is requested when grounded.
    pub jump_requested: bool,
}

impl Default for CharacterMoveIntent {
    fn default() -> Self {
        Self {
            planar_wish_dir: Vec3::ZERO,
            speed_scale: 1.0,
            jump_requested: false,
        }
    }
}

/// Telemetry and formal invariant certificate emitted by [`PhysicsWorld::step_lawful`].
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct LawfulStepReport {
    /// Number of continuous swept-capsule slide iterations executed (`0..=4`).
    pub slide_iterations: u32,
    /// True when an acute two-plane corner triggered `normalize(n1 x n2)` crease projection.
    pub crease_projected: bool,
    /// Post-step minimum world clearance from solid triangle surfaces in meters.
    pub min_clearance_meters: f64,
    /// Proof witness from Lean-verified `prism_kernel::sweptClearanceValid`.
    pub clearance_verified: bool,
}

fn sweep_capsule_stack(
    bvh: &TwoLevelBvh,
    foot: Vec3,
    radius: f64,
    eye_height: f64,
    delta: Vec3,
) -> Option<(SweptSphereHit, bool)> {
    let lower_center = Vec3::new(foot.x, foot.y + radius, foot.z);
    let upper_y = (eye_height - radius * half_f64()).max(radius);
    let upper_center = Vec3::new(foot.x, foot.y + upper_y, foot.z);

    let (_, hit) = bvh
        .sweep_capsule_counted(lower_center, upper_center - lower_center, radius, delta)
        .0?;
    let lower_at_contact = lower_center + delta.scale(hit.toi);
    let is_lower = hit.contact_point.y <= lower_at_contact.y + geometric_epsilon();
    Some((hit, is_lower))
}

fn depenetrate_capsule_stack(
    bvh: &TwoLevelBvh,
    foot: &mut Vec3,
    radius: f64,
    eye_height: f64,
    skin_m: f64,
) {
    let two = two_f64();
    let target_clear = radius + skin_m;
    let upper_y = (eye_height - radius * half_f64()).max(radius);
    let passes = 1usize + 1usize + 1usize;
    for _ in 0..passes {
        let mut moved = false;
        for offset_y in [radius, upper_y] {
            let center = Vec3::new(foot.x, foot.y + offset_y, foot.z);
            if let Some((_, _, outward_n, dist)) =
                bvh.min_surface_clearance(center, target_clear + skin_m * two)
            {
                if dist < target_clear {
                    let push = target_clear - dist;
                    *foot = *foot + outward_n.scale(push);
                    moved = true;
                }
            }
        }
        if !moved {
            break;
        }
    }
}

impl CapsuleController {
    /// Advances the capsule by `dt_seconds` against `two_level_bvh` using continuous swept-sphere
    /// collide-and-slide, walkable stair auto-stepping, acute 2-plane crease projection, and
    /// post-step `prism_kernel::sweptClearanceValid` verification.
    pub fn step_collide_and_slide<F>(
        &mut self,
        dt_seconds: f64,
        intent: CharacterMoveIntent,
        gravity_mps2: f64,
        two_level_bvh: &TwoLevelBvh,
        support_height_at: F,
    ) -> Result<LawfulStepReport, PhysicsStepError>
    where
        F: Fn(f64, f64) -> f64,
    {
        if !dt_seconds.is_finite() || dt_seconds <= 0.0 {
            return Err(PhysicsStepError::InvalidTimeStepSeconds {
                dt_seconds,
                max_dt_seconds: 1.0,
            });
        }

        let eps = geometric_epsilon();
        let half = half_f64();
        let two = two_f64();
        let kc = verdant_tensor::kernel_consts();
        let scale_milli = kc.skin_weight_scale_f64;
        let max_slides = kc.ccd_max_slide_iterations;
        let skin_milli_u64 = kc.ccd_skin_margin_milli;
        let skin_m = kc.ccd_skin_margin_meters;
        let max_step_m = kc.swept_capsule_max_step_meters;
        let walkable_ny = kc.nav_walkable_ny_min;

        let start_foot = self.foot_position;
        depenetrate_capsule_stack(
            two_level_bvh,
            &mut self.foot_position,
            self.radius_meters,
            self.eye_height_meters,
            skin_m,
        );

        let clean_wish = sanitize_vec3(intent.planar_wish_dir);
        let wish_planar = Vec3::new(clean_wish.x, 0.0, clean_wish.z);
        let wish = if wish_planar.length_squared() > eps {
            wish_planar.normalized()
        } else {
            Vec3::ZERO
        };
        let safe_scale = if intent.speed_scale.is_finite() {
            intent.speed_scale.max(0.0).min(two * two)
        } else {
            0.0
        };
        let speed = (self.move_speed_mps * safe_scale).min(kc.physics_max_speed_mps);
        self.foot_position = sanitize_vec3(self.foot_position);
        self.velocity = clamp_velocity_speed(self.velocity, kc.physics_max_speed_mps);
        let mut rem = wish.scale(speed * dt_seconds);

        let mut hit_planes: Vec<Vec3> = Vec::new();
        let mut slide_iterations = 0_u32;
        let mut crease_projected = false;

        for _ in 0..max_slides {
            let rem_len = rem.length();
            if rem_len <= eps {
                break;
            }
            slide_iterations += 1;

            let Some((hit, is_lower)) = sweep_capsule_stack(
                two_level_bvh,
                self.foot_position,
                self.radius_meters,
                self.eye_height_meters,
                rem,
            ) else {
                self.foot_position = self.foot_position + rem;
                break;
            };

            let safe_advance = (hit.toi * rem_len - skin_m).max(0.0);
            self.foot_position = self.foot_position + rem.scale(safe_advance / rem_len);
            rem = rem.scale((1.0 - hit.toi).max(0.0));

            // Attempt smooth walkable stair/riser auto-step when grounded and blocked at the feet:
            if is_lower && self.grounded && hit.contact_normal.y < walkable_ny && rem.length() > eps
            {
                let up_vec = Vec3::new(0.0, max_step_m, 0.0);
                let up_clear = sweep_capsule_stack(
                    two_level_bvh,
                    self.foot_position,
                    self.radius_meters,
                    self.eye_height_meters,
                    up_vec,
                )
                .is_none();
                if up_clear {
                    let raised = self.foot_position + up_vec;
                    let fwd_step = sweep_capsule_stack(
                        two_level_bvh,
                        raised,
                        self.radius_meters,
                        self.eye_height_meters,
                        rem,
                    );
                    let fwd_frac = fwd_step.map_or(1.0, |(fh, _)| fh.toi);
                    let fwd_travel = (fwd_frac * rem.length() - skin_m).max(0.0);
                    if fwd_travel > skin_m * two {
                        let advanced = raised + rem.scale(fwd_travel / rem.length());
                        let down_vec = Vec3::new(0.0, -(max_step_m + skin_m), 0.0);
                        if let Some((land_hit, _)) = sweep_capsule_stack(
                            two_level_bvh,
                            advanced,
                            self.radius_meters,
                            self.eye_height_meters,
                            down_vec,
                        ) {
                            if land_hit.contact_normal.y >= walkable_ny {
                                let drop = (land_hit.toi * down_vec.y.abs() - skin_m).max(0.0);
                                self.foot_position =
                                    Vec3::new(advanced.x, advanced.y - drop, advanced.z);
                                break;
                            }
                        }
                    }
                }
            }

            let mut n = hit.contact_normal;
            if n.y.abs() < walkable_ny {
                n = Vec3::new(n.x, 0.0, n.z).normalized();
            }
            let duplicate = hit_planes.iter().any(|p| p.dot(n) > 1.0 - skin_m);
            if !duplicate {
                hit_planes.push(n);
            }

            if hit_planes.len() == 1 {
                let vn = rem.dot(n);
                if vn < 0.0 {
                    rem = rem - n.scale(vn);
                }
            } else if hit_planes.len() == (1usize + 1usize) {
                let n1 = hit_planes[0];
                let n2 = hit_planes[1];
                let crease = n1.cross(n2);
                if crease.length() > eps {
                    let d = crease.normalized();
                    rem = d.scale(rem.dot(d));
                    crease_projected = true;
                } else {
                    break;
                }
            } else {
                // Three-plane wedge: halt cleanly with zero corner oscillation.
                break;
            }
        }

        // Jump impulse & vertical sweep:
        if intent.jump_requested && self.grounded {
            let jump_height = 1.0 - half * half;
            self.velocity.y = (two * gravity_mps2 * jump_height).sqrt();
            self.grounded = false;
        }
        self.velocity.y -= gravity_mps2 * dt_seconds;
        let v_delta = Vec3::new(0.0, self.velocity.y * dt_seconds, 0.0);

        if v_delta.y.abs() > eps {
            if let Some((v_hit, _)) = sweep_capsule_stack(
                two_level_bvh,
                self.foot_position,
                self.radius_meters,
                self.eye_height_meters,
                v_delta,
            ) {
                let v_len = v_delta.y.abs();
                let safe_v = (v_hit.toi * v_len - skin_m).max(0.0);
                self.foot_position.y += safe_v * v_delta.y.signum();
                if self.velocity.y <= 0.0 && v_hit.contact_normal.y >= walkable_ny {
                    self.velocity.y = 0.0;
                    self.grounded = true;
                } else if self.velocity.y > 0.0 && v_hit.contact_normal.y < 0.0 {
                    self.velocity.y = 0.0;
                }
            } else {
                self.foot_position.y += v_delta.y;
                if self.velocity.y < -half {
                    self.grounded = false;
                }
            }
        }

        let floor_y = support_height_at(self.foot_position.x, self.foot_position.z);
        if self.foot_position.y <= floor_y {
            self.foot_position.y = floor_y;
            if self.velocity.y < 0.0 {
                self.velocity.y = 0.0;
            }
            self.grounded = true;
        }

        depenetrate_capsule_stack(
            two_level_bvh,
            &mut self.foot_position,
            self.radius_meters,
            self.eye_height_meters,
            skin_m,
        );

        self.velocity.x = (self.foot_position.x - start_foot.x) / dt_seconds;
        self.velocity.z = (self.foot_position.z - start_foot.z) / dt_seconds;

        // Evaluate post-step clearance invariant via Lean-proved `prism_kernel::sweptClearanceValid`:
        let search_r = self.radius_meters * two + skin_m * two;
        let lower_c = Vec3::new(
            self.foot_position.x,
            self.foot_position.y + self.radius_meters,
            self.foot_position.z,
        );
        let upper_y = (self.eye_height_meters - self.radius_meters * half).max(self.radius_meters);
        let upper_c = Vec3::new(
            self.foot_position.x,
            self.foot_position.y + upper_y,
            self.foot_position.z,
        );
        let d_lo = two_level_bvh
            .min_surface_clearance(lower_c, search_r)
            .map_or(search_r, |(_, _, _, d)| d);
        let d_hi = two_level_bvh
            .min_surface_clearance(upper_c, search_r)
            .map_or(search_r, |(_, _, _, d)| d);
        let min_clearance_meters = d_lo.min(d_hi);

        let dist_milli = (min_clearance_meters * scale_milli).ceil().max(0.0) as u64;
        let radius_milli = (self.radius_meters * scale_milli).floor().max(0.0) as u64;
        let eps_milli = skin_milli_u64.min(1);
        let clearance_verified =
            crate::prism_kernel::sweptClearanceValid(dist_milli, radius_milli, eps_milli)
                .unwrap_or(false);

        Ok(LawfulStepReport {
            slide_iterations,
            crease_projected,
            min_clearance_meters,
            clearance_verified,
        })
    }
}
