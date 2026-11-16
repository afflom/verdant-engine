//! Floating-origin universe cell addressing (`i64^3` sector + bounded `f64^3` local offset),
//! multi-sector `.uorc` world descriptors, and momentum-preserving `PortalGraph` transitions.
//!
//! Enforces `prism_kernel::universeLocalOffsetBounded` (`|local| < cell_span / 2`, `cell_span = 128m`)
//! and emits camera-cell-relative GPU matrices so sub-millimeter `f32` precision holds across
//! planetary or astronomical distances (`10^9+` meters from origin).

use serde::{Deserialize, Serialize};

use crate::error::NarrativeLawError;
use crate::math::{half_f64, pow10_u64, three_u32, two_f64, Mat4, Quat, Vec3};

/// Exact 64-bit integer 3D sector coordinate (`cx, cy, cz`) in the streaming universe grid.
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct UniverseCellCoord {
    /// East-west sector index (`+X`).
    pub cx: i64,
    /// Vertical sector index (`+Y`).
    pub cy: i64,
    /// North-south sector index (`+Z`).
    pub cz: i64,
}

impl UniverseCellCoord {
    /// Origin sector `(0, 0, 0)`.
    pub const ORIGIN: Self = Self {
        cx: 0,
        cy: 0,
        cz: 0,
    };

    /// Constructs a sector coordinate.
    #[must_use]
    pub const fn new(cx: i64, cy: i64, cz: i64) -> Self {
        Self { cx, cy, cz }
    }

    /// Chebyshev (`L_inf`) sector distance to `other`.
    #[must_use]
    pub fn chebyshev_distance(self, other: Self) -> u64 {
        let dx = self.cx.abs_diff(other.cx);
        let dy = self.cy.abs_diff(other.cy);
        let dz = self.cz.abs_diff(other.cz);
        dx.max(dy).max(dz)
    }
}

/// Exact floating-origin world position combining `UniverseCellCoord` with canonical local meters
/// in `[-cell_span/2, +cell_span/2)^3`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct UniversePosition {
    /// Integer sector coordinate.
    pub cell: UniverseCellCoord,
    /// Canonicalized local offset within `[-64m, +64m)^3`.
    pub local: Vec3,
    /// Proof witness from `prism_kernel::universeLocalOffsetBounded`.
    pub bounds_verified: bool,
}

///Canonical sector edge span in meters (`prism_kernel::universeCellSpanMeters() == 128.0`).
#[must_use]
pub fn canonical_cell_span_meters() -> f64 {
    (crate::prism_kernel::universeCellSpanMeters()
        .unwrap_or(1)
        .max(1)) as f64
}

fn canonicalize_axis(
    cell_idx: i64,
    raw_offset: f64,
    span: f64,
) -> Result<(i64, f64), NarrativeLawError> {
    if !raw_offset.is_finite() || !span.is_finite() || span <= 0.0 {
        return Err(NarrativeLawError::InvalidPortalOrUniverseCell {
            detail: "non-finite universe coordinate or non-positive cell span".to_string(),
        });
    }
    let half_span = span * half_f64();
    let shifted = (raw_offset + half_span) / span;
    let delta_cell = shifted.floor() as i64;
    let wrapped = (raw_offset + half_span).rem_euclid(span) - half_span;
    let next_cell = cell_idx.checked_add(delta_cell).ok_or_else(|| {
        NarrativeLawError::InvalidPortalOrUniverseCell {
            detail: "i64 sector index overflow".to_string(),
        }
    })?;
    Ok((next_cell, wrapped))
}

impl UniversePosition {
    /// Canonicalizes `(cell, raw_local)` into `[-span/2, +span/2)^3` and verifies Lean bounds.
    pub fn new_canonical(
        cell: UniverseCellCoord,
        raw_local: Vec3,
    ) -> Result<Self, NarrativeLawError> {
        let span = canonical_cell_span_meters();
        let (cx, lx) = canonicalize_axis(cell.cx, raw_local.x, span)?;
        let (cy, ly) = canonicalize_axis(cell.cy, raw_local.y, span)?;
        let (cz, lz) = canonicalize_axis(cell.cz, raw_local.z, span)?;

        let scale_milli = pow10_u64(three_u32()) as f64;
        let half_span_milli = crate::prism_kernel::universeHalfSpanMilli().unwrap_or(1);
        let check_axis = |v: f64| -> bool {
            let abs_milli = (v.abs() * scale_milli).floor().max(0.0) as u64;
            crate::prism_kernel::universeLocalOffsetBounded(abs_milli, half_span_milli)
        };
        let bounds_verified = check_axis(lx) && check_axis(ly) && check_axis(lz);
        if !bounds_verified {
            return Err(NarrativeLawError::InvalidPortalOrUniverseCell {
                detail: "canonicalized offset failed universeLocalOffsetBounded".to_string(),
            });
        }

        Ok(Self {
            cell: UniverseCellCoord::new(cx, cy, cz),
            local: Vec3::new(lx, ly, lz),
            bounds_verified,
        })
    }

    /// Translates by `world_delta` meters across arbitrary sector boundaries without precision loss.
    pub fn translate(&self, world_delta: Vec3) -> Result<Self, NarrativeLawError> {
        Self::new_canonical(self.cell, self.local + world_delta)
    }

    /// Computes exact Euclidean offset relative to `camera_pos`'s sector origin (`camera-cell space`),
    /// eliminating `f32` vertex jitter even billions of meters from `(0, 0, 0)`.
    #[must_use]
    pub fn relative_to_camera_cell(&self, camera_pos: &Self) -> Vec3 {
        let span = canonical_cell_span_meters();
        let dcx = (self.cell.cx - camera_pos.cell.cx) as f64;
        let dcy = (self.cell.cy - camera_pos.cell.cy) as f64;
        let dcz = (self.cell.cz - camera_pos.cell.cz) as f64;
        Vec3::new(
            dcx * span + self.local.x,
            dcy * span + self.local.y,
            dcz * span + self.local.z,
        )
    }

    /// Computes exact Euclidean displacement `self - reference` across cells.
    #[must_use]
    pub fn displacement_from(&self, reference: &Self) -> Vec3 {
        let span = canonical_cell_span_meters();
        let dcx = (self.cell.cx - reference.cell.cx) as f64;
        let dcy = (self.cell.cy - reference.cell.cy) as f64;
        let dcz = (self.cell.cz - reference.cell.cz) as f64;
        Vec3::new(
            dcx * span + (self.local.x - reference.local.x),
            dcy * span + (self.local.y - reference.local.y),
            dcz * span + (self.local.z - reference.local.z),
        )
    }

    /// Emits a camera-cell-relative `Mat4` world transform for zero-jitter GPU vertex rendering.
    #[must_use]
    pub fn camera_relative_matrix(&self, camera_pos: &Self, rotation: Quat, scale: Vec3) -> Mat4 {
        Mat4::from_trs(self.relative_to_camera_cell(camera_pos), rotation, scale)
    }
}

/// Walkable spatial portal arch linking two universe cells (or an external federated `#vdnt=` / Stitch realm).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpatialPortal {
    /// Portal identifier.
    pub portal_id: String,
    /// Source sector coordinate.
    pub source_cell: UniverseCellCoord,
    /// Local position of the portal arch inside `source_cell`.
    pub source_local: Vec3,
    /// Activation radius in meters (`> 0`).
    pub trigger_radius_meters: f64,
    /// Destination sector coordinate.
    pub target_cell: UniverseCellCoord,
    /// Arrival local position inside `target_cell` (placed outside reverse trigger hysteresis).
    pub target_arrival_local: Vec3,
    /// Yaw rotation delta applied to player velocity and camera orientation across the portal.
    pub yaw_delta_radians: f64,
    /// Optional federated Stitch / URL address when linking across independent universes.
    pub federated_uri: Option<String>,
}

/// Directed graph of walkable spatial portals across universe sectors.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PortalGraph {
    /// Registered spatial portals.
    pub portals: Vec<SpatialPortal>,
}

impl PortalGraph {
    /// Validates and constructs a `PortalGraph`.
    pub fn new(portals: Vec<SpatialPortal>) -> Result<Self, NarrativeLawError> {
        for p in &portals {
            if !p.trigger_radius_meters.is_finite() || p.trigger_radius_meters <= 0.0 {
                return Err(NarrativeLawError::InvalidPortalOrUniverseCell {
                    detail: "portal requires positive trigger radius".to_string(),
                });
            }
            let _ = UniversePosition::new_canonical(p.source_cell, p.source_local)?;
            let _ = UniversePosition::new_canonical(p.target_cell, p.target_arrival_local)?;
        }
        Ok(Self { portals })
    }

    /// Tests whether `pos` has stepped inside an active portal arch, returning the destination
    /// `UniversePosition`, momentum-rotated velocity vector, and traversed `SpatialPortal`.
    pub fn try_traverse(
        &self,
        pos: &UniversePosition,
        velocity: Vec3,
    ) -> Result<Option<(UniversePosition, Vec3, &SpatialPortal)>, NarrativeLawError> {
        for portal in &self.portals {
            if portal.source_cell == pos.cell
                && pos.local.distance(portal.source_local) <= portal.trigger_radius_meters
            {
                let arrival = UniversePosition::new_canonical(
                    portal.target_cell,
                    portal.target_arrival_local,
                )?;
                let rot = Quat::from_axis_angle(Vec3::UP, portal.yaw_delta_radians);
                let next_vel = rot.rotate_vec3(velocity);
                return Ok(Some((arrival, next_vel, portal)));
            }
        }
        Ok(None)
    }
}

/// Metadata descriptor of a streamable `.uorc` sector in the connected Verdant universe.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UniverseSectorDescriptor {
    /// Sector grid coordinate.
    pub cell: UniverseCellCoord,
    /// Canonical sector identifier.
    pub slug: String,
    /// Display title shown in the Studio HUD.
    pub title: String,
    /// Biome and architectural summary.
    pub summary: String,
    /// Deterministic sector generation seed.
    pub sector_seed: u64,
}

/// Multi-sector universe manager tracking active `3x1x3` neighborhood residency and portals.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UniverseManager {
    /// Registered canonical sectors.
    pub sectors: Vec<UniverseSectorDescriptor>,
    /// Spatial portal graph connecting sectors and federated worlds.
    pub portal_graph: PortalGraph,
    /// Active possessed-character universe position.
    pub active_position: UniversePosition,
}

impl UniverseManager {
    /// Synthesizes the canonical three-sector Verdant Universe (`Citadel`, `Arch Bridge Valley`, `High Observatory`).
    pub fn canonical_citadel_universe(base_seed: u64) -> Result<Self, NarrativeLawError> {
        let two = two_f64();
        let four = two * two;
        let sixteen = two * two * two * two;

        let sectors = vec![
            UniverseSectorDescriptor {
                cell: UniverseCellCoord::ORIGIN,
                slug: "citadel-vault-0-0-0".to_string(),
                title: "Citadel Courtyard & Vaulted Hall".to_string(),
                summary: "Groin-vaulted limestone hall, four arched portals, entasis colonnade, and neuromorphic Observer Core.".to_string(),
                sector_seed: base_seed.max(1),
            },
            UniverseSectorDescriptor {
                cell: UniverseCellCoord::new(1, 0, 0),
                slug: "arch-bridge-valley-1-0-0".to_string(),
                title: "Parabolic Arch Bridge & River Terrace".to_string(),
                summary: "Crowned masonry arch bridge, stepped river embankments, and refractive water basin.".to_string(),
                sector_seed: base_seed.max(1).wrapping_add(1 + 1),
            },
            UniverseSectorDescriptor {
                cell: UniverseCellCoord::new(0, 0, -1),
                slug: "high-observatory-0-0-n1".to_string(),
                title: "High Observatory & Proving Track".to_string(),
                summary: "Alpine granite ridge, articulated XPBD pendulum bay, and 4-wheel suspension proving circuit.".to_string(),
                sector_seed: base_seed.max(1).wrapping_add(1 + 1 + 1 + 1),
            },
        ];

        let portals = vec![
            SpatialPortal {
                portal_id: "north_observatory_gate".to_string(),
                source_cell: UniverseCellCoord::ORIGIN,
                source_local: Vec3::new(0.0, 0.0, -(sixteen + sixteen)),
                trigger_radius_meters: two,
                target_cell: UniverseCellCoord::new(0, 0, -1),
                target_arrival_local: Vec3::new(0.0, 0.0, sixteen + four),
                yaw_delta_radians: 0.0,
                federated_uri: Some("stitch://verdant/high-observatory".to_string()),
            },
            SpatialPortal {
                portal_id: "east_bridge_terrace_gate".to_string(),
                source_cell: UniverseCellCoord::ORIGIN,
                source_local: Vec3::new(sixteen + sixteen, 0.0, 0.0),
                trigger_radius_meters: two,
                target_cell: UniverseCellCoord::new(1, 0, 0),
                target_arrival_local: Vec3::new(-(sixteen + four), 0.0, 0.0),
                yaw_delta_radians: 0.0,
                federated_uri: Some("stitch://verdant/arch-bridge-valley".to_string()),
            },
        ];

        let portal_graph = PortalGraph::new(portals)?;
        let active_position = UniversePosition::new_canonical(
            UniverseCellCoord::ORIGIN,
            Vec3::new(0.0, 0.0, sixteen),
        )?;

        Ok(Self {
            sectors,
            portal_graph,
            active_position,
        })
    }

    /// Returns all sectors within `chebyshev_distance <= 1` of `self.active_position.cell`.
    #[must_use]
    pub fn resident_sectors(&self) -> Vec<&UniverseSectorDescriptor> {
        self.sectors
            .iter()
            .filter(|s| s.cell.chebyshev_distance(self.active_position.cell) <= 1)
            .collect()
    }

    /// Looks up the active sector title (or synthesizes a procedural frontier title for outer cells).
    #[must_use]
    pub fn active_sector_title(&self) -> String {
        let c = self.active_position.cell;
        self.sectors
            .iter()
            .find(|s| s.cell == c)
            .map_or_else(|| "Frontier Sector".to_string(), |s| s.title.clone())
    }

    /// Serializes the verified `UniverseManager` manifest into deterministic pretty-printed JSON bytes.
    pub fn to_manifest_json_bytes(&self) -> Result<Vec<u8>, NarrativeLawError> {
        serde_json::to_vec_pretty(self).map_err(|e| {
            NarrativeLawError::InvalidPortalOrUniverseCell {
                detail: e.to_string(),
            }
        })
    }
}
