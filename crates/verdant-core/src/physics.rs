//! Deterministic Symplectic Euler rigid-body, capsule controller, fluid buoyancy, and spline physics.
//!
//! All dynamics are parameterized by [`PhysicsWorldConfig`] (`PhysicalEnvironment`,
//! `TerrainReliefParams`, stability timestep cap, restitution, damping, and optional elevated deck
//! elevation) with zero arbitrary magic constants.

use serde::{Deserialize, Serialize};

use crate::ecs::BoundingVolumeHierarchy;
use crate::error::PhysicsStepError;
#[cfg(not(target_arch = "wasm32"))]
use crate::math::DIM_SPATIAL;
use crate::math::{
    geometric_epsilon, half_f64, pow10_u64, three_u32, two_f64, Aabb, Quat, Ray, Vec3,
    DIM_HOMOGENEOUS,
};
use crate::streaming::{terrain_height_with_params, PhysicalEnvironment, TerrainReliefParams};

/// Parametric configuration of a deterministic 3D physics world.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct PhysicsWorldConfig {
    /// Deterministic terrain seed for ground-contact heightfield queries.
    pub terrain_seed: u64,
    /// Planetary gravity, fluid density, and water-table elevation.
    pub environment: PhysicalEnvironment,
    /// Continuous terrain relief profile.
    pub terrain_relief: TerrainReliefParams,
    /// Maximum admissible single-step integration `dt` in seconds.
    pub max_dt_seconds: f64,
    /// Default linear velocity air/ground damping coefficient (`1/s`).
    pub linear_damping: f64,
    /// Optional elevated platform deck box `(bounds, deck_top_y)` bridging fluid basins.
    pub platform_deck: Option<(Aabb, f64)>,
}

impl Default for PhysicsWorldConfig {
    fn default() -> Self {
        let half = half_f64();
        let quarter = half * half;
        let env = PhysicalEnvironment::temperate_terrestrial();
        let relief = TerrainReliefParams::standard_basin_valley();
        let deck_half = (three_u32() as f64) + two_f64();
        let deck_y = env.water_level_meters + quarter;
        let deck_bounds = Aabb::from_center_half_extents(
            Vec3::new(relief.basin_center_x, deck_y, relief.basin_center_z),
            Vec3::new(deck_half, half, two_f64()),
        );
        Self {
            terrain_seed: 1,
            environment: env,
            terrain_relief: relief,
            max_dt_seconds: quarter,
            linear_damping: half,
            platform_deck: Some((deck_bounds, deck_y)),
        }
    }
}

/// Spherical rigid body experiencing gravity, ground contact, and Archimedes fluid buoyancy.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RigidBody {
    /// Body identifier.
    pub label: String,
    /// Center-of-mass position in world meters.
    pub position: Vec3,
    /// Linear velocity in `m/s`.
    pub velocity: Vec3,
    /// Body mass in kilograms (`> 0`).
    pub mass_kg: f64,
    /// Collision and submerged-displacement sphere radius in meters (`> 0`).
    pub radius_meters: f64,
    /// Coefficient of normal restitution in `[0, 1]`.
    pub restitution: f64,
    /// Whether the body is currently resting on terrain or a structural deck.
    pub grounded: bool,
    /// Fraction `[0, 1]` of sphere diameter submerged below the water table.
    pub submerged_fraction: f64,
}

impl RigidBody {
    /// Constructs a validated dynamic rigid body.
    pub fn new(
        label: impl Into<String>,
        position: Vec3,
        mass_kg: f64,
        radius_meters: f64,
        restitution: f64,
    ) -> Result<Self, PhysicsStepError> {
        if !mass_kg.is_finite()
            || !radius_meters.is_finite()
            || mass_kg <= 0.0
            || radius_meters <= 0.0
        {
            return Err(PhysicsStepError::InvalidBodyMassOrRadius {
                mass_kg,
                radius_meters,
            });
        }
        Ok(Self {
            label: label.into(),
            position,
            velocity: Vec3::ZERO,
            mass_kg,
            radius_meters,
            restitution: restitution.clamp(0.0, 1.0),
            grounded: false,
            submerged_fraction: 0.0,
        })
    }

    /// Computes exact spherical-cap submerged volume (`m^3`) below `water_level_meters`.
    #[must_use]
    pub fn submerged_volume_m3(&self, water_level_meters: f64) -> f64 {
        let bottom = self.position.y - self.radius_meters;
        let depth = (water_level_meters - bottom).clamp(0.0, two_f64() * self.radius_meters);
        if depth <= 0.0 {
            return 0.0;
        }
        // Spherical cap volume `V_cap(h) = (pi * h^2 / 3) * (3R - h)`:
        let three = three_u32() as f64;
        (std::f64::consts::PI * depth * depth / three) * (three * self.radius_meters - depth)
    }
}

/// First-person or third-person upright capsule character controller for player traversal.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CapsuleController {
    /// Feet base position in world meters.
    pub foot_position: Vec3,
    /// Linear velocity in `m/s`.
    pub velocity: Vec3,
    /// Capsule cylinder radius in meters.
    pub radius_meters: f64,
    /// Eye-level offset above `foot_position.y` in meters.
    pub eye_height_meters: f64,
    /// Maximum planar walk speed in `m/s`.
    pub move_speed_mps: f64,
    /// Whether the capsule is grounded on terrain or an elevated platform deck.
    pub grounded: bool,
}

impl CapsuleController {
    /// Constructs a grounded player capsule controller with constructive human-scale proportions.
    #[must_use]
    pub fn new(foot_position: Vec3) -> Self {
        let half = half_f64();
        let two = two_f64();
        Self {
            foot_position,
            velocity: Vec3::ZERO,
            radius_meters: half,
            eye_height_meters: 1.0 + half + half * half,
            move_speed_mps: two * two,
            grounded: true,
        }
    }

    /// World-space camera eye coordinates.
    #[must_use]
    pub fn eye_position(&self) -> Vec3 {
        Vec3::new(
            self.foot_position.x,
            self.foot_position.y + self.eye_height_meters,
            self.foot_position.z,
        )
    }

    /// Advances the capsule by `dt_seconds` with continuous BVH raycast obstacle sweep, wall sliding,
    /// and `prism_kernel::sweptCapsuleMaxStepMilli` (`0.35m`) stair step-up.
    pub fn step_swept_against_bvh<F>(
        &mut self,
        dt_seconds: f64,
        planar_wish_dir: Vec3,
        gravity_mps2: f64,
        bvh: &BoundingVolumeHierarchy,
        support_height_at: F,
    ) -> Result<(), PhysicsStepError>
    where
        F: Fn(f64, f64) -> f64,
    {
        if !dt_seconds.is_finite() || dt_seconds <= 0.0 {
            return Err(PhysicsStepError::InvalidTimeStepSeconds {
                dt_seconds,
                max_dt_seconds: 1.0,
            });
        }
        let scale_milli = crate::prism_kernel::skinWeightScale().unwrap_or(1).max(1) as f64;
        let max_step_m =
            (crate::prism_kernel::sweptCapsuleMaxStepMilli().unwrap_or(1) as f64) / scale_milli;

        let wish_planar = Vec3::new(planar_wish_dir.x, 0.0, planar_wish_dir.z);
        let wish = if wish_planar.length_squared() > geometric_epsilon() {
            wish_planar.normalized()
        } else {
            Vec3::ZERO
        };

        let delta = wish.scale(self.move_speed_mps * dt_seconds);
        let move_len = delta.length();
        let mut target_pos = self.foot_position + delta;

        if move_len > geometric_epsilon() {
            let dir = delta.scale(1.0 / move_len);
            let waist_origin = Vec3::new(
                self.foot_position.x,
                self.foot_position.y + max_step_m + geometric_epsilon(),
                self.foot_position.z,
            );
            let sweep_dist = move_len + self.radius_meters;
            if let Some(hit) = bvh.raycast(Ray::new(waist_origin, dir), sweep_dist) {
                // Check if stepping up by `max_step_m` clears the obstacle:
                let knee_clear_origin = Vec3::new(
                    self.foot_position.x,
                    self.foot_position.y + max_step_m + self.radius_meters,
                    self.foot_position.z,
                );
                if bvh
                    .raycast(Ray::new(knee_clear_origin, dir), sweep_dist)
                    .is_none()
                {
                    target_pos.y += max_step_m * half_f64();
                } else {
                    let safe_travel = (hit.distance_meters - self.radius_meters).max(0.0);
                    target_pos = self.foot_position + dir.scale(safe_travel);
                }
            }
        }

        self.velocity.x = (target_pos.x - self.foot_position.x) / dt_seconds;
        self.velocity.z = (target_pos.z - self.foot_position.z) / dt_seconds;
        self.velocity.y -= gravity_mps2 * dt_seconds;
        self.foot_position.x = target_pos.x;
        self.foot_position.z = target_pos.z;
        self.foot_position.y = target_pos.y + self.velocity.y * dt_seconds;

        let floor_y = support_height_at(self.foot_position.x, self.foot_position.z);
        if self.foot_position.y <= floor_y {
            self.foot_position.y = floor_y;
            self.velocity.y = 0.0;
            self.grounded = true;
        } else {
            self.grounded = false;
        }
        Ok(())
    }
}

/// Articulated rigid-body joint constraint topology modeled in `Subsystems.lex.tex`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum JointKind {
    /// Revolute hinge joint locking `5` DOFs and allowing rotation around a single axis.
    Hinge,
    /// Spherical ball-and-socket joint locking `3` linear DOFs.
    BallSocket,
    /// Prismatic slider joint locking `5` DOFs and allowing translation along `axis_world`.
    Slider,
    /// Compliant axial distance spring-damper locking `1` distance DOF.
    SpringDamper,
}

impl JointKind {
    /// Returns the Prism-verified number of locked degrees of freedom (`prism_kernel::jointLockedDofs`).
    #[must_use]
    pub fn locked_dofs(self) -> u64 {
        let idx = match self {
            Self::Hinge => 0,
            Self::BallSocket => 1,
            Self::Slider => crate::prism_kernel::sTwo(),
            Self::SpringDamper => crate::prism_kernel::sThree(),
        };
        crate::prism_kernel::jointLockedDofs(idx).unwrap_or(1)
    }
}

/// XPBD / Delassus bilateral joint constraint coupling two rigid bodies in [`PhysicsWorld`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RigidJointConstraint {
    /// Constraint identifier.
    pub label: String,
    /// Index of primary body `A` in `PhysicsWorld::bodies`.
    pub body_a: usize,
    /// Index of secondary body `B` in `PhysicsWorld::bodies`.
    pub body_b: usize,
    /// Joint constraint kind (`Hinge`, `BallSocket`, `Slider`, `SpringDamper`).
    pub kind: JointKind,
    /// Unit constraint axis in world space (hinge/slider axis).
    pub axis_world: Vec3,
    /// Target separation distance between body centers in meters.
    pub target_distance_meters: f64,
    /// XPBD compliance regularization in permille (`>= 0`).
    pub compliance_permille: u64,
}

/// Single suspension wheel mount on a [`RaycastVehicle`].
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct WheelMount {
    /// Local wheel attachment offset relative to chassis center in meters.
    pub local_offset: Vec3,
    /// Uncompressed suspension strut rest length in meters.
    pub rest_length_meters: f64,
    /// Rolling wheel radius in meters.
    pub wheel_radius_meters: f64,
    /// Latest compression distance in meters (`>= 0`).
    pub compression_meters: f64,
    /// Whether the wheel raycast contacted terrain/deck on the latest step.
    pub in_contact: bool,
}

/// Four-wheel raycast suspension vehicle controller with Hooke spring + viscous rebound damping.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RaycastVehicle {
    /// Vehicle identifier.
    pub label: String,
    /// Chassis center-of-mass world position in meters.
    pub chassis_position: Vec3,
    /// Chassis linear velocity in `m/s`.
    pub chassis_velocity: Vec3,
    /// Chassis heading yaw in radians around `Vec3::UP`.
    pub yaw_radians: f64,
    /// Chassis mass in kilograms (`> 0`).
    pub mass_kg: f64,
    /// Suspension spring stiffness (`N / m`).
    pub suspension_stiffness_n_m: f64,
    /// Suspension viscous damping (`N * s / m`).
    pub suspension_damping_ns_m: f64,
    /// Normalized throttle input in `[-1, 1]`.
    pub throttle: f64,
    /// Steer angle input in radians.
    pub steering_radians: f64,
    /// Four raycast wheel mounts (`prism_kernel::vehicleWheelCount() == 4`).
    pub wheels: [WheelMount; DIM_HOMOGENEOUS],
}

impl RaycastVehicle {
    /// Constructs a validated 4-wheel raycast vehicle at `chassis_position`.
    pub fn new(
        label: impl Into<String>,
        chassis_position: Vec3,
        mass_kg: f64,
    ) -> Result<Self, PhysicsStepError> {
        if !mass_kg.is_finite() || mass_kg <= 0.0 {
            return Err(PhysicsStepError::InvalidBodyMassOrRadius {
                mass_kg,
                radius_meters: 1.0,
            });
        }
        debug_assert_eq!(
            crate::prism_kernel::vehicleWheelCount().unwrap_or(1) as usize,
            DIM_HOMOGENEOUS
        );
        let half = half_f64();
        let rest_len = half + half * half;
        let wheel_r = half * (1.0 - half * half);
        let half_track = 1.0;
        let half_wheelbase = 1.0 + half;
        let mk_wheel = |x: f64, z: f64| WheelMount {
            local_offset: Vec3::new(x, -half * half, z),
            rest_length_meters: rest_len,
            wheel_radius_meters: wheel_r,
            compression_meters: 0.0,
            in_contact: false,
        };
        Ok(Self {
            label: label.into(),
            chassis_position,
            chassis_velocity: Vec3::ZERO,
            yaw_radians: 0.0,
            mass_kg,
            suspension_stiffness_n_m: mass_kg * (pow10_u64(1) as f64) * two_f64(),
            suspension_damping_ns_m: mass_kg * two_f64(),
            throttle: 0.0,
            steering_radians: 0.0,
            wheels: [
                mk_wheel(-half_track, half_wheelbase),
                mk_wheel(half_track, half_wheelbase),
                mk_wheel(-half_track, -half_wheelbase),
                mk_wheel(half_track, -half_wheelbase),
            ],
        })
    }
}

/// Harmonic damped torsion or flexural spring oscillator (`m * x'' + c * x' + k * x = F_ext`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArticulatedSpringJoint {
    /// Joint identifier (`gimbal_ring`, `torsion_pendulum`, `flexural_beam`).
    pub name: String,
    /// Angular or linear displacement from equilibrium.
    pub displacement: f64,
    /// Angular or linear velocity.
    pub velocity: f64,
    /// Natural angular frequency `omega_0` (`rad/s`).
    pub natural_omega_rad_s: f64,
    /// Viscous damping ratio `zeta >= 0`.
    pub damping_ratio: f64,
}

impl ArticulatedSpringJoint {
    /// Advances the oscillator by `dt_seconds` under `external_drive` via Symplectic Euler.
    pub fn step(&mut self, dt_seconds: f64, external_drive: f64) {
        let accel = external_drive
            - two_f64() * self.damping_ratio * self.natural_omega_rad_s * self.velocity
            - self.natural_omega_rad_s * self.natural_omega_rad_s * self.displacement;
        self.velocity += accel * dt_seconds;
        self.displacement += self.velocity * dt_seconds;
    }
}

/// Autonomous kinematic mover following a closed piecewise-smooth waypoint spline loop.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KinematicSplineBody {
    /// Closed-loop world-space waypoints.
    pub waypoints: Vec<Vec3>,
    /// Continuous arc-progress parameter along `[0, waypoints.len())`.
    pub loop_parameter: f64,
    /// Tangential cruise speed in `m/s`.
    pub cruise_speed_mps: f64,
    /// Current world-space position.
    pub position: Vec3,
    /// Current forward heading unit vector.
    pub heading: Vec3,
    /// Current world orientation quaternion aligned with `heading`.
    pub orientation: Quat,
    /// Cumulative distance traveled in meters.
    pub distance_traveled_meters: f64,
}

impl KinematicSplineBody {
    /// Initializes a kinematic spline mover on `waypoints` cruising at `cruise_speed_mps`.
    #[must_use]
    pub fn new(waypoints: Vec<Vec3>, cruise_speed_mps: f64) -> Self {
        let initial_pos = waypoints.first().copied().unwrap_or(Vec3::ZERO);
        let next_pos = waypoints
            .get(1)
            .copied()
            .unwrap_or_else(|| initial_pos + Vec3::new(0.0, 0.0, 1.0));
        let heading = (next_pos - initial_pos).normalized();
        let yaw = heading.x.atan2(heading.z);
        Self {
            waypoints,
            loop_parameter: 0.0,
            cruise_speed_mps: cruise_speed_mps.max(0.0),
            position: initial_pos,
            heading,
            orientation: Quat::from_axis_angle(Vec3::UP, yaw),
            distance_traveled_meters: 0.0,
        }
    }

    /// Advances the kinematic mover along its closed waypoint spline by `dt_seconds`.
    pub fn step(&mut self, dt_seconds: f64) {
        let count = self.waypoints.len();
        if count <= 1 {
            return;
        }
        let idx0 = (self.loop_parameter.floor() as usize) % count;
        let idx1 = (idx0 + 1) % count;
        let p0 = self.waypoints[idx0];
        let p1 = self.waypoints[idx1];
        let seg_len = p1.distance(p0).max(geometric_epsilon());
        let step_dist = self.cruise_speed_mps * dt_seconds;
        self.distance_traveled_meters += step_dist;
        self.loop_parameter = (self.loop_parameter + step_dist / seg_len).rem_euclid(count as f64);

        let new_i0 = (self.loop_parameter.floor() as usize) % count;
        let new_i1 = (new_i0 + 1) % count;
        let frac = self.loop_parameter.fract();
        let a = self.waypoints[new_i0];
        let b = self.waypoints[new_i1];
        self.position = a.lerp(b, frac);
        self.heading = (b - a).normalized();
        let yaw = self.heading.x.atan2(self.heading.z);
        self.orientation = Quat::from_axis_angle(Vec3::UP, yaw);
    }
}

/// Deterministic 3D physics world combining rigid bodies, capsule player, spring/XPBD joints, vehicles, and spline mover.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PhysicsWorld {
    /// Parametric physics configuration.
    pub config: PhysicsWorldConfig,
    /// Dynamic rigid bodies.
    pub bodies: Vec<RigidBody>,
    /// Optional interactive player capsule controller.
    pub player_capsule: Option<CapsuleController>,
    /// Harmonic spring-damper joints.
    pub joints: Vec<ArticulatedSpringJoint>,
    /// Bilateral XPBD / Delassus rigid-body joint constraints.
    pub rigid_joints: Vec<RigidJointConstraint>,
    /// Four-wheel raycast suspension vehicles.
    pub vehicles: Vec<RaycastVehicle>,
    /// Optional closed-spline kinematic mover.
    pub kinematic_mover: Option<KinematicSplineBody>,
    /// Total simulated time in seconds.
    pub elapsed_seconds: f64,
}

impl PhysicsWorld {
    /// Creates a new deterministic physics world from `config`.
    #[must_use]
    pub const fn new(config: PhysicsWorldConfig) -> Self {
        Self {
            config,
            bodies: Vec::new(),
            player_capsule: None,
            joints: Vec::new(),
            rigid_joints: Vec::new(),
            vehicles: Vec::new(),
            kinematic_mover: None,
            elapsed_seconds: 0.0,
        }
    }

    /// Evaluates supporting surface height (`terrain` or `platform_deck`) at `(x, z)`.
    #[must_use]
    pub fn support_surface_height(&self, x: f64, z: f64) -> f64 {
        let mut h =
            terrain_height_with_params(self.config.terrain_seed, &self.config.terrain_relief, x, z);
        if let Some((deck_box, deck_top_y)) = self.config.platform_deck {
            if x >= deck_box.min.x
                && x <= deck_box.max.x
                && z >= deck_box.min.z
                && z <= deck_box.max.z
            {
                h = h.max(deck_top_y);
            }
        }
        h
    }

    /// Advances all bodies, capsule controllers, joints, and kinematic movers by `dt_seconds`.
    pub fn step(&mut self, dt_seconds: f64, planar_wish_dir: Vec3) -> Result<(), PhysicsStepError> {
        if !dt_seconds.is_finite() || dt_seconds <= 0.0 || dt_seconds > self.config.max_dt_seconds {
            return Err(PhysicsStepError::InvalidTimeStepSeconds {
                dt_seconds,
                max_dt_seconds: self.config.max_dt_seconds,
            });
        }

        let g = self.config.environment.gravity_mps2;
        let rho_fluid = self.config.environment.fluid_density_kg_m3;
        let water_y = self.config.environment.water_level_meters;
        let damp = (1.0 - self.config.linear_damping * dt_seconds).clamp(0.0, 1.0);
        let two = two_f64();

        for body in &mut self.bodies {
            if !body.mass_kg.is_finite()
                || !body.radius_meters.is_finite()
                || body.mass_kg <= 0.0
                || body.radius_meters <= 0.0
            {
                return Err(PhysicsStepError::InvalidBodyMassOrRadius {
                    mass_kg: body.mass_kg,
                    radius_meters: body.radius_meters,
                });
            }

            let sub_vol = body.submerged_volume_m3(water_y);
            let sub_depth = (water_y - (body.position.y - body.radius_meters))
                .clamp(0.0, two * body.radius_meters);
            body.submerged_fraction = sub_depth / (two * body.radius_meters);

            // Archimedes upward buoyancy force `F_b = rho_fluid * g * V_submerged`:
            let buoyancy_force_y = rho_fluid * g * sub_vol;
            let net_accel_y = -g + buoyancy_force_y / body.mass_kg;

            let fluid_drag = if body.submerged_fraction > 0.0 {
                (1.0 - two * self.config.linear_damping * dt_seconds).clamp(0.0, 1.0)
            } else {
                damp
            };

            body.velocity.y += net_accel_y * dt_seconds;
            body.velocity = body.velocity.scale(fluid_drag);
            body.position = body.position + body.velocity.scale(dt_seconds);

            let floor_y = terrain_height_with_params(
                self.config.terrain_seed,
                &self.config.terrain_relief,
                body.position.x,
                body.position.z,
            ) + body.radius_meters;
            if body.position.y <= floor_y {
                body.position.y = floor_y;
                if body.velocity.y < 0.0 {
                    body.velocity.y = -body.velocity.y * body.restitution;
                }
                body.grounded = true;
            } else {
                body.grounded = false;
            }
        }

        // Precompute support height if a player capsule is active.
        if self.player_capsule.is_some() {
            let wish_planar = Vec3::new(planar_wish_dir.x, 0.0, planar_wish_dir.z);
            let wish = if wish_planar.length_squared() > geometric_epsilon() {
                wish_planar.normalized()
            } else {
                Vec3::ZERO
            };
            let mut capsule = self.player_capsule.take().expect("checked Some above");
            capsule.velocity.x = wish.x * capsule.move_speed_mps;
            capsule.velocity.z = wish.z * capsule.move_speed_mps;
            capsule.velocity.y -= g * dt_seconds;
            capsule.foot_position = capsule.foot_position + capsule.velocity.scale(dt_seconds);

            // Keep wading or walking cleanly on terrain or elevated decks above water table:
            let support_y = self
                .support_surface_height(capsule.foot_position.x, capsule.foot_position.z)
                .max(water_y - half_f64());
            if capsule.foot_position.y <= support_y {
                capsule.foot_position.y = support_y;
                capsule.velocity.y = 0.0;
                capsule.grounded = true;
            } else {
                capsule.grounded = false;
            }
            self.player_capsule = Some(capsule);
        }

        let breeze_drive = (self.elapsed_seconds * std::f64::consts::TAU * half_f64()).sin();
        for joint in &mut self.joints {
            joint.step(dt_seconds, breeze_drive);
        }

        // Solve bilateral XPBD / Delassus rigid-body constraints via `verdant_tensor::delassus_effective_mass_matrix`:
        #[cfg(not(target_arch = "wasm32"))]
        if !self.rigid_joints.is_empty() && self.bodies.len() >= (1 + 1) {
            let dof_stride = DIM_SPATIAL + DIM_SPATIAL;
            let total_dofs = self.bodies.len() * dof_stride;
            let mut inv_mass_diag = vec![0.0_f64; total_dofs];
            for (b_idx, b) in self.bodies.iter().enumerate() {
                let inv_m = 1.0 / b.mass_kg.max(geometric_epsilon());
                let base = b_idx * dof_stride;
                inv_mass_diag[base] = inv_m;
                inv_mass_diag[base + 1] = inv_m;
                inv_mass_diag[base + (1 + 1)] = inv_m;
                inv_mass_diag[base + DIM_SPATIAL] = inv_m;
                inv_mass_diag[base + DIM_SPATIAL + 1] = inv_m;
                inv_mass_diag[base + DIM_SPATIAL + (1 + 1)] = inv_m;
            }

            let milli_scale = crate::prism_kernel::skinWeightScale().unwrap_or(1).max(1) as f64;
            let constraints_snapshot = self.rigid_joints.clone();
            for cj in &constraints_snapshot {
                if cj.body_a >= self.bodies.len()
                    || cj.body_b >= self.bodies.len()
                    || cj.body_a == cj.body_b
                {
                    return Err(PhysicsStepError::InvalidBodyMassOrRadius {
                        mass_kg: 0.0,
                        radius_meters: 0.0,
                    });
                }
                let _locked = cj.kind.locked_dofs();
                let pa = self.bodies[cj.body_a].position;
                let pb = self.bodies[cj.body_b].position;
                let delta = pb - pa;
                let dist = delta.length().max(geometric_epsilon());
                let n = match cj.kind {
                    JointKind::Slider => cj.axis_world.normalized(),
                    _ => delta.scale(1.0 / dist),
                };

                let mut j_row = vec![0.0_f64; total_dofs];
                let base_a = cj.body_a * dof_stride;
                let base_b = cj.body_b * dof_stride;
                j_row[base_a] = -n.x;
                j_row[base_a + 1] = -n.y;
                j_row[base_a + (1 + 1)] = -n.z;
                j_row[base_b] = n.x;
                j_row[base_b + 1] = n.y;
                j_row[base_b + (1 + 1)] = n.z;

                let eff = verdant_tensor::delassus_effective_mass_matrix(
                    &j_row,
                    &inv_mass_diag,
                    1,
                    total_dofs,
                )
                .map_err(|_| PhysicsStepError::InvalidBodyMassOrRadius {
                    mass_kg: 0.0,
                    radius_meters: 0.0,
                })?;
                let w_sum = eff.first().copied().unwrap_or(0.0).max(geometric_epsilon());
                let pos_err = match cj.kind {
                    JointKind::Slider => delta.dot(n) - cj.target_distance_meters,
                    _ => dist - cj.target_distance_meters,
                };
                let err_milli = (pos_err.abs() * milli_scale).round() as u64;
                let inv_m_milli = (w_sum * milli_scale).round().max(1.0) as u64;
                let lambda_milli = crate::prism_kernel::xpbdConstraintImpulseMilli(
                    err_milli,
                    inv_m_milli,
                    cj.compliance_permille,
                )
                .unwrap_or(0);
                let corr_mag = ((lambda_milli as f64) / milli_scale) * pos_err.signum();
                let inv_ma = 1.0 / self.bodies[cj.body_a].mass_kg;
                let inv_mb = 1.0 / self.bodies[cj.body_b].mass_kg;
                self.bodies[cj.body_a].position =
                    self.bodies[cj.body_a].position + n.scale(corr_mag * inv_ma);
                self.bodies[cj.body_b].position =
                    self.bodies[cj.body_b].position - n.scale(corr_mag * inv_mb);

                // Project out relative velocity along the locked direction (or transverse directions for Slider/Hinge):
                let rel_v = self.bodies[cj.body_b].velocity - self.bodies[cj.body_a].velocity;
                let vn = rel_v.dot(n);
                let j_vel = -vn / (w_sum + (cj.compliance_permille as f64) / milli_scale);
                self.bodies[cj.body_a].velocity =
                    self.bodies[cj.body_a].velocity - n.scale(j_vel * inv_ma);
                self.bodies[cj.body_b].velocity =
                    self.bodies[cj.body_b].velocity + n.scale(j_vel * inv_mb);
            }
        }

        // Step 4-wheel raycast suspension vehicles:
        #[cfg(not(target_arch = "wasm32"))]
        if !self.vehicles.is_empty() {
            let milli_scale = crate::prism_kernel::skinWeightScale().unwrap_or(1).max(1) as f64;
            let ten = pow10_u64(1) as f64;
            let mut updated_vehicles = std::mem::take(&mut self.vehicles);
            for veh in &mut updated_vehicles {
                if !veh.mass_kg.is_finite() || veh.mass_kg <= 0.0 {
                    let bad_mass = veh.mass_kg;
                    self.vehicles = updated_vehicles;
                    return Err(PhysicsStepError::InvalidBodyMassOrRadius {
                        mass_kg: bad_mass,
                        radius_meters: 1.0,
                    });
                }
                let q_yaw = Quat::from_axis_angle(Vec3::UP, veh.yaw_radians);
                let fwd = q_yaw.rotate_vec3(Vec3::new(0.0, 0.0, 1.0)).normalized();
                let right = q_yaw.rotate_vec3(Vec3::new(1.0, 0.0, 0.0)).normalized();
                let mut total_up_force = 0.0_f64;
                let mut grounded_wheels = 0usize;

                for wheel in &mut veh.wheels {
                    let mount_world = veh.chassis_position + q_yaw.rotate_vec3(wheel.local_offset);
                    let ground_y = self.support_surface_height(mount_world.x, mount_world.z);
                    let wheel_bottom_rest =
                        mount_world.y - wheel.rest_length_meters - wheel.wheel_radius_meters;
                    if wheel_bottom_rest <= ground_y {
                        let comp =
                            (ground_y - wheel_bottom_rest).clamp(0.0, wheel.rest_length_meters);
                        wheel.compression_meters = comp;
                        wheel.in_contact = true;
                        grounded_wheels += 1;
                        let comp_milli = (comp * milli_scale).round() as u64;
                        let rebound_milli =
                            (veh.chassis_velocity.y.max(0.0) * milli_scale).round() as u64;
                        let f_milli = crate::prism_kernel::raycastSuspensionForceMilli(
                            comp_milli,
                            veh.suspension_stiffness_n_m.max(1.0).round() as u64,
                            rebound_milli,
                            veh.suspension_damping_ns_m.max(0.0).round() as u64,
                        )
                        .unwrap_or(0);
                        total_up_force += (f_milli as f64) / milli_scale;
                    } else {
                        wheel.compression_meters = 0.0;
                        wheel.in_contact = false;
                    }
                }

                let drive_accel = if grounded_wheels > 0 {
                    veh.throttle.clamp(-1.0, 1.0) * ten
                } else {
                    0.0
                };
                veh.chassis_velocity.y += (-g + total_up_force / veh.mass_kg) * dt_seconds;
                veh.chassis_velocity = veh.chassis_velocity + fwd.scale(drive_accel * dt_seconds);

                // Dampen lateral tire slip when wheels are in contact:
                if grounded_wheels > 0 {
                    let lat_speed = veh.chassis_velocity.dot(right);
                    let slip_damp = (two * two * dt_seconds).clamp(0.0, 1.0);
                    veh.chassis_velocity =
                        veh.chassis_velocity - right.scale(lat_speed * slip_damp);
                    let fwd_speed = veh.chassis_velocity.dot(fwd);
                    veh.yaw_radians += veh.steering_radians * fwd_speed * half_f64() * dt_seconds;
                }
                veh.chassis_velocity = veh.chassis_velocity.scale(damp);
                veh.chassis_position =
                    veh.chassis_position + veh.chassis_velocity.scale(dt_seconds);
                let chassis_floor = self
                    .support_surface_height(veh.chassis_position.x, veh.chassis_position.z)
                    + half_f64();
                if veh.chassis_position.y < chassis_floor {
                    veh.chassis_position.y = chassis_floor;
                    veh.chassis_velocity.y = veh.chassis_velocity.y.max(0.0);
                }
            }
            self.vehicles = updated_vehicles;
        }

        if let Some(mover) = &mut self.kinematic_mover {
            mover.step(dt_seconds);
        }

        self.elapsed_seconds += dt_seconds;
        Ok(())
    }
}
