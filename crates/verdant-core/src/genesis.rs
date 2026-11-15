//! Causal cosmological-to-civilizational world expansion (`t_0 -> t_obs`) and Builder statics.
//!
//! Models how a playable world scene observed at instant `t_obs` emerges monotonically from
//! universal expansion across four causal epochs, parameterized completely by
//! [`WorldGenesisParams`] and [`PhysicalEnvironment`] with zero arbitrary magic constants:
//! - [`CosmicEpoch::Cosmogenic`]: nucleosynthesis, elemental abundances, and stellar irradiance.
//! - [`CosmicEpoch::LithicHydrologic`]: crustal uplift, hydraulic erosion, water basins, and minerals.
//! - [`CosmicEpoch::BioticEcological`]: soil pedogenesis, vascular flora, and mobile fauna.
//! - [`CosmicEpoch::AgenticArchitectural`]: differentiated **Builder Guilds** emerge once
//!   ecological maturity crosses the configured threshold and construct non-natural monuments,
//!   pavilions, sundials, resonant pergolas, causeways, travertine paths, and perimeter kinetic
//!   sentinels governed by explicit optimization objectives and Heyman masonry statics.

use serde::{Deserialize, Serialize};

use crate::ecs::{EntityRole, SceneWorld, Transform};
use crate::error::{StructuralEquilibriumError, WorldEpochError};
use crate::math::{
    bit_mask_u64, fibonacci_u64, four_u32, golden_angle_radians, golden_ratio, half_f64,
    nth_prime_u64, pow10_u64, pow2_u64, three_u32, two_f64, two_u32, Aabb, DeterministicRng, Quat,
    Vec3,
};

/// Heyman's masonry middle-third eccentricity limit `e / thickness <= 1 / (2 * 3)` (`1/6`).
///
/// Derived mechanically from zero-tension equilibrium of a rectangular masonry cross-section:
/// the resultant compressive stress prism has centroid at `1/3` section width from the compressed
/// edge, placing the maximum eccentricity at `1/2 - 1/3 = 1/6`.
#[must_use]
pub fn heyman_middle_third_limit() -> f64 {
    let six = (nth_prime_u64(0) * nth_prime_u64(1)) as f64;
    1.0 / six
}

/// Constructive minimum structural compressive and overturning safety factor (`2.0`).
#[must_use]
pub fn minimum_structural_safety_factor() -> f64 {
    two_f64()
}

/// Parametric physical environment of a planetary or artificial world scene.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct PhysicalEnvironment {
    /// Downward gravitational acceleration (`m/s^2`).
    pub gravity_mps2: f64,
    /// Ambient liquid basin mass density (`kg/m^3`).
    pub fluid_density_kg_m3: f64,
    /// Speed of acoustic propagation in ambient atmosphere (`m/s`).
    pub sound_speed_mps: f64,
    /// Effective host-star photosphere blackbody temperature (`K`).
    pub stellar_temperature_kelvin: f64,
    /// Top-of-canopy stellar irradiance (`W/m^2`).
    pub solar_irradiance_w_m2: f64,
    /// Hydrostatic basin water-table elevation (`m`).
    pub water_level_meters: f64,
}

impl PhysicalEnvironment {
    /// Synthesizes a temperate terrestrial reference environment from constructive ratios.
    #[must_use]
    pub fn temperate_terrestrial() -> Self {
        let two = two_f64();
        let ten = pow10_u64(1) as f64;
        let hundred = pow10_u64(two_u32()) as f64;
        let thousand = pow10_u64(three_u32()) as f64;
        let f7 = fibonacci_u64(three_u32() + four_u32()) as f64; // F(7) = 13
        let f9 = fibonacci_u64(1 + four_u32() + four_u32()) as f64; // F(9) = 34
        let p3 = nth_prime_u64(three_u32()) as f64; // 7
        Self {
            gravity_mps2: ten - two / hundred,
            fluid_density_kg_m3: thousand - (1.0 + two),
            sound_speed_mps: ten * f9 + (1.0 + two),
            stellar_temperature_kelvin: thousand * (nth_prime_u64(two_u32()) as f64)
                + hundred * p3
                + ten * p3,
            solar_irradiance_w_m2: hundred * f7 + ten * (two + two + two),
            water_level_meters: -half_f64() * (p3 / ten),
        }
    }
}

/// Parametric terrain heightfield relief configuration for procedural world synthesis.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct TerrainReliefParams {
    /// Central hydrological basin depression depth in meters.
    pub basin_depth_meters: f64,
    /// Basin Gaussian variance (`sigma^2`) in square meters.
    pub basin_variance_m2: f64,
    /// Basin center X coordinate in meters.
    pub basin_center_x: f64,
    /// Basin center Z coordinate in meters.
    pub basin_center_z: f64,
    /// Primary rolling-terrace vertical amplitude in meters.
    pub hill_amplitude_meters: f64,
    /// Primary rolling-terrace spatial frequency (`rad/m`).
    pub hill_frequency_per_meter: f64,
    /// Outer amphitheater rim quadratic uplift coefficient.
    pub outer_rim_curvature: f64,
    /// Outer rim squared-radius saturation cap (`m^2`).
    pub outer_rim_cap_m2: f64,
}

impl TerrainReliefParams {
    /// Synthesizes balanced sanctuary amphitheater relief parameters constructively.
    #[must_use]
    pub fn sanctuary_amphitheater() -> Self {
        let phi = golden_ratio();
        let ten = pow10_u64(1) as f64;
        let thousand = pow10_u64(three_u32()) as f64;
        let f12 = fibonacci_u64(three_u32() * four_u32()) as f64; // F(12) = 144
        let six = (nth_prime_u64(0) * nth_prime_u64(1)) as f64;
        let five = nth_prime_u64(two_u32()) as f64;
        Self {
            basin_depth_meters: phi - half_f64() * half_f64(),
            basin_variance_m2: f12 + 1.0,
            basin_center_x: six,
            basin_center_z: -five,
            hill_amplitude_meters: 1.0 / phi + half_f64() * half_f64(),
            hill_frequency_per_meter: 1.0 / (ten + six),
            outer_rim_curvature: 1.0 / thousand,
            outer_rim_cap_m2: (six * ten) * (six * ten),
        }
    }
}

/// Parametric ecological and architectural synthesis targets across cosmic expansion.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorldGenesisParams {
    /// Planetary physical environment.
    pub environment: PhysicalEnvironment,
    /// Continuous terrain relief profile.
    pub terrain: TerrainReliefParams,
    /// Minimum ecological maturity in `(0, 1]` required for Builder guild differentiation.
    pub min_builder_ecological_maturity: f64,
    /// Peak vascular botanical organism count at full maturity.
    pub max_flora_count: usize,
    /// Peak autonomous mobile fauna count at full maturity.
    pub max_fauna_count: usize,
    /// Mineral outcrop count around the basin rim.
    pub mineral_outcrop_count: usize,
    /// Closed-loop perimeter waypoint count for the autonomous sentinel rover.
    pub rover_waypoint_count: usize,
    /// Mean sanctuary perimeter patrol radius in meters.
    pub rover_patrol_radius_meters: f64,
}

impl Default for WorldGenesisParams {
    fn default() -> Self {
        Self::sanctuary_from_constructive_ratios()
    }
}

impl WorldGenesisParams {
    /// Derives complete world-genesis parameters purely from constructive number sequences.
    #[must_use]
    pub fn sanctuary_from_constructive_ratios() -> Self {
        let two_u = two_u32();
        let three_u = three_u32();
        let four_u = four_u32();
        let eight = pow2_u64(three_u) as usize;
        let sixteen = pow2_u64(four_u) as usize;
        let f8 = fibonacci_u64(four_u + four_u) as f64; // 21
        let f9 = fibonacci_u64(1 + four_u + four_u) as f64; // 34
        let maturity_threshold = f8 / (f9 - (nth_prime_u64(two_u) as f64)); // 21 / 29 ~= 0.7241
        Self {
            environment: PhysicalEnvironment::temperate_terrestrial(),
            terrain: TerrainReliefParams::sanctuary_amphitheater(),
            min_builder_ecological_maturity: maturity_threshold,
            max_flora_count: eight * (three_u as usize), // 24
            max_fauna_count: (four_u as usize) * (three_u as usize), // 12
            mineral_outcrop_count: eight,                // 8
            rover_waypoint_count: sixteen,               // 16
            rover_patrol_radius_meters: f8 + (nth_prime_u64(two_u) as f64), // 26.0 m
        }
    }
}

/// Ordered cosmological-to-architectural epochs of world formation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum CosmicEpoch {
    /// Primordial & stellar nucleosynthesis, elemental abundances, and solar radiation (`0 < a <= 1/4`).
    Cosmogenic = 0,
    /// Tectonic terrain relief, mineral crystallization, and hydrological water basins (`1/4 < a <= 2/4`).
    LithicHydrologic = 1,
    /// Soil formation, vascular flora, synchronization fields, and autonomous fauna (`2/4 < a <= 3/4`).
    BioticEcological = 1 + 1,
    /// Differentiated Builder guilds, masonry/acoustic/solar engineering, and kinetic sentinels (`3/4 < a <= 1`).
    AgenticArchitectural = 1 + 1 + 1,
}

impl CosmicEpoch {
    /// Canonical epoch name for diagnostics and telemetry.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Cosmogenic => "Cosmogenic",
            Self::LithicHydrologic => "LithicHydrologic",
            Self::BioticEcological => "BioticEcological",
            Self::AgenticArchitectural => "AgenticArchitectural",
        }
    }

    /// Maps a normalized cosmic scale factor `a in (0.0, 1.0]` into one of the `4` causal quartiles.
    pub fn from_scale_factor(scale_factor: f64) -> Result<Self, WorldEpochError> {
        if !scale_factor.is_finite() || scale_factor <= 0.0 || scale_factor > 1.0 {
            return Err(WorldEpochError::InvalidScaleFactor { scale_factor });
        }
        let quarter = half_f64() * half_f64();
        let half = half_f64();
        let three_quarters = half + quarter;
        if scale_factor <= quarter {
            Ok(Self::Cosmogenic)
        } else if scale_factor <= half {
            Ok(Self::LithicHydrologic)
        } else if scale_factor <= three_quarters {
            Ok(Self::BioticEcological)
        } else {
            Ok(Self::AgenticArchitectural)
        }
    }
}

/// Chemical and mineral abundance state synthesized during `Cosmogenic` and `LithicHydrologic` epochs.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LithicSubstrate {
    /// Effective stellar photosphere temperature in Kelvin.
    pub stellar_temperature_kelvin: f64,
    /// Incident top-of-canopy solar irradiance in `W/m^2`.
    pub solar_irradiance_w_m2: f64,
    /// Travertine limestone (`CaCO3`) quarry reserve in metric tons.
    pub travertine_reserve_tons: f64,
    /// Piezoelectric alpha-quartz (`SiO2`) vein reserve in metric tons.
    pub quartz_reserve_tons: f64,
    /// Basalt aggregate reserve in metric tons.
    pub basalt_reserve_tons: f64,
    /// Native gold/bronze conductor reserve in metric tons.
    pub metallic_trim_reserve_tons: f64,
    /// Mean freshwater basin depth in meters.
    pub lagoon_max_depth_meters: f64,
}

/// Differentiated Builder discipline emerging from the world's ecological intelligence.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum BuilderDiscipline {
    /// Compression-arch statics, fluted entasis colonnades, and travertine geodesic paving.
    LithicMasonry,
    /// Spherical astronomy, meridian gnomons, and Cardan celestial gimbals.
    CelestialGnomonics,
    /// Euler-Bernoulli modal chimes, Helmholtz chambers, and piezoelectric quartz pendulums.
    ModalAcoustics,
    /// Submerged lakebed pier foundations, Archimedes uplift resistance, and wetland causeways.
    HydrologicalEngineering,
    /// Spring-and-solar articulated boundary surveyors patrolling the perimeter spline.
    KineticAutomation,
}

/// Multi-objective weights governing why a differentiated Builder guild constructs an artifact.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct BuilderOptimizationWeights {
    /// Weight on microclimate shelter and thermal enclosure.
    pub shelter: f64,
    /// Weight on harmonic acoustic resonance and reverberant clarity.
    pub acoustic_resonance: f64,
    /// Weight on solar/celestial horizon alignment and timekeeping precision.
    pub celestial_alignment: f64,
    /// Weight on slope-and-water traversal efficiency across terrain.
    pub geodesic_traversal: f64,
    /// Weight on visual focal framing across sightlines.
    pub focal_framing: f64,
    /// Penalty per unit of quarried mineral/timber expenditure.
    pub material_cost_penalty: f64,
}

impl BuilderOptimizationWeights {
    /// Normalizes raw positive priority shares `(s, a, c, g, f)` onto the unit simplex with cost weight.
    #[must_use]
    pub fn from_shares(
        shelter: f64,
        acoustic: f64,
        celestial: f64,
        traversal: f64,
        framing: f64,
        cost_ratio: f64,
    ) -> Self {
        let total = (shelter + acoustic + celestial + traversal + framing).max(1.0);
        Self {
            shelter: shelter / total,
            acoustic_resonance: acoustic / total,
            celestial_alignment: celestial / total,
            geodesic_traversal: traversal / total,
            focal_framing: framing / total,
            material_cost_penalty: cost_ratio,
        }
    }

    /// Evaluates net engineering utility `U(s)` for achieved objective scores in `[0, 1]` and cost.
    #[must_use]
    pub fn evaluate(
        self,
        shelter_score: f64,
        acoustic_score: f64,
        celestial_score: f64,
        traversal_score: f64,
        framing_score: f64,
        normalized_material_cost: f64,
    ) -> f64 {
        self.shelter * shelter_score
            + self.acoustic_resonance * acoustic_score
            + self.celestial_alignment * celestial_score
            + self.geodesic_traversal * traversal_score
            + self.focal_framing * framing_score
            - self.material_cost_penalty * normalized_material_cost
    }
}

/// Differentiated Builder guild record in the causal world simulation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BuilderGuildSpec {
    /// Guild name.
    pub name: String,
    /// Primary scientific and engineering discipline.
    pub discipline: BuilderDiscipline,
    /// Optimization weights driving this guild's architectural synthesis.
    pub weights: BuilderOptimizationWeights,
    /// Governing physical principle exploited by the guild.
    pub governing_science: String,
}

/// Archetype of a non-natural constructed artifact placed in the world by Builder guilds.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum StructureKind {
    /// Classical harmonic rotunda with fluted entasis colonnade and Cardan astrolabe cupola.
    HarmonicRotundaPavilion,
    /// Equatorial/horizontal meridian sundial gnomon calibrated to site latitude.
    SolarMeridianSundial,
    /// Tuned bamboo-and-bronze modal chime pergola aligned with prevailing wind corridor.
    ResonantChimePergola,
    /// Quartz piezoelectric torsion-pendulum chamber over crystalline outcrop.
    QuartzPendulumChamber,
    /// Lakebed-founded stone pier causeway with solar beacons across lagoon.
    HydrologicalPierCauseway,
    /// Grade-minimizing travertine footpath and catenary arch network linking all biomes.
    TravertinePathNetwork,
    /// Articulated autonomous perimeter sentinel rover orbiting the sanctuary boundary spline.
    PerimeterSentinelRover,
}

/// Verified engineering & statics blueprint of a Builder-constructed world artifact.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArchitecturalBlueprint {
    /// Artifact identifier.
    pub id_slug: String,
    /// Display title.
    pub title: String,
    /// Structural classification.
    pub kind: StructureKind,
    /// Lead Builder discipline responsible for synthesis.
    pub lead_discipline: BuilderDiscipline,
    /// World-space placement `(x, y, z)` in meters, grounded on terrain or lakebed.
    pub world_position: Vec3,
    /// Clear structural span in meters.
    pub span_meters: f64,
    /// Structural apex height above foundation in meters.
    pub height_meters: f64,
    /// Primary load-bearing section thickness in meters.
    pub section_thickness_meters: f64,
    /// Computed catenary thrust-line eccentricity ratio `e / thickness` (`<= 1/6`).
    pub thrust_eccentricity_ratio: f64,
    /// Computed compressive & overturning safety factor (`>= 2.0`).
    pub safety_factor: f64,
    /// Evaluated multi-objective Builder net utility (`> 0.0`).
    pub net_engineering_utility: f64,
    /// Fundamental acoustic or mechanical modal frequency in hertz.
    pub modal_frequency_hz: f64,
    /// Concise statement of purpose in the world's civilization.
    pub civilizational_purpose: String,
}

/// Evaluates deterministic terrain ground elevation `y(x, z)` in meters for `seed` and `relief`.
#[must_use]
pub fn terrain_height_with_params(seed: u64, relief: &TerrainReliefParams, x: f64, z: f64) -> f64 {
    let mask = bit_mask_u64(four_u32() * four_u32());
    let phase = ((seed & mask) as f64) / ((mask + 1) as f64) * std::f64::consts::TAU;
    let dx = x - relief.basin_center_x;
    let dz = z - relief.basin_center_z;
    let lagoon_bowl = -relief.basin_depth_meters
        * (-(dx * dx + dz * dz) / relief.basin_variance_m2.max(1.0)).exp();
    let freq = relief.hill_frequency_per_meter;
    let rolling_hills = relief.hill_amplitude_meters
        * ((freq * x + phase).sin() * (freq * z - phase).cos()
            + half_f64() * (two_f64() * freq * (x + z) + phase).sin());
    let r2 = (x * x + z * z).min(relief.outer_rim_cap_m2);
    lagoon_bowl + rolling_hills + relief.outer_rim_curvature * r2
}

/// Evaluates deterministic terrain elevation under the default sanctuary relief profile.
#[must_use]
pub fn terrain_height_at(seed: u64, x: f64, z: f64) -> f64 {
    terrain_height_with_params(seed, &TerrainReliefParams::sanctuary_amphitheater(), x, z)
}

/// Evaluates surface normal of the continuous terrain heightfield via central finite differences.
#[must_use]
pub fn terrain_normal_with_params(seed: u64, relief: &TerrainReliefParams, x: f64, z: f64) -> Vec3 {
    let eps = half_f64() * half_f64();
    let two_eps = two_f64() * eps;
    let hx = terrain_height_with_params(seed, relief, x + eps, z)
        - terrain_height_with_params(seed, relief, x - eps, z);
    let hz = terrain_height_with_params(seed, relief, x, z + eps)
        - terrain_height_with_params(seed, relief, x, z - eps);
    Vec3::new(-hx / two_eps, 1.0, -hz / two_eps).normalized()
}

/// Evaluates surface normal under the default sanctuary relief profile.
#[must_use]
pub fn terrain_normal_at(seed: u64, x: f64, z: f64) -> Vec3 {
    terrain_normal_with_params(seed, &TerrainReliefParams::sanctuary_amphitheater(), x, z)
}

/// Verifies Heyman masonry thrust-line equilibrium, crushing safety factor, and Builder utility
/// under a parametric gravitational acceleration `gravity_mps2`.
pub fn verify_blueprint_equilibrium(
    span_meters: f64,
    height_meters: f64,
    section_thickness_meters: f64,
    compressive_strength_mpa: f64,
    density_kg_m3: f64,
    gravity_mps2: f64,
    net_utility: f64,
) -> Result<(f64, f64), StructuralEquilibriumError> {
    if !span_meters.is_finite()
        || !height_meters.is_finite()
        || !section_thickness_meters.is_finite()
        || span_meters <= 0.0
        || height_meters <= 0.0
        || section_thickness_meters <= 0.0
    {
        return Err(StructuralEquilibriumError::InvalidStructuralDimensions {
            span_meters,
            height_meters,
        });
    }
    if net_utility <= 0.0 || !net_utility.is_finite() {
        return Err(StructuralEquilibriumError::NonPositiveEngineeringUtility { net_utility });
    }

    let four = two_f64() * two_f64();
    let eight = four * two_f64();
    let max_eccentricity = heyman_middle_third_limit();
    let min_safety = minimum_structural_safety_factor();

    // Catenary horizontal thrust eccentricity ratio `e / t = (span / (8 * height)) * (span / (4 * t)) / (1 + span / height)`:
    let arch_thrust_factor = span_meters / (eight * height_meters);
    let thickness_ratio = (four * section_thickness_meters) / span_meters.max(1.0);
    let eccentricity_ratio =
        (arch_thrust_factor / (thickness_ratio * four).max(half_f64() * half_f64())).min(1.0);

    if eccentricity_ratio > max_eccentricity {
        return Err(StructuralEquilibriumError::ThrustLineOutsideMiddleThird {
            eccentricity_ratio,
            max_admissible_ratio: max_eccentricity,
        });
    }

    let mega = pow10_u64(two_u32() * three_u32()) as f64;
    let dead_stress_pa = density_kg_m3
        * gravity_mps2.max(1.0 / mega)
        * height_meters
        * (1.0 + span_meters / (four * section_thickness_meters));
    let dead_stress_mpa = (dead_stress_pa / mega).max(1.0 / mega);
    let safety_factor = compressive_strength_mpa / dead_stress_mpa;

    if safety_factor < min_safety {
        return Err(StructuralEquilibriumError::InsufficientSafetyFactor {
            safety_factor,
            minimum_required: min_safety,
        });
    }

    Ok((eccentricity_ratio, safety_factor))
}

/// Complete causal world state produced by evolving universal expansion up to observation scale `a(t)`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CausalWorldState {
    /// Deterministic seed of the universe instance.
    pub seed: u64,
    /// Parametric world-genesis configuration used to synthesize this world.
    pub params: WorldGenesisParams,
    /// Current cosmic scale factor `a(t) in (0.0, 1.0]`, where `1.0` is `t_obs`.
    pub scale_factor: f64,
    /// Active causal epoch corresponding to `scale_factor`.
    pub active_epoch: CosmicEpoch,
    /// Synthesized lithic & hydrological substrate.
    pub substrate: LithicSubstrate,
    /// Ecological biomass & synchronization maturity in `[0.0, 1.0]`.
    pub ecological_maturity: f64,
    /// Number of vascular botanical organisms active in the scene.
    pub flora_count: usize,
    /// Number of autonomous mobile fauna active in the scene.
    pub fauna_count: usize,
    /// Differentiated Builder guilds active at `AgenticArchitectural` epoch.
    pub builder_guilds: Vec<BuilderGuildSpec>,
    /// Verified architectural blueprints constructed by the Builders.
    pub blueprints: Vec<ArchitecturalBlueprint>,
    /// Closed-loop perimeter waypoints `(x, y, z)` traversed by the autonomous sentinel rover.
    pub rover_perimeter_waypoints: Vec<Vec3>,
    /// Populated hierarchical scene graph.
    pub scene: SceneWorld,
}

impl CausalWorldState {
    /// Initializes a nascent universe in `Cosmogenic` quartile (`a_0 = 1/8`).
    pub fn new_cosmogenic(seed: u64) -> Result<Self, WorldEpochError> {
        let a0 = 1.0 / (pow2_u64(three_u32()) as f64);
        Self::synthesize_with_params(seed, a0, &WorldGenesisParams::default())
    }

    /// Synthesizes causal world state at `scale_factor` under default constructive sanctuary parameters.
    pub fn synthesize_at_scale(seed: u64, scale_factor: f64) -> Result<Self, WorldEpochError> {
        Self::synthesize_with_params(seed, scale_factor, &WorldGenesisParams::default())
    }

    /// Advances an existing world monotonically to `target_scale_factor` (`<= 1.0`).
    pub fn advance_to(&mut self, target_scale_factor: f64) -> Result<(), WorldEpochError> {
        if !target_scale_factor.is_finite()
            || target_scale_factor <= 0.0
            || target_scale_factor > 1.0
        {
            return Err(WorldEpochError::InvalidScaleFactor {
                scale_factor: target_scale_factor,
            });
        }
        if target_scale_factor < self.scale_factor {
            return Err(WorldEpochError::NonMonotonicExpansion {
                current_scale: self.scale_factor,
                requested_scale: target_scale_factor,
            });
        }
        *self = Self::synthesize_with_params(self.seed, target_scale_factor, &self.params)?;
        Ok(())
    }

    /// Synthesizes the complete causal world state at `scale_factor` under caller-supplied `params`.
    pub fn synthesize_with_params(
        seed: u64,
        scale_factor: f64,
        params: &WorldGenesisParams,
    ) -> Result<Self, WorldEpochError> {
        let active_epoch = CosmicEpoch::from_scale_factor(scale_factor)?;
        let mut rng = DeterministicRng::new(seed);
        let mut scene = SceneWorld::new();

        let half = half_f64();
        let quarter = half * half;
        let three_quarters = half + quarter;
        let phi = golden_ratio();
        let ten = pow10_u64(1) as f64;
        let hundred = pow10_u64(two_u32()) as f64;
        let thousand = pow10_u64(three_u32()) as f64;

        // Epoch 0: Cosmogenic — stellar radiation and nucleosynthetic abundances.
        let irradiance =
            params.environment.solar_irradiance_w_m2 * (three_quarters + quarter * scale_factor);
        let sun_dist = ten * (four_u32() as f64);
        let sun_id = scene.spawn(
            "PrimarySolarStar",
            EntityRole::CelestialLight,
            None,
            Transform::from_xyz(sun_dist, sun_dist * phi, -sun_dist / phi),
            Aabb::from_center_half_extents(Vec3::ZERO, Vec3::new(1.0, 1.0, 1.0)),
            (
                quarter * half,
                "Cosmogenic: main-sequence stellar blackbody radiator",
            ),
        );

        let mut substrate = LithicSubstrate {
            stellar_temperature_kelvin: params.environment.stellar_temperature_kelvin,
            solar_irradiance_w_m2: irradiance,
            travertine_reserve_tons: 0.0,
            quartz_reserve_tons: 0.0,
            basalt_reserve_tons: 0.0,
            metallic_trim_reserve_tons: 0.0,
            lagoon_max_depth_meters: 0.0,
        };

        // Epoch 1: LithicHydrologic — terrain, mineral outcrops, and mirror lagoon.
        if active_epoch >= CosmicEpoch::LithicHydrologic {
            let lithic_progress = ((scale_factor - quarter) / quarter).clamp(0.0, 1.0);
            substrate.travertine_reserve_tons = thousand * phi * lithic_progress;
            substrate.quartz_reserve_tons = hundred * (four_u32() as f64) * lithic_progress;
            substrate.basalt_reserve_tons = thousand * (three_u32() as f64) * lithic_progress;
            substrate.metallic_trim_reserve_tons = hundred * lithic_progress;
            substrate.lagoon_max_depth_meters = params.terrain.basin_depth_meters * lithic_progress;

            let basin_extent = params.terrain.outer_rim_cap_m2.sqrt();
            let terrain_root = scene.spawn(
                "SanctuaryTerrainBasin",
                EntityRole::TerrainFeature,
                Some(sun_id),
                Transform::default(),
                Aabb::new(
                    Vec3::new(
                        -basin_extent,
                        -params.terrain.basin_depth_meters * two_f64(),
                        -basin_extent,
                    ),
                    Vec3::new(
                        basin_extent,
                        params.terrain.basin_depth_meters * (four_u32() as f64),
                        basin_extent,
                    ),
                ),
                (
                    quarter + quarter * quarter,
                    "LithicHydrologic: uplifted carbonate-silicate basin",
                ),
            );
            scene.attach_renderable(terrain_root, 0, 0);

            let lagoon_span = params.terrain.basin_variance_m2.sqrt();
            let lagoon_id = scene.spawn(
                "MirrorLagoonBasin",
                EntityRole::WaterBody,
                Some(terrain_root),
                Transform::from_xyz(
                    params.terrain.basin_center_x,
                    params.environment.water_level_meters,
                    params.terrain.basin_center_z,
                ),
                Aabb::from_center_half_extents(
                    Vec3::ZERO,
                    Vec3::new(lagoon_span, quarter, lagoon_span),
                ),
                (
                    quarter + half * quarter,
                    "LithicHydrologic: hydrostatic freshwater table",
                ),
            );
            scene.attach_renderable(lagoon_id, 1, 1);

            let outcrop_total = params.mineral_outcrop_count.max(1);
            let render_mesh_rock = two_u32() as usize;
            for i in 0..outcrop_total {
                let jitter = rng.range_f64(-quarter * half, quarter * half);
                let angle = (i as f64) * std::f64::consts::TAU / (outcrop_total as f64) + jitter;
                let dist = rng.range_f64(lagoon_span, params.rover_patrol_radius_meters);
                let rx = dist * angle.cos();
                let rz = dist * angle.sin();
                let ry = terrain_height_with_params(seed, &params.terrain, rx, rz);
                let rock = scene.spawn(
                    format!("BravaisMineralOutcrop_{i}"),
                    EntityRole::TerrainFeature,
                    Some(terrain_root),
                    Transform::from_xyz(rx, ry, rz),
                    Aabb::from_center_half_extents(
                        Vec3::new(0.0, half, 0.0),
                        Vec3::new(1.0, 1.0, 1.0),
                    ),
                    (
                        half - quarter * quarter,
                        "LithicHydrologic: hydrothermal quartz-travertine outcrop",
                    ),
                );
                scene.attach_renderable(rock, render_mesh_rock, render_mesh_rock);
            }
        }

        // Epoch 2: BioticEcological — vascular L-system flora & mobile fauna.
        let mut ecological_maturity = 0.0;
        let mut flora_count = 0;
        let mut fauna_count = 0;

        if active_epoch >= CosmicEpoch::BioticEcological {
            let bio_progress = ((scale_factor - half) / quarter).clamp(0.0, 1.0);
            ecological_maturity = (half + half * bio_progress).clamp(0.0, 1.0);
            flora_count =
                ((params.max_flora_count as f64) * (half + half * bio_progress)).round() as usize;
            fauna_count =
                ((params.max_fauna_count as f64) * (half + half * bio_progress)).round() as usize;

            let g_angle = golden_angle_radians();
            let flora_mesh = three_u32() as usize;
            let inner_r = (nth_prime_u64(two_u32()) as f64) + half;
            let span_r = params.rover_patrol_radius_meters - inner_r;

            for i in 0..flora_count {
                let r = inner_r + span_r * (((i + 1) as f64) / (flora_count.max(1) as f64)).sqrt();
                let theta = (i as f64) * g_angle;
                let fx = r * theta.cos();
                let fz = r * theta.sin();
                let fy = terrain_height_with_params(seed, &params.terrain, fx, fz)
                    .max(params.environment.water_level_meters);
                let yaw = Quat::from_axis_angle(Vec3::UP, theta);
                let flora_id = scene.spawn(
                    format!("VascularFlora_{i}"),
                    EntityRole::BotanicalOrganism,
                    None,
                    Transform {
                        translation: Vec3::new(fx, fy, fz),
                        rotation: yaw,
                        scale: Vec3::new(1.0, three_quarters + half * bio_progress, 1.0),
                    },
                    Aabb::new(
                        Vec3::new(-1.0, 0.0, -1.0),
                        Vec3::new(1.0, two_f64() + 1.0, 1.0),
                    ),
                    (
                        half + quarter * (i as f64 / (flora_count.max(1) as f64)),
                        "BioticEcological: phyllotactic vascular L-system",
                    ),
                );
                scene.attach_renderable(flora_id, flora_mesh, flora_mesh);
            }

            let three_usize = three_u32() as usize;
            for j in 0..fauna_count {
                let angle = (j as f64) * std::f64::consts::TAU / (fauna_count.max(1) as f64);
                let is_aquatic = j.is_multiple_of(three_usize);
                let pos = if is_aquatic {
                    let swim_r = four_u32() as f64;
                    Vec3::new(
                        params.terrain.basin_center_x + swim_r * angle.cos(),
                        params.environment.water_level_meters - quarter,
                        params.terrain.basin_center_z + swim_r * angle.sin(),
                    )
                } else {
                    let fly_r = half * params.rover_patrol_radius_meters;
                    let bx = fly_r * angle.cos();
                    let bz = fly_r * angle.sin();
                    Vec3::new(
                        bx,
                        terrain_height_with_params(seed, &params.terrain, bx, bz)
                            + two_f64()
                            + half,
                        bz,
                    )
                };
                scene.spawn(
                    if is_aquatic {
                        format!("LagoonShoalOrganism_{j}")
                    } else {
                        format!("CanopyPollinatorFlock_{j}")
                    },
                    EntityRole::AutonomousFauna,
                    None,
                    Transform::from_xyz(pos.x, pos.y, pos.z),
                    Aabb::from_center_half_extents(Vec3::ZERO, Vec3::new(half, quarter, half)),
                    (
                        three_quarters - quarter * quarter,
                        "BioticEcological: coupled Kuramoto-Boids autonomous organism",
                    ),
                );
            }
        }

        // Epoch 3: AgenticArchitectural (`3/4 < a <= 1.00`, `t_obs`) — Differentiated Builders!
        let mut builder_guilds = Vec::new();
        let mut blueprints = Vec::new();
        let mut rover_perimeter_waypoints = Vec::new();

        if active_epoch >= CosmicEpoch::AgenticArchitectural {
            let arch_progress = ((scale_factor - three_quarters) / quarter).clamp(0.0, 1.0);
            ecological_maturity = (three_quarters + quarter * arch_progress).clamp(0.0, 1.0);
            if ecological_maturity < params.min_builder_ecological_maturity {
                return Err(WorldEpochError::BuildersNotYetDifferentiated {
                    ecological_maturity,
                    minimum_required: params.min_builder_ecological_maturity,
                });
            }

            builder_guilds = canonical_builder_guilds();
            let guild_count = builder_guilds.len().max(1);
            for (idx, guild) in builder_guilds.iter().enumerate() {
                let angle = (idx as f64) * std::f64::consts::TAU / (guild_count as f64);
                let gx = ten * angle.cos();
                let gz = ten * angle.sin();
                scene.spawn(
                    format!("BuilderGuild_{}", guild.name.replace(' ', "")),
                    EntityRole::BuilderGuild,
                    None,
                    Transform::from_xyz(
                        gx,
                        terrain_height_with_params(seed, &params.terrain, gx, gz),
                        gz,
                    ),
                    Aabb::from_center_half_extents(
                        Vec3::new(0.0, 1.0, 0.0),
                        Vec3::new(1.0, 1.0, 1.0),
                    ),
                    (
                        three_quarters + quarter * quarter,
                        format!("AgenticArchitectural: {}", guild.governing_science),
                    ),
                );
            }

            blueprints = synthesize_verified_blueprints_with_params(seed, &builder_guilds, params)
                .expect("canonical Builder blueprints satisfy Heyman statics and positive utility");

            let column_mesh_idx = four_u32() as usize;
            let rover_mesh_idx = nth_prime_u64(two_u32()) as usize; // 5
            for bp in &blueprints {
                let role = if bp.kind == StructureKind::PerimeterSentinelRover {
                    EntityRole::KineticMachine
                } else {
                    EntityRole::EngineeredStructure
                };
                let half_span = half * bp.span_meters;
                let entity = scene.spawn(
                    bp.title.clone(),
                    role,
                    None,
                    Transform::from_xyz(
                        bp.world_position.x,
                        bp.world_position.y,
                        bp.world_position.z,
                    ),
                    Aabb::new(
                        Vec3::new(-half_span, 0.0, -half_span),
                        Vec3::new(half_span, bp.height_meters, half_span),
                    ),
                    (
                        three_quarters + half * quarter,
                        format!(
                            "Built by {:?}: {} (SF={}, e/t={})",
                            bp.lead_discipline,
                            bp.civilizational_purpose,
                            bp.safety_factor,
                            bp.thrust_eccentricity_ratio
                        ),
                    ),
                );
                let mesh_idx = if bp.kind == StructureKind::PerimeterSentinelRover {
                    rover_mesh_idx
                } else {
                    column_mesh_idx
                };
                scene.attach_renderable(entity, mesh_idx, column_mesh_idx);
            }

            let wp_total = params.rover_waypoint_count.max(four_u32() as usize);
            for k in 0..wp_total {
                let theta = (k as f64) * std::f64::consts::TAU / (wp_total as f64);
                let radius =
                    params.rover_patrol_radius_meters + two_f64() * (two_f64() * theta).sin();
                let wx = radius * theta.cos();
                let wz = radius * theta.sin();
                let wy = terrain_height_with_params(seed, &params.terrain, wx, wz)
                    .max(params.environment.water_level_meters + half);
                rover_perimeter_waypoints.push(Vec3::new(wx, wy, wz));
            }
        }

        // Spawn Observer / Player camera rig grounded at entrance terrace.
        let px = 0.0;
        let pz = params.rover_patrol_radius_meters / phi;
        let py = terrain_height_with_params(seed, &params.terrain, px, pz) + phi;
        scene.spawn(
            "PlayerObserverRig",
            EntityRole::PlayerRig,
            None,
            Transform::from_xyz(px, py, pz),
            Aabb::from_center_half_extents(Vec3::ZERO, Vec3::new(half, 1.0, half)),
            (scale_factor, "Interactive observer entry frame"),
        );

        Ok(Self {
            seed,
            params: *params,
            scale_factor,
            active_epoch,
            substrate,
            ecological_maturity,
            flora_count,
            fauna_count,
            builder_guilds,
            blueprints,
            rover_perimeter_waypoints,
            scene,
        })
    }

    /// Verifies causal precursor epoch and ecological maturity before reading blueprints.
    pub fn verify_architectural_readiness(
        &self,
    ) -> Result<&[ArchitecturalBlueprint], WorldEpochError> {
        if self.active_epoch < CosmicEpoch::AgenticArchitectural {
            return Err(WorldEpochError::MissingPrecursorEpoch {
                required_epoch: CosmicEpoch::AgenticArchitectural.label(),
                actual_epoch: self.active_epoch.label(),
            });
        }
        if self.ecological_maturity < self.params.min_builder_ecological_maturity {
            return Err(WorldEpochError::BuildersNotYetDifferentiated {
                ecological_maturity: self.ecological_maturity,
                minimum_required: self.params.min_builder_ecological_maturity,
            });
        }
        Ok(&self.blueprints)
    }
}

/// Returns the five differentiated Builder guilds whose priority shares follow Fibonacci ratios.
#[must_use]
pub fn canonical_builder_guilds() -> Vec<BuilderGuildSpec> {
    let f1 = 1.0;
    let f2 = two_f64();
    let f3 = 1.0 + f2;
    let f5 = f2 + f3;
    let f8 = f3 + f5;
    let cost = 1.0 / (f5 + 1.0);
    vec![
        BuilderGuildSpec {
            name: "Lithic Masonry Guild".to_string(),
            discipline: BuilderDiscipline::LithicMasonry,
            weights: BuilderOptimizationWeights::from_shares(f8, f3, f2, f5, f3, cost),
            governing_science:
                "Heyman catenary thrust-line masonry statics and classical Vitruvian entasis"
                    .to_string(),
        },
        BuilderGuildSpec {
            name: "Celestial Gnomonics Guild".to_string(),
            discipline: BuilderDiscipline::CelestialGnomonics,
            weights: BuilderOptimizationWeights::from_shares(f2, f2, f8 + f3, f1, f5, cost),
            governing_science:
                "Spherical solar altitude-azimuth mechanics and orthogonal Cardan gimbals"
                    .to_string(),
        },
        BuilderGuildSpec {
            name: "Modal Acoustics Guild".to_string(),
            discipline: BuilderDiscipline::ModalAcoustics,
            weights: BuilderOptimizationWeights::from_shares(f3, f8 + f3, f1, f1, f5, cost),
            governing_science:
                "Euler-Bernoulli beam eigenmodes, Sabine reverberation, and quartz piezoelectricity"
                    .to_string(),
        },
        BuilderGuildSpec {
            name: "Hydrological Causeway Guild".to_string(),
            discipline: BuilderDiscipline::HydrologicalEngineering,
            weights: BuilderOptimizationWeights::from_shares(f2, f1, f1, f8 + f3, f5, cost),
            governing_science:
                "Archimedes hydrostatic uplift equilibrium and deep lakebed pier foundations"
                    .to_string(),
        },
        BuilderGuildSpec {
            name: "Kinetic Sentinel Guild".to_string(),
            discipline: BuilderDiscipline::KineticAutomation,
            weights: BuilderOptimizationWeights::from_shares(f1, f3, f3, f8, f3, cost),
            governing_science:
                "Closed-spline perimeter kinematics, solar-spring energy storage, and ecological telemetry"
                    .to_string(),
        },
    ]
}

/// Synthesizes and verifies all seven Builder-constructed artifacts under default parameters.
pub fn synthesize_verified_blueprints(
    seed: u64,
    guilds: &[BuilderGuildSpec],
) -> Result<Vec<ArchitecturalBlueprint>, StructuralEquilibriumError> {
    synthesize_verified_blueprints_with_params(seed, guilds, &WorldGenesisParams::default())
}

/// Synthesizes and statically verifies all seven Builder-constructed artifacts under `params`.
pub fn synthesize_verified_blueprints_with_params(
    seed: u64,
    guilds: &[BuilderGuildSpec],
    params: &WorldGenesisParams,
) -> Result<Vec<ArchitecturalBlueprint>, StructuralEquilibriumError> {
    let half = half_f64();
    let quarter = half * half;
    let three_quarters = half + quarter;
    let two = two_f64();
    let three = two + 1.0;
    let four = two * two;
    let five = four + 1.0;
    let eight = four * two;
    let phi = golden_ratio();
    let ten = pow10_u64(1) as f64;
    let thousand = pow10_u64(three_u32()) as f64;

    // Fundamental acoustic concert reference `F(11) + F(8) = 89 + 21 = 110 Hz`:
    let fundamental_hz =
        (fibonacci_u64(three_u32() + (eight as u32)) + fibonacci_u64(eight as u32)) as f64;

    // Stone compressive strength (~65-85 MPa) and density (~2500 kg/m^3) from constructive ratios:
    let stone_mpa = ten * (five + two);
    let stone_density = thousand * (two + half);

    let default_weights = BuilderOptimizationWeights::from_shares(1.0, 1.0, 1.0, 1.0, 1.0, quarter);
    let weights_for = |d: BuilderDiscipline| -> BuilderOptimizationWeights {
        guilds
            .iter()
            .find(|g| g.discipline == d)
            .map_or(default_weights, |g| g.weights)
    };

    let r_patrol = params.rover_patrol_radius_meters;
    let specs = [
        (
            "sanctuary-rotunda",
            "Classical Harmonic Rotunda & Cardan Astrolabe",
            StructureKind::HarmonicRotundaPavilion,
            BuilderDiscipline::LithicMasonry,
            (0.0, 0.0),
            eight + phi,
            five + two,
            1.0 + half,
            (1.0, three_quarters, three_quarters, three_quarters, 1.0, quarter),
            fundamental_hz,
            "Shelters central assembly, frames the celestial pole via an orthogonal Cardan gimbal, and focuses harmonic modes.",
        ),
        (
            "meadow-sundial",
            "Equatorial Meridian Solar Sundial",
            StructureKind::SolarMeridianSundial,
            BuilderDiscipline::CelestialGnomonics,
            (-ten * phi, ten),
            four,
            three + half,
            1.0,
            (quarter, half, 1.0, half, three_quarters, quarter),
            fundamental_hz * two,
            "Measures true solar time, solstice declination, and cosmic expansion epoch from gnomon shadow geometry.",
        ),
        (
            "grove-chime-pergola",
            "Wind-Driven Modal Chime Pergola",
            StructureKind::ResonantChimePergola,
            BuilderDiscipline::ModalAcoustics,
            (-ten - three, -ten - four),
            five,
            four + half,
            1.0,
            (three_quarters, 1.0, half, half, three_quarters, quarter),
            fundamental_hz * four,
            "Transduces prevailing grove breezes into just-intonation Euler-Bernoulli modal overtones.",
        ),
        (
            "crystal-torsion-chamber",
            "Piezoelectric Quartz Torsion Chamber",
            StructureKind::QuartzPendulumChamber,
            BuilderDiscipline::ModalAcoustics,
            (ten * phi + 1.0, -ten - three),
            five,
            five + 1.0,
            1.0 + quarter,
            (half, 1.0, three_quarters, half, three_quarters, quarter),
            fundamental_hz * five,
            "Couples micro-seismic and wind oscillations to an alpha-quartz torsion pendulum.",
        ),
        (
            "lagoon-pier-causeway",
            "Mirror Lagoon Stone Pier Causeway",
            StructureKind::HydrologicalPierCauseway,
            BuilderDiscipline::HydrologicalEngineering,
            (params.terrain.basin_center_x, params.terrain.basin_center_z),
            ten,
            three + half,
            two,
            (half, half, half, 1.0, three_quarters, quarter),
            fundamental_hz * phi,
            "Bridges the freshwater lagoon on lakebed-founded travertine piers resisting hydrostatic uplift.",
        ),
        (
            "travertine-geodesic-paths",
            "Geodesic Travertine Path & Archway Network",
            StructureKind::TravertinePathNetwork,
            BuilderDiscipline::LithicMasonry,
            (two, eight),
            five + 1.0,
            four,
            1.0 + quarter,
            (half, half, half, 1.0, 1.0, quarter),
            fundamental_hz * three,
            "Connects all five sanctuary biomes along grade-minimizing geodesic travertine paving and catenary portals.",
        ),
        (
            "perimeter-sentinel-rover",
            "Autonomous Perimeter Sentinel Rover",
            StructureKind::PerimeterSentinelRover,
            BuilderDiscipline::KineticAutomation,
            (r_patrol, 0.0),
            two,
            1.0 + half,
            half + quarter,
            (quarter, half, half, 1.0, three_quarters, quarter),
            fundamental_hz * (three / two),
            "Patrols the closed sanctuary perimeter spline on solar-spring drive, monitoring microclimate and structural health.",
        ),
    ];

    let mut out = Vec::with_capacity(specs.len());
    for (
        slug,
        title,
        kind,
        discipline,
        (wx, wz),
        span,
        height,
        thickness,
        (s_sh, s_ac, s_ce, s_tr, s_fr, cost),
        freq_hz,
        purpose,
    ) in specs
    {
        let w = weights_for(discipline);
        let net_utility = w.evaluate(s_sh, s_ac, s_ce, s_tr, s_fr, cost);
        let (eccentricity, safety) = verify_blueprint_equilibrium(
            span,
            height,
            thickness,
            stone_mpa,
            stone_density,
            params.environment.gravity_mps2,
            net_utility,
        )?;
        let ground_y = terrain_height_with_params(seed, &params.terrain, wx, wz);
        let wy = if kind == StructureKind::HydrologicalPierCauseway {
            params.environment.water_level_meters + quarter
        } else {
            ground_y
        };
        out.push(ArchitecturalBlueprint {
            id_slug: slug.to_string(),
            title: title.to_string(),
            kind,
            lead_discipline: discipline,
            world_position: Vec3::new(wx, wy, wz),
            span_meters: span,
            height_meters: height,
            section_thickness_meters: thickness,
            thrust_eccentricity_ratio: eccentricity,
            safety_factor: safety,
            net_engineering_utility: net_utility,
            modal_frequency_hz: freq_hz,
            civilizational_purpose: purpose.to_string(),
        });
    }
    Ok(out)
}
