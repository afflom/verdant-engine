//! Deterministic Symplectic Euler rigid-body, capsule controller, fluid buoyancy, and spline physics.
//!
//! All dynamics are parameterized by [`PhysicsWorldConfig`] (`PhysicalEnvironment`,
//! `TerrainReliefParams`, stability timestep cap, restitution, damping, and optional causeway deck
//! elevation) with zero arbitrary magic constants.

use serde::{Deserialize, Serialize};

use crate::error::PhysicsStepError;
use crate::genesis::{terrain_height_with_params, PhysicalEnvironment, TerrainReliefParams};
use crate::math::{geometric_epsilon, half_f64, three_u32, two_f64, Aabb, Quat, Vec3};

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
    /// Optional stone causeway deck box `(bounds, deck_top_y)` bridging water basins.
    pub causeway_deck: Option<(Aabb, f64)>,
}

impl Default for PhysicsWorldConfig {
    fn default() -> Self {
        let half = half_f64();
        let quarter = half * half;
        let env = PhysicalEnvironment::temperate_terrestrial();
        let relief = TerrainReliefParams::sanctuary_amphitheater();
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
            causeway_deck: Some((deck_bounds, deck_y)),
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
    /// Whether the capsule is grounded on terrain or a structural causeway deck.
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
}

/// Harmonic damped torsion or flexural spring oscillator (`m * x'' + c * x' + k * x = F_ext`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArticulatedSpringJoint {
    /// Joint identifier (`rotunda_gimbal`, `quartz_torsion_pendulum`, `flora_culm`).
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

/// Autonomous rover following a closed piecewise-smooth waypoint loop around the world scene.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KinematicSplineRover {
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

impl KinematicSplineRover {
    /// Initializes an autonomous rover on `waypoints` cruising at `cruise_speed_mps`.
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

    /// Advances the rover along its closed perimeter spline by `dt_seconds`.
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

/// Deterministic 3D physics world combining rigid bodies, capsule player, spring joints, and rover.
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
    /// Optional autonomous perimeter sentinel rover.
    pub sentinel_rover: Option<KinematicSplineRover>,
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
            sentinel_rover: None,
            elapsed_seconds: 0.0,
        }
    }

    /// Evaluates supporting surface height (`terrain` or `causeway_deck`) at `(x, z)`.
    #[must_use]
    pub fn support_surface_height(&self, x: f64, z: f64) -> f64 {
        let mut h =
            terrain_height_with_params(self.config.terrain_seed, &self.config.terrain_relief, x, z);
        if let Some((deck_box, deck_top_y)) = self.config.causeway_deck {
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

    /// Advances all bodies, capsule controllers, joints, and rovers by `dt_seconds`.
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

            // Keep wading or walking cleanly on terrain or causeway piers above water table:
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

        if let Some(rover) = &mut self.sentinel_rover {
            rover.step(dt_seconds);
        }

        self.elapsed_seconds += dt_seconds;
        Ok(())
    }
}
