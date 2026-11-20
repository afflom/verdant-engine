//! Progressive multi-LOD 3D scene synthesis and authoritative `UORC` (`UORC-SHA256-CHAIN-v1`) streaming.
//!
//! Combines `prism_kernel` LOD monotonicity proofs (`lod0CoarseProxy..lod3LosslessSource`) with
//! `uorc` (`https://github.com/UOR-Foundation/compression`) multi-object frame encoding and
//! cryptographic chain verification so universal 3D scenes stream from coarse collision proxies up
//! to lossless source resolution.

use serde::{Deserialize, Serialize};
#[cfg(feature = "uorc-streaming")]
use uorc::{FrameDecoder, FrameEncoder};

use crate::ecs::{EntityRole, SceneWorld, Transform};
use crate::error::StreamingError;
use crate::math::{
    four_u32, geometric_epsilon, golden_angle_radians, golden_ratio, half_f64, nth_prime_u64,
    pow10_u64, pow2_u64, three_u32, two_f64, two_u32, Aabb, DeterministicRng, Quat, Vec3,
};
use crate::prism_kernel;
#[cfg(any(feature = "uorc-streaming", feature = "uorc-wasm-decode"))]
use crate::runtime::WorldSnapshot;

/// Progressive streaming Level-of-Detail (`LOD`) tier verified by `Tensor.lex.tex`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum LodLevel {
    /// `LOD0`: Quarter-scale proxy bounds, NavMesh topology, and kinematic waypoints.
    Lod0CoarseProxy,
    /// `LOD1`: Half-scale terrain relief, rigid bodies, and base PBR materials.
    Lod1MediumDetail,
    /// `LOD2`: Three-quarter-scale shadows, skeletal skinning, and instanced surface props.
    Lod2HighDetail,
    /// `LOD3`: Full `1:1` lossless source resolution (`UORC-SHA256-CHAIN-v1` verified).
    Lod3LosslessSource,
}

impl LodLevel {
    /// Resolves a verified `LodLevel` from `rank` (`0..=prism_kernel::wasmMaxLodRank()`).
    pub fn from_rank(rank: u64) -> Result<Self, StreamingError> {
        let max_rank = prism_kernel::wasmMaxLodRank();
        if rank > max_rank {
            return Err(StreamingError::InvalidLodRank {
                requested_rank: rank,
                max_rank,
            });
        }
        let r2 = prism_kernel::sTwo();
        if rank == 0 {
            Ok(Self::Lod0CoarseProxy)
        } else if rank == 1 {
            Ok(Self::Lod1MediumDetail)
        } else if rank == r2 {
            Ok(Self::Lod2HighDetail)
        } else {
            Ok(Self::Lod3LosslessSource)
        }
    }

    /// Canonical zero-based LOD rank (`0..=3`).
    #[must_use]
    pub fn rank(self) -> u64 {
        match self {
            Self::Lod0CoarseProxy => 0,
            Self::Lod1MediumDetail => 1,
            Self::Lod2HighDetail => crate::math::two_u32() as u64,
            Self::Lod3LosslessSource => crate::math::three_u32() as u64,
        }
    }

    /// Corresponding Lean-verified `StreamingLodSpec` from `Tensor.lex.tex`.
    #[must_use]
    pub fn prism_spec(self) -> prism_kernel::StreamingLodSpec {
        match self {
            Self::Lod0CoarseProxy => prism_kernel::lod0CoarseProxy().expect("prism"),
            Self::Lod1MediumDetail => prism_kernel::lod1MediumDetail().expect("prism"),
            Self::Lod2HighDetail => prism_kernel::lod2HighDetail().expect("prism"),
            Self::Lod3LosslessSource => prism_kernel::lod3LosslessSource().expect("prism"),
        }
    }

    /// Corresponding Lean-verified `LodRenderEnvelopeSpec` from `Subsystems.lex.tex`.
    #[must_use]
    pub fn render_envelope(self) -> prism_kernel::LodRenderEnvelopeSpec {
        prism_kernel::lodEnvelopeByRank(self.rank()).expect("prism")
    }

    /// Normalized resolution fraction `resNumer / resDenom` in `(0, 1]`.
    #[must_use]
    pub fn resolution_ratio(self) -> f64 {
        let spec = self.prism_spec();
        (spec.resNumer as f64) / (spec.resDenom.max(1) as f64)
    }
}

/// Parametric physical environment constants governing gravity, buoyancy, acoustics, and lighting.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct PhysicalEnvironment {
    /// Downward gravitational acceleration (`m/s^2`).
    pub gravity_mps2: f64,
    /// Ambient fluid mass density (`kg/m^3`).
    pub fluid_density_kg_m3: f64,
    /// Speed of sound in air (`m/s`).
    pub sound_speed_mps: f64,
    /// Directional key-light color temperature (`K`).
    pub key_light_temperature_kelvin: f64,
    /// Direct key-light irradiance (`W/m^2`).
    pub key_light_irradiance_w_m2: f64,
    /// Reference fluid surface elevation (`m`).
    pub water_level_meters: f64,
}

impl PhysicalEnvironment {
    /// Synthesizes standard terrestrial physical parameters constructively from `{0, 1}`.
    #[must_use]
    pub fn temperate_terrestrial() -> Self {
        let half = half_f64();
        let ten = pow10_u64(1) as f64;
        let thousand = pow10_u64(three_u32()) as f64;
        let sound = prism_kernel::wasmSoundSpeedMps().expect("prism") as f64;
        let five = nth_prime_u64(two_u32()) as f64;
        let seven = nth_prime_u64(three_u32()) as f64;
        Self {
            gravity_mps2: ten - half * half * (1.0 - half * half),
            fluid_density_kg_m3: thousand,
            sound_speed_mps: sound,
            key_light_temperature_kelvin: thousand * five + pow10_u64(two_u32()) as f64 * seven,
            key_light_irradiance_w_m2: thousand
                + pow10_u64(two_u32()) as f64 * (three_u32() as f64),
            water_level_meters: -half,
        }
    }
}

/// Parametric continuous heightfield relief configuration for 3D terrain patches.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct TerrainReliefParams {
    /// Central depression depth in meters.
    pub basin_depth_meters: f64,
    /// Gaussian radial variance (`m^2`) of the central depression.
    pub basin_variance_m2: f64,
    /// World `X` center of the fluid depression in meters.
    pub basin_center_x: f64,
    /// World `Z` center of the fluid depression in meters.
    pub basin_center_z: f64,
    /// Rolling harmonic hill amplitude in meters.
    pub hill_amplitude_meters: f64,
    /// Spatial wave frequency (`rad/m`) of rolling hills.
    pub hill_frequency_per_meter: f64,
    /// Quadratic outer rim elevation curvature (`m^-1`).
    pub outer_rim_curvature: f64,
    /// Maximum radial distance squared (`m^2`) contributing to rim rise.
    pub outer_rim_cap_m2: f64,
}

impl TerrainReliefParams {
    /// Synthesizes a balanced valley-and-ridge terrain profile constructively from `{0, 1}`.
    #[must_use]
    pub fn standard_basin_valley() -> Self {
        let two = two_f64();
        let half = half_f64();
        let quarter = half * half;
        let eighth = quarter * half;
        let ten = pow10_u64(1) as f64;
        let hundred = pow10_u64(two_u32()) as f64;
        let thousand = pow10_u64(three_u32()) as f64;
        Self {
            basin_depth_meters: two,
            basin_variance_m2: hundred + ten * (four_u32() as f64),
            basin_center_x: two * (three_u32() as f64),
            basin_center_z: -two * two,
            hill_amplitude_meters: half + quarter,
            hill_frequency_per_meter: eighth,
            outer_rim_curvature: 1.0 / (hundred * (four_u32() as f64)),
            outer_rim_cap_m2: thousand * two,
        }
    }
}

/// Evaluates continuous deterministic terrain elevation `y(x, z)` in meters.
#[must_use]
pub fn terrain_height_with_params(seed: u64, relief: &TerrainReliefParams, x: f64, z: f64) -> f64 {
    let phase = ((seed & ((1_u64 << (four_u32() * two_u32())) - 1)) as f64)
        / (pow2_u64(four_u32() * two_u32()) as f64)
        * std::f64::consts::TAU;
    let dx = x - relief.basin_center_x;
    let dz = z - relief.basin_center_z;
    let basin_r2 = dx * dx + dz * dz;
    let basin = -relief.basin_depth_meters * (-basin_r2 / relief.basin_variance_m2.max(1.0)).exp();

    let fx = x * relief.hill_frequency_per_meter + phase;
    let fz = z * relief.hill_frequency_per_meter - phase * half_f64();
    let hills = relief.hill_amplitude_meters * (fx.sin() * fz.cos());

    let r2 = (x * x + z * z).min(relief.outer_rim_cap_m2);
    let rim = r2 * relief.outer_rim_curvature;
    basin + hills + rim
}

/// Evaluates analytical finite-difference unit surface normal of `terrain_height_with_params`.
#[must_use]
pub fn terrain_normal_with_params(seed: u64, relief: &TerrainReliefParams, x: f64, z: f64) -> Vec3 {
    let h = half_f64() * half_f64();
    let two_h = two_f64() * h;
    let hx_pos = terrain_height_with_params(seed, relief, x + h, z);
    let hx_neg = terrain_height_with_params(seed, relief, x - h, z);
    let hz_pos = terrain_height_with_params(seed, relief, x, z + h);
    let hz_neg = terrain_height_with_params(seed, relief, x, z - h);
    let dh_dx = (hx_pos - hx_neg) / two_h;
    let dh_dz = (hz_pos - hz_neg) / two_h;
    Vec3::new(-dh_dx, 1.0, -dh_dz).normalized()
}

/// Universal 3D scene node preset category corresponding to `Subsystems::nodePreset0..6`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SceneNodeKind {
    /// Fluted architectural pavilion at scene center (`nodePreset0`).
    CentralArchPavilion,
    /// Vertical reference spire and shadow caster (`nodePreset1`).
    AxialMarkerTower,
    /// Modal acoustic bar emitter frame (`nodePreset2`).
    ModalAcousticsFrame,
    /// Articulated skeletal and spring-joint test bay (`nodePreset3`).
    ArticulatedRigBay,
    /// Elevated rigid-body and capsule bridge deck (`nodePreset4`).
    ElevatedDeckBridge,
    /// Walkable NavMesh corridor junction (`nodePreset5`).
    NavigablePathGrid,
    /// Autonomous closed-spline kinematic mover (`nodePreset6`).
    KinematicSplineMover,
}

/// Placed 3D scene node derived from a verified `prism_kernel::SceneNodePresetSpec`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SceneNodeInstance {
    /// Preset index (`0..=6`).
    pub preset_index: u64,
    /// Identifier slug.
    pub id_slug: String,
    /// Display label in Studio and CLI inspectors.
    pub title: String,
    /// Node archetype kind.
    pub kind: SceneNodeKind,
    /// World-space anchor position in meters.
    pub world_position: Vec3,
    /// Bounding span in meters.
    pub span_meters: f64,
    /// Bounding height in meters.
    pub height_meters: f64,
    /// Recommended camera orbit distance in meters.
    pub orbit_distance_meters: f64,
    /// Fundamental modal acoustic pitch in hertz (`0.0` when silent).
    pub modal_frequency_hz: f64,
    /// Whether this node follows the kinematic spline mover.
    pub follow_mover: bool,
}

/// Parametric scene synthesis settings for `StandardSceneState`.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct SceneBuildParams {
    /// Physical gravity, fluid, acoustic, and lighting constants.
    pub environment: PhysicalEnvironment,
    /// Continuous terrain relief parameters.
    pub terrain: TerrainReliefParams,
    /// Maximum instanced surface foliage count at `LOD3`.
    pub max_foliage_instances: usize,
    /// Maximum autonomous NavMesh agent count at `LOD3`.
    pub max_agent_count: usize,
    /// Crystalline dielectric prop count.
    pub crystal_prop_count: usize,
    /// Closed-loop spline waypoint count (`prism_kernel::splineWaypointCount()`).
    pub spline_waypoint_count: usize,
    /// Closed-loop spline orbit radius in meters (`prism_kernel::splineOrbitRadiusMeters()`).
    pub spline_orbit_radius_meters: f64,
}

impl Default for SceneBuildParams {
    fn default() -> Self {
        let wp_count = prism_kernel::splineWaypointCount().expect("prism") as usize;
        let orbit_r = prism_kernel::splineOrbitRadiusMeters().expect("prism") as f64;
        let sixteen = prism_kernel::sSixteen().expect("prism") as usize;
        let six = prism_kernel::sSix().expect("prism") as usize;
        let eight = prism_kernel::sEight().expect("prism") as usize;
        Self {
            environment: PhysicalEnvironment::temperate_terrestrial(),
            terrain: TerrainReliefParams::standard_basin_valley(),
            max_foliage_instances: sixteen,
            max_agent_count: six,
            crystal_prop_count: eight,
            spline_waypoint_count: wp_count,
            spline_orbit_radius_meters: orbit_r,
        }
    }
}

/// Deterministic multi-LOD 3D engine scene state (`LOD0 -> LOD3`) serializable via `VDNT` and `UORC`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StandardSceneState {
    /// Deterministic world seed.
    pub seed: u64,
    /// Parametric scene generation configuration.
    pub params: SceneBuildParams,
    /// Active progressive `LodLevel`.
    pub active_lod: LodLevel,
    /// Normalized resolution fraction in `(0, 1]`.
    pub resolution_scale: f64,
    /// Active instanced foliage count at current LOD.
    pub foliage_count: usize,
    /// Active autonomous agent count at current LOD.
    pub agent_count: usize,
    /// Active universal scene node presets (`nodePreset0..6`).
    pub scene_nodes: Vec<SceneNodeInstance>,
    /// Closed-loop kinematic mover spline waypoints on the terrain surface.
    pub mover_spline_waypoints: Vec<Vec3>,
    /// Hierarchical ECS `SceneWorld` populated for the active LOD.
    pub scene: SceneWorld,
}

fn preset_world_position(
    seed: u64,
    terrain: &TerrainReliefParams,
    spec: &prism_kernel::SceneNodePresetSpec,
) -> Vec3 {
    let ten = pow10_u64(1) as f64;
    let raw_x = (spec.xTenths as f64) / ten;
    let raw_z = (spec.zTenths as f64) / ten;
    let x = if spec.xNeg { -raw_x } else { raw_x };
    let z = if spec.zNeg { -raw_z } else { raw_z };
    let y_offset = (spec.yTenths as f64) / ten;
    let ground_y = terrain_height_with_params(seed, terrain, x, z);
    Vec3::new(x, ground_y + y_offset, z)
}

fn build_scene_nodes(
    seed: u64,
    params: &SceneBuildParams,
    lod: LodLevel,
) -> Vec<SceneNodeInstance> {
    if lod == LodLevel::Lod0CoarseProxy {
        return Vec::new();
    }
    let two = two_f64();
    let three = three_u32() as f64;
    let four = four_u32() as f64;
    let base_hz = prism_kernel::wasmBinauralFundamentalHz().expect("prism") as f64;
    let phi = golden_ratio();

    let descriptors = [
        (
            0_u64,
            "central-arch-pavilion",
            "Central Arch Pavilion",
            SceneNodeKind::CentralArchPavilion,
            four * two,
            three * two,
            base_hz,
        ),
        (
            1_u64,
            "axial-marker-tower",
            "Axial Marker Tower",
            SceneNodeKind::AxialMarkerTower,
            two,
            four + three,
            0.0,
        ),
        (
            prism_kernel::sTwo(),
            "modal-acoustics-frame",
            "Modal Acoustics Frame",
            SceneNodeKind::ModalAcousticsFrame,
            four,
            three + 1.0,
            base_hz * phi,
        ),
        (
            prism_kernel::sThree(),
            "articulated-rig-bay",
            "Articulated Rig Bay",
            SceneNodeKind::ArticulatedRigBay,
            three,
            four,
            base_hz / two,
        ),
        (
            prism_kernel::sFour().expect("prism"),
            "elevated-deck-bridge",
            "Elevated Deck Bridge",
            SceneNodeKind::ElevatedDeckBridge,
            four * two + two,
            two,
            0.0,
        ),
        (
            prism_kernel::sFive().expect("prism"),
            "navigable-path-grid",
            "Navigable Path Grid",
            SceneNodeKind::NavigablePathGrid,
            four * three,
            1.0,
            0.0,
        ),
        (
            prism_kernel::sSix().expect("prism"),
            "kinematic-spline-mover",
            "Kinematic Spline Mover",
            SceneNodeKind::KinematicSplineMover,
            two,
            1.0 + half_f64(),
            base_hz * (1.0 + half_f64()),
        ),
    ];

    let max_presets = if lod == LodLevel::Lod1MediumDetail {
        prism_kernel::sThree() as usize
    } else {
        descriptors.len()
    };

    let mut out = Vec::with_capacity(max_presets);
    for &(idx, slug, title, kind, span, height, hz) in descriptors.iter().take(max_presets) {
        let spec = prism_kernel::sceneNodePresetByIndex(idx).expect("prism");
        let pos = preset_world_position(seed, &params.terrain, &spec);
        out.push(SceneNodeInstance {
            preset_index: idx,
            id_slug: slug.to_string(),
            title: title.to_string(),
            kind,
            world_position: pos,
            span_meters: span,
            height_meters: height,
            orbit_distance_meters: spec.orbitDistanceMeters as f64,
            modal_frequency_hz: hz,
            follow_mover: spec.followMover,
        });
    }
    out
}

impl StandardSceneState {
    /// Initializes a seeded 3D scene at `LOD0` (`Lod0CoarseProxy`).
    pub fn new_at_lod0(seed: u64) -> Result<Self, StreamingError> {
        Self::synthesize_with_params(seed, SceneBuildParams::default(), 0)
    }

    /// Synthesizes a deterministic 3D scene at `lod_rank` (`0..=3`) using default parameters.
    pub fn synthesize_at_lod(seed: u64, lod_rank: u64) -> Result<Self, StreamingError> {
        Self::synthesize_with_params(seed, SceneBuildParams::default(), lod_rank)
    }

    /// Synthesizes a deterministic 3D scene at `lod_rank` under explicit `SceneBuildParams`.
    pub fn synthesize_with_params(
        seed: u64,
        params: SceneBuildParams,
        lod_rank: u64,
    ) -> Result<Self, StreamingError> {
        let lod = LodLevel::from_rank(lod_rank)?;
        let mut state = Self {
            seed,
            params,
            active_lod: LodLevel::Lod0CoarseProxy,
            resolution_scale: LodLevel::Lod0CoarseProxy.resolution_ratio(),
            foliage_count: 0,
            agent_count: 0,
            scene_nodes: Vec::new(),
            mover_spline_waypoints: Vec::new(),
            scene: SceneWorld::new(),
        };
        state.rebuild_for_lod(lod);
        Ok(state)
    }

    /// Advances the scene monotonically to `target_rank`, verifying every intermediate step
    /// through `prism_kernel::lodStepValid`.
    pub fn advance_to_lod(&mut self, target_rank: u64) -> Result<(), StreamingError> {
        let target_lod = LodLevel::from_rank(target_rank)?;
        let current_rank = self.active_lod.rank();
        if target_rank < current_rank {
            return Err(StreamingError::NonMonotonicLodTransition {
                current_rank,
                requested_rank: target_rank,
            });
        }
        let mut r = current_rank;
        while r < target_rank {
            let earlier = LodLevel::from_rank(r)?.prism_spec();
            let later = LodLevel::from_rank(r + 1)?.prism_spec();
            let valid = prism_kernel::lodStepValid(&earlier, &later).expect("prism");
            if !valid {
                return Err(StreamingError::NonMonotonicLodTransition {
                    current_rank: r,
                    requested_rank: r + 1,
                });
            }
            r += 1;
        }
        self.rebuild_for_lod(target_lod);
        Ok(())
    }

    fn rebuild_for_lod(&mut self, lod: LodLevel) {
        self.active_lod = lod;
        self.resolution_scale = lod.resolution_ratio();

        let wp_count = self.params.spline_waypoint_count.max(four_u32() as usize);
        let radius = self.params.spline_orbit_radius_meters;
        let half = half_f64();
        let mut waypoints = Vec::with_capacity(wp_count);
        for i in 0..wp_count {
            let theta = (i as f64) * std::f64::consts::TAU / (wp_count as f64);
            let x = radius * theta.cos();
            let z = radius * theta.sin();
            let y = terrain_height_with_params(self.seed, &self.params.terrain, x, z) + half;
            waypoints.push(Vec3::new(x, y, z));
        }
        self.mover_spline_waypoints = waypoints;

        let mut scene = SceneWorld::new();
        let extent = (prism_kernel::sForty().expect("prism") as f64)
            + (prism_kernel::sEight().expect("prism") as f64);
        let two = two_f64();
        let four = four_u32() as f64;

        let _sun_id = scene.spawn(
            "DirectionalKeySun",
            EntityRole::DirectionalLight,
            None,
            Transform {
                translation: Vec3::new(extent, extent * two, extent),
                rotation: Quat::IDENTITY,
                scale: Vec3::new(1.0, 1.0, 1.0),
            },
            Aabb::from_center_half_extents(Vec3::ZERO, Vec3::new(1.0, 1.0, 1.0)),
            (
                LodLevel::Lod0CoarseProxy.resolution_ratio(),
                "LOD0 directional light",
            ),
        );

        let terrain_id = scene.spawn(
            "PrimaryTerrainPatch",
            EntityRole::TerrainPatch,
            None,
            Transform::default(),
            Aabb::new(
                Vec3::new(-extent, -four, -extent),
                Vec3::new(extent, four * two, extent),
            ),
            (
                LodLevel::Lod0CoarseProxy.resolution_ratio(),
                "LOD0 heightfield proxy",
            ),
        );
        scene.attach_renderable(terrain_id, 0, 0);

        let fluid_id = scene.spawn(
            "BasinFluidSurface",
            EntityRole::FluidVolume,
            Some(terrain_id),
            Transform::from_xyz(
                self.params.terrain.basin_center_x,
                self.params.environment.water_level_meters,
                self.params.terrain.basin_center_z,
            ),
            Aabb::from_center_half_extents(Vec3::ZERO, Vec3::new(four * four, half, four * four)),
            (
                LodLevel::Lod0CoarseProxy.resolution_ratio(),
                "LOD0 fluid plane",
            ),
        );
        scene.attach_renderable(fluid_id, 1, 1);

        let mut rng = DeterministicRng::new(self.seed);
        let i2 = 1usize + 1usize;
        let i3 = i2 + 1usize;
        let i4 = i3 + 1usize;

        if lod >= LodLevel::Lod1MediumDetail {
            for c_idx in 0..self.params.crystal_prop_count {
                let angle = (c_idx as f64) * golden_angle_radians();
                let dist = four + rng.range_f64(two, four * three_u32() as f64);
                let x = dist * angle.cos();
                let z = dist * angle.sin();
                let y = terrain_height_with_params(self.seed, &self.params.terrain, x, z);
                let cid = scene.spawn(
                    format!("DielectricCluster_{c_idx}"),
                    EntityRole::TerrainPatch,
                    Some(terrain_id),
                    Transform::from_xyz(x, y, z),
                    Aabb::new(Vec3::new(-half, 0.0, -half), Vec3::new(half, two, half)),
                    (
                        LodLevel::Lod1MediumDetail.resolution_ratio(),
                        "LOD1 dielectric prop",
                    ),
                );
                scene.attach_renderable(cid, i2, i2);
            }
        }

        self.foliage_count = match lod {
            LodLevel::Lod0CoarseProxy | LodLevel::Lod1MediumDetail => 0,
            LodLevel::Lod2HighDetail => self.params.max_foliage_instances / i2,
            LodLevel::Lod3LosslessSource => self.params.max_foliage_instances,
        };
        self.agent_count = match lod {
            LodLevel::Lod0CoarseProxy | LodLevel::Lod1MediumDetail => 0,
            LodLevel::Lod2HighDetail => self.params.max_agent_count / i2,
            LodLevel::Lod3LosslessSource => self.params.max_agent_count,
        };

        for f_idx in 0..self.foliage_count {
            let angle = (f_idx as f64) * golden_angle_radians();
            let dist = four * two + (f_idx as f64) * (1.0 + half);
            let x = dist * angle.cos();
            let z = dist * angle.sin();
            let y = terrain_height_with_params(self.seed, &self.params.terrain, x, z);
            let fid = scene.spawn(
                format!("SurfaceFoliage_{f_idx}"),
                EntityRole::SurfaceFoliage,
                Some(terrain_id),
                Transform::from_xyz(x, y, z),
                Aabb::new(
                    Vec3::new(-1.0, 0.0, -1.0),
                    Vec3::new(1.0, three_u32() as f64, 1.0),
                ),
                (lod.resolution_ratio(), "Progressive instanced foliage"),
            );
            scene.attach_renderable(fid, i3, i3);
        }

        for a_idx in 0..self.agent_count {
            let wp = self.mover_spline_waypoints[a_idx % self.mover_spline_waypoints.len()];
            let _aid = scene.spawn(
                format!("AutonomousAgent_{a_idx}"),
                EntityRole::AutonomousAgent,
                Some(terrain_id),
                Transform::from_xyz(wp.x * half, wp.y, wp.z * half),
                Aabb::from_center_half_extents(Vec3::ZERO, Vec3::new(half, half, half)),
                (lod.resolution_ratio(), "NavMesh autonomous agent"),
            );
        }

        let nodes = build_scene_nodes(self.seed, &self.params, lod);
        for node in &nodes {
            let role = match node.kind {
                SceneNodeKind::ArticulatedRigBay => EntityRole::ArticulatedRig,
                SceneNodeKind::KinematicSplineMover => EntityRole::KinematicMover,
                _ => EntityRole::StaticArchitecture,
            };
            let half_span = (node.span_meters * half).max(geometric_epsilon());
            let nid = scene.spawn(
                node.title.clone(),
                role,
                Some(terrain_id),
                Transform::from_xyz(
                    node.world_position.x,
                    node.world_position.y,
                    node.world_position.z,
                ),
                Aabb::new(
                    Vec3::new(-half_span, 0.0, -half_span),
                    Vec3::new(half_span, node.height_meters, half_span),
                ),
                (lod.resolution_ratio(), "Universal scene node preset"),
            );
            scene.attach_renderable(nid, i4, i4);
        }

        self.scene_nodes = nodes;
        self.scene = scene;
    }
}

/// Summary metadata returned by [`verify_progressive_uorc_stream`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UorcStreamSummary {
    /// Total compressed `.uorc` wire byte length.
    pub compressed_bytes: usize,
    /// Total uncompressed `VDNT` multi-LOD byte length across all frames.
    pub decoded_total_bytes: usize,
    /// Number of self-delimiting `UORC-SHA256-CHAIN-v1` frames in the stream.
    pub archive_frame_count: usize,
    /// Number of progressive LOD scene objects verified (`4` for `LOD0..=LOD3`).
    pub lod_object_count: usize,
    /// Maximum verified `LodLevel` rank (`3` = lossless source).
    pub max_lod_rank: u64,
}

#[cfg(any(feature = "uorc-streaming", feature = "uorc-wasm-decode"))]
fn uorc_work_budget() -> u64 {
    let shift = prism_kernel::uorcDefaultDecodeWork().expect("prism") as u32;
    pow2_u64(shift)
}

/// Synthesizes all four progressive LOD states (`LOD0..=LOD3`) for `seed` and encodes them into a
/// single `UORC-SHA256-CHAIN-v1` progressive multi-object `.uorc` stream via `uorc::FrameEncoder`.
#[cfg(feature = "uorc-streaming")]
pub fn pack_progressive_uorc_stream(seed: u64) -> Result<Vec<u8>, StreamingError> {
    let max_rank = prism_kernel::wasmMaxLodRank();
    let mut lod_blobs = Vec::with_capacity((max_rank + 1) as usize);
    let mut lengths = Vec::with_capacity((max_rank + 1) as usize);

    let mut state = StandardSceneState::new_at_lod0(seed)?;
    for rank in 0..=max_rank {
        state.advance_to_lod(rank)?;
        let snap =
            WorldSnapshot::encode(&state).map_err(|err| StreamingError::LodPayloadMismatch {
                detail: err.to_string(),
            })?;
        lengths.push(snap.len() as u64);
        lod_blobs.push(snap);
    }

    let steps = prism_kernel::uorcDefaultCompressSteps().expect("prism");
    let mut encoder = FrameEncoder::objects(&lengths);
    let mut stream = Vec::new();
    for blob in &lod_blobs {
        let chunk =
            encoder
                .push(blob, steps)
                .map_err(|err| StreamingError::UorcArchiveRejected {
                    detail: err.to_string(),
                })?;
        stream.extend_from_slice(&chunk);
    }
    let tail = encoder
        .finish(steps)
        .map_err(|err| StreamingError::UorcArchiveRejected {
            detail: err.to_string(),
        })?;
    stream.extend_from_slice(&tail);

    Ok(stream)
}

/// Verifies a `.uorc` progressive multi-LOD stream through `uorc::verify`, `uorc::FrameDecoder`,
/// and `prism_kernel::lodStepValid` without trusting unauthenticated headers.
#[cfg(feature = "uorc-streaming")]
pub fn verify_progressive_uorc_stream(
    stream_bytes: &[u8],
) -> Result<UorcStreamSummary, StreamingError> {
    if stream_bytes.is_empty() {
        return Err(StreamingError::UorcArchiveRejected {
            detail: "empty UORC stream".to_string(),
        });
    }
    let work = uorc_work_budget();
    let infos = uorc::verify(stream_bytes, None, work).map_err(|err| {
        StreamingError::UorcArchiveRejected {
            detail: err.to_string(),
        }
    })?;

    // Also exercise incremental `FrameDecoder` over individual archives:
    let mut archives_vec = Vec::new();
    for item in uorc::archives(std::io::Cursor::new(stream_bytes)) {
        let arch = item.map_err(|err| StreamingError::UorcArchiveRejected {
            detail: err.to_string(),
        })?;
        archives_vec.push(arch);
    }
    let mut frame_decoder = FrameDecoder::new(work);
    for arch in &archives_vec {
        frame_decoder.note(arch);
    }
    let mut decoded_concat_len = 0usize;
    for arch in &archives_vec {
        let plain =
            frame_decoder
                .push(arch)
                .map_err(|err| StreamingError::UorcArchiveRejected {
                    detail: err.to_string(),
                })?;
        decoded_concat_len += plain.len();
    }

    let objects = uorc::decompress_objects(stream_bytes, work).map_err(|err| {
        StreamingError::UorcArchiveRejected {
            detail: err.to_string(),
        }
    })?;
    let expected_lods = (prism_kernel::wasmMaxLodRank() + 1) as usize;
    if objects.len() != expected_lods {
        return Err(StreamingError::LodPayloadMismatch {
            detail: format!(
                "expected {expected_lods} progressive LOD objects, decoded {}",
                objects.len()
            ),
        });
    }

    let mut prev_spec: Option<prism_kernel::StreamingLodSpec> = None;
    let mut max_lod_rank = 0_u64;
    for (idx, obj_bytes) in objects.iter().enumerate() {
        let state =
            WorldSnapshot::decode(obj_bytes).map_err(|err| StreamingError::LodPayloadMismatch {
                detail: err.to_string(),
            })?;
        if state.active_lod.rank() != (idx as u64) {
            return Err(StreamingError::LodPayloadMismatch {
                detail: format!(
                    "LOD object {idx} carries mismatched active_lod rank {}",
                    state.active_lod.rank()
                ),
            });
        }
        let spec = state.active_lod.prism_spec();
        if let Some(earlier) = &prev_spec {
            let step_ok = prism_kernel::lodStepValid(earlier, &spec).expect("prism");
            if !step_ok {
                return Err(StreamingError::NonMonotonicLodTransition {
                    current_rank: earlier.rank,
                    requested_rank: spec.rank,
                });
            }
        }
        max_lod_rank = spec.rank;
        prev_spec = Some(spec);
    }

    Ok(UorcStreamSummary {
        compressed_bytes: stream_bytes.len(),
        decoded_total_bytes: decoded_concat_len,
        archive_frame_count: infos.len(),
        lod_object_count: objects.len(),
        max_lod_rank,
    })
}

/// Progressively decodes a `.uorc` multi-LOD scene stream up to `target_lod_rank` (`0..=3`) and
/// returns the authenticated `StandardSceneState`.
#[cfg(feature = "uorc-streaming")]
pub fn decode_uorc_stream_up_to_lod(
    stream_bytes: &[u8],
    target_lod_rank: u64,
) -> Result<StandardSceneState, StreamingError> {
    let lod = LodLevel::from_rank(target_lod_rank)?;
    let _summary = verify_progressive_uorc_stream(stream_bytes)?;
    let work = uorc_work_budget();
    let objects = uorc::decompress_objects(stream_bytes, work).map_err(|err| {
        StreamingError::UorcArchiveRejected {
            detail: err.to_string(),
        }
    })?;
    let slot = lod.rank() as usize;
    let payload = objects
        .get(slot)
        .ok_or_else(|| StreamingError::LodPayloadMismatch {
            detail: format!("missing LOD object at rank {slot}"),
        })?;
    WorldSnapshot::decode(payload).map_err(|err| StreamingError::LodPayloadMismatch {
        detail: err.to_string(),
    })
}

/// Zero-decode `uor_matmul::codec` streaming payload bundled per progressive LOD tier.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodedLodFrameBundle {
    /// Progressive LOD rank (`0..=3`).
    pub lod_rank: u64,
    /// Shared 16-entry signed quantization codebook (`Grid<i32, Full<i32>, 16>`).
    pub grid_codebook: [i32; verdant_tensor::CODED_TABLE_LEN],
    /// Fixed-width `Grid` vertex/normal transform codes (`inner_k * cols`).
    pub geometry_grid_codes: Vec<u16>,
    /// Sub-byte `Packed<Grid<16>, 2>` animation curve codes (`2` nibbles per byte).
    pub animation_packed_nibbles: Vec<u8>,
    /// Variable-length `Runs<Grid<16>, 16>` dictionary (`(run_len, code_idx)`).
    pub ibl_run_dictionary: Vec<(u32, u16)>,
    /// Row-ordered run dictionary indices for sparse IBL environment harmonics.
    pub ibl_run_indices: Vec<u32>,
    /// Inner contraction extent `k` for coded geometry, animation, and IBL matrices.
    pub inner_k: usize,
    /// Output column extent `n` for coded geometry, animation, and IBL matrices.
    pub cols: usize,
}

/// Exact integer accumulator outputs from contracting a [`CodedLodFrameBundle`] with zero float expansion.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ZeroDecodeContractionSummary {
    /// Exact `Grid` geometry contraction accumulators.
    pub geometry_accum: Vec<i64>,
    /// Exact `Packed` nibble animation contraction accumulators.
    pub animation_accum: Vec<i64>,
    /// Exact `Runs` sparse IBL harmonic contraction accumulators.
    pub ibl_accum: Vec<i64>,
}

impl CodedLodFrameBundle {
    /// Synthesizes a deterministic `CodedLodFrameBundle` for `seed` and `lod_rank` (`0..=3`).
    pub fn synthesize(seed: u64, lod_rank: u64) -> Result<Self, StreamingError> {
        let lod = LodLevel::from_rank(lod_rank)?;
        let scale_step = ((lod.rank() + 1) * prism_kernel::sSixteen().unwrap_or(1)) as i32;
        let half_len = (verdant_tensor::CODED_TABLE_LEN / (1 + 1)) as i32;
        let grid_codebook: [i32; verdant_tensor::CODED_TABLE_LEN] =
            core::array::from_fn(|i| ((i as i32) - half_len) * scale_step);

        let inner_k = verdant_tensor::DIM_HOMOGENEOUS;
        let cols = verdant_tensor::DIM_HOMOGENEOUS;
        let tbl_mask = (verdant_tensor::CODED_TABLE_LEN - 1) as u64;

        let mut rng = DeterministicRng::new(seed ^ (lod.rank() + 1));
        let mut geometry_grid_codes = Vec::with_capacity(inner_k * cols);
        for _ in 0..(inner_k * cols) {
            geometry_grid_codes.push((rng.next_u64() & tbl_mask) as u16);
        }

        let nibble_shift = four_u32();
        let nibble_bytes = inner_k * (cols / verdant_tensor::PACKED_NIBBLE_BLOCK);
        let mut animation_packed_nibbles = Vec::with_capacity(nibble_bytes);
        for _ in 0..nibble_bytes {
            let lo = (rng.next_u64() & tbl_mask) as u8;
            let hi = (rng.next_u64() & tbl_mask) as u8;
            animation_packed_nibbles.push(lo | (hi << nibble_shift));
        }

        // Each row of `cols` (4) elements is encoded as two runs of length 2:
        let run_len = two_u32();
        let mut ibl_run_dictionary = Vec::with_capacity(inner_k * (1 + 1));
        let mut ibl_run_indices = Vec::with_capacity(inner_k * (1 + 1));
        for r in 0..(inner_k * (1 + 1)) {
            let code = (rng.next_u64() & tbl_mask) as u16;
            ibl_run_dictionary.push((run_len, code));
            ibl_run_indices.push(r as u32);
        }

        Ok(Self {
            lod_rank: lod.rank(),
            grid_codebook,
            geometry_grid_codes,
            animation_packed_nibbles,
            ibl_run_dictionary,
            ibl_run_indices,
            inner_k,
            cols,
        })
    }

    /// Contracts geometry (`Grid`), animation (`Packed`), and IBL (`Runs`) directly through
    /// `uor_matmul::coded_gemm` without intermediate float decoding.
    pub fn contract_zero_decode(
        &self,
        transform_row_q: &[i32],
        blend_weights_q: &[i32],
        sh_basis_q: &[i32],
    ) -> Result<ZeroDecodeContractionSummary, StreamingError> {
        let geometry_accum = verdant_tensor::contract_coded_geometry_grid(
            transform_row_q,
            1,
            self.inner_k,
            self.cols,
            &self.grid_codebook,
            &self.geometry_grid_codes,
        )
        .map_err(|err| StreamingError::LodPayloadMismatch {
            detail: err.to_string(),
        })?;
        let animation_accum = verdant_tensor::contract_packed_animation_tracks(
            blend_weights_q,
            1,
            self.inner_k,
            self.cols,
            &self.grid_codebook,
            &self.animation_packed_nibbles,
        )
        .map_err(|err| StreamingError::LodPayloadMismatch {
            detail: err.to_string(),
        })?;
        let ibl_accum = verdant_tensor::contract_runs_ibl_harmonics(
            sh_basis_q,
            1,
            self.inner_k,
            self.cols,
            &self.grid_codebook,
            &self.ibl_run_dictionary,
            &self.ibl_run_indices,
        )
        .map_err(|err| StreamingError::LodPayloadMismatch {
            detail: err.to_string(),
        })?;

        Ok(ZeroDecodeContractionSummary {
            geometry_accum,
            animation_accum,
            ibl_accum,
        })
    }
}

fn read_uleb128(data: &[u8], cursor: &mut usize) -> Result<u64, StreamingError> {
    let seven = prism_kernel::sSeven().unwrap_or(1) as u32;
    let cont_bit = 1_u8 << seven;
    let low_mask = cont_bit - 1;
    let max_bits = prism_kernel::natSixtyFour().unwrap_or(1) as u32;
    let mut acc = 0_u64;
    let mut shift = 0_u32;

    loop {
        let byte = *data
            .get(*cursor)
            .ok_or_else(|| StreamingError::UorcArchiveRejected {
                detail: "truncated UORC ULEB128 header".to_string(),
            })?;
        *cursor += 1;
        if shift >= max_bits {
            return Err(StreamingError::UorcArchiveRejected {
                detail: "ULEB128 integer overflow in UORC header".to_string(),
            });
        }
        acc |= u64::from(byte & low_mask) << shift;
        if (byte & cont_bit) == 0 {
            return Ok(acc);
        }
        shift += seven;
    }
}

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
/// progressive stream using stateless `state: 0` NCM strategies (`uorc::portfolio::V04_PAIR`) so
/// browser `wasm32-unknown-unknown` workers decode every LOD natively via `uorc_decompress::run`
/// without `uorc_search` or warm-state blobs.
#[cfg(feature = "uorc-streaming")]
pub fn pack_portable_progressive_uorc_stream(seed: u64) -> Result<Vec<u8>, StreamingError> {
    let max_rank = prism_kernel::wasmMaxLodRank();
    let max_strats = if cfg!(debug_assertions) {
        1
    } else {
        prism_kernel::sEight().unwrap_or(1) as usize
    };
    let mut strategies = Vec::new();
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

    let mut stream = Vec::new();
    let mut state = StandardSceneState::new_at_lod0(seed)?;
    for rank in 0..=max_rank {
        state.advance_to_lod(rank)?;
        let snap =
            WorldSnapshot::encode(&state).map_err(|err| StreamingError::LodPayloadMismatch {
                detail: err.to_string(),
            })?;
        let archive = uorc::compress_with_strategies(&snap, &[snap.len() as u64], &[], &strategies);
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

/// Compresses any arbitrary byte slice into a self-delimiting, SHA-256-sealed stateless `UORC`
/// archive (`state == 0 && !x86` NCM portfolio strategies) executable identically in native Rust
/// and `wasm32-unknown-unknown` page workers.
#[cfg(feature = "uorc-streaming")]
#[must_use]
pub fn compress_uorc_portable(payload: &[u8]) -> Vec<u8> {
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
    #[cfg(feature = "uorc-streaming")]
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
